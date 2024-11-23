use extension::postgres::Type;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
#[sea_orm(enum_name = "team_user_status")]
pub enum TeamUserStatus {
    #[sea_orm(iden = "team_user_status")]
    Enum,
    #[sea_orm(iden = "joined")]
    Joined,
    #[sea_orm(iden = "lefted")]
    Lefted,
}

#[derive(DeriveIden)]
#[sea_orm(enum_name = "TeamRoles")]
pub enum TeamUserRoles {
    #[sea_orm(iden = "team_user_roles")]
    Enum,
    #[sea_orm(iden = "Owner", comment = "Highest level of access")]
    Owner,
    #[sea_orm(
        iden = "Admin",
        comment = "Administrator with full management rights except billing"
    )]
    Admin,
    #[sea_orm(iden = "Manager", comment = "Manage projects and team activities")]
    Manager,
    #[sea_orm(iden = "Member", comment = "Access and contribute to projects")]
    Member,
    #[sea_orm(
        iden = "Guest",
        comment = "View-only access for external collaborators"
    )]
    Guest,
}

#[derive(DeriveIden)]
enum TeamUsers {
    Table,
    TeamId,
    UserId,
    Role,
    Status,
    JoinedAt,
    LeftedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(TeamUserStatus::Enum)
                    .values(vec![TeamUserStatus::Joined, TeamUserStatus::Lefted])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(TeamUserRoles::Enum)
                    .values(vec![
                        TeamUserRoles::Owner,
                        TeamUserRoles::Admin,
                        TeamUserRoles::Manager,
                        TeamUserRoles::Member,
                        TeamUserRoles::Guest,
                    ])
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
                    .col(
                        date_time(TeamUsers::JoinedAt)
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into())),
                    )
                    .col(date_time_null(TeamUsers::LeftedAt))
                    .col(
                        ColumnDef::new(TeamUsers::Role)
                            .custom(TeamUserRoles::Enum)
                            .default(TeamUserRoles::Member.to_string())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TeamUsers::Status)
                            .custom(TeamUserStatus::Enum)
                            .default(TeamUserStatus::Joined.to_string())
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(TeamUsers::TeamId)
                            .col(TeamUsers::UserId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TeamUsers::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(TeamUserRoles::Enum).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(TeamUserStatus::Enum).to_owned())
            .await
    }
}
