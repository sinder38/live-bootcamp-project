use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    services::{banned_user_store::HashsetBannedTokenStore, hashmap_user_store::HashmapUserStore},
    utils::constants::prod,
    AppState, Application,
};

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let app_state = AppState {
        user_store,
        banned_token_store: banned_token_store,
    };

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
