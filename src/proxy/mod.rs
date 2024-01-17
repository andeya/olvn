use axum_core::extract::Request;

use crate::ars::{IngressLocationSpec, ServiceIdentifier};

mod discovery;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceEndpoint {
    pub identifier: ServiceIdentifier,
}

impl IngressLocationSpec {
    pub(crate) fn reverse_proxy(&self, req: Request) {}
}
