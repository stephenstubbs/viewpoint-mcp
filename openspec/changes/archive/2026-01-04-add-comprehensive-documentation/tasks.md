## 1. Enable Documentation Lints

- [x] 1.1 Add `missing_docs = "warn"` to `[workspace.lints.rust]` in root `Cargo.toml`
- [x] 1.2 Add `#![warn(missing_docs)]` to `crates/viewpoint-mcp/src/lib.rs`
- [x] 1.3 Add `#![warn(missing_docs)]` to `crates/viewpoint-mcp-cli/src/main.rs`
- [x] 1.4 Run `cargo clippy --workspace` to identify all undocumented items

## 2. Document Core Library (`lib.rs`)

- [x] 2.1 Enhance `lib.rs` module doc with crate overview and quick start example
- [x] 2.2 Add examples to re-exported types

## 3. Document Browser Module

- [x] 3.1 Add examples to `BrowserState` struct and methods
- [x] 3.2 Add examples to `BrowserConfig` and builder pattern
- [x] 3.3 Add examples to `ContextState` and context management
- [x] 3.4 Document `ConsoleBuffer` and console message types
- [x] 3.5 Document `BrowserError` variants with error handling examples
- [x] 3.6 Ensure all public items in `browser/` have doc comments

## 4. Document Snapshot Module

- [x] 4.1 Add examples to `AccessibilitySnapshot::capture()`
- [x] 4.2 Add examples to `SnapshotElement` traversal
- [x] 4.3 Add examples to `ElementRef` parsing and formatting
- [x] 4.4 Document `SnapshotFormatter` configuration
- [x] 4.5 Document `StaleRefDetector` usage
- [x] 4.6 Document element classification (`ElementTier`, `classify_role`)
- [x] 4.7 Ensure all public items in `snapshot/` have doc comments

## 5. Document Tools Module

- [x] 5.1 Add examples to `Tool` trait implementation
- [x] 5.2 Add examples to `ToolRegistry` creation and lookup
- [x] 5.3 Document `ToolError` variants with handling examples
- [x] 5.4 Document `Capability` enum and capability checking
- [x] 5.5 Add brief doc comments to each tool struct (30 tools)
- [x] 5.6 Ensure all public items in `tools/` have doc comments

## 6. Document Server Module

- [x] 6.1 Add examples to `McpServer` creation and configuration
- [x] 6.2 Add examples to `ServerConfig` builder
- [x] 6.3 Document MCP protocol types in `protocol.rs`
- [x] 6.4 Document `ServerError` variants
- [x] 6.5 Ensure all public items in `server/` have doc comments

## 7. Document Transport Module

- [x] 7.1 Add examples to `StdioTransport` setup and running
- [x] 7.2 Add examples to `SseTransport` and `SseConfig`
- [x] 7.3 Document transport error types
- [x] 7.4 Ensure all public items in `transport/` have doc comments

## 8. Create Crate READMEs

- [x] 8.1 Create `crates/viewpoint-mcp/README.md` with library usage guide
- [x] 8.2 Create `crates/viewpoint-mcp-cli/README.md` with CLI reference
- [x] 8.3 Update root `README.md` with library usage section and architecture overview

## 9. Validation

- [x] 9.1 Run `cargo doc --workspace --no-deps` and verify no warnings
- [x] 9.2 Run `cargo test --workspace --doc` to verify all doctests pass
- [x] 9.3 Run `cargo clippy --workspace` to verify no `missing_docs` warnings
- [x] 9.4 Manual review of generated rustdoc for completeness
