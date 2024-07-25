use loco_rs::{
    model::{ModelError, ModelResult},
    prelude::*,
};
use sea_orm::RuntimeErr;
use serde::{Deserialize, Serialize};
use sqlx::types::{
    chrono::{NaiveDate, NaiveDateTime},
    JsonValue, Uuid,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Model {
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id: i32,
    pub band_name: Option<String>,
    pub date: NaiveDate,
    pub venue: Option<String>,
    pub setlist: Option<JsonValue>,
    pub creator_pid: Uuid,
}

impl Model {
    pub async fn list_by_creator_pid(db: &DatabaseConnection, pid: Uuid) -> ModelResult<Vec<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM sets WHERE creator_pid = $1", pid)
            .fetch_all(db.get_postgres_connection_pool())
            .await
            .map_err(|e| ModelError::DbErr(DbErr::Query(RuntimeErr::SqlxError(e))))
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        let set = sqlx::query_as!(Self, "SELECT * FROM sets WHERE id = $1", id)
            .fetch_optional(db.get_postgres_connection_pool())
            .await
            .map_err(|e| ModelError::DbErr(DbErr::Query(RuntimeErr::SqlxError(e))))?;
        set.ok_or(ModelError::EntityNotFound)
    }

    pub async fn delete_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<()> {
        sqlx::query!("DELETE FROM sets WHERE id = $1", id)
            .execute(db.get_postgres_connection_pool())
            .await
            .map_err(|e| ModelError::DbErr(DbErr::Query(RuntimeErr::SqlxError(e))))?;
        Ok(())
    }

    pub async fn list(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM sets")
            .fetch_all(db.get_postgres_connection_pool())
            .await
            .map_err(|e| ModelError::DbErr(DbErr::Query(RuntimeErr::SqlxError(e))))
    }

    pub async fn insert(&self, db: &DatabaseConnection) -> ModelResult<()> {
        sqlx::query!(
            "INSERT INTO sets (band_name, date, venue, setlist, creator_pid) VALUES ($1, $2, $3, $4, $5)",
            self.band_name,
            self.date,
            self.venue,
            self.setlist,
            self.creator_pid
        )
        .execute(db.get_postgres_connection_pool())
        .await
        .map_err(|e| ModelError::DbErr(DbErr::Query(RuntimeErr::SqlxError(e))))?;
        Ok(())
    }

    pub async fn update(&self, db: &DatabaseConnection) -> ModelResult<()> {
        sqlx::query!(
            "UPDATE sets SET band_name = $1, date = $2, venue = $3, setlist = $4 WHERE id = $5",
            self.band_name,
            self.date,
            self.venue,
            self.setlist,
            self.id
        )
        .execute(db.get_postgres_connection_pool())
        .await
        .map_err(|e| ModelError::DbErr(DbErr::Query(RuntimeErr::SqlxError(e))))?;
        Ok(())
    }
}
