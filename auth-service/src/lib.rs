use std::error::Error;

use axum::{routing::post, serve::Serve, Router};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

mod app_state;
pub mod domain;
pub mod routes;
pub mod services;

use routes::*;

pub use crate::app_state::{AppState, UserStoreType};

pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .route_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_token))
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

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
