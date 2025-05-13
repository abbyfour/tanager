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
    paths: Vec<String>,
    tags: RequestedTags,
}

async fn edit_tags(Json(payload): Json<EditRequest>) -> impl IntoResponse {
    let tag_editor = TagEditor::new();
    let mut successes = 0;

    for path in &payload.paths {
        let mut tagged_file = match Probe::open(path).and_then(|p| p.read()) {
            Ok(f) => f,
            Err(e) => return format!("Failed to open file: {e}"),
        };

        let mut request = tags::tag_editor::TagEditRequest {
            tagged_file: &mut tagged_file,
            tags: &payload.tags,
        };

        let result = tag_editor.apply_and_save(&mut request, path.clone());

        match result {
            Ok(_) => successes += 1,
            Err(e) => return format!("Failed to apply tags: {e}"),
        }
    }

    if successes == payload.paths.len() {
        return "All tags edited successfully".to_string();
    } else {
        return format!(
            "Failed to edit tags for {} files",
            payload.paths.len() - successes
        )
        .to_string();
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/edit", post(edit_tags));

    let addr = SocketAddr::from(([0, 0, 0, 0], 2222));
    println!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    serve(listener, app)
        .await
        .unwrap_or_else(|err| eprintln!("Server error: {err}"));
}
