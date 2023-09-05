use std::io::{self, Write};
use std::panic;
use std::process::Command;

use api::handle_errors::Error;
use api::types::contact as contact_types;
use api::types::database as database_types;
use api::{config_api, oneshot, setup_store};
use futures_util::future::FutureExt; // Required by catch_unwind.
use sqlx;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;

mod requests;

const RUN_SLOW_TESTS: bool = false;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Init integration tests");
    let config = config_api::Config::new().expect("Config can't be set");
    recreate_database(&config).await;
    if RUN_SLOW_TESTS {
        test_setup_store_returns_expected_error_if_invalid_config().await;
    }
    let store = setup_store(&config).await.unwrap();
    println!("Init start the api web server");
    let handler = oneshot(&config, &store).await;
    test_insert_db_data().await;
    test_add_contact().await;
    let url_api = format!(
        "http://{:?}.{:?}.{:?}.{:?}:{:?}",
        config.api_host[0],
        config.api_host[1],
        config.api_host[2],
        config.api_host[3],
        config.api_port,
    );
    test_get_contacts(&url_api).await;
    test_get_contacts_check_search_every_column(&url_api).await;
    test_get_contacts_if_invalid_path(&url_api).await;
    test_get_contacts_if_nonexistent_path(&url_api).await;
    test_get_contacts_if_query_has_one_row_result_but_the_contact_id_has_more_rows(&url_api).await;
    test_get_contacts_with_accents(&url_api).await;
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

// TODO change id column to serial in the db model?
// TODO id starts at 0 or 1?
async fn test_insert_db_data() {
    let category = database_types::Category {
        id: 1,
        category: "Family".to_string(),
    };
    let contact_db = requests::post_categories_insert_new(&category).await;
    assert_eq!(category, contact_db);
    let category = database_types::Category {
        id: 2,
        category: "Work".to_string(),
    };
    let contact_db = requests::post_categories_insert_new(&category).await;
    assert_eq!(category, contact_db);
    // TODO change assertions with function get table data and compare results.
}

async fn test_add_contact() {
    println!("Init test_add_contact");
    println!("Init insert data in db");
    let new_contact = contact_types::NewContact {
        user_name: "John".to_string(),
        user_surname: Some("Doe".to_string()),
        nicknames: vec!["Johnny".to_string(), "Joy".to_string()],
        phones: vec![
            contact_types::Phone {
                value: 666111222,
                description: Some("Work".to_string()),
            },
            contact_types::Phone {
                value: 666111333,
                description: None,
            },
        ],
        categories_id: vec![1, 2],
        addresses: vec!["address 1".to_string(), "address 2".to_string()],
        emails: vec!["john2@mail.com".to_string(), "john@mail.com".to_string()],
        urls: vec!["john-home.com".to_string(), "john-music.com".to_string()],
        facebook_urls: vec!["facebook/John".to_string(), "facebook/John2".to_string()],
        twitter_handles: vec!["JohnT".to_string(), "JohnT2".to_string()],
        instagram_handles: vec!["JohnnyIns".to_string(), "JohnnyIns2".to_string()],
        note: Some("Jane's brother".to_string()),
    };
    requests::post_contacts_insert_new(new_contact).await;
    let result = requests::get_contact_by_id(1).await;
    let expected_result = contact_types::Contact {
        user_id: 1,
        user_name: "John".to_string(),
        user_surname: Some("Doe".to_string()),
        nicknames: vec!["Johnny".to_string(), "Joy".to_string()],
        phones: vec![
            contact_types::Phone {
                value: 666111222,
                description: Some("Work".to_string()),
            },
            contact_types::Phone {
                value: 666111333,
                description: None,
            },
        ],
        categories: vec!["Family".to_string(), "Work".to_string()],
        addresses: vec!["address 1".to_string(), "address 2".to_string()],
        emails: vec!["john2@mail.com".to_string(), "john@mail.com".to_string()],
        urls: vec!["john-home.com".to_string(), "john-music.com".to_string()],
        facebook_urls: vec!["facebook/John".to_string(), "facebook/John2".to_string()],
        twitter_handles: vec!["JohnT".to_string(), "JohnT2".to_string()],
        instagram_handles: vec!["JohnnyIns".to_string(), "JohnnyIns2".to_string()],
        note: Some("Jane's brother".to_string()),
    };
    assert_eq!(expected_result, result);
}

async fn test_setup_store_returns_expected_error_if_invalid_config() {
    println!(
        "Init test_setup_store_raises_exception_if_invalid_config (this test takes 30 seconds)"
    );
    let config_wrong = config_api::Config {
        api_host: [1, 2, 3, 4],
        api_port: 1,
        database_host: "localhost".to_string(),
        database_name: "test".to_string(),
        database_password: "foo".to_string(),
        database_port: 1,
        database_user: "foo".to_string(),
        log_file_name: "foo.log".to_string(),
        log_level_api: "info".to_string(),
        log_level_handle_errors: "info".to_string(),
        log_level_warp: "error".to_string(),
        log_path_name: "/tmp".to_string(),
    };
    let result = panic::AssertUnwindSafe(setup_store(&config_wrong))
        .catch_unwind()
        .await;
    match result.unwrap().unwrap_err() {
        Error::DatabaseQueryError(_) => println!("✓"),
        _ => panic!("Test error"),
    };
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

async fn test_get_contacts(url_api: &String) {
    println!("Init test_get_contacts");
    let client = reqwest::Client::new();
    let url = format!("{url_api}/contacts?query=ohn");
    let response = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<Vec<contact_types::Contact>>()
        .await
        .unwrap();
    let expected_result = vec![contact_types::Contact {
        user_id: 1,
        user_name: "John".to_string(),
        user_surname: Some("Doe".to_string()),
        nicknames: vec!["Johnny".to_string(), "Joy".to_string()],
        phones: vec![
            contact_types::Phone {
                value: 666111222,
                description: Some("Work".to_string()),
            },
            contact_types::Phone {
                value: 666111333,
                description: None,
            },
        ],
        categories: vec!["Family".to_string(), "Work".to_string()],
        addresses: vec!["address 1".to_string(), "address 2".to_string()],
        emails: vec!["john2@mail.com".to_string(), "john@mail.com".to_string()],
        urls: vec!["john-home.com".to_string(), "john-music.com".to_string()],
        facebook_urls: vec!["facebook/John".to_string(), "facebook/John2".to_string()],
        twitter_handles: vec!["JohnT".to_string(), "JohnT2".to_string()],
        instagram_handles: vec!["JohnnyIns".to_string(), "JohnnyIns2".to_string()],
        note: Some("Jane's brother".to_string()),
    }];
    assert_eq!(expected_result, response);
    println!("✓");
}

async fn test_get_contacts_check_search_every_column(url_api: &String) {
    println!("Init test_get_contacts_check_search_every_column");
    println!("Init insert data in db");
    let new_contact = contact_types::NewContact {
        user_name: "only in column user_name".to_string(),
        user_surname: Some("only in column user_surname".to_string()),
        nicknames: vec!["only in column nicknames".to_string()],
        phones: vec![contact_types::Phone {
            value: 123456789,
            description: Some("only in column phone_description".to_string()),
        }],
        categories_id: vec![],
        addresses: vec!["only in column address".to_string()],
        emails: vec!["only in column emails".to_string()],
        urls: vec!["only in column urls".to_string()],
        facebook_urls: vec!["only in column facebook_urls".to_string()],
        twitter_handles: vec!["only in column twitter_handles".to_string()],
        instagram_handles: vec!["only in column instagram_handles".to_string()],
        note: Some("only in column note".to_string()),
    };
    requests::post_contacts_insert_new(new_contact).await;
    let client = reqwest::Client::new();
    let url = format!("{url_api}/contacts?query=only in column user_name");
    let response = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<Vec<contact_types::Contact>>()
        .await
        .unwrap();
    let expected_result = vec![contact_types::Contact {
        user_id: 2,
        user_name: "only in column user_name".to_string(),
        user_surname: Some("only in column user_surname".to_string()),
        nicknames: vec!["only in column nicknames".to_string()],
        phones: vec![contact_types::Phone {
            value: 123456789,
            description: Some("only in column phone_description".to_string()),
        }],
        categories: vec![],
        addresses: vec!["only in column address".to_string()],
        emails: vec!["only in column emails".to_string()],
        urls: vec!["only in column urls".to_string()],
        facebook_urls: vec!["only in column facebook_urls".to_string()],
        twitter_handles: vec!["only in column twitter_handles".to_string()],
        instagram_handles: vec!["only in column instagram_handles".to_string()],
        note: Some("only in column note".to_string()),
    }];
    assert_eq!(expected_result, response);
    println!("✓");
}

async fn test_get_contacts_if_invalid_path(url_api: &String) {
    println!("Init test_get_contacts_if_invalid_path");
    let client = reqwest::Client::new();
    let url = format!("{url_api}/contacts/a");
    let response = client.get(url).send().await.unwrap().text().await.unwrap();
    let expected_result = Error::Unknown.to_string();
    assert_eq!(expected_result, response);
    println!("✓");
}

async fn test_get_contacts_if_nonexistent_path(url_api: &String) {
    println!("Init test_get_contacts_if_nonexistent_path");
    let client = reqwest::Client::new();
    let url = format!("{url_api}/nonexistent_path/");
    let response = client.get(url).send().await.unwrap().text().await.unwrap();
    let expected_result = Error::Unknown.to_string();
    assert_eq!(expected_result, response);
    println!("✓");
}

async fn test_get_contacts_if_query_has_one_row_result_but_the_contact_id_has_more_rows(
    url_api: &String,
) {
    println!("Init test_get_contacts_if_query_has_one_row_result_but_the_contact_id_has_more_rows");
    println!("Init insert test data in db");
    let new_contact = contact_types::NewContact {
        user_name: "Boby".to_string(),
        user_surname: None,
        nicknames: vec!["FooNickname".to_string(), "BarNickname".to_string()],
        phones: vec![],
        categories_id: vec![],
        addresses: vec![],
        emails: vec![],
        urls: vec![],
        facebook_urls: vec![],
        twitter_handles: vec![],
        instagram_handles: vec![],
        note: None,
    };
    requests::post_contacts_insert_new(new_contact).await;
    let client = reqwest::Client::new();
    // TODO user api method
    let url = format!("{url_api}/contacts?query=fooNick");
    let response = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<Vec<contact_types::Contact>>()
        .await
        .unwrap();
    let expected_result = vec![contact_types::Contact {
        user_id: 2,
        user_name: "Boby".to_string(),
        user_surname: None,
        nicknames: vec!["BarNickname".to_string(), "FooNickname".to_string()],
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

async fn test_get_contacts_with_accents(url_api: &String) {
    println!("Init test_get_contacts_with_accents");
    println!("Init insert test data in db");
    let new_contact = contact_types::NewContact {
        user_name: "MartínÁÉÍÓÚáéíóú".to_string(),
        user_surname: None,
        nicknames: vec![],
        phones: vec![],
        categories_id: vec![],
        addresses: vec![],
        emails: vec![],
        urls: vec![],
        facebook_urls: vec![],
        twitter_handles: vec![],
        instagram_handles: vec![],
        note: None,
    };
    requests::post_contacts_insert_new(new_contact).await;
    let client = reqwest::Client::new();
    // TODO use api methods
    // Test search term with accent.
    let url_search_term_with_accent = format!("{url_api}/contacts?query=martínáéíóúáéíóú");
    let response_search_term_with_accent = client
        .get(url_search_term_with_accent)
        .send()
        .await
        .unwrap()
        .json::<Vec<contact_types::Contact>>()
        .await
        .unwrap();
    // Test search term without accent.
    let url_search_term_without_accent = format!("{url_api}/contacts?query=martinaeiouaeiou");
    let response_search_term_without_accent = client
        .get(url_search_term_without_accent)
        .send()
        .await
        .unwrap()
        .json::<Vec<contact_types::Contact>>()
        .await
        .unwrap();
    let expected_result = vec![contact_types::Contact {
        user_id: 3,
        user_name: "MartínÁÉÍÓÚáéíóú".to_string(),
        user_surname: None,
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
    assert_eq!(expected_result, response_search_term_with_accent);
    assert_eq!(expected_result, response_search_term_without_accent);
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
        .json::<Vec<contact_types::Contact>>()
        .await
        .unwrap();
    assert!(response.is_empty());
    println!("✓");
}

async fn test_get_contacts_if_missing_parameters(url_api: &String) {
    println!("Init test_get_contacts_if_missing_parameters");
    let client = reqwest::Client::new();
    let url = format!("{url_api}/contacts");
    let response = client.get(url).send().await.unwrap().text().await.unwrap();
    let expected_result = format!("{}", Error::MissingParameters);
    assert_eq!(response, expected_result);
    println!("✓");
}

async fn test_get_contacts_if_missing_parameters_and_url_ends_in_slash(url_api: &String) {
    println!("Init test_get_contacts_if_missing_parameters_and_url_ends_in_slash");
    let client = reqwest::Client::new();
    let url = format!("{url_api}/contacts/");
    let response = client.get(url).send().await.unwrap().text().await.unwrap();
    let expected_result = format!("{}", Error::MissingParameters);
    assert_eq!(response, expected_result);
    println!("✓");
}

async fn test_get_contact_by_id(url_api: &String) {
    println!("Init test_get_contact_by_id");
    let client = reqwest::Client::new();
    // TODO use API
    let url = format!("{url_api}/contacts/1");
    let response = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<contact_types::Contact>()
        .await
        .unwrap();
    let expected_result = contact_types::Contact {
        user_id: 1,
        user_name: "John".to_string(),
        user_surname: Some("Doe".to_string()),
        nicknames: vec!["Johnny".to_string(), "Joy".to_string()],
        phones: vec![
            contact_types::Phone {
                value: 666111222,
                description: Some("Work".to_string()),
            },
            contact_types::Phone {
                value: 666111333,
                description: None,
            },
        ],
        categories: vec!["Family".to_string(), "Work".to_string()],
        addresses: vec!["address 1".to_string(), "address 2".to_string()],
        emails: vec!["john2@mail.com".to_string(), "john@mail.com".to_string()],
        urls: vec!["john-home.com".to_string(), "john-music.com".to_string()],
        facebook_urls: vec!["facebook/John".to_string(), "facebook/John2".to_string()],
        twitter_handles: vec!["JohnT".to_string(), "JohnT2".to_string()],
        instagram_handles: vec!["JohnnyIns".to_string(), "JohnnyIns2".to_string()],
        note: Some("Jane's brother".to_string()),
    };
    assert_eq!(expected_result, response);
    println!("✓");
}

async fn test_get_contact_by_id_if_id_does_not_exist(url_api: &String) {
    println!("Init test_get_contact_by_id_if_id_does_not_exist");
    let client = reqwest::Client::new();
    // TODO use api
    let url = format!("{url_api}/contacts/999");
    let response = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<Option<contact_types::Contact>>()
        .await
        .unwrap();
    let expected_result = None;
    assert_eq!(expected_result, response);
    println!("✓");
}
