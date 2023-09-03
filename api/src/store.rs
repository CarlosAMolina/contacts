use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::types::database as database_types;
use handle_errors::Error;
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::{Postgres, Row};

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;
        Ok(Store {
            connection: db_pool,
        })
    }

    pub async fn add_address(
        &self,
        address: database_types::Address
    ) -> Result<database_types::Address, Error> {
        match sqlx::query(
            "INSERT INTO contacts.addresses (id_user, address)
           VALUES ($1, $2)
           RETURNING id_user, address",
        )
        .bind(address.id_user)
        .bind(address.address)
        .map(|row: PgRow| database_types::Address {
            id_user: row.get("id_user"),
            address: row.get("address"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(address_db) => Ok(address_db),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    pub async fn add_email(
        &self,
        email: database_types::Email
    ) -> Result<database_types::Email, Error> {
        match sqlx::query(
            "INSERT INTO contacts.emails (id_user, email)
           VALUES ($1, $2)
           RETURNING id_user, email",
        )
        .bind(email.id_user)
        .bind(email.email)
        .map(|row: PgRow| database_types::Email{
            id_user: row.get("id_user"),
            email: row.get("email"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(email_db) => Ok(email_db),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    pub async fn add_category(
        &self,
        category: database_types::Category
    ) -> Result<database_types::Category, Error> {
        match sqlx::query(
            "INSERT INTO contacts.categories (id, category)
           VALUES ($1, $2)
           RETURNING id, category",
        )
        .bind(category.id)
        .bind(category.category)
        .map(|row: PgRow| database_types::Category {
            id: row.get("id"),
            category: row.get("category"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(category_db) => Ok(category_db),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    pub async fn add_facebook(
        &self,
        facebook: database_types::Facebook,
    ) -> Result<database_types::Facebook, Error> {
        match sqlx::query(
            "INSERT INTO contacts.facebook (id_user, url)
           VALUES ($1, $2)
           RETURNING id_user, url",
        )
        .bind(facebook.id_user)
        .bind(facebook.url)
        .map(|row: PgRow| database_types::Facebook {
            id_user: row.get("id_user"),
            url: row.get("url"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(facebook_db) => Ok(facebook_db),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    pub async fn add_twitter(
        &self,
        twitter: database_types::Twitter,
    ) -> Result<database_types::Twitter, Error> {
        match sqlx::query(
            "INSERT INTO contacts.twitter (id_user, handle)
           VALUES ($1, $2)
           RETURNING id_user, handle",
        )
        .bind(twitter.id_user)
        .bind(twitter.handle)
        .map(|row: PgRow| database_types::Twitter {
            id_user: row.get("id_user"),
            handle: row.get("handle"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(url) => Ok(url),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    pub async fn add_url(
        &self,
        url: database_types::Url,
    ) -> Result<database_types::Url, Error> {
        match sqlx::query(
            "INSERT INTO contacts.urls (id_user, url)
           VALUES ($1, $2)
           RETURNING id_user, url",
        )
        .bind(url.id_user)
        .bind(url.url)
        .map(|row: PgRow| database_types::Url {
            id_user: row.get("id_user"),
            url: row.get("url"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(url) => Ok(url),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    pub async fn add_user(
        &self,
        new_user: database_types::NewUser,
    ) -> Result<database_types::User, Error> {
        match sqlx::query(
            "INSERT INTO contacts.users (name, surname)
           VALUES ($1, $2)
           RETURNING id, name, surname",
        )
        .bind(new_user.name)
        .bind(new_user.surname)
        .map(|row: PgRow| database_types::User {
            id: row.get("id"),
            name: row.get("name"),
            surname: row.get("surname"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(user) => Ok(user),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    pub async fn add_user_category(
        &self,
        user_category: database_types::UserCategory
    ) -> Result<database_types::UserCategory, Error> {
        match sqlx::query(
            "INSERT INTO contacts.users_categories (id_user, id_category)
           VALUES ($1, $2)
           RETURNING id_user, id_category",
        )
        .bind(user_category.id_user)
        .bind(user_category.id_category)
        .map(|row: PgRow| database_types::UserCategory {
            id_user: row.get("id_user"),
            id_category: row.get("id_category"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(user_category_db) => Ok(user_category_db),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    pub async fn add_nickname(
        &self,
        nickname: database_types::Nickname,
    ) -> Result<database_types::Nickname, Error> {
        match sqlx::query(
            "INSERT INTO contacts.nicknames (id_user, nickname)
           VALUES ($1, $2)
           RETURNING id_user, nickname",
        )
        .bind(nickname.id_user)
        .bind(nickname.nickname)
        .map(|row: PgRow| database_types::Nickname {
            id_user: row.get("id_user"),
            nickname: row.get("nickname"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(nickname_db) => Ok(nickname_db),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    pub async fn add_phone(
        &self,
        phone: database_types::Phone,
    ) -> Result<database_types::Phone, Error> {
        match sqlx::query(
            "INSERT INTO contacts.phones (id_user, phone, description)
           VALUES ($1, $2, $3)
           RETURNING id_user, phone, description",
        )
        .bind(phone.id_user)
        .bind(phone.phone)
        .bind(phone.description)
        .map(|row: PgRow| database_types::Phone {
            id_user: row.get("id_user"),
            phone: row.get("phone"),
            description: row.get("description"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(phone_db) => Ok(phone_db),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    pub async fn get_all_data_by_query(
        &self,
        query: String,
    ) -> Result<Vec<database_types::AllData>, Error> {
        let mut query = query.to_lowercase();
        query = query
            .replace("á", "a")
            .replace("é", "e")
            .replace("í", "i")
            .replace("ó", "o")
            .replace("ú", "u");
        query = format!("%{}%", query);
        match sqlx::query(
            "SELECT
               *
             from
               contacts.all_data
             WHERE
               id IN (
                 SELECT
                   id
                 from
                   contacts.all_data
                 WHERE
                   TRANSLATE(
                     LOWER(
                       CONCAT_WS(
                         ' ',
                         name,
                         surname,
                         nickname,
                         phone,
                         phone_description,
                         category,
                         address,
                         email,
                         url,
                         facebook_url,
                         twitter_handle,
                         instagram_handle,
                         note
                       )
                     ),
                     'áéíóú',
                     'aeiou'
                   ) LIKE $1
               )
             ORDER BY
               LOWER(
                 CONCAT_WS(' ', name, surname)
               ) ASC;
             ",
        )
        .bind(query)
        .map(|row: PgRow| database_types::AllData {
            user_id: row.get("id"),
            user_name: row.get("name"),
            user_surname: row.get("surname"),
            nickname: row.get("nickname"),
            phone: row.get("phone"),
            phone_description: row.get("phone_description"),
            category: row.get("category"),
            address: row.get("address"),
            email: row.get("email"),
            url: row.get("url"),
            facebook_url: row.get("facebook_url"),
            twitter_handle: row.get("twitter_handle"),
            instagram_handle: row.get("instagram_handle"),
            note: row.get("note"),
        })
        .fetch_all(&self.connection)
        .await
        {
            Ok(all_data) => Ok(all_data),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    pub async fn get_all_data_by_id(&self, id: i32) -> Result<Vec<database_types::AllData>, Error> {
        match sqlx::query("SELECT * from contacts.all_data WHERE id = $1;")
            .bind(id)
            .map(|row: PgRow| database_types::AllData {
                user_id: row.get("id"),
                user_name: row.get("name"),
                user_surname: row.get("surname"),
                nickname: row.get("nickname"),
                phone: row.get("phone"),
                phone_description: row.get("phone_description"),
                category: row.get("category"),
                address: row.get("address"),
                email: row.get("email"),
                url: row.get("url"),
                facebook_url: row.get("facebook_url"),
                twitter_handle: row.get("twitter_handle"),
                instagram_handle: row.get("instagram_handle"),
                note: row.get("note"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(all_data) => Ok(all_data),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }
}
