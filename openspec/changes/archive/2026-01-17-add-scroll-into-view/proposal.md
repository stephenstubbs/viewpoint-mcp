# Change: Add scroll element into view tool

## Why

Viewpoint 0.4.3 introduces the `scroll_into_view_if_needed()` API on locators, enabling explicit scrolling of elements into the viewport. This is useful when AI agents need to scroll to elements before taking screenshots, or when elements are outside the visible viewport and need to be brought into view for visibility-sensitive operations.

## What Changes

- Add new `browser_scroll_into_view` tool that scrolls an element into the visible viewport
- Update Viewpoint dependencies from 0.4.2 to 0.4.3 to access the new scroll API

## Impact

- Affected specs: `browser-tools`
- Affected code: 
  - `Cargo.toml` (workspace dependencies)
  - `crates/viewpoint-mcp/src/tools/` (new tool module)
  - `crates/viewpoint-mcp/src/tools/mod.rs` (tool registration)
