use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub jti: String,
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub first_name: String,
    pub last_name: String,
    pub client_id: String,
    pub iat: u64,
    pub exp: u64,
}
