use axum::routing::Router;
use utoipa::OpenApi;

use crate::app::App;

pub(crate) mod fallback;
mod health;
mod vault;
mod web;

/// Function to bind routes from:
/// - [`health`]
/// - [`vault`]
/// - [`web`]
pub fn bind_routes(router: Router<App>) -> Router<App> {
    // root level routes
    let r = health::bind_routes();
    let r = web::bind_routes(r);

    // api level routes
    let vault_routes = vault::bind_routes(Router::new());

    router.merge(r).nest("/api/v1", vault_routes)
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "ClawVault API Documentation",
        description = r#"API documentation for ClawVault Backend.

## Overview

* https://claw-vault.up.railway.app

#### Contact
- [API Support](mailto:shashank.verma2002@gmail.com)"#,
        contact(name = "API Support", email = "shashank.verma2002@gmail.com"),
        license(
            name = "MIT",
            url = "https://raw.githubusercontent.com/Claw-Vault/claw-vault/refs/heads/main/LICENSE"
        ),
    ),
    paths(health::health, vault::api::encrypt, vault::api::decrypt, vault::api::has_claw),
    components(schemas(
        lib_core::enums::ValidDuration,
        lib_core::EmptyResponse,
        lib_domain::dto::vault::req::EncryptRequest,
        lib_domain::dto::vault::req::DecryptRequest,
        lib_domain::dto::vault::res::EncryptResponse,
        lib_domain::dto::vault::res::DecryptResponse,
    )),
    servers()
)]
pub struct ApiDoc;
