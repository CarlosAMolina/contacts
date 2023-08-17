use api::types::database::AllData;
use api::{config_api, handle_errors, oneshot, setup_store, store};
use sqlx;
use sqlx::Row;
use sqlx::postgres::PgPoolOptions;
use std::io::{self, Write};
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    println!("Init integration tests");
    let config = config_api::Config::new().expect("Config can't be set");
    recreate_database(&config).await;
    let store = setup_store(&config).await;
    println!("Init start the api web server");
    let handler = oneshot(store).await;

    // TODO run migrations to create the tables.

    println!("Init test get_contacts");
    test_get_contacts().await;

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
        .await.unwrap();
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

async fn exists_database(config: &config_api::Config, postgres_connection: &sqlx::Pool<sqlx::Postgres>) -> bool {
    let url = format!(
        "postgres://{}:{}@{}:{}",
        config.database_user, config.database_password, config.database_host, config.database_port,
    );
    println!(
        "Init check database {} exists. URL: {}",
        config.database_name, url
    );
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

async fn test_get_contacts() {
    let client = reqwest::Client::new();
    // TODO use config to create the URL
    let res = client
        .get("http://localhost:3030/contacts?query=arlos%20a")
        .send()
        .await
        .unwrap()
        .json::<AllData>()
        .await
        .unwrap();
    println!("{:?}", res); // TODO
                           // TODO assert_eq!(res.id, 1);
                           // TODO assert_eq!(res.title, q.title);
}
