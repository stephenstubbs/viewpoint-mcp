# viewpoint-mcp

An MCP (Model Context Protocol) server for browser automation, powered by [Viewpoint](https://github.com/stephenstubbs/viewpoint) - the Rust equivalent of Playwright.

## Features

- **30 Browser Tools**: Complete browser automation toolkit for LLMs
- **Accessibility Snapshots**: Semantic page representation with element references
- **Multi-Context Support**: Isolated browser contexts for parallel operations
- **Vision & PDF Capabilities**: Optional coordinate-based interactions and PDF generation
- **MCP Server**: JSON-RPC over stdio or SSE transport

## Installation

### Using Nix Flakes

Run directly without installing:

```bash
nix run github:stephenstubbs/viewpoint-mcp
```

Or add to your flake inputs:

```nix
{
  inputs.viewpoint-mcp.url = "github:stephenstubbs/viewpoint-mcp";
}
```

Then use `inputs.viewpoint-mcp.packages.${system}.default` in your configuration.

### Development Shell

Enter a development environment with all dependencies:

```bash
nix develop github:stephenstubbs/viewpoint-mcp
```

## Usage

### MCP Server Mode (Default)

Run as an MCP server on stdio:

```bash
viewpoint-mcp
```

With headless browser:

```bash
viewpoint-mcp --headless
```

### SSE Transport

Run with SSE transport on a specific port:

```bash
viewpoint-mcp --port 8080
```

With a custom API key:

```bash
viewpoint-mcp --port 8080 --api-key your-secret-key
```

### Options

| Option | Description |
|--------|-------------|
| `--headless` | Run browser in headless mode |
| `--browser <TYPE>` | Browser type: `chromium` (default) or `chrome` |
| `--viewport-size <WxH>` | Viewport size (e.g., `1280x720`) |
| `--cdp-endpoint <URL>` | Connect to existing browser via CDP |
| `--user-data-dir <PATH>` | Browser profile persistence directory |
| `--port <PORT>` | Enable SSE transport on specified port |
| `--api-key <KEY>` | API key for SSE authentication |
| `--caps <CAPS>` | Enable capabilities: `vision`, `pdf` (comma-separated) |

## Library Usage

The MCP server can also be used as a library in your Rust projects:

```rust
use viewpoint_mcp::{McpServer, ServerConfig};
use viewpoint_mcp::transport::StdioTransport;
use viewpoint_mcp::browser::{BrowserConfig, ViewportSize};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configure the server
    let config = ServerConfig {
        browser: BrowserConfig {
            headless: true,
            viewport: Some(ViewportSize::new(1920, 1080)),
            ..Default::default()
        },
        capabilities: vec!["vision".to_string()],
        ..Default::default()
    };

    // Create and run the server
    let server = McpServer::new(config);
    let transport = StdioTransport::new(server);
    transport.run().await?;

    Ok(())
}
```

For SSE transport:

```rust
use viewpoint_mcp::{McpServer, ServerConfig};
use viewpoint_mcp::transport::{SseTransport, SseConfig};

let server = McpServer::new(ServerConfig::default());
let config = SseConfig::new(8080);
let transport = SseTransport::new(server, config);
transport.run().await?;
```

See the [library crate documentation](crates/viewpoint-mcp/README.md) for more details.

## OpenCode Setup

Add viewpoint-mcp to your OpenCode configuration in `opencode.json`:

```json
{
  "$schema": "https://opencode.ai/config.json",
  "mcp": {
    "viewpoint-mcp": {
      "type": "local",
      "command": ["nix", "run", "github:stephenstubbs/viewpoint-mcp"]
    }
  }
}
```

With headless mode and vision capabilities:

```json
{
  "$schema": "https://opencode.ai/config.json",
  "mcp": {
    "viewpoint-mcp": {
      "type": "local",
      "command": ["nix", "run", "github:stephenstubbs/viewpoint-mcp", "--", "--headless", "--caps", "vision"]
    }
  }
}
```

Once configured, the browser tools (prefixed with `browser_`) will be available to the LLM.

## Available Tools

### Navigation
- `browser_navigate` - Navigate to a URL
- `browser_navigate_back` - Go back in history

### Interaction
- `browser_click` - Click element by reference
- `browser_type` - Type text into element
- `browser_fill_form` - Fill multiple form fields
- `browser_hover` - Hover over element
- `browser_drag` - Drag between elements
- `browser_select_option` - Select dropdown option
- `browser_press_key` - Press keyboard key
- `browser_file_upload` - Upload files

### Inspection
- `browser_snapshot` - Capture accessibility tree
- `browser_take_screenshot` - Take screenshot
- `browser_console_messages` - Get console logs
- `browser_network_requests` - List network activity

### State
- `browser_evaluate` - Execute JavaScript
- `browser_wait_for` - Wait for conditions
- `browser_handle_dialog` - Handle alerts/dialogs

### Management
- `browser_close` - Close page/browser
- `browser_resize` - Resize viewport
- `browser_tabs` - Manage browser tabs
- `browser_install` - Install browser

### Context Management
- `browser_context_create` - Create isolated context
- `browser_context_switch` - Switch active context
- `browser_context_list` - List all contexts
- `browser_context_close` - Close context
- `browser_context_save_storage` - Export cookies/storage

### Vision (requires `--caps vision`)
- `browser_mouse_click_xy` - Click at coordinates
- `browser_mouse_move_xy` - Move to coordinates
- `browser_mouse_drag_xy` - Drag between coordinates

### PDF (requires `--caps pdf`)
- `browser_pdf_save` - Save page as PDF

## Architecture

```
viewpoint-mcp/
├── crates/
│   ├── viewpoint-mcp/       # Core library
│   │   ├── browser/         # Browser state management
│   │   ├── server/          # MCP protocol implementation
│   │   ├── snapshot/        # Accessibility tree capture
│   │   ├── tools/           # 30 browser automation tools
│   │   └── transport/       # stdio and SSE transports
│   └── viewpoint-mcp-cli/   # CLI binary
```

### Key Components

- **Browser State**: Lazy initialization, multi-context support, connection recovery
- **Accessibility Snapshots**: Semantic page representation with element references
- **Tool Registry**: Capability-aware tool management
- **Transport Layer**: stdio for CLI clients, SSE for HTTP clients

## Platform Support

- Linux (x86_64, aarch64)
- macOS (x86_64, aarch64)

## License

MIT
