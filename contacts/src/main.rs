#![warn(clippy::all)]

use handle_errors::return_error;
use warp::{http::Method, Filter};

mod routes;
mod store;
mod types;

struct Config {
    database_host: String,
    database_name: String,
    database_password: String,
    database_port: u16,
    database_user: String,
}

// TODO pub async fn setup_store(config: &config::Config) -> Result<store::Store, handle_errors::Error> {
pub async fn setup_store() -> store::Store {
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

    let store = setup_store().await;
    let store_filter = warp::any().map(move || store.clone());

    let id_filter = warp::any().map(|| uuid::Uuid::new_v4().to_string());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_contacts_all = warp::get()
        .and(warp::path("contacts"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and(id_filter)
        .and_then(routes::contact::get_contacts_all);

    // TODO let get_contact_by_id = warp::get()
    // TODO     .and(warp::path("contacts"))
    // TODO     .and(warp::path::param::<String>())
    // TODO     .and(warp::path::end())
    // TODO     .and(store_filter.clone())
    // TODO     .and_then(routes::contact::get_contact_by_id);

    let routes = get_contacts_all
        //.or(get_contact_by_id)
        .with(cors)
        .with(log)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
