use std::collections::HashMap;

use super::Namespace;

pub struct EgressRegistry {
    pub specifications: HashMap<Namespace, EgressSpecification>,
}

pub struct EgressSpecification {
    pub namespace: Namespace,
    pub service_definitions: HashMap<u32, UpstreamServer>,
}

pub struct ServiceDiscoveryMode(u8);

pub struct UpstreamServer {
    pub id: u32,
    pub uniform_service_name: String,
    pub service_discover_mode: ServiceDiscoveryMode,
    pub service_discover_identifier: String,
}

// such as `http`, `grpc`, `websocket`, `domain-direct`, `custom`
pub struct ProtocolType(u8);

// such as `json`, `protobuf`, `thrift`, `custom`
pub struct CodecType(u8);

pub struct ServiceDefinition {
    pub id: u32,
    pub uniform_service_name: String,
    pub api_definitions: HashMap<u32, ApiDefinition>,
    pub protocol_type: ProtocolType,
}

pub struct ApiDefinition {
    pub id: u32,
    pub uniform_service_name: String,
    pub method: String,
    pub path: String,
    pub request_definition: ApiEntity,
    pub response_definition: ApiEntity,
}

pub struct ApiEntity {
    pub codec_type: CodecType,
}
