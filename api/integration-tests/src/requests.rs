use api::types::contact as contact_types;
use api::types::database as database_types;

// TODO use config file
static URL_API: &str = "http://localhost:3030";

pub async fn get_contact_by_id(id: u32) -> contact_types::Contact {
    let url = format!("{URL_API}/contacts/{id}");
    let client = reqwest::Client::new();
    client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<contact_types::Contact>()
        .await
        .unwrap()
}

pub async fn post_contacts_insert_new(new_contact: contact_types::NewContact) -> contact_types::Contact {
    let url = format!("{URL_API}/contacts");
    let client = reqwest::Client::new();
    client
        .post(url)
        .json(&new_contact)
        .send()
        .await
        .unwrap()
        .json::<contact_types::Contact>()
        .await
        .unwrap()
}

// TODO rm
pub async fn post_users_insert_new(new_user: database_types::NewUser) -> database_types::User {
    let url = format!("{URL_API}/users");
    let client = reqwest::Client::new();
    client
        .post(url)
        .json(&new_user)
        .send()
        .await
        .unwrap()
        .json::<database_types::User>()
        .await
        .unwrap()
}

pub async fn post_nicknames_insert_new(
    nickname: database_types::Nickname,
) -> database_types::Nickname {
    let url = format!("{URL_API}/nicknames");
    let client = reqwest::Client::new();
    client
        .post(url)
        .json(&nickname)
        .send()
        .await
        .unwrap()
        .json::<database_types::Nickname>()
        .await
        .unwrap()
}
