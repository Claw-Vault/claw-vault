use std::sync::Arc;

use axum::{extract::Path, Extension};

use crate::{
    app::{App, AppError, HtmlTemplate},
    core::dao,
};

pub async fn index(Extension(app): Extension<Arc<App>>) -> Result<HtmlTemplate, AppError> {
    let (_, _, tera) = app.expand();
    Ok(HtmlTemplate(tera, "index.html", None))
}

pub async fn privacy(Extension(app): Extension<Arc<App>>) -> Result<HtmlTemplate, AppError> {
    let (_, _, tera) = app.expand();
    Ok(HtmlTemplate(tera, "privacy.html", None))
}

pub async fn vault(
    Extension(app): Extension<Arc<App>>,
    Path(id): Path<String>,
) -> Result<HtmlTemplate, HtmlTemplate> {
    let (_, db, tera) = app.expand();

    let claw = match dao::get_claw_by_id(id, &db).await {
        Ok(claw) => claw,
        Err(_) => return Err(HtmlTemplate(tera, "404.html", None)),
    };

    let mut ctx = tera::Context::new();
    ctx.insert("id", &claw.id);

    Ok(HtmlTemplate(tera, "vault.html", Some(ctx)))
}
