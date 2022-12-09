use axum::{routing::get, Router};
use std::net::SocketAddr;

mod controller;
mod error;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route_service("/stars/:user_name", get(controller::get_stars_of_user));

    let address = SocketAddr::from(([127, 0, 0, 1], 1337));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
