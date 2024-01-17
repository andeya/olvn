use crate::error::*;
use crate::plugin::layer;
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
                    .get(&location.upstream_service_id)
                    .context(NoUpstreamSnafu {
                        id: location.upstream_service_id,
                    })
                    .context(ArsSnafu)?;
                let service = Arc::new(service.clone());
                router = router.route(
                    location.path.as_str(),
                    top_level_handler_fn!(location.method, || async move {
                        // TODO:
                        let s = format!("{:?}", service);
                        println!("response: {:?}", s);
                        s
                    }),
                );
            }
            gw_router = gw_router.route(ingress_domain_group.domain_name, layer(router));
        }
        Ok(gw_router)
    }
}
