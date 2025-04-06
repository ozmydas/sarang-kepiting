use sqlx::{query, PgPool, Pool, Postgres};
use std::task::Poll;
use uuid::Uuid;

use super::model::NotesModel;
use crate::{config::database::get_db, utils::custom_trait};

pub struct NoteRepo {
    pub db: &'static PgPool,
}

impl NoteRepo {
    // set db
    pub fn new(db: &'static PgPool) -> Self {
        Self { db }
    } // end func

    // create data
    pub async fn create(&self, item: NotesModel) -> Result<NotesModel, sqlx::Error> {
        let record = sqlx::query_as::<_, NotesModel>(
            "INSERT INTO notes_lists (code, title, content) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(item.code)
        .bind(item.title)
        .bind(item.content)
        .fetch_one(self.db)
        .await?;

        Ok(record)
    } // end func

     // get data
     pub async fn get_by_code(&self, code: String) -> Result<NotesModel, sqlx::Error> {
        let record = sqlx::query_as::<_, NotesModel>(
            "SELECT * from notes_lists WHERE code = $1",
        )
        .bind(code)
        .fetch_one(self.db)
        .await?;

        Ok(record)
    } // end func
}
