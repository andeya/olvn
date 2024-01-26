pub use snafu::prelude::*;

use crate::ars::CodecId;
use crate::ars::ConverterId;
use crate::ars::MethodMapperId;

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
