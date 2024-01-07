use std::{future::Future, sync::Mutex};

use crate::routing::{DynamicRouter, Router};
pub use tokio::net::TcpListener;
pub struct Serve {
    make_service: Mutex<Option<DynamicRouter>>,
}
impl Serve {
    pub const fn new() -> Self {
        Self {
            make_service: Mutex::new(None),
        }
    }
    fn get_or_init_make_service(&self) -> DynamicRouter {
        let mut make_service = self.make_service.lock().unwrap();
        if make_service.is_none() {
            let _ = make_service.insert(DynamicRouter::new());
        }
        make_service.clone().unwrap()
    }
    pub(crate) fn switch_router(&self, router: Router) -> &Self {
        self.get_or_init_make_service().switch(router);
        self
    }
    pub async fn serve(&self, listener: TcpListener) -> Result<(), std::io::Error> {
        axum::serve(listener, self.get_or_init_make_service()).await
    }
    pub async fn serve_with_graceful_shutdown<F>(&self, listener: TcpListener, signal: F) -> Result<(), std::io::Error>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        axum::serve(listener, self.get_or_init_make_service())
            .with_graceful_shutdown(signal)
            .await
    }
}
