use axum::Router;

use crate::ars::Domain;

pub(crate) const FALLBACK_NO_DOMAIN: Domain = Domain::new();

#[derive(Debug, Clone)]
pub(crate) struct DomainRouter {
    pub(crate) domain: Domain, // $FALLBACK_NO_DOMAIN is fallback
    pub(crate) router: Router,
}

impl Default for DomainRouter {
    fn default() -> Self {
        Self::new(Default::default(), Router::new())
    }
}

impl DomainRouter {
    pub fn new(domain: Domain, router: Router) -> Self {
        Self { domain, router }
    }
}
