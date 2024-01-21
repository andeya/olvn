use crate::ars::{ConverterId, Entity};

use crate::error::*;

#[derive(Debug, Clone, Copy)]
pub struct Converter {
    pub id: ConverterId,
    pub convert: fn(Entity) -> Result<Entity, GwError>,
}
