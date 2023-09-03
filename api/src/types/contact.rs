use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Contact {
    pub user_id: i32,
    pub user_name: String,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct NewContact {
    pub user_name: String,
    pub user_surname: Option<String>,
    pub nicknames: Vec<String>,
    pub phones: Vec<Phone>,
    pub categories_id: Vec<i32>,
    pub addresses: Vec<String>,
    pub emails: Vec<String>,
    pub urls: Vec<String>,
    pub facebook_urls: Vec<String>,
    pub twitter_handles: Vec<String>,
    pub instagram_handles: Vec<String>,
    pub note: Option<String>,
}

// TODO rename value to phone as is the name used in db.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Phone {
    pub value: i64,
    pub description: Option<String>,
}
