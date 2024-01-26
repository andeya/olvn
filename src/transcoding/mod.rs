mod codec;
mod convert;
mod method_mapper;
use axum_core::extract::Request;
use heck::AsUpperCamelCase;

pub use self::codec::Codec;
pub use self::convert::Converter;
pub use self::method_mapper::MethodMapper;
use crate::ars::{CodecId, ConverterId, Entity, MethodMapperId};
use crate::error::*;

#[derive(Debug, Clone)]
pub struct Transcoding {
    converter_store: [Option<Converter>; 255],
    codec_store: [Option<Codec>; 255],
    method_mapper_store: [Option<MethodMapper>; 255],
}

impl Default for Transcoding {
    fn default() -> Self {
        let mut v = Self::new();
        v.converter_store[0] = Some(Converter {
            id: ConverterId(0),
            convert: |entity| Ok(entity),
        });
        v.codec_store[0] = Some(Codec {
            id: CodecId(0),
            encode: |entity| serde_json::to_vec(&entity).context(JsonMarshalSnafu),
            decode: |bytes| serde_json::from_slice(bytes).context(JsonUnmarshalSnafu),
        });
        v.method_mapper_store[0] = Some(MethodMapper {
            id: MethodMapperId(0),
            map_method: |request| {
                Ok(format!(
                    "{}{}",
                    AsUpperCamelCase(request.method().to_string()),
                    AsUpperCamelCase(request.uri().path().rsplitn(2, "/").next().unwrap())
                ))
            },
        });
        v
    }
}

impl Transcoding {
    pub const fn new() -> Self {
        Self {
            converter_store: [None; 255],
            codec_store: [None; 255],
            method_mapper_store: [None; 255],
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
    pub fn get_converter(&self, converter_id: ConverterId) -> Option<Converter> {
        self.converter_store[converter_id.0 as usize].clone()
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
    pub fn get_codec(&self, codec_id: CodecId) -> Option<Codec> {
        self.codec_store[codec_id.0 as usize].clone()
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
            self.method_mapper_store[method_mapper.id.0 as usize] = Some(method_mapper);
        }
        self
    }
    pub fn unregister_method_mappers(mut self, method_mapper_ids: Vec<MethodMapperId>) -> Self {
        for method_mapper_id in method_mapper_ids {
            self.method_mapper_store[method_mapper_id.0 as usize] = None;
        }
        self
    }
    pub fn get_method_mapper(&self, method_mapper_id: MethodMapperId) -> Option<MethodMapper> {
        self.method_mapper_store[method_mapper_id.0 as usize].clone()
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
