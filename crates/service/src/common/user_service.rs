use crate::_entities::prelude::*;
use crate::_entities::sea_orm_active_enums::Status;
use crate::_entities::users::{self};
use crate::utils::encrypt::{generator_unique_id, PassVerify};
use crate::utils::DbResult;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::Uuid;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(FromQueryResult, DerivePartialModel, Serialize)]
#[sea_orm(entity = "Users")]
pub struct PartialUser {
    pub user_id: Uuid,
    pub unique_id: String,
    pub email: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
    pub default_team_id: Option<Uuid>,
    pub status: Status,
    #[sea_orm(from_expr = "Expr::cust(\"to_char(users.created_at, 'YYYY-MM-DD HH:mm:ss')\")")]
    pub created_at: Option<String>,
    #[sea_orm(from_expr = "Expr::cust(\"to_char(users.updated_at, 'YYYY-MM-DD HH:mm:ss')\")")]
    pub updated_at: Option<String>,
}

pub type ModelResult = DbResult<users::Model>;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserDto {
    pub email: String,
    pub password: String,
    pub username: String,
    pub avatar: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserDto {
    pub email: Option<String>,
    pub username: Option<String>,
    pub avatar: Option<String>,
    pub display_name: Option<String>,
    pub default_team_id: Option<Uuid>,
}

pub struct UserService {
    pub db: Arc<DatabaseConnection>,
}

impl UserService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn find_user_all(&self) -> DbResult<Vec<PartialUser>> {
        Users::find()
            .into_partial_model::<PartialUser>()
            .all(self.db.as_ref())
            .await
    }

    pub async fn find_user_by_id(&self, id: Uuid) -> DbResult<Option<PartialUser>> {
        Users::find_by_id(id)
            .into_partial_model::<PartialUser>()
            .one(self.db.as_ref())
            .await
    }

    pub async fn find_user_by_email(&self, email: &str) -> DbResult<Option<users::Model>> {
        Users::find()
            .filter(users::Column::Email.eq(email))
            .one(self.db.as_ref())
            .await
    }

    pub async fn check_email_exist(&self, email: &str) -> DbResult<bool> {
        Ok(self.find_user_by_email(email).await?.is_some())
    }

    pub async fn check_username_exist(&self, username: &str) -> DbResult<bool> {
        Ok(Users::find()
            .filter(users::Column::Username.eq(username))
            .one(self.db.as_ref())
            .await?
            .is_some())
    }

    pub async fn find_users_in_page(
        &self,
        page: u64,
        users_per_page: u64,
    ) -> DbResult<(Vec<PartialUser>, u64)> {
        let page = if page == 0 { 1 } else { page };
        let users_per_page = if users_per_page == 0 {
            10
        } else {
            users_per_page
        };

        let query = Users::find()
            .order_by_asc(users::Column::CreatedAt)
            .into_partial_model::<PartialUser>();

        let paginator = query.paginate(self.db.as_ref(), users_per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn create_user(&self, form_data: CreateUserDto) -> DbResult<users::ActiveModel> {
        if form_data.email.trim().is_empty() {
            return Err(DbErr::Custom("Email cannot be empty".to_string()));
        }

        if form_data.username.trim().is_empty() {
            return Err(DbErr::Custom("Username cannot be empty".to_string()));
        }

        if form_data.password.trim().is_empty() {
            return Err(DbErr::Custom("Password cannot be empty".to_string()));
        }

        if self.check_email_exist(&form_data.email).await? {
            return Err(DbErr::Custom(
                "User with this email already exists".to_string(),
            ));
        }

        if self.check_username_exist(&form_data.username).await? {
            return Err(DbErr::Custom(
                "User with this username already exists".to_string(),
            ));
        }

        users::ActiveModel {
            email: Set(form_data.email.clone()),
            username: Set(Some(form_data.username)),
            display_name: Set(form_data.display_name),
            avatar: Set(form_data.avatar),
            password: Set(PassVerify::encrypt_password(&form_data.password).unwrap()),
            unique_id: Set(generator_unique_id(&form_data.email)),
            ..Default::default()
        }
        .save(self.db.as_ref())
        .await
    }

    pub async fn update_password_by_email(&self, email: &str, password: &str) -> ModelResult {
        if password.trim().is_empty() {
            return Err(DbErr::Custom("New password cannot be empty".to_string()));
        }

        let mut user = self
            .find_user_by_email(email)
            .await?
            .ok_or(DbErr::Custom("Cannot find user by the email!".to_string()))?
            .into_active_model();
        user.password = Set(PassVerify::encrypt_password(password).unwrap());
        user.update(self.db.as_ref()).await
    }

    pub async fn update_user_by_id(&self, id: Uuid, form_data: UpdateUserDto) -> ModelResult {
        let mut user: users::ActiveModel = Users::find_by_id(id)
            .one(self.db.as_ref())
            .await?
            .ok_or(DbErr::Custom("Cannot find user".to_owned()))?
            .into_active_model();

        if let Some(email) = form_data.email {
            if email.trim().is_empty() {
                return Err(DbErr::Custom("Email cannot be empty".to_string()));
            }
            if let Some(user_model) = self.find_user_by_email(&email).await? {
                if user_model.user_id != id {
                    return Err(DbErr::Custom("Email already exists".to_string()));
                }
            }
            user.email = Set(email.clone());
            user.unique_id = Set(generator_unique_id(&email));
        }

        if let Some(username) = form_data.username {
            if username.trim().is_empty() {
                return Err(DbErr::Custom("Username cannot be empty".to_string()));
            }
            if let Some(user_model) = Users::find()
                .filter(users::Column::Username.eq(&username))
                .one(self.db.as_ref())
                .await?
            {
                if user_model.user_id != id {
                    return Err(DbErr::Custom("Username already exists".to_string()));
                }
            }
            user.username = Set(Some(username));
        }

        user.display_name = Set(form_data.display_name);
        user.avatar = Set(form_data.avatar);
        user.default_team_id = Set(form_data.default_team_id);

        user.update(self.db.as_ref()).await
    }

    pub async fn verify_password_by_email(
        &self,
        email: &str,
        password: &str,
    ) -> DbResult<users::Model> {
        if password.trim().is_empty() {
            return Err(DbErr::Custom("Password cannot be empty".to_string()));
        }

        let user = self
            .find_user_by_email(email)
            .await?
            .ok_or(DbErr::Custom("Cannot find user by the email!".to_string()))?;

        if PassVerify::verify_password(password, &user.password).is_err() {
            return Err(DbErr::Custom("Invalid password".to_string()));
        }

        Ok(user)
    }

    pub async fn delete_user(&self, id: Uuid) -> DbResult<DeleteResult> {
        let user = Users::find_by_id(id)
            .one(self.db.as_ref())
            .await?
            .ok_or(DbErr::Custom("Cannot find user".to_owned()))?
            .into_active_model();

        user.delete(self.db.as_ref()).await
    }
}
