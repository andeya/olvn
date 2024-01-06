use axum::body::{Body, HttpBody};
use axum::routing::future::RouteFuture;
use axum::routing::Router;
use axum_core::{
    extract::Request,
    response::{IntoResponse, Response},
};
use std::convert::Infallible;

use arc_swap::{ArcSwap, AsRaw};
use std::sync::Arc;
use std::task::{Context, Poll};
use tower_service::Service;

#[derive(Debug, Clone)]
pub(crate) struct DynamicRouter<S = ()> {
    inner: Arc<ArcSwap<Router<S>>>,
}

impl<S> Default for DynamicRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<S> DynamicRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(ArcSwap::from(Arc::new(Router::default()))),
        }
    }

    pub(crate) fn switch(&self, router: Router<S>) {
        self.inner.store(Arc::new(router))
    }
}

// for `axum::serve(listener, router)`
const _: () = {
    use axum::serve::IncomingStream;

    impl Service<IncomingStream<'_>> for DynamicRouter<()> {
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

impl<B> Service<Request<B>> for DynamicRouter<()>
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
        loop {
            return unsafe { &mut *self.inner.load().as_raw() }.call(req);
        }
    }
}

unsafe impl Send for DynamicRouter<()> {}
unsafe impl Sync for DynamicRouter<()> {}
