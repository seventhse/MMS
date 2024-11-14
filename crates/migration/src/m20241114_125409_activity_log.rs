use async_trait::async_trait;
use extension::postgres::Type;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
#[sea_orm(enum_name = "actions")]
pub enum Actions {
    #[sea_orm(iden = "actions")]
    Enum,
    #[sea_orm(iden = "created")]
    Created,
    #[sea_orm(iden = "updated")]
    Updated,
    #[sea_orm(iden = "removed")]
    Removed,
}

#[derive(DeriveIden)]
#[sea_orm(enum_name = "target_types")]
pub enum TargetTypes {
    #[sea_orm(iden = "target_types")]
    Enum,
    #[sea_orm(iden = "user")]
    User,
    #[sea_orm(iden = "team")]
    Team,
    #[sea_orm(iden = "role")]
    Role,
}

#[derive(DeriveIden)]
enum ActivityLog {
    Table,
    LogId,
    UserId,
    ActionType,
    Description,
    TargetId,
    TargetType,
    CreatedAt,
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Actions::Enum)
                    .values(vec![Actions::Created, Actions::Updated, Actions::Removed])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(TargetTypes::Enum)
                    .values(vec![
                        TargetTypes::User,
                        TargetTypes::Team,
                        TargetTypes::Role,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ActivityLog::Table)
                    .if_not_exists()
                    .col(
                        uuid(ActivityLog::LogId)
                            .default(SimpleExpr::Custom("gen_random_uuid()".into()))
                            .not_null()
                            .primary_key(),
                    )
                    .col(uuid(ActivityLog::UserId).not_null())
                    .col(
                        ColumnDef::new(ActivityLog::ActionType)
                            .custom(Actions::Enum)
                            .not_null(),
                    )
                    .col(string(ActivityLog::Description))
                    .col(uuid(ActivityLog::TargetId).not_null())
                    .col(
                        ColumnDef::new(ActivityLog::TargetType)
                            .custom(TargetTypes::Enum)
                            .not_null(),
                    )
                    .col(
                        date_time(ActivityLog::CreatedAt)
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into())),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ActivityLog::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(Actions::Enum).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(Actions::Enum).to_owned())
            .await
    }
}
