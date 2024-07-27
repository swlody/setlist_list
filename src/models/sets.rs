use chrono::NaiveDateTime;
use loco_rs::model::{ModelError, ModelResult};
use serde::{Deserialize, Serialize};
use sqlx::{
    types::{chrono::NaiveDate, JsonValue, Uuid},
    PgPool,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Model {
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id: Uuid,
    pub band_name: Option<String>,
    pub date: NaiveDate,
    pub venue: Option<String>,
    pub setlist: Option<JsonValue>,
    pub creator_id: Uuid,
}

impl Model {
    pub async fn list_by_creator_id(db: &PgPool, id: Uuid) -> ModelResult<Vec<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM sets WHERE creator_id = $1", id)
                .fetch_all(db)
                .await?,
        )
    }

    pub async fn find_by_id(db: &PgPool, id: Uuid) -> ModelResult<Self> {
        let set = sqlx::query_as!(Self, "SELECT * FROM sets WHERE id = $1", id)
            .fetch_optional(db)
            .await?;
        set.ok_or(ModelError::EntityNotFound)
    }

    pub async fn delete_by_id(db: &PgPool, id: Uuid) -> ModelResult<()> {
        sqlx::query!("DELETE FROM sets WHERE id = $1", id)
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn list_all(db: &PgPool) -> ModelResult<Vec<Self>> {
        Ok(sqlx::query_as!(Self, "SELECT * FROM sets")
            .fetch_all(db)
            .await?)
    }

    pub async fn insert(&self, db: &PgPool) -> ModelResult<()> {
        sqlx::query!(
            "INSERT INTO sets (id, band_name, date, venue, setlist, creator_id) VALUES ($1, $2, $3, $4, $5, $6)",
            self.id,
            self.band_name,
            self.date,
            self.venue,
            self.setlist,
            self.creator_id
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn update(&self, db: &PgPool) -> ModelResult<()> {
        sqlx::query!(
            "UPDATE sets SET band_name = $1, date = $2, venue = $3, setlist = $4 WHERE id = $5",
            self.band_name,
            self.date,
            self.venue,
            self.setlist,
            self.id
        )
        .execute(db)
        .await?;
        Ok(())
    }
}
