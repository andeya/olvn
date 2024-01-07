use crate::routing::host::get_host_from_request;
use crate::routing::host::Host;
use axum::body::HttpBody;
use axum::http::Request;
use axum::routing;
use axum::routing::future::RouteFuture;
use axum::Router;
use http::StatusCode;
use std::collections::BTreeMap;
use std::convert::Infallible;
use tower::Service;

const FALLBACK_HOST: Host = Host::new();

#[derive(Debug, Clone)]
pub(crate) struct HostRouter {
    pub host: Host, // $FALLBACK_HOST is fallback
    pub router: Router,
}
impl Default for HostRouter {
    fn default() -> Self {
        Self::new("".to_owned(), Router::new())
    }
}
impl HostRouter {
    pub fn new(host: Host, router: Router) -> Self {
        Self { host: host, router }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct InnerDynamicRouter(Vec<HostRouter>);

impl Default for InnerDynamicRouter {
    fn default() -> Self {
        Self::new(None)
    }
}

impl InnerDynamicRouter {
    fn new(fallback_router: Option<Router>) -> Self {
        Self(vec![HostRouter::new(
            FALLBACK_HOST.clone(),
            fallback_router.unwrap_or_else(not_found_router),
        )])
    }
    #[inline]
    pub(crate) fn fallback_router(&self) -> &HostRouter {
        &self.0[0]
    }
    #[inline]
    pub(crate) fn fallback_router_mut(&mut self) -> &mut HostRouter {
        &mut self.0[0]
    }
}

impl InnerDynamicRouter {
    #[inline]
    pub(crate) fn call<B>(&mut self, req: Request<B>) -> RouteFuture<Infallible>
    where
        B: HttpBody<Data = bytes::Bytes> + Send + 'static,
        B::Error: Into<axum_core::BoxError>,
    {
        if let Some(hostname) = get_host_from_request(&req) {
            for router in &mut self.0[1..] {
                if router.host == hostname {
                    return router.router.call(req);
                }
            }
        }
        return self.fallback_router_mut().router.call(req);
    }
}

pub(crate) fn not_found_router() -> Router {
    Router::new().route(
        "/*path",
        routing::any(|| async {
            return StatusCode::NOT_FOUND;
        }),
    )
}

#[derive(Debug, Clone)]
pub struct DynamicRouter(BTreeMap<Host, Router>);

impl DynamicRouter {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    #[inline]
    pub fn from_fallback(fallback: Router) -> Self {
        Self::new().set_fallback(Some(fallback))
    }
    #[inline]
    pub fn set_fallback(mut self, fallback: Option<Router>) -> Self {
        let _ = self.0.insert(FALLBACK_HOST, fallback.unwrap_or_else(not_found_router));
        self
    }
    pub(crate) fn into_inner<F: Fn() -> Router>(mut self, fallback: F) -> InnerDynamicRouter {
        let mut inner = InnerDynamicRouter::new(Some(if let Some(fallback) = self.0.remove(FALLBACK_HOST.as_str()) {
            fallback
        } else {
            fallback()
        }));
        for ele in self.0 {
            inner.0.push(HostRouter::new(ele.0, ele.1));
        }
        inner
    }
}
