//! Form and evaluate tool integration tests

use serde_json::json;
use viewpoint_mcp::tools::{BrowserFillFormTool, BrowserNavigateTool, Tool};

use super::create_browser;

#[tokio::test]
async fn test_fill_form_with_one_invalid_ref() {
    use viewpoint_mcp::tools::BrowserSnapshotTool;

    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let fill_tool = BrowserFillFormTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<input type='text' name='field1'><input type='text' name='field2'>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Get a valid ref from snapshot
    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // Extract a valid ref if present
    let re = regex::Regex::new(r"\[ref=(e\d+)\]").unwrap();
    let valid_ref = re
        .captures(&snapshot)
        .map(|c| c.get(1).unwrap().as_str().to_string());

    if let Some(ref_str) = valid_ref {
        // Try to fill with mix of valid and invalid refs
        let result = fill_tool
            .execute(
                &json!({
                    "fields": [
                        { "name": "valid field", "type": "textbox", "ref": ref_str, "value": "valid" },
                        { "name": "invalid field", "type": "textbox", "ref": "c0p0f0e99999", "value": "invalid" }
                    ]
                }),
                &mut browser,
            )
            .await;

        // Should fail due to invalid ref
        assert!(result.is_err(), "Fill with one invalid ref should fail");
    }

    browser.shutdown().await;
}

#[tokio::test]
async fn test_evaluate_with_removed_element_ref() {
    use viewpoint_mcp::tools::{BrowserEvaluateTool, BrowserSnapshotTool};

    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<div id='container'><button id='btn'>Test</button></div>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Get a ref for the button
    // New format: c{ctx}p{page}f{frame}e{counter}
    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let re = regex::Regex::new(r"\[ref=(c\d+p\d+f\d+e\d+)\]").unwrap();

    if let Some(captures) = re.captures(&snapshot) {
        let ref_str = captures.get(1).unwrap().as_str();

        // Remove the element
        eval_tool
            .execute(
                &json!({ "function": "() => document.getElementById('container').innerHTML = ''" }),
                &mut browser,
            )
            .await
            .unwrap();

        // Try to evaluate on the removed element
        let result = eval_tool
            .execute(
                &json!({
                    "ref": ref_str,
                    "element": "removed button",
                    "function": "(el) => el.textContent"
                }),
                &mut browser,
            )
            .await;

        // Should fail since element no longer exists
        assert!(result.is_err(), "Evaluate on removed element should fail");
    }

    browser.shutdown().await;
}
