# Design: Fix Browser Automation Bugs

## Context

During integration testing of the viewpoint-mcp browser automation server, three bugs were discovered in this project, plus one upstream bug in viewpoint-core (tracked separately).

## Goals

- Fix the three viewpoint-mcp bugs
- Bump viewpoint dependency after upstream fix is released
- Ensure proper test coverage for each fix

## Non-Goals

- Fixing viewpoint-core bugs (tracked in separate proposal)
- Adding new features beyond fixing existing functionality
- Performance optimizations

## Decision 1: Element-Scoped Evaluate Fix

### Problem
`browser_evaluate` with element ref returns empty `{}` instead of actual result.

### Root Cause Investigation Needed
The issue may be in:
1. The JavaScript wrapper construction
2. The `locator.evaluate()` implementation in viewpoint-core
3. Result serialization

### Solution
Debug and fix the evaluation chain. Most likely the issue is that `locator.evaluate()` is not receiving or passing the element correctly, or the result serialization is failing.

## Decision 2: Storage State Fix

### Problem
`storage_state()` fails with "CDP protocol error -32001: Session with given id not found".

### Root Cause
The `storage_state()` method in viewpoint-core uses page session IDs from `pages.read().await`, but these sessions may be stale or invalid.

### Solution
1. Validate page sessions before use
2. Skip pages with invalid/stale sessions
3. If no valid pages exist, return appropriate error with guidance

## Decision 3: Console Messages Implementation

### Problem
No console interception is implemented.

### Solution
Implement proper console message capture using CDP's `Runtime.consoleAPICalled` event.

**Design Decisions:**
- **Storage**: Per-page (messages are page-specific)
- **Buffer size**: 1000 messages max, with oldest-first eviction (matches Playwright's default)
- **Subscription**: Always enabled (low overhead, no opt-in required)

**Approach:**
1. Subscribe to `Runtime.consoleAPICalled` events when page is created
2. Store messages in per-page buffer with 1000 message limit
3. `browser_console_messages` retrieves stored messages filtered by level

```rust
// In browser/context.rs
const CONSOLE_BUFFER_MAX: usize = 1000;

pub struct PageState {
    // ... existing fields
    console_messages: VecDeque<ConsoleMessage>,
}

impl PageState {
    fn add_console_message(&mut self, msg: ConsoleMessage) {
        if self.console_messages.len() >= CONSOLE_BUFFER_MAX {
            self.console_messages.pop_front(); // Evict oldest
        }
        self.console_messages.push_back(msg);
    }
}
```

## Decision 4: Dependency Bump

### Problem
Iframe element interaction requires fix in viewpoint-core.

### Solution
After `fix-iframe-ref-resolution` is merged in viewpoint and a new version is released:
1. Bump viewpoint-core dependency in Cargo.toml
2. Run full integration test suite
3. Add integration tests for iframe element interaction

## Risks / Trade-offs

| Risk | Impact | Mitigation |
|------|--------|------------|
| Blocked on viewpoint-core release | Medium | Can implement/test other fixes in parallel |
| CDP session handling is complex | Low | Add proper error handling and fallbacks |
| Console message storage could grow large | Low | 1000 message limit with oldest-first eviction |

## Migration Plan

1. **Phase 1**: Fix evaluate, console, storage bugs (can proceed immediately)
2. **Phase 2**: Wait for viewpoint-core release with iframe fix
3. **Phase 3**: Bump dependency and add iframe interaction tests
4. **Phase 4**: Run full integration test suite

## Resolved Questions

1. **Console messages stored per-page** - Messages are page-specific, so storing per-page is the natural fit.
   
2. **Maximum buffer size: 1000 messages** - Matches Playwright's default, provides good balance between memory usage and message retention.

3. **Console subscription always enabled** - Low overhead, no need for opt-in complexity.
