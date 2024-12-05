use axum::{routing::get, routing::post, Json, Router};
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use sqlx::Postgres;
use sqlx::postgres::PgPoolOptions;
use sqlx_core::pool::PoolOptions;
use tokio::net::TcpListener;

mod api;
mod repositories;
mod services;

struct AppState {}

#[tokio::main]
async fn main() {
    let pool: PoolOptions<Postgres> = PgPoolOptions::new().max_connections(5).acquire_timeout(Duration::from_secs(5));

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
