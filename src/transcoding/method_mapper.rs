use axum_core::extract::Request;

use crate::ars::MethodMapperId;
use heck::AsUpperCamelCase;

use crate::error::*;

#[derive(Debug, Clone, Copy)]
pub struct MethodMapper {
    pub id: MethodMapperId,
    pub map_method: fn(&Request) -> Result<String, GwError>,
}

impl MethodMapperId {
    pub const UNKNOWN: MethodMapperId = MethodMapperId(0);
    pub const DEFAULT: MethodMapperId = MethodMapperId(1);
    pub const MIN_CUSTOM_ID: MethodMapperId = MethodMapperId(100);
    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }
}

impl MethodMapper {
    pub(crate) fn buildin() -> [Option<Self>; MethodMapperId::MIN_CUSTOM_ID.as_usize()] {
        let mut list = [None; MethodMapperId::MIN_CUSTOM_ID.as_usize()];
        list[MethodMapperId::DEFAULT.as_usize()] = Some(Self {
            id: MethodMapperId(0),
            map_method: |request| {
                Ok(format!(
                    "{}{}",
                    AsUpperCamelCase(request.method().to_string()),
                    AsUpperCamelCase(request.uri().path().rsplitn(2, "/").next().unwrap())
                ))
            },
        });
        list
    }
}
