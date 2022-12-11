use router::router;
use std::net::SocketAddr;

mod controller;
mod error;
mod model;
mod router;

#[cfg(test)]
mod router_tests;

pub struct ApplicationState {
    pub octocrab: octocrab::Octocrab,
    pub edgedb: edgedb_tokio::Client,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let address = SocketAddr::from(([127, 0, 0, 1], 1337));

    let router = router().await?;

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
