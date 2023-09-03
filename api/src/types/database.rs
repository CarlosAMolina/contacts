use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub struct AllData {
    pub user_id: i32,
    pub user_name: String,
    pub user_surname: Option<String>,
    pub nickname: Option<String>,
    pub phone: Option<i64>,
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

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Category {
    pub id: i32,
    pub category: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewUser {
    pub name: String,
    pub surname: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Nickname {
    pub id_user: i32,
    pub nickname: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Phone {
    pub id_user: i32,
    pub phone: i64,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub surname: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct UserCategory {
    pub id_user: i32,
    pub id_category: i32,
}
