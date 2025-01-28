use crate::auth::jwt::AccessTokenClaims;
use crate::types::tasks::{Task, TaskPayload};
use crate::AppState;
use actix_web::middleware::from_fn;
use actix_web::{delete, get, post, put, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;

#[get("/get")]
async fn get_tasks(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let pool = &data.pool;
    let extensions = req.extensions();
    let claims = match extensions.get::<AccessTokenClaims>() {
        Some(claims) => claims,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match sqlx::query_as!(
        Task,
        "SELECT id, name, description, type, due_date, date_completed, date_deleted FROM tasks WHERE user_id = $1",
        &claims.sub
    )
        .fetch_all(pool)
        .await
    {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/add")]
async fn add_task(
    task: web::Json<TaskPayload>,
    req: HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let pool = &data.pool;
    let extensions = req.extensions();
    let claims = match extensions.get::<AccessTokenClaims>() {
        Some(claims) => claims,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let task = Task::new_from_task_payload(&task);

    match sqlx::query!(
        r#"
        INSERT INTO tasks (id, name, description, type, due_date, date_completed, date_deleted, user_id)
        VALUES ($1, $2, $3, $4, $5, NULL, NULL, $6)
        "#,
        task.id,
        task.name,
        task.description,
        task.r#type,
        task.due_date,
        claims.sub,
    )
        .execute(pool)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(task),
        Err(e) => {
            eprintln!("Error inserting task: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[put("/update")]
async fn update_task(
    task: web::Json<Task>,
    req: HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let pool = &data.pool;
    let extensions = req.extensions();
    match extensions.get::<AccessTokenClaims>() {
        Some(_) => {  },
        None => return HttpResponse::Unauthorized().finish(),
    };

    match sqlx::query!(
        r#"
        UPDATE tasks SET name = $1, description = $2, type = $3, due_date = $4, date_completed = $5, date_deleted = $6 WHERE id = $7
        "#,
        task.name,
        task.description,
        task.r#type,
        task.due_date,
        task.date_completed,
        task.date_deleted,
        task.id
    )
        .execute(pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 1 {
                HttpResponse::Ok().json(task)
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            eprintln!("Error inserting task: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("/delete/{id}")]
async fn delete_task(
    id: web::Path<Uuid>,
    req: HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let pool = &data.pool;
    let extensions = req.extensions();
    match extensions.get::<AccessTokenClaims>() {
        Some(_) => {}
        None => return HttpResponse::Unauthorized().finish(),
    };

    match sqlx::query!("DELETE FROM tasks WHERE id = $1", id.into_inner())
        .execute(pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 1 {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks")
            .service(get_tasks)
            .service(add_task)
            .service(update_task)
            .service(delete_task)
            .wrap(from_fn(crate::middleware::access_token_middleware)),
    );
}
