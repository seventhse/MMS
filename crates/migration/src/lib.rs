use async_trait::async_trait;
pub use sea_orm_migration::*;
pub struct Migrator;
mod m20241114_124406_team;
mod m20241114_124420_team_user;
mod m20241114_124434_user;
mod m20241114_125409_activity_log;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241114_124406_team::Migration),
            Box::new(m20241114_124420_team_user::Migration),
            Box::new(m20241114_124434_user::Migration),
            Box::new(m20241114_125409_activity_log::Migration),
        ]
    }
}
