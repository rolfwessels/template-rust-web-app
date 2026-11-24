use axum::{
    Router,
    extract::Path,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use include_dir::{Dir, include_dir};

static WEB: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../web/dist");

pub fn routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/assets/{*path}", get(asset))
}

async fn index() -> Response {
    embedded_file("index.html")
}

async fn asset(Path(path): Path<String>) -> Response {
    embedded_file(&format!("assets/{path}"))
}

fn embedded_file(path: &str) -> Response {
    let Some(file) = WEB.get_file(path) else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let content_type = mime_guess::from_path(file.path()).first_or_octet_stream();
    (
        [(header::CONTENT_TYPE, content_type.as_ref())],
        file.contents(),
    )
        .into_response()
}
