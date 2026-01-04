# Change: Fix Integration Test Loading and Project.md Compliance

## Why

The project has two issues violating `project.md` conventions:

1. **Integration tests in subdirectories are not being run**: The `tests/` directory contains 7 subdirectories (`context/`, `interaction/`, `inspection/`, `optional/`, `tools_management/`, `browser_integration/`, `stale_refs/`) with `mod.rs` files and test modules, but Cargo only runs `.rs` files directly in `tests/` as integration test binaries. These subdirectory modules are never loaded, meaning hundreds of integration tests are silently skipped.

2. **Inline test module in `format.rs`**: The file `src/snapshot/format.rs` contains an inline `#[cfg(test)] mod tests { ... }` block, violating the project.md rule: "No inline tests (`#[cfg(test)] mod tests` blocks)".

## What Changes

- Add entry point `.rs` files in `tests/` for each subdirectory module (`context.rs`, `interaction.rs`, `inspection.rs`, `optional.rs`, `tools_management.rs`, `browser_integration.rs`, `stale_refs.rs`)
- Move inline tests from `src/snapshot/format.rs` to `src/snapshot/tests/format_tests.rs`

## Impact

- Affected specs: None (testing infrastructure only)
- Affected code:
  - `crates/viewpoint-mcp/tests/` - Add 7 new entry point files
  - `crates/viewpoint-mcp/src/snapshot/format.rs` - Remove inline tests
  - `crates/viewpoint-mcp/src/snapshot/tests/` - Add format tests
