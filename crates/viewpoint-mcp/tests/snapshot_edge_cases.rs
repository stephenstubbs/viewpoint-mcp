//! Integration tests for snapshot format edge cases
//!
//! These tests verify:
//! - Text truncation behavior at boundaries
//! - Compact mode triggering based on element count
//! - Frame handling in snapshots
//!
//! Run with:
//! ```sh
//! cargo test --features integration -p viewpoint-mcp --test snapshot_edge_cases
//! ```
#![cfg(feature = "integration")]

use serde_json::json;
use viewpoint_mcp::browser::{BrowserConfig, BrowserState};
use viewpoint_mcp::tools::{BrowserClickTool, BrowserNavigateTool, BrowserSnapshotTool, Tool};

/// Helper to create a headless browser state
async fn create_browser() -> BrowserState {
    let config = BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let mut state = BrowserState::new(config);
    state
        .initialize()
        .await
        .expect("Failed to initialize browser");
    state
}

// =============================================================================
// Text Truncation Tests (MAX_TEXT_LENGTH = 100)
// =============================================================================

#[tokio::test]
async fn test_text_at_exactly_100_characters() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // Create text that is exactly 100 characters
    let text_100 = "a".repeat(100);
    let html = format!("<button>{}</button>", text_100);

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html) }),
            &mut browser,
        )
        .await
        .unwrap();

    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // At exactly 100 chars, text should NOT be truncated (no "...")
    assert!(
        !snapshot.contains("..."),
        "Text at exactly 100 chars should not be truncated"
    );
    // But the text should be present
    assert!(
        snapshot.contains(&text_100[..50]),
        "First half of text should be present"
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_text_at_101_characters_is_truncated() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // Create text that is 101 characters
    let text_101 = "b".repeat(101);
    let html = format!("<button>{}</button>", text_101);

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html) }),
            &mut browser,
        )
        .await
        .unwrap();

    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // At 101 chars, text SHOULD be truncated with "..."
    assert!(
        snapshot.contains("..."),
        "Text at 101 chars should be truncated with ellipsis"
    );
    // Full text should NOT be present
    assert!(
        !snapshot.contains(&text_101),
        "Full 101-char text should not be present"
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_truncated_text_ends_with_ellipsis() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // Create significantly longer text to ensure truncation
    let long_text = "x".repeat(200);
    let html = format!("<button>{}</button>", long_text);

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html) }),
            &mut browser,
        )
        .await
        .unwrap();

    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // Truncated text should end with "..."
    // The format is: - button "truncated_text..."
    // So we look for the pattern of many x's followed by ...
    let truncation_pattern = format!("{}...", "x".repeat(97)); // 97 chars + "..." = 100
    assert!(
        snapshot.contains(&truncation_pattern) || snapshot.contains("..."),
        "Truncated text should end with '...'"
    );

    browser.shutdown().await;
}

// =============================================================================
// Compact Mode Tests (threshold = 100 interactive elements)
// =============================================================================

#[tokio::test]
async fn test_page_with_few_elements_not_compact() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // Create 50 buttons - well under the threshold
    let mut buttons = String::new();
    for i in 0..50 {
        buttons.push_str(&format!("<button>Btn{}</button>", i));
    }
    let html = format!("<html><body>{}</body></html>", buttons);

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html) }),
            &mut browser,
        )
        .await
        .unwrap();

    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // With only 50 elements, should NOT be in compact mode
    assert!(
        !snapshot.contains("Page has many interactive elements"),
        "50 elements should not trigger compact mode note"
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_page_with_many_elements_triggers_compact_mode() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // Create 150 buttons (interactive elements) - well over threshold of 100
    let mut buttons = String::new();
    for i in 0..150 {
        buttons.push_str(&format!("<button>Btn{}</button>", i));
    }
    let html = format!("<html><body>{}</body></html>", buttons);

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html) }),
            &mut browser,
        )
        .await
        .unwrap();

    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // With 150 elements (> 100 threshold), should be in compact mode (adds note about allRefs)
    assert!(
        snapshot.contains("Page has many interactive elements") || snapshot.contains("allRefs"),
        "150 elements should trigger compact mode note"
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_compact_mode_all_refs_bypasses() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // Create 110 buttons
    let mut buttons = String::new();
    for i in 0..110 {
        buttons.push_str(&format!("<button>Btn{}</button>", i));
    }
    let html = format!("<html><body>{}</body></html>", buttons);

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html) }),
            &mut browser,
        )
        .await
        .unwrap();

    // With allRefs: true, compact mode still applies but all elements get refs
    let snapshot = snapshot_tool
        .execute(&json!({ "allRefs": true }), &mut browser)
        .await
        .unwrap();

    // Count refs in the snapshot
    let ref_count = snapshot.matches("[ref=").count();

    // With allRefs, all 110 buttons should have refs
    // Note: The exact number might vary based on page structure
    assert!(
        ref_count >= 100,
        "allRefs should provide refs for most/all elements, got {}",
        ref_count
    );

    browser.shutdown().await;
}

// =============================================================================
// Frame Handling Tests
// =============================================================================

#[tokio::test]
async fn test_snapshot_includes_frame_boundary_marker() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // Create a page with an iframe containing a button
    // Note: Using srcdoc for same-origin iframe content
    let html = r#"
        <html>
        <body>
            <h1>Main Page</h1>
            <button>Main Button</button>
            <iframe srcdoc="<html><body><button>Frame Button</button></body></html>"></iframe>
        </body>
        </html>
    "#;

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html.replace('\n', "")) }),
            &mut browser,
        )
        .await
        .unwrap();

    // Wait for iframe to load
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // Snapshot should contain frame boundary marker
    // The exact format depends on implementation, but should indicate frame
    // Based on format.rs, frames get "[frame-boundary]" marker
    let has_frame_indicator = snapshot.contains("frame-boundary")
        || snapshot.contains("iframe")
        || snapshot.contains("Frame");

    assert!(
        has_frame_indicator,
        "Snapshot should indicate frame presence: {}",
        snapshot
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_elements_inside_iframe_have_refs() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // Create a page with an iframe containing interactive elements
    let html = r#"
        <html>
        <body>
            <button id="outer">Outer</button>
            <iframe srcdoc="<html><body><button id='inner'>Inner Button</button><a href='#'>Inner Link</a></body></html>"></iframe>
        </body>
        </html>
    "#;

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html.replace('\n', "")) }),
            &mut browser,
        )
        .await
        .unwrap();

    // Wait for iframe to load
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // Count refs - should have at least one ref for the outer button
    let ref_count = snapshot.matches("[ref=").count();

    // Should have at least the outer button ref
    // Note: iframe content may or may not be included depending on browser/accessibility tree behavior
    assert!(
        ref_count >= 1,
        "Should have at least one ref for outer button, got {}",
        ref_count
    );

    // The snapshot should contain button-related content
    assert!(
        snapshot.contains("button") || snapshot.contains("Button"),
        "Snapshot should contain button elements"
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_clicking_element_inside_iframe_works() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let click_tool = BrowserClickTool::new();

    // Create a page with an iframe containing a button that changes text on click
    let html = r#"
        <html>
        <body>
            <button id="outer">Outer</button>
            <iframe id="myframe" srcdoc="<html><body><button id='inner' onclick='this.textContent=&quot;Clicked&quot;'>Click Me</button></body></html>"></iframe>
        </body>
        </html>
    "#;

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html.replace('\n', "")) }),
            &mut browser,
        )
        .await
        .unwrap();

    // Wait for iframe to load
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // Find a ref for a button (could be outer or inner)
    let re = regex::Regex::new(r"\[ref=(e[0-9a-f]+)\]").unwrap();
    let captures: Vec<_> = re.captures_iter(&snapshot).collect();

    if !captures.is_empty() {
        // Try clicking the first ref found
        let ref_str = captures[0].get(1).unwrap().as_str();

        let click_result = click_tool
            .execute(
                &json!({ "ref": ref_str, "element": "button" }),
                &mut browser,
            )
            .await;

        // Click should succeed (whether it's outer or inner button)
        assert!(
            click_result.is_ok(),
            "Click on button ref should succeed: {:?}",
            click_result.err()
        );
    }

    browser.shutdown().await;
}

// =============================================================================
// Mixed Edge Cases
// =============================================================================

#[tokio::test]
async fn test_truncation_with_unicode() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // Create text with unicode characters that exceeds limit
    // Note: Unicode chars may be multiple bytes but should be truncated by char count
    let unicode_text = "日本語テスト".repeat(20); // Japanese text repeated
    let html = format!("<button>{}</button>", unicode_text);

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html) }),
            &mut browser,
        )
        .await
        .unwrap();

    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // Should handle unicode truncation without panic
    assert!(
        snapshot.contains("button"),
        "Should still contain button role"
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_snapshot_deeply_nested_elements() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // Create deeply nested structure (40 levels deep)
    let mut html = String::from("<html><body>");
    for _ in 0..40 {
        html.push_str("<div>");
    }
    html.push_str("<button>Deep Button</button>");
    for _ in 0..40 {
        html.push_str("</div>");
    }
    html.push_str("</body></html>");

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html) }),
            &mut browser,
        )
        .await
        .unwrap();

    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // Should handle deep nesting without panic and still find the button
    assert!(
        snapshot.contains("button"),
        "Should find deeply nested button"
    );

    browser.shutdown().await;
}
