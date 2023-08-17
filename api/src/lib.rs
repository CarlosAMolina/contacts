use tokio::sync::{oneshot, oneshot::Sender};
use warp::{http::Method, Filter, Reply};

pub use handle_errors;

pub mod config_api; // TODO try use mod instead of mod
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

// TODO pub async fn setup_store(config: &config::Config) -> Result<store::Store, handle_errors::Error> {
pub async fn setup_store(config: &config_api::Config) -> store::Store {
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

async fn build_routes(store: store::Store) -> impl Filter<Extract = impl Reply> + Clone {
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
    get_contacts
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

pub async fn oneshot(store: &store::Store) -> OneshotHandler {
    let routes = build_routes(store.clone()).await;
    let (tx, rx) = oneshot::channel::<i32>();
    // TODO use config in the url
    let socket: std::net::SocketAddr = "127.0.0.1:3030"
        .to_string()
        .parse()
        .expect("Not a valid address");

    let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(socket, async {
        rx.await.ok();
    });

    tokio::task::spawn(server);

    OneshotHandler { sender: tx }
}
