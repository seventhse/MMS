use std::rc::Rc;

use crate::utils::bytes_to_payload;
use actix_web::body::{BoxBody, MessageBody};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{web::Bytes, Error, HttpResponse};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use futures_util::FutureExt;
use serde_json::json;

pub struct ErrorInterceptor;

impl<S, B> Transform<S, ServiceRequest> for ErrorInterceptor
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = ErrorInterceptorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ErrorInterceptorMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct ErrorInterceptorMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ErrorInterceptorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        async move {
            let body = req.extract::<Bytes>().await.unwrap();
            log::info!(
                "\nApi request url is: \n{};\n body is: \n{:#?}",
                req.request().uri(),
                &body
            );
            req.set_payload(bytes_to_payload(body.clone()));

            let res = svc.call(req).await?;

            match res.response().error() {
                None => {
                    let status = res.status();
                    if status.is_success() {
                        return Ok(res.map_into_boxed_body());
                    }

                    log::error!(
                        "\nRequest error uri is: \n{}; \n body is: \n{:#?}",
                        res.request().uri(),
                        body
                    );
                    let error_body = json!({
                        "code": status.as_u16() as i32,
                        "message": res.response().error().unwrap_or(
                          &actix_web::error::ErrorBadRequest(format!("Not found request route {}!",res.request().uri()))
                        ).to_string()
                    });

                    let new_response = HttpResponse::Ok().json(error_body);

                    Ok(ServiceResponse::new(res.request().clone(),new_response))
                }
                Some(error) => {
                    let new_request = res.request().clone();
                    dbg!(&error);
                    let error_repsonse = error.as_response_error();
                    log::info!("\nError response: {:#?}", error_repsonse);
                    log::error!(
                        "\nApi `{}` request error: {:#?}",
                        new_request.uri(),
                        error.to_string()
                    );
                    let error_body = json!({
                        "code": error_repsonse.status_code().as_u16() as i32,
                        "message": error.to_string()
                    });

                    let new_response = HttpResponse::Ok().json(error_body);

                    Ok(ServiceResponse::new(new_request, new_response))
                }
            }
        }
        .boxed_local()
    }
}
