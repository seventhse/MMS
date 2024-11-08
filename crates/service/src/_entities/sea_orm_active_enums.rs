//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "actions")]
pub enum Actions {
    #[sea_orm(string_value = "created")]
    Created,
    #[sea_orm(string_value = "removed")]
    Removed,
    #[sea_orm(string_value = "updated")]
    Updated,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "status")]
pub enum Status {
    #[sea_orm(string_value = "active")]
    Active,
    #[sea_orm(string_value = "inactive")]
    Inactive,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "target_types")]
pub enum TargetTypes {
    #[sea_orm(string_value = "role")]
    Role,
    #[sea_orm(string_value = "team")]
    Team,
    #[sea_orm(string_value = "user")]
    User,
}
