use std::net::TcpListener;

use lib::{configuration, router};

#[derive(Debug)]
pub struct TestApp {
    pub config: configuration::GlobalConfig,
}

impl TestApp {
    pub fn new() -> Self {
        // Parsing config
        let path = std::env::current_dir()
            .expect("couldn't get current directory")
            .join("config");

        let config = configuration::GlobalConfig::build(Some("test".into()), path)
            .expect("couldn't build config");

        Self { config }
    }

    pub async fn spawn_server(&mut self) {
        // Create tcp listener
        let listener = TcpListener::bind(format!("{}:0", &self.config.application.address))
            .expect("couldn't create tcp listener");
        let local_addr = listener
            .local_addr()
            .expect("couldn't get local address from listener");

        let router = router::make_router();

        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(router.into_make_service())
                .await
                .unwrap()
        });

        self.config.application.port = local_addr.port();
    }

    pub fn get_http_uri(&self, path: Option<&'static str>) -> String {
        let path = path.unwrap_or("");

        format!("http://{}{}", &self.config.application.address(), path)
    }
}
