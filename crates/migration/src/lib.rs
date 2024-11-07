use async_trait::async_trait;
pub use sea_orm_migration::*;
pub struct Migrator;
mod m20241028_173251_auth_create_table;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20241028_173251_auth_create_table::Migration)]
    }
}
