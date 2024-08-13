use chrono::NaiveDateTime;
use loco_rs::model::{ModelError, ModelResult};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, PgPool};

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
    pub start_time: NaiveDateTime,
    pub duration_seconds: Option<i32>,
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
            r#"INSERT INTO sets (id, creator_id, dj_names, venue, city, event_name, start_time, duration_seconds)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
            self.id,
            self.creator_id,
            &self.dj_names,
            self.venue,
            self.city,
            self.event_name,
            self.start_time,
            self.duration_seconds
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn update(&self, db: &PgPool) -> ModelResult<()> {
        sqlx::query!(
            r#"UPDATE sets
            SET dj_names = $1, venue = $2, city = $3, event_name = $4, start_time = $5, duration_seconds = $6"#,
            &self.dj_names,
            self.venue,
            self.city,
            self.event_name,
            self.start_time,
            self.duration_seconds,
        )
        .execute(db)
        .await?;
        Ok(())
    }
}
