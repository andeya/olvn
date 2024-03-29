use std::collections::HashMap;

use fake::Dummy;

use super::Domain;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, Dummy)]
pub struct HeaderName(String);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, derive_more::Display, Dummy)]
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

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, Dummy)]
pub struct IngressSpec {
    pub domain_groups: HashMap<Domain, IngressRouteMapper>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, Dummy)]
pub struct IngressRouteMapper {
    pub domain_name: Domain,
    pub routes: Vec<IngressRouteSpec>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, Dummy)]
pub struct IngressRouteSpec {
    pub id: u32,
    /// such as `/a/b/c`
    pub path: String,
    pub method: Method,
    pub proxy_hide_headers: Vec<HeaderName>,
    pub proxy_pass_headers: Vec<HeaderName>,
    pub upstream_service_id: u32,
    /// If None, use automapper
    pub upstream_method_name: Option<String>,
}
