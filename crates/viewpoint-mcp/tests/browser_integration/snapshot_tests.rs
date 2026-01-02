//! Accessibility snapshot integration tests

use super::headless_config;
use viewpoint_mcp::browser::BrowserState;
use viewpoint_mcp::snapshot::{AccessibilitySnapshot, SnapshotOptions};

#[tokio::test]
async fn test_accessibility_snapshot_basic() {
    let config = headless_config();
    let mut state = BrowserState::new(config);

    state
        .initialize()
        .await
        .expect("Failed to initialize browser");

    // Get the active page
    let ctx = state.active_context().expect("Should have context");
    let page = ctx.active_page().expect("Should have page");

    // Set page content (using set_content like viewpoint-core tests for proper DOM access)
    page.set_content(
        r#"
        <html><body>
            <h1>Test Page</h1>
            <button id="btn">Click me</button>
            <input type="text" placeholder="Enter text">
        </body></html>
    "#,
    )
    .set()
    .await
    .expect("Failed to set content");

    // Capture accessibility snapshot
    let options = SnapshotOptions::default();
    let snapshot = AccessibilitySnapshot::capture(page, options)
        .await
        .expect("Failed to capture snapshot");

    // Format the snapshot
    let formatted = snapshot.format();

    // Verify basic structure
    assert!(snapshot.element_count() > 0, "Should have elements");
    assert!(formatted.contains("heading"), "Should contain heading");
    assert!(formatted.contains("button"), "Should contain button");
    assert!(formatted.contains("textbox"), "Should contain textbox");

    // Interactive elements should have refs
    // Note: Refs are provided by viewpoint-core's aria_snapshot API
    // Format is now c{ctx}p{page}f{frame}e{counter} (e.g., c0p0f0e1)
    assert!(
        snapshot.ref_count() > 0,
        "Should have refs for interactive elements"
    );
    assert!(
        formatted.contains("[ref=c"),
        "Should have ref annotations in new format"
    );

    state.shutdown().await;
}

#[tokio::test]
async fn test_accessibility_snapshot_ref_lookup() {
    let config = headless_config();
    let mut state = BrowserState::new(config);

    state
        .initialize()
        .await
        .expect("Failed to initialize browser");

    let ctx = state.active_context().expect("Should have context");
    let page = ctx.active_page().expect("Should have page");

    // Navigate to a page with identifiable elements
    page.goto("data:text/html,<html><body><button id='submit-btn'>Submit</button></body></html>")
        .goto()
        .await
        .expect("Failed to navigate");

    // Capture snapshot
    let options = SnapshotOptions::default();
    let snapshot = AccessibilitySnapshot::capture(page, options)
        .await
        .expect("Failed to capture snapshot");

    // Find a ref in the formatted output
    // Format is now c{ctx}p{page}f{frame}e{counter} (e.g., c0p0f0e1)
    let formatted = snapshot.format();
    assert!(
        formatted.contains("[ref=c"),
        "Should have element refs in new format"
    );

    // Extract a ref from the output and look it up
    if let Some(start) = formatted.find("[ref=c") {
        let end = formatted[start..].find(']').unwrap() + start;
        let ref_str = &formatted[start + 5..end]; // Skip "[ref="

        // Look up the ref
        let result = snapshot.lookup(ref_str);
        assert!(result.is_ok(), "Should be able to look up ref: {ref_str}");
    }

    state.shutdown().await;
}

#[tokio::test]
async fn test_accessibility_snapshot_stability_across_refreshes() {
    let config = headless_config();
    let mut state = BrowserState::new(config);

    state
        .initialize()
        .await
        .expect("Failed to initialize browser");

    let ctx = state.active_context().expect("Should have context");
    let page = ctx.active_page().expect("Should have page");

    // Navigate to a page with a stable element ID
    let html = r#"<html><body>
        <button id="stable-id">Stable Button</button>
    </body></html>"#;
    page.goto(&format!("data:text/html,{html}"))
        .goto()
        .await
        .expect("Failed to navigate");

    // First snapshot
    let options = SnapshotOptions::default();
    let snapshot1 = AccessibilitySnapshot::capture(page, options.clone())
        .await
        .expect("Failed to capture first snapshot");

    // Refresh the page
    page.reload().await.expect("Failed to reload");

    // Second snapshot
    let snapshot2 = AccessibilitySnapshot::capture(page, options)
        .await
        .expect("Failed to capture second snapshot");

    // The formatted output should have similar refs for same elements
    let formatted1 = snapshot1.format();
    let formatted2 = snapshot2.format();

    // Both should have the button with refs
    assert!(
        formatted1.contains("button"),
        "First snapshot should have button"
    );
    assert!(
        formatted2.contains("button"),
        "Second snapshot should have button"
    );

    // Element counts should be the same
    assert_eq!(
        snapshot1.element_count(),
        snapshot2.element_count(),
        "Element count should be stable"
    );

    state.shutdown().await;
}

#[tokio::test]
async fn test_accessibility_snapshot_compact_mode() {
    let config = headless_config();
    let mut state = BrowserState::new(config);

    state
        .initialize()
        .await
        .expect("Failed to initialize browser");

    let ctx = state.active_context().expect("Should have context");
    let page = ctx.active_page().expect("Should have page");

    // Create a page with many buttons to trigger compact mode (>100 elements)
    let mut buttons = String::new();
    for i in 0..110 {
        buttons.push_str(&format!("<button>Button {i}</button>"));
    }
    let html = format!("<html><body>{buttons}</body></html>");

    page.goto(&format!("data:text/html,{html}"))
        .goto()
        .await
        .expect("Failed to navigate");

    // Capture without allRefs - should trigger compact mode
    let options = SnapshotOptions {
        all_refs: false,
        ..Default::default()
    };
    let snapshot = AccessibilitySnapshot::capture(page, options)
        .await
        .expect("Failed to capture snapshot");

    assert!(
        snapshot.is_compact(),
        "Should be in compact mode with many elements"
    );

    // Capture with allRefs - should not be compact
    let options_all = SnapshotOptions {
        all_refs: true,
        ..Default::default()
    };
    let snapshot_all = AccessibilitySnapshot::capture(page, options_all)
        .await
        .expect("Failed to capture snapshot with allRefs");

    assert!(
        !snapshot_all.is_compact(),
        "Should not be compact with allRefs"
    );

    state.shutdown().await;
}
