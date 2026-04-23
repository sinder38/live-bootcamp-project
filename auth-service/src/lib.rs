use std::error::Error;

use axum::{http::StatusCode, response::IntoResponse, routing::get, serve::Serve, Router};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let asset_dir =
            ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

        let router = Router::new()
            .route("/signup", get(signup))
            .route("/login", get(login))
            .route("/logout", get(logout))
            .route("/verify-2fa", get(verify_2fa))
            .route("/verify-token", get(verify_token))
            .fallback_service(asset_dir);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }

    pub fn server(&self) -> &Serve<TcpListener, Router, Router> {
        &self.server
    }
}

async fn signup() -> impl IntoResponse {
    StatusCode::OK
}

async fn login() -> impl IntoResponse {
    StatusCode::OK
}

async fn logout() -> impl IntoResponse {
    StatusCode::OK
}

async fn verify_2fa() -> impl IntoResponse {
    StatusCode::OK
}

async fn verify_token() -> impl IntoResponse {
    StatusCode::OK
}
