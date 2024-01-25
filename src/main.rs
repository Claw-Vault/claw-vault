use std::sync::Arc;
use std::time::Duration;

use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::Response;
use axum::{Extension, Router};
use tower_http::trace::TraceLayer;
use tracing::{Level, Span};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod app;
mod core;
mod handlers;
mod middleware;
mod routes;
mod tests;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .with(tracing_subscriber::fmt::layer().with_thread_ids(true))
        .init();

    // load env
    dotenv::dotenv().ok();

    // initialize app
    let app = app::App::init().await;

    // initialize notifier - used for signaling jobs
    let notify = Arc::new(tokio::sync::Notify::new());

    // schedule cleaner
    core::cleaner::schedule_cleaner(app.clone(), notify.clone()).await;

    // serve app
    serve(app, notify).await;
}

/// Serves axum backend server
async fn serve(app: Arc<app::App>, notify: Arc<tokio::sync::Notify>) {
    // Prepare swagger
    let swagger =
        SwaggerUi::new("/swagger").url("/api-doc/openapi.json", routes::ApiDoc::openapi());

    // build our application with a route
    // bind routes
    let router = routes::bind_routes(Router::new())
        .merge(swagger)
        .fallback(handlers::fallback_handler)
        .layer(Extension(app))
        .layer(
            TraceLayer::new_for_http().on_request(|req: &Request<_>, _: &Span| {
                let req_id = get_header(middleware::X_REQUEST_ID, req.headers());
                tracing::info!(req_id = req_id, method = ?req.method(), uri=?req.uri());
            }),
        )
        .layer(axum::middleware::from_fn(middleware::request_id))
        .layer(TraceLayer::new_for_http().on_response(
            |res: &Response, latency: Duration, _: &Span| {
                let req_id = get_header(middleware::X_REQUEST_ID, res.headers());
                let message = format!(
                    "Completed with status {} in {} ms",
                    res.status(),
                    latency.as_millis()
                );
                match res.status() {
                    StatusCode::OK => tracing::info!(req_id = req_id, message),
                    StatusCode::INTERNAL_SERVER_ERROR => tracing::error!(req_id = req_id, message),
                    _ => tracing::warn!(req_id = req_id, message),
                }
            },
        ));

    // run our app with hyper, listening globally on port 3000
    let addr = get_addr().await;
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal(notify))
        .await
        .unwrap();
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

fn get_header<T>(header: T, headers: &axum::http::HeaderMap) -> &str
where
    T: axum::http::header::AsHeaderName,
{
    if let Some(v) = headers.get(header) {
        v.to_str().unwrap_or_else(|_| "<nil>")
    } else {
        "<nil>"
    }
}
