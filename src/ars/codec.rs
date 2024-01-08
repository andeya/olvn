use std::collections::HashMap;

use lazy_static::lazy_static;

// such as `json`, `protobuf`, `thrift`, `custom`
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CodecType(u8);

pub trait Conversion: Send + Sync {
    fn source_codec_type(&'static self) -> CodecType;
    fn target_codec_type(&'static self) -> CodecType;
    fn convert(&'static self, bytes: Vec<u8>) -> Vec<u8>;
}

lazy_static! {
    pub(crate) static ref CODEC_LIBS: HashMap<(CodecType, CodecType), &'static dyn Conversion> = HashMap::new();
}
