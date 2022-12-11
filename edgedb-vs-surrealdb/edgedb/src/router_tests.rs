use axum::http::StatusCode;
use axum_test_helper::TestClient;
use pretty_assertions::assert_eq;
use std::process::Command;

const EDGEDB_INSTANCE: &str = "router_tests";

#[ctor::ctor]
fn setup() {
    std::env::set_var("RUST_LOG", "debug");
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

#[tokio::test]
async fn test_add_repository() -> anyhow::Result<()> {
    let get_repos_comtrya_comtrya = mockito::mock("GET", "/repos/comtrya/comtrya")
        .with_status(200)
        .with_body_from_file("./fixtures/get_repos_comtrya_comtrya.json")
        .create();

    {
        let router = crate::router::router().await?;
        let client = TestClient::new(router);

        let result = client.post("/repository/comtrya/comtrya").send().await;

        assert_eq!(result.status(), StatusCode::OK);
    }

    get_repos_comtrya_comtrya.assert();

    Ok(())
}