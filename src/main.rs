use actix_web::{web, App, HttpServer, Responder};
use sqlx::{Pool, Postgres};

mod auth;
pub mod config;
mod types;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(pool: Pool<Postgres>, jwt_secret: String) -> Self {
        Self { pool, jwt_secret }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::load::Config::load();
    let pool = sqlx::postgres::PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    let app_state = AppState::new(pool, config.jwt_secret);

    HttpServer::new(move || App::new().app_data(web::Data::new(app_state.clone())))
        .bind(config.server_socket)?
        .run()
        .await?;

    Ok(())
}
