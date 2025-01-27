use actix_web::{middleware, web, App, HttpServer};
use sqlx::{Pool, Postgres};

pub mod auth;
pub mod config;
pub mod routes;
pub mod types;

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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(middleware::Logger::default())
            .configure(routes::user::config)
    })
    .bind(config.server_socket)?
    .run()
    .await?;

    Ok(())
}
