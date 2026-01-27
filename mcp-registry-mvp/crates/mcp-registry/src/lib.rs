use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use std::{fs, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

use mcp_protocol::{DiscoverRequest, DiscoverResponse, McpServerEntry};
use axum::http::StatusCode;

#[derive(Clone)]
pub struct RegistryState {
    servers: Arc<tokio::sync::RwLock<Vec<McpServerEntry>>>,
}


async fn register(
    State(state): State<RegistryState>,
    Json(entry): Json<McpServerEntry>,
) -> StatusCode {

   
    let mut servers = state.servers.write().await;

    // replace if server_id already exists
    servers.retain(|s| s.server_id != entry.server_id);
    servers.push(entry);

    StatusCode::OK
}

pub fn load_registry(path: &str) -> Vec<McpServerEntry> {
    let data = fs::read_to_string(path).expect("failed to read registry file");
    serde_json::from_str(&data).expect("invalid registry json")
}

async fn discover(
    State(state): State<RegistryState>,
    Json(req): Json<DiscoverRequest>,
) -> Json<DiscoverResponse> {

let servers = state.servers.read().await;

let matches = servers
    .iter()
    .filter(|s| s.scope == req.scope)
    .filter(|s| {
        req.required_tools
            .iter()
            .all(|tool| s.tools.contains(tool))
    })
    .cloned()
    .collect();



    Json(DiscoverResponse { servers: matches })
}

pub async fn run_registry(addr: SocketAddr, data_path: &str) {
    let state = RegistryState {
        servers: Arc::new(RwLock::new(Vec::new())),
    };

    let app = Router::new()
    .route("/register", post(register))
    .route("/discover", post(discover))
    .with_state(state);

    println!("Registry listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

