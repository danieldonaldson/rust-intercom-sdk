use std::{collections::HashMap, default};

use serde::{de, Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Pages {
    pub page: usize,
    pub per_page: usize,
    pub total_pages: usize,
    pub next: Option<NextObject>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NextObject {
    pub page: i64,
    pub starting_after: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TagData {
    #[serde(rename = "type")]
    pub tag_type: String,
    pub id: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Tags {
    pub data: Vec<TagData>,
    pub url: String,
    pub total_count: i64,
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ListResponse<T> {
    pub total_count: usize,
    pub pages: Pages,
    pub data: Vec<T>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CatchAll {
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}
