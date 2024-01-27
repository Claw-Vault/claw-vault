use std::time::Duration;

use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use rand::RngCore;

pub async fn intercept(mut req: Request, next: Next) -> Response {
    let id = generate_id();
    tracing::info!(req_id = id, method = ?req.method(), uri=?req.uri());

    req.extensions_mut().insert(id.clone());

    let start = std::time::Instant::now();
    let res = next.run(req).await;
    log_response(id, res, start.elapsed())
}

fn generate_id() -> String {
    let mut rng = rand::thread_rng();
    let mut b = [0u8; 3];
    rng.fill_bytes(&mut b);

    hex::encode(b)
}

fn log_response(req_id: String, res: Response, latency: Duration) -> Response {
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
    res
}
