use std::time::Duration;

use crate::core::cipher;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Router};
use tower_http::trace::TraceLayer;
use tracing::Span;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod core;
mod routes;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Prepare swagger
    let swagger =
        SwaggerUi::new("/swagger").url("/api-doc/openapi.json", routes::ApiDoc::openapi());

    // build our application with a route
    // bind routes
    let app = routes::bind_routes(Router::new())
        .merge(swagger)
        .layer(Extension(cipher::Cipher::new()))
        .layer(
            TraceLayer::new_for_http()
                .on_request(|_: &Request<_>, _: &Span| tracing::info!("Intercepted "))
                .on_response(|response: &Response, latency: Duration, _: &Span| {
                    tracing::info!(
                        "Completed with status {} in {} ms",
                        response.status(),
                        latency.as_millis()
                    )
                })
                // By default `TraceLayer` will log 5xx responses but we're doing our specific
                // logging of errors so disable that
                .on_failure(()),
        )
        .fallback(fallback_handler);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn fallback_handler() -> impl IntoResponse {
    (StatusCode::UNAUTHORIZED, "Nothing to see here")
}
