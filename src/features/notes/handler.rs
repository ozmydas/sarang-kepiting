use axum::extract::{Path, Query, Request};
use axum::http::request;
use axum::routing::{delete, post};
use axum::Json;
use axum::{routing::get, Router};
use axum_extra::extract::WithRejection;
use serde_json::Value;
use uuid::Uuid;

use crate::models::app_model;
use crate::utils::custom_response::{output_json, ApiError, ResponseResult};

use super::model::NotesPayload;

/***********/

pub fn notes_routes() -> Router {
    Router::new()
        .route("/notes", get(list_handler))
        .route("/notes/new", get(create_handler))
        .route("/notes/new", post(save_handler))
        .route("/notes/{code}", get(detail_handler))
        .route("/notes/{code}", post(update_handler))
        .route("/notes/{code}", delete(delete_handler))
}

/***********/

pub async fn list_handler(
    paginatition: Query<app_model::Pagination>,
) -> ResponseResult<Json<Value>> {
    match super::service::list_service(paginatition).await {
        Ok(res) => output_json(true, "data retrieved successfully".into(), res, None),
        Err(err) => output_json(false, err.to_string(), [0; 0], Some(204)),
    }
} //end func

pub async fn create_handler(request: Request) -> ResponseResult<Json<Value>> {
    match super::service::new_create_info(request).await {
        Ok(res) => output_json(true, "data retrieved successfully".into(), res, None),
        Err(err) => output_json(false, err.to_string(), [0; 0], Some(204)),
    }
} //end func

pub async fn save_handler(
    WithRejection(Json(payload), _): WithRejection<Json<NotesPayload>, ApiError>,
) -> ResponseResult<Json<Value>> {
    match super::service::save_service(payload).await {
        Ok(res) => output_json(true, "data saved successfully".into(), res, None),
        Err(err) => output_json(false, err.to_string(), [0; 0], Some(204)),
    }
} //end func

pub async fn detail_handler(
    WithRejection(Path(code), _): WithRejection<Path<Uuid>, ApiError>,
) -> ResponseResult<Json<Value>> {
    match super::service::detail_service(code.into()).await {
        Ok(res) => output_json(true, "data retrieved successfully".into(), res, None),
        Err(err) => output_json(false, err.to_string(), [0; 0], Some(204)),
    }
} //end func

pub async fn update_handler(
    WithRejection(Path(code), _): WithRejection<Path<Uuid>, ApiError>,
    WithRejection(Json(payload), _): WithRejection<Json<NotesPayload>, ApiError>,
) -> ResponseResult<Json<Value>> {
    match super::service::update_service(code.into(), payload).await {
        Ok(res) => output_json(true, "data saved successfully".into(), res, None),
        Err(err) => output_json(false, err.to_string(), [0; 0], Some(204)),
    }
} //end func

pub async fn delete_handler(
    WithRejection(Path(code), _): WithRejection<Path<Uuid>, ApiError>,
) -> ResponseResult<Json<Value>> {
    match super::service::delete_service(code.into()).await {
        Ok(res) => output_json(true, "data deleted successfully".into(), res, None),
        Err(err) => output_json(false, err.to_string(), [0; 0], Some(204)),
    }
} //end func
