mod codec;
mod convert;

use self::codec::Codec;
use self::convert::Converter;
use crate::ars::{CodecId, ConverterId, Entity};
use crate::error::*;

#[derive(Debug, Clone)]
pub struct Transcoding {
    converter_store: [Option<Converter>; 255],
    codec_store: [Option<Codec>; 255],
}

impl Default for Transcoding {
    fn default() -> Self {
        Self::new()
    }
}

impl Transcoding {
    pub const fn new() -> Self {
        Self {
            converter_store: [None; 255],
            codec_store: [None; 255],
        }
    }
    pub fn register_converters(mut self, converters: Vec<Converter>) -> Self {
        for converter in converters {
            self.converter_store[converter.id.0 as usize] = Some(converter);
        }
        self
    }
    pub fn unregister_converters(mut self, converter_ids: Vec<ConverterId>) -> Self {
        for converter_id in converter_ids {
            self.converter_store[converter_id.0 as usize] = None;
        }
        self
    }
    pub fn register_codecs(mut self, codecs: Vec<Codec>) -> Self {
        for codec in codecs {
            self.codec_store[codec.id.0 as usize] = Some(codec);
        }
        self
    }
    pub fn unregister_codecs(mut self, codec_ids: Vec<CodecId>) -> Self {
        for codec_id in codec_ids {
            self.codec_store[codec_id.0 as usize] = None;
        }
        self
    }
    pub fn get_converter(&self, converter_id: ConverterId) -> Option<Converter> {
        self.converter_store[converter_id.0 as usize].clone()
    }
    pub fn get_codec(&self, codec_id: CodecId) -> Option<Codec> {
        self.codec_store[codec_id.0 as usize].clone()
    }
    pub fn convert(&self, converter_id: ConverterId, entity: Entity) -> Result<Entity, GwError> {
        if let Some(converter) = self.get_converter(converter_id) {
            (converter.convert)(entity)
        } else {
            Err(GwError::Converter {
                source: ConverterError::NoConverter { id: converter_id },
            })
        }
    }
    pub fn encode(&self, codec_id: CodecId, entity: Entity) -> Result<Vec<u8>, GwError> {
        if let Some(codec) = self.get_codec(codec_id) {
            (codec.encode)(entity)
        } else {
            Err(GwError::Codec {
                source: CodecError::NoCodec { id: codec_id },
            })
        }
    }
    pub fn decode(&self, codec_id: CodecId, bytes: &[u8]) -> Result<Entity, GwError> {
        if let Some(codec) = self.get_codec(codec_id) {
            (codec.decode)(bytes)
        } else {
            Err(GwError::Codec {
                source: CodecError::NoCodec { id: codec_id },
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Transcoding;

    #[test]
    fn test_transcoding() {
        println!("{:?}", Transcoding::default())
    }
}
