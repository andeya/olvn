use crate::error::*;
use crate::{
    ars::{CodecId, Entity},
    routing::Request,
};

#[derive(Debug, Clone, Copy)]

pub struct Codec {
    pub id: CodecId,
    pub encode: fn(Entity) -> Result<Vec<u8>, GwError>,
    pub decode: fn(&[u8]) -> Result<Entity, GwError>,
}

#[allow(dead_code)]
impl CodecId {
    pub const NO_CARE: CodecId = CodecId(0u8);
    pub const JSON: CodecId = CodecId(1u8);
    pub const FORM_URLENCODED: CodecId = CodecId(2u8);
    pub const FORM_DATA: CodecId = CodecId(3u8);
    pub const TEXT_HTML: CodecId = CodecId(4u8);
    pub const TEXT_PLAIN: CodecId = CodecId(5u8);
    pub const TEXT_PROTOBUF: CodecId = CodecId(6u8);
    pub const MIN_CUSTOM_NUM: CodecId = CodecId(100u8);

    pub(crate) fn from_request<F: Fn(&Request) -> Option<Self>>(req: &Request, main_mapping: Option<F>) -> Self {
        if let Some(main_mapping) = main_mapping {
            if let Some(et) = main_mapping(req) {
                return et;
            }
        }

        if let Some(h) = req.headers().get("Content-Type") {
            match h.as_bytes().splitn(2, |b| *b == b';').next() {
                Some(h) if h == b"application/json" => CodecId::JSON,
                Some(h) if h == b"application/x-www-form-urlencoded" => CodecId::FORM_URLENCODED,
                Some(h) if h == b"multipart/form-data" => CodecId::FORM_DATA,
                Some(h) if h == b"text/plain" => CodecId::TEXT_PLAIN,
                Some(h) if h == b"application/x-protobuf" => CodecId::TEXT_PROTOBUF,
                Some(h) if h == b"text/html" => CodecId::TEXT_HTML,
                _ => Self::NO_CARE,
            }
        } else {
            Self::NO_CARE
        }
    }

    pub(crate) fn to_content_type<F: Fn(Self) -> Option<&'static str>>(self, main_mapping: Option<F>) -> &'static str {
        if let Some(main_mapping) = main_mapping {
            if let Some(et) = main_mapping(self) {
                return et;
            }
        }
        match self {
            CodecId::JSON => "application/json",
            CodecId::FORM_URLENCODED => "application/x-www-form-urlencoded",
            CodecId::FORM_DATA => "multipart/form-data",
            CodecId::TEXT_PLAIN => "text/plain",
            CodecId::TEXT_PROTOBUF => "application/x-protobuf",
            CodecId::TEXT_HTML => "text/html",
            _ => "application/octet-stream",
        }
    }
}
