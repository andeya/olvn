// such as `http`, `grpc`, `websocket`, `domain-direct`, `custom`
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProtocolType(u8);
