# Change: Fix Browser Automation Bugs

## Why

Testing revealed several bugs in the browser automation tools that break key functionality:

1. **Element-scoped JavaScript evaluation returns empty** - `browser_evaluate` with an element ref returns `{}` instead of the actual result.

2. **Storage state save fails with CDP error** - `browser_context_save_storage` fails with "CDP protocol error -32001: Session with given id not found".

3. **Console messages not captured** - `browser_console_messages` returns "Console monitoring may not be enabled" because there's no console interception.

**Note**: The iframe element interaction bug is tracked separately in viewpoint-core (`fix-iframe-ref-resolution`). This proposal will be unblocked once viewpoint-core is updated and the dependency is bumped.

## What Changes

- **browser_evaluate**: Fix element-scoped evaluation to properly return results
- **browser_console_messages**: Implement proper console interception on page load
- **browser_context_save_storage**: Fix CDP session handling for storage state collection
- **Dependency**: Bump viewpoint-core version after `fix-iframe-ref-resolution` is merged

## Impact

- Affected specs: `browser-tools`
- Affected code:
  - `src/tools/browser_evaluate.rs`
  - `src/tools/browser_console_messages.rs`
  - `src/tools/browser_context_save_storage.rs`
  - `src/browser/context.rs`
  - `Cargo.toml` (dependency bump)
