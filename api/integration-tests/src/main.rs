use api::{config_api, handle_errors, oneshot, setup_store, store};
use sqlx;
use sqlx::Row;
use std::io::{self, Write};
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    println!("Init integration tests");
    let config = config_api::Config::new().expect("Config can't be set");

    let store = setup_store(&config).await;

    recreate_database(&config, &store).await;
    println!("Init start the api web server");
    let handler = oneshot(store).await;

    println!("Init shut down the api web server");
    let _ = handler.sender.send(1);

    Ok(())
}

async fn recreate_database(config: &config_api::Config, store: &store::Store) {
    if exists_database(&config, &store).await {
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
        let s = Command::new("sqlx")
            .arg("database")
            .arg("drop")
            .arg("--database-url")
            .arg(&url)
            .arg("-y")
            .output()
            .expect("sqlx command failed to start");
        io::stdout().write_all(&s.stderr).unwrap();
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
    let s = Command::new("sqlx")
        .arg("database")
        .arg("create")
        .arg("--database-url")
        .arg(url)
        .output()
        .expect("sqlx command failed to start");
    io::stdout().write_all(&s.stderr).unwrap();
    if !exists_database(&config, &store).await {
        panic!("The database has not been created");
    }
}

async fn exists_database(config: &config_api::Config, store: &store::Store) -> bool {
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
        .fetch_all(&store.connection)
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
