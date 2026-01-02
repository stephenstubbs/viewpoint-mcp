# Change: Audit and Address Spec-Implementation Gaps

## Why

A comprehensive audit of the viewpoint-mcp codebase against its specifications revealed several implementation gaps and areas where testing coverage could be strengthened. While the core functionality is solid (174 unit tests and 150+ integration tests pass), there are specific spec requirements that need attention to ensure full compliance and reliability.

## What Changes

### 1. Implementation Gaps Requiring Fixes

#### browser_click Tool - Incomplete Feature Support
- **button parameter**: The `button` field is parsed but marked with `#[allow(dead_code)]` - right/middle click not implemented
- **modifiers parameter**: The `modifiers` field is parsed but marked with `#[allow(dead_code)]` - modifier keys not applied during clicks
- **Spec reference**: `browser-tools/spec.md` - "Supports left/right/middle click, double-click, and modifier keys"

#### browser_context_create Tool - Proxy Support Now Available
- **proxy parameter**: Accepted in schema but proxy configuration not applied to viewpoint-core context
- **Status**: viewpoint 0.3.0 now includes the context proxy API - implementation can proceed
- **storageState parameter**: Accepted but returns placeholder success (viewpoint-core storage state API needs implementation in viewpoint library first)
- **Spec reference**: `browser-tools/spec.md` - Context Management Tools section

#### browser_context_save_storage Tool - Placeholder Implementation
- Currently returns success message without actually saving storage state
- **Dependency**: Requires viewpoint-core to expose storage state export API
- **Spec reference**: `browser-tools/spec.md` - "Save context storage state" scenario

#### Element Reference Stability
- Current refs use CDP backendNodeId (`e{backendNodeId}`) which changes on page refresh
- Spec requires: "stable identifier from explicit IDs" and "stable identifier from test attributes"
- The hybrid identification strategy (id/data-testid/name prioritization) is not implemented
- **Spec reference**: `accessibility-snapshots/spec.md` - Element Reference System requirement

### 2. Missing Test Coverage

#### Stale Reference Detection Tests
- `StaleRefDetector` has implementation but no dedicated integration tests
- Scenarios to test:
  - "Reference exists but element changed" (role/name changes)
  - "Graceful handling of minor changes" 
  - "Snapshot history for comparison"
- **Spec reference**: `accessibility-snapshots/spec.md` - Stale Reference Detection requirement

#### Context-Prefixed Refs Tests
- Multi-context mode with refs prefixed by context name (e.g., `clean:e1a2b`)
- No integration tests verifying ref format in multi-context scenarios
- **Spec reference**: `accessibility-snapshots/spec.md` - "In multi-context mode, refs are prefixed with context name"

#### Connection Loss Recovery Tests
- `BrowserState::handle_potential_connection_loss()` is implemented
- No integration tests simulating actual browser crash/disconnect
- Current test `test_browser_cdp_endpoint_connection` doesn't test recovery
- **Spec reference**: `browser-state/spec.md` - Connection Loss Recovery requirement

### 3. Test Robustness Improvements

#### Add Missing Error Path Tests
- `browser_type` with invalid ref (tests exist for click but not type)
- `browser_fill_form` with mixed valid/invalid refs
- `browser_drag` with mismatched element types
- `browser_evaluate` with element ref on non-existent element

#### Add Edge Case Tests
- Very long text content truncation (100 char limit per spec)
- Compact mode threshold (>100 interactive elements)
- Frame boundary markers in snapshots
- Dialog handling with concurrent dialogs

### 4. Documentation Gaps

#### Spec Purpose Sections
- All four specs have placeholder "TBD - created by archiving" purpose sections
- These should be updated with meaningful purpose descriptions

## Impact

- Affected specs: All four (`accessibility-snapshots`, `browser-tools`, `browser-state`, `mcp-server`)
- Affected code:
  - `tools/browser_click.rs` - Add button/modifier support
  - `tools/browser_context_create.rs` - Add proxy wiring
  - `tools/browser_context_save_storage.rs` - Proper error handling
  - `snapshot/reference.rs` - Hybrid identification (depends on viewpoint-core)
  - `tests/` - New test files for coverage gaps

## Validation

After implementation:
- All existing tests continue to pass
- New tests for identified gaps pass
- `cargo clippy --workspace --all-targets` passes
- `cargo test --workspace --features integration` passes with new coverage

## Dependencies on viewpoint Library

**Resolved Dependencies:**
- **Proxy configuration (viewpoint 0.3.0)**: Now available via `BrowserContext` API - implemented in viewpoint-mcp
- **Storage state export (viewpoint 0.3.1)**: `BrowserContext::storage_state()` API now available - implemented in viewpoint-mcp
- **Page-scoped element refs (viewpoint 0.3.1)**: Refs now use format `c{ctx}p{page}f{frame}e{counter}` preventing cross-context/page misuse

**Pending in viewpoint (proposal created: add-stable-element-refs):**
- **Stable element identification**: Enhanced refs with CSS selector, ARIA signature, and test-id fallbacks for surviving DOM mutations
