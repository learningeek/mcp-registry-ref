use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:8000".parse().unwrap();
    let data_path = "registry.json";

    mcp_registry::run_registry(addr, data_path).await;
}

