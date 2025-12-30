# Tasks

## 1. Enable Snapshot Caching
- [x] 1.1 Modify `browser_snapshot.rs` to check `context.get_cached_snapshot()` before capturing
- [x] 1.2 Cache newly captured snapshots with `context.cache_snapshot(snapshot)`
- [x] 1.3 Handle `allRefs: true` parameter (bypass cache when requesting different ref mode)

## 2. Optimize Tree Counting
- [x] 2.1 Add `counts() -> (usize, usize)` method to `SnapshotElement` for single-pass counting
- [x] 2.2 Update `browser_snapshot.rs` to use `counts()` instead of separate `ref_count()` and `element_count()` calls

## 3. Add Performance Instrumentation
- [x] 3.1 Add `tracing::instrument` to `browser_snapshot` execute method
- [x] 3.2 Add debug spans for `browser_initialize`, `capture_snapshot`, and `format_snapshot` phases
- [x] 3.3 Log cache hit/miss for debugging

## 4. Optimize Formatter (Optional)
- [x] 4.1 Pre-allocate indent strings up to a reasonable max depth
- [x] 4.2 Use pre-allocated output buffer with estimated capacity

## 5. Validation
- [x] 5.1 Run unit tests: `cargo test --workspace`
- [x] 5.2 Run integration tests: `cargo test --workspace --features integration`
- [x] 5.3 Manual testing with DuckDuckGo to verify cache hits work correctly
