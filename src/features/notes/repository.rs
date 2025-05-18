use std::result;

use chrono::Local;
use sqlx::{PgPool, Postgres, Transaction};

use super::model::{NotesModel, NotesTagsModel};

pub struct NoteRepo<'a> {
    pub db: &'static PgPool,
    table: &'a str,
}

impl NoteRepo<'_> {
    /// set db
    pub fn new(db: &'static PgPool) -> Self {
        Self {
            db,
            table: "notes_lists",
        }
    } // end func

    // get data all
    pub async fn get_all(&self, limit: i32, offset: i32) -> Result<Vec<NotesModel>, sqlx::Error> {
        let record = sqlx::query_as::<_, NotesModel>(
            format!(
                "SELECT * from {} WHERE is_deleted = false ORDER BY ID DESC LIMIT $1 OFFSET $2",
                self.table
            )
            .as_str(),
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;

        Ok(record)
    } // end func

    /// get data single
    pub async fn get_single(&self, code: String) -> Result<NotesModel, sqlx::Error> {
        let record = sqlx::query_as::<_, NotesModel>(
            format!(
                "SELECT * from {} WHERE code = $1 AND is_deleted = false",
                self.table
            )
            .as_str(),
        )
        .bind(code)
        .fetch_one(self.db)
        .await?;

        Ok(record)
    } // end func

    /// create data
    pub async fn create(&self, item: NotesModel) -> Result<NotesModel, sqlx::Error> {
        let record = sqlx::query_as::<_, NotesModel>(
            format!(
                "INSERT INTO {} (code, title, content) VALUES ($1, $2, $3) RETURNING *",
                self.table
            )
            .as_str(),
        )
        .bind(item.code)
        .bind(item.title)
        .bind(item.content)
        .fetch_one(self.db)
        .await?;

        Ok(record)
    } // end func

    /// update data with transaction
    pub async fn update(&self, item: NotesModel,  tx: &mut Transaction<'static, Postgres>) -> Result<NotesModel, sqlx::Error> {
        let sql = format!(
            "UPDATE {}
        SET
            type = type,
            title = $2,
            content = $3,
            visibility = $4,
            updated_at = $5
        WHERE
            code = $1
        RETURNING *",
            self.table
        );

        let mut record = sqlx::query_as::<_, NotesModel>(&sql).bind(item.code);

        // nanti bisa pake loop
        record = record.bind(item.title);
        record = record.bind(item.content);
        record = record.bind(item.visibility);
        record = record.bind(Local::now());

        // let result = record.fetch_one(self.db).await?;
        let result = record.fetch_one(&mut **tx).await?;

        Ok(result)
    } // end func

    /// delete data
    pub async fn delete(&self, code: String) -> Result<NotesModel, sqlx::Error> {
        let record = sqlx::query_as::<_, NotesModel>(
            format!("DELETE FROM {} WHERE code = $1 RETURNING *", self.table,).as_str(),
        )
        .bind(code)
        .fetch_one(self.db)
        .await?;

        Ok(record)
    } // end func

    /// delete data
    pub async fn delete_soft(&self, code: String) -> Result<NotesModel, sqlx::Error> {
        let record = sqlx::query_as::<_, NotesModel>(
            format!(
                "UPDATE {} SET is_deleted = true, updated_at = $2 WHERE code = $1 RETURNING *",
                self.table
            )
            .as_str(),
        )
        .bind(code)
        .bind(Local::now())
        .fetch_one(self.db)
        .await?;

        Ok(record)
    } // end func


    /****************************/

    // get some needed data for create new notes like available tags etc

    pub async fn get_available_tags(&self, limit: i32) -> Result<Vec<NotesTagsModel>, sqlx::Error> {
        let record = sqlx::query_as::<_, NotesTagsModel>(
            format!(
                "select distinct(tag) from notes_tags nt where created_by is null LIMIT $1"
            )
            .as_str(),
        )
        .bind(limit)
        .fetch_all(self.db)
        .await?;

        Ok(record)
    } // end func

    pub async fn remove_tags(&self, note_code: String, tx: &mut Transaction<'static, Postgres>) -> Result<(), sqlx::Error> {
        let record = sqlx::query(
            format!("DELETE FROM notes_tags WHERE note_code = $1").as_str(),
        )
        .bind(note_code)
        .execute(&mut **tx)
        .await?;

        Ok(())
    } // end func
}
