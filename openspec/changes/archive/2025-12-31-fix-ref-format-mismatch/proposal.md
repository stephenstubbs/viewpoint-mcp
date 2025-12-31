# Change: Fix Element Reference Format Mismatch

## Why

The MCP server crashes with a panic when attempting to interact with elements (click, type, etc.) because viewpoint-mcp generates custom hash-based refs (e.g., `e2d2ce5`) but viewpoint-core's `locator_from_ref()` expects refs in the format `e{backendNodeId}` where backendNodeId is a decimal number from CDP.

**Error observed:**
```
thread 'main' panicked at viewpoint-core-0.2.10/src/page/ref_resolution/mod.rs:216:14:
Invalid ref format. Refs must be in format 'e{backendNodeId}': EvaluationError("Invalid backend node ID in ref: invalid digit found in string")
```

## What Changes

- **Remove custom hash-based ref generation** in viewpoint-mcp
- **Use viewpoint-core's native `node_ref` field** from `AriaSnapshot` which already provides refs in the correct `e{backendNodeId}` format
- **Simplify the reference system** by eliminating the `RefGenerator` and custom hash computation
- **Update `ElementRef` type** to store the backend node ID directly

## Impact

- Affected specs: `accessibility-snapshots`
- Affected code:
  - `snapshot/reference.rs` - Simplify to use viewpoint-core refs
  - `snapshot/capture.rs` - Use `aria.node_ref` instead of generating hashes
  - `snapshot/element.rs` - Update ref storage
  - `snapshot/stale.rs` - Adjust stale detection for new ref format
  - All tool files using `locator_from_ref()` - No changes needed (already correct)
