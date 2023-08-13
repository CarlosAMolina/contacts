use sqlx;
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::Reject,
    Rejection, Reply,
};
use tracing::{event, Level};


#[derive(Debug)]
pub enum Error {
    MissingParameters,
    ParseError(std::num::ParseIntError),
    ContactNotFound,
    StartGreaterThanEnd,
    DatabaseQueryError(sqlx::Error),
    NotImplemented,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ContactNotFound=> write!(f, "Contact not found"),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::ParseError(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            }
            Error::StartGreaterThanEnd => write!(f, "The start is greater than the end"),
            Error::DatabaseQueryError(_) => write!(f, "Cannot update, invalid data"),
            Error::NotImplemented => write!(f, "Not implemented"),
        }
    }
}

impl Reject for Error {}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(crate::Error::DatabaseQueryError(e)) = r.find() {
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
    } else if r.is_not_found() {
        event!(Level::WARN, "Requested route was not found");
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    } else {
        event!(Level::ERROR, "Unknown error: {:?}", r);
        Ok(warp::reply::with_status(
            "Unknown error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
