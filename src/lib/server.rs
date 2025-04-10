use crate::auth::{access_token_claims, refresh_token_claims, Authenticator};
use crate::handlers;
use crate::services::task::TaskService;
use crate::services::user::UserService;
use axum::extract::FromRef;
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, post, put};
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
pub struct AppState<U, T>
where
    U: UserService,
    T: TaskService,
{
    /// Service for the users.
    pub user_service: Arc<U>,
    pub task_service: Arc<T>,
    /// Authenticator used to authenticate tokens.
    pub authenticator: Arc<Authenticator>,
}

impl<U, T> AppState<U, T>
where
    U: UserService,
    T: TaskService,
{
    pub fn new(
        user_service: Arc<U>,
        task_service: Arc<T>,
        authenticator: Arc<Authenticator>,
    ) -> Self {
        Self {
            user_service,
            task_service,
            authenticator,
        }
    }
}

/// AuthState is substate of `AppState` for authentication.
#[derive(Clone)]
pub struct AuthState {
    pub authenticator: Arc<Authenticator>,
}

impl<U, T> FromRef<AppState<U, T>> for AuthState
where
    U: UserService,
    T: TaskService,
{
    fn from_ref(state: &AppState<U, T>) -> Self {
        Self {
            authenticator: state.authenticator.clone(),
        }
    }
}

/// UserState is substate of `AppState` for users.
#[derive(Clone)]
pub struct UserState<T: UserService> {
    pub user_service: Arc<T>,
}

impl<U, T> FromRef<AppState<U, T>> for UserState<U>
where
    U: UserService,
    T: TaskService,
{
    fn from_ref(state: &AppState<U, T>) -> Self {
        Self {
            user_service: state.user_service.clone(),
        }
    }
}

/// TaskState is substate of `AppState` for tasks.
#[derive(Clone)]
pub struct TaskState<T: TaskService> {
    pub task_service: Arc<T>,
}

impl<U, T> FromRef<AppState<U, T>> for TaskState<T>
where
    U: UserService,
    T: TaskService,
{
    fn from_ref(state: &AppState<U, T>) -> Self {
        Self {
            task_service: state.task_service.clone(),
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
    pub async fn new<U, T>(
        server_config: ServerConfig<'_>,
        user_service: U,
        task_service: T,
    ) -> Result<Self, std::io::Error>
    where
        U: UserService,
        T: TaskService,
    {
        let app_state = AppState::new(
            Arc::new(user_service),
            Arc::new(task_service),
            server_config.authenticator,
        );

        let tcp_listener = TcpListener::bind(server_config.server_addr).await?;
        let router = Router::new()
            .nest(
                "/users",
                Router::new()
                    .route("/register", post(handlers::user::register))
                    .route("/login", post(handlers::user::login))
                    .route(
                        "/refresh",
                        get(handlers::user::refresh)
                            .layer(from_fn_with_state(app_state.clone(), refresh_token_claims)),
                    ),
            )
            .nest(
                "/tasks",
                Router::new()
                    .route("/add", post(handlers::task::add_task))
                    .route("/get", get(handlers::task::get_tasks))
                    .route("/update", put(handlers::task::update_task))
                    .route("/delete/{id}", delete(handlers::task::delete_task))
                    .layer(from_fn_with_state(app_state.clone(), access_token_claims)),
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
