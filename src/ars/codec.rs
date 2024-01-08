// such as `json`, `protobuf`, `thrift`, `custom`
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CodecType(u8);
