#![warn(clippy::all)]

use config::Config;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use warp::{http::Method, Filter};

use handle_errors::return_error;

mod routes;
mod store;
mod transformers;
mod types;

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct ConfigArgs {
    api_host: [u8; 4],
    api_port: u16,
    database_host: String,
    database_name: String,
    database_password: String,
    database_port: u16,
    database_user: String,
    log_level_api: String,
    log_level_warp: String,
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

#[tracing::instrument] // TODO rm?
#[tokio::main]
async fn main() {
    let log_filter = format!(
        // TODO "handle_errors={},api={},warp={}",
        "api={},warp={}", // TODO
        //config.log_level_api, config.log_level_warp // TODO
        "info",
        "error" // TODO RM
    );
    let logfile = RollingFileAppender::new(Rotation::DAILY, "/tmp", "contacts-api.log");
    let (non_blocking_logfile, _guard_logfile) = tracing_appender::non_blocking(logfile);
    let (non_blocking_stdout, _guard_stdout) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
        .with_env_filter(log_filter) // TODO use
        .with_writer(non_blocking_logfile.and(non_blocking_stdout)) // TODO use
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    let trace = warp::trace(|info| {
        let referer = info.referer().unwrap_or("");
        let mut remote_addr = "".to_string();
        if let Some(value) = info.remote_addr() {
            remote_addr = format!("{:?}", value);
        }
        let request_headers = format!("{:?}", info.request_headers());
        let user_agent = info.user_agent().unwrap_or("");
        let version = format!("{:?}", info.version());
        tracing::info_span!(
              "get_contacts request",
              method = %info.method(),
              path = %info.path(),
              version = %version,
              referer = %referer,
              user_agent = %user_agent,
              remote_addr = %remote_addr,
              request_headers = %request_headers,
              id = %uuid::Uuid::new_v4(),
        )
    });

    let config = ConfigArgs::new();
    let store = setup_store(&config).await;
    let store_filter = warp::any().map(move || store.clone());
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_contacts = warp::get()
        .and(warp::path("contacts"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::contact::get_contacts)
        .with(trace);

    let get_contact_by_id = warp::get()
        .and(warp::path("contacts"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::contact::get_contact_by_id);

    let routes = get_contacts
        .or(get_contact_by_id)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    tracing::info!(
        "Server running addr={}.{}.{}.{}:{}",
        config.api_host[0],
        config.api_host[1],
        config.api_host[2],
        config.api_host[3],
        config.api_port
    );
    warp::serve(routes)
        .run((config.api_host, config.api_port))
        .await;
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
            api_port: 3030,
            database_host: "localhost".to_string(),
            database_name: "contacts".to_string(),
            database_password: "pw".to_string(),
            database_port: 5432,
            database_user: "postgres".to_string(),
            log_level_api: "info".to_string(),
            log_level_warp: "error".to_string(),
        };
        let expected_in_docker = ConfigArgs {
            api_host: [0, 0, 0, 0],
            api_port: 3030,
            database_host: "172.20.0.5".to_string(), // App in localhost // TODO use config file
            database_name: "contacts".to_string(),
            database_password: "pw".to_string(),
            database_port: 5432,
            database_user: "postgres".to_string(),
            log_level_api: "info".to_string(),
            log_level_warp: "error".to_string(),
        };
        assert_eq!(expected_not_in_docker, ConfigArgs::new());
        std::env::set_var("IS_DOCKER_RUNNING", "true");
        assert_eq!(expected_in_docker, ConfigArgs::new());
    }
}
