use csv::StringRecord;

use crate::types::contact;

impl contact::Contact {
    pub fn new(
        id: &str,
        name: &str,
        surname: &str,
        nickname: &str,
        phone: &str,
        phone_description: &str,
        category: &str,
        address: &str,
        email: &str,
        url: &str,
        facebook_url: &str,
        twitter_handle: &str,
        note: &str,
    ) -> Self {
        let phone_str = phone.to_string();
        let phone: Option<usize> = if phone_str.is_empty() {
            None
        } else {
            Some(phone_str.parse::<usize>().unwrap())
        };
        contact::Contact {
            id: id.to_string().parse::<usize>().unwrap(),
            name: name.to_string(),
            surname: surname.to_string(),
            nickname: nickname.to_string(),
            phone,
            phone_description: phone_description.to_string(),
            category: category.to_string(),
            address: address.to_string(),
            email: email.to_string(),
            url: url.to_string(),
            facebook_url: facebook_url.to_string(),
            twitter_handle: twitter_handle.to_string(),
            note: note.to_string(),
        }
    }

    pub fn new_from_csv_record(record: &StringRecord) -> contact::Contact {
        contact::Contact::new(
            &record[0],
            &record[1],
            &record[2],
            &record[3],
            &record[4],
            &record[5],
            &record[6],
            &record[7],
            &record[8],
            &record[9],
            &record[10],
            &record[11],
            &record[12],
        )
    }
}
