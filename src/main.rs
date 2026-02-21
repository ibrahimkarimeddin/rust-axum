use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod config;
mod error;
mod handlers;
mod models;
mod repositories;
mod services;

use crate::handlers::user_handler::{create_user, get_user, get_users, update_user, delete_user, AppState};
use crate::repositories::user_repo::UserRepository;
use crate::services::user_service::UserService;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let pool = config::db::connect().await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    let user_repo = UserRepository::new(pool);

    let user_service = UserService::new(user_repo);

    let state = AppState { user_service };

    let app = Router::new()
        .route("/users", post(create_user).get(get_users))
        .route("/users/{id}", get(get_user).put(update_user).delete(delete_user))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server started, listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}