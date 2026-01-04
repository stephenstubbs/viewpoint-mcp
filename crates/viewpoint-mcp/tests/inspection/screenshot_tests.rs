//! Screenshot tool integration tests

use serde_json::json;
use viewpoint_mcp::tools::{
    BrowserNavigateTool, BrowserSnapshotTool, BrowserTakeScreenshotTool, Tool,
};

use super::create_browser;

#[tokio::test]
async fn test_screenshot_viewport() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let screenshot_tool = BrowserTakeScreenshotTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Screenshot Test</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = screenshot_tool.execute(&json!({}), &mut browser).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    // Output should be base64 encoded image or contain image data info
    assert!(!output.is_empty());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_screenshot_full_page() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let screenshot_tool = BrowserTakeScreenshotTool::new();

    // Create a tall page
    let html = r#"<div style="height: 3000px; background: linear-gradient(red, blue);">
        <h1>Top</h1>
        <div style="position: absolute; bottom: 0;">Bottom</div>
    </div>"#;

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html) }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = screenshot_tool
        .execute(&json!({ "fullPage": true }), &mut browser)
        .await;
    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_screenshot_jpeg_format() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let screenshot_tool = BrowserTakeScreenshotTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>JPEG Test</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = screenshot_tool
        .execute(&json!({ "type": "jpeg" }), &mut browser)
        .await;
    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_screenshot_element() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let screenshot_tool = BrowserTakeScreenshotTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1 id='title'>Element Screenshot Test</h1><button>Click</button>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Get snapshot to find a ref
    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    eprintln!("Snapshot:\n{snapshot}");

    // Extract a ref from the snapshot - look for a button or heading ref
    // Format is now c{ctx}p{page}f{frame}e{counter} (e.g., c0p0f0e1)
    let ref_pattern = regex::Regex::new(r"\[ref=(c\d+p\d+f\d+e\d+)\]").unwrap();
    let captures: Vec<_> = ref_pattern.captures_iter(&snapshot).collect();
    assert!(
        !captures.is_empty(),
        "Should find at least one ref in snapshot"
    );

    // Use the first ref found
    let element_ref = captures[0].get(1).unwrap().as_str();
    eprintln!("Using ref: {element_ref}");

    // Take element screenshot
    let result = screenshot_tool
        .execute(
            &json!({
                "ref": element_ref,
                "element": "test element"
            }),
            &mut browser,
        )
        .await;

    eprintln!("Screenshot result: {result:?}");
    assert!(
        result.is_ok(),
        "Element screenshot failed: {:?}",
        result.err()
    );

    browser.shutdown().await;
}

/// Test that verifies the bounding_box workaround for element screenshots.
///
/// Note: In viewpoint-core 0.2.16, `locator.screenshot()` doesn't work with
/// ref-based locators (returns NotFound error), but `locator.bounding_box()`
/// works correctly. We use this workaround in browser_take_screenshot.
#[tokio::test]
async fn test_element_screenshot_bounding_box_workaround() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Test</h1><button id='btn'>Click Me</button>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Get the page directly
    let context = browser.active_context().unwrap();
    let page = context.active_page().unwrap();

    // Get snapshot refs
    let snapshot = page
        .aria_snapshot_with_frames()
        .await
        .expect("Snapshot failed");

    // Find a ref
    fn find_first_ref(node: &viewpoint_core::page::locator::aria::AriaSnapshot) -> Option<String> {
        if let Some(r) = &node.node_ref {
            return Some(r.clone());
        }
        for child in &node.children {
            if let Some(r) = find_first_ref(child) {
                return Some(r);
            }
        }
        None
    }

    let ref_str = find_first_ref(&snapshot).expect("No ref found in snapshot");

    // Get locator from ref
    let locator = page.locator_from_ref(&ref_str);

    // Verify bounding_box works (this is our workaround)
    let bbox = locator
        .bounding_box()
        .await
        .expect("bounding_box should succeed")
        .expect("element should have a bounding box");

    // Take page screenshot with clip using bounding box
    let screenshot_bytes = page
        .screenshot()
        .clip(bbox.x, bbox.y, bbox.width, bbox.height)
        .capture()
        .await
        .expect("clip screenshot should succeed");

    assert!(
        !screenshot_bytes.is_empty(),
        "Screenshot should not be empty"
    );

    browser.shutdown().await;
}
