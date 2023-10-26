
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUpdatesJsonResponse {
    /// Changed items
    pub items: Option<Vec<i64>>,

    /// Changed profiles
    pub profiles: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub by: String,

    pub dead: Option<bool>,

    pub deleted: Option<bool>,

    pub descendants: Option<i64>,

    pub id: i64,

    pub kids: Option<Vec<i64>>,

    pub parent: Option<i64>,

    pub parts: Option<Vec<i64>>,

    pub poll: Option<i64>,

    pub score: Option<i64>,

    pub text: Option<String>,

    pub time: i64,

    pub title: Option<String>,

    #[serde(rename = "type")]
    pub item_type: String,

    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// The user's optional self-description. HTML
    pub about: Option<String>,

    /// Creation date of the user, in Unix Time
    pub created: Option<i64>,

    pub id: Option<String>,

    /// The user's karma
    pub karma: Option<i64>,

    pub submitted: Option<serde_json::Value>,
}
