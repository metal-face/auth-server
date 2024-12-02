use axum::{Router, routing::get, routing::post};
use tokio::net::TcpListener;
mod api;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/users", post(api::v1::users::users::create_user()));

    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn root() {}