use mcp_protocol::McpServerEntry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let entry = McpServerEntry {
        server_id: "finance.mcp.local".into(),
        endpoint: "http://127.0.0.1:9001".into(),
        tools: vec!["invoice_lookup".into()],
        scope: "finance".into(),
    };

    let client = reqwest::Client::new();

    client
        .post("http://127.0.0.1:8000/register")
        .json(&entry)
        .send()
        .await?;

    println!("MCP server registered");

    // fake server loop
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
