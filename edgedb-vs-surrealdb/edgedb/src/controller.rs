use crate::{error::ApplicationError, model::Repository, ApplicationState};
use axum::{extract::Path, Extension, Json};
use edgedb_protocol::value::Value;
use std::sync::Arc;

pub async fn get_all_repositories_by_language(
    Extension(state): Extension<Arc<ApplicationState>>,
) -> Result<String, ApplicationError> {
    state.edgedb.ensure_connected().await?;

    let query = r#"group Repository { name } by .language;"#;

    let repositories = state.edgedb.query_json(query, &()).await?; // FIXME: use query::<R, _>

    Ok(repositories.to_string())
}

pub async fn get_all_repositories(
    Extension(state): Extension<Arc<ApplicationState>>,
) -> Result<Json<Vec<Repository>>, ApplicationError> {
    state.edgedb.ensure_connected().await?;

    let query = r#"SELECT Repository { id, name, url, language };"#;

    let repositories = state.edgedb.query::<Repository, _>(query, &()).await?;

    Ok(Json(repositories))
}

pub async fn add_user(
    Path(user_name): Path<String>,
    Extension(state): Extension<Arc<ApplicationState>>,
) -> Result<(), ApplicationError> {
    state.edgedb.ensure_connected().await?;

    let url = format!("{}users/{}/repos", state.octocrab.base_url, user_name);

    let request_builder = state.octocrab.request_builder(url, reqwest::Method::GET);

    let response = state.octocrab.execute(request_builder).await?;

    let repositories = response.json::<Vec<octocrab::models::Repository>>().await?;

    for repository in repositories {
        let query = r#"
            INSERT Repository {
                name := <str>$0,
                url := <str>$1,
                language := <str>$2
            };
        "#;

        let language = repository
            .language
            .and_then(|language| serde_json::to_string(&language).ok())
            .map(|language| language.replace('"', "")) // replace quotes from "serde_json::to_string"
            .unwrap_or_else(|| "not found".to_string());

        state
            .edgedb
            .query_single::<Value, _>(
                query,
                &(repository.name, repository.url.to_string(), language),
            )
            .await?
            .ok_or_else(|| anyhow::anyhow!("Failed to insert repository"))?;
    }

    Ok(())
}
