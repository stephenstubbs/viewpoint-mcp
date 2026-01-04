# Proposal: Track Page Activation Events

## Problem

When users interact with browser tabs directly (clicking on tabs in the browser UI), viewpoint-mcp's `active_page_index` and `current_url` get out of sync with the actual browser state. This causes:

1. `browser_close` reports the wrong URL in its message
2. `browser_tabs list` may show incorrect `[active]` marker
3. Operations intended for the "current" tab may affect the wrong page

The root cause: viewpoint-mcp only updates `active_page_index` when `switch_page()` is called programmatically via the `browser_tabs select` tool, not when users click tabs directly.

## Dependency

This change depends on viewpoint-core's `track-page-activation` change, which adds the `on_page_activated` event.

## Solution

Subscribe to viewpoint-core's `on_page_activated` event to keep `active_page_index` and `current_url` synchronized:

1. **Subscribe to `on_page_activated`** in `ContextState::new()`
2. **Update `active_page_index`** by finding the activated page's position in the pages list
3. **Update `current_url`** from the activated page
4. **Invalidate snapshot cache** since the active page changed

This follows the same pattern as the existing `on_page` subscription for console buffer setup.

## Scope

- Subscribe to `on_page_activated` in `ContextState::new()`
- Store handler ID to keep subscription alive
- Update state when activation events fire

## Acceptance Criteria

1. User clicks tab in browser UI -> `active_page_index` updates
2. User clicks tab in browser UI -> `current_url` updates
3. `browser_close` reports the correct URL after user tab switch
4. `browser_tabs list` shows correct `[active]` marker after user tab switch

## Risk Assessment

- **Low risk**: Follows established `on_page` subscription pattern
- **Dependency**: Requires viewpoint-core `track-page-activation` to be completed first
- **No breaking changes**: Internal state management only
