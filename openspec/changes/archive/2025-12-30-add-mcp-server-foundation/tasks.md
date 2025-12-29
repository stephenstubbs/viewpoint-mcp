# Tasks: Add MCP Server Foundation

## 1. Project Setup
- [x] 1.1 Create workspace structure with `viewpoint-mcp` and `viewpoint-mcp-cli` crates
- [x] 1.2 Add workspace dependencies to root Cargo.toml
- [x] 1.3 Configure clippy lints and rustfmt in workspace

## 2. MCP Protocol Implementation
- [x] 2.1 Implement JSON-RPC message types (Request, Response, Notification)
- [x] 2.2 Implement MCP initialization handshake
- [x] 2.3 Implement tool listing (`tools/list`)
- [x] 2.4 Implement tool invocation (`tools/call`)
- [x] 2.5 Add unit tests for protocol handling

## 3. Transport Layer
- [x] 3.1 Implement stdio transport (stdin/stdout JSON-RPC)
- [x] 3.2 Implement SSE transport (HTTP server with SSE responses)
- [x] 3.3 Add transport selection via `--port` flag
- [x] 3.4 Implement API key authentication for SSE transport
- [x] 3.5 Implement auto-generation of API key when `--api-key` not provided
- [x] 3.6 Add unit tests for both transports
- [x] 3.7 Add unit tests for SSE authentication (valid key, invalid key, missing key)

## 4. Browser State Management
- [x] 4.1 Create BrowserState struct with multi-context support (HashMap of named contexts)
- [x] 4.2 Create ContextState struct for managing context/pages/proxy per context
- [x] 4.3 Implement lazy browser initialization on first tool call
- [x] 4.4 Add browser launch configuration from CLI args
- [x] 4.5 Add CDP endpoint connection support (`--cdp-endpoint`)
- [x] 4.6 Add graceful shutdown handling (close all contexts)
- [x] 4.7 Implement "default" context auto-creation for backward compatibility

## 5. CLI Implementation
- [x] 5.1 Add CLI argument parsing with clap
- [x] 5.2 Implement `--headless`, `--browser`, `--viewport-size` flags
- [x] 5.3 Implement `--cdp-endpoint`, `--user-data-dir` flags
- [x] 5.4 Implement `--port` flag for SSE mode
- [x] 5.5 Implement `--api-key` flag for SSE authentication
- [x] 5.6 Implement `--caps` flag for optional capabilities

## 6. Viewpoint Integration
- [x] 6.1 Integrate viewpoint-core v0.2.5 for real browser automation
- [x] 6.2 Wire up BrowserState to use viewpoint_core::Browser
- [x] 6.3 Wire up ContextState to use viewpoint_core::BrowserContext and Page
- [x] 6.4 Implement user_data_dir support for persistent browser profiles

## 7. Integration Testing
- [x] 7.1 Add integration test for stdio transport with mock client
- [x] 7.2 Add integration test for browser initialization and shutdown
- [x] 7.3 Add integration test for multi-context creation and isolation
- [x] 7.4 Add integration test for user_data_dir persistence
- [x] 7.5 Add integration test for CDP endpoint connection
- [x] 7.6 Add integration test for context with proxy configuration

## Notes

- Viewpoint-core v0.2.5 is now fully integrated with real browser automation
- Integration tests require Chromium to be installed (run with `cargo test --features integration`)
- Fixed CDP connection to support both HTTP endpoints (via `connect_over_cdp`) and WebSocket URLs (via `connect`)
- **Test counts:**
  - 23 unit tests (no browser required, run with `cargo test`)
  - 10 browser integration tests (require Chromium)
  - 4 stdio transport integration tests (test CLI binary)
  - **Total: 37 tests with `cargo test --features integration`**
