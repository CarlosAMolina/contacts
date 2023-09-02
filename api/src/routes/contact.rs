use std::collections::HashMap;

use crate::store::Store;
use crate::transformers;
use crate::types::contact as contact_types;
use crate::types::database as database_types;
use crate::types::query::extract_query;
use tracing::{event, Level};

// TODO not use store.clone
pub async fn add_contact (
    store: Store,
    new_contact: contact_types::NewContact,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(Level::INFO, "contact={:?}", new_contact);
    let new_user = database_types::NewUser {
        name: new_contact.user_name,
        surname: new_contact.user_surname,
    };
    // TODO use route method
    // TODO improve if-else blocks
    let add_user_result = add_user(store.clone(), new_user).await;
    if let Err(e) = add_user_result {
        return Err(e);
    }
    let user_db = add_user_result.unwrap();
    let nicknames = new_contact.nicknames;
    for nickname_value in nicknames.iter().cloned() {
        let nickname = database_types::Nickname {
            id_user: user_db.id,
            nickname: nickname_value,
        };
        if let Err(e) = add_nickname(store.clone(), nickname).await {
            return Err(e);
        }
    }
    get_contact_by_id(user_db.id, store).await
}

async fn add_user(
    store: Store,
    new_user: database_types::NewUser,
) -> Result<database_types::User, warp::Rejection> {
    event!(Level::INFO, "user={:?}", new_user);
    match store.add_user(new_user).await {
        Ok(user) => Ok(user),
        Err(e) => return Err(warp::reject::custom(e)),
    }
}

async fn add_nickname(
    store: Store,
    nickname: database_types::Nickname,
) -> Result<database_types::Nickname, warp::Rejection> {
    event!(Level::INFO, "nickname={:?}", nickname);
    match store.add_nickname(nickname).await {
        Ok(nickname_db) => Ok(nickname_db),
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

// TODO use pagination
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
