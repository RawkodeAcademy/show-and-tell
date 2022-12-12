use crate::{controller, ApplicationState};
use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

pub async fn router() -> anyhow::Result<Router> {
    #[cfg(not(test))]
    let base_url = "https://api.github.com/";

    #[cfg(test)]
    let base_url = mockito::server_url();

    let octocrab = octocrab::OctocrabBuilder::new()
        .base_url(base_url)?
        .personal_token("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".to_string())
        .build()?;

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
