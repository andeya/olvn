use crate::ars::ArsExpand;
use crate::plugin::layer;
use crate::{
    ars::{Ars, Method},
    error::GwError,
    routing::GwRouter,
    routing::Router,
};

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
        let ars: ArsExpand = ars.try_into()?;
        let mut gw_router = GwRouter::new();
        println!("namespace: {}", &*ars.namespace);
        for (_, ingress_domain_group) in ars.domain_groups {
            println!("domain_name: {}", &*ingress_domain_group.domain_name);
            let mut router = Router::new();
            for route_spec in ingress_domain_group.routes {
                let path = route_spec.path.clone();
                router = router.route(
                    path.as_str(),
                    top_level_handler_fn!(route_spec.method, |req| async move { route_spec.reverse_proxy(req) }),
                );
            }
            gw_router = gw_router.route(ingress_domain_group.domain_name, layer(router));
        }
        Ok(gw_router)
    }
}
