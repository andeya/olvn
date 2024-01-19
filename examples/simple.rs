use std::collections::HashMap;
use std::sync::Arc;

use olvn::ars::*;
use olvn::*;

#[olvn::main]
async fn main() {
    static SERVE: Serve = Serve::new();
    let mut ars = Ars::new();
    ars.namespace = "default".into();
    ars.egress.services.insert(
        1,
        Arc::new(ServiceSpec {
            id: 1,
            service_name: "p.s.m".to_owned(),
            service_identifier: "p.s.m".into(),
            methods: HashMap::new(),
            default_encoding_type: 0u8.into(),
        }),
    );
    ars.ingress.domain_groups.insert(
        "www.github.com".into(),
        IngressDomainGroup {
            domain_name: "www.github.com".into(),
            locations: vec![IngressLocation {
                id: 1,
                path: "/".to_owned(),
                method: Method::Any,
                proxy_hide_headers: vec![],
                proxy_pass_headers: vec![],
                upstream_service_id: 1,
                upstream_method_id: None,
            }],
        },
    );
    SERVE.hot_update_ars(ars).unwrap();
    // run our app with hyper, listening globally on port 3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    SERVE.serve(listener).await.unwrap();
    // SERVE.serve_with_graceful_shutdown(listener, signal()).await.unwrap();
}
