use api::types::database as database_types;

pub async fn post_contacts_insert_new(new_user: database_types::NewUser) -> database_types::User {
    let client = reqwest::Client::new();
    client
        .post("http://localhost:3030/contacts")
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
    let client = reqwest::Client::new();
    client
        .post("http://localhost:3030/nicknames")
        .json(&nickname)
        .send()
        .await
        .unwrap()
        .json::<database_types::Nickname>()
        .await
        .unwrap()
}
