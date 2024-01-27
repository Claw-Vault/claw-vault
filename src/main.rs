use std::sync::Arc;

use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod core;
mod handlers;
mod routes;
mod server;
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
    server::serve(app, notify).await;
}
