# Tasks: Add Accessibility Snapshots

## 1. Accessibility Tree Capture
- [x] 1.1 Add accessibility tree retrieval from Viewpoint Page
- [x] 1.2 Map CDP accessibility nodes to internal representation
- [x] 1.3 Handle frames and iframes in accessibility tree

## 2. Interactive Element Classification
- [x] 2.1 Implement Tier 1 classification (always interactive roles)
- [x] 2.2 Implement Tier 2 classification (contextually interactive)
- [x] 2.3 Implement Tier 3 exclusion (structural/non-interactive roles)
- [x] 2.4 Add element count threshold for compact output mode (>100 elements)
- [x] 2.5 Add `allRefs` parameter support for full ref output

## 3. Element Reference System (Hybrid Stability)
- [x] 3.1 Implement hash generation from `id` attribute (primary)
- [x] 3.2 Implement hash generation from `data-testid`/`name` attributes (secondary)
- [x] 3.3 Implement fallback hash from role + name + DOM path
- [x] 3.4 Implement ref format `e{hash}` with 4-6 char stable identifier
- [x] 3.5 Add context prefix for multi-context mode (e.g., `clean:e1a2b`)
- [x] 3.6 Implement reference-to-element lookup map

## 4. Stale Reference Detection
- [x] 4.1 Implement snapshot history (keep 1 previous snapshot)
- [x] 4.2 Implement ref validation on tool call
- [x] 4.3 Implement "element changed" detection with before/after details
- [x] 4.4 Implement "element removed" detection with similar element suggestions
- [x] 4.5 Implement "minor change" graceful handling with warning

## 5. Snapshot Formatting
- [x] 5.1 Implement tree-to-text formatter for LLM output
- [x] 5.2 Include element roles, names, and refs in output
- [x] 5.3 Handle text content truncation for large pages
- [x] 5.4 Add indentation for tree structure
- [x] 5.5 Add compact mode indicator when refs are limited

## 6. Integration
- [x] 6.1 Wire snapshot capture to `browser_snapshot` tool
- [x] 6.2 Add element ref resolution to action tools (click, type, etc.)
- [x] 6.3 Add snapshot caching for performance
- [x] 6.4 Integrate with multi-context system (context-prefixed refs)

## 7. Testing
- [x] 7.1 Add unit tests for element classification tiers
- [x] 7.2 Add unit tests for hash stability (same element = same hash)
- [x] 7.3 Add unit tests for stale reference detection scenarios
- [x] 7.4 Add unit tests for snapshot formatting
- [x] 7.5 Add integration test with real page accessibility tree
- [x] 7.6 Add integration test for ref stability across page refreshes
