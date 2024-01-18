use axum_core::{extract::Request, response::IntoResponse};
use http::Response;

use crate::ars::{IngressLocationSpec, ServiceIdentifier};

mod discovery;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceEndpoint {
    pub identifier: ServiceIdentifier,
}

impl IngressLocationSpec {
    pub(crate) fn reverse_proxy(&self, req: Request<axum_core::body::Body>) -> Response<axum_core::body::Body> {
        println!("req: {:?}", req);
        format!("{:?}", self).into_response()
    }
}
