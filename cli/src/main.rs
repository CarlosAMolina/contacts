use config::Config;
use reqwest;
use clap::{Parser, ValueEnum};
use serde::Deserialize;


#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct ConfigArgs {
    api_host: String,
    api_port: u16,
}

impl ConfigArgs {
    pub fn new() -> ConfigArgs {
        let is_app_in_docker = std::env::var("IS_DOCKER_RUNNING")
            .ok()
            .map(|val| val.parse::<bool>())
            .unwrap_or(Ok(false))
            .unwrap();
        let config_file_name = match is_app_in_docker {
            true => "setup-docker.toml",
            false => "setup-local.toml",
        };
        let config = Config::builder()
            .add_source(config::File::with_name(config_file_name))
            .build()
            .unwrap();
        config.try_deserialize::<ConfigArgs>().unwrap()
    }
}


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
    let config = ConfigArgs::new();
    if let Some(id) = cli.id {
        let url = format!("http://{host}:{port}/contacts/{id}",
                          host = config.api_host,
                          port = config.api_port,
                          id = id);
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
                "http://{host}:{port}/contacts?query={query}",
                host = config.api_host,
                port = config.api_port,
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
    summary = format!("{}. ID {:?}", summary, contact.user_id.0);
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


#[cfg(test)]
mod config_tests {
    use super::*;

    // As Rust runs test in parallel, we run two tests in the same function
    // in order to not affect each test when env variables are modified.
    #[test]
    fn config_files_are_detected_correctly() {
        let expected_not_in_docker = ConfigArgs {
            api_host: "localhost".to_string(),
            api_port: 3030,
        };
        let expected_in_docker = ConfigArgs {
            api_host: "172.20.0.6".to_string(),
            api_port: 3030,
        };
        assert_eq!(expected_not_in_docker, ConfigArgs::new());
        std::env::set_var("IS_DOCKER_RUNNING", "true");
        assert_eq!(expected_in_docker, ConfigArgs::new());
    }
}
