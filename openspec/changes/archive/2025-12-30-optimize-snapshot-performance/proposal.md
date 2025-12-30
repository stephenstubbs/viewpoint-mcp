# Change: Optimize Snapshot Performance

## Why
The `browser_snapshot` tool is slow because it always captures a fresh accessibility tree from the browser, even when a valid cached snapshot exists. The caching infrastructure (`get_cached_snapshot`, `cache_snapshot`, `invalidate_cache`) is already implemented in `ContextState` but the snapshot tool never uses it. This causes unnecessary latency for repeated snapshot requests within the 5-second cache TTL window.

## What Changes
- Enable snapshot caching in `browser_snapshot` tool to use `get_cached_snapshot()` when a valid cache exists
- Add `counts()` method to `SnapshotElement` for single-pass ref/element counting (avoids two tree traversals)
- Add tracing instrumentation spans for performance debugging and profiling
- Optimize string formatting with pre-allocated indent strings

## Impact
- Affected specs: `accessibility-snapshots`
- Affected code:
  - `crates/viewpoint-mcp/src/tools/browser_snapshot.rs` - use cache, add instrumentation
  - `crates/viewpoint-mcp/src/snapshot/element.rs` - add `counts()` method
  - `crates/viewpoint-mcp/src/snapshot/format.rs` - optimize indentation allocation
