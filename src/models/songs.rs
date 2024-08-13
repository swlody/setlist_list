use loco_rs::model::ModelResult;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
struct Model {
    setlist_id: Uuid,
    track_title: String,
    track_artist: String,
    track_number: Option<i32>,
    track_start_time_offset_seconds: Option<i32>,
    track_duration_seconds: Option<i32>,
}

impl Model {
    pub fn from_song_and_setlist_id(
        song: crate::controllers::sets::Song,
        setlist_id: Uuid,
    ) -> Self {
        Self {
            setlist_id,
            track_title: song.track_title,
            track_artist: song.track_artist,
            track_number: song.track_number,
            track_start_time_offset_seconds: song.track_start_time_offset_seconds,
            track_duration_seconds: song.track_duration_seconds,
        }
    }

    pub async fn list_by_setlist_id(db: &PgPool, setlist_id: Uuid) -> ModelResult<Vec<Self>> {
        Ok(sqlx::query_as!(
            Self,
            "SELECT * FROM setlist_songs WHERE setlist_id = $1",
            setlist_id
        )
        .fetch_all(db)
        .await?)
    }

    pub async fn insert_many(db: &PgPool, many: &[Self]) -> ModelResult<()> {
        for song in many {
            song.insert(db).await?;
        }
        Ok(())
    }

    pub async fn insert(&self, db: &PgPool) -> ModelResult<()> {
        sqlx::query_as!(
            Self,
            r#"INSERT INTO setlist_songs
            (setlist_id, track_title, track_artist, track_number, track_start_time_offset_seconds, track_duration_seconds) VALUES ($1, $2, $3, $4, $5, $6)"#,
            self.setlist_id,
            &self.track_title,
            &self.track_artist,
            self.track_number,
            self.track_start_time_offset_seconds,
            self.track_duration_seconds
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn update(&self, db: &PgPool, many: &[Self], setlist_id: Uuid) -> ModelResult<()> {
        // TODO this is so dumb
        sqlx::query!(
            "DELETE FROM setlist_songs WHERE setlist_id = $1",
            setlist_id
        )
        .execute(db)
        .await?;

        Self::insert_many(db, many).await
    }
}
