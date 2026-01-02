//! Click tool integration tests

use serde_json::json;
use viewpoint_mcp::tools::{BrowserClickTool, BrowserNavigateTool, Tool};

use super::{create_browser, extract_first_ref};

#[tokio::test]
async fn test_click_missing_ref() {
    let mut browser = create_browser().await;
    let tool = BrowserClickTool::new();

    let result = tool
        .execute(&json!({ "element": "Some element" }), &mut browser)
        .await;

    assert!(result.is_err());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_click_tool_initialization() {
    let tool = BrowserClickTool::new();

    assert_eq!(tool.name(), "browser_click");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("ref"))
    );
}

#[tokio::test]
async fn test_interaction_requires_page() {
    let mut browser = create_browser().await;
    let click_tool = BrowserClickTool::new();

    // Navigate first to ensure we have a page
    let nav_tool = BrowserNavigateTool::new();
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<button>Test</button>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Now test - will fail on ref lookup but shows page exists
    let result = click_tool
        .execute(
            &json!({ "ref": "nonexistent", "element": "test" }),
            &mut browser,
        )
        .await;

    // Should fail due to ref not found, not due to no page
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("ref") || err.contains("Element") || err.contains("not found"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_click_right_button() {
    use viewpoint_mcp::tools::{BrowserEvaluateTool, BrowserSnapshotTool};

    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let click_tool = BrowserClickTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    // Create a page with button
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<html><body><button id='btn'>Click me</button></body></html>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Get the button ref FIRST (before modifying the page)
    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let ref_str = extract_first_ref(&snapshot).expect("Should find button ref");

    // Add event listener AFTER getting ref (to ensure ref map is populated)
    eval_tool
        .execute(
            &json!({ "function": "() => { document.getElementById('btn').onmousedown = function(e) { this.textContent = 'button=' + e.button; }; }" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Perform right-click (button 2)
    let result = click_tool
        .execute(
            &json!({
                "ref": ref_str,
                "element": "test button",
                "button": "right"
            }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_ok(),
        "Right-click should succeed: {:?}",
        result.err()
    );

    // Verify the button was right-clicked (button 2) using evaluate
    let button_text = eval_tool
        .execute(
            &json!({ "function": "() => document.getElementById('btn').textContent" }),
            &mut browser,
        )
        .await
        .unwrap();

    assert!(
        button_text.contains("button=2"),
        "Button should show button=2 for right-click, got: {}",
        button_text
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_click_middle_button() {
    use viewpoint_mcp::tools::{BrowserEvaluateTool, BrowserSnapshotTool};

    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let click_tool = BrowserClickTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    // Create a page with button
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<html><body><button id='btn'>Click me</button></body></html>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Get the button ref FIRST (before modifying the page)
    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let ref_str = extract_first_ref(&snapshot).expect("Should find button ref");

    // Add event listener AFTER getting ref
    eval_tool
        .execute(
            &json!({ "function": "() => { document.getElementById('btn').onmousedown = function(e) { this.textContent = 'button=' + e.button; }; }" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Perform middle-click (button 1)
    let result = click_tool
        .execute(
            &json!({
                "ref": ref_str,
                "element": "test button",
                "button": "middle"
            }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_ok(),
        "Middle-click should succeed: {:?}",
        result.err()
    );

    // Verify the button was middle-clicked (button 1) using evaluate
    let button_text = eval_tool
        .execute(
            &json!({ "function": "() => document.getElementById('btn').textContent" }),
            &mut browser,
        )
        .await
        .unwrap();

    assert!(
        button_text.contains("button=1"),
        "Button should show button=1 for middle-click, got: {}",
        button_text
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_click_with_ctrl_modifier() {
    use viewpoint_mcp::tools::BrowserSnapshotTool;

    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let click_tool = BrowserClickTool::new();

    // Create a page that tracks Ctrl+click
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<html><body><button id='btn' onclick='if(event.ctrlKey){this.textContent=\"Ctrl-clicked\";}else{this.textContent=\"Normal-clicked\";}'>Click me</button></body></html>" }),
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

    // Perform Ctrl+click
    let result = click_tool
        .execute(
            &json!({
                "ref": ref_str,
                "element": "test button",
                "modifiers": ["Control"]
            }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_ok(),
        "Ctrl+click should succeed: {:?}",
        result.err()
    );

    // Verify the Ctrl modifier was detected
    let snapshot_after = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    assert!(
        snapshot_after.contains("Ctrl-clicked"),
        "Button should have detected Ctrl modifier: {}",
        snapshot_after
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_click_with_shift_modifier() {
    use viewpoint_mcp::tools::BrowserSnapshotTool;

    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let click_tool = BrowserClickTool::new();

    // Create a page that tracks Shift+click
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<html><body><button id='btn' onclick='if(event.shiftKey){this.textContent=\"Shift-clicked\";}else{this.textContent=\"Normal-clicked\";}'>Click me</button></body></html>" }),
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

    // Perform Shift+click
    let result = click_tool
        .execute(
            &json!({
                "ref": ref_str,
                "element": "test button",
                "modifiers": ["Shift"]
            }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_ok(),
        "Shift+click should succeed: {:?}",
        result.err()
    );

    // Verify the Shift modifier was detected
    let snapshot_after = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    assert!(
        snapshot_after.contains("Shift-clicked"),
        "Button should have detected Shift modifier: {}",
        snapshot_after
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_click_with_multiple_modifiers() {
    use viewpoint_mcp::tools::BrowserSnapshotTool;

    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let click_tool = BrowserClickTool::new();

    // Create a page that tracks Ctrl+Shift+click
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<html><body><button id='btn' onclick='if(event.ctrlKey && event.shiftKey){this.textContent=\"Ctrl-Shift-clicked\";}else{this.textContent=\"Other-clicked\";}'>Click me</button></body></html>" }),
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

    // Perform Ctrl+Shift+click
    let result = click_tool
        .execute(
            &json!({
                "ref": ref_str,
                "element": "test button",
                "modifiers": ["Control", "Shift"]
            }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_ok(),
        "Ctrl+Shift+click should succeed: {:?}",
        result.err()
    );

    // Verify both modifiers were detected
    let snapshot_after = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    assert!(
        snapshot_after.contains("Ctrl-Shift-clicked"),
        "Button should have detected both modifiers: {}",
        snapshot_after
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_double_click() {
    use viewpoint_mcp::tools::BrowserSnapshotTool;

    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let click_tool = BrowserClickTool::new();

    // Create a page that tracks double-click
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<html><body><button id='btn' ondblclick='this.textContent=\"Double-clicked\"'>Click me</button></body></html>" }),
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

    // Perform double-click
    let result = click_tool
        .execute(
            &json!({
                "ref": ref_str,
                "element": "test button",
                "doubleClick": true
            }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_ok(),
        "Double-click should succeed: {:?}",
        result.err()
    );

    // Verify the double-click was detected
    let snapshot_after = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    assert!(
        snapshot_after.contains("Double-clicked"),
        "Button should have been double-clicked: {}",
        snapshot_after
    );

    browser.shutdown().await;
}
