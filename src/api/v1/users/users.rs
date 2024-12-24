use crate::models::user_dto::UserDTO;
use crate::services::users::users::validate_user;
use crate::AppState;
use axum::extract::State;
use axum::http::{Response, StatusCode};
use axum::response::{IntoResponse, Json};
use std::sync::Arc;

#[axum::debug_handler]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(UserDTO {
        first_name,
        last_name,
        email,
        password,
        ..
    }): Json<UserDTO>,
) -> impl IntoResponse {
    let user = UserDTO {
        first_name,
        last_name,
        email,
        password,
        created_at: None,
        updated_at: None,
        deleted_at: None,
    };

    match validate_user(user, &state.db).await {
        Ok(user) => {
            let user_dto = UserDTO {
                first_name: user.first_name,
                last_name: user.last_name,
                email: user.email,
                password: None,
                created_at: Some(user.created_at),
                updated_at: Some(user.updated_at),
                deleted_at: user.deleted_at,
            };
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Json(user_dto).into_response())
                .unwrap())
        }
        Err(err) => Err(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(err.to_string().into_response())
            .unwrap()),
    }
}
