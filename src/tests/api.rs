use axum::{
    body::{Body, Bytes},
    http::{header, Method, Request, Response, StatusCode},
};
use http_body_util::BodyExt;
use tower::util::ServiceExt;

use crate::{app, server};

#[tokio::test]
async fn encrypt_empty_body() {
    dotenv::dotenv().ok();

    let response = encrypt_req(Body::empty()).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn encrypt_1_min() {
    encrypt_n_min(r#"{ "validity": 60, "data": "random data" }"#, r#""valid_for":"1 minute""#)
        .await;
}

#[tokio::test]
async fn encrypt_15_mins() {
    encrypt_n_min(r#"{ "validity": 900, "data": "random data" }"#, r#""valid_for":"15 minutes""#)
        .await;
}

#[tokio::test]
async fn encrypt_30_mins() {
    encrypt_n_min(r#"{ "validity": 1800, "data": "random data" }"#, r#""valid_for":"30 minutes""#)
        .await;
}

async fn encrypt_n_min(body: &'static str, expected: &'static str) {
    dotenv::dotenv().ok();

    let response = encrypt_req(Body::from(body)).await;
    assert_eq!(response.status(), StatusCode::OK);

    let (_, b) = response.into_parts();
    let b = get_body(b).await;
    assert!(b.contains(expected));
}

#[tokio::test]
async fn decrypt_empty_body() {
    dotenv::dotenv().ok();

    let response = decrypt_req(Body::empty()).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn decrypt() {
    dotenv::dotenv().ok();

    let er = encrypt_req(Body::from(r#"{ "validity": 60, "data": "random data" }"#)).await;
    let eb = er.into_parts();
    let mut eb = get_body(eb.1).await;
    eb = eb.replace("data_id", "id");
    eb = eb.replace("key_id", "key");
    eb = eb.replace(r#""valid_for":"1 minute""#, "");
    eb = eb.replace(",}", "}");

    let response = decrypt_req(Body::from(eb)).await;
    assert_eq!(response.status(), StatusCode::OK);

    let (_, b) = response.into_parts();
    let b = get_body(b).await;
    assert!(b.contains(r#""data":"random data""#));
}

async fn encrypt_req(body: Body) -> Response<Body> {
    req(body, "/api/v1/encrypt").await
}

async fn decrypt_req(body: Body) -> Response<Body> {
    req(body, "/api/v1/decrypt").await
}

async fn req(body: Body, uri: &'static str) -> Response<Body> {
    let app = app::init().await;
    app.bootstrap().await;
    let router = server::get_router(app.clone()).await;

    let response = router
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(uri)
                .header(header::CONTENT_TYPE, "application/json")
                .body(body)
                .unwrap(),
        )
        .await
        .unwrap();

    response
}

// Consumes body and prints
async fn get_body<B>(body: B) -> String
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => return String::from("<nil>"),
    };

    if let Ok(b) = String::from_utf8(bytes.to_vec()) {
        b
    } else {
        String::from("<nil>")
    }
}
