use http::HeaderValue;

use crate::ars::{Codec, Entity, IngressLocationSpec, ServiceIdentifier};
use crate::routing::{IntoResponse, Request, Response};
mod discovery;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceEndpoint {
    pub identifier: ServiceIdentifier,
}

impl IngressLocationSpec {
    #[inline]
    pub(crate) fn reverse_proxy(&self, req: Request) -> Response {
        println!("req: {:?}", req);
        format!("{:?}", self).into_response()
    }
    #[inline]
    fn request_to_entity(&self, req: &Request) -> Option<Entity> {
        if let Some(upstream_method) = &self.upstream_method {
            let codec = upstream_method
                .inbound_codec
                .clone()
                .unwrap_or(self.upstream_service.default_codec);
        } else {
        }
        unimplemented!()
    }
    #[inline]
    fn entity_to_response<T>(&self, resp: T) -> Response {
        unimplemented!()
    }
}

#[allow(dead_code)]
impl Codec {
    const NO_CARE: Codec = Codec(0u8);
    const JSON: Codec = Codec(1u8);
    const FORM_URLENCODED: Codec = Codec(2u8);
    const FORM_DATA: Codec = Codec(3u8);
    const TEXT_HTML: Codec = Codec(4u8);
    const TEXT_PLAIN: Codec = Codec(5u8);
    const TEXT_PROTOBUF: Codec = Codec(6u8);
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
}

impl From<&HeaderValue> for Codec {
    fn from(h: &HeaderValue) -> Self {
        match h {
            h if Codec::is_json(h) => Codec::JSON,
            h if Codec::is_form_data(h) => Codec::FORM_DATA,
            h if Codec::is_form_urlencoded(h) => Codec::FORM_URLENCODED,
            h if Codec::is_html(h) => Codec::TEXT_HTML,
            h if Codec::is_plain(h) => Codec::TEXT_PLAIN,
            h if Codec::is_protobuf(h) => Codec::TEXT_PROTOBUF,
            _ => Codec::NO_CARE,
        }
    }
}

impl From<&Request> for Codec {
    fn from(value: &Request) -> Self {
        if let Some(h) = value.headers().get("Content-Type") {
            h.into()
        } else {
            Codec::NO_CARE
        }
    }
}
