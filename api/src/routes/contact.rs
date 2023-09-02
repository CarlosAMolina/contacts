use std::collections::HashMap;

use crate::store::Store;
use crate::transformers;
use crate::types::contact as contact_types;
use crate::types::database as database_types;
use crate::types::query::extract_query;
use tracing::{event, Level};

// TODO use
pub async fn add_contact (
    store: Store,
    new_contact: contact_types::NewContact,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(Level::INFO, "contact={:?}", new_contact);
    let new_user = database_types::NewUser {
        name: new_contact.user_name,
        surname: new_contact.user_surname,
    };
    // TODO user route method
    let user_db = store.add_user(new_user).await;
    if let Err(e) = user_db {
        return Err(warp::reject::custom(e));
    }
    let user_db_ok = user_db.unwrap();
    let nicknames = new_contact.nicknames;
    // TODO manage all nicknames not only the first one
    let nickname = database_types::Nickname {
        id_user: user_db_ok.id,
        nickname: nicknames[0].clone(),
    };
    let nickname_db = store.add_nickname(nickname).await;
    if let Err(e) = nickname_db {
        return Err(warp::reject::custom(e));
    }
    // TODO improve previous if-else
    // TODO use get contact by id
    let contact = contact_types::Contact {
        user_id: user_db_ok.id,
        user_name: user_db_ok.name,
        user_surname: user_db_ok.surname,
        nicknames,
        phones: vec![],
        categories: vec![],
        addresses: vec![],
        emails: vec![],
        urls: vec![],
        facebook_urls: vec![],
        twitter_handles: vec![],
        instagram_handles: vec![],
        note: None,
    };
    return Ok(warp::reply::json(&contact));
}

// TODO rm pub
pub async fn add_user(
    store: Store,
    new_user: database_types::NewUser,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(Level::INFO, "user={:?}", new_user);
    match store.add_user(new_user).await {
        Ok(user) => Ok(warp::reply::json(&user)),
        Err(e) => return Err(warp::reject::custom(e)),
    }
}


// TODO rm pub
pub async fn add_nickname(
    store: Store,
    nickname: database_types::Nickname,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(Level::INFO, "nickname={:?}", nickname);
    match store.add_nickname(nickname).await {
        Ok(nickname_db) => Ok(warp::reply::json(&nickname_db)),
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
