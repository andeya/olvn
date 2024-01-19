use http::HeaderValue;

use crate::{ars::EncodingType, routing::Request};

#[allow(dead_code)]
impl EncodingType {
    const NO_CARE: EncodingType = EncodingType(0u8);
    const JSON: EncodingType = EncodingType(1u8);
    const FORM_URLENCODED: EncodingType = EncodingType(2u8);
    const FORM_DATA: EncodingType = EncodingType(3u8);
    const TEXT_HTML: EncodingType = EncodingType(4u8);
    const TEXT_PLAIN: EncodingType = EncodingType(5u8);
    const TEXT_PROTOBUF: EncodingType = EncodingType(6u8);
    #[inline]
    fn is(x: &'static [u8], h: &HeaderValue) -> bool {
        let b = h.as_bytes();
        b == x || b.starts_with(x)
    }
    fn is_json(h: &HeaderValue) -> bool {
        Self::is(b"application/json", h)
    }
    fn is_form_urlencoded(h: &HeaderValue) -> bool {
        Self::is(b"application/x-www-form-urlencoded", h)
    }
    fn is_form_data(h: &HeaderValue) -> bool {
        Self::is(b"multipart/form-data", h)
    }
    fn is_html(h: &HeaderValue) -> bool {
        Self::is(b"text/html", h)
    }
    fn is_plain(h: &HeaderValue) -> bool {
        Self::is(b"text/plain", h)
    }
    fn is_protobuf(h: &HeaderValue) -> bool {
        Self::is(b"application/x-protobuf", h)
    }

    pub(crate) fn from_request<F: Fn(&Request) -> Self>(req: &Request, fallback: F) -> Self {
        if let Some(h) = req.headers().get("Content-Type") {
            match h {
                h if EncodingType::is_json(h) => EncodingType::JSON,
                h if EncodingType::is_form_urlencoded(h) => EncodingType::FORM_URLENCODED,
                h if EncodingType::is_form_data(h) => EncodingType::FORM_DATA,
                h if EncodingType::is_plain(h) => EncodingType::TEXT_PLAIN,
                h if EncodingType::is_protobuf(h) => EncodingType::TEXT_PROTOBUF,
                h if EncodingType::is_html(h) => EncodingType::TEXT_HTML,
                _ => fallback(req),
            }
        } else {
            fallback(req)
        }
    }
}
