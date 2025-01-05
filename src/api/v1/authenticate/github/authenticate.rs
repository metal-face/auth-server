use axum::extract::Query;
use axum::http::StatusCode;
use axum::Json;
use std::env;

#[derive(serde::Deserialize)]
pub struct QueryParams {
    code: String,
}

#[derive(serde::Serialize)]
struct GitHubRequest {
    client_id: String,
    client_secret: String,
    code: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GitHubResponse {
    access_token: String,
    token_type: String,
    scope: String,
}

pub async fn github_authentication(query_params: Query<QueryParams>) -> StatusCode {
    let client_id = env::var("GITHUB_CLIENT_ID").unwrap();
    let client_secret = env::var("GITHUB_CLIENT_SECRET").unwrap();
    let code = query_params.code.clone();

    let body: GitHubRequest = GitHubRequest {
        client_id,
        client_secret,
        code,
    };

    let req = reqwest::Client::new()
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await
        .unwrap()
        .json::<GitHubResponse>()
        .await
        .unwrap();

    println!("{:?}", req);

    StatusCode::ACCEPTED
}
