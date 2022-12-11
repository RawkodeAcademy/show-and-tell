use crate::{controller, ApplicationState};
use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::sync::Arc;

pub async fn router() -> anyhow::Result<Router> {
    #[cfg(not(test))]
    let base_url = "https://api.github.com/";

    #[cfg(test)]
    let base_url = mockito::server_url();

    let octocrab = octocrab::OctocrabBuilder::new()
        .base_url(base_url)?
        .build()?;

    let edgedb = edgedb_tokio::create_client().await?;

    let shared_state = Arc::new(ApplicationState { octocrab, edgedb });

    let router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route_service("/repository", get(controller::get_all_repositories))
        .route_service(
            "/repository/by/language",
            get(controller::get_all_repositories_by_language),
        )
        .route_service("/repository/:owner/:repo", post(controller::add_repository))
        .layer(Extension(shared_state));

    Ok(router)
}
