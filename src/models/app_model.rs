use std::{fmt, str::FromStr};

use serde::{de, Deserialize, Deserializer};

#[derive(Deserialize, Default)]
pub struct Pagination {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub page: Option<i32>,
    pub size: Option<i32>,
    pub keyword: Option<String>,
}

/// Serde deserialization decorator to map empty Strings to None,
/// referensi : https://github.com/tokio-rs/axum/blob/main/examples/query-params-with-empty-strings/src/main.rs
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}