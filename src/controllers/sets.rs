#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use auth::JWTWithUser;
use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::{sea_query::Order, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::{
    initializers::minijinja_view_engine::MiniJinjaView,
    models::{
        _entities::sets::{ActiveModel, Column, Entity, Model},
        users,
    },
    utils::get_user_name,
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
    fn update(&self, item: &mut ActiveModel) {
        item.band_name = Set(self.band_name.clone());
        item.date = Set(self.date);
        item.venue = Set(self.venue.clone());
        item.setlist = Set(self.setlist.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(
    jwt_user: Option<JWTWithUser<users::Model>>,
    ViewEngine(v): ViewEngine<MiniJinjaView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = Entity::find()
        .order_by(Column::Id, Order::Desc)
        .all(&ctx.db)
        .await;
    let user_name = get_user_name(jwt_user).unwrap_or_default();
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
    let user_name = get_user_name(jwt_user).unwrap_or_default();
    views::sets::create(&v, &user_name)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn edit(
    jwt_user: Option<JWTWithUser<users::Model>>,
    Path(id): Path<i32>,
    ViewEngine(v): ViewEngine<MiniJinjaView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await;
    let user_name = get_user_name(jwt_user).unwrap_or_default();
    if let Ok(item) = item {
        views::sets::edit(&v, &item, &user_name)
    } else {
        views::index::not_found(&v, &user_name)
    }
}

#[debug_handler]
pub async fn show(
    jwt_user: Option<JWTWithUser<users::Model>>,
    Path(id): Path<i32>,
    ViewEngine(v): ViewEngine<MiniJinjaView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await;
    let user_name = get_user_name(jwt_user).unwrap_or_default();
    if let Ok(item) = item {
        views::sets::show(&v, &item, &user_name)
    } else {
        views::index::not_found(&v, &user_name)
    }
}

#[debug_handler]
pub async fn add(
    jwt_user: Option<JWTWithUser<users::Model>>,
    ViewEngine(v): ViewEngine<MiniJinjaView>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    let user_name = get_user_name(jwt_user).unwrap_or_default();
    views::sets::show(&v, &item, &user_name)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
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
