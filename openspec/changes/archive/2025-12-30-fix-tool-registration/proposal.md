# Change: Fix tool registration in MCP server

## Why
The MCP server starts successfully and responds to protocol messages, but `tools/list` returns an empty array because `McpServer::new()` creates an empty `ToolRegistry` without registering any tools. All 30 browser tools exist as structs but are never added to the registry.

## What Changes
- Add a helper function to register all browser tools with the registry
- Modify `McpServer::new()` to register all tools during initialization
- Parse capability flags to enable optional tools (vision, pdf)

## Impact
- Affected specs: mcp-server (tool registration behavior)
- Affected code: `crates/viewpoint-mcp/src/server/protocol.rs`
