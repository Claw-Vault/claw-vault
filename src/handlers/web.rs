use std::sync::Arc;

use axum::{extract::Path, Extension};

use crate::app::{App, AppError, HtmlTemplate};

pub async fn index(Extension(app): Extension<Arc<App>>) -> Result<HtmlTemplate, AppError> {
    let (_, _, tera) = app.expand();

    let mut ctx = tera::Context::new();
    ctx.insert("name", "Shank");

    Ok(HtmlTemplate(tera, "index.html", ctx))
}

pub async fn store(Extension(app): Extension<Arc<App>>) -> Result<HtmlTemplate, AppError> {
    let (_, _, tera) = app.expand();

    let ctx = tera::Context::new();
    Ok(HtmlTemplate(tera, "store.html", ctx))
}

pub async fn vault(
    Extension(app): Extension<Arc<App>>,
    Path(id): Path<String>,
) -> Result<HtmlTemplate, AppError> {
    let (_, _, tera) = app.expand();

    let mut ctx = tera::Context::new();
    ctx.insert("id", &id);

    Ok(HtmlTemplate(tera, "vault.html", ctx))
}
