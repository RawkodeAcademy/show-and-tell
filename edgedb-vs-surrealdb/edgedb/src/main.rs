use std::net::SocketAddr;

use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", axum::handler::get(|| async { "Hello, World!" }));

    let address = SocketAddr::from(([127, 0, 0, 1], 1337));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
