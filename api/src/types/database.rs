use serde::Serialize;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub surname: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct AllData {
    pub user_id: i32,
    pub user_name: Option<String>,
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
