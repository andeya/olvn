use super::ServiceEndpoint;
use crate::error::*;

pub(crate) fn new_http_endpoint(addr: String) -> Result<Box<dyn ServiceEndpoint>, GwError> {
    Ok(Box::new(HttpEndpoint {}))
}

struct HttpEndpoint {}

impl ServiceEndpoint for HttpEndpoint {
    fn reverse_proxy(&self, req: axum_core::extract::Request) -> http::Response<axum_core::body::Body> {
        todo!()
    }
}
