# Change: Add Comprehensive Documentation

## Why

The codebase currently lacks comprehensive documentation:
- **Zero** `# Example` sections in doc comments across 130+ public items
- No per-crate README files explaining library usage
- Missing documentation on many public functions and methods
- No enforcement of documentation standards via clippy lints
- The root README focuses on CLI usage but lacks library API guidance

High-quality documentation is essential for:
- Enabling users to integrate the library into their projects
- Providing discoverable, rustdoc-generated API references
- Ensuring maintainability through self-documenting code
- Supporting LLM-assisted development with clear context

## What Changes

### 1. Enable Documentation Lints
- Add `#![warn(missing_docs)]` to enforce documentation on all public items
- This ensures future contributions maintain documentation standards

### 2. Add Rustdoc Examples to All Public APIs
- Add `# Examples` sections with runnable doctests to all public functions, structs, enums, and traits
- Cover the 5 main modules: `tools`, `snapshot`, `browser`, `server`, `transport`
- Approximately 130 public items need documentation improvements

### 3. Create Per-Crate README Files
- `crates/viewpoint-mcp/README.md` - Library usage guide with examples
- `crates/viewpoint-mcp-cli/README.md` - CLI-specific documentation

### 4. Enhance Root README
- Add library usage section with code examples
- Document the public API entry points
- Add architecture overview

## Impact

- **Affected code**: All 57 source files in `crates/viewpoint-mcp/src/`
- **Affected docs**: Root README, new crate READMEs
- **New spec**: `documentation` capability defining standards
- **Risk**: Low - documentation-only changes, no functional impact
- **Build impact**: Doctests will be compiled and run as part of `cargo test`
