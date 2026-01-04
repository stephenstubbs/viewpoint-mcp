# Tasks

## 1. Cleanup Empty Directories

- [x] 1.1 Remove empty `crates/viewpoint-mcp/src/server/handler/` directory
- [x] 1.2 Remove empty `crates/viewpoint-mcp/src/server/protocol/` directory  
- [x] 1.3 Remove empty `crates/viewpoint-mcp/src/transport/sse/` directory
- [x] 1.4 Remove empty `crates/viewpoint-mcp/src/transport/stdio/` directory

## 2. Verification

- [x] 2.1 Run `cargo build --workspace` to confirm no breakage
- [x] 2.2 Run `cargo test --workspace` to confirm unit tests pass
- [x] 2.3 Run `cargo test --workspace --features integration` to confirm integration tests pass
