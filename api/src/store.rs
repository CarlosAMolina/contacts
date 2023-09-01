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

    pub async fn add_user(
        &self,
        new_user: database_types::NewUser,
    ) -> Result<database_types::User, Error> {
        match sqlx::query(
            "INSERT INTO contacts.user (name, surname)
           VALUES ($1, $2)
           RETURNING name, surname",
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

    pub async fn get_all_data_by_query(&self, query: String) -> Result<Vec<database_types::AllData>, Error> {
        let mut query = query.to_lowercase();
        query = query
            .replace("á", "a")
            .replace("é", "e")
            .replace("í", "i")
            .replace("ó", "o")
            .replace("ú", "u")
            ;
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
