use std::collections::HashMap;

use axum::http::{HeaderName, Method};

use super::{Domain, Namespace};

pub struct IngressRegistry {
    pub specifications: HashMap<Namespace, IngressSpecification>,
}

pub struct IngressSpecification {
    pub namespace: Namespace,
    pub domain_groups: HashMap<Domain, IngressDomainGroup>,
}

pub struct IngressDomainGroup {
    pub domain_name: Domain,
    pub locations: Vec<IngressLocation>,
}

// via: nginx https://blog.51cto.com/blief/1739178
pub struct IngressLocation {
    pub id: u32,
    /// such as `/a/b/c`
    pub path: String,
    /// None means any method
    pub method: Option<Method>,
    pub proxy_hide_headers: Vec<HeaderName>,
    pub proxy_pass_headers: Vec<HeaderName>,
    pub upstream_server_id: u32,
    /// If None, proxy transparently
    pub upstream_api_id: Option<u32>,
}
