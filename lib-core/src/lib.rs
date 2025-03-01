use std::{error::Error, fmt::Display};

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use interceptor::ReqId;
use serde::Serialize;
use utoipa::ToSchema;
use validator::Validate;

pub mod config;
pub mod enums;
pub mod interceptor;
pub mod vault;

#[derive(Serialize, ToSchema)]
pub struct EmptyResponse {
    status: u16,
    message: String,
}
impl EmptyResponse {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        EmptyResponse { status: status.as_u16(), message: message.into() }
    }
}

/// Custom Json wrapper handling json payload
/// parsing errors.
///
/// See more: [`axum::Json`] [`validator`]
pub struct Json<T>(pub T);

impl<T> IntoResponse for Json<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, axum::Json(self.0)).into_response()
    }
}

/// Custom Json wrapper handling json payload
///
/// Struct being extract must have [`serde::Deserialize`] and [`validator::Validate`] to validate the payload
#[async_trait]
impl<S, T> FromRequest<S> for Json<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    T: Validate,
    S: Send + Sync,
{
    type Rejection = ApiResponse<()>;

    async fn from_request(req: Request, state: &S) -> Result<Self, ApiResponse<()>> {
        let req_id: ReqId = {
            let req = &req;
            let id: &ReqId = req.extensions().get().unwrap();
            id.clone()
        };

        let axum::Json(payload) = axum::Json::<T>::from_request(req, state).await.map_err(|e| {
            let err_msg = e.body_text();
            ApiResponse::Err(AppError::err(ErrType::InvalidBody, e, err_msg), req_id.clone())
        })?;

        payload.validate().map_err(|e| {
            let err_msg = format!("Bad Payload: {}", e);
            ApiResponse::Err(AppError::err(ErrType::ValidationErr, e, err_msg), req_id.clone())
        })?;

        Ok(Json(payload))
    }
}

#[derive(Debug)]
pub enum ErrType {
    Unauthorized,
    BadRequest,
    NotFound,
    ServerError,
    DbError,
    VaultError,
    ValidationErr,
    InvalidBody,
    TooManyRequests,
}
impl Display for ErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ErrType::Unauthorized => "Unauthorized",
                ErrType::BadRequest => "BadRequest",
                ErrType::NotFound => "NotFound",
                ErrType::ServerError => "ServerError",
                ErrType::DbError => "DbError",
                ErrType::VaultError => "VaultError",
                ErrType::ValidationErr => "ValidationErr",
                ErrType::InvalidBody => "InvalidBody",
                ErrType::TooManyRequests => "TooManyRequests",
            }
        )
    }
}

#[derive(Debug)]
pub struct AppError {
    pub _type: ErrType,
    message: String,
    at: String,
    err_msg: String,
}

impl AppError {
    pub fn new(_type: ErrType, message: impl Into<String>) -> Self {
        AppError::init(_type, None, message)
    }

    pub fn err(_type: ErrType, err: impl Into<Box<dyn Error>>, message: impl Into<String>) -> Self {
        AppError::init(_type, Some(err.into()), message)
    }

    fn init(_type: ErrType, err: Option<Box<dyn Error>>, message: impl Into<String>) -> Self {
        let at = AppError::caller();
        AppError {
            _type,
            message: message.into(),
            at,
            err_msg: err.map(|e| e.to_string()).unwrap_or("".into()),
        }
    }

    fn caller() -> String {
        let mut file_addr = String::from("");

        let bt = backtrace::Backtrace::new_unresolved();
        let frame = match bt.frames().get(3) {
            Some(frame) => frame,
            _ => return "".into(),
        };
        backtrace::resolve(frame.ip(), |symbol| {
            let file_path = match symbol.filename() {
                Some(path) => path,
                _ => return,
            };

            let file_name = file_path
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| "".into());
            let lineno = symbol.lineno().unwrap_or(0);
            let colno = symbol.colno().unwrap_or(0);
            file_addr = format!("{}:{}:{}", file_name, lineno, colno);
        });
        file_addr
    }

    pub fn get_messages(self) -> (String, String, String) {
        (self.message, self.err_msg, self.at)
    }
}

// impl From<(surrealdb::Error, &str)> for AppError {
//     fn from((err, msg): (surrealdb::Error, &str)) -> Self {
//         AppError::init(ErrType::DbError, Some(Box::new(err)), msg)
//     }
// }

pub type AppResult<T> = Result<T, AppError>;

pub enum ApiResponse<T> {
    Ok(T),
    Err(AppError, ReqId),
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn map_res(res: AppResult<T>, req_id: ReqId) -> Self {
        match res {
            Ok(ok) => ApiResponse::Ok(ok),
            Err(err) => ApiResponse::Err(err, req_id),
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
    axum::Json<T>: IntoResponse,
{
    /// Function to map errors into appropriate responses
    fn into_response(self) -> Response {
        match self {
            ApiResponse::Ok(json) => Json(json).into_response(),
            ApiResponse::Err(err, req_id) => {
                let id: &str = req_id.as_ref();
                let _type = err._type;
                let err_msg = err.err_msg;
                let message = format!("[{}]: {}", _type, err.message);
                let at = err.at;

                let status = match _type {
                    ErrType::InvalidBody => StatusCode::BAD_REQUEST,
                    ErrType::Unauthorized => StatusCode::UNAUTHORIZED,
                    ErrType::BadRequest => StatusCode::BAD_REQUEST,
                    ErrType::NotFound => StatusCode::NOT_FOUND,
                    ErrType::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
                    ErrType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
                    ErrType::VaultError => StatusCode::INTERNAL_SERVER_ERROR,
                    ErrType::ValidationErr => StatusCode::BAD_REQUEST,
                    ErrType::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
                };

                match status {
                    StatusCode::INTERNAL_SERVER_ERROR | StatusCode::FAILED_DEPENDENCY => {
                        tracing::error!(req_id = id, message = message, at = at, err = err_msg)
                    }
                    _ => tracing::warn!(req_id = id, message = message, at = at, err = err_msg),
                };

                (status, Json(EmptyResponse { status: status.as_u16(), message })).into_response()
            }
        }
    }
}

impl From<JsonRejection> for ApiResponse<()> {
    fn from(rejection: JsonRejection) -> Self {
        ApiResponse::Err(
            AppError::err(ErrType::InvalidBody, rejection, "Invalid payload"),
            "".into(),
        )
    }
}
