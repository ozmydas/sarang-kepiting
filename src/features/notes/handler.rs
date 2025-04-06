use axum::response::IntoResponse;
use axum::routing::{delete, post};
use axum::Json;
use axum::{routing::get, Router};
use axum_extra::extract::WithRejection;
use serde_json::{json, Value};

use crate::utils::custom_response::{output_json, ApiError, ResponseResult};

use super::model::NotesPayload;

/***********/

pub fn notes_routes() -> Router {
    Router::new()
        .route("/notes", get(list_handler))
        .route("/notes", post(save_handler))
        .route("/notes/{code}", get(detail_handler))
        .route("/notes/{code}", post(update_handler))
        .route("/notes/{code}", delete(delete_handler))
}

/***********/

pub async fn list_handler() -> ResponseResult<Json<Value>> {
    super::service::list_service();
    let data = [0; 0];

    output_json(false, String::from("test by system"), data, None)
} //end func

pub async fn save_handler(
    WithRejection(Json(payload), _): WithRejection<Json<NotesPayload>, ApiError>,
) -> ResponseResult<Json<Value>> {
    match super::service::save_service(payload).await {
        Ok(res) => output_json(true, "data saved successfully".into(), res, None),
        Err(err) => output_json(false, err.to_string(), [0;0], None),
    }
} //end func

pub async fn detail_handler() -> ResponseResult<Json<Value>> {
    super::service::list_service();
    let data = [0; 0];

    output_json(false, String::from("test by system"), data, None)
} //end func

pub async fn update_handler() -> ResponseResult<Json<Value>> {
    super::service::list_service();
    let data = [0; 0];

    output_json(false, String::from("test by system"), data, None)
} //end func

pub async fn delete_handler() -> ResponseResult<Json<Value>> {
    super::service::list_service();

    output_json(false, String::from("test by system"), "", None)
} //end func
