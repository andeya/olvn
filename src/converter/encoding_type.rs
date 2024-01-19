use crate::{ars::EncodingType, routing::Request};

#[allow(dead_code)]
impl EncodingType {
    pub const NO_CARE: EncodingType = EncodingType(0u8);
    pub const JSON: EncodingType = EncodingType(1u8);
    pub const FORM_URLENCODED: EncodingType = EncodingType(2u8);
    pub const FORM_DATA: EncodingType = EncodingType(3u8);
    pub const TEXT_HTML: EncodingType = EncodingType(4u8);
    pub const TEXT_PLAIN: EncodingType = EncodingType(5u8);
    pub const TEXT_PROTOBUF: EncodingType = EncodingType(6u8);
    pub const MIN_CUSTOM_NUM: EncodingType = EncodingType(100u8);

    pub(crate) fn from_request<F: Fn(&Request) -> Option<Self>>(req: &Request, main_mapping: Option<F>) -> Self {
        if let Some(main_mapping) = main_mapping {
            if let Some(et) = main_mapping(req) {
                return et;
            }
        }

        if let Some(h) = req.headers().get("Content-Type") {
            match h.as_bytes().splitn(2, |b| *b == b';').next() {
                Some(h) if h == b"application/json" => EncodingType::JSON,
                Some(h) if h == b"application/x-www-form-urlencoded" => EncodingType::FORM_URLENCODED,
                Some(h) if h == b"multipart/form-data" => EncodingType::FORM_DATA,
                Some(h) if h == b"text/plain" => EncodingType::TEXT_PLAIN,
                Some(h) if h == b"application/x-protobuf" => EncodingType::TEXT_PROTOBUF,
                Some(h) if h == b"text/html" => EncodingType::TEXT_HTML,
                _ => Self::NO_CARE,
            }
        } else {
            Self::NO_CARE
        }
    }
}
