use sqlx;
use sqlx::Row;
use tokio::sync::{oneshot, oneshot::Sender};
use tracing::{event, Level};
use warp::{http::Method, Filter, Reply};

pub use handle_errors;

pub mod config_api;
mod routes;
pub mod store;
mod transformers;
pub mod types;

pub struct TraceInfo {
    pub id: uuid::Uuid,
    pub method: Method,
    pub path: String,
    pub referer: String,
    pub remote_addr: String,
    pub request_headers: String,
    pub user_agent: String,
    pub version: String,
}

impl TraceInfo {
    pub fn new(info: &warp::trace::Info<'_>) -> TraceInfo {
        TraceInfo {
            id: TraceInfo::get_id(),
            method: info.method().clone(),
            path: info.path().to_string(),
            referer: TraceInfo::get_referer(&info),
            remote_addr: TraceInfo::get_remote_addr(&info),
            request_headers: TraceInfo::get_request_headers(&info),
            user_agent: TraceInfo::get_user_agent(&info),
            version: TraceInfo::get_version(&info),
        }
    }

    fn get_id() -> uuid::Uuid {
        uuid::Uuid::new_v4()
    }

    fn get_referer(info: &warp::trace::Info<'_>) -> String {
        info.referer().unwrap_or("").to_string()
    }

    fn get_remote_addr(info: &warp::trace::Info<'_>) -> String {
        let mut remote_addr = "".to_string();
        if let Some(value) = info.remote_addr() {
            remote_addr = format!("{:?}", value);
        }
        remote_addr
    }

    fn get_request_headers(info: &warp::trace::Info<'_>) -> String {
        format!("{:?}", info.request_headers())
    }

    fn get_user_agent(info: &warp::trace::Info<'_>) -> String {
        info.user_agent().unwrap_or("").to_string()
    }

    fn get_version(info: &warp::trace::Info<'_>) -> String {
        format!("{:?}", info.version())
    }
}

pub async fn setup_store(
    config: &config_api::Config,
) -> Result<store::Store, handle_errors::Error> {
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database_user,
        config.database_password,
        config.database_host,
        config.database_port,
        config.database_name
    );
    let store = store::Store::new(&db_url)
        .await
        .map_err(|e| handle_errors::Error::DatabaseQueryError(e))?;
    add_db_schema(&store).await;
    run_migrations(&store).await?;
    Ok(store)
}

async fn add_db_schema(store: &store::Store) {
    // TODO use trace instead of print
    println!("Init create schema");
    sqlx::query("CREATE SCHEMA IF NOT EXISTS contacts")
        //.execute(&postgres_connection)
        .execute(&store.clone().connection)
        .await
        .unwrap();
}

async fn run_migrations(store: &store::Store) -> Result<(), handle_errors::Error> {
    // TODO use trace instead of print
    println!("Init migrations");
    event!(Level::INFO, "Init migrations"); // TODO does not work, initialice the trace in
                                            // setup_store()
    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .map_err(|e| handle_errors::Error::MigrationError(e))?;
    assert_migrations_have_correctly_executed(&store.connection).await;
    Ok(())
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
        println!("Table names ({}): {:?}", table_names.len(), table_names);
        println!(
            "Expected table names ({}): {:?}",
            expected_table_names.len(),
            expected_table_names
        );
        panic!("Table names do not match the expected ones");
    }
}

async fn build_routes(store: store::Store) -> impl Filter<Extract = impl Reply> + Clone {
    let store_filter = warp::any().map(move || store.clone());
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let add_contact = warp::post()
        .and(warp::path("contacts"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::contact::add_contact)
        .with(warp::trace(|info| {
            let info_values = TraceInfo::new(&info);
            tracing::info_span!(
                  "add_contact request",
                  method = %info_values.method,
                  path = %info_values.path,
                  version = %info_values.version,
                  referer = %info_values.referer,
                  user_agent = %info_values.user_agent,
                  remote_addr = %info_values.remote_addr,
                  request_headers = %info_values.request_headers,
                  id = %info_values.id,
            )
        }));

    // TODO rm
    let add_category = warp::post()
        .and(warp::path("categories"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::contact::add_category);

    let get_contacts = warp::get()
        .and(warp::path("contacts"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::contact::get_contacts)
        .with(warp::trace(|info| {
            let info_values = TraceInfo::new(&info);
            tracing::info_span!(
                  "get_contacts request",
                  method = %info_values.method,
                  path = %info_values.path,
                  version = %info_values.version,
                  referer = %info_values.referer,
                  user_agent = %info_values.user_agent,
                  remote_addr = %info_values.remote_addr,
                  request_headers = %info_values.request_headers,
                  id = %info_values.id,
            )
        }));

    let get_contact_by_id = warp::get()
        .and(warp::path("contacts"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::contact::get_contact_by_id)
        .with(warp::trace(|info| {
            let info_values = TraceInfo::new(&info);
            tracing::info_span!(
                  "get_contact_by_id request",
                  method = %info_values.method,
                  path = %info_values.path,
                  version = %info_values.version,
                  referer = %info_values.referer,
                  user_agent = %info_values.user_agent,
                  remote_addr = %info_values.remote_addr,
                  request_headers = %info_values.request_headers,
                  id = %info_values.id,
            )
        }));

    add_contact
        .or(add_category)
        .or(get_contacts)
        .or(get_contact_by_id)
        .with(cors)
        .with(warp::trace::request())
        .recover(handle_errors::return_error)
}

pub async fn run(config: config_api::Config, store: store::Store) {
    let routes = build_routes(store).await;
    warp::serve(routes)
        .run((config.api_host, config.api_port))
        .await;
}

pub struct OneshotHandler {
    pub sender: Sender<i32>,
}

pub async fn oneshot(config: &config_api::Config, store: &store::Store) -> OneshotHandler {
    let routes = build_routes(store.clone()).await;
    let (tx, rx) = oneshot::channel::<i32>();
    let socket: std::net::SocketAddr = format!(
        "{:?}.{:?}.{:?}.{:?}:{:?}",
        config.api_host[0],
        config.api_host[1],
        config.api_host[2],
        config.api_host[3],
        config.api_port,
    )
    .parse()
    .expect("Not a valid address");

    let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(socket, async {
        rx.await.ok();
    });

    tokio::task::spawn(server);

    OneshotHandler { sender: tx }
}
