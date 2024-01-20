use std::sync::Arc;

use crate::ars::{Entity, RouteSpec, ServiceIdentifier};
use crate::converter::ConverterIndex;
use crate::routing::{IntoResponse, Request, Response};
mod discovery;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceEndpoint {
    pub identifier: ServiceIdentifier,
}

#[derive(Debug, Clone)]
pub struct ProxyHandler {
    route_spec: RouteSpec,
    converter_index: Arc<ConverterIndex>,
}

impl ProxyHandler {
    pub(crate) fn new(route_spec: RouteSpec, converter_index: Arc<ConverterIndex>) -> Self {
        Self {
            route_spec,
            converter_index,
        }
    }

    #[inline]
    pub(crate) fn reverse_proxy(&self, req: Request) -> Response {
        println!("{:?}", req);
        format!("{:?}", self.route_spec).into_response()
    }
    #[inline]
    fn convert_request(&self, _req: &Request) -> Option<Entity> {
        unimplemented!()
    }
    #[inline]
    fn convert_response<T>(&self, _resp: T) -> Response {
        unimplemented!()
    }
}
