use std::any;

use anyhow::{Error, Ok};
use axum::{
    extract::{Query, Request},
    http::request,
};
use uuid::Uuid;

use crate::{config::database::get_db, models::app_model::Pagination};

use super::{
    model::{NotesModel, NotesNewCreateInfo, NotesPayload, NotesTagsModel},
    repository::NoteRepo,
};

pub async fn list_service(query: Query<Pagination>) -> Result<Vec<NotesModel>, Error> {
    let params = query.0;
    let limit = params.size.unwrap_or(10);
    let offset = (params.page.unwrap_or(1) - 1) * limit;
    Ok(NoteRepo::new(get_db()).get_all(limit, offset).await?)
} //end func

pub async fn save_service(payload: NotesPayload) -> Result<NotesModel, Error> {
    let item = NotesModel {
        code: Uuid::new_v4().into(),
        title: payload.title,
        content: payload.content,
        ..Default::default()
    };

    Ok(NoteRepo::new(get_db()).create(item).await?)
} //end func

pub async fn detail_service(code: String) -> Result<NotesModel, Error> {
    Ok(NoteRepo::new(get_db()).get_single(code).await?)
} //end func

pub async fn update_service(code: String, payload: NotesPayload) -> Result<NotesModel, Error> {
    let item = NotesModel {
        code: code.clone(),
        title: payload.title,
        content: payload.content,
        visibility: payload.visibility,
        ..Default::default()
    };

    println!("🏷️ {:#?}", payload.tags);
    let db = get_db();
    let mut tx = db.begin().await?;

    // update data
    let updated_main_data = NoteRepo::new(db).update(item, &mut tx).await?;

    // remove tags
    NoteRepo::new(db).remove_tags(code, &mut tx).await?;

    tx.commit().await;

    Ok(updated_main_data)
} //end func

pub async fn delete_service(code: String) -> Result<NotesModel, Error> {
    Ok(NoteRepo::new(get_db()).delete_soft(code).await?)
} //end func

pub async fn new_create_info(request: Request) -> Result<NotesNewCreateInfo, Error> {
    let tags = NoteRepo::new(get_db()).get_available_tags(50).await?;

    let available_tags: Vec<String> = tags.iter().map(|x| x.tag.clone()).collect();

    Ok(NotesNewCreateInfo {
        tags: available_tags,
        ..Default::default()
    })
} //end func
