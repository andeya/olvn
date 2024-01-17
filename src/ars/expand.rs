use std::{collections::HashMap, sync::Arc};

use crate::error::GwError;

use super::{Ars, Domain, HeaderName, Method, MethodSpec, Namespace, ServiceSpec};

#[derive(Debug)]
pub struct IngressDomainGroupExpand {
    pub domain_name: Domain,
    pub locations: Vec<IngressLocationSpec>,
}

#[derive(Debug)]
pub struct IngressLocationSpec {
    pub id: u32,
    /// such as `/a/b/c`
    pub path: String,
    pub method: Method,
    pub proxy_hide_headers: Vec<HeaderName>,
    pub proxy_pass_headers: Vec<HeaderName>,
    pub upstream_service: Arc<ServiceSpec>,
    /// If None, proxy transparently
    pub upstream_method: Arc<Option<MethodSpec>>,
}

#[derive(Debug)]
pub struct ArsExpand {
    pub namespace: Namespace,
    pub domain_groups: HashMap<Domain, IngressDomainGroupExpand>,
}

impl TryFrom<Ars> for ArsExpand {
    type Error = GwError;

    fn try_from(value: Ars) -> Result<Self, Self::Error> {
        todo!()
    }
}
