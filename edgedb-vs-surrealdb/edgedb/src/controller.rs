use axum::{extract::Path, Json};

use crate::error::ApplicationError;

pub async fn get_stars_of_user(
    Path(user_name): Path<String>,
) -> Result<Json<Vec<octocrab::models::Repository>>, ApplicationError> {
    let token = std::env::var("PERSONAL_GITHUB_TOKEN")?;
    let builder = octocrab::OctocrabBuilder::new()
        .personal_token(token)
        .build()?;

    let uri = format!("{}users/{}/starred", builder.base_url, user_name);

    let request_builder = builder.request_builder(uri, reqwest::Method::GET);

    let response = builder.execute(request_builder).await?;

    let repositories = response.json::<Vec<octocrab::models::Repository>>().await?;

    Ok(Json(repositories))
}
