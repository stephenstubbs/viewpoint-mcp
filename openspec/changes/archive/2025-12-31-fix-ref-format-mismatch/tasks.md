# Tasks

## 1. Update Snapshot Capture

- [x] 1.1 Update `snapshot/capture.rs` to use `aria.node_ref` from viewpoint-core instead of generating custom hashes
- [x] 1.2 Remove calls to `RefGenerator::generate()` in `convert_aria_snapshot()`
- [x] 1.3 Store the viewpoint-core ref directly in `element.element_ref`

## 2. Simplify Reference Types

- [x] 2.1 Update `ElementRef` in `snapshot/reference.rs` to store viewpoint-core ref string directly
- [x] 2.2 Update `ElementRef::parse()` to accept the viewpoint-core format
- [x] 2.3 Remove `RefGenerator` struct and hash computation methods
- [x] 2.4 Update `ElementRef::to_ref_string()` to return the ref as-is

## 3. Update Element Type

- [x] 3.1 Update `SnapshotElement.element_ref` field type if needed
- [x] 3.2 Update `ref_map` in `AccessibilitySnapshot` to use viewpoint-core refs as keys

## 4. Update Stale Detection

- [x] 4.1 Update `StaleRefDetector` to work with viewpoint-core ref format
- [x] 4.2 Update `validate_ref()` logic for new ref format

## 5. Verification

- [x] 5.1 Run unit tests: `cargo test --workspace`
- [x] 5.2 Run integration tests: `cargo test --workspace --features integration`
- [x] 5.3 Manual test: Navigate to a page, take snapshot, interact with elements
- [x] 5.4 Verify error handling for stale/invalid refs works correctly

## 6. Dependency Update (Added)

- [x] 6.1 Update viewpoint-core from 0.2.10 to 0.2.11 (fixes Frame.aria_snapshot() ref issue)
- [x] 6.2 Revert workaround: change `page.aria_snapshot()` back to `page.aria_snapshot_with_frames()`
