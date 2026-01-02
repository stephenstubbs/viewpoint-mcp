# Tasks

## 1. Fix Implementation Gaps

### 1.1 browser_click Button and Modifier Support
- [x] 1.1.1 Add `MouseButton` enum mapping to viewpoint-core's mouse button types
- [x] 1.1.2 Implement right-click using `locator.click().button(Right)`
- [x] 1.1.3 Implement middle-click using `locator.click().button(Middle)`
- [x] 1.1.4 Add modifier key support by wrapping click with keyboard modifiers
- [x] 1.1.5 Add integration tests for right-click and middle-click
- [x] 1.1.6 Add integration tests for modifier key combinations (Ctrl+click, Shift+click)

### 1.2 browser_context_create Proxy Configuration (viewpoint 0.3.0 ready)
- [x] 1.2.1 Update viewpoint dependency to 0.3.0 in workspace Cargo.toml
- [x] 1.2.2 Update `BrowserState::create_context()` to accept `ProxyConfig` parameter
- [x] 1.2.3 Wire `ProxyInput` from tool to `ProxyConfig` for viewpoint-core context creation
- [x] 1.2.4 Update `browser_context_create` tool to pass proxy config to `BrowserState`
- [x] 1.2.5 Remove "proxy configuration noted but not yet applied" placeholder message
- [ ] 1.2.6 Add integration test verifying proxy configuration with mock proxy server

### 1.3 browser_context_save_storage Implementation (viewpoint 0.3.1 ready)
- [x] 1.3.1 Update viewpoint dependency to 0.3.1 (includes storage_state API)
- [x] 1.3.2 Implement using `context.storage_state().await` and `.save(path).await`
- [x] 1.3.3 Add `get_context(name)` method to `BrowserState` for named context access
- [x] 1.3.4 Add integration tests for storage state save (4 tests in tools_context.rs)

## 2. Add Missing Tests

### 2.1 Stale Reference Detection Tests
- [x] 2.1.1 Create `tests/stale_refs.rs` integration test file
- [x] 2.1.2 Test: element removed from page between snapshots
- [x] 2.1.3 Test: element role changed between snapshots
- [x] 2.1.4 Test: element name changed between snapshots (minor change handling)
- [x] 2.1.5 Test: ref from current snapshot works correctly

### 2.2 Multi-Context Ref Format Tests
- [x] 2.2.1 Add test: snapshot in named context has context-prefixed refs
- [x] 2.2.2 Add test: clicking with context-prefixed ref succeeds
- [x] 2.2.3 Add test: ref from one context fails in another context

### 2.3 Connection Loss Recovery Tests
- [x] 2.3.1 Create `tests/connection_recovery.rs` integration test file
- [x] 2.3.2 Test: tool call after browser process killed returns error (marked #[ignore])
- [x] 2.3.3 Test: subsequent tool call after reset triggers re-initialization
- [x] 2.3.4 Test: non-connection errors don't trigger reset

### 2.4 Error Path Tests
- [x] 2.4.1 Add test: `browser_type` with non-existent ref
- [x] 2.4.2 Add test: `browser_type` with invalid ref format
- [x] 2.4.3 Add test: `browser_fill_form` with one invalid ref in array
- [x] 2.4.4 Add test: `browser_drag` with invalid startRef
- [x] 2.4.5 Add test: `browser_evaluate` with ref on removed element

## 3. Test Edge Cases

### 3.1 Snapshot Format Edge Cases
- [x] 3.1.1 Add test: text content at exactly 100 characters (boundary)
- [x] 3.1.2 Add test: text content at 101 characters (truncation)
- [x] 3.1.3 Add test: verify truncated text ends with "..."
- [x] 3.1.4 Add test: page with exactly 100 interactive elements (threshold)
- [x] 3.1.5 Add test: page with exactly 101 interactive elements (compact mode)

### 3.2 Frame Handling Tests
- [x] 3.2.1 Add test: snapshot of page with iframe includes frame boundary marker
- [x] 3.2.2 Add test: elements inside iframe have refs
- [x] 3.2.3 Add test: clicking element inside iframe works

## 4. Documentation Updates

### 4.1 Update Spec Purpose Sections
- [x] 4.1.1 Update `accessibility-snapshots/spec.md` purpose section
- [x] 4.1.2 Update `browser-tools/spec.md` purpose section
- [x] 4.1.3 Update `browser-state/spec.md` purpose section
- [x] 4.1.4 Update `mcp-server/spec.md` purpose section

## 5. Viewpoint Library Changes (Separate Proposal)

These tasks require changes in `/home/stephenstubbs/Work/tooling/viewpoint`:
- [x] 5.1 ~~Create proposal for storage state export API~~ (completed in viewpoint 0.3.1)
- [x] 5.2 ~~Create proposal for proxy configuration in context creation~~ (completed in viewpoint 0.3.0)
- [x] 5.3 ~~Create proposal for stable element identification (hybrid refs)~~ (proposal created: add-stable-element-refs)
