pub mod _entities;
pub mod common;
pub mod enum_serialize;
pub mod utils;
use std::sync::Arc;

use common::{auth_service::AuthService, user_service::UserService};
pub use sea_orm;
use sea_orm::DatabaseConnection;

pub struct Service {
    pub db: Arc<DatabaseConnection>,
    pub auth_service: Arc<AuthService>,
    pub user_service: Arc<UserService>,
}

impl Service {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        let user_service = Arc::new(UserService::new(db.clone()));
        let auth_service = Arc::new(AuthService::new(db.clone(), user_service.clone()));
        Self {
            db,
            auth_service,
            user_service,
        }
    }
}
