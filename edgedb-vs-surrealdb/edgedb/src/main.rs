use axum::{routing::get, Extension, Router};
use std::{net::SocketAddr, sync::Arc};

mod controller;
mod error;

pub struct ApplicationState {
    pub octocrab: octocrab::Octocrab,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let octocrab = octocrab::OctocrabBuilder::new()
        .personal_token(std::env::var("PERSONAL_GITHUB_TOKEN").unwrap())
        .build()?;

    let shared_state = Arc::new(ApplicationState { octocrab });

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route_service("/stars/:user_name", get(controller::get_stars_of_user))
        .layer(Extension(shared_state));

    let address = SocketAddr::from(([127, 0, 0, 1], 1337));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
