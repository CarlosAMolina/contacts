use handle_errors::Error;
use std::collections::HashMap;

use crate::store::Store;
use crate::transformers;
use crate::types::pagination::extract_pagination;

pub async fn get_contact_by_id(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_all_data_by_id(id).await {
        Ok(all_data_vec) => Ok(warp::reply::json(
            &transformers::types::get_contact_from_all_data(all_data_vec),
        )),
        Err(e) => return Err(warp::reject::custom(e)),
    }
}

// TODO not use id in each log, add it automatically.
pub async fn get_contacts(
    params: HashMap<String, String>,
    store: Store,
    id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("{} Start querying contacts", id);
    log::info!("{} Params: {:?}", id, params);
    if params.is_empty() {
        log::info!("{} No pagination used", id);
        match store.get_all_data().await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => return Err(warp::reject::custom(e)),
        }
    } else {
        let query = params.get("query").unwrap();
        log::info!("{} Start searching query: {}", id, query);
        //TODO let pagination = extract_pagination(params, res.len())?;
        //TODO log::info!("{} Pagination set {:?}", id, &pagination);
        match store
            // TODO use pagination .get_all_data_by_query(pagination.limit, pagination.offset)
            .get_all_data_by_query(query)
            .await
        {
            Ok(all_data_vec) => Ok(warp::reply::json(
                &transformers::types::get_contacts_from_all_data(all_data_vec),
            )),
            Err(e) => return Err(warp::reject::custom(e)),
        }
    }
}
