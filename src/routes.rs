use axum::{
    middleware,
    routing::{get, get_service, post},
    Router,
};
use tower_http::services::ServeDir;

use crate::{
    features::{hello, notes, passgen},
    middlewares,
};

pub fn app_routes() -> Router {
    Router::new()
        .merge(hello::handler::hello_routes())
}

pub fn api_routes() -> Router {
    Router::new()
        .merge(passgen::handler::passgen_routes())
        .merge(notes::handler::notes_routes())
        .layer(middleware::from_fn(middlewares::middleman::my_middleware))
}

pub fn routes_static() -> Router {
    Router::new().nest_service("/files", get_service(ServeDir::new("./files")))
}
