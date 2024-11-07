use std::{env::set_var, fmt::Display};

use actix_settings::{BasicSettings, Mode};
use derive_more::derive::Debug;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct PGDb {
    host: String,
    port: i32,
    username: String,
    password: String,
    db: String,
}

impl Display for PGDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //postgres://season:season@localhost:5432/marketing_monitor
        write!(
            f,
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Cors {
    pub allowed_origin: Option<String>,
    pub allowed_methods: Option<Vec<String>>,
    pub allowed_headers: Option<Vec<String>>,
    pub allowed_header: Option<Vec<String>>,
    pub enable_credentials: Option<bool>,
    pub max_age: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Token {
    pub secret: String,
    pub exipre: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct ApplicationSettings {
    #[serde(rename = "pg-database")]
    pub pg_database: PGDb,
    pub token: Option<Token>,
    pub route_whites: Option<Vec<String>>,
    pub cors: Option<Cors>,
}

pub type AppSettings = BasicSettings<ApplicationSettings>;

pub fn init_settings() -> std::io::Result<AppSettings> {
    let mut settings = AppSettings::parse_toml("./Server.toml")
        .expect("Faild to parse `Setting` from config.toml");

    AppSettings::override_field_with_env_var(&mut settings.actix.hosts, "APPLICATION__HOSTS")?;

    let token = settings.application.token.clone().unwrap_or_else(|| Token {
        secret: "some-secret".to_string(),
        exipre: 1000 * 60 * 10,
    });

    unsafe {
        set_var("JWT_SECRET", token.secret);
        set_var("JWT_EXIPRE", token.exipre.to_string());
        set_var(
            "RUST_LOG",
            match &settings.actix.mode {
                Mode::Development => "info",
                Mode::Production => "error",
            },
        );

        set_var("RUST_BACKTRACE", "1");
    }

    Ok(settings)
}
