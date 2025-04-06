use anyhow::Error;
use uuid::Uuid;

use crate::{config::database::get_db, utils::custom_trait::RepositoryTrait};

use super::{model::{NotesModel, NotesPayload}, repository::{self, NoteRepo}};

pub async fn list_service() -> Result<NotesModel, Error> {
    Ok(NoteRepo::rows().await?)
} //end func

pub async  fn save_service(payload: NotesPayload) -> Result<NotesModel, Error> {
    
    let code = Uuid::new_v4().to_string();

    let new_item = NoteRepo {
        db: &get_db(),
        code: &code,
        title: &payload.title,
        content: &payload.content,
    };

    Ok(new_item.create().await?)
} //end func

pub fn detail_service() -> Option<String> {
    todo!()
} //end func

pub fn update_service() -> Option<String> {
    todo!()
} //end func

pub fn delete_service() -> Option<String> {
    todo!()
} //end func
