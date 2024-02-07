use crate::error::*;
use axum_core::extract::Request;
use http::Response;

mod _http;

pub trait ServiceEndpoint {
    fn reverse_proxy(&self, req: Request) -> Response<axum_core::body::Body>;
}

#[derive(Debug, Clone, Copy, derive_more::Display)]
pub struct Protocol(u8);

impl Protocol {
    const HTTP: Protocol = Protocol(0u8);
    pub const MIN_CUSTOM_ID: Protocol = Protocol(100u8);
}

pub struct ServiceEndpointFactory {
    endpoint_factory: [Option<fn(addr: String) -> Result<Box<dyn ServiceEndpoint>, GwError>>; u8::MAX as usize],
}
impl Default for ServiceEndpointFactory {
    fn default() -> Self {
        let mut s = Self::new();
        s.register(Protocol::HTTP, _http::new_http_endpoint);
        s
    }
}
impl ServiceEndpointFactory {
    fn new() -> Self {
        Self {
            endpoint_factory: [None; u8::MAX as usize],
        }
    }

    pub fn register(
        &mut self,
        protocol: Protocol,
        factory: fn(addr: String) -> Result<Box<dyn ServiceEndpoint>, GwError>,
    ) {
        self.endpoint_factory[protocol.0 as usize] = Some(factory);
    }

    pub fn create(&self, protocol: Protocol, addr: String) -> Result<Box<dyn ServiceEndpoint>, GwError> {
        self.endpoint_factory[protocol.0 as usize]
            .as_ref()
            .ok_or(GwError::Protocol {
                source: ProtocolError::NoProtocol { protocol },
            })?(addr)
    }
}
