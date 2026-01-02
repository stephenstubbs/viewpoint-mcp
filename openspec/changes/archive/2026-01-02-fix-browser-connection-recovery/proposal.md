# Change: Fix browser connection recovery after WebSocket loss

## Why

When the Chromium browser process dies unexpectedly (crash, timeout, killed), the MCP server's `BrowserState` retains a stale `initialized = true` flag. Subsequent tool calls fail with "WebSocket connection lost" errors indefinitely because `initialize()` returns early without re-launching the browser.

This manifests as zombie `<defunct>` chromium processes and a non-functional MCP server that requires restart.

## What Changes

- Detect `CdpError::ConnectionLost` errors in tool execution
- Reset `BrowserState` when connection loss is detected (clear contexts, browser ref, set `initialized = false`)
- Allow automatic re-initialization on the next tool call
- Add logging to aid debugging connection issues

## Impact

- Affected specs: `browser-state` (new spec for MCP browser state management)
- Affected code: 
  - `crates/viewpoint-mcp/src/browser/state.rs`
  - `crates/viewpoint-mcp/src/tools/*.rs` (error handling)

## Design Decisions

**Why not fix in viewpoint-core?**
- The core library correctly throws `CdpError::ConnectionLost` - it's doing its job
- Recovery semantics are consumer-specific (MCP has contexts, pages state to manage)
- Different consumers may want different recovery strategies (retry, fail fast, etc.)

**Why reset instead of reconnect?**
- Reconnecting to a dead browser process is impossible
- The MCP pattern is lazy initialization - simpler to reset and let next call reinitialize
- Cleaner state management with less edge cases
