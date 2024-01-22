use std::sync::Arc;

use crate::core::{cipher, dto};
use axum::{
    extract::{rejection::JsonRejection, FromRequest},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct App {
    cipher: Arc<cipher::Cipher>,
    db: Arc<DatabaseConnection>,
    tera: Arc<tera::Tera>,
}

impl App {
    pub fn new(db_conn: DatabaseConnection, tera: tera::Tera) -> Self {
        return App {
            cipher: Arc::new(cipher::Cipher::new()),
            db: Arc::new(db_conn),
            tera: Arc::new(tera),
        };
    }

    pub fn expand(&self) -> (Arc<cipher::Cipher>, Arc<DatabaseConnection>, Arc<tera::Tera>) {
        return (self.cipher.clone(), self.db.clone(), self.tera.clone());
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

pub struct HtmlTemplate(pub Arc<tera::Tera>, pub &'static str, pub tera::Context);

impl IntoResponse for HtmlTemplate {
    fn into_response(self) -> Response {
        match self.0.render(self.1, &self.2) {
            Ok(html) => Html(html).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        }
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
