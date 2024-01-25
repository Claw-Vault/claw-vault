use std::sync::Arc;

use crate::core::{cipher, dao, dto};
use axum::{
    extract::{rejection::JsonRejection, FromRequest},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use sea_orm::DatabaseConnection;

/// Struct to hold instances of common objects
/// in app.
///
/// This struct is passed on to every API path.
///
/// All instances are hold in [`Arc`]
/// to minimize cloning cost for every request
/// being handled.
#[derive(Clone)]
pub struct App {
    cipher: Arc<cipher::Cipher>,
    db: Arc<DatabaseConnection>,
    tera: Arc<tera::Tera>,
}

impl App {
    /// Initializes app with required objects
    ///
    /// Returns [`App`]
    pub async fn init() -> Arc<App> {
        let db = dao::connect_db().await;
        let tera = App::setup_tera().await;
        let _ = std::env::var("ASSETS_DIR").expect("ASSETS_DIR not set");
        Arc::new(App::new(db, tera))
    }

    /// Setup tera for html templates
    ///
    /// Expects `TEMPLATE_DIR` from env else panics
    ///
    /// Returns [`tera::Tera`]
    async fn setup_tera() -> tera::Tera {
        let template_dir = std::env::var("TEMPLATE_DIR").expect("TEMPLATE_DIR not set");
        let template_dir = format!("{}/**/*.html", template_dir);
        let mut tera = tera::Tera::new(&template_dir).expect("Failed to initialize Tera");
        crate::core::tera::add_404(&mut tera);
        crate::core::tera::add_index(&mut tera);
        crate::core::tera::add_privacy(&mut tera);
        crate::core::tera::add_vault(&mut tera);
        tera
    }

    /// Private constructor for [`App`]
    ///
    /// Takes in [`DatabaseConnection`] and [`tera::Tera`]
    fn new(db_conn: DatabaseConnection, tera: tera::Tera) -> Self {
        App {
            cipher: Arc::new(cipher::Cipher::new()),
            db: Arc::new(db_conn),
            tera: Arc::new(tera),
        }
    }

    /// This functions returns instances of the objects
    /// held by the struct.
    pub fn expand(
        &self,
    ) -> (
        Arc<cipher::Cipher>,
        Arc<DatabaseConnection>,
        Arc<tera::Tera>,
    ) {
        (self.cipher.clone(), self.db.clone(), self.tera.clone())
    }

    /// Terminates the object that are required for graceful shutdown
    ///
    /// For now it closes [`DatabaseConnection`]
    pub async fn terminate(&self) {
        self.db
            .as_ref()
            .to_owned()
            .close()
            .await
            .expect("Failed to disconnect DB");

        tracing::info!("Terminated App");
    }
}

/// Custom Json wrapper handling json pyload
/// parsing errors.
///
/// See more: [`axum::Json`]
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

/// Struct for rendering html templates
///
/// It takes in:
/// - [`tera::Tera`]    : Template renderer
/// - [`str`]           : Html file name
/// - [`tera::Context`] : Context to bind variables to template
pub struct HtmlTemplate(
    pub Arc<tera::Tera>,
    pub &'static str,
    pub Option<StatusCode>,
    pub Option<tera::Context>,
);

impl IntoResponse for HtmlTemplate {
    /// Function to render the provided template
    fn into_response(self) -> Response {
        let ctx = if let Some(ctx) = self.3 {
            ctx
        } else {
            tera::Context::new()
        };

        let status_code = if let Some(status) = self.2 {
            status
        } else {
            StatusCode::OK
        };

        match self.0.render(self.1, &ctx) {
            Ok(html) => (status_code, Html(html)).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        }
    }
}

/// Enum for customize and handling errors
pub enum AppError {
    BadRequest(String),
    InvalidBody(JsonRejection),
    ServerError(String),
    DbError(String),
}

impl IntoResponse for AppError {
    /// Function to map errors into appropriate responses
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
