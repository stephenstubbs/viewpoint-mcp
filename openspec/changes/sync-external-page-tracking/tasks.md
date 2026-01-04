## 0. Dependency
- [x] 0.1 Complete viewpoint-core `fix-pages-return-type` change first
- [x] 0.2 Update viewpoint-core dependency version in Cargo.toml

## 1. Remove Duplicate Page Tracking
- [x] 1.1 Remove `pages: Vec<Page>` from `ContextState`
- [x] 1.2 Update `page_count()` to call `context.pages().await.len()`
- [x] 1.3 Update `active_page()` to get from `context.pages().await`
- [x] 1.4 Update `new_page()` - page is now automatically tracked by viewpoint-core
- [x] 1.5 Update `close_page()` to work with context's page list
- [x] 1.6 Update `switch_page()` to validate against context's pages

## 2. Console Buffer Tracking
- [x] 2.1 Change `console_buffers` from `Vec` to `HashMap<String, SharedConsoleBuffer>`
- [x] 2.2 Subscribe to `on_page` for console buffer setup
- [x] 2.3 Update `active_console_buffer()` to look up by target_id

## 3. Testing
- [ ] 3.1 Test `browser_tabs list` shows external tabs
- [ ] 3.2 Test `browser_tabs select` works with external tabs  
- [ ] 3.3 Test console capture for external pages
- [x] 3.4 Run `cargo test --workspace`
- [x] 3.5 Run `cargo test --workspace --features integration`
