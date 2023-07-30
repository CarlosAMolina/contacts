use clap::{Parser, ValueEnum};
use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Contact {
    pub user_id: UserId,
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

#[derive(Debug, Deserialize)]
pub struct UserId(pub i32);

#[derive(Parser)]
#[command(arg_required_else_help = true, version)]
struct Cli {
    search_term: Option<Vec<String>>,

    #[arg(short, long)]
    id: Option<i32>,

    #[arg(short, long, value_enum)]
    format: Option<Format>,
}

#[derive(Clone, Debug, ValueEnum)]
enum Format {
    /// Show all contact info.
    Long,
    /// Show a summary of the contact info.
    Short,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if let Some(id) = cli.id {
        let url = format!("http://localhost:3030/contacts/{id}", id = id);
        let response = reqwest::get(url).await.unwrap();
        if response.status() != reqwest::StatusCode::OK {
            panic!("Unexpected error: {:?}", response);
        }
        let contacts = response.json::<Vec<Contact>>().await.unwrap();
        for contact in contacts {
            print_contact_all(contact);
        }
    } else {
        if let Some(search_term_vector) = &cli.search_term {
            let search_term = search_term_vector.join(" ");
            println!("Init search term {}", search_term);
            let is_long_format = match cli.format {
                Some(Format::Long) => true,
                _ => false,
            };
            let url = format!(
                "http://localhost:3030/contacts?query={query}",
                query = search_term
            );
            let response = reqwest::get(url).await.unwrap();
            if response.status() != reqwest::StatusCode::OK {
                panic!("Unexpected error: {:?}", response);
            }
            let contacts = response.json::<Vec<Contact>>().await.unwrap();
            if is_long_format {
                for contact in contacts {
                    print_contact_all(contact);
                }
            } else {
                for contact in contacts {
                    print_contact_short(contact);
                }
            }
        }
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
    summary = format!("{}. ID {:?}", summary, contact.user_id);
    println!("{}", summary);
}

fn print_contact_all(contact: Contact) {
    println!("## User ID {:?}", contact.user_id.0);
    print_option_if_has_value(contact.user_name, "name".to_string());
    print_option_if_has_value(contact.user_surname, "surname".to_string());
    print_option_if_has_value(contact.nickname, "nickname".to_string());
    print_option_if_has_value(contact.phone, "phone".to_string());
    print_option_if_has_value(contact.phone_description, "phone_description".to_string());
    print_option_if_has_value(contact.category, "category".to_string());
    print_option_if_has_value(contact.address, "address".to_string());
    print_option_if_has_value(contact.email, "email".to_string());
    print_option_if_has_value(contact.url, "url".to_string());
    print_option_if_has_value(contact.facebook_url, "facebook url".to_string());
    print_option_if_has_value(contact.twitter_handle, "twitter handle".to_string());
    print_option_if_has_value(contact.instagram_handle, "instagram handle".to_string());
    print_option_if_has_value(contact.note, "note".to_string());
}

fn print_option_if_has_value<T: std::fmt::Display>(option: Option<T>, prefix_text: String) {
    if let Some(value) = option {
        println!("{}: {}", prefix_text, value);
    }
}
