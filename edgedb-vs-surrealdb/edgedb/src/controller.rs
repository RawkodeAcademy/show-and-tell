use crate::{error::ApplicationError, ApplicationState};
use axum::{extract::Path, Extension, Json};
use std::sync::Arc;

pub async fn get_stars_of_user(
    Path(user_name): Path<String>,
    Extension(state): Extension<Arc<ApplicationState>>,
) -> Result<Json<Vec<octocrab::models::Repository>>, ApplicationError> {
    let uri = format!("{}users/{}/starred", state.octocrab.base_url, user_name);

    let request_builder = state.octocrab.request_builder(uri, reqwest::Method::GET);

    let response = state.octocrab.execute(request_builder).await?;

    let repositories = response.json::<Vec<octocrab::models::Repository>>().await?;

    Ok(Json(repositories))
}
