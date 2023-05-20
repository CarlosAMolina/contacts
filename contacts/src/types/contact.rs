use serde::{Deserialize, Serialize};

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
