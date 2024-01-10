use std::collections::HashMap;

use super::Namespace;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EgressRegistry {
    pub specifications: HashMap<Namespace, EgressSpec>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EgressSpec {
    pub namespace: Namespace,
    pub service_spec: HashMap<u32, ServiceSpec>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceSpec {
    pub id: u32,
    pub service_type: ServiceType,
    pub uniform_service_name: String,
    pub service_discover_identifier: String,
    pub handler_spec: HashMap<u32, MethodSpec>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MethodSpec {
    pub id: u32,
    pub method_name: String,
    pub inbound_spec: EntitySchema,
    pub outbound_spec: EntitySchema,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum EntitySchema {
    Bool(bool, Option<HttpParamType>),
    I8(i8, Option<HttpParamType>),
    I16(i16, Option<HttpParamType>),
    I32(i32, Option<HttpParamType>),
    I64(i64, Option<HttpParamType>),
    I128(i128, Option<HttpParamType>),
    U8(u8, Option<HttpParamType>),
    U16(u16, Option<HttpParamType>),
    U32(u32, Option<HttpParamType>),
    U64(u64, Option<HttpParamType>),
    U128(u128, Option<HttpParamType>),
    F32(f32, Option<HttpParamType>),
    F64(f64, Option<HttpParamType>),
    String(String, Option<HttpParamType>),
    Array(Vec<EntitySchema>, Option<HttpParamType>),
    Object(std::collections::HashMap<String, EntitySchema>, Option<HttpParamType>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum HttpParamType {
    Body,
}

impl Default for HttpParamType {
    fn default() -> Self {
        HttpParamType::Body
    }
}

// such as `http`, `grpc`, `websocket`, `domain-direct`, `custom`
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceType(u8);
