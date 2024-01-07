use std::collections::HashMap;

use axum::http::{HeaderName, Method};

pub struct IngressSpecification {
    domain_groups: HashMap<String, IngressDomainGroup>,
}

pub struct IngressDomainGroup {
    domain_name: String,
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
