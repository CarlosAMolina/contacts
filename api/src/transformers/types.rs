use crate::types::contact::{Contact, Phone};
use crate::types::database::AllData;

// TODO not use clone
pub fn get_contact_from_all_data(all_data_vec: Vec<AllData>) -> Contact {
    let mut all_data_phone_unique: Vec<_> = all_data_vec
        .iter()
        .filter(|row| row.phone.is_some())
        .collect();
    all_data_phone_unique.dedup_by(|a, b| a.phone == b.phone);
    let phones: Vec<Phone> = all_data_phone_unique
        .iter()
        .map(|row| Phone {
            value: row.phone.unwrap(),
            description: row.phone_description.clone(),
        })
        .collect();
    let mut addresses = vec![];
    let mut categories = vec![];
    let mut emails = vec![];
    let mut facebook_urls = vec![];
    let mut instagram_handles = vec![];
    let mut nicknames = vec![];
    let mut twitter_handles = vec![];
    let mut urls = vec![];
    let note = all_data_vec[0].note.clone();
    let user_id = all_data_vec[0].user_id.clone();
    let user_name = all_data_vec[0].user_name.clone();
    let user_surname = all_data_vec[0].user_surname.clone();
    for all_data in all_data_vec.iter().cloned() {
        if let Some(value) = all_data.address {
            push_to_vector_if_new(&mut addresses, value);
        }
        if let Some(value) = all_data.category {
            push_to_vector_if_new(&mut categories, value);
        }
        if let Some(value) = all_data.email {
            push_to_vector_if_new(&mut emails, value);
        }
        if let Some(value) = all_data.facebook_url {
            push_to_vector_if_new(&mut facebook_urls, value);
        }
        if let Some(value) = all_data.instagram_handle {
            push_to_vector_if_new(&mut instagram_handles, value);
        }
        if let Some(value) = all_data.nickname {
            push_to_vector_if_new(&mut nicknames, value);
        }
        if let Some(value) = all_data.twitter_handle {
            push_to_vector_if_new(&mut twitter_handles, value);
        }
        if let Some(value) = all_data.url {
            push_to_vector_if_new(&mut urls, value);
        }
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

fn push_to_vector_if_new(array: &mut Vec<String>, value: String) {
    if !array.contains(&value) {
        array.push(value);
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn get_contact_from_all_data_manages_duplicates_correctly() {
        // TODO test empty phone
        let all_data_vec = vec![
            AllData {
                user_id: 1,
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
            // Empty phone
            AllData {
                user_id: 1,
                user_name: Some("John".to_string()),
                user_surname: Some("Doe".to_string()),
                nickname: Some("Johnny".to_string()),
                phone: None,
                phone_description: None,
                category: Some("Friends".to_string()),
                address: Some("C/ 123".to_string()),
                email: Some("john@doe.com".to_string()),
                url: Some("john.com".to_string()),
                facebook_url: Some("facebook/John".to_string()),
                twitter_handle: Some("JohnT".to_string()),
                instagram_handle: Some("JohnI".to_string()),
                note: Some("Foo bar".to_string()),
            },
            // Empty phone descrition and twitter handle
            AllData {
                user_id: 1,
                user_name: Some("John".to_string()),
                user_surname: Some("Doe".to_string()),
                nickname: Some("Johnny".to_string()),
                phone: Some(666666661),
                phone_description: None,
                category: Some("Friends".to_string()),
                address: Some("C/ 123".to_string()),
                email: Some("john@doe.com".to_string()),
                url: Some("john.com".to_string()),
                facebook_url: Some("facebook/John2".to_string()),
                twitter_handle: None,
                instagram_handle: Some("JohnI".to_string()),
                note: Some("Foo bar".to_string()),
            },
        ];
        let expected = Contact {
            user_id: 1,
            user_name: Some("John".to_string()),
            user_surname: Some("Doe".to_string()),
            nicknames: vec![("Johnny".to_string())],
            phones: vec![
                Phone {
                    value: 666666666,
                    description: Some("Work".to_string()),
                },
                Phone {
                    value: 666666661,
                    description: None,
                },
            ],
            categories: vec!["Friends".to_string()],
            addresses: vec!["C/ 123".to_string()],
            emails: vec!["john@doe.com".to_string()],
            urls: vec!["john.com".to_string()],
            facebook_urls: vec!["facebook/John".to_string(), "facebook/John2".to_string()],
            twitter_handles: vec!["JohnT".to_string()],
            instagram_handles: vec!["JohnI".to_string()],
            note: Some("Foo bar".to_string()),
        };
        let result = get_contact_from_all_data(all_data_vec);
        assert_eq!(expected, result);
    }
}
