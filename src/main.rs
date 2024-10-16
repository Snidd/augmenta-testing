use axum::response::Html;
use axum::routing::{get, post};
use std::error::Error;
use tokio::net::TcpListener;

use axum::{Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct EntityUpdated {
    id: String,
    numeric_id: i32,
    timestamp: String,
    fields: Vec<FieldModel>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct FieldModel {
    field_type_id: String,
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[axum::debug_handler]
async fn entity_updated(Json(payload): Json<EntityUpdated>) -> Json<EntityUpdated> {
    // Process the payload here
    Json(payload)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/", get(index))
        .route("/api/listeners/EntityUpdated", post(entity_updated));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
