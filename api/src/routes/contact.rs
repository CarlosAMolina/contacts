use std::collections::HashMap;

use crate::store::Store;
use crate::transformers;
use crate::types::database as database_types;
use crate::types::query::extract_query;
use tracing::{event, Level};

// TODO use pagination

pub async fn add_contact(
    store: Store,
    new_user: database_types::NewUser,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(Level::INFO, "user={:?}", new_user);
    match store.add_user(new_user).await {
        Ok(user) => Ok(warp::reply::json(&user)),
        Err(e) => return Err(warp::reject::custom(e)),
    }
}

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
