use crate::{controller, ApplicationState};
use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::{info, warn};

pub async fn router() -> anyhow::Result<Router> {
    #[cfg(not(test))]
    let base_url = "https://api.github.com/";

    #[cfg(test)]
    let base_url = mockito::server_url();

    let mut octocrab_builder = octocrab::OctocrabBuilder::new().base_url(base_url)?;

    if let Ok(personal_token) = std::env::var("GITHUB_TOKEN") {
        info!("Using GitHub personal token from GITHUB_TOKEN environment variable.");

        octocrab_builder = octocrab_builder.personal_token(personal_token);
    } else {
        warn!("GITHUB_TOKEN environment variable not found. Using GitHub API without authentication. This is rate limited.");
    }

    let octocrab = octocrab_builder.build()?;

    let edgedb = edgedb_tokio::create_client().await?;

    let shared_state = Arc::new(ApplicationState { octocrab, edgedb });

    let router = Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/", get(|| async { "Hello, World!" }))
        .route_service("/user/:user_name", post(controller::add_user))
        .route_service(
            "/repository/by/language",
            get(controller::get_all_repositories_by_language),
        )
        .route_service("/repository", get(controller::get_all_repositories))
        .layer(Extension(shared_state));

    Ok(router)
}
