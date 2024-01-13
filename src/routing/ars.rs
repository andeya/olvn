use axum::http::Request;
use axum::middleware;
use axum::{middleware::Next, response::Response};
use tower::ServiceBuilder;

use crate::error::*;
use crate::state::Context;
use crate::{
    ars::{Ars, Method},
    error::GwError,
    routing::GwRouter,
    routing::Router,
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
            let mut router = Router::new();
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
                router = router.route(
                    location.path.as_str(),
                    top_level_handler_fn!(location.method, || async move {
                        // TODO:
                        format!("{:?}", service)
                    }),
                );
            }
            router = router.layer(ServiceBuilder::new().layer(middleware::from_fn(plugin)));
            gw_router = gw_router.route(ingress_domain_group.domain_name, router);
        }
        Ok(gw_router)
    }
}

async fn plugin(request: Request<axum_core::body::Body>, next: Next) -> Response {
    let mut state = request.extensions().get::<Context>().unwrap().clone();
    println!(
        "[PLUGIN-PRE] Handling a request to {}{}, state={:?}",
        state.host,
        request.uri(),
        state
    );
    state.host = "test".to_owned();
    let response = next.run(request).await;
    println!("[PLUGIN-POST] Returning a response, state={:?}", state);
    response
}
