pub mod encrypt;
pub mod jwt;

use sea_orm::DbErr;
pub type DbResult<T> = Result<T, DbErr>;
