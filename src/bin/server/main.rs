use auth::Authenticator;
use dotenvy::dotenv;
use server::config::Config;
use server::server::{Server, ServerConfig};
use server::{auth, repositories, services};
use sqlx::PgPool;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    if dotenv().is_err() {
        println!("Failed to load .env file using");
    }

    let config = Config::new_from_env().expect("Failed to load config");
    let database = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    let authenticator = Arc::new(Authenticator::new(config.secret));

    let user_repository = repositories::user::PostgresUserRepository::new(database.clone());
    let token_repository = repositories::token::PostgresTokenRepository::new(database.clone());
    let user_service = services::user::DefaultUserService::new(
        Arc::new(user_repository),
        Arc::new(token_repository),
        authenticator.clone(),
    );

    let task_repository = repositories::task::PostgresTaskRepository::new(database.clone());
    let task_service = services::task::DefaultTaskService::new(Arc::new(task_repository));

    let server_config = ServerConfig::new(&config.server_addr, authenticator.clone());

    let server = Server::new(server_config, user_service, task_service)
        .await
        .expect("Failed to create a new server");

    server.run().await.expect("Failed to run server");
}
