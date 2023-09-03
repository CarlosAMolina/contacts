use std::collections::HashMap;

use crate::store::Store;
use crate::transformers;
use crate::types::contact as contact_types;
use crate::types::database as database_types;
use crate::types::query::extract_query;
use tracing::{event, Level};

pub async fn add_category(
    store: Store,
    category: database_types::Category,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(Level::INFO, "category={:?}", category);
    match store.add_category(category).await {
        Ok(category) => Ok(warp::reply::json(&category)),
        Err(e) => return Err(warp::reject::custom(e)),
    }
}

// TODO not use store.clone
pub async fn add_contact(
    store: Store,
    new_contact: contact_types::NewContact,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(Level::INFO, "contact={:?}", new_contact);
    let new_user = database_types::NewUser {
        name: new_contact.user_name,
        surname: new_contact.user_surname,
    };
    let add_user_result = add_user(store.clone(), new_user).await;
    if let Err(e) = add_user_result {
        return Err(e);
    }
    let user_db = add_user_result.unwrap();
    for nickname_value in new_contact.nicknames.iter().cloned() {
        let nickname = database_types::Nickname {
            id_user: user_db.id,
            nickname: nickname_value,
        };
        if let Err(e) = add_nickname(store.clone(), nickname).await {
            return Err(e);
        }
    }
    for phone in new_contact.phones.iter().cloned() {
        let phone = database_types::Phone {
            id_user: user_db.id,
            phone: phone.value,
            description: phone.description,
        };
        if let Err(e) = add_phone(store.clone(), phone).await {
            return Err(e);
        }
    }
    for category_id in new_contact.categories_id.iter().cloned() {
        let user_category = database_types::UserCategory {
            id_user: user_db.id,
            id_category: category_id,
        };
        if let Err(e) = add_user_category(store.clone(), user_category).await {
            return Err(e);
        }
    }
    for address_value in new_contact.addresses.iter().cloned() {
        let address = database_types::Address {
            id_user: user_db.id,
            address: address_value,
        };
        if let Err(e) = add_address(store.clone(), address).await {
            return Err(e);
        }
    }
    for email_value in new_contact.emails.iter().cloned() {
        let email= database_types::Email {
            id_user: user_db.id,
            email: email_value,
        };
        if let Err(e) = add_email(store.clone(), email).await {
            return Err(e);
        }
    }
    for url_value in new_contact.urls.iter().cloned() {
        let url= database_types::Url {
            id_user: user_db.id,
            url: url_value,
        };
        if let Err(e) = add_url(store.clone(), url).await {
            return Err(e);
        }
    }
    get_contact_by_id(user_db.id, store).await
}

async fn add_address(
    store: Store,
    address: database_types::Address,
) -> Result<database_types::Address, warp::Rejection> {
    event!(Level::INFO, "address={:?}", address);
    match store.add_address(address).await {
        Ok(address) => Ok(address),
        Err(e) => return Err(warp::reject::custom(e)),
    }
}

async fn add_email(
    store: Store,
    email: database_types::Email,
) -> Result<database_types::Email, warp::Rejection> {
    event!(Level::INFO, "email={:?}", email);
    match store.add_email(email).await {
        Ok(email) => Ok(email),
        Err(e) => return Err(warp::reject::custom(e)),
    }
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

async fn add_phone(
    store: Store,
    phone: database_types::Phone,
) -> Result<database_types::Phone, warp::Rejection> {
    event!(Level::INFO, "phone={:?}", phone);
    match store.add_phone(phone).await {
        Ok(phone_db) => Ok(phone_db),
        Err(e) => return Err(warp::reject::custom(e)),
    }
}

async fn add_url(
    store: Store,
    url: database_types::Url,
) -> Result<database_types::Url, warp::Rejection> {
    event!(Level::INFO, "url={:?}", url);
    match store.add_url(url).await {
        Ok(url_db) => Ok(url_db),
        Err(e) => return Err(warp::reject::custom(e)),
    }
}

// TODO allow insert new categories

async fn add_user_category(
    store: Store,
    user_category: database_types::UserCategory,
) -> Result<database_types::UserCategory, warp::Rejection> {
    event!(Level::INFO, "user_category={:?}", user_category);
    match store.add_user_category(user_category).await {
        Ok(user_category_db) => Ok(user_category_db),
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
