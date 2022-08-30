use super::*;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn test_get_user_data() {
    let routes = Router::new().route("/random", get(healthcheck));

    let response = routes
        .oneshot(
            Request::builder()
                .uri("/user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["name"], "Emil");
    assert_eq!(body["email"], "emil@viaplaygroup.com");
    assert_eq!(body["age"], 7);
}

#[tokio::test]
async fn test_get_dog() {
    let routes = Router::new().route("/random", get(healthcheck));

    let response = routes
        .oneshot(
            Request::builder()
                .uri("/dog")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    let dog = body["dog"].to_string();

    assert_eq!(dog.contains("images.dog.ceo"), true);
}