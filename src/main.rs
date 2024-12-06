use crate::api::v1::users::users::create_user;
use axum::{routing::post, Router};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;

mod api;
mod config;
mod repositories;
mod services;

struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    let db_connection_str: String = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://metalface:password@localhost".to_string());

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&db_connection_str)
        .await
        .expect("Can't connect to database");

    let shared_state: Arc<AppState> = Arc::new(AppState { db: pool });

    let app: Router = Router::new()
        .route("/users", post(create_user))
        .with_state(shared_state);

    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
