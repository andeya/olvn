pub use snafu::prelude::*;

use crate::ars::CodecId;
use crate::ars::ConverterId;
use crate::ars::MethodMapperId;
use crate::ars::ServiceIdentifier;
use std::fmt::Display;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum GwError {
    #[snafu(display("ARS: {}", source))]
    Ars { source: ArsError },

    #[snafu(display("Codec: {}", source))]
    Codec { source: CodecError },

    #[snafu(display("Converter: {}", source))]
    Converter { source: ConverterError },

    #[snafu(display("MethodMapper: {}", source))]
    MethodMapper { source: MethodMapperError },

    #[snafu(display("JsonMarshal: {}", source))]
    JsonMarshal { source: serde_json::Error },

    #[snafu(display("JsonUnmarshal: {}", source))]
    JsonUnmarshal { source: serde_json::Error },

    #[snafu(display("Discovery: {}", source))]
    Discovery { source: DiscoveryError },

    #[snafu(display("Could not read file: {}", source))]
    Read { source: std::io::Error },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum ArsError {
    #[snafu(display("Could not found upstream service, id={}", id))]
    NoUpstreamService { id: u32 },
    #[snafu(display("Could not found upstream method, name={}", name))]
    NoUpstreamMethod { name: String },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum CodecError {
    #[snafu(display("Could not found codec, id={id}"))]
    NoCodec { id: CodecId },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum ConverterError {
    #[snafu(display("Could not found converter, id={id}"))]
    NoConverter { id: ConverterId },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum MethodMapperError {
    #[snafu(display("Could not found method mapper, id={id}"))]
    NoMethodMapper { id: MethodMapperId },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum DiscoveryError {
    #[snafu(display("Invalid service identifier {service_identifier}, reason={source}"))]
    InvalidServiceIdentifier {
        source: AnyReason,
        service_identifier: ServiceIdentifier,
    },
    #[snafu(display("Could not found discovery, scheme={scheme}"))]
    NoDiscovery { scheme: String },
}

#[derive(Debug)]
pub struct AnyReason(String);

impl AnyReason {
    pub fn new(source: String) -> Self {
        Self(source)
    }
}

impl Display for AnyReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for AnyReason {}
