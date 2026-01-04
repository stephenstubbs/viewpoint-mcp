//! Basic snapshot tool integration tests

use serde_json::json;
use viewpoint_mcp::tools::{BrowserNavigateTool, BrowserSnapshotTool, Tool};

use super::create_browser;

#[tokio::test]
async fn test_snapshot_basic_page() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Hello World</h1><button>Click</button>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = snapshot_tool.execute(&json!({}), &mut browser).await;
    assert!(
        result.is_ok(),
        "Snapshot should succeed: {:?}",
        result.err()
    );

    let snapshot = result.unwrap();
    assert!(snapshot.contains("element"), "Should contain element info");
    assert!(snapshot.contains("ref"), "Should contain refs");

    browser.shutdown().await;
}

#[tokio::test]
async fn test_snapshot_with_all_refs() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // Create page with many elements
    let mut buttons = String::new();
    for i in 0..50 {
        buttons.push_str(&format!("<button>Button {i}</button>"));
    }
    let html = format!("<html><body>{buttons}</body></html>");

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html) }),
            &mut browser,
        )
        .await
        .unwrap();

    // Without allRefs
    let result1 = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // With allRefs
    let result2 = snapshot_tool
        .execute(&json!({ "allRefs": true }), &mut browser)
        .await
        .unwrap();

    // Both should contain buttons
    assert!(result1.contains("button"));
    assert!(result2.contains("button"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_snapshot_form_elements() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    let html = r#"<form>
        <input type="text" placeholder="Username">
        <input type="password" placeholder="Password">
        <input type="email" placeholder="Email">
        <textarea placeholder="Comments"></textarea>
        <select><option>Option 1</option><option>Option 2</option></select>
        <input type="checkbox" id="agree"> Agree
        <input type="radio" name="choice" value="a"> A
        <input type="radio" name="choice" value="b"> B
        <button type="submit">Submit</button>
    </form>"#;

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html) }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = snapshot_tool.execute(&json!({}), &mut browser).await;
    assert!(result.is_ok());

    let snapshot = result.unwrap();
    assert!(snapshot.contains("textbox"));
    assert!(snapshot.contains("button"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_snapshot_empty_page() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<html><body></body></html>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = snapshot_tool.execute(&json!({}), &mut browser).await;
    // Empty page might succeed with minimal content or fail
    // Just ensure no panic - outcome is implementation-dependent
    let _ = result;

    browser.shutdown().await;
}

#[tokio::test]
async fn test_snapshot_after_navigation() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // First page
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Page 1</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let snap1 = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    assert!(!snap1.is_empty(), "First snapshot should not be empty");

    // Second page
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Page 2</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let snap2 = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    assert!(!snap2.is_empty(), "Second snapshot should not be empty");

    browser.shutdown().await;
}
