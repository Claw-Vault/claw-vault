use std::sync::Arc;
use std::time::Duration;

use app::{App, HtmlTemplate};
use axum::extract::Request;
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
mod routes;
mod tests;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_line_number(true)
                .with_thread_ids(true)
                .with_thread_names(true),
        )
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
        .fallback(fallback_handler)
        .layer(Extension(app))
        .layer(
            TraceLayer::new_for_http()
                .on_request(|req: &Request<_>, _: &Span| {
                    let agent =
                        if let Some(agent) = req.headers().get(axum::http::header::USER_AGENT) {
                            agent.to_str().unwrap_or_else(|_| "<err>")
                        } else {
                            "<nil>"
                        };

                    tracing::info!(user_agent = agent, "{} {}", req.method(), req.uri());
                })
                .on_response(|response: &Response, latency: Duration, _: &Span| {
                    tracing::info!(
                        "Completed with status {} in {} ms",
                        response.status(),
                        latency.as_millis(),
                    )
                })
                // By default `TraceLayer` will log 5xx responses but we're doing our specific
                // logging of errors so disable that
                .on_failure(()),
        );

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

/// Handler for routes that are not defined
async fn fallback_handler(Extension(app): Extension<Arc<App>>) -> HtmlTemplate {
    let (_, _, tera) = app.expand();
    HtmlTemplate(tera, "404.html", None)
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
