//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(schema_name = "mm_auth", table_name = "team_roles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub team_role_id: Uuid,
    pub team_id: Uuid,
    pub role_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
