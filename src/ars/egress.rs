use std::collections::HashMap;

use super::{codec::CodecType, protocol::ProtocolType, Namespace};

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EgressRegistry {
    pub specifications: HashMap<Namespace, EgressSpecification>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EgressSpecification {
    pub namespace: Namespace,
    pub service_definitions: HashMap<u32, UpstreamServer>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceDiscoveryMode(u8);

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpstreamServer {
    pub id: u32,
    pub uniform_service_name: String,
    pub service_discover_mode: ServiceDiscoveryMode,
    pub service_discover_identifier: String,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceDefinition {
    pub id: u32,
    pub uniform_service_name: String,
    pub api_definitions: HashMap<u32, ApiDefinition>,
    pub protocol_type: ProtocolType,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiDefinition {
    pub id: u32,
    pub uniform_service_name: String,
    pub method: String,
    pub path: String,
    pub request_definition: ApiEntity,
    pub response_definition: ApiEntity,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiEntity {
    pub codec_type: CodecType,
}
