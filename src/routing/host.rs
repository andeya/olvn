use axum::body::HttpBody;
use axum::http::Request;
use http::header::{HeaderMap, FORWARDED};

const X_FORWARDED_HOST_HEADER_KEY: &str = "X-Forwarded-Host";

/// Extractor that resolves the hostname of the request.
///
/// Hostname is resolved through the following, in order:
/// - `Forwarded` header
/// - `X-Forwarded-Host` header
/// - `Host` header
/// - request target / URI
///
/// Note that user agents can set `X-Forwarded-Host` and `Host` headers to arbitrary values so make
/// sure to validate them to avoid security issues.
pub(crate) type Host = String;

pub(crate) fn get_host_from_request<B>(req: &Request<B>) -> Option<Host>
where
    B: HttpBody<Data = bytes::Bytes> + Send + 'static,
    B::Error: Into<axum_core::BoxError>,
{
    let headers = req.headers();
    if let Some(host) = parse_forwarded(&headers) {
        return Some(host.to_owned());
    }

    if let Some(host) = headers
        .get(X_FORWARDED_HOST_HEADER_KEY)
        .and_then(|host| host.to_str().ok())
    {
        return Some(host.to_owned());
    }

    if let Some(host) = headers.get(http::header::HOST).and_then(|host| host.to_str().ok()) {
        return Some(host.to_owned());
    }

    if let Some(host) = req.uri().host() {
        return Some(host.to_owned());
    }

    None
}

#[allow(warnings)]
fn parse_forwarded(headers: &HeaderMap) -> Option<&str> {
    // if there are multiple `Forwarded` `HeaderMap::get` will return the first one
    let forwarded_values = headers.get(FORWARDED)?.to_str().ok()?;

    // get the first set of values
    let first_value = forwarded_values.split(',').nth(0)?;

    // find the value of the `host` field
    first_value.split(';').find_map(|pair| {
        let (key, value) = pair.split_once('=')?;
        key.trim()
            .eq_ignore_ascii_case("host")
            .then(|| value.trim().trim_matches('"'))
    })
}
