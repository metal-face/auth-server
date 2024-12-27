use crate::models::credentials::Credentials;
use crate::services::authenticate::authenticate::validate_user_credentials;
use crate::utilities::mint_jwt::mint_jwt;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::sync::Arc;

#[axum::debug_handler]
pub async fn log_in(
    State(state): State<Arc<AppState>>,
    Json(Credentials { email, password }): Json<Credentials>,
) -> impl IntoResponse {
    match validate_user_credentials(email, password, &state.db).await {
        Ok(user) => {
            let jwt = mint_jwt(&user, &state.db).await;

            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Json(&jwt).into_response())
                .unwrap())
        }

        Err(err) => Err(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(err.to_string().into_response())
            .unwrap()),
    }
}
