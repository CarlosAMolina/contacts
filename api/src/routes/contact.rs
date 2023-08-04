use handle_errors::Error;
use std::collections::HashMap;
use std::println;

use crate::store::Store;
use crate::types::contact::{AllData, Phone, Contact};
use crate::types::pagination::extract_pagination;

pub async fn get_contact_by_id(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_all_data_by_id(id).await {
        Ok(all_data) => Ok(warp::reply::json(&get_contact_from_all_data(all_data))),
        Err(e) => return Err(warp::reject::custom(e)),
    }
}

// TODO not use clone
// TODO create the vectors in a better way: https://stackoverflow.com/questions/64819025/is-there-a-simple-way-remove-duplicate-elements-from-an-array
fn get_contact_from_all_data(all_data_vec: Vec<AllData>) -> Contact {
    let mut addresses = vec![];
    let mut categories = vec![];
    let mut emails = vec![];
    let mut facebook_urls = vec![];
    let mut instagram_handles = vec![];
    let mut nicknames = vec![];
    let mut phones = vec![];
    let mut twitter_handles = vec![];
    let mut urls = vec![];
    let note = all_data_vec[0].note.clone();
    let user_id = all_data_vec[0].user_id.clone();
    let user_name = all_data_vec[0].user_name.clone();
    let user_surname = all_data_vec[0].user_surname.clone();
    // TODO fix phone, only the 1st value is used.
    for all_data in all_data_vec.iter().cloned() {
        if let Some(value) = all_data.address  { if !addresses.contains(&value) {addresses.push(value);}}
        if let Some(value) = all_data.category { if !categories.contains(&value) {categories.push(value); }}
        if let Some(value) = all_data.email    { if !emails.contains(&value) {emails.push(value); }}
        if let Some(value) = all_data.facebook_url { if !facebook_urls.contains(&value) {facebook_urls.push(value); }}
        if let Some(value) = all_data.instagram_handle { if !instagram_handles.contains(&value) {instagram_handles.push(value); }}
        if let Some(value) = all_data.nickname { if !nicknames.contains(&value) {nicknames.push(value); }}
        if let Some(value) = all_data.phone { if phones.is_empty() {phones.push(Phone{ value, description: all_data.phone_description}); }  }
        if let Some(value) = all_data.twitter_handle { if !twitter_handles.contains(&value) {twitter_handles.push(value); }}
        if let Some(value) = all_data.url { if !urls.contains(&value) {urls.push(value); }}
    }
    Contact {
        user_id,
        user_name,
        user_surname,
        nicknames,
        phones,
        categories,
        addresses,
        emails,
        urls,
        facebook_urls,
        twitter_handles,
        instagram_handles,
        note,
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


#[cfg(test)]
mod config_tests {
    use super::*;
    use crate::types::contact;

    #[test]
    fn get_contact_from_all_data_manages_duplicates_correctly() {
        let all_data_vec = vec![
            contact::AllData {
                user_id: contact::UserId(1),
                user_name: Some("John".to_string()),
                user_surname: Some("Doe".to_string()),
                nickname: Some("Johnny".to_string()),
                phone: Some(666666666),
                phone_description: Some("Work".to_string()),
                category: Some("Friends".to_string()),
                address: Some("C/ 123".to_string()),
                email: Some("john@doe.com".to_string()),
                url: Some("john.com".to_string()),
                facebook_url: Some("facebook/John".to_string()),
                twitter_handle: Some("JohnT".to_string()),
                instagram_handle: Some("JohnI".to_string()),
                note: Some("Foo bar".to_string()),
            },
            contact::AllData {
                user_id: contact::UserId(1),
                user_name: Some("John".to_string()),
                user_surname: Some("Doe".to_string()),
                nickname: Some("Johnny".to_string()),
                phone: Some(666666666),
                phone_description: Some("Work".to_string()),
                category: Some("Friends".to_string()),
                address: Some("C/ 123".to_string()),
                email: Some("john@doe.com".to_string()),
                url: Some("john.com".to_string()),
                facebook_url: Some("facebook/John".to_string()),
                twitter_handle: Some("JohnT".to_string()),
                instagram_handle: Some("JohnI".to_string()),
                note: Some("Foo bar".to_string()),
            }
        ];
        let expected = contact::Contact {
            user_id: contact::UserId(1),
            user_name: Some("John".to_string()),
            user_surname: Some("Doe".to_string()),
            nicknames: vec![("Johnny".to_string())],
            phones: vec![contact::Phone{value: 666666666, description: Some("Work".to_string())}],
            categories: vec!["Friends".to_string()],
            addresses: vec!["C/ 123".to_string()],
            emails: vec!["john@doe.com".to_string()],
            urls: vec!["john.com".to_string()],
            facebook_urls: vec!["facebook/John".to_string()],
            twitter_handles: vec!["JohnT".to_string()],
            instagram_handles: vec!["JohnI".to_string()],
            note: Some("Foo bar".to_string()),
        };
        let result = get_contact_from_all_data(all_data_vec);
        assert_eq!(expected, result);
    }
}
