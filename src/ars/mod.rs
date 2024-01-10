//! # API Runtime Specification
//!
//! API Runtime Specification (ARS), all necessary data
//! required for normal request traffic in the gateway data plane.

pub mod egress;
pub mod ingress;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ARS {
    pub ingress: ingress::IngressRegistry,
    pub egress: egress::EgressRegistry,
}

impl ARS {
    pub fn new() -> Self {
        Self {
            ingress: ingress::IngressRegistry::default(),
            egress: egress::EgressRegistry::default(),
        }
    }
}

#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Namespace(String);

#[derive(Debug, Clone, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Domain(pub String);
impl Default for Domain {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl PartialEq<String> for Domain {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl Domain {
    pub const fn new() -> Self {
        Self(String::new())
    }
}
use axum::body::Body;
use axum::http::{Request, Response};

use self::ingress::InnerIngressLocation;

#[allow(unused)]
fn reverse_proxy(_location: &InnerIngressLocation, _req: Request<Body>) -> Response<Body> {
    todo!()
}
