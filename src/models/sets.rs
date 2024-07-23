use loco_rs::model::{self, ModelResult};
use sea_orm::entity::prelude::*;

pub use super::_entities::sets::{self, ActiveModel, Entity, Model};

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}

impl super::_entities::sets::Model {
    pub async fn find_by_creator_pid(db: &DatabaseConnection, pid: Uuid) -> ModelResult<Vec<Self>> {
        Ok(sets::Entity::find()
            .filter(
                model::query::condition()
                    .eq(sets::Column::CreatorPid, pid)
                    .build(),
            )
            .all(db)
            .await?)
    }
}
