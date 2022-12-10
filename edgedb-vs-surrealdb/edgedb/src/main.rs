use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::{net::SocketAddr, sync::Arc};

mod controller;
mod error;
mod model;

pub struct ApplicationState {
    pub octocrab: octocrab::Octocrab,
    pub edgedb: edgedb_tokio::Client,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let octocrab = octocrab::OctocrabBuilder::new()
        .personal_token(std::env::var("PERSONAL_GITHUB_TOKEN").unwrap())
        .build()?;

    let edgedb = edgedb_tokio::create_client().await?;

    let shared_state = Arc::new(ApplicationState { octocrab, edgedb });

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route_service("/repository", get(controller::get_all_repositories))
        .route_service(
            "/repository/by/language",
            get(controller::get_all_repositories_by_language),
        )
        .route_service("/repository/:owner/:repo", post(controller::add_repository))
        .layer(Extension(shared_state));

    let address = SocketAddr::from(([127, 0, 0, 1], 1337));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
