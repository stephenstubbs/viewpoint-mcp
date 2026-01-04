# viewpoint-mcp-cli

Command-line interface for the Viewpoint MCP server.

## Installation

### Using Nix Flakes

```bash
nix run github:stephenstubbs/viewpoint-mcp
```

### From Source

```bash
cargo install --path .
```

## Usage

### Basic Usage

Run as an MCP server on stdio (default):

```bash
viewpoint-mcp
```

### Headless Mode

```bash
viewpoint-mcp --headless
```

### SSE Transport

Run with HTTP transport on a specific port:

```bash
viewpoint-mcp --port 8080
```

With a custom API key:

```bash
viewpoint-mcp --port 8080 --api-key your-secret-key
```

### Connect to Existing Browser

```bash
viewpoint-mcp --cdp-endpoint http://localhost:9222
```

### Persistent Profile

```bash
viewpoint-mcp --user-data-dir /path/to/profile
```

### Enable Capabilities

```bash
viewpoint-mcp --caps vision,pdf
```

## Options

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

## MCP Client Configuration

### OpenCode

Add to `opencode.json`:

```json
{
  "mcp": {
    "viewpoint-mcp": {
      "type": "local",
      "command": ["viewpoint-mcp", "--headless"]
    }
  }
}
```

### Claude Desktop

Add to `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "viewpoint-mcp": {
      "command": "viewpoint-mcp",
      "args": ["--headless"]
    }
  }
}
```

## License

MIT
