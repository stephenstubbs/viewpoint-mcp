## 1. Split Large Test File

- [x] 1.1 Create `tests/inspection/snapshot_basic_tests.rs` with basic snapshot tests (lines 1-173)
- [x] 1.2 Create `tests/inspection/snapshot_cache_tests.rs` with caching tests (lines 174-262)
- [x] 1.3 Create `tests/inspection/wait_tests.rs` with wait_for tests (lines 263-331)
- [x] 1.4 Create `tests/inspection/evaluate_tests.rs` with evaluate tests (lines 332-758)
- [x] 1.5 Update `tests/inspection.rs` entry point to reference new modules
- [x] 1.6 Remove original `snapshot_tests.rs`
- [x] 1.7 Verify all tests pass with `cargo test --features integration -p viewpoint-mcp --test inspection`

## 2. Fix Clippy Warnings

- [x] 2.1 Fix `use_self` warnings in `browser/console.rs` (5 instances)
- [x] 2.2 Fix `doc_markdown` warnings - add backticks around code in docs (5 instances)
- [x] 2.3 Fix `if_same_then_else` - collapse identical if branches (2 instances)
- [x] 2.4 Fix `match_same_arms` in `browser/console.rs` - remove redundant arm
- [x] 2.5 Fix remaining clippy warnings (let_else, single_match, map_unwrap_or, pub_use_private)
- [x] 2.6 Run `cargo clippy --workspace` to verify zero warnings

## 3. Update Testing Spec

- [x] 3.1 Update `openspec/specs/testing/spec.md` with proper purpose statement

## 4. Replace Inline JavaScript with `js!` Macro

- [x] 4.1 Add `use viewpoint_js::js;` import to `browser_wait_for.rs`
- [x] 4.2 Replace `format!(r#"() => document.body.innerText.includes("{escaped_text}")"#)` with `js!` macro
- [x] 4.3 Replace `format!(r#"() => !document.body.innerText.includes("{escaped_text}")"#)` with `js!` macro
- [x] 4.4 Add `use viewpoint_js::js;` import to `browser_network_requests.rs`
- [x] 4.5 Replace the Performance API JavaScript block with `js!` macro
- [x] 4.6 Verify all affected tools work correctly with integration tests

## 5. Validation

- [x] 5.1 Run `cargo test --workspace` to verify unit tests
- [x] 5.2 Run `cargo test --workspace --features integration` to verify integration tests
- [x] 5.3 Run `cargo clippy --workspace` to verify no warnings
- [x] 5.4 Run `openspec validate audit-convention-compliance --strict`
