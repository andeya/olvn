use crate::{ars::ServiceIdentifier, error::*};
use http::uri::{Authority, InvalidUri, PathAndQuery, Uri};
use snafu::{IntoError, ResultExt};
use std::ops::Deref;

impl ServiceIdentifier {
    fn parse(&self) -> Result<Uri, InvalidUri> {
        self.deref().parse()
    }
}

pub trait ServiceEndpoint {
    fn authority(&self) -> Authority;
    fn path_and_query(&self) -> PathAndQuery;
}

#[derive(Debug, Clone)]
pub struct Discovery {
    pub scheme: String,
    pub discover: fn(Uri) -> Result<Box<dyn ServiceEndpoint>, GwError>,
}

pub struct DiscoveryCenter {
    discovery_list: Vec<Discovery>,
}

impl DiscoveryCenter {
    pub(crate) fn new() -> Self {
        Self { discovery_list: vec![] }
    }
    pub fn register(&mut self, discovery: Discovery) {
        self.discovery_list.push(discovery);
    }
    pub fn discover(&self, service_identifier: &ServiceIdentifier) -> Result<Box<dyn ServiceEndpoint>, GwError> {
        let uri = service_identifier
            .parse()
            .map_err(|e| AnyReason::new(e.to_string()))
            .context(InvalidServiceIdentifierSnafu {
                service_identifier: service_identifier.clone(),
            })
            .context(DiscoverySnafu)?;
        let scheme: String = uri
            .scheme()
            .ok_or_else(|| AnyReason::new("invalid scheme".to_string()))
            .context(InvalidServiceIdentifierSnafu {
                service_identifier: service_identifier.clone(),
            })
            .context(DiscoverySnafu)?
            .to_string();
        for discovery in &self.discovery_list {
            if discovery.scheme == scheme {
                return (discovery.discover)(uri);
            }
        }
        Err(DiscoverySnafu.into_error(NoDiscoverySnafu { scheme }.into_error(snafu::NoneError)))
    }
}
