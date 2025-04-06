use axum::{
    extract::rejection::JsonRejection,
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
    #[error(transparent)]
    // first json extract error rejection
    JsonExtractorRejection(#[from] JsonRejection),

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
        println!("=>> {:<12} - Error Response - {self:?}", "INTO RES");

        // match error enum
        let (status, error_message) = match &self {
            ApiError::JsonExtractorRejection(_) => (StatusCode::BAD_REQUEST, "Error JSON format"),
            ApiError::InvalidJson(_) => (StatusCode::BAD_REQUEST, "Invalid JSON format"),
            ApiError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
        };

        let body = json!({
            "status" : false,
            "message" : error_message,
            "error" : status.to_string(),
        });

        (status, Json(body)).into_response()
    }
}


pub fn output_json<T>(status: bool, msg: String, data: T, code : Option<u8>) -> ResponseResult<Json<Value>> where T:Serialize{
    let body = Json(json!({
        "status": status,
        "message": msg,
        "data": data,
        "code": code.unwrap_or(200)
    }));
    Ok(body)
}