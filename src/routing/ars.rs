use axum::http::Request;
use std::task::{Context, Poll};
use tower::Service;

use crate::error::*;
use crate::{
    ars::{Ars, Method},
    error::GwError,
    routing::GwRouter,
    routing::StateRouter,
};
use std::sync::Arc;

macro_rules! top_level_handler_fn {
    ($method:expr, $handler:expr) => {
        match $method {
            Method::Get => axum::routing::get($handler),
            Method::Post => axum::routing::post($handler),
            Method::Put => axum::routing::put($handler),
            Method::Delete => axum::routing::delete($handler),
            Method::Head => axum::routing::head($handler),
            Method::Options => axum::routing::options($handler),
            Method::Trace => axum::routing::trace($handler),
            Method::Patch => axum::routing::patch($handler),
            Method::Any => axum::routing::any($handler),
        }
    };
}

impl GwRouter {
    pub(crate) fn from_ars(ars: Ars) -> Result<GwRouter, GwError> {
        let mut gw_router = GwRouter::new();
        println!("namespace: {}", &*ars.namespace);
        for (_, ingress_domain_group) in ars.ingress.domain_groups {
            println!("domain_name: {}", &*ingress_domain_group.domain_name);
            let mut router = StateRouter::new();
            for location in ingress_domain_group.locations {
                let service = ars
                    .egress
                    .services
                    .get(&location.upstream_server_id)
                    .context(NoUpstreamSnafu {
                        id: location.upstream_server_id,
                    })
                    .context(ArsSnafu)?;
                let service = Arc::new(service.clone());
                router = router
                    .route(
                        location.path.as_str(),
                        top_level_handler_fn!(location.method, || async move {
                            // TODO:
                            format!("{:?}", service)
                        }),
                    )
                    .route_layer(PluginRouteLayer);
            }
            gw_router = gw_router.route(ingress_domain_group.domain_name, router);
        }
        Ok(gw_router)
    }
}

#[derive(Debug, Clone)]
struct PluginRouteLayer;

#[derive(Debug, Clone)]
struct PluginRouteService<S> {
    inner: S,
}

impl<S> tower::Layer<S> for PluginRouteLayer {
    type Service = PluginRouteService<S>;

    // 初始化服务
    fn layer(&self, inner: S) -> Self::Service {
        PluginRouteService { inner }
    }
}

impl<ResBody, S> Service<Request<ResBody>> for PluginRouteService<S>
where
    S: Service<Request<ResBody>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    #[allow(unused_mut)]
    fn call(&mut self, mut req: Request<ResBody>) -> Self::Future {
        println!("[PLUGIN-PRE] Handling a request to {}", req.uri());
        let resp = self.inner.call(req);
        println!("[PLUGIN-POST] Returning a response");
        resp
    }
}
