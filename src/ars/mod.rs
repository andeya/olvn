//! # API Runtime Specification
//!
//! The API Runtime Specification (ARS) is a specification for the runtime
//! environment of an API, containing all the necessary data required for a
//! normal flow of an API request in the data plane of the gateway.

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
