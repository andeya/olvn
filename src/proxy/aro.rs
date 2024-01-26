use std::{collections::HashMap, sync::Arc};

use crate::error::*;
use crate::transcoding::{Codec, Converter, MethodMapper, Transcoding};

use crate::ars::{
    Ars, ConvertOption, Domain, EntitySchema, HeaderName, HttpLoc, Method, MethodSpec, Namespace, ObjectSchema,
    ParameterSpec, ServiceIdentifier, ServiceSpec,
};

/// API Runtime Object, it is instance created in the runtime
/// according to the API runtime specification.
#[derive(Debug, Clone)]
pub struct Aro {
    pub(crate) namespace: Namespace,
    pub(crate) services: HashMap<u32, Arc<ResolvedServiceSpec>>,
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
    pub(crate) upstream_service: Arc<ResolvedServiceSpec>,
    /// If None, use automapper
    pub(crate) upstream_method: Option<Arc<ResolvedMethodSpec>>,
}

#[derive(Debug)]
pub struct ResolvedServiceSpec {
    pub id: u32,
    pub service_name: String,
    pub service_identifier: ServiceIdentifier,
    pub default_codec: Codec,
    pub methods: HashMap<String, Arc<ResolvedMethodSpec>>,
    /// Automatic mapping algorithm from http to service method
    pub method_mapper: MethodMapper,
}

impl ResolvedServiceSpec {
    fn try_from(value: ServiceSpec, transcoding: &Arc<Transcoding>) -> Result<Self, GwError> {
        Ok(Self {
            id: value.id,
            service_name: value.service_name,
            service_identifier: value.service_identifier,
            default_codec: transcoding
                .get_codec(value.default_codec_id)
                .context(NoCodecSnafu {
                    id: value.default_codec_id,
                })
                .context(CodecSnafu)?,
            methods: value
                .methods
                .into_iter()
                .map(|(_, method)| {
                    Ok((
                        method.method_name.clone(),
                        Arc::new(ResolvedMethodSpec::from_try(method, transcoding)?),
                    ))
                })
                .collect::<Result<HashMap<_, _>, _>>()?,
            method_mapper: transcoding
                .get_method_mapper(value.method_mapper)
                .context(NoMethodMapperSnafu {
                    id: value.method_mapper,
                })
                .context(MethodMapperSnafu)?,
        })
    }
}

#[derive(Debug)]
pub struct ResolvedMethodSpec {
    pub id: u32,
    pub method_name: String,
    pub inbound_spec: ResolvedParameterSpec,
    pub outbound_spec: ResolvedParameterSpec,
}

impl ResolvedMethodSpec {
    fn from_try(value: MethodSpec, transcoding: &Arc<Transcoding>) -> Result<Self, GwError> {
        Ok(Self {
            id: value.id,
            method_name: value.method_name,
            inbound_spec: ResolvedParameterSpec::try_from(value.inbound_spec, transcoding)?,
            outbound_spec: ResolvedParameterSpec::try_from(value.outbound_spec, transcoding)?,
        })
    }
}

#[derive(Debug)]
pub struct ResolvedParameterSpec {
    pub entity_spec: ResolvedEntitySchema,
    pub codec: Option<Codec>,
    pub convert_option: ResolvedConvertOption,
}

impl ResolvedParameterSpec {
    fn try_from(value: ParameterSpec, transcoding: &Arc<Transcoding>) -> Result<Self, GwError> {
        Ok(Self {
            entity_spec: ResolvedEntitySchema::try_from(value.entity_spec, transcoding)?,
            codec: if let Some(id) = value.codec_id {
                Some(
                    transcoding
                        .get_codec(id)
                        .context(NoCodecSnafu { id })
                        .context(CodecSnafu)?,
                )
            } else {
                None
            },
            convert_option: ResolvedConvertOption::try_from(value.convert_option, transcoding)?,
        })
    }
}

#[derive(Debug)]
pub enum ResolvedEntitySchema {
    Bool,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    String,
    Array { elem: Box<ResolvedEntitySchema> },
    Object { fields: Vec<ResolvedObjectSchema> },
}

impl ResolvedEntitySchema {
    fn try_from(value: EntitySchema, transcoding: &Arc<Transcoding>) -> Result<Self, GwError> {
        Ok(match value {
            EntitySchema::Bool => Self::Bool,
            EntitySchema::I8 => Self::I8,
            EntitySchema::I16 => Self::I16,
            EntitySchema::I32 => Self::I32,
            EntitySchema::I64 => Self::I64,
            EntitySchema::I128 => Self::I128,
            EntitySchema::U8 => Self::U8,
            EntitySchema::U16 => Self::U16,
            EntitySchema::U32 => Self::U32,
            EntitySchema::U64 => Self::U64,
            EntitySchema::U128 => Self::U128,
            EntitySchema::F32 => Self::F32,
            EntitySchema::F64 => Self::F64,
            EntitySchema::String => Self::String,
            EntitySchema::Array { elem } => Self::Array {
                elem: Box::new(ResolvedEntitySchema::try_from(*elem, transcoding)?),
            },
            EntitySchema::Object { fields } => Self::Object {
                fields: fields
                    .into_iter()
                    .map(|field| ResolvedObjectSchema::try_from(field, transcoding))
                    .collect::<Result<Vec<_>, _>>()?,
            },
        })
    }
}

#[derive(Debug)]
pub struct ResolvedObjectSchema {
    pub field_id: u32,
    pub field_name: String,
    pub field_type: Box<ResolvedEntitySchema>,
    pub convert_option: ResolvedConvertOption,
}

impl ResolvedObjectSchema {
    fn try_from(value: ObjectSchema, transcoding: &Arc<Transcoding>) -> Result<Self, GwError> {
        Ok(Self {
            field_id: value.field_id,
            field_name: value.field_name,
            field_type: Box::new(ResolvedEntitySchema::try_from(*value.field_type, transcoding)?),
            convert_option: ResolvedConvertOption::try_from(value.convert_option, transcoding)?,
        })
    }
}

#[derive(Debug)]
pub struct ResolvedConvertOption {
    pub default_value: Option<String>,
    pub http_loc: Option<HttpLoc>,
    pub from_http_with: Option<Converter>,
    pub to_http_with: Option<Converter>,
}

impl ResolvedConvertOption {
    fn try_from(value: ConvertOption, transcoding: &Arc<Transcoding>) -> Result<Self, GwError> {
        Ok(Self {
            default_value: value.default_value,
            http_loc: value.http_loc,
            from_http_with: if let Some(id) = value.from_http_with {
                Some(
                    transcoding
                        .get_converter(id)
                        .context(NoConverterSnafu { id })
                        .context(ConverterSnafu)?,
                )
            } else {
                None
            },
            to_http_with: if let Some(id) = value.from_http_with {
                Some(
                    transcoding
                        .get_converter(id)
                        .context(NoConverterSnafu { id })
                        .context(ConverterSnafu)?,
                )
            } else {
                None
            },
        })
    }
}

impl Aro {
    pub fn try_from(value: Ars, transcoding: Arc<Transcoding>) -> Result<Self, GwError> {
        let namespace = value.namespace;
        let mut domain_groups = HashMap::new();
        let mut services: HashMap<u32, Arc<ResolvedServiceSpec>> = HashMap::new();
        for (id, service) in value.egress.services {
            services.insert(id, Arc::new(ResolvedServiceSpec::try_from(service, &transcoding)?));
        }
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
                            .values()
                            .find(|method| method.id == upstream_method_id)
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
                });

                handlers.push(handler);
            }

            let route_mapper = RouteMapper {
                domain_name: domain.clone(),
                handlers,
            };

            domain_groups.insert(domain, route_mapper);
        }

        Ok(Aro {
            namespace,
            services,
            domain_groups,
        })
    }
}
