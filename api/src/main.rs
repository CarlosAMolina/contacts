#![warn(clippy::all)]

use config::Config;
use warp::{http::Method, Filter};

use handle_errors::return_error;

mod routes;
mod store;
mod types;

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct ConfigArgs {
    api_host: [u8; 4],
    database_host: String,
    database_name: String,
    database_password: String,
    database_port: u16,
    database_user: String,
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

// TODO pub async fn setup_store(config: &config::Config) -> Result<store::Store, handle_errors::Error> {
pub async fn setup_store(config: &ConfigArgs) -> store::Store {
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database_user,
        config.database_password,
        config.database_host,
        config.database_port,
        config.database_name
    );

    store::Store::new(&db_url).await.unwrap()
    // TODO use .map_err(|e| handle_errors::Error::DatabaseQueryError(e))?;
}

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let log = warp::log::custom(|info| {
        log::info!(
            "{} {} {} {:?} from {} with {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
            info.remote_addr().unwrap(),
            info.request_headers()
        );
    });

    let config = ConfigArgs::new();
    let store = setup_store(&config).await;
    let store_filter = warp::any().map(move || store.clone());

    let id_filter = warp::any().map(|| uuid::Uuid::new_v4().to_string());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_contacts = warp::get()
        .and(warp::path("contacts"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and(id_filter)
        .and_then(routes::contact::get_contacts);

    let get_contact_by_id = warp::get()
        .and(warp::path("contacts"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::contact::get_contact_by_id);

    let routes = get_contacts
        .or(get_contact_by_id)
        .with(cors)
        .with(log)
        .recover(return_error);

    warp::serve(routes).run((config.api_host, 3030)).await;
}

#[cfg(test)]
mod config_tests {
    use super::*;

    // As Rust runs test in parallel, we run two tests in the same function
    // in order to not affect each test when env variables are modified.
    #[test]
    fn config_files_are_detected_correctly() {
        let expected_not_in_docker = ConfigArgs {
            api_host: [127, 0, 0, 1],
            database_host: "localhost".to_string(),
            database_name: "contacts".to_string(),
            database_password: "pw".to_string(),
            database_port: 5432,
            database_user: "postgres".to_string(),
        };
        let expected_in_docker = ConfigArgs {
            api_host: [0, 0, 0, 0],
            database_host: "172.20.0.5".to_string(), // App in localhost // TODO use config file
            database_name: "contacts".to_string(),
            database_password: "pw".to_string(),
            database_port: 5432,
            database_user: "postgres".to_string(),
        };
        assert_eq!(expected_not_in_docker, ConfigArgs::new());
        std::env::set_var("IS_DOCKER_RUNNING", "true");
        assert_eq!(expected_in_docker, ConfigArgs::new());
    }
}
