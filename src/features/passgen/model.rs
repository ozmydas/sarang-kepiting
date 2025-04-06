use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PassgenPayload {
    #[serde(default, alias = "over12")]
    pub over12: bool,

    #[serde(default, alias = "includeUppercase")]
    pub include_uppercase: bool,

    #[serde(default, alias = "includeNumber")]
    pub include_number: bool,

    #[serde(default, alias = "includeSpecialChars")]
    pub include_special_chars: bool,

    #[serde(default, alias = "nonEnglishWord")]
    pub non_english_word: bool,
}
