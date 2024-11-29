use axum::{Router, routing::get};


fn main() {
    let app = Router::new().route("/", get(root));

}


async fn root() {}