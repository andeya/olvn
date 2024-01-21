use std::{collections::HashMap, sync::Arc};

use crate::error::*;
use crate::transcoding::Transcoding;

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
    pub(crate) transcoding: Arc<Transcoding>,
}

impl ArsExpand {
    pub fn try_from(value: Ars, transcoding: Arc<Transcoding>) -> Result<Self, GwError> {
        let namespace = value.namespace;
        let mut domain_groups = HashMap::new();
        let services = value.egress.services;

        for (domain, domain_group) in value.ingress.domain_groups {
            let mut handlers = Vec::new();

            for route_spec in domain_group.routes {
                let id = route_spec.id;
                let path = route_spec.path;
                let method = route_spec.method;
                let proxy_hide_headers = route_spec.proxy_hide_headers;
                let proxy_pass_headers = route_spec.proxy_pass_headers;
                let upstream_service = services
                    .get(&route_spec.upstream_service_id)
                    .context(NoUpstreamServiceSnafu {
                        id: route_spec.upstream_service_id,
                    })
                    .context(ArsSnafu)?
                    .clone();

                let upstream_method = if let Some(upstream_method_id) = route_spec.upstream_method_id {
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
                    transcoding: transcoding.clone(),
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
