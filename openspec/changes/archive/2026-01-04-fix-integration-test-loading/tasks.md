# Tasks

## 1. Fix Integration Test Loading

- [x] 1.1 Create `tests/context.rs` entry point that declares `mod context;`
- [x] 1.2 Create `tests/interaction.rs` entry point that declares `mod interaction;`
- [x] 1.3 Create `tests/inspection.rs` entry point that declares `mod inspection;`
- [x] 1.4 Create `tests/optional.rs` entry point that declares `mod optional;`
- [x] 1.5 Create `tests/tools_management.rs` entry point that declares `mod tools_management;`
- [x] 1.6 Create `tests/browser_integration.rs` entry point that declares `mod browser_integration;`
- [x] 1.7 Create `tests/stale_refs.rs` entry point that declares `mod stale_refs;`

## 2. Move Inline Tests from format.rs

- [x] 2.1 Create `src/snapshot/tests/format_tests.rs` with tests moved from `format.rs`
- [x] 2.2 Add `mod format_tests;` to `src/snapshot/tests/mod.rs`
- [x] 2.3 Remove inline `#[cfg(test)] mod tests { ... }` block from `src/snapshot/format.rs`

## 3. Verification

- [x] 3.1 Run `cargo test --workspace` and verify all unit tests pass
- [x] 3.2 Run `cargo test --workspace --features integration` and verify integration tests from subdirectories are now discovered and run
- [x] 3.3 Verify test count increased (should see tests from context, interaction, etc. modules)
