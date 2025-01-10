pub mod christies;
pub mod date;
pub mod event;
pub mod identifier;
pub mod location;
pub mod provenance_set;
pub mod reference;

pub use crate::{
    date::Date,
    event::Event,
    identifier::{IdText, Identifier},
    provenance_set::ProvenanceSet,
    reference::Reference,
};

use axum::{
    extract::{Path, State},
    // handler::Handler,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
// use tokio::fs;
use tower_http::services::ServeDir;

#[derive(Debug, Clone)]
struct AppState {
    data: Arc<HashMap<String, HashMap<String, serde_json::Value>>>,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    property: String,
    id: String,
    value: serde_json::Value,
}

async fn get_property(
    Path((property, id)): Path<(String, String)>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(property_map) = state.data.get(&property) {
        if let Some(value) = property_map.get(&id) {
            let response = ApiResponse {
                property,
                id,
                value: value.clone(),
            };
            return (StatusCode::OK, Json(response)).into_response();
        }
    }
    (StatusCode::NOT_FOUND, "Not Found").into_response()
}

#[tokio::main]
async fn main() {
    // Example data
    let mut data = HashMap::new();
    let mut property_map = HashMap::new();
    property_map.insert("1".to_string(), serde_json::json!({"name": "Item 1"}));
    property_map.insert("2".to_string(), serde_json::json!({"name": "Item 2"}));
    data.insert("property".to_string(), property_map);

    let state = AppState {
        data: Arc::new(data),
    };

    // Serve static files from the "static" directory
    let static_files = get_service(ServeDir::new("html")).handle_error(|error| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", error),
        )
    });

    // Build our application with a route
    let app = Router::new()
        .fallback_service(static_files)
        .route("/api/{property}/{id}", get(get_property))
        .with_state(state);

    // Run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
