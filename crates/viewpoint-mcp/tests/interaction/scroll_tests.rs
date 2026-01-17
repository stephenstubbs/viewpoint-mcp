//! Scroll into view tool integration tests

use serde_json::json;
use viewpoint_mcp::tools::{
    BrowserEvaluateTool, BrowserNavigateTool, BrowserScrollIntoViewTool, BrowserSnapshotTool, Tool,
};

use super::{create_browser, extract_first_ref};

#[tokio::test]
async fn test_scroll_into_view_tool_initialization() {
    let tool = BrowserScrollIntoViewTool::new();

    assert_eq!(tool.name(), "browser_scroll_into_view");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("ref"))
    );
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("element"))
    );
}

#[tokio::test]
async fn test_scroll_into_view_missing_ref() {
    let mut browser = create_browser().await;
    let tool = BrowserScrollIntoViewTool::new();

    let result = tool
        .execute(&json!({ "element": "Some element" }), &mut browser)
        .await;

    assert!(result.is_err());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_scroll_into_view_nonexistent_ref() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let scroll_tool = BrowserScrollIntoViewTool::new();

    // Navigate first to ensure we have a page
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<button>Test</button>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Now test with a nonexistent ref
    let result = scroll_tool
        .execute(
            &json!({ "ref": "nonexistent", "element": "test" }),
            &mut browser,
        )
        .await;

    // Should fail due to ref not found
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("ref") || err.contains("Element") || err.contains("not found"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_scroll_into_view_success() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let scroll_tool = BrowserScrollIntoViewTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    // Create a page with content that requires scrolling
    // The button is placed far down the page to be initially out of view
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<html><body style='height:3000px;'><button id='btn' style='position:absolute;top:2500px;'>Bottom Button</button></body></html>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Check initial scroll position (should be at top)
    let initial_scroll = eval_tool
        .execute(&json!({ "function": "() => window.scrollY" }), &mut browser)
        .await
        .unwrap();
    // Extract scroll value from result (may be "Evaluation result: 0" or just "0")
    let initial_value: f64 = initial_scroll
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .next()
        .unwrap_or(-1.0);
    assert!(
        initial_value < 100.0,
        "Should start at top: {}",
        initial_scroll
    );

    // Get the button ref
    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let ref_str = extract_first_ref(&snapshot).expect("Should find button ref");

    // Scroll the button into view
    let result = scroll_tool
        .execute(
            &json!({
                "ref": ref_str,
                "element": "Bottom Button"
            }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_ok(),
        "Scroll into view should succeed: {:?}",
        result.err()
    );
    assert!(result.unwrap().contains("Scrolled"));

    // Verify the page has scrolled down
    let final_scroll = eval_tool
        .execute(&json!({ "function": "() => window.scrollY" }), &mut browser)
        .await
        .unwrap();

    // Parse the scroll value - extract the number from the result string
    // The result may be in format "Evaluation result: 2264" or just "2264"
    let scroll_value: f64 = final_scroll
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .next()
        .unwrap_or(0.0);
    assert!(
        scroll_value > 1000.0,
        "Page should have scrolled down, scrollY = {}",
        final_scroll
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_scroll_into_view_already_visible() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let scroll_tool = BrowserScrollIntoViewTool::new();

    // Create a page where the button is already visible
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<html><body><button id='btn'>Visible Button</button></body></html>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Get the button ref
    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let ref_str = extract_first_ref(&snapshot).expect("Should find button ref");

    // Scroll the button into view (should succeed even though it's already visible)
    let result = scroll_tool
        .execute(
            &json!({
                "ref": ref_str,
                "element": "Visible Button"
            }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_ok(),
        "Scroll into view should succeed for visible element: {:?}",
        result.err()
    );

    browser.shutdown().await;
}
