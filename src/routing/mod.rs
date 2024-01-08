pub use self::gw::GwRouter;
use self::gw::InnerGwRouter;
use crate::state::GwState;
use arc_swap::{ArcSwap, AsRaw};
use axum::body::HttpBody;
use axum::http::Request;
use axum::routing::future::RouteFuture;
use axum::routing::Router;
use axum_core::response::Response;
use std::convert::Infallible;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::Service;

mod domain;
pub mod gw;
pub(self) mod host;

pub type StateRouter = Router<GwState>;

#[derive(Debug, Clone)]
pub(crate) struct DynRouter {
    // routers[0] is the fallback router, and its domain is $FALLBACK_DOMAIN.
    inner_router: Arc<ArcSwap<InnerGwRouter>>,
}
unsafe impl Send for DynRouter {}
unsafe impl Sync for DynRouter {}
impl Default for DynRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl DynRouter {
    pub(crate) fn new() -> Self {
        Self {
            inner_router: Arc::new(ArcSwap::from(Arc::new(InnerGwRouter::default()))),
        }
    }
    // pub(crate) fn get_inner_routers(&self) -> &InnerDynamicRouter {
    //     unsafe { &*self.inner_router.load().as_raw() }
    // }
    pub(crate) fn refresh(&self, router: GwRouter) {
        self.inner_router.store(Arc::new(
            router.into_inner(|| self.inner_router.load().fallback_router().router.clone()),
        ))
    }
}

// for `axum::serve(listener, router)`
const _: () = {
    use axum::serve::IncomingStream;

    impl Service<IncomingStream<'_>> for DynRouter {
        type Response = Self;
        type Error = Infallible;
        type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, _req: IncomingStream<'_>) -> Self::Future {
            std::future::ready(Ok(self.clone()))
        }
    }
};

impl<B> Service<Request<B>> for DynRouter
where
    B: HttpBody<Data = bytes::Bytes> + Send + 'static,
    B::Error: Into<axum_core::BoxError>,
{
    type Response = Response;
    type Error = Infallible;
    type Future = RouteFuture<Infallible>;

    #[inline]
    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    fn call(&mut self, req: Request<B>) -> Self::Future {
        unsafe { &mut *self.inner_router.load().as_raw() }.call(req)
    }
}
