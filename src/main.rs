use crate::config::Config;

mod config;

#[derive(Clone)]
struct Services {}

#[derive(Clone)]
struct App {
    pub services: Services,
}

#[tokio::main]
async fn main() {
    let config = Config::new();

    let app = App {
        services: Services {},
    };

    let listener = tokio::net::TcpListener::bind(config.address)
        .await
        .expect("Failed to bind tcp listener");

    let router = axum::Router::new().with_state(app);
    axum::serve(listener, router)
        .await
        .expect("Failed to start server");
}
