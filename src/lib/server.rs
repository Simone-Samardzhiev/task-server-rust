use crate::services::user::UserService;
use axum::Router;
use std::sync::Arc;
use axum::routing::post;
use tokio::net::TcpListener;
use crate::handlers;

/// `ServerConfig` holds server configuration.
pub struct ServerConfig<'a> {
    server_addr: &'a str,
}

#[derive(Clone)]
pub struct AppState<T>
where
    T: UserService,
{
    pub user_service: Arc<T>,
}

impl<'a> ServerConfig<'a> {
    /// `new` method used to create new server configuration.
    pub fn new(server_addr: &'a str) -> Self {
        ServerConfig { server_addr }
    }
}

/// `Server` is runnable struct create local server.
pub struct Server {
    listener: TcpListener,
    router: Router,
}

impl Server {
    /// `new` will create a new server bound to server address specified in `ServerConfig`
    pub async fn new(
        server_config: ServerConfig<'_>,
        user_service: impl UserService,
    ) -> Result<Self, std::io::Error> {
        let app_state = AppState {
            user_service: Arc::new(user_service),
        };

        let tcp_listener = TcpListener::bind(server_config.server_addr).await?;
        let router = Router::new()
            .nest("/users", Router::new()
                .route("/register", post(handlers::user::register))
            )
            .with_state(app_state);

        Ok(Server {
            listener: tcp_listener,
            router,
        })
    }

    /// `run` will run the server.
    pub async fn run(self) -> Result<(), std::io::Error> {
        axum::serve(self.listener, self.router).await?;
        Ok(())
    }
}
