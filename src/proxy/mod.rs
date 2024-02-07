mod aro;

use crate::error::GwError;
use crate::routing::{IntoResponse, Request, Response};
use crate::state::GwContext;
pub use aro::{Aro, ProxyHandler};

impl ProxyHandler {
    #[inline]
    pub(crate) fn reverse_proxy(&self, req: Request) -> Response {
        let state = req.extensions().get::<GwContext>().unwrap();
        println!("{:?}", req);
        format!("method={}, path={}, state={:?}", self.method, self.path, state).into_response()
    }
    #[inline]
    fn convert_request(&self, req: &mut Request) -> Result<(), GwError> {
        unimplemented!()
    }
    #[inline]
    fn convert_response<T>(&self, _resp: T) -> Response {
        unimplemented!()
    }
}
