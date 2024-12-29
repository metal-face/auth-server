use crate::models::user_dto::UserDTO;
use crate::repositories::users::users::get_user_by_id;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    match get_user_by_id(&state.db, Uuid::from_str(&user_id).unwrap()).await {
        Ok(user) => {
            let user_dto = UserDTO::from(user);

            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Json(user_dto).into_response())
                .unwrap())
        }
        Err(err) => Err(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Json(err.to_string()).into_response())
            .unwrap()),
    }
}
