mod access_log;
use crate::state::GwContext;
pub use access_log::*;
use axum::http::Request;
use axum::middleware;
use axum::Router;
use axum::{middleware::Next, response::Response};
use axum_core::body::Body;
use std::ops::{Deref, DerefMut};
use tower::ServiceBuilder;

pub fn layer<S: Clone + Send + Sync + 'static>(router: Router<S>) -> Router<S> {
    router.layer(ServiceBuilder::new().layer(middleware::from_fn(AccessLog::plugin)))
}

#[allow(async_fn_in_trait)]
pub trait Plugin: Clone {
    async fn pre(req_ctx: RequestCtx<'_>) -> Option<Response<Body>>;
    async fn post(resp_ctx: ResponseCtx);
    async fn plugin(mut request: Request<Body>, next: Next) -> Response {
        let req_ctx = RequestCtx::new(&mut request);
        let state = req_ctx.state().clone();
        let pre_resp = Self::pre(req_ctx).await;
        if let Some(resp) = pre_resp {
            return resp;
        }
        let mut response = next.run(request).await;
        Self::post(ResponseCtx::new(&mut response, state)).await;
        response
    }
}

pub struct RequestCtx<'a> {
    request: &'a mut Request<Body>,
}

impl Deref for RequestCtx<'_> {
    type Target = Request<Body>;

    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl DerefMut for RequestCtx<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

impl<'a> RequestCtx<'a> {
    fn new(request: &'a mut Request<Body>) -> Self {
        Self { request }
    }
    pub fn state_mut(&'a mut self) -> &'a mut GwContext {
        self.request.extensions_mut().get_mut::<GwContext>().unwrap()
    }
    pub fn state(&'a self) -> &'a GwContext {
        self.request.extensions().get::<GwContext>().unwrap()
    }
}

pub struct ResponseCtx<'a> {
    response: &'a mut Response<Body>,
    state: GwContext,
}

impl Deref for ResponseCtx<'_> {
    type Target = Response<Body>;

    fn deref(&self) -> &Self::Target {
        &self.response
    }
}

impl DerefMut for ResponseCtx<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.response
    }
}

impl<'a> ResponseCtx<'a> {
    fn new(response: &'a mut Response<Body>, state: GwContext) -> Self {
        Self { response, state }
    }
    pub fn state_mut(&'a mut self) -> &'a mut GwContext {
        &mut self.state
    }
    pub fn state(&'a self) -> &'a GwContext {
        &self.state
    }
}
