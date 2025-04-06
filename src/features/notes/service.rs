use anyhow::Error;
use uuid::Uuid;

use crate::config::database::get_db;

use super::{
    model::{NotesModel, NotesPayload},
    repository::{self, NoteRepo},
};

pub async fn list_service() -> Result<NotesModel, Error> {
    // Ok(NoteRepo::rows().await?)
    todo!()
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
    Ok(NoteRepo::new(get_db()).get_by_code(code).await?)
} //end func

pub fn update_service() -> Option<String> {
    todo!()
} //end func

pub fn delete_service() -> Option<String> {
    todo!()
} //end func
