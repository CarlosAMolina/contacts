use config::Config;
use reqwest;
use clap::{Parser, ValueEnum};
use serde;

mod types;

use crate::types::contact::{AllData, Contact, Phone};


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
        let contact = response.json::<Contact>().await.unwrap();
        print_contact(contact);
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
            let all_data_vec= response.json::<Vec<AllData>>().await.unwrap();
            if is_long_format {
                for all_data in all_data_vec{
                    print_all_data_all(all_data);
                }
            } else {
                for all_data in all_data_vec{
                    print_all_data_summary(all_data);
                }
            }
        }
    }
}

fn print_all_data_summary(all_data: AllData) {
    let mut summary: String;
    match all_data.phone {
        Some(value) => {
            summary = value.to_string();
        }
        None => summary = "".to_string(),
    };
    if let Some(value) = all_data.phone_description {
        summary = format!("{} ({})", summary, value);
    }
    if let Some(value) = all_data.user_name {
        summary = format!("{} {}", summary, value);
    }
    if let Some(value) = all_data.user_surname {
        summary = format!("{} {}", summary, value);
    }
    if let Some(value) = all_data.nickname {
        summary = format!("{}. {}", summary, value);
    }
    if let Some(value) = all_data.category {
        summary = format!("{}. {}", summary, value);
    }
    summary = format!("{}. ID {:?}", summary, all_data.user_id);
    println!("{}", summary);
}

fn print_all_data_all(all_data: AllData) {
    println!("## User ID {:?}", all_data.user_id);
    print_option_if_has_value(all_data.user_name, "name".to_string());
    print_option_if_has_value(all_data.user_surname, "surname".to_string());
    print_option_if_has_value(all_data.nickname, "nickname".to_string());
    print_option_if_has_value(all_data.phone, "phone".to_string());
    print_option_if_has_value(all_data.phone_description, "phone_description".to_string());
    print_option_if_has_value(all_data.category, "category".to_string());
    print_option_if_has_value(all_data.address, "address".to_string());
    print_option_if_has_value(all_data.email, "email".to_string());
    print_option_if_has_value(all_data.url, "url".to_string());
    print_option_if_has_value(all_data.facebook_url, "facebook url".to_string());
    print_option_if_has_value(all_data.twitter_handle, "twitter handle".to_string());
    print_option_if_has_value(all_data.instagram_handle, "instagram handle".to_string());
    print_option_if_has_value(all_data.note, "note".to_string());
}

fn print_option_if_has_value<T: std::fmt::Display>(option: Option<T>, prefix_text: String) {
    if let Some(value) = option {
        println!("{}: {}", prefix_text, value);
    }
}


fn print_contact(contact: Contact) {
    print_option_if_has_value(contact.user_name, "name".to_string());
    print_option_if_has_value(contact.user_surname, "surname".to_string());
    print_vector_if_not_empty(contact.nicknames, "nicknames".to_string());
    print_phones_if_not_empty(contact.phones);
    print_vector_if_not_empty(contact.categories, "categories".to_string());
    print_vector_if_not_empty(contact.addresses, "addresses".to_string());
    print_vector_if_not_empty(contact.emails, "emails".to_string());
    print_vector_if_not_empty(contact.urls, "urls".to_string());
    print_vector_if_not_empty(contact.facebook_urls, "facebook urls".to_string());
    print_vector_if_not_empty(contact.twitter_handles, "twitter handles".to_string());
    print_vector_if_not_empty(contact.instagram_handles, "instagram handles".to_string());
    print_option_if_has_value(contact.note, "note".to_string());
    println!("user ID: {:?}", contact.user_id);
}

fn print_vector_if_not_empty(array: Vec<String>, prefix_text: String) {
    if !array.is_empty() {
        println!("{}: {}", prefix_text, array.join(", "));
    }
}

fn print_phones_if_not_empty(phones: Vec<Phone>) {
    if !phones.is_empty() {
        println!("phones:");
        for phone in phones {
            if let Some(description) = phone.description {
                println!("  {:?} ({})", phone.value, description);
            } else {
                println!("  {}", phone.value);
            }
        }
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
