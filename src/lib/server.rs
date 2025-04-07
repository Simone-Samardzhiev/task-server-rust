use axum::Router;
use tokio::net::TcpListener;

/// `ServerConfig` holds server configuration.
pub struct ServerConfig<'a> {
    server_addr: &'a str,
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
    pub async fn new(server_config: ServerConfig<'_>) -> Result<Self, std::io::Error> {
        let tcp_listener = TcpListener::bind(server_config.server_addr).await?;
        let router = Router::new();
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
