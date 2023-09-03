-- Add up migration script here
CREATE VIEW contacts.all_data AS
 SELECT users.id,
    users.name,
    users.surname,
    nicknames.nickname,
    phones.phone,
    phones.description AS phone_description,
    categories.category,
    addresses.address,
    emails.email,
    urls.url,
    facebook.url AS facebook_url,
    twitter.handle AS twitter_handle,
    instagram.handle AS instagram_handle,
    notes.note
   FROM (((((((((((contacts.users
     LEFT JOIN contacts.addresses ON ((users.id = addresses.id_user)))
     LEFT JOIN contacts.users_categories ON ((users.id = users_categories.id_user)))
     LEFT JOIN contacts.categories ON ((users_categories.id_category = categories.id)))
     LEFT JOIN contacts.emails ON ((users.id = emails.id_user)))
     LEFT JOIN contacts.facebook ON ((users.id = facebook.id_user)))
     LEFT JOIN contacts.instagram ON ((users.id = instagram.id_user)))
     LEFT JOIN contacts.nicknames ON ((users.id = nicknames.id_user)))
     LEFT JOIN contacts.notes ON ((users.id = notes.id_user)))
     LEFT JOIN contacts.phones ON ((users.id = phones.id_user)))
     LEFT JOIN contacts.twitter ON ((users.id = twitter.id_user)))
     LEFT JOIN contacts.urls ON ((users.id = urls.id_user)));
