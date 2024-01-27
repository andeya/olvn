use crate::ars::{ConverterId, Entity};

use crate::error::*;

#[derive(Debug, Clone, Copy)]
pub struct Converter {
    pub id: ConverterId,
    pub convert: fn(Entity) -> Result<Entity, GwError>,
}

impl ConverterId {
    const UNKNOWN: ConverterId = ConverterId(0u8);
    pub const MIN_CUSTOM_ID: ConverterId = ConverterId(100u8);
    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }
}

impl Converter {
    pub(crate) fn buildin() -> [Option<Self>; ConverterId::MIN_CUSTOM_ID.as_usize()] {
        let list = [None; ConverterId::MIN_CUSTOM_ID.as_usize()];
        list
    }
}
