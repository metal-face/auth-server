use axum::{routing::get, routing::post, Json, Router};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;
use sqlx_core::pool::Pool;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;

mod api;
mod repositories;
mod services;

struct AppState {}

#[tokio::main]
async fn main() {
    let db_connection_str: String = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://metalface:password@localhost".to_string());

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&db_connection_str)
        .await
        .expect("Can't connect to database");

    let shared_state: Arc<AppState> = Arc::new(AppState {});

    let app: Router = Router::new().route(
        "/users",
        post({
            let shared_state: Arc<AppState> = Arc::clone(&shared_state);
            move |body: Json<api::v1::users::users::CreateUser>| {
                api::v1::users::users::create_user(body, shared_state)
            }
        }),
    );

    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
