use api::handle_errors::Error;
use api::store::Store;
use api::types::contact::Contact;
use api::{config_api, oneshot, setup_store};
use sqlx;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use std::fmt::format;
use std::io::{self, Write};
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Init integration tests");
    let config = config_api::Config::new().expect("Config can't be set");
    recreate_database(&config).await;
    let store = setup_store(&config).await;
    println!("Init start the api web server");
    let handler = oneshot(&store).await;
    add_db_schema(&store).await;
    run_migrations(&store).await;
    insert_db_data(&store).await;
    let url_api = format!("http://{:?}.{:?}.{:?}.{:?}:{:?}",
                     config.api_host[0],
                     config.api_host[1],
                     config.api_host[2],
                     config.api_host[3],
                     config.api_port,
                     );
    test_get_contacts(&url_api).await;
    test_get_contacts_if_invalid_path(&url_api).await;
    test_get_contacts_if_no_results(&url_api).await;
    test_get_contacts_if_missing_parameters(&url_api).await;
    test_get_contacts_if_missing_parameters_and_url_ends_in_slash(&url_api).await;
    test_get_contact_by_id(&url_api).await;
    test_get_contact_by_id_if_id_does_not_exist(&url_api).await;
    println!("Init shut down the api web server");
    let _ = handler.sender.send(1);
    Ok(())
}

async fn recreate_database(config: &config_api::Config) {
    let postgres_url = format!(
        "postgres://{}:{}@{}:{}",
        config.database_user, config.database_password, config.database_host, config.database_port,
    );
    println!("Init create postgres connection. URL: {}", postgres_url);
    let postgres_connection = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_url)
        .await
        .unwrap();
    if exists_database(&config, &postgres_connection).await {
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.database_user,
            config.database_password,
            config.database_host,
            config.database_port,
            config.database_name
        );
        println!(
            "Init delete database {}. URL: {}",
            config.database_name, url
        );
        let command = Command::new("sqlx")
            .arg("database")
            .arg("drop")
            .arg("--database-url")
            .arg(&url)
            .arg("-y")
            .output()
            .expect("sqlx command failed to start");
        if command.status.code().unwrap() != 0 {
            panic!("Unsucessful command: {:?}", command);
        }
        io::stdout().write_all(&command.stderr).unwrap();
    } else {
        println!("The database {} does not exist", config.database_name);
    }
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database_user,
        config.database_password,
        config.database_host,
        config.database_port,
        config.database_name
    );
    println!(
        "Init create database {}. URL: {}",
        config.database_name, url
    );
    let command = Command::new("sqlx")
        .arg("database")
        .arg("create")
        .arg("--database-url")
        .arg(url)
        .output()
        .expect("sqlx command failed to start");
    if command.status.code().unwrap() != 0 {
        panic!("Unsucessful command: {:?}", command);
    }
    io::stdout().write_all(&command.stderr).unwrap();
    if !exists_database(&config, &postgres_connection).await {
        panic!("The database has not been created");
    }
}

async fn exists_database(
    config: &config_api::Config,
    postgres_connection: &sqlx::Pool<sqlx::Postgres>,
) -> bool {
    println!("Init check database {} exists", config.database_name);
    let database_names: Vec<_> = sqlx::query("SELECT datname FROM pg_database")
        .map(|row: sqlx::postgres::PgRow| row.get::<String, _>("datname").to_string())
        .fetch_all(postgres_connection)
        .await
        .unwrap();
    if database_names.contains(&config.database_name) {
        println!("The database {} exists", config.database_name);
        true
    } else {
        println!("The database {} does not exist", config.database_name);
        false
    }
}

async fn add_db_schema(store: &Store) {
    println!("Init create schema");
    sqlx::query("CREATE SCHEMA IF NOT EXISTS contacts")
        //.execute(&postgres_connection)
        .execute(&store.clone().connection)
        .await
        .unwrap();
}

async fn run_migrations(store: &Store) {
    println!("Init migrations");
    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .unwrap();
    // TODO .map_err(|e| handle_errors::Error::MigrationError(e))?;
    assert_migrations_have_correctly_executed(&store.connection).await;
}

async fn assert_migrations_have_correctly_executed(
    postgres_connection: &sqlx::Pool<sqlx::Postgres>,
) {
    println!("Init assert migrations have correctly executed");
    println!("Init get all tables");
    let query = "SELECT table_name FROM information_schema.tables WHERE table_schema = 'contacts'";
    let mut table_names: Vec<_> = sqlx::query(query)
        .map(|row: sqlx::postgres::PgRow| row.get::<String, _>("table_name").to_string())
        .fetch_all(postgres_connection)
        .await
        .unwrap();
    table_names.sort();
    println!("Table names ({}): {:?}", table_names.len(), table_names);
    let expected_table_names = vec![
        "addresses".to_string(),
        "all_data".to_string(),
        "categories".to_string(),
        "emails".to_string(),
        "facebook".to_string(),
        "instagram".to_string(),
        "nicknames".to_string(),
        "notes".to_string(),
        "phones".to_string(),
        "twitter".to_string(),
        "urls".to_string(),
        "users".to_string(),
        "users_categories".to_string(),
    ];
    if expected_table_names == table_names {
        println!("The tables have been created correctly");
    } else {
        println!(
            "Expected table names ({}): {:?}",
            expected_table_names.len(),
            expected_table_names
        );
        panic!("Table names do not match the expected ones");
    }
}

async fn insert_db_data(store: &Store) {
    println!("Init insert data in db");
    // TODO use api methods
    sqlx::query(
        "INSERT INTO contacts.users (id, name, surname)
        VALUES (1, 'John', 'Doe')",
    )
    .execute(&store.connection)
    .await
    .unwrap();
}

async fn test_get_contacts(url_api: &String) {
    println!("Init test_get_contacts");
    let client = reqwest::Client::new();
    let url = format!("{url_api}/contacts?query=ohn");
    let response = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<Vec<Contact>>()
        .await
        .unwrap();
    let expected_result = vec![Contact {
        user_id: 1,
        user_name: Some("John".to_string()),
        user_surname: Some("Doe".to_string()),
        nicknames: vec![],
        phones: vec![],
        categories: vec![],
        addresses: vec![],
        emails: vec![],
        urls: vec![],
        facebook_urls: vec![],
        twitter_handles: vec![],
        instagram_handles: vec![],
        note: None,
    }];
    assert_eq!(expected_result, response);
    println!("✓");
}

async fn test_get_contacts_if_invalid_path(url_api: &String) {
    println!("Init test_get_contacts_if_invalid_path");
    let client = reqwest::Client::new();
    let url = format!("{url_api}/contacts/a");
    let response = client
        .get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let expected_result = Error::RouteNotFound.to_string();
    assert_eq!(response, expected_result);
    println!("✓");
}

async fn test_get_contacts_if_no_results(url_api: &String) {
    println!("Init test_get_contacts_if_no_results");
    let client = reqwest::Client::new();
    let url = format!("{url_api}/contacts?query=asdfasdfsadf");
    let response = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<Vec<Contact>>()
        .await
        .unwrap();
    assert!(response.is_empty());
    println!("✓");
}

async fn test_get_contacts_if_missing_parameters(url_api: &String) {
    println!("Init test_get_contacts_if_missing_parameters");
    let client = reqwest::Client::new();
    let url = format!("{url_api}/contacts");
    let response = client
        .get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let expected_result = format!("{}", Error::MissingParameters);
    assert_eq!(response, expected_result);
    println!("✓");
}

async fn test_get_contacts_if_missing_parameters_and_url_ends_in_slash(url_api: &String) {
    println!("Init test_get_contacts_if_missing_parameters_and_url_ends_in_slash");
    let client = reqwest::Client::new();
    let url = format!("{url_api}/contacts/");
    let response = client
        .get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let expected_result = format!("{}", Error::MissingParameters);
    assert_eq!(response, expected_result);
    println!("✓");
}


async fn test_get_contact_by_id(url_api: &String) {
    println!("Init test_get_contact_by_id");
    let client = reqwest::Client::new();
    let url = format!("{url_api}/contacts/1");
    let response = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<Contact>()
        .await
        .unwrap();
    let expected_result = Contact {
        user_id: 1,
        user_name: Some("John".to_string()),
        user_surname: Some("Doe".to_string()),
        nicknames: vec![],
        phones: vec![],
        categories: vec![],
        addresses: vec![],
        emails: vec![],
        urls: vec![],
        facebook_urls: vec![],
        twitter_handles: vec![],
        instagram_handles: vec![],
        note: None,
    };
    assert_eq!(expected_result, response);
    println!("✓");
}

async fn test_get_contact_by_id_if_id_does_not_exist(url_api: &String) {
    println!("Init test_get_contact_by_id_if_id_does_not_exist");
    let client = reqwest::Client::new();
    let url = format!("{url_api}/contacts/999");
    let response = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<Option<Contact>>()
        .await
        .unwrap();
    let expected_result = None;
    assert_eq!(expected_result, response);
    println!("✓");
}

