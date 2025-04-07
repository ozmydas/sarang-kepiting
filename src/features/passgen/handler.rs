use axum::routing::post;
use axum::Json;
use axum::Router;
use axum_extra::extract::WithRejection;
use serde_json::{json, Value};

use super::model::PassgenPayload;

use crate::utils;
use crate::utils::custom_response::{ApiError, ResponseResult};

/***********/

pub fn passgen_routes() -> Router {
    Router::new().route("/passgen", post(api_passgen))
}

/***********/

pub async fn api_passgen(
    WithRejection(Json(payload), _): WithRejection<Json<PassgenPayload>, ApiError>,
) -> ResponseResult<Json<Value>> {
    let body = Json(json!({
            "status" : true,
            "message": "response return success",
            "data": {
                "password" : utils::password_generator::generate_password(payload.over12, payload.non_english_word, payload.include_special_chars, payload.include_uppercase, payload.include_number)
            }
        }
    ));

    Ok(body)
}
