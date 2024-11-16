use std::sync::Arc;

use crate::_entities::prelude::Teams;
use crate::_entities::teams;
use crate::utils::encrypt::generator_unique_id;
use crate::utils::DbResult;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::Uuid;
use sea_orm::DatabaseConnection;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(FromQueryResult, DerivePartialModel, Serialize)]
#[sea_orm(entity = "Teams")]
pub struct FormatTeam {
    pub team_id: Uuid,
    pub team_unique_id: String,
    pub team_name: String,
    pub team_avatar: Option<String>,
    pub team_namespace: String,
    pub description: Option<String>,
    #[sea_orm(from_expr = "Expr::cust(\"to_char(teams.created_at, 'YYYY-MM-DD HH:mm:ss')\")")]
    pub created_at: Option<String>,
    #[sea_orm(from_expr = "Expr::cust(\"to_char(teams.updated_at, 'YYYY-MM-DD HH:mm:ss')\")")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamDto {
    pub team_name: String,
    pub team_namespace: String,
    pub team_avatar: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTeamDto {
    pub team_name: Option<String>,
    pub team_namespace: Option<String>,
    pub team_avatar: Option<String>,
    pub description: Option<String>,
}

pub struct TeamService {
    pub db: Arc<DatabaseConnection>,
}

impl TeamService {
    pub fn new(db: Arc<DatabaseConnection>) -> TeamService {
        Self { db }
    }

    pub async fn check_namespace_exists(&self, namespace: &str) -> DbResult<bool> {
        let existing_team = Teams::find()
            .select_only()
            .columns([teams::Column::TeamNamespace])
            .filter(teams::Column::TeamNamespace.eq(namespace))
            .one(self.db.as_ref())
            .await?;

        Ok(existing_team.is_some())
    }

    pub async fn find_project_all(&self) -> DbResult<Vec<FormatTeam>> {
        Teams::find()
            .into_partial_model::<FormatTeam>()
            .all(self.db.as_ref())
            .await
    }

    pub async fn find_by_id(&self, id: Uuid) -> DbResult<Option<FormatTeam>> {
        Teams::find_by_id(id)
            .into_partial_model::<FormatTeam>()
            .one(self.db.as_ref())
            .await
    }

    pub async fn create_team(&self, form_data: CreateTeamDto) -> DbResult<teams::ActiveModel> {
        if form_data.team_name.trim().is_empty() {
            return Err(DbErr::Custom("Team name cannot be empty".to_string()));
        }

        if form_data.team_namespace.trim().is_empty() {
            return Err(DbErr::Custom("Team namespace cannot be empty".to_string()));
        }

        let existing_team = Teams::find()
            .filter(teams::Column::TeamNamespace.eq(&form_data.team_namespace))
            .one(self.db.as_ref())
            .await?;

        if existing_team.is_some() {
            return Err(DbErr::Custom(
                "Team with this namespace already exists".to_string(),
            ));
        }

        teams::ActiveModel {
            team_name: Set(form_data.team_name.clone()),
            team_unique_id: Set(generator_unique_id(&form_data.team_namespace)),
            team_namespace: Set(form_data.team_namespace.clone()),
            team_avatar: Set(form_data.team_avatar),
            description: Set(form_data.description),
            ..Default::default()
        }
        .save(self.db.as_ref())
        .await
    }

    pub async fn update_team(&self, id: Uuid, form_data: UpdateTeamDto) -> DbResult<()> {
        let mut team = Teams::find_by_id(id)
            .one(self.db.as_ref())
            .await?
            .ok_or(DbErr::Custom("Cannot find team!".to_string()))?
            .into_active_model();

        if let Some(team_name) = form_data.team_name {
            if team_name.trim().is_empty() {
                return Err(DbErr::Custom("Team name cannot be empty".to_string()));
            }
            team.team_name = Set(team_name);
        }

        if let Some(team_namespace) = form_data.team_namespace {
            if team_namespace.trim().is_empty() {
                return Err(DbErr::Custom("Team namespace cannot be empty".to_string()));
            }

            let existing_team = Teams::find()
                .filter(teams::Column::TeamNamespace.eq(&team_namespace))
                .one(self.db.as_ref())
                .await?;

            if existing_team.is_some() {
                return Err(DbErr::Custom(
                    "Team with this namespace already exists".to_string(),
                ));
            }

            team.team_namespace = Set(team_namespace.clone());
            team.team_unique_id = Set(generator_unique_id(&team_namespace))
        }

        if let Some(description) = form_data.description {
            team.description = Set(Some(description));
        }

        if let Some(team_avatar) = form_data.team_avatar {
            team.team_avatar = Set(Some(team_avatar));
        }

        team.update(self.db.as_ref()).await?;
        Ok(())
    }

    pub async fn delete_team(&self, id: Uuid) -> DbResult<()> {
        Teams::delete_by_id(id).exec(self.db.as_ref()).await?;
        Ok(())
    }
}
