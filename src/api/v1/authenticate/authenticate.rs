use crate::services::authenticate::authenticate::validate_user_credentials;
use crate::AppState;
use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    jti: String,
    iss: String,
    aud: String,
    sub: String,
    first_name: String,
    last_name: String,
    client_id: String,
    iat: u64,
    exp: u64,
}

#[axum::debug_handler]
pub async fn log_in(
    State(state): State<Arc<AppState>>,
    Json(Credentials { email, password }): Json<Credentials>,
) -> impl IntoResponse {
    match validate_user_credentials(email, password, &state.db).await {
        Ok(user) => {
            let in_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let my_claims = Claims {
                jti: Uuid::new_v4().to_string(),
                aud: "client".to_owned(),
                sub: user.email.to_owned(),
                iss: "auth_server".to_owned(),
                first_name: user.first_name.to_owned(),
                last_name: user.last_name.to_owned(),
                client_id: Uuid::new_v4().to_string(),
                iat: in_at,
                exp: in_at + 1_209_600,
            };

            let jwt = encode(
                &Header::default(),
                &my_claims,
                &EncodingKey::from_secret(env::var("PRIVATE_KEY").unwrap().as_ref()),
            );

            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(
                    Json(jwt.map_err(|_| {
                        Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::empty().into_response())
                            .unwrap()
                    })?)
                    .into_response(),
                )
                .unwrap())
        }

        Err(err) => Err(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(err.to_string().into_response())
            .unwrap()),
    }
}
