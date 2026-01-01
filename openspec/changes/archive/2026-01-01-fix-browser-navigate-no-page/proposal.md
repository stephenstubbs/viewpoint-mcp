# Change: Fix browser_navigate failing after all pages closed

## Why

After closing all browser pages (via `browser_close` when only one page exists), `browser_navigate` fails with "Browser not available: No active page" instead of recovering gracefully. Users must work around this by manually calling `browser_tabs` with `action: "new"` before navigating again.

This is a bug: the browser context is still alive and healthy, only the pages have been closed. The navigate tool should auto-recover by creating a new page, matching user expectations.

## What Changes

- `browser_navigate` will auto-create a new page if the context exists but has no pages
- The fix is minimal: add page creation logic before attempting navigation
- No breaking changes to existing behavior

## Impact

- Affected specs: browser-tools
- Affected code: `crates/viewpoint-mcp/src/tools/browser_navigate.rs`
