use actix_web::http::StatusCode;
use migration::DbErr;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct Empty;

#[derive(Debug, Clone, Copy)]
pub enum ApiStatusCode {
    Ok = 200,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    InternalServerError = 500,
}

impl ApiStatusCode {
    pub fn else_msg(&self, msg: Option<&str>) -> String {
        match msg {
            Some(val) => val.to_string(),
            None => self.to_string(),
        }
    }
}

impl From<ApiStatusCode> for StatusCode {
    fn from(code: ApiStatusCode) -> StatusCode {
        StatusCode::from_u16(code as u16).unwrap()
    }
}

impl From<StatusCode> for ApiStatusCode {
    fn from(status: StatusCode) -> ApiStatusCode {
        match status.as_u16() {
            200 => ApiStatusCode::Ok,
            400 => ApiStatusCode::BadRequest,
            401 => ApiStatusCode::Unauthorized,
            403 => ApiStatusCode::Forbidden,
            500 => ApiStatusCode::InternalServerError,
            _ => ApiStatusCode::InternalServerError,
        }
    }
}

impl Serialize for ApiStatusCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(*self as i32)
    }
}

impl fmt::Display for ApiStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            ApiStatusCode::Ok => "Successful",
            ApiStatusCode::BadRequest => "Bad Request",
            ApiStatusCode::Unauthorized => "Unauthorized",
            ApiStatusCode::Forbidden => "Forbidden",
            ApiStatusCode::InternalServerError => "Internal Server Error",
        };
        write!(f, "{}", message)
    }
}

impl Default for ApiStatusCode {
    fn default() -> ApiStatusCode {
        ApiStatusCode::Ok
    }
}

#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    code: ApiStatusCode,
    message: String,
    data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn new(code: ApiStatusCode, message: Option<&str>, data: Option<T>) -> Self {
        ApiResponse {
            code,
            message: code.else_msg(message),
            data,
        }
    }

    pub fn success(message: Option<&str>, data: Option<T>) -> Self {
        Self::new(ApiStatusCode::Ok, message, data)
    }

    pub fn msg(code: ApiStatusCode, message: Option<&str>) -> Self {
        ApiResponse {
            code,
            message: code.else_msg(message),
            data: None,
        }
    }

    pub fn ok(message: Option<&str>) -> Self {
        Self::msg(ApiStatusCode::Ok, message)
    }

    pub fn bad_request(message: Option<&str>) -> Self {
        Self::msg(ApiStatusCode::BadRequest, message)
    }

    pub fn unauthorized(message: Option<&str>) -> Self {
        Self::msg(ApiStatusCode::Unauthorized, message)
    }

    pub fn forbidden(message: Option<&str>) -> Self {
        Self::msg(ApiStatusCode::Forbidden, message)
    }

    pub fn error(message: Option<&str>) -> Self {
        Self::msg(ApiStatusCode::InternalServerError, message)
    }
}

pub fn handle_response_by_service<T>(res: Result<T, DbErr>) -> ApiResponse<T>
where
    T: serde::Serialize,
{
    match res {
        Ok(res) => ApiResponse::<T>::success(None, Some(res)),
        Err(e) => {
            log::error!("Service error: {:#?}", e);
            match e {
                DbErr::Custom(str) => ApiResponse::<T>::bad_request(Some(&str)),
                _ => ApiResponse::<T>::error(None),
            }
        }
    }
}
