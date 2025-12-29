## 1. Implementation

- [x] 1.1 Create `register_all_tools()` function in `tools/registry.rs` that registers all 30 browser tools
- [x] 1.2 Update `McpServer::new()` to parse capabilities from config and call `register_all_tools()`
- [x] 1.3 Add unit test verifying tools are registered on server creation

## 2. Validation

- [x] 2.1 Run `cargo test --workspace` to verify unit tests pass
- [x] 2.2 Run `cargo test --workspace --features integration` to verify integration tests pass
- [x] 2.3 Manual test: run `mcp-viewpoint` and verify `tools/list` returns 26 core tools (30 minus 3 vision tools minus 1 pdf tool when not enabled)
