#[cfg(test)]
mod tests {
    use crate::routing;
    use axum::{routing::get, Router};
    use lazy_static::lazy_static;
    use routing::DynamicRouter;
    #[tokio::test]
    async fn it_work() {
        // build our application with a single route
        lazy_static! {
            static ref ROUTER: DynamicRouter = DynamicRouter::new();
        }
        static mut BLUE: Option<Router> = None;
        static mut GREEN: Option<Router> = None;
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
                            ROUTER.switch(GREEN.clone().unwrap());
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
                            ROUTER.switch(BLUE.clone().unwrap());
                            println!("Switch to Blue!");
                            "Switch to Blue!"
                        }),
                    ),
            );
            ROUTER.switch(BLUE.clone().unwrap());
        }

        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, ROUTER.clone()).await.unwrap();
    }
}
