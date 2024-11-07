#[allow(dead_code)]
mod common;

pub mod guards;
pub mod handlers;
pub mod middlewares;
pub mod routes;
pub mod settings;
pub mod utils;

use actix_settings::ApplySettings;
use actix_web::middleware::{Compress, Condition, Logger};
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use guards::auth::bearer_validator;
use middlewares::{cors, error_interceptor};
use service::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use service::Service;
use settings::{init_settings, AppSettings};
use std::sync::Arc;

async fn init_pg(settings: &AppSettings) -> std::io::Result<DatabaseConnection> {
    let pg_config = &settings.application.pg_database;
    log::info!("Connection database url: {}", &pg_config.to_string());

    let mut database_opt = ConnectOptions::new(&pg_config.to_string());

    database_opt
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let conn = Database::connect(database_opt).await.map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "Connection database error, Check your database config.",
        )
    })?;

    Ok(conn)
}

pub fn init_logger(settings: &AppSettings) {
    if !settings.actix.enable_log {
        return;
    }

    env_logger::init();
}

fn init_config() -> std::io::Result<AppSettings> {
    let settings = init_settings()?;
    init_logger(&settings);

    // log::info!("settings : {:#?}", &settings);
    log::info!(
        "starting HTTP server at: http://{}:{}",
        &settings.actix.hosts[0].host,
        &settings.actix.hosts[0].port
    );
    Ok(settings)
}

pub async fn app_run() -> std::io::Result<()> {
    let settings = init_config()?;
    let conn = Arc::new(init_pg(&settings).await?);
    let service = Arc::new(Service::new(conn.clone()));

    log::info!("============ Server starting =================");
    HttpServer::new({
        let settings = settings.clone();
        move || {
            let auth = HttpAuthentication::with_fn(bearer_validator);

            App::new()
                .wrap(Condition::new(
                    settings.actix.enable_compression,
                    Compress::default(),
                ))
                .wrap(Logger::default())
                .wrap(cors::init_cors(&settings))
                .wrap(auth)
                .wrap(error_interceptor::ErrorInterceptor)
                .app_data(web::Data::new(settings.clone()))
                .app_data(web::Data::new(conn.clone()))
                .app_data(web::Data::new(service.clone()))
                .configure(routes::config)
        }
    })
    .try_apply_settings(&settings)?
    .run()
    .await
}
