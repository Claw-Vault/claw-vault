use std::sync::Arc;

use axum::{http::StatusCode, Extension};

use crate::core::app::{App, HtmlTemplate};

pub mod api;
pub mod web;

/// Handler for routes that are not defined
pub async fn fallback_handler(Extension(app): Extension<Arc<App>>) -> HtmlTemplate {
    let (_, _, tera) = app.expand();
    HtmlTemplate(tera, "404.html", Some(StatusCode::BAD_REQUEST), None)
}
