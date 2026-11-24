//! HTTP, GraphQL, and storage adapters.

pub mod graphql;
pub mod memory_list;

#[cfg(feature = "embed-web")]
mod web_assets;

use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    Json, Router,
    routing::{get, post_service},
};
use graphql::AppSchema;
use serde_json::{Value, json};
use tower_http::trace::TraceLayer;

pub fn router(schema: AppSchema) -> Router {
    let app = Router::new()
        .route("/healthz", get(health))
        .route("/graphql", post_service(GraphQL::new(schema.clone())))
        .route_service("/graphql/ws", GraphQLSubscription::new(schema));

    #[cfg(feature = "embed-web")]
    let app = app.merge(web_assets::routes());

    app.layer(TraceLayer::new_for_http())
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
