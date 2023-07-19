use std::env;

use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Contact {
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

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let query = args[1].clone();
    let url = format!(
        "http://localhost:3030/contacts?query={query}",
        query = query
    );
    let response = reqwest::get(url).await.unwrap();
    if response.status() != reqwest::StatusCode::OK {
        panic!("Unexpected error: {:?}", response);
    }
    let contacts = response.json::<Vec<Contact>>().await.unwrap();
    for contact in contacts {
        print_contact_short(contact);
    }
}

fn print_contact_short(contact: Contact) {
    let mut summary: String;
    match contact.phone {
        Some(value) => {
            summary = value.to_string();
        }
        None => summary = "".to_string(),
    };
    if let Some(value) = contact.phone_description {
        summary = format!("{} ({})", summary, value);
    }
    if let Some(value) = contact.user_name {
        summary = format!("{} {}", summary, value);
    }
    if let Some(value) = contact.user_surname {
        summary = format!("{} {}", summary, value);
    }
    if let Some(value) = contact.nickname {
        summary = format!("{}. {}", summary, value);
    }
    if let Some(value) = contact.category {
        summary = format!("{}. {}", summary, value);
    }
    summary = format!("{}. ID {}", summary, contact.user_id);
    println!("{}", summary);
}

fn print_contact_all(contact: Contact) {
    println!("## User ID {}", contact.user_id);
    print_option_if_has_value_from_string(contact.user_name, "name".to_string());
    print_option_if_has_value_from_string(contact.user_surname, "surname".to_string());
    print_option_if_has_value_from_string(contact.nickname, "nickname".to_string());
    print_option_if_has_value_from_int(contact.phone, "phone".to_string());
    print_option_if_has_value_from_string(
        contact.phone_description,
        "phone_description".to_string(),
    );
    print_option_if_has_value_from_string(contact.category, "category".to_string());
    print_option_if_has_value_from_string(contact.address, "address".to_string());
    print_option_if_has_value_from_string(contact.email, "email".to_string());
    print_option_if_has_value_from_string(contact.url, "url".to_string());
    print_option_if_has_value_from_string(contact.facebook_url, "facebook url".to_string());
    print_option_if_has_value_from_string(contact.twitter_handle, "twitter handle".to_string());
    print_option_if_has_value_from_string(contact.instagram_handle, "instagram handle".to_string());
    print_option_if_has_value_from_string(contact.note, "note".to_string());
}

fn print_option_if_has_value_from_string(option: Option<String>, prefix_text: String) {
    if let Some(value) = option {
        println!("{}: {}", prefix_text, value);
    }
}

fn print_option_if_has_value_from_int(option: Option<i64>, prefix_text: String) {
    if let Some(value) = option {
        println!("{}: {}", prefix_text, value);
    }
}
