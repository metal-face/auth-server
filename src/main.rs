use crate::api::v1::authenticate::credentials::authenticate::log_in;
use crate::api::v1::users::create_user::create_user;
use crate::api::v1::users::get_user_by_id::get_user;
use axum::routing::get;
use axum::{routing::post, Router};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use crate::api::v1::authenticate::github::authenticate::github_authentication;

mod api;
mod models;
mod repositories;
mod services;
mod utilities;

struct AppState {
    db: PgPool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv::dotenv().ok();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost:5432/auth-server".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&db_connection_str)
        .await
        .expect("Can't connect to database");

    let shared_state = Arc::new(AppState { db: pool });

    let app = Router::new()
        .route("/users", post(create_user))
        .route("/authenticate/credentials", post(log_in))
        .route("/authenticate/github", post(github_authentication))
        .route("/users/:id", get(get_user))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
