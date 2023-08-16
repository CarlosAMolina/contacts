use api::{config_api, handle_errors};
use sqlx;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use std::io::{self, Write};
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    println!("Init integration tests");
    let config = config_api::Config::new().expect("Config can't be set");

    if exists_database(&config).await {
        println!("The database {} exists", config.database_name);

        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.database_user,
            config.database_password,
            config.database_host,
            config.database_port,
            config.database_name
        );
        println!(
            "Init delete database {:?}. URL: {}",
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
        "Init create database {:?}. URL: {}",
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
    if !exists_database(&config).await {
        panic!("The database has not been created");
    }

    Ok(())
}

async fn exists_database(config: &config_api::Config) -> bool {
    let url = format!(
        "postgres://{}:{}@{}:{}",
        config.database_user, config.database_password, config.database_host, config.database_port,
    );
    println!(
        "Init check database {} exists. URL: {}",
        config.database_name, url
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap();
    let database_names: Vec<_> = sqlx::query("SELECT datname FROM pg_database")
        .map(|row: sqlx::postgres::PgRow| row.get::<String, _>("datname").to_string())
        .fetch_all(&pool)
        .await
        .unwrap();
    database_names.contains(&config.database_name)
}
