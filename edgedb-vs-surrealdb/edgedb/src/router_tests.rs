use axum::http::StatusCode;
use axum_test_helper::TestClient;
use pretty_assertions::assert_eq;
use std::process::Command;

use crate::model::Repository;

const EDGEDB_INSTANCE: &str = "router_tests";

#[ctor::ctor]
fn setup() {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("EDGEDB_INSTANCE", EDGEDB_INSTANCE);

    let _ = env_logger::try_init();

    Command::new("bash")
        .arg("./setup.sh")
        .arg(EDGEDB_INSTANCE)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

#[ctor::dtor]
fn teardown() {
    std::env::remove_var("RUST_LOG");
    std::env::remove_var("EDGEDB_INSTANCE");

    Command::new("bash")
        .arg("./teardown.sh")
        .arg(EDGEDB_INSTANCE)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

async fn clean_tables() -> anyhow::Result<()> {
    let client = edgedb_tokio::create_client().await?;
    client.query_json("DELETE Repository;", &()).await?;

    Ok(())
}

#[tokio::test]
async fn test_add_user() -> anyhow::Result<()> {
    clean_tables().await?;

    let get_users_icepuma_starred = mockito::mock("GET", "/users/icepuma/starred")
        .with_status(200)
        .with_body_from_file("./fixtures/get_users_icepuma_starred.json")
        .create();

    {
        let router = crate::router::router().await?;
        let client = TestClient::new(router);

        let result = client.post("/user/icepuma").send().await;

        assert_eq!(result.status(), StatusCode::OK);
    }

    get_users_icepuma_starred.assert();

    Ok(())
}

#[tokio::test]
async fn test_list_empty_repositories() -> anyhow::Result<()> {
    clean_tables().await?;

    let router = crate::router::router().await?;
    let client = TestClient::new(router);

    let result = client.get("/repository").send().await;

    assert_eq!(result.status(), StatusCode::OK);

    let repositories = result.json::<Vec<Repository>>().await;

    assert_eq!(repositories, vec![]);

    Ok(())
}

#[tokio::test]
async fn test_list_repositories() -> anyhow::Result<()> {
    clean_tables().await?;

    let get_users_icepuma_starred = mockito::mock("GET", "/users/icepuma/starred")
        .with_status(200)
        .with_body_from_file("./fixtures/get_users_icepuma_starred.json")
        .create();

    {
        let router = crate::router::router().await?;
        let client = TestClient::new(router);

        let result = client.post("/user/icepuma").send().await;
        assert_eq!(result.status(), StatusCode::OK);

        let result = client.get("/repository").send().await;

        assert_eq!(result.status(), StatusCode::OK);

        let repositories = result.json::<Vec<Repository>>().await;

        assert_eq!(repositories.len(), 30);
    }

    get_users_icepuma_starred.assert();

    Ok(())
}
