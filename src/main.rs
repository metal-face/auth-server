use axum::{Router, routing::get, routing::post, Json};
use tokio::net::TcpListener;
use std::sync::Arc;
use serde_json::json;

mod api;
mod services;

struct AppState {}

#[tokio::main]
async fn main() {
    let shared_state: Arc<AppState> = Arc::new(AppState {});
    let app: Router = Router::new().route("/users", post({
        let shared_state: Arc<AppState> = Arc::clone(&shared_state);
        move |body: Json<api::v1::users::users::CreateUser>| api::v1::users::users::create_user(body, shared_state)
    }));

    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}