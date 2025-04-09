use crate::auth::Authenticator;
use crate::handlers;
use crate::services::user::UserService;
use axum::extract::FromRef;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;
use tokio::net::TcpListener;

/// `ServerConfig` holds server configuration.
pub struct ServerConfig<'a> {
    server_addr: &'a str,
    authenticator: Arc<Authenticator>,
}

impl<'a> ServerConfig<'a> {
    /// `new` method used to create new server configuration.
    pub fn new(server_addr: &'a str, authenticator: Arc<Authenticator>) -> Self {
        ServerConfig {
            server_addr,
            authenticator,
        }
    }
}

/// Struct holding the app state.
#[derive(Clone)]
pub struct AppState<T>
where
    T: UserService,
{
    /// Service for the users.
    pub user_service: Arc<T>,
    /// Authenticator used to authenticate tokens.
    pub authenticator: Arc<Authenticator>,
}

impl<T: UserService> AppState<T> {
    pub fn new(user_service: Arc<T>, authenticator: Arc<Authenticator>) -> Self {
        Self {
            user_service,
            authenticator,
        }
    }
}

/// AuthState is sub state of `AppState`
#[derive(Clone)]
pub struct AuthState {
    pub authenticator: Arc<Authenticator>,
}

impl AuthState {
    pub fn new(authenticator: Arc<Authenticator>) -> Self {
        Self { authenticator }
    }
}

impl<T: UserService> FromRef<AppState<T>> for AuthState {
    fn from_ref(state: &AppState<T>) -> Self {
        Self {
            authenticator: state.authenticator.clone(),
        }
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
        let app_state = AppState::new(Arc::new(user_service), server_config.authenticator);

        let tcp_listener = TcpListener::bind(server_config.server_addr).await?;
        let router = Router::new()
            .nest(
                "/users",
                Router::new().route("/register", post(handlers::user::register)),
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
