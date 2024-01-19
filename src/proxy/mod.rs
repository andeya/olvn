use crate::ars::{Entity, IngressLocationSpec, ServiceIdentifier};
use crate::routing::{IntoResponse, Request, Response};
mod discovery;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceEndpoint {
    pub identifier: ServiceIdentifier,
}

impl IngressLocationSpec {
    #[inline]
    pub(crate) fn reverse_proxy(&self, req: Request) -> Response {
        println!("req: {:?}", req);
        format!("{:?}", self).into_response()
    }
    #[inline]
    fn request_to_entity(&self, _req: &Request) -> Option<Entity> {
        if let Some(upstream_method) = &self.upstream_method {
            let _codec = upstream_method
                .inbound_encoding_type
                .clone()
                .unwrap_or(self.upstream_service.default_encoding_type);
        } else {
        }
        unimplemented!()
    }
    #[inline]
    fn entity_to_response<T>(&self, _resp: T) -> Response {
        unimplemented!()
    }
}
