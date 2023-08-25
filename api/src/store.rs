use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::types::database::AllData;
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

    pub async fn get_all_data_by_query(&self, query: String) -> Result<Vec<AllData>, Error> {
        let query = format!("%{}%", query.to_lowercase());
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
                   ) LIKE $1
               )
             ORDER BY
               LOWER(
                 CONCAT_WS(' ', name, surname)
               ) ASC;
             ",

        )
        .bind(query)
        .map(|row: PgRow| AllData {
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

    pub async fn get_all_data_by_id(&self, id: i32) -> Result<Vec<AllData>, Error> {
        match sqlx::query("SELECT * from contacts.all_data WHERE id = $1;")
            .bind(id)
            .map(|row: PgRow| AllData {
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
