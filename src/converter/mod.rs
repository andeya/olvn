mod encoding_type;

use crate::{
    ars::EncodingType,
    routing::{Request, Response},
};

pub type RequestConverter = fn(&Request) -> Vec<u8>;

pub type ResponseConverter = fn(&[u8]) -> Response;

pub struct ConverterIndex {
    request_converters: [[RequestConverter; 255]; 255],
    response_converters: [[ResponseConverter; 255]; 255],
}
fn request_converter(_req: &Request) -> Vec<u8> {
    todo!()
}
fn response_converter(_: &[u8]) -> Response {
    todo!()
}
impl Default for ConverterIndex {
    fn default() -> Self {
        Self {
            request_converters: [[request_converter; 255]; 255],
            response_converters: [[response_converter; 255]; 255],
        }
    }
}

impl ConverterIndex {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register_request_converter(&mut self, from: EncodingType, to: EncodingType, converter: RequestConverter) {
        self.request_converters[from.0 as usize][to.0 as usize] = converter;
    }
    pub fn register_response_converter(&mut self, from: EncodingType, to: EncodingType, converter: ResponseConverter) {
        self.response_converters[from.0 as usize][to.0 as usize] = converter;
    }
    pub fn convert_request(&self, from: EncodingType, to: EncodingType, req: &Request) -> Vec<u8> {
        let converter = self.request_converters[from.0 as usize][to.0 as usize];
        converter(req)
    }
    pub fn convert_response(&self, from: EncodingType, to: EncodingType, resp: &[u8]) -> Response {
        let converter = self.response_converters[from.0 as usize][to.0 as usize];
        converter(resp)
    }
}
