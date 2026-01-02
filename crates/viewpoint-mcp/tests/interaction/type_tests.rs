//! Type tool integration tests

use serde_json::json;
use viewpoint_mcp::tools::{BrowserNavigateTool, BrowserTypeTool, Tool};

use super::create_browser;

#[tokio::test]
async fn test_type_nonexistent_ref() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let type_tool = BrowserTypeTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<input type='text' placeholder='Test'>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Try to type into a non-existent ref
    let result = type_tool
        .execute(
            &json!({ "ref": "c0p0f0e99999", "element": "nonexistent", "text": "hello" }),
            &mut browser,
        )
        .await;

    assert!(result.is_err(), "Type with non-existent ref should fail");
    let err = result.unwrap_err().to_string();
    assert!(err.contains("not found") || err.contains("Element") || err.contains("ref"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_type_invalid_ref_format() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let type_tool = BrowserTypeTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<input type='text'>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Try invalid ref formats
    for invalid_ref in ["", "invalid", "123", "xyz"] {
        let result = type_tool
            .execute(
                &json!({ "ref": invalid_ref, "element": "test", "text": "hello" }),
                &mut browser,
            )
            .await;

        assert!(
            result.is_err(),
            "Type with invalid ref '{invalid_ref}' should fail"
        );
    }

    browser.shutdown().await;
}
