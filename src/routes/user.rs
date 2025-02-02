use crate::middleware::refresh_token_middleware;
use crate::types::user::{TokenGroup, UserPayload};
use crate::{auth, types, AppState};
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::Row;

#[post("/register")]
async fn register(user: web::Json<UserPayload>, data: web::Data<AppState>) -> impl Responder {
    let pool = &data.pool;

    // Count the users with this email.
    let row = match sqlx::query("SELECT COUNT(id) FROM users WHERE email = $1")
        .bind(&user.email)
        .fetch_one(pool)
        .await
    {
        Ok(row) => row,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let count: i64 = match row.try_get(0) {
        Ok(count) => count,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // If the count is not equal to zero then the email is already in use so we return.
    if count != 0 {
        return HttpResponse::Conflict().body("User already registered");
    }

    let id = uuid::Uuid::new_v4();

    // Hash the password for security.
    let hash = match auth::passwords::hash_password(&user.password) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Insert the user into the database.
    match sqlx::query("INSERT INTO users(id, email, password) VALUES ($1, $2, $3)")
        .bind(&id)
        .bind(&user.email)
        .bind(hash)
        .execute(pool)
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn respond_with_token_group(
    data: web::Data<AppState>,
    id: uuid::Uuid,
    sub: uuid::Uuid,
) -> HttpResponse {
    let pool = &data.pool;
    match sqlx::query!(
        "INSERT INTO tokens(id, user_id, expire_date) VALUES ($1, $2, $3)",
        &id,
        &sub,
        (chrono::Utc::now() + chrono::Duration::days(14)).naive_utc()
    )
    .execute(pool)
    .await
    {
        Err(_) => return HttpResponse::InternalServerError().finish(),
        _ => {}
    }

    // Create and encode refresh token and access token.
    let refresh_token = match auth::jwt::RefreshTokenClaims::new(
        id,
        sub,
        (chrono::Utc::now() + chrono::Duration::days(14)).timestamp(),
    )
    .encode(&data.jwt_secret)
    {
        Ok(refresh_token) => refresh_token,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let access_token = match auth::jwt::AccessTokenClaims::new(
        sub,
        (chrono::Utc::now() + chrono::Duration::minutes(10)).timestamp(),
    )
    .encode(&data.jwt_secret)
    {
        Ok(access_token) => access_token,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    HttpResponse::Ok().json(TokenGroup {
        refresh_token,
        access_token,
    })
}

#[post("/login")]
async fn login(user: web::Json<UserPayload>, data: web::Data<AppState>) -> impl Responder {
    let pool = &data.pool;

    // Get a user with the same email.
    let fetched_user = match sqlx::query_as!(
        types::user::User,
        "SELECT * FROM users WHERE email = $1",
        &user.email
    )
    .fetch_one(pool)
    .await
    {
        Ok(row) => row,
        Err(e) => {
            return match e {
                // If the error is RowNotFound the user doesn't exist so we return.
                sqlx::Error::RowNotFound => HttpResponse::NotFound().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            };
        }
    };

    // Check if the passwords match.
    if !auth::passwords::verify_password(&user.password, &fetched_user.password) {
        return HttpResponse::Unauthorized().finish();
    }

    // Delete any token connected with the user.
    match sqlx::query!("DELETE FROM tokens WHERE user_id = $1", &fetched_user.id)
        .execute(pool)
        .await
    {
        Err(_) => return HttpResponse::InternalServerError().finish(),
        _ => {}
    }

    let id = uuid::Uuid::new_v4();
    let sub = fetched_user.id;

    respond_with_token_group(data, id, sub).await
}

#[get("/refresh")]
async fn refresh(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let extensions = req.extensions();
    let claims = match extensions.get::<auth::jwt::RefreshTokenClaims>() {
        Some(claims) => claims,
        None => return HttpResponse::Unauthorized().finish(),
    };

    // Delete the token from the database so it can't be used again.
    match sqlx::query!("DELETE FROM tokens WHERE id = $1", &claims.id)
        .execute(&data.pool)
        .await
    {
        Err(_) => return HttpResponse::InternalServerError().finish(),
        _ => {}
    }

    let id = uuid::Uuid::new_v4();
    let sub = claims.sub;

    respond_with_token_group(data, id, sub).await
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(register)
            .service(login)
            .service(
                web::scope("")
                    .service(refresh)
                    .wrap(actix_web::middleware::from_fn(refresh_token_middleware)),
            ),
    );
}
