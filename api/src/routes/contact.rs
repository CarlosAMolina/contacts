use handle_errors::Error;
use std::collections::HashMap;

use crate::store::Store;
use crate::transformers;
use tracing::{event, Level};

// TODO use pagination

pub async fn get_contact_by_id(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    event!(Level::INFO, "id={}", id);
    match store.get_all_data_by_id(id).await {
        Ok(all_data_vec) => Ok(warp::reply::json(
            &transformers::types::get_contact_from_all_data(all_data_vec),
        )),
        Err(e) => return Err(warp::reject::custom(e)),
    }
}

pub async fn get_contacts(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(Level::INFO, "params={:?}", params);
    let query = extract_query(params)?;
    match store.get_all_data_by_query(query).await {
        Ok(all_data_vec) => Ok(warp::reply::json(
            &transformers::types::get_contacts_from_all_data(all_data_vec),
        )),
        Err(e) => return Err(warp::reject::custom(e)),
    }
}

// TODO create Query type and move to types/query.rs
fn extract_query(params: HashMap<String, String>) -> Result<String, Error> {
    if params.contains_key("query") {
        let query = params.get("query").unwrap().clone();
        return Ok(query);
    } else {
        return Err(Error::MissingParameters);
    };
}

#[cfg(test)]
mod tests_extract_query{
    use super::*;

    #[test]
    fn test_extract_query_if_valid_parameters() {
        let mut params = HashMap::new();
        params.insert(String::from("query"), String::from("john"));
        let result = extract_query(params);
        let expected_result = "john".to_string();
        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_extract_query_if_missing_parameters() {
        let params = HashMap::new();
        let result = format!("{}", extract_query(params).unwrap_err());
        let expected_result = format!("{}", Error::MissingParameters);
        assert_eq!(result, expected_result);
    }

}
