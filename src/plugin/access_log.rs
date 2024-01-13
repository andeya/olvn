use axum::response::Response;
use axum_core::body::Body;
use tokio::time::Instant;

use super::{Plugin, RequestCtx, ResponseCtx};

#[derive(Debug, Clone)]
pub struct AccessLog;

impl Plugin for AccessLog {
    async fn pre(req_ctx: RequestCtx<'_>) -> Option<Response<Body>> {
        let uri = req_ctx.uri().to_string();
        let state = req_ctx.state();
        println!(
            "[AccessLog-PRE] Handling a request to {}{}, state={:?}",
            state.host, uri, state
        );
        None
    }

    async fn post(resp_ctx: ResponseCtx<'_>) {
        let state = resp_ctx.state();
        let cost_time = Instant::now() - state.start_time;
        println!(
            "[AccessLog-POST] Handling a request to {}{}, state={:?}, cost_time={}μs",
            state.host,
            state.uri,
            state,
            cost_time.as_micros()
        );
    }
}
