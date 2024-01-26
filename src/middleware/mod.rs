use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use rand::RngCore;

pub const X_REQUEST_ID: &str = "X-Req-Id";

pub async fn request_id(req: Request, next: Next) -> Response {
    let id = generate_id();
    let id = id.as_str();

    let mut req = req;
    req.headers_mut()
        .append(X_REQUEST_ID, HeaderValue::from_str(id).unwrap());

    let mut res = next.run(req).await;
    res.headers_mut()
        .append(X_REQUEST_ID, HeaderValue::from_str(id).unwrap());
    res
}

fn generate_id() -> String {
    let mut rng = rand::thread_rng();
    let mut b = [0u8; 3];
    rng.fill_bytes(&mut b);

    hex::encode(b)
}
