mod encoding_type;

use crate::error::*;

use crate::{
    ars::EncodingType,
    error::GwError,
    routing::{Request, Response},
};

pub type RequestConverter = fn(&Request) -> Result<Vec<u8>, GwError>;

pub type ResponseConverter = fn(&[u8]) -> Result<Response, GwError>;

pub struct ConverterIndex {
    request_converters: [[Option<RequestConverter>; 255]; 255],
    response_converters: [[Option<ResponseConverter>; 255]; 255],
}

impl Default for ConverterIndex {
    fn default() -> Self {
        Self {
            request_converters: [[None; 255]; 255],
            response_converters: [[None; 255]; 255],
        }
    }
}

impl ConverterIndex {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register_request_converter(
        &mut self,
        from: EncodingType,
        to: EncodingType,
        converter: Option<RequestConverter>,
    ) {
        self.request_converters[from.0 as usize][to.0 as usize] = converter;
    }
    pub fn register_response_converter(
        &mut self,
        from: EncodingType,
        to: EncodingType,
        converter: Option<ResponseConverter>,
    ) {
        self.response_converters[from.0 as usize][to.0 as usize] = converter;
    }
    pub fn convert_request(&self, from: EncodingType, to: EncodingType, req: &Request) -> Result<Vec<u8>, GwError> {
        let converter = self.request_converters[from.0 as usize][to.0 as usize];
        converter
            .context(NoConverterSnafu { from, to })
            .context(ConverterSnafu)?(req)
    }
    pub fn convert_response(&self, from: EncodingType, to: EncodingType, resp: &[u8]) -> Result<Response, GwError> {
        let converter = self.response_converters[from.0 as usize][to.0 as usize];
        converter
            .context(NoConverterSnafu { from, to })
            .context(ConverterSnafu)?(resp)
    }
}
