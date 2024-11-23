use crate::_entities::prelude::TeamUsers;
use crate::_entities::sea_orm_active_enums::{TeamUserRoles, TeamUserStatus};
use crate::_entities::{self, team_users, teams, users};
use crate::utils::DbResult;
use sea_orm::sea_query::Expr;
use sea_orm::sqlx::types::chrono;
use sea_orm::{
    sqlx::types::Uuid, DatabaseConnection, EntityTrait, EnumIter, Related, RelationDef,
    RelationTrait,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DbErr, FromQueryResult, JoinType, QueryFilter,
    QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    Team,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => team_users::Entity::belongs_to(users::Entity)
                .from(team_users::Column::UserId)
                .to(users::Column::UserId)
                .into(),
            Self::Team => team_users::Entity::belongs_to(teams::Entity)
                .from(team_users::Column::TeamId)
                .to(teams::Column::TeamId)
                .into(),
        }
    }
}

impl Related<_entities::users::Entity> for team_users::Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<_entities::teams::Entity> for team_users::Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}

pub struct TeamUserService {
    db: Arc<DatabaseConnection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeftTeamDto {
    #[serde(rename = "teamId")]
    pub team_id: Uuid,
    #[serde(rename = "userId")]
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinTeamDto {
    #[serde(rename = "teamId")]
    pub team_id: Uuid,
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    pub role: TeamUserRoles,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct TeamOfUser {
    #[serde(rename = "teamId")]
    pub team_id: Uuid,
    #[serde(rename = "teamName")]
    pub team_name: String,
    #[serde(rename = "teamAvatar")]
    pub team_avatar: String,
    #[serde(rename = "teamUniqueId")]
    pub team_unique_id: String,
    #[serde(rename = "teamNamespace")]
    pub team_namespace: String,
    pub description: String,
    pub role: TeamUserRoles,
    #[serde(rename = "joinedAt")]
    pub joined_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct UserOfTeam {
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    pub username: String,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub email: String,
    pub avatar: Option<String>,
    pub role: TeamUserRoles,
    pub status: TeamUserStatus,
    #[serde(rename = "joinedAt")]
    pub joined_at: String,
    #[serde(rename = "leftedAt")]
    pub lefted_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct FindRoleVo {
    role: TeamUserRoles,
}

impl TeamUserService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn get_user_role_by_team(
        &self,
        team_id: Uuid,
        user_id: Uuid,
    ) -> DbResult<TeamUserRoles> {
        let role = TeamUsers::find()
            .select_only()
            .column(team_users::Column::Role)
            .filter(team_users::Column::TeamId.eq(team_id))
            .filter(team_users::Column::UserId.eq(user_id))
            .into_model::<FindRoleVo>()
            .one(self.db.as_ref())
            .await?;

        match role {
            Some(val) => Ok(val.role),
            None => Err(DbErr::Custom(
                "Not found role for this team and current user".to_string(),
            )),
        }
    }

    pub async fn join_team(&self, payload: JoinTeamDto) -> DbResult<()> {
        // Check if user is already in team
        let existing = TeamUsers::find()
            .filter(team_users::Column::TeamId.eq(payload.team_id))
            .filter(team_users::Column::UserId.eq(payload.user_id))
            .one(self.db.as_ref())
            .await?;

        if let Some(_) = existing {
            return Err(DbErr::Custom("User is already in team".to_string()));
        }

        let model = team_users::ActiveModel {
            team_id: Set(payload.team_id),
            user_id: Set(payload.user_id),
            role: Set(payload.role),
            status: Set(TeamUserStatus::Joined),
            lefted_at: Set(None),
            ..Default::default()
        };

        model.insert(self.db.as_ref()).await?;
        Ok(())
    }

    pub async fn left_team(&self, payload: LeftTeamDto) -> DbResult<()> {
        let team_user = TeamUsers::find()
            .filter(team_users::Column::UserId.eq(payload.user_id))
            .filter(team_users::Column::TeamId.eq(payload.team_id))
            .one(self.db.as_ref())
            .await?;

        match team_user {
            Some(model) => {
                let mut model: team_users::ActiveModel = model.into();
                model.status = Set(TeamUserStatus::Lefted);
                model.lefted_at = Set(Some(chrono::Utc::now().naive_utc()));
                model.update(self.db.as_ref()).await?;
                Ok(())
            }
            None => Ok(()),
        }
    }

    pub async fn find_teams_by_user(&self, user_id: Uuid) -> DbResult<Vec<TeamOfUser>> {
        TeamUsers::find()
            .select_only()
            .column_as(teams::Column::TeamId, "team_id")
            .column_as(teams::Column::TeamName, "team_name")
            .column_as(teams::Column::TeamAvatar, "team_avatar")
            .column_as(teams::Column::TeamUniqueId, "team_unique_id")
            .column_as(teams::Column::TeamNamespace, "team_namespace")
            .column_as(teams::Column::Description, "description")
            .column_as(team_users::Column::Status, "status")
            .column_as(
                Expr::cust("to_char(team_users.joined_at, 'YYYY-MM-DD HH:mm:ss')"),
                "joined_at",
            )
            .column(team_users::Column::Role)
            .join(JoinType::LeftJoin, Relation::User.def())
            .join(JoinType::LeftJoin, Relation::Team.def())
            .filter(
                Condition::any()
                    .add(team_users::Column::UserId.eq(user_id))
                    .add(team_users::Column::Status.eq(TeamUserStatus::Joined)),
            )
            .into_model::<TeamOfUser>()
            .all(self.db.as_ref())
            .await
    }

    pub async fn find_users_by_team(&self, team_id: Uuid) -> DbResult<Vec<UserOfTeam>> {
        TeamUsers::find()
            .select_only()
            .column_as(users::Column::UserId, "user_id")
            .column_as(users::Column::Username, "username")
            .column_as(users::Column::DisplayName, "display_name")
            .column_as(users::Column::Email, "email")
            .column_as(users::Column::Avatar, "avatar")
            .column_as(
                Expr::cust("to_char(team_users.joined_at, 'YYYY-MM-DD HH:mm:ss')"),
                "joined_at",
            )
            .column_as(
                Expr::cust("to_char(team_users.lefted_at, 'YYYY-MM-DD HH:mm:ss')"),
                "lefted_at",
            )
            .column(team_users::Column::Status)
            .column(team_users::Column::Role)
            .join(JoinType::LeftJoin, Relation::User.def())
            .filter(team_users::Column::TeamId.eq(team_id))
            .order_by(team_users::Column::Status, sea_orm::Order::Desc)
            .into_model::<UserOfTeam>()
            .all(self.db.as_ref())
            .await
    }

    pub async fn check_user_in_team(&self, user_id: Uuid, team_id: Uuid) -> DbResult<bool> {
        let exists = TeamUsers::find()
            .filter(team_users::Column::UserId.eq(user_id))
            .filter(team_users::Column::TeamId.eq(team_id))
            .filter(team_users::Column::Status.eq(TeamUserStatus::Joined))
            .one(self.db.as_ref())
            .await?;

        Ok(exists.is_some())
    }

    pub async fn left_all_relation_by_user(&self, user_id: Uuid) -> DbResult<()> {
        TeamUsers::update_many()
            .col_expr(
                team_users::Column::Status,
                Expr::value(TeamUserStatus::Lefted),
            )
            .col_expr(
                team_users::Column::LeftedAt,
                Expr::value(chrono::Utc::now()),
            )
            .filter(team_users::Column::UserId.eq(user_id))
            .exec(self.db.as_ref())
            .await?;

        Ok(())
    }

    pub async fn clean_relation_by_team(&self, team_id: Uuid) -> DbResult<()> {
        TeamUsers::delete_many()
            .filter(team_users::Column::TeamId.eq(team_id))
            .exec(self.db.as_ref())
            .await?;

        Ok(())
    }
}
