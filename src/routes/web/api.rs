use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use lib_core::HtmlTemplate;

use crate::app::App;

pub async fn index(State(app): State<App>) -> HtmlTemplate {
    HtmlTemplate { tera: app.tera(), template: "index.html", status: None, ctx: None }
}

pub async fn privacy(State(app): State<App>) -> HtmlTemplate {
    HtmlTemplate { tera: app.tera(), template: "privacy.html", status: None, ctx: None }
}

pub async fn vault(
    State(app): State<App>,
    Query(id): Query<String>,
) -> Result<HtmlTemplate, HtmlTemplate> {
    let err_template = HtmlTemplate {
        tera: app.tera(),
        template: "404.html",
        status: Some(StatusCode::NOT_FOUND),
        ctx: None,
    };

    let claw = match app.service().ds().get_claw(&id).await {
        Ok(claw) => claw.ok_or_else(|| err_template),
        Err(_) => return Err(err_template),
    }?;

    let mut ctx = tera::Context::new();
    ctx.insert("id", &claw.id);

    Ok(HtmlTemplate { tera: app.tera(), template: "vault.html", status: None, ctx: Some(ctx) })
}
