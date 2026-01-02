## 1. Formatting and Clippy Fixes (Quick Wins)

### 1.1 Apply formatting
- [x] 1.1.1 Run `cargo fmt --all` to fix all formatting issues
- [x] 1.1.2 Verify with `cargo fmt --check`

### 1.2 Fix clippy warnings
- [x] 1.2.1 Fix `needless_raw_string_hashes` in `browser_network_requests.rs`
- [x] 1.2.2 Fix `doc_markdown` warnings (add backticks to code in doc comments)
- [x] 1.2.3 Fix `map_unwrap_or` in `snapshot/format.rs` and `snapshot/reference.rs`
- [x] 1.2.4 Verify with `cargo clippy --workspace --all-targets -- -D warnings`

## 2. Large Integration Test Refactoring

### 2.1 Split tools_interaction.rs (1013 lines)
- [x] 2.1.1 Create `tests/interaction/mod.rs` with submodule declarations
- [x] 2.1.2 Extract click tests to `tests/interaction/click_tests.rs`
- [x] 2.1.3 Extract type tests to `tests/interaction/type_tests.rs`
- [x] 2.1.4 Extract form tests to `tests/interaction/form_tests.rs`
- [x] 2.1.5 Extract key tests to `tests/interaction/key_tests.rs`
- [x] 2.1.6 Extract drag tests to `tests/interaction/drag_tests.rs`
- [x] 2.1.7 Remove original `tools_interaction.rs`

### 2.2 Split tools_inspection.rs (848 lines)
- [x] 2.2.1 Create `tests/inspection/mod.rs` with submodule declarations
- [x] 2.2.2 Extract snapshot tests to `tests/inspection/snapshot_tests.rs`
- [x] 2.2.3 Extract screenshot tests to `tests/inspection/screenshot_tests.rs`
- [x] 2.2.4 Extract console/network tests to `tests/inspection/console_network_tests.rs`
- [x] 2.2.5 Remove original `tools_inspection.rs`

### 2.3 Split tools_optional.rs (802 lines)
- [x] 2.3.1 Create `tests/optional/mod.rs` with submodule declarations
- [x] 2.3.2 Extract vision tests to `tests/optional/vision_tests.rs`
- [x] 2.3.3 Extract PDF tests to `tests/optional/pdf_tests.rs`
- [x] 2.3.4 Remove original `tools_optional.rs`

### 2.4 Split tools_context.rs (657 lines)
- [x] 2.4.1 Create `tests/context/mod.rs` with submodule declarations
- [x] 2.4.2 Extract create tests to `tests/context/create_tests.rs`
- [x] 2.4.3 Extract switch tests to `tests/context/switch_tests.rs`
- [x] 2.4.4 Extract list tests to `tests/context/list_tests.rs`
- [x] 2.4.5 Extract close tests to `tests/context/close_tests.rs`
- [x] 2.4.6 Extract storage tests to `tests/context/storage_tests.rs`
- [x] 2.4.7 Extract integration tests to `tests/context/integration_tests.rs`
- [x] 2.4.8 Remove original `tools_context.rs`

### 2.5 Split browser_integration.rs (649 lines)
- [x] 2.5.1 Create `tests/browser_integration/mod.rs` with submodule declarations
- [x] 2.5.2 Extract state tests to `tests/browser_integration/state_tests.rs`
- [x] 2.5.3 Extract snapshot tests to `tests/browser_integration/snapshot_tests.rs`
- [x] 2.5.4 Remove original `browser_integration.rs`

### 2.6 Split tools_management.rs (610 lines)
- [x] 2.6.1 Create `tests/tools_management/mod.rs` with submodule declarations
- [x] 2.6.2 Extract tab tests to `tests/tools_management/tabs_tests.rs`
- [x] 2.6.3 Extract resize tests to `tests/tools_management/resize_tests.rs`
- [x] 2.6.4 Extract close tests to `tests/tools_management/close_tests.rs`
- [x] 2.6.5 Extract dialog tests to `tests/tools_management/dialog_tests.rs`
- [x] 2.6.6 Extract install tests to `tests/tools_management/install_tests.rs`
- [x] 2.6.7 Extract integration tests to `tests/tools_management/integration_tests.rs`
- [x] 2.6.8 Remove original `tools_management.rs`

### 2.7 Split stale_refs.rs (568 lines)
- [x] 2.7.1 Create `tests/stale_refs/mod.rs` with submodule declarations
- [x] 2.7.2 Extract detection tests to `tests/stale_refs/detection.rs`
- [x] 2.7.3 Extract edge case tests to `tests/stale_refs/edge_cases.rs`
- [x] 2.7.4 Extract multi-context tests to `tests/stale_refs/multi_context.rs`
- [x] 2.7.5 Remove original `stale_refs.rs`

## 3. Tools Module Test Migration

### 3.1 Create tools tests infrastructure
- [x] 3.1.1 Create `src/tools/tests/mod.rs` with module declarations
- [x] 3.1.2 Add `#[cfg(test)] mod tests;` to `src/tools/mod.rs`

### 3.2 Migrate navigation tool tests
- [x] 3.2.1 Extract tests from `browser_navigate.rs` to `tests/browser_navigate_tests.rs`
- [x] 3.2.2 Extract tests from `browser_navigate_back.rs` to `tests/browser_navigate_back_tests.rs`
- [x] 3.2.3 Remove inline test modules from both files

### 3.3 Migrate interaction tool tests
- [x] 3.3.1 Extract tests from `browser_click.rs` to `tests/browser_click_tests.rs`
- [x] 3.3.2 Extract tests from `browser_type.rs` to `tests/browser_type_tests.rs`
- [x] 3.3.3 Extract tests from `browser_hover.rs` to `tests/browser_hover_tests.rs`
- [x] 3.3.4 Extract tests from `browser_drag.rs` to `tests/browser_drag_tests.rs`
- [x] 3.3.5 Extract tests from `browser_fill_form.rs` to `tests/browser_fill_form_tests.rs`
- [x] 3.3.6 Extract tests from `browser_select_option.rs` to `tests/browser_select_option_tests.rs`
- [x] 3.3.7 Extract tests from `browser_press_key.rs` to `tests/browser_press_key_tests.rs`
- [x] 3.3.8 Extract tests from `browser_file_upload.rs` to `tests/browser_file_upload_tests.rs`
- [x] 3.3.9 Remove inline test modules from all interaction files

### 3.4 Migrate inspection tool tests
- [x] 3.4.1 Extract tests from `browser_snapshot.rs` to `tests/browser_snapshot_tests.rs`
- [x] 3.4.2 Extract tests from `browser_take_screenshot.rs` to `tests/browser_take_screenshot_tests.rs`
- [x] 3.4.3 Extract tests from `browser_console_messages.rs` to `tests/browser_console_messages_tests.rs`
- [x] 3.4.4 Extract tests from `browser_network_requests.rs` to `tests/browser_network_requests_tests.rs`
- [x] 3.4.5 Remove inline test modules from all inspection files

### 3.5 Migrate state tool tests
- [x] 3.5.1 Extract tests from `browser_evaluate.rs` to `tests/browser_evaluate_tests.rs`
- [x] 3.5.2 Extract tests from `browser_handle_dialog.rs` to `tests/browser_handle_dialog_tests.rs`
- [x] 3.5.3 Extract tests from `browser_wait_for.rs` to `tests/browser_wait_for_tests.rs`
- [x] 3.5.4 Remove inline test modules from all state files

### 3.6 Migrate management tool tests
- [x] 3.6.1 Extract tests from `browser_close.rs` to `tests/browser_close_tests.rs`
- [x] 3.6.2 Extract tests from `browser_install.rs` to `tests/browser_install_tests.rs`
- [x] 3.6.3 Extract tests from `browser_resize.rs` to `tests/browser_resize_tests.rs`
- [x] 3.6.4 Extract tests from `browser_tabs.rs` to `tests/browser_tabs_tests.rs`
- [x] 3.6.5 Remove inline test modules from all management files

### 3.7 Migrate context management tool tests
- [x] 3.7.1 Extract tests from `browser_context_create.rs` to `tests/browser_context_create_tests.rs`
- [x] 3.7.2 Extract tests from `browser_context_switch.rs` to `tests/browser_context_switch_tests.rs`
- [x] 3.7.3 Extract tests from `browser_context_list.rs` to `tests/browser_context_list_tests.rs`
- [x] 3.7.4 Extract tests from `browser_context_close.rs` to `tests/browser_context_close_tests.rs`
- [x] 3.7.5 Extract tests from `browser_context_save_storage.rs` to `tests/browser_context_save_storage_tests.rs`
- [x] 3.7.6 Remove inline test modules from all context files

### 3.8 Migrate optional capability tool tests
- [x] 3.8.1 Extract tests from `browser_mouse_click_xy.rs` to `tests/browser_mouse_click_xy_tests.rs`
- [x] 3.8.2 Extract tests from `browser_mouse_move_xy.rs` to `tests/browser_mouse_move_xy_tests.rs`
- [x] 3.8.3 Extract tests from `browser_mouse_drag_xy.rs` to `tests/browser_mouse_drag_xy_tests.rs`
- [x] 3.8.4 Extract tests from `browser_pdf_save.rs` to `tests/browser_pdf_save_tests.rs`
- [x] 3.8.5 Remove inline test modules from all optional capability files

### 3.9 Migrate framework tests
- [x] 3.9.1 Extract tests from `registry.rs` to `tests/registry_tests.rs`
- [x] 3.9.2 Remove inline test module from `registry.rs`

## 4. Snapshot Module Test Migration

### 4.1 Migrate reference tests
- [x] 4.1.1 Move tests from `snapshot/reference.rs` to `snapshot/tests/mod.rs`
- [x] 4.1.2 Update `snapshot/tests/mod.rs` to include the reference tests
- [x] 4.1.3 Remove inline test module from `reference.rs`

## 5. Final Validation

### 5.1 Verify all tests pass
- [x] 5.1.1 Run `cargo test --workspace` to ensure all unit tests pass (180 tests passing)
- [x] 5.1.2 Integration tests excluded (require browser - manual testing)

### 5.2 Verify convention compliance
- [x] 5.2.1 Run `cargo fmt --check` to confirm no formatting issues
- [x] 5.2.2 Run `cargo clippy --workspace --all-targets -- -D warnings` to confirm no warnings
- [x] 5.2.3 Verify no files exceed 500 lines
- [x] 5.2.4 Verify no inline `#[cfg(test)] mod tests {` blocks remain in source files (only `mod tests;` declarations remain)
