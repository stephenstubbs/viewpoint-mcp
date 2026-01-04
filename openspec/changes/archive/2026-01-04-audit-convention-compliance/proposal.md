# Change: Audit Convention Compliance

## Why

A comprehensive audit of the viewpoint-mcp codebase against `project.md` conventions identified several issues requiring attention:

1. **One integration test file exceeds 500 lines** - `snapshot_tests.rs` is 758 lines
2. **18 clippy warnings** - Various pedantic lint issues not yet addressed
3. **Testing spec has placeholder purpose** - Spec created via archive needs purpose updated
4. **Inline JavaScript instead of `js!` macro** - Two tool files use raw `format!` strings for JavaScript instead of the `viewpoint_js::js!` macro

## What Changes

### 1. Large File Refactoring

Split `crates/viewpoint-mcp/tests/inspection/snapshot_tests.rs` (758 lines) into focused test modules:

| Current File | Lines | Proposed Split |
|--------------|-------|----------------|
| `snapshot_tests.rs` | 758 | `snapshot_basic_tests.rs`, `snapshot_cache_tests.rs`, `wait_tests.rs`, `evaluate_tests.rs` |

### 2. Clippy Warning Resolution

Address 18 warnings across these categories:

| Warning Type | Count | Fix |
|--------------|-------|-----|
| `use_self` | 5 | Replace type name with `Self` in impl blocks |
| `doc_markdown` | 5 | Add backticks around code in doc comments |
| `if_same_then_else` | 2 | Collapse identical if branches |
| `match_same_arms` | 1 | Remove redundant match arms |
| `let_else` | 1 | Use `let...else` pattern |
| `single_match` | 1 | Use `if let` instead of single-pattern match |
| `map_unwrap_or` | 1 | Use `map_or` instead of `map().unwrap_or()` |
| `pub_use_private` | 1 | Adjust visibility or move item |
| Other | 1 | Case-by-case fixes |

### 3. Testing Spec Purpose

Update `openspec/specs/testing/spec.md` with proper purpose statement (currently placeholder text from archive).

### 4. Use `viewpoint_js::js!` Macro for Inline JavaScript

Replace raw `format!` strings containing JavaScript with the `js!` macro from `viewpoint-js`:

| File | Current Approach | Required Change |
|------|-----------------|-----------------|
| `browser_wait_for.rs` | `format!(r#"() => document.body.innerText.includes(...)"#)` | Use `js!` macro with proper interpolation |
| `browser_network_requests.rs` | `format!(r"(() => {{ ... }})()")` | Use `js!` macro for the entire JavaScript block |

The `js!` macro provides:
- Compile-time JavaScript syntax validation
- Proper escaping and interpolation via `@{}` syntax
- Consistent code style across the codebase (already used in `browser_evaluate.rs`)

## Impact

- **Affected specs**: `testing`
- **Affected code**:
  - `tests/inspection/snapshot_tests.rs` - Split into smaller modules
  - `src/tools/browser_wait_for.rs` - Replace format! with js! macro
  - `src/tools/browser_network_requests.rs` - Replace format! with js! macro
  - Various source files - Clippy warning fixes
- **Risk**: Low - Test organization, lint fixes, and macro adoption have no functional impact

## Compliance Summary

### Already Compliant

- Workspace structure with multiple crates
- Edition 2024 in all Cargo.toml files
- Folder modules with `mod.rs` pattern
- No inline test modules (all use external `tests/` folders)
- thiserror for library errors, anyhow for CLI
- Error naming follows `{Module}Error` convention
- Result type aliases defined per module
- Pedantic clippy lints enabled in workspace
- Integration tests guarded with `#![cfg(feature = "integration")]`
- jj for version control (`.jj` directory present)
- Nix + direnv for build environment
- Code formatted with rustfmt

### Needs Work

- 1 file over 500 lines (test file)
- 18 clippy warnings
- 1 spec with placeholder purpose
- 2 files using inline JavaScript instead of `js!` macro
