use sqlx;
use tracing::{event, Level};
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::Reject,
    Rejection, Reply,
};

#[derive(Debug)]
pub enum Error {
    MigrationError(sqlx::migrate::MigrateError),
    MissingParameters,
    ParseError(std::num::ParseIntError),
    ContactNotFound,
    StartGreaterThanEnd,
    DatabaseQueryError(sqlx::Error),
    NotImplemented,
    Unknown,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ContactNotFound => write!(f, "Contact not found"),
            Error::MigrationError(_) => write!(f, "Cannot migrate data"),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::ParseError(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            }
            Error::StartGreaterThanEnd => write!(f, "The start is greater than the end"),
            Error::DatabaseQueryError(_) => write!(f, "Database query error"),
            Error::NotImplemented => write!(f, "Not implemented"),
            Error::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl Reject for Error {}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    // TODO implement event!(Level::ERROR .. in all cases
    if let Some(crate::Error::DatabaseQueryError(e)) = r.find() {
        // TODO implement all DB errors
        event!(Level::ERROR, "Database query error");
        Ok(warp::reply::with_status(
            e.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        event!(Level::ERROR, "Unknown error: {:?}", r);
        Ok(warp::reply::with_status(
            Error::Unknown.to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
