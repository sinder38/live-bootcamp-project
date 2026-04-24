use axum::{response::Html, routing::get, serve, Router};
use tower_http::services::ServeDir;

use auth_service::{AppState, Application, UserStoreType};

//...

#[tokio::main]
async fn main() {
    let user_store = UserStoreType::default();
    let app_state = AppState { user_store };

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
