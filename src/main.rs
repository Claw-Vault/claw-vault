use std::sync::Arc;
use std::time::Duration;

use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Router};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tower_http::trace::TraceLayer;
use tracing::Span;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod app;
mod core;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Connect to database
    let db = connect_db().await;

    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Prepare swagger
    let swagger =
        SwaggerUi::new("/swagger").url("/api-doc/openapi.json", routes::ApiDoc::openapi());

    // initialize app
    let app = Arc::new(app::App::new(db));

    // build our application with a route
    // bind routes
    let router = routes::bind_routes(Router::new())
        .merge(swagger)
        .layer(Extension(app))
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
    axum::serve(listener, router).await.unwrap();
}

async fn connect_db() -> DatabaseConnection {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut opts = ConnectOptions::new(db_url.to_owned());
    opts.max_connections(255)
        .min_connections(10)
        .connect_timeout(Duration::from_secs(15))
        .sqlx_logging(false);

    Database::connect(opts)
        .await
        .expect("Failed to connect database")
}

async fn fallback_handler() -> impl IntoResponse {
    (StatusCode::UNAUTHORIZED, "Nothing to see here")
}
