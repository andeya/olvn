use crate::ars::{Entity, RouteSpec, ServiceIdentifier};
use crate::routing::{IntoResponse, Request, Response};
mod discovery;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceEndpoint {
    pub identifier: ServiceIdentifier,
}

impl RouteSpec {
    #[inline]
    pub(crate) fn reverse_proxy(&self, req: Request) -> Response {
        println!("req: {:?}", req);
        format!("{:?}", self).into_response()
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
