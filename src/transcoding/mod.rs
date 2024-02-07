mod codec;
mod convert;
mod method_mapper;

use axum_core::extract::Request;

pub use self::codec::Codec;
pub use self::convert::Converter;
pub use self::method_mapper::MethodMapper;
use crate::ars::{CodecId, ConverterId, Entity, MethodMapperId};
use crate::error::*;

#[derive(Debug, Clone)]
pub struct Transcoding {
    converter_store: [Option<Converter>; u8::MAX as usize],
    codec_store: [Option<Codec>; u8::MAX as usize],
    method_mapper_store: [Option<MethodMapper>; u8::MAX as usize],
}

impl Default for Transcoding {
    fn default() -> Self {
        let mut v = Self::new();
        v.register_default();
        v
    }
}

impl Transcoding {
    pub const fn new() -> Self {
        Self {
            converter_store: [None; u8::MAX as usize],
            codec_store: [None; u8::MAX as usize],
            method_mapper_store: [None; u8::MAX as usize],
        }
    }
    pub fn register_default(&mut self) {
        self.codec_store[..CodecId::MIN_CUSTOM_ID.as_usize()].copy_from_slice(&Codec::buildin());
        self.converter_store[..ConverterId::MIN_CUSTOM_ID.as_usize()].copy_from_slice(&Converter::buildin());
        self.method_mapper_store[..MethodMapperId::MIN_CUSTOM_ID.as_usize()].copy_from_slice(&MethodMapper::buildin());
    }
    pub fn register_converters(mut self, converters: Vec<Converter>) -> Self {
        for converter in converters {
            self.converter_store[converter.id.as_usize()] = Some(converter);
        }
        self
    }
    pub fn unregister_converters(mut self, converter_ids: Vec<ConverterId>) -> Self {
        for converter_id in converter_ids {
            self.converter_store[converter_id.as_usize()] = None;
        }
        self
    }
    pub fn get_converter(&self, converter_id: ConverterId) -> Option<Converter> {
        self.converter_store[converter_id.as_usize()].clone()
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

    pub fn register_codecs(mut self, codecs: Vec<Codec>) -> Self {
        for codec in codecs {
            self.codec_store[codec.id.as_usize()] = Some(codec);
        }
        self
    }
    pub fn unregister_codecs(mut self, codec_ids: Vec<CodecId>) -> Self {
        for codec_id in codec_ids {
            self.codec_store[codec_id.as_usize()] = None;
        }
        self
    }
    pub fn get_codec(&self, codec_id: CodecId) -> Option<Codec> {
        self.codec_store[codec_id.as_usize()].clone()
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

    pub fn register_method_mappers(mut self, method_mappers: Vec<MethodMapper>) -> Self {
        for method_mapper in method_mappers {
            self.method_mapper_store[method_mapper.id.as_usize()] = Some(method_mapper);
        }
        self
    }
    pub fn unregister_method_mappers(mut self, method_mapper_ids: Vec<MethodMapperId>) -> Self {
        for method_mapper_id in method_mapper_ids {
            self.method_mapper_store[method_mapper_id.as_usize()] = None;
        }
        self
    }
    pub fn get_method_mapper(&self, method_mapper_id: MethodMapperId) -> Option<MethodMapper> {
        self.method_mapper_store[method_mapper_id.as_usize()].clone()
    }
    pub fn map_method(&self, method_mapper_id: MethodMapperId, request: &Request) -> Result<String, GwError> {
        if let Some(method_mapper) = self.get_method_mapper(method_mapper_id) {
            (method_mapper.map_method)(request)
        } else {
            Err(GwError::MethodMapper {
                source: MethodMapperError::NoMethodMapper { id: method_mapper_id },
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
