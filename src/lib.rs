mod routing;
mod serve;
pub use crate::routing::*;
pub use crate::serve::*;


#[cfg(test)]
mod tests {
    use crate::serve::*;
    use axum::{routing::get, Router};
    use tokio::time::{sleep, Duration};
    async fn signal() {
        sleep(Duration::from_secs(10)).await;
        println!("Signal!");
    }

    #[tokio::test]
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
                            SERVE.switch_router(GREEN.clone().unwrap());
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
                            SERVE.switch_router(BLUE.clone().unwrap());
                            println!("Switch to Blue!");
                            "Switch to Blue!"
                        }),
                    ),
            );
            SERVE.switch_router(BLUE.clone().unwrap());
        }

        // run our app with hyper, listening globally on port 3000
        let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
        // SERVE.serve(listener).await.unwrap();
        SERVE.serve_with_graceful_shutdown(listener, signal()).await.unwrap();
    }
}
