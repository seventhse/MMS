use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Teams {
    Table,
    TeamId,
    TeamUniqueId,
    TeamName,
    TeamAvatar,
    TeamNamespace,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Teams::Table)
                    .if_not_exists()
                    .col(
                        uuid(Teams::TeamId)
                            .default(SimpleExpr::Custom("gen_random_uuid()".into()))
                            .not_null()
                            .primary_key()
                            .comment("Team unique identifier"),
                    )
                    .col(string(Teams::TeamUniqueId).not_null())
                    .col(string(Teams::TeamName).not_null().comment("Team name"))
                    .col(string_null(Teams::TeamAvatar).comment("Team avatar"))
                    .col(string(Teams::TeamNamespace).not_null())
                    .col(string_null(Teams::Description).comment("Team description"))
                    .col(
                        date_time(Teams::CreatedAt)
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into())),
                    )
                    .col(
                        date_time(Teams::UpdatedAt)
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into())),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Teams::Table).to_owned())
            .await
    }
}
