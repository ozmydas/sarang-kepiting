use std::env;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use axum::extract::Request;
use axum::http::request;
use axum::response::IntoResponse;
use axum::{routing::get, Router};

use crate::utils::custom_response::output_json;

/***********/

pub fn hello_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/seeker", get(seeker))
}

/***********/

pub async fn hello_world() -> impl IntoResponse {
    "Hello world!"
}

pub async fn seeker(request: Request) -> impl IntoResponse {
    let headers = request.headers();

    let date = headers
        .get("z-date")
        .map_or("2000-01-01", |val| val.to_str().unwrap());

    let uid = headers
        .get("z-uuid")
        .map_or("0000", |val| val.to_str().unwrap());

    let password = format!("{}.{}.{}", env::var("APP_KEY").unwrap(), uid, date);
    let salt = SaltString::generate(&mut OsRng);

    let argon_params = argon2::Params::new(1024, 2, 1, Some(64)).unwrap();

    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, argon_params);

    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();

    output_json(true, "data retrieved successfully".into(), password_hash, None)
}
