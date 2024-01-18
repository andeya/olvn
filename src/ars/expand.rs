use std::{collections::HashMap, sync::Arc};

use crate::error::*;

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
        let namespace = value.namespace;
        let mut domain_groups = HashMap::new();
        let services = value.egress.services;
        for (domain, domain_group) in value.ingress.domain_groups {
            let mut locations = Vec::new();

            for location in domain_group.locations {
                let id = location.id;
                let path = location.path;
                let method = location.method;
                let proxy_hide_headers = location.proxy_hide_headers;
                let proxy_pass_headers = location.proxy_pass_headers;
                let upstream_service = services
                    .get(&location.upstream_service_id)
                    .context(NoUpstreamSnafu {
                        id: location.upstream_service_id,
                    })
                    .context(ArsSnafu)?
                    .clone();

                let upstream_method = if let Some(upstream_method_id) = location.upstream_method_id {
                    Some(
                        upstream_service
                            .methods
                            .get(&upstream_method_id)
                            .context(NoUpstreamSnafu { id: upstream_method_id })
                            .context(ArsSnafu)?
                            .clone(),
                    )
                } else {
                    None
                };

                let ingress_location_spec = IngressLocationSpec {
                    id,
                    path,
                    method,
                    proxy_hide_headers,
                    proxy_pass_headers,
                    upstream_service: Arc::new(upstream_service),
                    upstream_method: Arc::new(upstream_method),
                };

                locations.push(ingress_location_spec);
            }

            let ingress_domain_group_expand = IngressDomainGroupExpand {
                domain_name: domain.clone(),
                locations,
            };

            domain_groups.insert(domain, ingress_domain_group_expand);
        }

        Ok(ArsExpand {
            namespace,
            domain_groups,
        })
    }
}
