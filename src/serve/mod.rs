use crate::routing::{DynamicRouter, GwRouter};
use std::{future::Future, sync::Mutex};
pub use tokio::net::TcpListener;

pub struct Serve {
    make_service: Mutex<Option<GwRouter>>,
}
impl Serve {
    pub const fn new() -> Self {
        Self {
            make_service: Mutex::new(None),
        }
    }
    fn get_or_init_router(&self) -> GwRouter {
        let mut make_service = self.make_service.lock().unwrap();
        if make_service.is_none() {
            let _ = make_service.insert(GwRouter::new());
        }
        make_service.clone().unwrap()
    }
    pub fn refresh_router(&self, router: DynamicRouter) -> &Self {
        self.get_or_init_router().refresh(router);
        self
    }
    pub async fn serve(&self, listener: TcpListener) -> Result<(), std::io::Error> {
        axum::serve(listener, self.get_or_init_router()).await
    }
    pub async fn serve_with_graceful_shutdown<F>(&self, listener: TcpListener, signal: F) -> Result<(), std::io::Error>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        axum::serve(listener, self.get_or_init_router())
            .with_graceful_shutdown(signal)
            .await
    }
}
