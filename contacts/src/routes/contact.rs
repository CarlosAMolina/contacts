use handle_errors::Error;
use std::collections::HashMap;

use crate::store::Store;
use crate::types::contact::{Contact, ContactId};
use crate::types::pagination::extract_pagination;

pub async fn get_contact(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.contacts.read().await.get(&ContactId(id)) {
        Some(contact) => Ok(warp::reply::json(&contact)),
        None => Err(warp::reject::custom(Error::ContactNotFound)),
    }
}

pub async fn get_contacts(
    params: HashMap<String, String>,
    store: Store,
    id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("{} Start querying contacts", id);
    println!("{:?}", params);
    if params.is_empty() {
        log::info!("{} No pagination used", id);
        let res: Vec<Contact> = store.contacts.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Contact> = store.contacts.read().await.values().cloned().collect();
        let pagination = extract_pagination(params, res.len())?;
        log::info!("{} Pagination set {:?}", id, &pagination);
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    }
}
