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

#[derive(DeriveIden)]
enum TeamUsers {
    Table,
    TeamId,
    UserId,
    TeamRoleId,
    Status,
    JoinedAt,
    LeftedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    RoleId,
    RoleName,
    Description,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum TeamRoles {
    Table,
    TeamRoleId,
    TeamId,
    RoleId,
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
                    .as_enum(Status::Enum)
                    .values(vec![Status::Active, Status::Inactive])
                    .to_owned(),
            )
            .await?;
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
            .await?;

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
                    .col(string(Teams::TeamNamespace).unique_key().not_null())
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
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(
                        uuid(Roles::RoleId)
                            .default(SimpleExpr::Custom("gen_random_uuid()".into()))
                            .not_null()
                            .primary_key()
                            .comment("Role unique identifier"),
                    )
                    .col(string(Roles::RoleName).not_null().comment("Role name"))
                    .col(string_null(Roles::Description).comment("Role description"))
                    .col(
                        ColumnDef::new(Roles::Status)
                            .custom(Status::Enum)
                            .default(Status::Active.to_string())
                            .not_null(),
                    )
                    .col(
                        date_time(Roles::CreatedAt)
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into())),
                    )
                    .col(
                        date_time(Roles::UpdatedAt)
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into())),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TeamRoles::Table)
                    .if_not_exists()
                    .col(
                        uuid(TeamRoles::TeamRoleId)
                            .default(SimpleExpr::Custom("gen_random_uuid()".into()))
                            .not_null()
                            .primary_key(),
                    )
                    .col(uuid(TeamRoles::TeamId).not_null())
                    .col(uuid(TeamRoles::RoleId).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TeamUsers::Table)
                    .if_not_exists()
                    .col(uuid(TeamUsers::TeamId).not_null())
                    .col(uuid(TeamUsers::UserId).not_null())
                    .col(uuid(TeamUsers::TeamRoleId).not_null())
                    .col(
                        ColumnDef::new(TeamUsers::Status)
                            .custom(Status::Enum)
                            .default(Status::Active.to_string())
                            .not_null(),
                    )
                    .col(
                        date_time(TeamUsers::JoinedAt)
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into())),
                    )
                    .col(date_time(TeamUsers::LeftedAt))
                    .col(
                        date_time(TeamUsers::UpdatedAt)
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into())),
                    )
                    .primary_key(
                        Index::create()
                            .col(TeamUsers::TeamId)
                            .col(TeamUsers::UserId),
                    )
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
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TeamUsers::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ActivityLog::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(TeamRoles::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Teams::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Roles::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(Status::Enum).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(Actions::Enum).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(Actions::Enum).to_owned())
            .await?;

        Ok(())
    }
}
