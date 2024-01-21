use std::sync::Arc;

use crate::plugin::layer;
use crate::proxy::ArsExpand;
use crate::transcoding::Transcoding;
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
    pub(crate) fn from_ars(ars: Ars, transcoding: Arc<Transcoding>) -> Result<GwRouter, GwError> {
        let ars = ArsExpand::try_from(ars, transcoding)?;
        let mut gw_router = GwRouter::new(None);
        println!("namespace: {}", &*ars.namespace);
        for (_, route_mapper) in ars.domain_groups {
            println!("domain_name: {}", &*route_mapper.domain_name);
            let mut router = Router::new();
            for proxy_handler in route_mapper.handlers {
                let path = proxy_handler.path.clone();
                router = router.route(
                    path.as_str(),
                    top_level_handler_fn!(
                        proxy_handler.method,
                        |req| async move { proxy_handler.reverse_proxy(req) }
                    ),
                );
            }
            gw_router = gw_router.route(route_mapper.domain_name, layer(router));
        }
        Ok(gw_router)
    }
}
