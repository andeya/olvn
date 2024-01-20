use std::{collections::HashMap, sync::Arc};

use crate::converter::ConverterIndex;
use crate::error::*;

use crate::ars::{Ars, Domain, HeaderName, Method, MethodSpec, Namespace, ServiceSpec};

#[derive(Debug, Clone)]
pub struct ArsExpand {
    pub(crate) namespace: Namespace,
    pub(crate) domain_groups: HashMap<Domain, RouteMapper>,
}

#[derive(Debug, Clone)]
pub struct RouteMapper {
    pub(crate) domain_name: Domain,
    pub(crate) handlers: Vec<Arc<ProxyHandler>>,
}

#[derive(Debug)]
pub struct ProxyHandler {
    pub(crate) id: u32,
    /// such as `/a/b/c`
    pub(crate) path: String,
    pub(crate) method: Method,
    pub(crate) proxy_hide_headers: Vec<HeaderName>,
    pub(crate) proxy_pass_headers: Vec<HeaderName>,
    pub(crate) upstream_service: Arc<ServiceSpec>,
    /// If None, proxy transparently
    pub(crate) upstream_method: Option<Arc<MethodSpec>>,
    pub(crate) converter_index: Arc<ConverterIndex>,
}

impl ArsExpand {
    pub fn try_from(value: Ars, converter_index: Arc<ConverterIndex>) -> Result<Self, GwError> {
        let namespace = value.namespace;
        let mut domain_groups = HashMap::new();
        let services = value.egress.services;

        for (domain, domain_group) in value.ingress.domain_groups {
            let mut handlers = Vec::new();

            for location in domain_group.routes {
                let id = location.id;
                let path = location.path;
                let method = location.method;
                let proxy_hide_headers = location.proxy_hide_headers;
                let proxy_pass_headers = location.proxy_pass_headers;
                let upstream_service = services
                    .get(&location.upstream_service_id)
                    .context(NoUpstreamServiceSnafu {
                        id: location.upstream_service_id,
                    })
                    .context(ArsSnafu)?
                    .clone();

                let upstream_method = if let Some(upstream_method_id) = location.upstream_method_id {
                    Some(
                        upstream_service
                            .methods
                            .get(&upstream_method_id)
                            .context(NoUpstreamMethodSnafu { id: upstream_method_id })
                            .context(ArsSnafu)?
                            .clone(),
                    )
                } else {
                    None
                };

                let handler = Arc::new(ProxyHandler {
                    id,
                    path,
                    method,
                    proxy_hide_headers,
                    proxy_pass_headers,
                    upstream_service,
                    upstream_method,
                    converter_index: converter_index.clone(),
                });

                handlers.push(handler);
            }

            let ingress_domain_group_expand = RouteMapper {
                domain_name: domain.clone(),
                handlers,
            };

            domain_groups.insert(domain, ingress_domain_group_expand);
        }

        Ok(ArsExpand {
            namespace,
            domain_groups,
        })
    }
}
