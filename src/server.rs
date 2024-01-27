use std::sync::Arc;

use axum::{Extension, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    core::{app::App, interceptor},
    handlers, routes,
};

/// Serves axum backend server
pub async fn serve(app: Arc<App>, notify: Arc<tokio::sync::Notify>) {
    // build our application with a route
    // bind routes
    let router = get_router(app).await;

    // run our app with hyper, listening globally on port 3000
    let addr = get_addr().await;
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal(notify))
        .await
        .unwrap();
}

pub async fn get_router(app: Arc<App>) -> Router {
    // Prepare swagger
    let swagger =
        SwaggerUi::new("/swagger").url("/api-doc/openapi.json", routes::ApiDoc::openapi());

    routes::bind_routes(Router::new())
        .merge(swagger)
        .fallback(handlers::fallback_handler)
        .layer(Extension(app))
        .layer(axum::middleware::from_fn(interceptor::intercept))
}

/// Returns socket address for binding
///
/// Concatenate `PORT` from env to `0.0.0.0`
///
/// Defaults to `3000` if env not set
async fn get_addr() -> String {
    let port = std::env::var("PORT").unwrap_or_else(|_| {
        tracing::info!("PORT was not provided, default to 3000");
        String::from("3000")
    });
    format!("0.0.0.0:{}", port)
}

/// Function that listens to signals and notify waiters
pub async fn shutdown_signal(notify: Arc<tokio::sync::Notify>) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {notify.notify_waiters()},
        _ = terminate => {notify.notify_waiters()},
    }
}
