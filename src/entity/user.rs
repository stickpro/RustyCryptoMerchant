use chrono::{DateTime, Utc};
use fake::faker::internet::en::{FreeEmail, Password};
use fake::Dummy;
use sea_orm::entity::prelude::*;

use crate::error::ResourceType;

use super::AppEntity;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Dummy, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[dummy(faker = "FreeEmail()")]
    #[sea_orm(columnt_type = "Text", unique, index)]
    pub email: String,
    #[dummy(faker = "Password(8..100)")]
    #[sea_orm(column_type = "Text")]
    pub password: String,
    pub processing_owner_id: String,
    pub is_active: bool,
    pub is_2fa: bool,
    pub secret_2fa: String,
    pub phone: String,
    pub location: String,
    pub language: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AppEntity for Model {
    const RESOURCE: ResourceType = ResourceType::User;
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::store::Entity")]
    Store
}

impl Related<super::store::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Store::def()
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}