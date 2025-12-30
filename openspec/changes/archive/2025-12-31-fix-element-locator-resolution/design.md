# Design: Element Locator Resolution

## Problem Analysis

### Current (Broken) Flow
1. `browser_snapshot` captures accessibility tree and assigns refs based on element attributes
2. User calls `browser_click` with `ref: "e1a2b3"`
3. Tool parses ref, extracts hash `1a2b3`
4. Tool creates CSS selector `[data-ref='1a2b3']`
5. **FAILS**: No elements have `data-ref` attributes - they don't exist in DOM

### Root Cause
The refs are synthetic identifiers computed from element properties (id, test-id, name, role, accessible name, DOM path). They are stored in the `AccessibilitySnapshot`'s ref map, but are never added to the actual DOM elements.

## Solution Design

### Approach: Native Viewpoint Ref Resolution API

Viewpoint 0.2.9 introduces native support for resolving snapshot refs to DOM elements:

- `page.locator_from_ref(ref_string)` - Returns a `Locator` with auto-waiting
- `page.element_from_ref(ref_string).await` - Returns an `ElementHandle` directly

This eliminates the need for custom role+name mapping or tree traversal.

### Required Changes

#### 1. Upgrade Viewpoint Dependencies

In workspace `Cargo.toml`:
```toml
viewpoint-core = "0.2.9"
viewpoint-cdp = "0.2.9"
```

#### 2. Update Tool Execute Methods

Each tool's execute method changes from:

```rust
// OLD (broken)
let selector = format!("[data-ref='{}']", element_ref.hash);
let locator = page.locator(&selector);
locator.click().await?;
```

To:

```rust
// NEW (working with viewpoint 0.2.9)
let locator = page.locator_from_ref(&input.ref);
locator.click().await?;
```

For tools that need an `ElementHandle` (like `browser_evaluate`):

```rust
// For ElementHandle access
let element = page.element_from_ref(&input.ref).await?;
element.evaluate(function).await?;
```

### API Details (from viewpoint 0.2.9)

From viewpoint's advanced-locators spec:

```rust
// Get a Locator (auto-waiting, chainable)
let locator = page.locator_from_ref("e12345");
await locator.click().await;

// Get an ElementHandle (direct DOM access)
let element_handle = page.element_from_ref("e12345").await;
```

**Error Handling:**
- **Invalid Ref**: Providing an invalid or malformed ref string returns an appropriate error
- **Stale Ref**: Providing a ref for an element removed from DOM returns an error indicating the element no longer exists

**Ref Format:**
Ref strings are opaque and protocol-agnostic, not exposing underlying implementation details like CDP specifics.

### Simplifications

The following originally-planned components are **no longer needed**:

1. ~~`find_element_by_ref()` method on `AccessibilitySnapshot`~~ - viewpoint handles lookup internally
2. ~~`tools/locator_helper.rs` module~~ - no role-to-AriaRole mapping needed
3. ~~Role+name locator building logic~~ - viewpoint uses its internal resolution mechanism
4. ~~DOM path to nth-matching fallback~~ - handled by viewpoint

### Error Messages

Tool error messages should be updated to reflect the new resolution mechanism:

```rust
// Example error handling
match page.locator_from_ref(&ref_string).click().await {
    Ok(_) => Ok(/* success */),
    Err(e) if e.is_stale_ref() => Err(ToolError::StaleRef {
        ref_string,
        suggestion: "Element no longer exists. Take a new snapshot to see current page state.",
    }),
    Err(e) if e.is_invalid_ref() => Err(ToolError::InvalidRef {
        ref_string,
        expected_format: "e{hash} (e.g., e1a2b3)",
    }),
    Err(e) => Err(ToolError::from(e)),
}
```

### Migration Benefits

1. **Simpler codebase** - No custom tree traversal or role mapping code
2. **Maintained by viewpoint** - Resolution logic improvements come from upstream
3. **Better error handling** - Viewpoint provides meaningful stale/invalid ref errors
4. **Auto-waiting** - `Locator` objects from `locator_from_ref()` have built-in waiting
5. **Consistency** - Same resolution logic used by viewpoint tests and MCP tools

### Alternatives Considered

#### Alternative 1: Inject data-ref attributes into DOM (Original Problem)
- Modify page DOM to add `data-ref` attributes
- **Rejected**: Modifies page state, could break JavaScript, security concerns

#### Alternative 2: Role+Name Locators (Original Proposal)
- Build locators using `get_by_role(role).with_name(name)`
- **Superseded**: Viewpoint 0.2.9 now provides native ref resolution, making this unnecessary

#### Alternative 3: XPath from DOM path
- Convert the internal `dom_path` to XPath selector
- **Rejected**: DOM path is index-based (`/0/1/2`), fragile to page structure changes

### Selected Approach
**Native viewpoint ref resolution API** because:
- Uses existing viewpoint-core API (`locator_from_ref`, `element_from_ref`)
- No custom code to maintain
- Upstream improvements benefit this project automatically
- Consistent with how viewpoint handles refs internally
