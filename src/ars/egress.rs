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
#[serde(rename_all = "snake_case")]
pub enum EntitySchema {
    Bool {
        http_param: Option<HttpParam>,
    },
    I8 {
        http_param: Option<HttpParam>,
    },
    I16 {
        http_param: Option<HttpParam>,
    },
    I32 {
        http_param: Option<HttpParam>,
    },
    I64 {
        http_param: Option<HttpParam>,
    },
    I128 {
        http_param: Option<HttpParam>,
    },
    U8 {
        http_param: Option<HttpParam>,
    },
    U16 {
        http_param: Option<HttpParam>,
    },
    U32 {
        http_param: Option<HttpParam>,
    },
    U64 {
        http_param: Option<HttpParam>,
    },
    U128 {
        http_param: Option<HttpParam>,
    },
    F32 {
        http_param: Option<HttpParam>,
    },
    F64 {
        http_param: Option<HttpParam>,
    },
    String {
        http_param: Option<HttpParam>,
    },
    Array {
        elem_type: Box<EntitySchema>,
        http_param: Option<HttpParam>,
    },
    Object {
        fields: Vec<ObjectSchema>,
        http_param: Option<HttpParam>,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ObjectSchema {
    pub key: String,
    pub value_type: Box<EntitySchema>,
    pub http_param: Option<HttpParam>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HttpParam {
    Body(Vec<String>),
    Header(Vec<String>),
    Cookie(Vec<String>),
    Query(Vec<String>),
    Path(Vec<String>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceDiscoverType {
    Http,
    Grpc,
    Websocket,
    DomainDirect,
    Custom,
}

impl Default for ServiceDiscoverType {
    fn default() -> Self {
        Self::Http
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Entity {
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
    String(String),
    Array(Vec<EntitySchema>),
    Object(std::collections::HashMap<String, EntitySchema>),
}

// such as `http`, `grpc`, `websocket`, `domain-direct`, `custom`
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceType(u8);

#[test]
fn entity_schema() {
    let schema = EntitySchema::Object {
        fields: vec![ObjectSchema {
            key: "a".to_string(),
            value_type: Box::new(EntitySchema::String {
                http_param: Some(HttpParam::Body(vec!["a".to_owned()])),
            }),
            http_param: None,
        }],
        http_param: Some(HttpParam::Header(vec!["X-Olvn-Identifier".to_owned()])),
    };
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
