# Tasks

## 1. Upgrade Viewpoint Dependencies
- [x] 1.1 Update `Cargo.toml` (workspace) to use viewpoint-core 0.2.9 and viewpoint-cdp 0.2.9
- [x] 1.2 Run `cargo update` to update lock file
- [x] 1.3 Verify build compiles with new version: `cargo build --workspace`

## 2. Fix browser_click Tool
- [x] 2.1 Replace `[data-ref='{}']` selector with `page.locator_from_ref(ref_string)`
- [x] 2.2 Update error messages for ref resolution failures
- [x] 2.3 Test clicking elements on real pages

## 3. Fix browser_type Tool
- [x] 3.1 Replace CSS selector with `page.locator_from_ref(ref_string)`
- [x] 3.2 Verify typing works in textboxes, searchboxes, comboboxes
- [x] 3.3 Test submit functionality (Enter key)

## 4. Fix browser_hover Tool
- [x] 4.1 Replace CSS selector with `page.locator_from_ref(ref_string)`
- [x] 4.2 Test hover triggers tooltip/menu display

## 5. Fix browser_drag Tool
- [x] 5.1 Replace both source and target selectors with `page.locator_from_ref()`
- [x] 5.2 Test drag-and-drop between elements

## 6. Fix browser_fill_form Tool
- [x] 6.1 Replace CSS selector with `page.locator_from_ref()` for each field
- [x] 6.2 Test filling multiple field types (textbox, checkbox, radio, combobox)

## 7. Fix browser_select_option Tool
- [x] 7.1 Replace CSS selector with `page.locator_from_ref()`
- [x] 7.2 Test selecting options in dropdowns

## 8. Fix browser_evaluate Tool (Element Mode)
- [x] 8.1 Replace CSS selector with `page.locator_from_ref()` when `ref` is provided
- [x] 8.2 Test evaluating JavaScript on specific elements

## 9. Fix browser_take_screenshot Tool (Element Mode)
- [x] 9.1 Replace CSS selector with `page.locator_from_ref()` when `ref` is provided
- [x] 9.2 Test screenshotting specific elements

## 10. Integration Testing
- [x] 10.1 Run full test suite: `cargo test --workspace`
- [x] 10.2 Run integration tests: `cargo test --workspace --features integration`
- [x] 10.3 Manual testing: Navigate to DuckDuckGo, search, click results
- [x] 10.4 Manual testing: Fill out a form on a test page

## 11. Update Specs
- [x] 11.1 Update accessibility-snapshots spec with native ref resolution
- [x] 11.2 Verify browser-tools spec scenarios still accurate
