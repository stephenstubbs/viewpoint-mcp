# Design: Fix Element Reference Format Mismatch

## Context

The viewpoint-mcp server panics when tools try to interact with elements because of a ref format mismatch:

1. **viewpoint-mcp** generates custom hash-based refs like `e2d2ce5` (6-char hex hash)
2. **viewpoint-core** expects refs like `e12345` (decimal backend node ID)

The viewpoint-core `AriaSnapshot` struct already provides refs in the correct format via its `node_ref` field:

```rust
/// Unique reference identifier for this element.
/// The ref follows the format `e{backendNodeId}` where
/// `backendNodeId` is the CDP backend node identifier.
#[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
pub node_ref: Option<String>,
```

## Goals / Non-Goals

**Goals:**
- Fix the panic when using element interaction tools
- Simplify the ref system by using viewpoint-core's native refs
- Maintain backward-compatible snapshot output format

**Non-Goals:**
- Changing the ref display format shown to users (still `e{id}`)
- Implementing custom hash-based stability (viewpoint-core handles this)
- Multi-context ref prefixes (can be added later if needed)

## Decisions

### Decision: Use viewpoint-core's native refs

**What:** Remove custom hash generation and use `aria.node_ref` directly from viewpoint-core's `AriaSnapshot`.

**Why:**
- viewpoint-core already generates refs in the format required by `locator_from_ref()`
- Eliminates code duplication and maintenance burden
- Refs are guaranteed to work with viewpoint-core's element resolution API
- Backend node IDs are stable within a page session

**Trade-off:** Backend node IDs are not stable across page reloads (unlike the hash-based approach). However:
- LLMs typically work with fresh snapshots anyway
- The stale ref detection system handles this case
- Simplicity outweighs theoretical stability benefits

### Decision: Simplify ElementRef to wrapper type

**What:** Change `ElementRef` from a hash-based struct to a simple wrapper around the viewpoint-core ref string.

**Why:**
- No need for custom hash computation
- Direct passthrough to `locator_from_ref()`
- Simpler code, fewer potential bugs

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Refs not stable across page reloads | Stale detection already handles this; LLMs take fresh snapshots |
| Multi-context prefixes not supported | Can be added as a separate enhancement if needed |
| Backend node IDs are numbers, not readable | Display format unchanged (`e{id}`); IDs are just as opaque as hashes to users |

## Migration Plan

1. Update `snapshot/capture.rs` to use `aria.node_ref` instead of generating hashes
2. Simplify `ElementRef` to store the viewpoint-core ref directly
3. Remove `RefGenerator` and hash computation code
4. Update stale detection to work with new ref format
5. Run integration tests to verify element interactions work

**Rollback:** Revert to hash-based refs if issues discovered (unlikely given this aligns with viewpoint-core's design)

## Open Questions

None - the approach is straightforward: use the ref format that viewpoint-core expects and provides.
