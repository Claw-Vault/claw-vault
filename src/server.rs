use std::{net::SocketAddr, sync::Arc};

use axum::{extract::Request, Router};
use futures::{pin_mut, FutureExt};
use hyper::body::Incoming;
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use lib_core::config::Config;
use tokio::{
    net::{TcpListener, TcpStream},
    signal,
};
use tower::Service;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{app, routes};

/// Serves axum backend server
pub async fn serve() {
    let app = app::init().await;
    app.bootstrap().await;

    // build our application with a route
    // bind routes
    let router = get_router(app).await;

    // run our app with hyper, listening globally
    let listener = tokio::net::TcpListener::bind(Config::get_host_addr())
        .await
        .expect("Failed to start TCP listener");
    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    let (tx_signal, rx_signal) = tokio::sync::watch::channel(());
    let tx_signal = Arc::new(tx_signal);

    tokio::spawn(async move {
        shutdown_signal().await;
        tracing::debug!("received graceful shutdown signal. Telling tasks to shutdown");
        drop(rx_signal);
    });

    loop {
        let (tcp_stream, _addr) = tokio::select! {
            conn = tcp_accept(&listener) => match conn {
                Some(conn) => conn,
                None => continue,
            },
            _ = tx_signal.closed() => break,
        };

        if let Err(err) = tcp_stream.set_nodelay(true) {
            tracing::debug!("failed to set TCP_NODELAY on incoming connection: {err:#}");
        }
        let tcp_stream = TokioIo::new(tcp_stream);

        let tower_service = router.clone();
        let tx_signal = tx_signal.clone();
        tokio::runtime::Handle::current().spawn(async move {
            let hyper_service = hyper::service::service_fn(move |req: Request<Incoming>| {
                tower_service.clone().call(req)
            });

            let builder = Builder::new(TokioExecutor::new());
            let conn = builder.serve_connection_with_upgrades(tcp_stream, hyper_service);
            pin_mut!(conn);

            let signal_closed = tx_signal.closed().fuse();
            pin_mut!(signal_closed);

            loop {
                tokio::select! {
                    result = conn.as_mut() => {
                        if let Err(_err) = result {
                            tracing::debug!("failed to serve connection: {_err:#}");
                        }
                        break;
                    }
                    _ = &mut signal_closed => {
                        break;
                    }
                }
            }
        });
    }
}

fn is_connection_error(e: &std::io::Error) -> bool {
    matches!(
        e.kind(),
        std::io::ErrorKind::ConnectionRefused
            | std::io::ErrorKind::ConnectionAborted
            | std::io::ErrorKind::ConnectionReset
    )
}

async fn tcp_accept(listener: &TcpListener) -> Option<(TcpStream, SocketAddr)> {
    match listener.accept().await {
        Ok(conn) => Some(conn),
        Err(e) => {
            if is_connection_error(&e) {
                return None;
            }
            tracing::error!("accept error: {e}");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            None
        }
    }
}

pub async fn get_router(app: app::App) -> Router {
    // Prepare swagger
    let swagger =
        SwaggerUi::new("/swagger").url("/api-docs/openapi.json", routes::ApiDoc::openapi());

    routes::bind_routes(Router::<app::App>::new())
        .merge(swagger)
        .fallback(routes::fallback::fallback_handler)
        .layer(axum::middleware::from_fn(lib_core::interceptor::intercept))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(app)
}

/// Function that listens to signals and notify waiters
async fn shutdown_signal() {
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
}
