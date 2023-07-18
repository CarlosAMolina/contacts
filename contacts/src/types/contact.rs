use serde::{Deserialize, Serialize};

// TODO remov Contact or AllData
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Contact {
    pub id: usize,
    pub name: String,
    pub surname: String,
    pub nickname: String,
    pub phone: Option<usize>,
    pub phone_description: String,
    pub category: String,
    pub address: String,
    pub email: String,
    pub url: String,
    pub facebook_url: String,
    pub twitter_handle: String,
    pub note: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ContactId(pub String);

#[derive(Debug, Serialize)]
pub struct AllData {
    pub user_id: i32, // TODO move to struct user_id
    pub user_name: Option<String>,
    pub user_surname: Option<String>,
    pub nickname: Option<String>,
    pub phone: Option<i64>, // TODO set correct type for bigint
    pub phone_description: Option<String>,
    pub category: Option<String>,
    pub address: Option<String>,
    pub email: Option<String>,
    pub url: Option<String>,
    pub facebook_url: Option<String>,
    pub twitter_handle: Option<String>,
    pub instagram_handle: Option<String>,
    pub note: Option<String>,
}
