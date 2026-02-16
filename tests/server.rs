use axum::body::Body;
use axum::http::{Request, StatusCode};
use leptos::config::get_configuration;
use tower::ServiceExt;

use crate::*;

fn temp_store() -> listing::ListingStore {
    let path = std::env::temp_dir().join("entangle_test_server.json");
    let _ = std::fs::remove_file(&path);
    listing::ListingStore::load(path)
}

#[tokio::test]
async fn with_api_nesting() {
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let app = entangle::server::router(leptos_options, true, temp_store());
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/swap")
                .header("content-type", "application/json")
                .body(Body::from(swap_request_json(true)))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
