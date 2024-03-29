//! # API Runtime Specification
//!
//! API Runtime Specification (ARS), all necessary data
//! required for normal request traffic in the gateway data plane.

mod egress;
mod ingress;
pub use egress::*;
use fake::Dummy;
pub use ingress::*;
use std::ops::Deref;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, Dummy)]
pub struct Ars {
    pub namespace: Namespace,
    pub ingress: ingress::IngressSpec,
    pub egress: egress::EgressSpec,
}

impl Ars {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize, Dummy)]
pub struct Namespace(String);

impl Deref for Namespace {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for Namespace {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Namespace {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

#[derive(Default, Debug, Clone, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd, Eq, PartialEq, Dummy)]
pub struct Domain(pub String);

impl From<String> for Domain {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Domain {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl Deref for Domain {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
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

#[cfg(test)]
mod tests {
    use crate::ars::*;
    use fake::{Fake, Faker};
    #[test]
    fn ars() {
        let ars: Ars = Faker.fake();
        println!("{}", serde_json::to_string_pretty(&ars).unwrap());
    }
}
