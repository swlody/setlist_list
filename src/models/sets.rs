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
    pub creator_id: Uuid,
    pub dj_names: Vec<String>,
    pub venue: Option<String>,
    pub city: Option<String>,
    pub event_name: Option<String>,
    pub event_date: NaiveDate,
    pub doors_time: Option<NaiveDateTime>,
    pub scheduled_start: Option<NaiveDateTime>,
    pub actual_start: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub setlist: Option<JsonValue>,
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
            "INSERT INTO sets (id, creator_id, dj_names, venue, city, event_name, event_date, doors_time, scheduled_start, actual_start, end_time, setlist) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
            self.id,
            self.creator_id,
            &self.dj_names,
            self.venue,
            self.city,
            self.event_name,
            self.event_date,
            self.doors_time,
            self.scheduled_start,
            self.actual_start,
            self.end_time,
            self.setlist
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn update(&self, db: &PgPool) -> ModelResult<()> {
        sqlx::query!(
            "UPDATE sets SET dj_names = $1, venue = $2, city = $3, event_name = $4, event_date = $5, doors_time = $6, scheduled_start = $7, actual_start = $8, end_time = $9, setlist = $10 WHERE id = $11",
            &self.dj_names,
            self.venue,
            self.city,
            self.event_name,
            self.event_date,
            self.doors_time,
            self.scheduled_start,
            self.actual_start,
            self.end_time,
            self.setlist,
            self.id
        )
        .execute(db)
        .await?;
        Ok(())
    }
}
