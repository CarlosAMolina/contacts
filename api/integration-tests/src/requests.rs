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

pub async fn post_categories_insert_new(category: &database_types::Category) -> database_types::Category {
    let url = format!("{URL_API}/categories");
    let client = reqwest::Client::new();
    client
        .post(url)
        .json(&category)
        .send()
        .await
        .unwrap()
        .json::<database_types::Category>()
        .await
        .unwrap()
}
