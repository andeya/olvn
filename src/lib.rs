pub mod ars;
mod routing;
mod serve;
pub mod state;

pub use crate::routing::*;
pub use crate::serve::*;
pub use tokio::test;

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
        static mut BLUE: Option<StateRouter> = None;
        static mut GREEN: Option<StateRouter> = None;
        static SERVE: Serve = Serve::new();
        unsafe {
            BLUE = Some(
                StateRouter::new()
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
                            SERVE.refresh_router(DynamicRouter::from_fallback(GREEN.clone().unwrap()));
                            println!("Switch to Green!");
                            "Switch to Green!"
                        }),
                    ),
            );

            GREEN = Some(
                StateRouter::new()
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
                            SERVE.refresh_router(DynamicRouter::from_fallback(BLUE.clone().unwrap()));
                            println!("Switch to Blue!");
                            "Switch to Blue!"
                        }),
                    ),
            );
            SERVE.refresh_router(DynamicRouter::from_fallback(BLUE.clone().unwrap()));
        }

        // run our app with hyper, listening globally on port 3000
        let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
        SERVE.serve(listener).await.unwrap();
        // SERVE.serve_with_graceful_shutdown(listener, signal()).await.unwrap();
    }
}
