use crate::settings::AppSettings;
use actix_cors::Cors;

pub fn init_cors(settings: &AppSettings) -> Cors {
    let mut cors = Cors::default();

    let application = &settings.application;

    if let Some(cors_settings) = application.cors.clone() {
        // Set allowed origin
        if let Some(origin) = &cors_settings.allowed_origin {
            cors = cors.allowed_origin(origin);
        }

        // Set allowed methods
        if let Some(methods) = &cors_settings.allowed_methods {
            cors = cors.allowed_methods(methods.iter().map(String::as_str).collect::<Vec<&str>>());
        }

        // Set allowed headers from allowed_headers
        if let Some(headers) = &cors_settings.allowed_headers {
            cors = cors.allowed_headers(headers.iter().map(String::as_str).collect::<Vec<&str>>());
        }

        // Set allowed header from allowed_header
        if let Some(headers) = &cors_settings.allowed_header {
            for header in headers {
                cors = cors.allowed_header(header);
            }
        }

        // Set credentials support
        if let Some(enable_credentials) = cors_settings.enable_credentials {
            if enable_credentials {
                cors = cors.supports_credentials();
            }
        }

        // Set max age
        if let Some(max_age) = cors_settings.max_age {
            cors = cors.max_age(max_age as usize);
        }
    }
    cors
}
