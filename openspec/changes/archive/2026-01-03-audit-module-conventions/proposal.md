# Change: Audit Project Convention Compliance

## Why
An audit of the viewpoint-mcp codebase against the conventions defined in `project.md` revealed multiple violations that need to be addressed to maintain code quality and consistency:

1. **Inline test modules** (32 files) - Violates "No inline tests" convention
2. **Large files** (7 integration tests) - Exceed the 500-line maximum
3. **Clippy warnings** - Pedantic lint warnings not addressed
4. **Formatting issues** - Code not formatted with `rustfmt`

## What Changes

### 1. Test Module Organization (32 files)
Move all inline `#[cfg(test)] mod tests { ... }` blocks to dedicated `tests/` folder modules per the convention:
```
module_name/
├── mod.rs        # Public exports
├── tests/        # Unit tests (folder module)
│   ├── mod.rs
│   └── *.rs
```

### 2. Large File Refactoring (7 files)
Split integration test files exceeding 500 lines into smaller, focused test modules:
| File | Lines | Action |
|------|-------|--------|
| `tests/tools_interaction.rs` | 853 | Split by tool category |
| `tests/tools_inspection.rs` | 750 | Split by tool |
| `tests/tools_optional.rs` | 740 | Split by capability |
| `tests/tools_context.rs` | 616 | Split by context operation |
| `tests/browser_integration.rs` | 592 | Split by test category |
| `tests/tools_management.rs` | 577 | Split by tool |
| `tests/stale_refs.rs` | 493 | Keep (under limit) |

### 3. Clippy Compliance
Address pedantic clippy warnings:
- `needless_raw_string_hashes` - Remove unnecessary `#` from raw strings
- `doc_markdown` - Add backticks around code in doc comments
- `map_unwrap_or` - Use `map_or` instead of `map().unwrap_or()`

### 4. Formatting
Run `cargo fmt` to fix all formatting inconsistencies.

## Impact

- **Affected specs**: None (code organization only, no behavior changes)
- **Affected code**:
  - `src/tools/*.rs` - Remove inline tests (31 files)
  - `src/tools/tests/` - Add consolidated tests
  - `src/snapshot/reference.rs` - Remove inline tests
  - `src/snapshot/tests/` - Add reference tests
  - `tests/*.rs` - Refactor large files (6 files)
  - Multiple source files - Clippy and format fixes

## Files with Inline Tests (to be migrated)

### Tools Module (31 files)
All `browser_*.rs` files plus `registry.rs`

### Snapshot Module (1 file)  
`reference.rs`

## Modules Already Following Convention
- `browser/mod.rs` -> `browser/tests/`
- `server/mod.rs` -> `server/tests/`
- `snapshot/mod.rs` -> `snapshot/tests/`
- `transport/mod.rs` -> `transport/tests/`
