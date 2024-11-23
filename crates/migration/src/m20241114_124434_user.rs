use async_trait::async_trait;
use extension::postgres::Type;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
#[sea_orm(enum_name = "status")]
pub enum Status {
    #[sea_orm(iden = "status")]
    Enum,
    #[sea_orm(iden = "active")]
    Active,
    #[sea_orm(iden = "inactive")]
    Inactive,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    UserId,
    Email,
    Password,
    UniqueId,
    Username,
    DisplayName,
    Avatar,
    DefaultTeamId,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Status::Enum)
                    .values(vec![Status::Active, Status::Inactive])
                    .to_owned(),
            )
            .await?;

        let user_table = Table::create()
            .table(Users::Table)
            .if_not_exists()
            .col(
                uuid(Users::UserId)
                    .default(SimpleExpr::Custom("gen_random_uuid()".into()))
                    .not_null()
                    .primary_key()
                    .comment("User unique identifier"),
            )
            .col(string(Users::UniqueId).not_null())
            .col(
                string(Users::Email)
                    .not_null()
                    .comment("User email address"),
            )
            .col(string_null(Users::Username).comment("User username"))
            .col(string_null(Users::DisplayName).comment("User display name"))
            .col(string_null(Users::Avatar).comment("User avatar"))
            .col(uuid_null(Users::DefaultTeamId).comment("User default team ID"))
            .col(
                string(Users::Password)
                    .not_null()
                    .comment("User hashed password"),
            )
            .col(
                ColumnDef::new(Users::Status)
                    .custom(Status::Enum)
                    .default(Status::Active.to_string())
                    .not_null(),
            )
            .col(
                date_time(Users::CreatedAt).default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into())),
            )
            .col(
                date_time(Users::UpdatedAt).default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into())),
            )
            .to_owned();
        manager.create_table(user_table).await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-users-email")
                    .table(Users::Table)
                    .col(Users::Email)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(Status::Enum).to_owned())
            .await
    }
}
