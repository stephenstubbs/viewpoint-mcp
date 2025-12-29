# Change: Add MCP Server Foundation

## Why
We need a foundational MCP server that can receive tool requests over stdio/SSE transport, manage browser lifecycle, and dispatch to tool handlers. This is the core infrastructure all browser tools depend on.

## What Changes
- Add `viewpoint-mcp` crate with MCP protocol implementation
- Add `viewpoint-mcp-cli` crate for the CLI binary
- Implement JSON-RPC over stdio transport (default)
- Implement SSE transport for remote/Docker deployments
- Add browser connection management (launch, connect via CDP endpoint)
- Add CLI argument parsing matching playwright-mcp flags

## Impact
- Affected specs: `mcp-server` (new capability)
- Affected code: New workspace crates
- Dependencies: `viewpoint-core`, `viewpoint-cdp`, `tokio`, `serde_json`
