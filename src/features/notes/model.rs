use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct NotesPayload {
    #[serde(alias = "title")]
    pub title: String,

    #[serde(alias = "content")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Default)]
pub struct NotesModel {
    #[serde(alias = "id")]
    pub id: Option<i32>,

    #[serde(alias = "user_id")]
    pub user_id: Option<i32>,

    #[serde(alias = "code")]
    pub code: String, // required

    #[serde(alias = "type")]
    pub r#type: Option<String>,

    #[serde(alias = "title")]
    pub title: String, // required

    #[serde(alias = "content")]
    pub content: String, // required

    #[serde(alias = "visibility")]
    pub visibility: Option<String>,

    #[serde(alias = "is_deleted")]
    pub is_deleted: Option<bool>,

    #[serde(alias = "created_at")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(alias = "updated_at")]
    pub updated_at: Option<DateTime<Utc>>,
}
