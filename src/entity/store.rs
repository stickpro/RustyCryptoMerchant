use chrono::{DateTime, Utc};
use sea_orm::prelude::{Uuid};
use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;

use crate::error::ResourceType;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "stores")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub site: String,
    pub currency_id: String,
    pub rate_source: String,
    pub return_url: String,
    pub success_url: String,
    pub rate_scale: Decimal,
    pub status: bool,
    pub create_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}

impl super::AppEntity for Model {
    const RESOURCE: crate::error::ResourceType = ResourceType::Store;
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::UserId",
    to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}


#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}