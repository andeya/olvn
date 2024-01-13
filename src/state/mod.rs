use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use tokio::time::Instant;
#[derive(Debug, Clone)]
pub struct GwContext(Arc<GwState>);

#[derive(Debug, Clone)]
pub struct GwState {
    pub host: String,
    pub uri: String,
    pub start_time: Instant,
}

impl Deref for GwContext {
    type Target = GwState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GwContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(Arc::as_ptr(&self.0) as *mut GwState) }
    }
}

impl From<GwState> for GwContext {
    fn from(value: GwState) -> Self {
        Self(Arc::new(value))
    }
}

impl GwContext {
    pub fn count(&self) -> usize {
        Arc::strong_count(&self.0)
    }
}
