# Tasks: Fix Browser Automation Bugs

## 1. Fix Element-Scoped Evaluate

- [x] 1.1 Debug `locator.evaluate()` to trace element passing and result serialization
- [x] 1.2 Fix JavaScript wrapper or locator evaluation chain (used `js!` macro with raw interpolation)
- [x] 1.3 Add integration test for element-scoped evaluate returning object
- [x] 1.4 Add integration test for element-scoped evaluate returning string
- [x] 1.5 Add integration test for element-scoped evaluate returning null
- [x] 1.6 Add integration test for element-scoped evaluate modifying element

## 2. Fix Storage State Save

- [x] 2.1 Identify which CDP command uses invalid session (fixed in viewpoint 0.3.5)
- [x] 2.2 Add session validation before storage state collection (fixed in viewpoint 0.3.5)
- [x] 2.3 Skip pages with invalid/stale sessions gracefully (fixed in viewpoint 0.3.5)
- [x] 2.4 Integration test exists: `test_context_save_storage_basic`

## 3. Implement Console Message Capture

- [x] 3.1 Add `ConsoleMessage` type to browser module (`browser/console.rs`)
- [x] 3.2 Add per-page console message buffer (VecDeque, max 1000)
- [x] 3.3 Subscribe to `page.on_console()` on page creation
- [x] 3.4 Update `browser_console_messages` to return stored messages with level filtering
- [x] 3.5 Add integration test: capture console.log, console.error, console.warn
- [x] 3.6 Add integration test: filter by level (error, warning, debug)
- [x] 3.7 Add integration test: empty page with no console output

## 4. Bump Viewpoint Dependency

- [x] 4.1 Wait for viewpoint-core release with `fix-iframe-ref-resolution` (v0.3.5 released)
- [x] 4.2 Update Cargo.toml with new viewpoint-core version (0.3.5)
- [x] 4.3 Add integration test: click element inside iframe
- [x] 4.4 Add integration test: type into input inside iframe
- [x] 4.5 Add integration test: nested iframe interaction
- [x] 4.6 Add integration test: verify iframe elements have frame ID in ref
- [x] 4.7 Run full integration test suite

## Dependencies

- Tasks 1.x, 2.x, 3.x can run in parallel
- Task 4.x blocked on viewpoint-core release ✅ RELEASED

## Validation

```bash
# Run unit tests
cargo test --workspace

# Run integration tests (requires Chromium)
cargo test --workspace --features integration
```

## Summary of Changes

### Files Modified
- `Cargo.toml` - bumped viewpoint deps to 0.3.5, added viewpoint-js
- `crates/viewpoint-mcp/Cargo.toml` - added viewpoint-js dependency
- `crates/viewpoint-mcp/src/browser/mod.rs` - exported console module
- `crates/viewpoint-mcp/src/browser/console.rs` - NEW: console message buffer
- `crates/viewpoint-mcp/src/browser/context.rs` - added console buffer management
- `crates/viewpoint-mcp/src/tools/browser_evaluate.rs` - fixed element evaluation with js! macro
- `crates/viewpoint-mcp/src/tools/browser_console_messages.rs` - use stored messages
- `crates/viewpoint-mcp/tests/inspection/console_network_tests.rs` - updated console tests
- `crates/viewpoint-mcp/tests/inspection/snapshot_tests.rs` - added element-scoped evaluate tests
- `crates/viewpoint-mcp/tests/interaction/iframe_tests.rs` - NEW: iframe interaction tests
- `crates/viewpoint-mcp/tests/interaction.rs` - added iframe_tests module
