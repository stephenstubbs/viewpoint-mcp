//! Multi-Context Ref Format Tests
//!
//! Tests verifying that refs work correctly across multiple browser contexts.

use serde_json::json;
use viewpoint_mcp::tools::{
    BrowserClickTool, BrowserContextCreateTool, BrowserContextSwitchTool, BrowserNavigateTool,
    BrowserSnapshotTool, Tool,
};

use super::{create_browser, extract_first_ref, extract_ref};

#[tokio::test]
async fn test_snapshot_in_named_context_may_have_context_prefixed_refs() {
    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // Create a named context
    create_tool
        .execute(&json!({ "name": "test_ctx" }), &mut browser)
        .await
        .unwrap();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<button>Context Button</button>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // Snapshot should contain refs in the new format c{ctx}p{page}f{frame}e{counter}
    // Context index is embedded in the ref (c0, c1, etc.) rather than as a name prefix
    let has_ref = snapshot.contains("[ref=c");
    assert!(
        has_ref,
        "Snapshot should contain at least one ref in new format"
    );

    // Verify ref format
    if let Some(ref_str) = extract_ref(&snapshot) {
        assert!(
            ref_str.starts_with('c'),
            "Ref should be in new format c{{ctx}}p{{page}}f{{frame}}e{{counter}}: {}",
            ref_str
        );
    }

    browser.shutdown().await;
}

#[tokio::test]
async fn test_clicking_with_context_prefixed_ref_succeeds() {
    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let click_tool = BrowserClickTool::new();

    // Create a named context
    create_tool
        .execute(&json!({ "name": "click_ctx" }), &mut browser)
        .await
        .unwrap();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<button id='btn'>Click Test</button>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // Get any ref from snapshot (plain or prefixed)
    let ref_str = extract_ref(&snapshot)
        .or_else(|| extract_first_ref(&snapshot))
        .expect("Should find a ref");

    // Click should succeed with the ref (whether prefixed or not)
    let result = click_tool
        .execute(
            &json!({ "ref": ref_str, "element": "button" }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_ok(),
        "Click with ref '{}' should succeed: {:?}",
        ref_str,
        result.err()
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_ref_from_one_context_fails_in_another_context() {
    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();
    let switch_tool = BrowserContextSwitchTool::new();
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let click_tool = BrowserClickTool::new();

    // Create context A and get a ref
    create_tool
        .execute(&json!({ "name": "ctx_a" }), &mut browser)
        .await
        .unwrap();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<button id='btn_a'>Button A</button>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let snapshot_a = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let ref_a = extract_first_ref(&snapshot_a).expect("Should find ref in context A");

    // Create context B with different content
    create_tool
        .execute(&json!({ "name": "ctx_b" }), &mut browser)
        .await
        .unwrap();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<button id='btn_b'>Button B</button>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Try to use ref from context A in context B - should fail
    // (refs are page-specific, not shared between contexts)
    let result = click_tool
        .execute(
            &json!({ "ref": ref_a, "element": "button from A" }),
            &mut browser,
        )
        .await;

    // This should fail because:
    // 1. Different context/page means different DOM
    // 2. Ref from context A doesn't exist in context B's page
    assert!(
        result.is_err(),
        "Ref from context A should fail in context B"
    );

    // Now switch back to A and verify the ref still works there
    switch_tool
        .execute(&json!({ "name": "ctx_a" }), &mut browser)
        .await
        .unwrap();

    // Take fresh snapshot in context A
    let snapshot_a2 = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let ref_a2 = extract_first_ref(&snapshot_a2).expect("Should find ref in context A again");

    let result_a = click_tool
        .execute(
            &json!({ "ref": ref_a2, "element": "button A" }),
            &mut browser,
        )
        .await;

    assert!(
        result_a.is_ok(),
        "Fresh ref in context A should still work: {:?}",
        result_a.err()
    );

    browser.shutdown().await;
}
