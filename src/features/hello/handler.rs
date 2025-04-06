use axum::response::IntoResponse;
use axum::{routing::get, Router};

/***********/

pub fn hello_routes() -> Router {
    Router::new().route("/", get(hello_world))
}

/***********/

pub async fn hello_world() -> impl IntoResponse {
    "Hello world!"
}
