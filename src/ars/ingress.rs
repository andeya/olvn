use std::{collections::HashMap, sync::Arc};

use super::{
    egress::{MethodSpec, ServiceSpec},
    Domain,
};

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HeaderName(String);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Method {
    Any,
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Patch,
}

impl Default for Method {
    fn default() -> Self {
        Method::Any
    }
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IngressSpec {
    pub domain_groups: HashMap<Domain, IngressDomainGroup>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IngressDomainGroup {
    pub domain_name: Domain,
    pub locations: Vec<IngressLocation>,
}

// via: nginx https://blog.51cto.com/blief/1739178
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IngressLocation {
    pub id: u32,
    /// such as `/a/b/c`
    pub path: String,
    pub method: Method,
    pub proxy_hide_headers: Vec<HeaderName>,
    pub proxy_pass_headers: Vec<HeaderName>,
    pub upstream_server_id: u32,
    /// If None, proxy transparently
    pub upstream_api_id: Option<u32>,
}

#[derive(Debug)]
pub struct InnerIngressLocation {
    pub id: u32,
    /// such as `/a/b/c`
    pub path: String,
    pub method: Method,
    pub proxy_hide_headers: Vec<HeaderName>,
    pub proxy_pass_headers: Vec<HeaderName>,
    pub upstream_server: Arc<ServiceSpec>,
    /// If None, proxy transparently
    pub upstream_api_id: Arc<Option<MethodSpec>>,
}
