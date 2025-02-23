use axum::{extract::State, http::StatusCode};
use lib_core::HtmlTemplate;

use crate::app::App;

/// Handler for routes that are not defined
pub async fn fallback_handler(State(app): State<App>) -> HtmlTemplate {
    HtmlTemplate {
        tera: app.tera(),
        template: "404.html",
        status: Some(StatusCode::BAD_REQUEST),
        ctx: None,
    }
}
