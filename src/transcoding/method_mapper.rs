use axum_core::extract::Request;

use crate::ars::MethodMapperId;

use crate::error::*;

#[derive(Debug, Clone, Copy)]
pub struct MethodMapper {
    pub id: MethodMapperId,
    pub map_method: fn(&Request) -> Result<String, GwError>,
}
