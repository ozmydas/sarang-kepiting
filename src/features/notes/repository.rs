use sqlx::{query, Pool, Postgres};
use uuid::Uuid;

use crate::{config::database::get_db, utils::custom_trait};

use super::model::NotesModel;

pub struct NoteRepo<'a> {
    pub db: &'a Pool<Postgres>,
    pub code: &'a str,
    pub title: &'a str,
    pub content: &'a str,
}


impl<'a> custom_trait::RepositoryTrait for NoteRepo<'a> {
    type Row = NotesModel;

    // create data
    async fn create(&self) -> Result<Self::Row, sqlx::Error> {
        let record = sqlx::query_as::<_, NotesModel>(
            "INSERT INTO notes_lists (code, title, content) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(self.code)
        .bind(self.title)
        .bind(self.content)
        .fetch_one(self.db)
        .await?;

        Ok(record)
    }

    async fn rows() -> Result<Self::Row, sqlx::Error> {
        todo!()
    }

    async fn detail() -> Result<Self::Row, sqlx::Error> {
        todo!()
    } // end func

    async fn update(&self) -> Result<Self::Row, sqlx::Error> {
        todo!()
    }

    async fn delete(&self) -> Result<Self::Row, sqlx::Error> {
        todo!()
    }
}
