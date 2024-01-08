//! # API Runtime Specification
//!
//! API Runtime Specification (ARS), all necessary data
//! required for normal request traffic in the gateway data plane.

pub mod egress;
pub mod ingress;

#[derive(Debug, Clone)]
pub struct Namespace(String);

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Domain(pub String);
impl Default for Domain {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl PartialEq<String> for Domain {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl Domain {
    pub const fn new() -> Self {
        Self(String::new())
    }
}
