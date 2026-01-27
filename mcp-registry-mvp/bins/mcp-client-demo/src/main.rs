use mcp_protocol::{DiscoverRequest, DiscoverResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let req = DiscoverRequest {
        required_tools: vec!["invoice_lookup".into()],
        scope: "finance".into(),
    };

    let resp: DiscoverResponse = client
        .post("http://127.0.0.1:8000/discover")
        .json(&req)
        .send()
        .await?
        .json()
        .await?;

    println!("Discovered MCP servers:");
    for s in resp.servers {
        println!("  {} → {}", s.server_id, s.endpoint);
    }

    Ok(())
}

