pub mod ars;
pub mod converter;
pub mod error;
mod routing;
mod serve;
pub mod state;
pub use crate::routing::*;
pub use crate::serve::*;
pub use tokio::{main, test};
pub mod plugin;
mod proxy;

#[cfg(test)]
mod tests {
    use crate::*;
    use axum::routing::get;
    use tokio::time::{sleep, Duration};

    #[allow(unused)]
    async fn signal() {
        sleep(Duration::from_secs(10)).await;
        println!("Signal!");
    }

    #[crate::test]
    async fn it_work() {
        static mut BLUE: Option<Router> = None;
        static mut GREEN: Option<Router> = None;
        static SERVE: Serve = Serve::new();
        unsafe {
            BLUE = Some(
                Router::new()
                    .route(
                        "/",
                        get(|| async {
                            println!("Hello, Blue!");
                            "Hello, Blue!"
                        }),
                    )
                    .route(
                        "/green",
                        get(|| async {
                            SERVE.hot_update(GwRouter::new(Some(GREEN.clone().unwrap())));
                            println!("Switch to Green!");
                            "Switch to Green!"
                        }),
                    ),
            );

            GREEN = Some(
                Router::new()
                    .route(
                        "/",
                        get(|| async {
                            println!("Hello, Green!");
                            "Hello, Green!"
                        }),
                    )
                    .route(
                        "/blue",
                        get(|| async {
                            SERVE.hot_update(GwRouter::new(Some(BLUE.clone().unwrap())));
                            println!("Switch to Blue!");
                            "Switch to Blue!"
                        }),
                    ),
            );
            SERVE.hot_update(GwRouter::new(Some(BLUE.clone().unwrap())));
        }

        // run our app with hyper, listening globally on port 3000
        let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
        SERVE.serve(listener).await.unwrap();
        // SERVE.serve_with_graceful_shutdown(listener, signal()).await.unwrap();
    }
}
