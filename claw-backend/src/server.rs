use lib_core::{
    config::Config,
    interceptor::{ReqIDInterceptor, ServiceInterceptor},
};
use lib_domain::claw_proto::{self, claw_vault_server::ClawVaultServer};
use tokio::signal;

use crate::app::{self};

pub async fn serve() {
    let app = app::init().await;
    app.bootstrap().await;

    let shutdown = shutdown_signal(app.clone());
    let addr = Config::get_host_addr().parse().expect("Failed to parse socket addr");

    tonic::transport::Server::builder()
        .add_service(
            tonic_reflection::server::Builder::configure()
                .register_encoded_file_descriptor_set(claw_proto::FILE_DESCRIPTOR_SET)
                .build_v1alpha()
                .expect("Failed to build reflection service"),
        )
        .add_service(ServiceInterceptor::new(
            ClawVaultServer::from_arc(app),
            ReqIDInterceptor::default(),
        ))
        .serve_with_shutdown(addr, shutdown)
        .await
        .expect("Failed to start gRPC server");
}

/// Function that listens to signals and notify waiters
async fn shutdown_signal(app: app::App) {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    app.close().await;
}
