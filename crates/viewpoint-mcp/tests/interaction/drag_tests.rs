//! Drag tool integration tests

use serde_json::json;
use viewpoint_mcp::tools::{BrowserDragTool, BrowserNavigateTool, Tool};

use super::create_browser;

#[tokio::test]
async fn test_drag_with_invalid_start_ref() {
    use viewpoint_mcp::tools::BrowserSnapshotTool;

    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let drag_tool = BrowserDragTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<html><body><button id='a'>Drag Source</button><button id='b'>Drop Target</button></body></html>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Get a valid ref for the drop target
    // New format: c{ctx}p{page}f{frame}e{counter}
    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let re = regex::Regex::new(r"\[ref=(c\d+p\d+f\d+e\d+)\]").unwrap();
    let captures: Vec<_> = re.captures_iter(&snapshot).collect();

    if !captures.is_empty() {
        let valid_ref = captures[0].get(1).unwrap().as_str();

        // Try drag with invalid startRef (non-existent element)
        let result = drag_tool
            .execute(
                &json!({
                    "startRef": "c0p0f0e99999",
                    "startElement": "invalid",
                    "endRef": valid_ref,
                    "endElement": "valid drop target"
                }),
                &mut browser,
            )
            .await;

        assert!(result.is_err(), "Drag with invalid startRef should fail");
    }

    browser.shutdown().await;
}
