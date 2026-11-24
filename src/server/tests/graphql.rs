use app_core::StringListApp;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use futures_util::StreamExt;
use serde_json::json;
use std::time::Duration;
use tokio::time::timeout;
use tower::ServiceExt;

use template_rust_web_app::{graphql::schema, memory_list::InMemoryList, router};

fn sample_app() -> StringListApp {
    StringListApp::new(InMemoryList::new(vec!["seed".into()]), || {
        "new-string".into()
    })
}

#[tokio::test]
async fn exposes_the_health_endpoint() {
    let response = router(schema(sample_app()))
        .oneshot(Request::get("/healthz").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), 1024)
        .await
        .unwrap();
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&body).unwrap(),
        json!({"status": "ok"})
    );
}

#[tokio::test]
async fn query_mutation_and_subscription_share_the_list() {
    let schema = schema(sample_app());
    let initial = schema.execute("{ strings }").await.into_result().unwrap();
    assert_eq!(
        initial.data.into_json().unwrap(),
        json!({"strings": ["seed"]})
    );

    let mut updates = schema.execute_stream("subscription { stringsChanged }");
    let first = timeout(Duration::from_secs(2), updates.next())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        first.into_result().unwrap().data.into_json().unwrap(),
        json!({"stringsChanged": ["seed"]})
    );

    let mutation = schema
        .execute("mutation { addRandomString }")
        .await
        .into_result()
        .unwrap();
    assert_eq!(
        mutation.data.into_json().unwrap(),
        json!({"addRandomString": "new-string"})
    );

    let updated = timeout(Duration::from_secs(2), updates.next())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        updated.into_result().unwrap().data.into_json().unwrap(),
        json!({"stringsChanged": ["seed", "new-string"]})
    );
}
