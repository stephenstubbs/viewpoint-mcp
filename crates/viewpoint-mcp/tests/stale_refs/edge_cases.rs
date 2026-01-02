//! Edge Case Tests for Stale References
//!
//! Tests covering edge cases like rapid sequences and invalid ref formats.

use serde_json::json;
use viewpoint_mcp::tools::{BrowserClickTool, BrowserNavigateTool, BrowserSnapshotTool, Tool};

use super::{create_browser, extract_first_ref};

#[tokio::test]
async fn test_rapid_snapshot_click_sequence() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let click_tool = BrowserClickTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<button id='btn'>Click</button>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Rapid sequence of snapshot -> click -> snapshot -> click
    for _ in 0..5 {
        let snapshot = snapshot_tool
            .execute(&json!({}), &mut browser)
            .await
            .unwrap();
        if let Some(ref_str) = extract_first_ref(&snapshot) {
            let _ = click_tool
                .execute(
                    &json!({ "ref": ref_str, "element": "button" }),
                    &mut browser,
                )
                .await;
        }
    }

    // Should complete without panicking
    browser.shutdown().await;
}

#[tokio::test]
async fn test_invalid_ref_format() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let click_tool = BrowserClickTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<button>Test</button>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Try with various invalid ref formats
    for invalid_ref in ["", "invalid", "123", "ref123", "x123"] {
        let result = click_tool
            .execute(
                &json!({ "ref": invalid_ref, "element": "test" }),
                &mut browser,
            )
            .await;

        assert!(result.is_err(), "Invalid ref '{}' should fail", invalid_ref);
    }

    browser.shutdown().await;
}
