use handle_errors::Error;
use std::collections::HashMap;
use std::println;

use crate::store::Store;
use crate::types::pagination::extract_pagination;

pub async fn get_contact_by_id(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_all_data_by_id(id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => return Err(warp::reject::custom(e)),
    }
}

pub async fn get_contacts(
    params: HashMap<String, String>,
    store: Store,
    id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO log::info does not work, I use println instead
    log::info!("{} Start querying contacts", id);
    println!("Params: {:?}", params);
    if params.is_empty() {
        log::info!("{} No pagination used", id);
        match store.get_all_data().await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => return Err(warp::reject::custom(e)),
        }
    } else {
        let query = params.get("query").unwrap();
        println!("Start searching {}", query);
        //TODO let pagination = extract_pagination(params, res.len())?;
        //TODO log::info!("{} Pagination set {:?}", id, &pagination);
        match store
            // TODO use pagination .get_all_data_by_query(pagination.limit, pagination.offset)
            .get_all_data_by_query(query)
            .await
        {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => return Err(warp::reject::custom(e)),
        }
    }
}
