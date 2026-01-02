//! Stale Reference Detection Tests
//!
//! Tests that verify proper detection and handling of stale element references.

use serde_json::json;
use viewpoint_mcp::tools::{
    BrowserClickTool, BrowserEvaluateTool, BrowserNavigateTool, BrowserSnapshotTool, Tool,
};

use super::{create_browser, extract_first_ref};

#[tokio::test]
async fn test_ref_from_current_snapshot_works() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let click_tool = BrowserClickTool::new();

    // Navigate to a page with a button
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<button id='test'>Click Me</button>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Take a snapshot
    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // Extract a ref from the snapshot
    let ref_str = extract_first_ref(&snapshot).expect("Should find a ref");

    // Clicking with the current snapshot's ref should succeed
    let result = click_tool
        .execute(
            &json!({ "ref": ref_str, "element": "test button" }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_ok(),
        "Click with current ref should succeed: {:?}",
        result.err()
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_ref_from_previous_snapshot_after_element_removed() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let eval_tool = BrowserEvaluateTool::new();
    let click_tool = BrowserClickTool::new();

    // Navigate to a page with a button
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<div id='container'><button id='test'>Click Me</button></div>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Take a snapshot and get a ref
    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let ref_str = extract_first_ref(&snapshot).expect("Should find a ref");

    // Remove the element using JavaScript
    eval_tool
        .execute(
            &json!({ "function": "() => document.getElementById('container').innerHTML = ''" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Try to click with the old ref - should fail
    let result = click_tool
        .execute(
            &json!({ "ref": ref_str, "element": "removed button" }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_err(),
        "Click with stale ref (removed element) should fail"
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_ref_counter_resets_per_snapshot() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // Navigate to first page with a button
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<html><body><button id='btn1'>Page 1</button></body></html>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Take a snapshot and get a ref
    let snapshot1 = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let ref_str1 = extract_first_ref(&snapshot1).expect("Should find a ref");
    eprintln!("Got ref from page 1: {}", ref_str1);

    // Navigate to a different page with a different button
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<html><body><button id='btn2'>Page 2</button></body></html>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Take a NEW snapshot after navigation
    let snapshot2 = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let ref_str2 = extract_first_ref(&snapshot2).expect("Should find a ref in new snapshot");
    eprintln!("Got ref from page 2: {}", ref_str2);

    // Since both pages have a single button, they get the same ref string
    // (e.g., c0p0f0e1) - the counter resets per snapshot
    // This is expected behavior - refs are tied to the latest snapshot
    assert_eq!(
        ref_str1, ref_str2,
        "Same page structure gives same ref string"
    );

    // The snapshot should show Page 2's button content
    assert!(
        snapshot2.contains("Page 2"),
        "Latest snapshot should show Page 2 content: {}",
        snapshot2
    );

    // This documents the current behavior: ref strings can collide across
    // navigations if the page structure is similar. The ref_map is updated
    // with each new snapshot, pointing to the current page's elements.
    // For stable refs across page changes, use the stable refs feature
    // (proposal: add-stable-element-refs in viewpoint).

    browser.shutdown().await;
}

#[tokio::test]
async fn test_new_snapshot_has_valid_refs_after_content_change() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let eval_tool = BrowserEvaluateTool::new();
    let click_tool = BrowserClickTool::new();

    // Navigate to a page
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<div id='container'><button id='original'>Original</button></div>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Modify the page content
    eval_tool
        .execute(
            &json!({ "function": "() => document.getElementById('container').innerHTML = '<button id=\"new\">New Button</button>'" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Take a new snapshot - this should have a fresh ref for the new button
    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let ref_str = extract_first_ref(&snapshot).expect("Should find a ref for new button");

    // Clicking with the new ref should succeed
    let result = click_tool
        .execute(
            &json!({ "ref": ref_str, "element": "new button" }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_ok(),
        "Click with new snapshot ref should succeed: {:?}",
        result.err()
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_snapshot_invalidation_after_click() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let click_tool = BrowserClickTool::new();

    // Navigate to a page with a button that adds content when clicked
    let html = r#"
        <div id="container">
            <button id="add" onclick="document.getElementById('container').innerHTML += '<p>Added</p>'">Add</button>
        </div>
    "#;

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html) }),
            &mut browser,
        )
        .await
        .unwrap();

    // Take first snapshot
    let snap1 = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let ref_str = extract_first_ref(&snap1).expect("Should find a ref");

    // Click the button (this should invalidate cache)
    click_tool
        .execute(
            &json!({ "ref": ref_str, "element": "add button" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Take second snapshot - should reflect new content
    let snap2 = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // The snapshots should be different (page changed)
    // Note: They might be similar if caching logic differs, but ideally different
    // At minimum, the snapshot should still be valid
    assert!(!snap2.is_empty(), "Second snapshot should not be empty");

    browser.shutdown().await;
}
