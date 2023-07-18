use std::env;

use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::Pool;
use sqlx::{Postgres, Row};
use tokio;

struct Config {
    database_host: String,
    database_name: String,
    database_password: String,
    database_port: u16,
    database_user: String,
}

#[derive(Debug)]
pub struct AllData {
    user_id: i32, // TODO move to struct user_id
    user_name: Option<String>,
    user_surname: Option<String>,
    nickname: Option<String>,
    phone: Option<i64>, // TODO set correct type for bigint
    phone_description: Option<String>,
    category: Option<String>,
    address: Option<String>,
    email: Option<String>,
    url: Option<String>,
    facebook_url: Option<String>,
    twitter_handle: Option<String>,
    instagram_handle: Option<String>,
    note: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = Config {
        database_host: "localhost".to_string(),
        database_name: "contacts".to_string(),
        database_password: "pw".to_string(),
        database_port: 5432,
        database_user: "postgres".to_string(),
    };
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database_user,
        config.database_password,
        config.database_host,
        config.database_port,
        config.database_name
    );
    let db_connection = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let term_to_search = args[1].clone();
    let all_db_data = get_all_db_data(&db_connection, term_to_search).await;
    for row in all_db_data {
        print_all_data_row(row);
    }
    Ok(())
}

async fn get_all_db_data(db_connection: &Pool<Postgres>, term_to_search: String) -> Vec<AllData> {
    println!("Init get all data");
    let term_to_search = format!("%{}%", term_to_search.to_lowercase());
    sqlx::query(
        "SELECT * from contacts.all_data
WHERE LOWER(name) LIKE $1
OR LOWER(surname) LIKE $1
OR LOWER(nickname) LIKE $1
OR CAST(phone AS VARCHAR) LIKE $1
OR LOWER(phone_description) LIKE $1
OR LOWER(category) LIKE $1
OR LOWER(address) LIKE $1
OR LOWER(email) LIKE $1
OR LOWER(url) LIKE $1
OR LOWER(facebook_url) LIKE $1
OR LOWER(twitter_handle) LIKE $1
OR LOWER(instagram_handle) LIKE $1
OR LOWER(note) LIKE $1
;
",
    )
    .bind(term_to_search)
    .map(|row: PgRow| AllData {
        user_id: row.get("id"),
        user_name: row.get("name"),
        user_surname: row.get("surname"),
        nickname: row.get("nickname"),
        phone: row.get("phone"),
        phone_description: row.get("phone_description"),
        category: row.get("category"),
        address: row.get("address"),
        email: row.get("email"),
        url: row.get("url"),
        facebook_url: row.get("facebook_url"),
        twitter_handle: row.get("twitter_handle"),
        instagram_handle: row.get("instagram_handle"),
        note: row.get("note"),
    })
    .fetch_all(db_connection)
    .await
    .unwrap()
}

fn print_all_data_row(row: AllData) {
    println!("## User ID {}", row.user_id);
    print_option_if_has_value_from_string(row.user_name, "name".to_string());
    print_option_if_has_value_from_string(row.user_surname, "surname".to_string());
    print_option_if_has_value_from_string(row.nickname, "nickname".to_string());
    print_option_if_has_value_from_int(row.phone, "phone".to_string());
    print_option_if_has_value_from_string(row.phone_description, "phone_description".to_string());
    print_option_if_has_value_from_string(row.category, "category".to_string());
    print_option_if_has_value_from_string(row.address, "address".to_string());
    print_option_if_has_value_from_string(row.email, "email".to_string());
    print_option_if_has_value_from_string(row.url, "url".to_string());
    print_option_if_has_value_from_string(row.facebook_url, "facebook url".to_string());
    print_option_if_has_value_from_string(row.twitter_handle, "twitter handle".to_string());
    print_option_if_has_value_from_string(row.instagram_handle, "instagram handle".to_string());
    print_option_if_has_value_from_string(row.note, "note".to_string());
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
