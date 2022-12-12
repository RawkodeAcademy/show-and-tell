use router::router;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "edgedb=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let address = SocketAddr::from(([127, 0, 0, 1], 1337));

    let router = router().await?;

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
