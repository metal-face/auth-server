use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;

pub async fn github_authentication() -> impl IntoResponse {
    Response::builder().status(StatusCode::ACCEPTED).body(().into_response()).unwrap();
}