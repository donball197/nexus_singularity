use axum::{
    routing::{get, post},
    extract::{Query, Json},
    response::IntoResponse,
    Router,
};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct ReadQuery {
    pub path: String,
}

#[derive(Deserialize)]
pub struct SavePayload {
    pub path: String,
    pub content: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/files", get(list_files))
        .route("/read", get(read_file))
        .route("/save", post(save_file))
}

async fn list_files() -> impl IntoResponse {
    let mut out = Vec::new();
    let root = Path::new("src");

    if let Ok(entries) = fs::read_dir(root) {
        for e in entries.flatten() {
            let p = e.path();
            out.push(serde_json::json!({
                "name": p.file_name().unwrap().to_string_lossy(),
                "path": p.to_string_lossy(),
                "is_dir": p.is_dir()
            }));
        }
    }

    Json(out)
}

async fn read_file(Query(q): Query<ReadQuery>) -> impl IntoResponse {
    fs::read_to_string(&q.path).unwrap_or_default()
}

async fn save_file(Json(p): Json<SavePayload>) -> impl IntoResponse {
    match fs::write(&p.path, p.content) {
        Ok(_) => "Saved",
        Err(_) => "Error",
    }
}
