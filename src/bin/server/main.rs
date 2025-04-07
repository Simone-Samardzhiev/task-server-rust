use dotenvy::dotenv;
use server::config::Config;
use server::server::{Server, ServerConfig};
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    if dotenv().is_err() {
        println!("Failed to load .env file using");
    }

    let config = Config::new_from_env().expect("Failed to load config");
    let database = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    let server_config = ServerConfig::new(&config.server_addr);

    let server = Server::new(server_config)
        .await
        .expect("Failed to create a new server");

    server.run().await.expect("Failed to run server");
}
