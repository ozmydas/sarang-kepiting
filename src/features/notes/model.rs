use chrono::{DateTime, Local, Utc};
use chrono_tz::Asia::Jakarta;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct NotesPayload {
    #[serde(alias = "title")]
    pub title: String,

    #[serde(alias = "content")]
    pub content: String,

    #[serde(alias = "visibility", default = "default_visibility")]
    pub visibility: String,

    #[serde(alias = "tags")]
    pub tags: Vec<String>,
}

fn default_visibility() -> String {
    "public".to_string()
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

    #[serde(alias = "snippet")]
    pub snippet: Option<String>,

    #[serde(alias = "visibility")]
    pub visibility: String,

    #[serde(alias = "is_deleted")]
    pub is_deleted: Option<bool>,

    #[serde(alias = "created_at")]
    pub created_at: Option<DateTime<Local>>,

    #[serde(alias = "updated_at")]
    pub updated_at: Option<DateTime<Local>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Default)]
pub struct NotesTagsModel {
    #[serde(alias = "id")]
    #[sqlx(default)]
    pub id: Option<i32>,

    #[serde(alias = "note_code")]
    #[sqlx(default)]
    pub note_code: String, // required

    #[serde(alias = "tag")]
    pub tag: String, // required

    #[serde(alias = "created_by")]
    #[sqlx(default)]
    pub created_by: Option<i32>,

    #[serde(alias = "created_at")]
    #[sqlx(default)]
    pub created_at: Option<DateTime<Local>>, // timestamp with time zone
}

#[derive(Debug, Serialize, Deserialize, FromRow, Default)]
pub struct NotesNewCreateInfo {
    #[serde(rename = "available_tags")]
    pub tags: Vec<String>,
    #[serde(rename = "available_folders")]
    pub folder: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Default)]
pub struct NotesList {
    #[serde(alias = "id")]
    pub id: Option<i32>,

    #[serde(alias = "code")]
    pub code: String, // required

    #[serde(alias = "title")]
    pub title: String, // required

    #[serde(alias = "tags", serialize_with = "tags_modifier", default)]
    #[sqlx(default)]
    pub tags: Option<String>,

    #[serde(alias = "snippet")]
    pub snippet: Option<String>,

    #[serde(alias = "updated_at")]
    pub updated_at: Option<DateTime<Local>>,
}

fn tags_modifier<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(value) => {
            let array_tags: Vec<String> = value
                .split(',')
                .map(|part| part.trim().to_string()) // trim spaces and convert to String
                .collect();
            array_tags.serialize(serializer)
        }
        None => serializer.serialize_none(),
    }

    // let forced_vec = vec!["hello".to_string(), "world".to_string()];
}
