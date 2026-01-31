MCP Registry — Minimal Enterprise MCP Server Discovery

Problem

Today, MCP clients discover MCP servers manually:

Hardcoded endpoints in config files

Wiki pages / Slack messages

Tribal knowledge

This breaks down quickly in enterprises:

Endpoints change

Servers proliferate

No visibility into available tools

No standard discovery mechanism

MCP has no native discovery layer today.

What This Project Does

This project provides a minimal, self-hosted MCP registry that enables:

MCP servers to self-register

MCP clients to discover servers by capability

No hardcoded endpoints

No cloud dependencies

No UI

No auth (by design, for MVP)

It is intended to run inside enterprise networks as a lightweight control plane.

Non-Goals (Intentional)

This MVP explicitly does not include:

Authentication / RBAC

Heartbeats or TTL

Persistence beyond memory

Federation across registries

Public marketplaces

Those are future layers, not MVP requirements.


Core Concepts
MCP Server Entry

Each MCP server registers itself with:

server_id

endpoint

tools

scope

Discovery

Clients query the registry by:

required tools

scope

The registry returns matching MCP server endpoints.

API Overview

Register MCP Server

POST /register

{
  "server_id": "finance.mcp.local",
  "endpoint": "http://127.0.0.1:9001",
  "tools": ["invoice_lookup"],
  "scope": "finance"
}

Discover MCP Servers

POST /discover
{
  "required_tools": ["invoice_lookup"],
  "scope": "finance"
}

Response:
{
  "servers": [
    {
      "server_id": "finance.mcp.local",
      "endpoint": "http://127.0.0.1:9001",
      "tools": ["invoice_lookup"],
      "scope": "finance"
    }
  ]
}

Why This Exists

This project formalizes what enterprises already do informally:

Maintain a mental or documented list of MCP servers

Share endpoints manually

Hope configs stay in sync

The registry turns that into:

A single source of truth

A machine-readable discovery API

A foundation for future trust, policy, and lifecycle features

Status

✅ Working MVP

🧪 In-memory registry

🔧 Designed for extension

📗 Integration Guide

MCP Server Integration

When Should a Server Register?

An MCP server should register:

On startup

After deployment

After configuration changes

Minimal Integration Example

use mcp_protocol::McpServerEntry;


let entry = McpServerEntry {
    server_id: "finance.mcp.local".into(),
    endpoint: "http://127.0.0.1:9001".into(),
    tools: vec!["invoice_lookup".into()],
    scope: "finance".into(),
};

reqwest::Client::new()
    .post("http://registry.internal:8000/register")
    .json(&entry)
    .send()
    .await?;
    
That’s it.
The MCP server does not need to know:

Which clients exist

Who will consume it

How discovery works internally

MCP Client Integration

Discovery Flow

Client starts

Client queries registry

Registry returns matching MCP server endpoints

Client connects to MCP server normally

Minimal Client Example

use mcp_protocol::{DiscoverRequest, DiscoverResponse};


let req = DiscoverRequest {
    required_tools: vec!["invoice_lookup".into()],
    scope: "finance".into(),
};

let resp: DiscoverResponse = reqwest::Client::new()
    .post("http://registry.internal:8000/discover")
    .json(&req)
    .send()
    .await?
    .json()
    .await?;
The client never hardcodes MCP server endpoints.
