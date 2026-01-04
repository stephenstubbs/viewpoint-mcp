# Change: Fix Browser State and Tool Issues

## Why
Testing revealed several bugs in viewpoint-mcp that affect reliability and user experience:
1. Snapshot parsing fails on minimal HTML pages with empty accessibility trees
2. Context list returns stale URLs instead of current page URLs (bug fix - spec already requires current URL)
3. File upload tool fails to locate file input elements

These issues prevent normal operation in common scenarios and reduce confidence in the tool's reliability.

**Update**: A fourth issue (Ctrl+Click tabs not tracked) has been fixed upstream in viewpoint-core 0.3.6 via unified CDP event-driven page tracking. The fix uses `Target.targetCreated` and `Target.targetDestroyed` CDP events to automatically track all pages. viewpoint-mcp will automatically benefit via the existing `context.pages()` API once updated to use viewpoint 0.3.6+.

## What Changes
- **Fix snapshot parsing for empty/minimal pages**: Handle null/empty accessibility tree responses gracefully instead of failing with deserialization errors
- **Fix context URL tracking** (bug fix): Query the actual current page URL dynamically via `page.url()` instead of relying on stale cached state. The existing spec already requires `currentUrl: URL of active page` - this is just fixing the implementation.
- **Fix file upload element detection**: Improve file input locator strategy to handle hidden inputs and use viewpoint's file chooser handling
- **Update viewpoint dependency**: Bump to 0.3.6+ to get automatic page tracking via CDP events

## Impact
- Affected specs: `accessibility-snapshots`, `browser-tools`
- Affected code:
  - `crates/viewpoint-mcp/src/snapshot/` - snapshot capture and parsing
  - `crates/viewpoint-mcp/src/browser/context.rs` - context URL tracking (bug fix)
  - `crates/viewpoint-mcp/src/tools/` - file upload tool implementation
  - `Cargo.toml` - viewpoint dependency version
