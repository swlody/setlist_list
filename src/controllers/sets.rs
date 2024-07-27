#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use auth::JWTWithUser;
use axum::{debug_handler, http::uri::PathAndQuery};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    initializers::minijinja_view_engine::MiniJinjaView,
    models::{sets, users},
    utils::{get_username, hx_redirect},
    views,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub band_name: Option<String>,
    pub date: chrono::NaiveDate,
    pub venue: Option<String>,
    pub setlist: Option<serde_json::Value>,
}

impl Params {
    fn update(&self, item: &mut sets::Model) {
        item.band_name.clone_from(&self.band_name);
        item.date = self.date;
        item.venue.clone_from(&self.venue);
        item.setlist.clone_from(&self.setlist);
    }
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
    let uuid = uuid::Uuid::parse_str(&auth.claims.id).map_err(|e| ModelError::Any(e.into()))?;
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
