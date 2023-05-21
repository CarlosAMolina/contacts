use handle_errors::Error;
use std::collections::HashMap;
use std::println;

use crate::file_csv;
use crate::store::Store;
use crate::types::contact::{Contact, ContactId};
use crate::types::pagination::extract_pagination;

pub async fn get_contact_by_id(
    id: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.contacts.read().await.get(&ContactId(id)) {
        Some(contact) => Ok(warp::reply::json(&contact)),
        None => Err(warp::reject::custom(Error::ContactNotFound)),
    }
}

pub async fn get_contacts_all(
    params: HashMap<String, String>,
    store: Store,
    id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO log::info does not work, I use println instead
    log::info!("{} Start querying contacts", id);
    println!("{:?}", params);
    if params.is_empty() {
        let res: Vec<Contact> = store.contacts.read().await.values().cloned().collect();
        log::info!("{} No pagination used", id);
        Ok(warp::reply::json(&res))
    } else if params.contains_key("query") {
        let query = params.get("query").unwrap();
        println!("Start searching {}", query);
        let res: Vec<Contact> = store.contacts.read().await.values().cloned().collect();
        // TODO use await?
        let result: Vec<Contact> = res
            .iter()
            .filter(|contact| contact.contains(query))
            .cloned()
            .collect();
        Ok(warp::reply::json(&result))
    } else {
        let res: Vec<Contact> = store.contacts.read().await.values().cloned().collect();
        let pagination = extract_pagination(params, res.len())?;
        log::info!("{} Pagination set {:?}", id, &pagination);
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    }
}
