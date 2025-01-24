use crate::types::user::UserPayload;
use crate::{auth, AppState};
use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse, HttpResponseBuilder, Responder};
use sqlx::Row;

#[post("/register")]
async fn register(user: web::Json<UserPayload>, data: web::Data<AppState>) -> impl Responder {
    let pool = &data.pool;

    let row = match sqlx::query("SELECT COUNT(id) FROM users WHERE email = $1")
        .bind(&user.email)
        .fetch_one(pool)
        .await
    {
        Ok(row) => row,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let count: i64 = row.get(0);
    if count > 0 {
        return HttpResponseBuilder::new(StatusCode::CONFLICT).body("Email already registered");
    }

    let id = uuid::Uuid::new_v4();
    let hash = match auth::passwords::hash_password(&user.password) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

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

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").service(register));
}
