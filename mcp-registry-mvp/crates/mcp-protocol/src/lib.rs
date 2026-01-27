use serde::{Deserialize, Serialize};

use std::time::Instant;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct McpServerEntry {
    pub server_id: String,
    pub endpoint: String,
    pub tools: Vec<String>,
    pub scope: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoverRequest {
    pub required_tools: Vec<String>,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoverResponse {
    pub servers: Vec<McpServerEntry>,
}

