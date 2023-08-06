use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Contact {
    pub user_id: i32,
    pub user_name: Option<String>,
    pub user_surname: Option<String>,
    pub nicknames: Vec<String>,
    pub phones: Vec<Phone>,
    pub categories: Vec<String>,
    pub addresses: Vec<String>,
    pub emails: Vec<String>,
    pub urls: Vec<String>,
    pub facebook_urls: Vec<String>,
    pub twitter_handles: Vec<String>,
    pub instagram_handles: Vec<String>,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Phone {
    pub value: i64,
    pub description: Option<String>,
}

