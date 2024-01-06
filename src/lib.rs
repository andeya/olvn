mod routing;
mod serve;

#[cfg(test)]
mod tests {
    use axum::{routing::get, Router};

    #[tokio::test]
    async fn it_work() {
        // build our application with a single route
        let app = Router::new().route(
            "/",
            get(|| async {
                println!("Hello, world!");
                "Hello, World!"
            }),
        );

        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}
