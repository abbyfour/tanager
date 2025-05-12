mod tags;

use axum::{extract::Json, response::IntoResponse, routing::post, serve, Router};
use lofty::Probe;
use serde::Deserialize;
use std::net::SocketAddr;
use tags::requested_tags::RequestedTags;
use tokio::net::TcpListener;

use crate::tags::tag_editor::TagEditor;

#[derive(Deserialize)]
struct EditRequest {
    path: String,
    tags: RequestedTags,
}

async fn edit_tags(Json(payload): Json<EditRequest>) -> impl IntoResponse {
    let tagged_file = match Probe::open(&payload.path).and_then(|p| p.read()) {
        Ok(f) => f,
        Err(e) => return format!("Failed to open file: {e}"),
    };

    let result = TagEditor::new(tagged_file, payload.tags).apply_and_save(payload.path);

    match result {
        Ok(_) => "Tags updated!".to_string(),
        Err(e) => return format!("Failed to apply tags: {e}"),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/edit", post(edit_tags));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    serve(listener, app)
        .await
        .unwrap_or_else(|err| eprintln!("Server error: {err}"));
}
