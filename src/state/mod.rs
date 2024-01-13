use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

#[derive(Debug, Clone, Default)]
pub struct Context(Arc<GwState>);

#[derive(Debug, Clone, Default)]
pub struct GwState {
    pub host: String,
}

impl Deref for Context {
    type Target = GwState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::make_mut(&mut self.0)
    }
}

impl From<GwState> for Context {
    fn from(value: GwState) -> Self {
        Self(Arc::new(value))
    }
}
