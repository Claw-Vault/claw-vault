use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod routes;
mod server;
mod tests;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .with(tracing_subscriber::fmt::layer().with_thread_ids(true).json().flatten_event(true))
        .init();

    // load env
    dotenv::dotenv().ok();

    // serve app
    server::serve().await;

    tracing::info!("Server has stopped.");
}
