use crate::ars::Domain;
use crate::state::GwState;
use crate::StateRouter;
use axum::body::HttpBody;
use axum::http::Request;
use axum::routing::future::RouteFuture;
use axum::Router;
use http::StatusCode;
use std::collections::BTreeMap;
use std::convert::Infallible;
use tower::Service;

use super::domain::{DomainRouter, FALLBACK_NO_DOMAIN};

#[derive(Debug, Clone)]
pub struct GwRouter(BTreeMap<Domain, StateRouter>);

impl GwRouter {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    #[inline]
    pub fn from_fallback(fallback: StateRouter) -> Self {
        Self::new().fallback(Some(fallback))
    }
    #[inline]
    pub fn fallback(mut self, fallback: Option<StateRouter>) -> Self {
        let _ = self
            .0
            .insert(FALLBACK_NO_DOMAIN, fallback.unwrap_or_else(not_found_router));
        self
    }
    pub fn route(mut self, domain: Domain, router: StateRouter) -> Self {
        let _ = self.0.insert(domain, router);
        self
    }
    pub fn get(&self, domain: &Domain) -> Option<&StateRouter> {
        self.0.get(domain)
    }
    pub fn get_mut(&mut self, domain: &Domain) -> Option<&mut StateRouter> {
        self.0.get_mut(domain)
    }
    pub(crate) fn into_inner<F: Fn() -> Router>(mut self, fallback: F) -> InnerGwRouter {
        let state = GwState::default();
        let mut inner = InnerGwRouter::new(Some(if let Some(fallback) = self.0.remove(&FALLBACK_NO_DOMAIN) {
            fallback.with_state(state.clone())
        } else {
            fallback()
        }));
        for ele in self.0 {
            inner.0.push(DomainRouter::new(ele.0, ele.1.with_state(state.clone())));
        }
        inner
    }
}

#[derive(Debug, Clone)]
pub(crate) struct InnerGwRouter(Vec<DomainRouter>);

impl Default for InnerGwRouter {
    fn default() -> Self {
        Self::new(None)
    }
}

impl InnerGwRouter {
    fn new(fallback_router: Option<Router>) -> Self {
        Self(vec![DomainRouter::new(
            FALLBACK_NO_DOMAIN.clone(),
            fallback_router.unwrap_or_else(not_found_router),
        )])
    }
    #[inline]
    pub(crate) fn fallback_router(&self) -> &DomainRouter {
        &self.0[0]
    }
    #[inline]
    pub(crate) fn fallback_router_mut(&mut self) -> &mut DomainRouter {
        &mut self.0[0]
    }
}

impl InnerGwRouter {
    #[inline]
    pub(crate) fn call<B>(&mut self, req: Request<B>) -> RouteFuture<Infallible>
    where
        B: HttpBody<Data = bytes::Bytes> + Send + 'static,
        B::Error: Into<axum_core::BoxError>,
    {
        if let Some(hostname) = host::get_host_from_request(&req) {
            for router in &mut self.0[1..] {
                if router.domain == hostname {
                    return router.router.call(req);
                }
            }
        }
        return self.fallback_router_mut().router.call(req);
    }
}

pub(crate) fn not_found_router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new().fallback(|req: Request<axum_core::body::Body>| async move {
        return (
            StatusCode::NOT_FOUND,
            format!(
                "404. The requested URL {}{} was not found on this server.",
                host::get_host_from_request(&req).unwrap_or_default(),
                req.uri().path(),
            ),
        );
    })
}

mod host {
    use axum::body::HttpBody;
    use axum::http::Request;
    use http::header::{HeaderMap, FORWARDED};

    const X_FORWARDED_HOST_HEADER_KEY: &str = "X-Forwarded-Host";

    /// Extractor that resolves the hostname of the request.
    ///
    /// Hostname is resolved through the following, in order:
    /// - `Forwarded` header
    /// - `X-Forwarded-Host` header
    /// - `Host` header
    /// - request target / URI
    ///
    /// Note that user agents can set `X-Forwarded-Host` and `Host` headers to arbitrary values so make
    /// sure to validate them to avoid security issues.
    pub(super) type Host = String;

    pub(super) fn get_host_from_request<B>(req: &Request<B>) -> Option<Host>
    where
        B: HttpBody<Data = bytes::Bytes> + Send + 'static,
        B::Error: Into<axum_core::BoxError>,
    {
        let headers = req.headers();
        if let Some(host) = parse_forwarded(&headers) {
            return Some(host.to_owned());
        }

        if let Some(host) = headers
            .get(X_FORWARDED_HOST_HEADER_KEY)
            .and_then(|host| host.to_str().ok())
        {
            return Some(host.to_owned());
        }

        if let Some(host) = headers.get(http::header::HOST).and_then(|host| host.to_str().ok()) {
            return Some(host.to_owned());
        }

        if let Some(host) = req.uri().host() {
            return Some(host.to_owned());
        }

        None
    }

    #[allow(warnings)]
    fn parse_forwarded(headers: &HeaderMap) -> Option<&str> {
        // if there are multiple `Forwarded` `HeaderMap::get` will return the first one
        let forwarded_values = headers.get(FORWARDED)?.to_str().ok()?;

        // get the first set of values
        let first_value = forwarded_values.split(',').nth(0)?;

        // find the value of the `host` field
        first_value.split(';').find_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            key.trim()
                .eq_ignore_ascii_case("host")
                .then(|| value.trim().trim_matches('"'))
        })
    }
}
