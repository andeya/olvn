use crate::{
    ars::Ars,
    error::GwError,
    routing::{DynRouter, GwRouter},
    transcoding::Transcoding,
};
use std::{
    future::Future,
    sync::{Arc, Mutex},
};
pub use tokio::net::TcpListener;

pub struct Serve {
    make_service: Mutex<Option<DynRouter>>,
}
impl Serve {
    pub const fn new() -> Self {
        Self {
            make_service: Mutex::new(None),
        }
    }
    fn get_or_init_router(&self) -> DynRouter {
        let mut make_service = self.make_service.lock().unwrap();
        if make_service.is_none() {
            let _ = make_service.insert(DynRouter::new());
        }
        make_service.clone().unwrap()
    }
    pub fn hot_update(&self, router: GwRouter) -> &Self {
        self.get_or_init_router().refresh(router);
        self
    }
    pub fn hot_update_ars(&self, ars: Ars, transcoding: Arc<Transcoding>) -> Result<(), GwError> {
        self.get_or_init_router().refresh(GwRouter::from_ars(ars, transcoding)?);
        Ok(())
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
