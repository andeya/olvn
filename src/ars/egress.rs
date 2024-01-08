use std::collections::HashMap;

use super::Namespace;

pub struct EgressRegistry {
    specifications: HashMap<Namespace, EgressSpecification>,
}

pub struct EgressSpecification {
    namespace: Namespace,
    service_definitions: HashMap<u32, UpstreamServer>,
}

pub struct ServiceDiscoveryMode(u8);

pub struct UpstreamServer {
    id: u32,
    uniform_service_name: String,
    service_discover_mode: ServiceDiscoveryMode,
    service_discover_identifier: String,
}

// such as `http`, `grpc`, `websocket`, `domain-direct`, `custom`
pub struct ProtocolType(u8);

// such as `json`, `protobuf`, `thrift`, `custom`
pub struct CodecType(u8);

pub struct ServiceDefinition {
    id: u32,
    uniform_service_name: String,
    api_definitions: HashMap<u32, ApiDefinition>,
    protocol_type: ProtocolType,
}

pub struct ApiDefinition {
    id: u32,
    uniform_service_name: String,
    method: String,
    path: String,
    request_definition: ApiEntity,
    response_definition: ApiEntity,
}

pub struct ApiEntity {
    codec_type: CodecType,
}
