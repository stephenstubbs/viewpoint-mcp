## 1. Dependency Update

- [x] 1.1 Update Viewpoint dependencies to 0.4.3 in workspace `Cargo.toml`

## 2. Implementation

- [x] 2.1 Create `browser_scroll_into_view.rs` tool module with parameters for `ref` and `element`
- [x] 2.2 Register the new tool in `tools/mod.rs`

## 3. Testing

- [x] 3.1 Add unit tests in `tools/tests/browser_scroll_into_view_tests.rs`
- [x] 3.2 Add integration test with real browser to verify scroll behavior
- [x] 3.3 Run `cargo test --workspace` (unit tests)
- [x] 3.4 Run `cargo test --workspace --features integration` (integration tests)

## 4. Validation

- [x] 4.1 Run `cargo clippy --workspace` and fix any warnings
- [x] 4.2 Run `cargo fmt --check` to verify formatting
