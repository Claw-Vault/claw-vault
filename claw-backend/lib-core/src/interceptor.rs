use std::sync::Arc;

use chrono::Utc;
use http::{HeaderMap, HeaderValue};

pub type ReqId = Arc<str>;

const REQ_ID_HEADER: &str = "x-cv-id";

pub trait RpcService:
    tonic::codegen::Service<
        tonic::codegen::http::Request<tonic::body::BoxBody>,
        Response = tonic::codegen::http::Response<tonic::body::BoxBody>,
    > + Send
    + Clone
    + 'static
{
}

impl<T> RpcService for T where
    T: tonic::codegen::Service<
            tonic::codegen::http::Request<tonic::body::BoxBody>,
            Response = tonic::codegen::http::Response<tonic::body::BoxBody>,
        > + Send
        + Clone
        + 'static
{
}

#[tonic::async_trait]
pub trait RpcInterceptor<S>
where
    S: RpcService,
{
    async fn call(
        &self,
        req: tonic::codegen::http::Request<tonic::body::BoxBody>,
        service: S,
    ) -> Result<tonic::codegen::http::Response<tonic::body::BoxBody>, S::Error>;
}

#[derive(Clone)]
pub struct ServiceInterceptor<S, I>
where
    S: RpcService,
    I: RpcInterceptor<S>,
{
    inner: S,
    interceptor: I,
}

impl<S, I> ServiceInterceptor<S, I>
where
    S: RpcService,
    I: RpcInterceptor<S>,
{
    pub fn new(inner: S, interceptor: I) -> Self {
        Self { inner, interceptor }
    }
}

impl<S, I> tonic::codegen::Service<tonic::codegen::http::Request<tonic::body::BoxBody>>
    for ServiceInterceptor<S, I>
where
    S: RpcService + Sync,
    S::Future: Send,
    I: RpcInterceptor<S> + Send + Clone + 'static + Sync,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = tonic::codegen::BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: tonic::codegen::http::Request<tonic::body::BoxBody>) -> Self::Future {
        let interceptor = self.interceptor.clone();
        let inner = self.inner.clone();
        Box::pin(async move { interceptor.call(req, inner).await })
    }
}

impl<S, I> tonic::server::NamedService for ServiceInterceptor<S, I>
where
    S: tonic::server::NamedService + RpcService,
    I: RpcInterceptor<S>,
{
    const NAME: &'static str = S::NAME;
}

#[derive(Default, Clone)]
pub struct ReqIDInterceptor;

#[tonic::async_trait]
impl<S> RpcInterceptor<S> for ReqIDInterceptor
where
    S: RpcService,
    S::Future: Send,
{
    async fn call(
        &self,
        req: tonic::codegen::http::Request<tonic::body::BoxBody>,
        mut service: S,
    ) -> Result<tonic::codegen::http::Response<tonic::body::BoxBody>, S::Error> {
        let id: ReqId = {
            let timestamp = Utc::now().timestamp_millis();
            let id = nanoid::nanoid!(22);

            format!("{}_{}", id, timestamp).into()
        };
        let uri = req.uri().path();

        let method = req.method().clone();
        tracing::info!(req_id = id.as_ref(), message = format!("{}: {}", method, uri), method = ?method, uri = ?uri);

        let start = std::time::Instant::now();
        let mut res = service.call(req).await?;
        res.headers_mut()
            .insert(REQ_ID_HEADER, tonic::codegen::http::HeaderValue::from_str(&id).unwrap());

        log_response(id, res.headers(), start.elapsed());
        Ok(res)
    }
}

fn log_response(req_id: ReqId, headers: &HeaderMap<HeaderValue>, duration: std::time::Duration) {
    let status = headers
        .get("grpc-status")
        .cloned()
        .map(|h| tonic::Code::from_bytes(h.to_str().unwrap().as_bytes()))
        .unwrap_or(tonic::Code::Ok);

    let grpc_msg = headers
        .get("grpc-message")
        .cloned()
        .map(|h| h.to_str().unwrap().replace("%20", " "))
        .unwrap_or(String::default());

    let message = format!("Completed with status {}: {}", status as u8, status.to_string());
    let duration = format!("{:?}", duration);
    match status {
        tonic::Code::Internal => {
            tracing::error!(req_id = req_id.as_ref(), message, duration, grpc_msg)
        }
        _ => tracing::warn!(req_id = req_id.as_ref(), message, duration, grpc_msg),
    };
}
