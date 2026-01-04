# Change: Align Codebase with project.md Conventions

## Why

An audit of the codebase against `openspec/project.md` revealed several minor deviations from the documented conventions. While the codebase largely follows the conventions, there are cleanup items related to empty directories and convention documentation gaps that should be addressed for consistency.

## What Changes

### Cleanup Required

1. **Remove empty module subdirectories**: The following empty directories exist alongside their `.rs` file counterparts (convention says folder modules only, but these directories are unused):
   - `crates/viewpoint-mcp/src/server/handler/` (empty)
   - `crates/viewpoint-mcp/src/server/protocol/` (empty)
   - `crates/viewpoint-mcp/src/transport/sse/` (empty)
   - `crates/viewpoint-mcp/src/transport/stdio/` (empty)

### Items Verified as Compliant

1. **Workspace structure**: Uses Cargo workspace with multiple crates (viewpoint-mcp, viewpoint-mcp-cli) - COMPLIANT
2. **Edition 2024**: All Cargo.toml files use `edition = "2024"` - COMPLIANT
3. **Folder modules**: All modules use `mod.rs` with subdirectories - COMPLIANT (except empty stubs)
4. **No inline tests**: All tests use `#[cfg(test)] mod tests;` referencing external test folders - COMPLIANT
5. **Maximum 500 lines per file**: Largest file is 499 lines (`snapshot_edge_cases.rs`) - COMPLIANT
6. **thiserror for errors**: Error modules use thiserror - COMPLIANT
7. **Error naming convention**: Error types follow `{Module}Error` pattern - COMPLIANT
8. **Result type aliases**: Modules define `type Result<T> = std::result::Result<T, {Module}Error>` - COMPLIANT
9. **Clippy lints**: Workspace enables pedantic lints - COMPLIANT
10. **Integration test feature flag**: All integration tests guarded with `#![cfg(feature = "integration")]` - COMPLIANT
11. **jj for VCS**: Repository uses jj (`.jj` directory present) - COMPLIANT
12. **Nix + direnv**: Uses `flake.nix` + `.envrc` - COMPLIANT

### Documentation Gaps (Not Addressed in This Change)

The following items in the codebase have patterns that could be documented in project.md, but are outside scope of this cleanup:

- Tool module organization (individual `.rs` files for each tool in `tools/` module)
- Hexagonal architecture details (ports/adapters boundaries not explicitly documented)

## Impact

- Affected specs: None (cleanup only, no behavior changes)
- Affected code: Empty directories to be removed
- Risk: Very low - removing empty directories has no functional impact
