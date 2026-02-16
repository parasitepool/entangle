use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

use crate::*;

#[tokio::test]
async fn post_swap_returns_200() {
    let app = entangle::api::router();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/swap")
                .header("content-type", "application/json")
                .body(Body::from(swap_request_json(true)))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let psbt_str = json["psbt_base64"].as_str().unwrap();
    // Verify it parses as a valid PSBT
    let _: bitcoin::Psbt = psbt_str.parse().unwrap();
}

#[tokio::test]
async fn post_swap_network_mismatch_returns_400() {
    let app = entangle::api::router();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/swap")
                .header("content-type", "application/json")
                .body(Body::from(mismatch_request_json()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json.get("error").is_some());
}

#[tokio::test]
async fn post_swap_invalid_json_returns_400() {
    let app = entangle::api::router();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/swap")
                .header("content-type", "application/json")
                .body(Body::from("not json"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
