use axum::{
    extract::rejection::{JsonRejection, PathRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::{json, Value};
use thiserror::Error;

pub type ResponseResult<T> = core::result::Result<T, ApiError>;

#[derive(Debug, Error)]
pub enum ApiError {
    // first json extract error rejection
    #[error("transparent")]
    JsonExtractorRejection(#[from] JsonRejection),

    // first path extract error rejection
    #[error("path")]
    PathExtractorRejection(#[from] PathRejection),

    // For deserialization issues
    #[error("Invalid JSON format")]
    InvalidJson(#[from] serde_json::Error),

    // For general internal server issues
    #[error("Internal server error")]
    InternalServerError,

    // For specific application errors, you could add more
    #[error("Not found")]
    NotFound,

    // Another example, can be used for cases like unauthorized access
    #[error("Unauthorized")]
    Unauthorized,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        println!("=>> {:<12} - Error Response - {:#?}", "INTO RES", self);

        // match error enum
        let (status, error_message, code) = match &self {
            ApiError::JsonExtractorRejection(res) => (
                StatusCode::BAD_REQUEST,
                res.body_text().to_string(),
                res.status(),
            ),
            ApiError::PathExtractorRejection(res) => (
                StatusCode::BAD_REQUEST,
                res.body_text().to_string(),
                res.status(),
            ),
            ApiError::InvalidJson(res) => (
                StatusCode::BAD_REQUEST,
                String::from("Invalid JSON"),
                StatusCode::from_u16(400).unwrap(),
            ),
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                String::from("Not found"),
                StatusCode::from_u16(404).unwrap(),
            ),
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                String::from("Unauthorized"),
                StatusCode::from_u16(401).unwrap(),
            ),
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
                StatusCode::from_u16(500).unwrap(),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Unknown server error"),
                StatusCode::from_u16(500).unwrap(),
            ),
        };

        let body = json!({
            "status" : false,
            "message" : error_message,
            "error" : status.to_string(),
            "code" :  code.to_string()
        });

        (status, Json(body)).into_response()
    }
}

pub fn output_json<T>(
    status: bool,
    msg: String,
    data: T,
    code: Option<u16>,
) -> ResponseResult<Json<Value>>
where
    T: Serialize,
{
    let body = Json(json!({
        "status": status,
        "message": msg,
        "data": data,
        "code": code.unwrap_or(200)
    }));
    Ok(body)
}
