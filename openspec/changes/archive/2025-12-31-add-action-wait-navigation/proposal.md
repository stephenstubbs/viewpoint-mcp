# Change: Add Action Wait Navigation Support

## Why

When actions (like clicking a submit button or typing in a search box with submit) trigger page navigation, the current implementation returns immediately after the action completes. This causes issues where the browser closes or the next tool call fails because the navigation hasn't finished loading. For example, submitting a search on DuckDuckGo causes the browser to close before the results page loads.

Viewpoint 0.2.10 adds an "action wait navigation" feature that allows actions to automatically wait for any triggered navigation to complete before returning.

## What Changes

- Update `viewpoint-core` and `viewpoint-cdp` dependencies from `0.2.9` to `0.2.10`
- Integrate the action wait navigation feature into interaction tools:
  - `browser_click` - clicks may trigger navigation (links, submit buttons)
  - `browser_type` (with `submit: true`) - form submission often navigates
  - `browser_press_key` (e.g., Enter) - may trigger form submission
  - `browser_fill_form` - form fields may auto-submit
  - `browser_select_option` - selection may trigger navigation
- Follow playwright-mcp's pattern: after performing an action, wait for any resulting navigation or network activity to settle before returning

## Impact

- Affected specs: `browser-tools`
- Affected code:
  - `Cargo.toml` (workspace dependencies)
  - `crates/viewpoint-mcp/src/tools/browser_click.rs`
  - `crates/viewpoint-mcp/src/tools/browser_type.rs`
  - `crates/viewpoint-mcp/src/tools/browser_press_key.rs`
  - `crates/viewpoint-mcp/src/tools/browser_fill_form.rs`
  - `crates/viewpoint-mcp/src/tools/browser_select_option.rs`
  - Potentially a new shared utility module for the wait-for-completion logic
