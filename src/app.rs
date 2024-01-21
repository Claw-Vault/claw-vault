use std::sync::Arc;

use crate::core::{cipher, dto};
use axum::{
    extract::{rejection::JsonRejection, FromRequest},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct App {
    cipher: Arc<cipher::Cipher>,
    db: Arc<DatabaseConnection>,
}

impl App {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        return App {
            cipher: Arc::new(cipher::Cipher::new()),
            db: Arc::new(db_conn),
        };
    }

    pub fn expand(self) -> (Arc<cipher::Cipher>, Arc<DatabaseConnection>) {
        return (self.cipher, self.db);
    }
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct Json<T>(pub T);

impl<T> IntoResponse for Json<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

pub enum AppError {
    BadRequest(String),
    InvalidBody(JsonRejection),
    ServerError(String),
    DbError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::InvalidBody(_) => (StatusCode::BAD_REQUEST, String::from("Invalid payload")),
            AppError::BadRequest(err) => (StatusCode::BAD_REQUEST, err),
            AppError::ServerError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
            AppError::DbError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("DbErr: {}", err))
            }
        };

        (status, Json(dto::ErrorMessage::new(status, message))).into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        Self::InvalidBody(rejection)
    }
}
