# viewpoint-mcp

Rust library for building MCP (Model Context Protocol) servers with browser automation capabilities.

## Overview

This crate provides the core library for the Viewpoint MCP server. It can be used to:

- Embed an MCP server in your own application
- Build custom browser automation tools
- Extend the default tool set with custom implementations

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
viewpoint-mcp = "0.1"
tokio = { version = "1", features = ["full"] }
```

### Basic Server

```rust
use viewpoint_mcp::{McpServer, ServerConfig};
use viewpoint_mcp::transport::StdioTransport;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = McpServer::new(ServerConfig::default());
    let transport = StdioTransport::new(server);
    transport.run().await?;
    Ok(())
}
```

### Custom Configuration

```rust
use viewpoint_mcp::{McpServer, ServerConfig};
use viewpoint_mcp::browser::{BrowserConfig, ViewportSize};

let config = ServerConfig {
    browser: BrowserConfig {
        headless: true,
        viewport: Some(ViewportSize::new(1920, 1080)),
        ..Default::default()
    },
    capabilities: vec!["vision".to_string(), "pdf".to_string()],
    ..Default::default()
};

let server = McpServer::new(config);
```

### SSE Transport

For HTTP-based clients:

```rust
use viewpoint_mcp::{McpServer, ServerConfig};
use viewpoint_mcp::transport::{SseTransport, SseConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = McpServer::new(ServerConfig::default());
    let config = SseConfig::new(8080);
    
    println!("API Key: {}", config.api_key);
    
    let transport = SseTransport::new(server, config);
    transport.run().await?;
    Ok(())
}
```

## Module Structure

- **`browser`**: Browser state and configuration management
- **`server`**: MCP protocol implementation
- **`snapshot`**: Accessibility tree capture and element references
- **`tools`**: MCP tool implementations (30 browser tools)
- **`transport`**: Communication layers (stdio, SSE)

## Capabilities

Optional capabilities that enable additional tools:

| Capability | Tools Enabled |
|------------|---------------|
| `vision` | `browser_mouse_click_xy`, `browser_mouse_move_xy`, `browser_mouse_drag_xy` |
| `pdf` | `browser_pdf_save` |

## License

MIT
