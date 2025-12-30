# Change: Fix Element Locator Resolution

## Why
All element interaction tools (click, type, hover, drag, etc.) are fundamentally broken. They generate CSS selectors like `[data-ref='<hash>']` to locate elements, but these `data-ref` attributes **do not exist** on DOM elements. The refs are synthetic identifiers generated during accessibility snapshot processing and are never injected into the page DOM.

This causes all ref-based interactions to silently fail or timeout because the locators never match any elements.

**Affected tools (9 total):**
- `browser_click`
- `browser_type`
- `browser_hover`
- `browser_drag`
- `browser_fill_form`
- `browser_select_option`
- `browser_evaluate` (when targeting element)
- `browser_take_screenshot` (when targeting element)

## What Changes
1. **Upgrade viewpoint dependencies** from 0.2.8 to 0.2.9
2. **Use native ref resolution API** - viewpoint 0.2.9 now provides `page.locator_from_ref(ref_string)` and `page.element_from_ref(ref_string)` methods that directly resolve snapshot refs to DOM elements
3. **Update all interaction tools** to use the native `locator_from_ref()` API instead of non-existent `data-ref` selectors
4. **Remove custom workarounds** - no need for role+name based locators or custom tree traversal since viewpoint handles this internally

## Impact
- **Affected specs**: `accessibility-snapshots`, `browser-tools`
- **Affected code**:
  - `Cargo.toml` (workspace) - upgrade viewpoint-core and viewpoint-cdp to 0.2.9
  - `crates/viewpoint-mcp/src/tools/browser_click.rs` - use `locator_from_ref()`
  - `crates/viewpoint-mcp/src/tools/browser_type.rs` - use `locator_from_ref()`
  - `crates/viewpoint-mcp/src/tools/browser_hover.rs` - use `locator_from_ref()`
  - `crates/viewpoint-mcp/src/tools/browser_drag.rs` - use `locator_from_ref()`
  - `crates/viewpoint-mcp/src/tools/browser_fill_form.rs` - use `locator_from_ref()`
  - `crates/viewpoint-mcp/src/tools/browser_select_option.rs` - use `locator_from_ref()`
  - `crates/viewpoint-mcp/src/tools/browser_evaluate.rs` - use `locator_from_ref()` or `element_from_ref()`
  - `crates/viewpoint-mcp/src/tools/browser_take_screenshot.rs` - use `locator_from_ref()`

## Simplification vs Original Proposal
The original proposal required:
- Adding `find_element_by_ref()` to `AccessibilitySnapshot` for tree traversal
- Creating `tools/locator_helper.rs` for role-to-AriaRole mapping
- Building role+name locators manually

With viewpoint 0.2.9, these are **no longer needed** because:
- `page.locator_from_ref("e12345")` returns a `Locator` object directly
- `page.element_from_ref("e12345").await` returns an `ElementHandle` directly
- The viewpoint library handles the internal mapping from ref to DOM element
