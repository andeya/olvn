pub use snafu::prelude::*;

use crate::ars::EncodingType;
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum GwError {
    #[snafu(display("ARS: {}", source))]
    Ars { source: ArsError },

    #[snafu(display("Converter: {}", source))]
    Converter { source: ConverterError },

    #[snafu(display("Could not read file: {}", source))]
    Read { source: std::io::Error },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum ArsError {
    #[snafu(display("Could not found upstream service, id={}", id))]
    NoUpstreamService { id: u32 },
    #[snafu(display("Could not found upstream method, id={}", id))]
    NoUpstreamMethod { id: u32 },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum ConverterError {
    #[snafu(display("Could not found converter, from={from}, to={to}"))]
    NoConverter { from: EncodingType, to: EncodingType },
}
