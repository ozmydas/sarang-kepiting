use std::result;

use chrono::Local;
use sqlx::{PgPool, Postgres, Transaction};

use super::model::{self, NotesModel, NotesTagsModel};

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
    pub async fn get_all(&self, limit: i32, offset: i32) -> Result<Vec<model::NotesList>, sqlx::Error> {
        let record = sqlx::query_as::<_, model::NotesList>(
            format!(
                "SELECT nl.id, nl.code, nl.title, nl.snippet, nl.updated_at, string_agg(nt.tag, ',') as tags from {} nl left join notes_tags nt on nt.note_code = nl.code WHERE is_deleted = false group by nl.id, nl.code, nl.title, nl.snippet, nl.updated_at ORDER BY nl.id DESC limit $1 OFFSET $2",
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
    pub async fn get_single(&self, code: String) -> Result<model::NotesModel, sqlx::Error> {
        let record = sqlx::query_as::<_, model::NotesModel>(
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
    pub async fn create(
        &self,
        item: model::NotesModel,
        tx: &mut Transaction<'static, Postgres>,
    ) -> Result<model::NotesModel, sqlx::Error> {
        let record = sqlx::query_as::<_, model::NotesModel>(
            format!(
                "INSERT INTO {} (code, title, content) VALUES ($1, $2, $3) RETURNING *",
                self.table
            )
            .as_str(),
        )
        .bind(item.code)
        .bind(item.title)
        .bind(item.content)
        .fetch_one(&mut **tx)
        .await?;

        Ok(record)
    } // end func

    /// update data with transaction
    pub async fn update(
        &self,
        item: model::NotesModel,
        tx: &mut Transaction<'static, Postgres>,
    ) -> Result<model::NotesModel, sqlx::Error> {
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

        let mut record = sqlx::query_as::<_, model::NotesModel>(&sql).bind(item.code);

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
    pub async fn delete(&self, code: String) -> Result<model::NotesModel, sqlx::Error> {
        let record = sqlx::query_as::<_, model::NotesModel>(
            format!("DELETE FROM {} WHERE code = $1 RETURNING *", self.table,).as_str(),
        )
        .bind(code)
        .fetch_one(self.db)
        .await?;

        Ok(record)
    } // end func

    /// delete data
    pub async fn delete_soft(&self, code: String) -> Result<model::NotesModel, sqlx::Error> {
        let record = sqlx::query_as::<_, model::NotesModel>(
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

    pub async fn get_available_tags(&self, limit: i32) -> Result<Vec<model::NotesTagsModel>, sqlx::Error> {
        let record = sqlx::query_as::<_, model::NotesTagsModel>(
            format!("select distinct(tag) from notes_tags nt where created_by is null LIMIT $1")
                .as_str(),
        )
        .bind(limit)
        .fetch_all(self.db)
        .await?;

        Ok(record)
    } // end func

    pub async fn remove_tags(
        &self,
        note_code: String,
        tx: &mut Transaction<'static, Postgres>,
    ) -> Result<(), sqlx::Error> {
        let record = sqlx::query(format!("DELETE FROM notes_tags WHERE note_code = $1").as_str())
            .bind(note_code)
            .execute(&mut **tx)
            .await?;

        Ok(())
    } // end func

    /// insert tags
    pub async fn insert_tags(
        &self,
        items: Vec<model::NotesTagsModel>,
        tx: &mut Transaction<'static, Postgres>,
    ) -> Result<(), sqlx::Error> {
        // presql
        let mut values = String::new();
        let mut i = 0;

        for item in &items {
            i += 1;
            let a = i;

            i += 1;
            let b = i;

            i += 1;
            let c = i;

            values.push_str(&format!("(${}, ${}, ${}),", a, b, c));
        }

        // fix last comma
        values.pop();

        let sql = format!(
            "INSERT INTO {} (note_code, tag, created_by) VALUES {}",
            "notes_tags", values
        );

        let mut record = sqlx::query(&sql);

        // loop for bind
        for item in items {
            record = record.bind(item.note_code);
            record = record.bind(item.tag);
            record = record.bind(item.created_by);
        }

        let result = record.execute(&mut **tx).await?;

        Ok(())
    } // end func
}
