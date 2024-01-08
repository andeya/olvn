use std::collections::HashMap;

use axum::http::{HeaderName, Method};

use super::{Namespace, Domain};

pub struct IngressRegistry {
    specifications: HashMap<Namespace, IngressSpecification>,
}

pub struct IngressSpecification {
    namespace: Namespace,
    domain_groups: HashMap<Domain, IngressDomainGroup>,
}

pub struct IngressDomainGroup {
    domain_name: Domain,
    locations: Vec<IngressLocation>,
}

// via: nginx https://blog.51cto.com/blief/1739178
pub struct IngressLocation {
    id: u32,
    /// such as `/a/b/c`
    path: String,
    /// None means any method
    method: Option<Method>,
    proxy_hide_headers: Vec<HeaderName>,
    proxy_pass_headers: Vec<HeaderName>,
    upstream_server_id: u32,
    /// If None, proxy transparently
    upstream_api_id: Option<u32>,
}
