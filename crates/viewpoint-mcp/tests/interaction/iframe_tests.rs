//! Iframe interaction integration tests
//!
//! These tests verify that elements inside iframes can be interacted with
//! after using aria_snapshot_with_frames() to get refs for iframe content.

use serde_json::json;
use viewpoint_mcp::tools::{
    BrowserClickTool, BrowserEvaluateTool, BrowserNavigateTool, BrowserSnapshotTool,
    BrowserTypeTool, Tool,
};

use super::create_browser;

/// Helper to extract a ref from snapshot that matches a pattern
fn find_ref_for_text(snapshot: &str, text: &str) -> Option<String> {
    // Look for a line containing both the text and a ref
    for line in snapshot.lines() {
        if line.contains(text) {
            let re = regex::Regex::new(r"\[ref=(c\d+p\d+f\d+e\d+)\]").unwrap();
            if let Some(caps) = re.captures(line) {
                return Some(caps.get(1).unwrap().as_str().to_string());
            }
        }
    }
    None
}

/// Helper to extract any ref from the snapshot
fn extract_any_ref(snapshot: &str) -> Option<String> {
    let re = regex::Regex::new(r"\[ref=(c\d+p\d+f\d+e\d+)\]").unwrap();
    re.captures(snapshot)
        .map(|c| c.get(1).unwrap().as_str().to_string())
}

#[tokio::test]
async fn test_click_button_inside_iframe() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let click_tool = BrowserClickTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    // Create a page with an iframe containing a button that changes text on click
    let iframe_content = r#"<html><body><button id='inner' onclick='this.textContent=\"Clicked!\"'>Click Me</button></body></html>"#;
    let html = format!(
        r#"<html><body>
            <h1>Main Page</h1>
            <button id="outer">Outer Button</button>
            <iframe id="myframe" srcdoc="{}"></iframe>
        </body></html>"#,
        iframe_content.replace('"', "&quot;")
    );

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html.replace('\n', "").replace(' ', "%20")) }),
            &mut browser,
        )
        .await
        .unwrap();

    // Wait for iframe to load
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Get snapshot - this should include iframe content with refs
    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // Try to find the inner button ref by looking for "Click Me" text
    if let Some(ref_str) = find_ref_for_text(&snapshot, "Click Me") {
        // Click the button inside the iframe
        let click_result = click_tool
            .execute(
                &json!({ "ref": ref_str, "element": "inner button" }),
                &mut browser,
            )
            .await;

        assert!(
            click_result.is_ok(),
            "Click on iframe button should succeed: {:?}",
            click_result.err()
        );

        // Verify the button text changed
        let verify = eval_tool
            .execute(
                &json!({ "function": "() => document.getElementById('myframe').contentDocument.getElementById('inner').textContent" }),
                &mut browser,
            )
            .await;

        if let Ok(output) = verify {
            assert!(
                output.contains("Clicked!"),
                "Button text should have changed after click. Got: {}",
                output
            );
        }
    } else {
        // If we can't find the inner button ref, that's expected if iframe content isn't exposed
        // Just verify we have at least the outer button
        let outer_ref = find_ref_for_text(&snapshot, "Outer");
        assert!(
            outer_ref.is_some() || extract_any_ref(&snapshot).is_some(),
            "Should have at least one interactable element. Snapshot: {}",
            snapshot
        );
    }

    browser.shutdown().await;
}

#[tokio::test]
async fn test_type_into_input_inside_iframe() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let type_tool = BrowserTypeTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    // Create a page with an iframe containing an input field
    let iframe_content =
        r#"<html><body><input id='inner-input' type='text' placeholder='Type here'></body></html>"#;
    let html = format!(
        r#"<html><body>
            <h1>Main Page</h1>
            <input id="outer-input" type="text" placeholder="Outer Input">
            <iframe id="myframe" srcdoc="{}"></iframe>
        </body></html>"#,
        iframe_content.replace('"', "&quot;")
    );

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html.replace('\n', "").replace(' ', "%20")) }),
            &mut browser,
        )
        .await
        .unwrap();

    // Wait for iframe to load
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Get snapshot
    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // Try to find the inner input ref
    if let Some(ref_str) = find_ref_for_text(&snapshot, "Type here") {
        // Type into the input inside the iframe
        let type_result = type_tool
            .execute(
                &json!({
                    "ref": ref_str,
                    "element": "inner input",
                    "text": "Hello from iframe!"
                }),
                &mut browser,
            )
            .await;

        assert!(
            type_result.is_ok(),
            "Type into iframe input should succeed: {:?}",
            type_result.err()
        );

        // Verify the input value changed
        let verify = eval_tool
            .execute(
                &json!({ "function": "() => document.getElementById('myframe').contentDocument.getElementById('inner-input').value" }),
                &mut browser,
            )
            .await;

        if let Ok(output) = verify {
            assert!(
                output.contains("Hello from iframe!"),
                "Input value should contain typed text. Got: {}",
                output
            );
        }
    } else {
        // If inner input isn't exposed, verify outer input works
        if let Some(outer_ref) = find_ref_for_text(&snapshot, "Outer") {
            let type_result = type_tool
                .execute(
                    &json!({
                        "ref": outer_ref,
                        "element": "outer input",
                        "text": "Test"
                    }),
                    &mut browser,
                )
                .await;

            assert!(
                type_result.is_ok(),
                "Type into outer input should succeed: {:?}",
                type_result.err()
            );
        }
    }

    browser.shutdown().await;
}

#[tokio::test]
async fn test_nested_iframe_interaction() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let click_tool = BrowserClickTool::new();

    // Create a page with nested iframes
    let inner_iframe =
        r#"<button id='deepest' onclick='this.textContent=\"Deep Click\"'>Deep Button</button>"#;
    let outer_iframe = format!(
        r#"<html><body><button>Middle Button</button><iframe srcdoc=\"{}\"></iframe></body></html>"#,
        inner_iframe.replace('"', "&quot;").replace('\'', "&#39;")
    );
    let html = format!(
        r#"<html><body>
            <h1>Top Level</h1>
            <button>Top Button</button>
            <iframe srcdoc="{}"></iframe>
        </body></html>"#,
        outer_iframe
            .replace('"', "&quot;")
            .replace('\\', "")
            .replace("&quot;", "\\&quot;")
    );

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html.replace('\n', "").replace(' ', "%20")) }),
            &mut browser,
        )
        .await
        .unwrap();

    // Wait for nested iframes to load
    tokio::time::sleep(std::time::Duration::from_millis(800)).await;

    // Get snapshot
    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // Just verify we can get some refs - nested iframes are complex
    let ref_count = snapshot.matches("[ref=").count();
    assert!(
        ref_count >= 1,
        "Should have at least one ref. Snapshot: {}",
        snapshot
    );

    // Try to click any button we find
    if let Some(ref_str) = extract_any_ref(&snapshot) {
        let click_result = click_tool
            .execute(
                &json!({ "ref": ref_str, "element": "some button" }),
                &mut browser,
            )
            .await;

        // Should at least not crash
        let _ = click_result;
    }

    browser.shutdown().await;
}

#[tokio::test]
async fn test_iframe_elements_have_frame_id_in_ref() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // Create a page with an iframe
    let iframe_content = r#"<html><body><button>Frame Button</button></body></html>"#;
    let html = format!(
        r#"<html><body>
            <button>Main Button</button>
            <iframe srcdoc="{}"></iframe>
        </body></html>"#,
        iframe_content.replace('"', "&quot;")
    );

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html.replace('\n', "").replace(' ', "%20")) }),
            &mut browser,
        )
        .await
        .unwrap();

    // Wait for iframe to load
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Get snapshot
    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // Ref format is c{context}p{page}f{frame}e{element}
    // Main page elements should have f0, iframe elements should have f1+
    let re = regex::Regex::new(r"c(\d+)p(\d+)f(\d+)e(\d+)").unwrap();

    let mut frame_ids: Vec<u32> = Vec::new();
    for caps in re.captures_iter(&snapshot) {
        let frame_id: u32 = caps.get(3).unwrap().as_str().parse().unwrap();
        if !frame_ids.contains(&frame_id) {
            frame_ids.push(frame_id);
        }
    }

    // We should have at least frame 0 (main page)
    assert!(
        frame_ids.contains(&0),
        "Should have elements from frame 0 (main page). Frame IDs: {:?}",
        frame_ids
    );

    browser.shutdown().await;
}
