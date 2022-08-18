use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct DownloadImage {
    pub url: String,
    pub size: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Dictionary {
    pub entries: HashMap<String, DictionaryEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub md5: String,
    pub ext: String,
    pub url: String,
    pub rating: String,
    pub id: i64
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub tags: String,
    pub output: Option<String>,
    pub limit: u8,
    pub sfw: Option<bool>,
    pub dictionary: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Posts {
    #[serde(rename = "posts")]
    pub posts: Vec<Post>,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "file")]
    pub file: File,

    #[serde(rename = "rating")]
    pub rating: String,

    #[serde(rename = "sources")]
    pub sources: Vec<String>,

    #[serde(rename = "pools")]
    pub pools: Vec<Option<serde_json::Value>>,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "is_favorited")]
    pub is_favorited: bool,

    #[serde(rename = "has_notes")]
    pub has_notes: bool,

    #[serde(rename = "duration")]
    pub duration: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "width")]
    pub width: i64,

    #[serde(rename = "height")]
    pub height: i64,

    #[serde(rename = "ext")]
    pub ext: String,

    #[serde(rename = "size")]
    pub size: i64,

    #[serde(rename = "md5")]
    pub md5: String,

    #[serde(rename = "url")]
    pub url: String,
}
