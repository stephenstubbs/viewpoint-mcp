# Tasks: Track Page Activation Events

## 0. Dependency

- [x] 0.1 Complete viewpoint-core `track-page-activation` change first
- [x] 0.2 Update viewpoint-core dependency version in Cargo.toml (0.4.0 -> 0.4.1)

## 1. Subscribe to Page Activation Events

- [x] 1.1 Add `_page_activated_handler_id: HandlerId` field to `ContextState`
- [x] 1.2 Subscribe to `context.on_page_activated()` in `ContextState::new()`
- [x] 1.3 In handler: store activated page's target_id for later index lookup
- [x] 1.4 In handler: update `active_page_index` via `sync_active_page_index()`
- [x] 1.5 In handler: update `current_url` from activated page's URL
- [x] 1.6 In handler: invalidate snapshot cache via `cache_invalidated` flag

## 2. State Management

- [x] 2.1 Create `SharedPageState` struct with thread-safe fields
- [x] 2.2 Add `active_page_index: AtomicUsize` to `SharedPageState`
- [x] 2.3 Add `current_url: RwLock<Option<String>>` to `SharedPageState`
- [x] 2.4 Add `cache_invalidated: RwLock<bool>` to `SharedPageState`
- [x] 2.5 Add `activated_target_id: RwLock<Option<String>>` for deferred index lookup
- [x] 2.6 Update `active_page_index()` to sync pending activation before returning
- [x] 2.7 Update all tools to use async `active_page_index().await`
- [x] 2.8 Update all tools to use `current_url()` and `set_current_url()` methods
- [x] 2.9 Update `get_cached_snapshot()` to check `cache_invalidated` flag
- [x] 2.10 Update `cache_snapshot()` to be async

## 3. Testing

- [x] 3.1 Run `cargo test --workspace` - 188 unit tests pass
- [x] 3.2 Run `cargo test --workspace --features integration` - all integration tests pass
- [x] 3.3 Fix test files to use new async `current_url()` method
