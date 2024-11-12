use std::sync::Arc;

use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use service::Service;

use crate::settings::AppSettings;

pub async fn bearer_validator(
    req: ServiceRequest,
    credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    log::info!("\n ================== Auth validator start ======================= \n");

    match credentials {
        Some(credentials) => {
            let token = credentials.token();
            log::info!(
                "\nBeaere validator uri: {:#?} \n token: {}",
                &req.uri(),
                token
            );

            let service = req
                .app_data::<actix_web::web::Data<Arc<Service>>>()
                .unwrap();
            let bool = service.auth_service.verify_token(token).await;

            if bool {
                Ok(req)
            } else {
                Err((
                    actix_web::error::ErrorUnauthorized("Invalid Bearer token"),
                    req,
                ))
            }
        }
        None => {
            if let Some(settings) = req.app_data::<actix_web::web::Data<AppSettings>>() {
                let uri = req.path().to_string();
                log::info!("Check uri: {} is not validator authorized", &uri);
                let route_white = settings
                    .application
                    .clone()
                    .route_whites
                    .unwrap_or(Vec::new());
                if route_white.contains(&uri) {
                    return Ok(req);
                }
            }
            Err((
                actix_web::error::ErrorUnauthorized(
                    "Authorization header not found or missing Bearer token",
                ),
                req,
            ))
        }
    }
}
