use auth::JWTWithUser;
use axum::{debug_handler, http::uri::PathAndQuery};
use chrono::{NaiveDateTime, Utc};
use loco_rs::prelude::*;
use serde::{de, Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use crate::{
    initializers::minijinja_view_engine::MiniJinjaView,
    models::{sets, users},
    utils::{get_username, hx_redirect},
    views,
};

fn html_datetime<'de, D>(d: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    let formatted = if s.len() == 16 {
        // "yyyy-MM-ddTHH:mm" has length 16
        format!("{s}:00")
    } else {
        s
    };

    NaiveDateTime::parse_from_str(&formatted, "%Y-%m-%dT%H:%M:%S").map_err(de::Error::custom)
}

#[derive(Serialize, Deserialize)]
pub struct Params {
    pub dj_names: Vec<String>,
    pub venue: Option<String>,
    pub city: Option<String>,
    pub event_name: Option<String>,
    #[serde(deserialize_with = "html_datetime")]
    pub start_time: NaiveDateTime,
    pub duration_seconds: Option<i32>,
    pub setlist: Vec<Song>,
}

impl Params {
    fn update(self, item: &mut sets::Model) {
        item.updated_at = Utc::now().naive_utc();
        item.dj_names = self.dj_names;
        item.venue = self.venue;
        item.city = self.city;
        item.event_name = self.event_name;
        item.start_time = self.start_time;
        item.duration_seconds = self.duration_seconds;
    }
}

#[derive(Serialize, Deserialize)]
pub struct Song {
    pub track_title: String,
    pub track_artist: String,
    pub track_number: Option<i32>,
    pub track_start_time_offset_seconds: Option<i32>,
    pub track_duration_seconds: Option<i32>,
}

async fn load_item(ctx: &AppContext, id: Uuid) -> Result<sets::Model> {
    Ok(sets::Model::find_by_id(&ctx.db, id).await?)
}

#[debug_handler]
pub async fn list(
    jwt_user: Option<JWTWithUser<users::Model>>,
    ViewEngine(v): ViewEngine<MiniJinjaView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user_name = get_username(jwt_user).unwrap_or_default();
    let item = sets::Model::list_all(&ctx.db).await;
    if let Ok(item) = item {
        views::sets::list(&v, &item, &user_name)
    } else {
        views::index::not_found(&v, &user_name)
    }
}

#[debug_handler]
pub async fn new(
    jwt_user: Option<JWTWithUser<users::Model>>,
    ViewEngine(v): ViewEngine<MiniJinjaView>,
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    let user_name = get_username(jwt_user).unwrap_or_default();
    if user_name.is_empty() {
        views::index::unauthorized(&v)
    } else {
        views::sets::create(&v, &user_name)
    }
}

#[debug_handler]
pub async fn update(
    Path(id): Path<Uuid>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let mut item = load_item(&ctx, id).await?;
    params.update(&mut item);
    item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn edit(
    jwt_user: Option<JWTWithUser<users::Model>>,
    Path(id): Path<Uuid>,
    ViewEngine(v): ViewEngine<MiniJinjaView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user_name = get_username(jwt_user).unwrap_or_default();
    if user_name.is_empty() {
        views::index::unauthorized(&v)
    } else {
        let item = load_item(&ctx, id).await;
        if let Ok(item) = item {
            views::sets::edit(&v, &item, &user_name)
        } else {
            views::index::not_found(&v, &user_name)
        }
    }
}

#[debug_handler]
pub async fn show(
    jwt_user: Option<JWTWithUser<users::Model>>,
    path: Option<Path<Uuid>>,
    ViewEngine(v): ViewEngine<MiniJinjaView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user_name = get_username(jwt_user).unwrap_or_default();
    if let Some(Path(id)) = path {
        let item = load_item(&ctx, id).await;
        if let Ok(item) = item {
            views::sets::show(&v, &item, &user_name)
        } else {
            views::index::not_found(&v, &user_name)
        }
    } else {
        views::index::not_found(&v, &user_name)
    }
}

#[debug_handler]
pub async fn add(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let uuid = uuid::Uuid::parse_str(&auth.claims.id)?;
    let mut item = sets::Model {
        id: Uuid::now_v7(),
        creator_id: uuid,
        ..Default::default()
    };
    params.update(&mut item);
    item.insert(&ctx.db).await?;
    hx_redirect(&PathAndQuery::from_static("/sets"))
}

#[debug_handler]
pub async fn remove(Path(id): Path<Uuid>, State(ctx): State<AppContext>) -> Result<Response> {
    sets::Model::delete_by_id(&ctx.db, id).await?;
    hx_redirect(&PathAndQuery::from_static("/sets"))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("sets")
        .add("/", get(list))
        .add("/new", get(new))
        .add("/:id", get(show))
        .add("/:id/edit", get(edit))
        .add("/:id", post(update))
        .add("/:id", delete(remove))
        .add("/", post(add))
}
