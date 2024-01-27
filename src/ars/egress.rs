use fake::Dummy;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, Dummy)]
pub struct EgressSpec {
    pub services: HashMap<u32, ServiceSpec>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, Dummy)]
pub struct ServiceSpec {
    pub id: u32,
    pub service_name: String,
    pub service_identifier: ServiceIdentifier,
    pub default_codec_id: CodecId,
    pub methods: HashMap<String, MethodSpec>,
    /// Automatic mapping algorithm from http to service method
    pub method_mapper: MethodMapperId,
}

#[derive(
    Default, Debug, derive_more::Display, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq, Dummy,
)]
pub struct MethodMapperId(pub u8);

impl From<u8> for MethodMapperId {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Dummy)]
pub struct MethodSpec {
    pub method_name: String,
    pub inbound_spec: ParameterSpec,
    pub outbound_spec: ParameterSpec,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Dummy)]
pub struct ParameterSpec {
    pub entity_spec: EntitySchema,
    pub codec_id: Option<CodecId>,
    pub convert_option: ConvertOption,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Dummy)]
#[serde(rename_all = "snake_case")]
pub enum EntitySchema {
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
    Array { elem: Box<EntitySchema> },
    Object { fields: Vec<ObjectSchema> },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Dummy)]
pub struct ObjectSchema {
    pub field_id: u32,
    pub field_name: String,
    pub field_type: Box<EntitySchema>,
    pub convert_option: ConvertOption,
}

#[derive(Debug, Clone, Copy, derive_more::Display, serde::Serialize, serde::Deserialize, Dummy)]
pub struct ConverterId(pub u8);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Dummy)]
pub struct ConvertOption {
    pub default_value: Option<String>,
    pub http_loc: Option<HttpLoc>,
    pub from_http_with: Option<ConverterId>,
    pub to_http_with: Option<ConverterId>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Dummy)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "loc", content = "key")]
pub enum HttpLoc {
    Body(Option<String>),
    Header(Option<String>),
    Cookie(Option<String>),
    Query(Option<String>),
    Path(Option<String>),
    Plugin(Option<String>),
    Env(Option<String>),
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

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self)
            .map_err(|_| std::fmt::Error)
            .and_then(|s| write!(f, "{}", s))
    }
}

#[derive(
    Default, Debug, derive_more::Display, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq, Dummy,
)]
pub struct CodecId(pub u8);

impl From<u8> for CodecId {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, Dummy)]
pub struct ServiceIdentifier(String);
impl ServiceIdentifier {
    pub fn new() -> Self {
        ServiceIdentifier(String::new())
    }
}

impl Display for ServiceIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ServiceIdentifier {
    fn from(s: String) -> Self {
        ServiceIdentifier(s)
    }
}

impl From<&str> for ServiceIdentifier {
    fn from(s: &str) -> Self {
        ServiceIdentifier(s.to_owned())
    }
}

impl Deref for ServiceIdentifier {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
