//! Evaluate tool integration tests

use serde_json::json;
use viewpoint_mcp::tools::{BrowserEvaluateTool, BrowserNavigateTool, BrowserSnapshotTool, Tool};

use super::create_browser;

#[tokio::test]
async fn test_evaluate_simple_expression() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Test</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = eval_tool
        .execute(&json!({ "function": "() => 2 + 2" }), &mut browser)
        .await;
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("4"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_evaluate_document_title() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<html><head><title>My Title</title></head></html>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = eval_tool
        .execute(&json!({ "function": "() => document.title" }), &mut browser)
        .await;
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("My Title"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_evaluate_dom_query() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<div class='test'>Content</div>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = eval_tool
        .execute(
            &json!({ "function": "() => document.querySelector('.test').textContent" }),
            &mut browser,
        )
        .await;
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("Content"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_evaluate_returns_object() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Test</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = eval_tool
        .execute(
            &json!({ "function": "() => ({ name: 'test', value: 42 })" }),
            &mut browser,
        )
        .await;
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("name") || output.contains("test"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_evaluate_error_handling() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Test</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = eval_tool
        .execute(
            &json!({ "function": "() => { throw new Error('Test error'); }" }),
            &mut browser,
        )
        .await;

    // Should error or return error info
    let _ = result;

    browser.shutdown().await;
}

#[tokio::test]
async fn test_evaluate_async_function() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Test</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = eval_tool
        .execute(
            &json!({ "function": "async () => { await new Promise(r => setTimeout(r, 100)); return 'done'; }" }),
            &mut browser,
        )
        .await;
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("done"));

    browser.shutdown().await;
}

/// Helper to extract first ref from snapshot
fn extract_first_ref(snapshot: &str) -> Option<String> {
    let re = regex::Regex::new(r"\[ref=(c\d+p\d+f\d+e\d+)\]").unwrap();
    re.captures(snapshot)
        .map(|c| c.get(1).unwrap().as_str().to_string())
}

#[tokio::test]
async fn test_evaluate_element_scoped_returns_string() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    // Create page with a button that has text content
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<html><body><button id='btn'>Hello World</button></body></html>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Get the button ref from snapshot
    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let ref_str = extract_first_ref(&snapshot).expect("Should find button ref");

    // Evaluate with element ref - get textContent (string)
    let result = eval_tool
        .execute(
            &json!({
                "function": "(el) => el.textContent",
                "ref": ref_str,
                "element": "test button"
            }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_ok(),
        "Element-scoped evaluate should succeed: {:?}",
        result.err()
    );

    let output = result.unwrap();
    assert!(
        output.contains("Hello World"),
        "Should return element's textContent. Got: {}",
        output
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_evaluate_element_scoped_returns_object() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    // Create page with a button
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<html><body><button id='myBtn' data-value='42'>Test</button></body></html>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Get the button ref from snapshot
    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let ref_str = extract_first_ref(&snapshot).expect("Should find button ref");

    // Evaluate with element ref - return an object with element properties
    let result = eval_tool
        .execute(
            &json!({
                "function": "(el) => ({ id: el.id, tagName: el.tagName, dataValue: el.dataset.value })",
                "ref": ref_str,
                "element": "test button"
            }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_ok(),
        "Element-scoped evaluate returning object should succeed: {:?}",
        result.err()
    );

    let output = result.unwrap();
    assert!(
        output.contains("myBtn") && output.contains("BUTTON") && output.contains("42"),
        "Should return object with element properties. Got: {}",
        output
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_evaluate_element_scoped_returns_null() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    // Create page with a button without data attribute
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<html><body><button>Test</button></body></html>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Get the button ref from snapshot
    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let ref_str = extract_first_ref(&snapshot).expect("Should find button ref");

    // Evaluate with element ref - getAttribute returns null for non-existent attr
    let result = eval_tool
        .execute(
            &json!({
                "function": "(el) => el.getAttribute('nonexistent')",
                "ref": ref_str,
                "element": "test button"
            }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_ok(),
        "Element-scoped evaluate returning null should succeed: {:?}",
        result.err()
    );

    let output = result.unwrap();
    assert!(
        output.contains("null"),
        "Should return null for non-existent attribute. Got: {}",
        output
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_evaluate_element_scoped_modifies_element() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    // Create page with a button
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<html><body><button id='btn'>Original</button></body></html>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Get the button ref from snapshot
    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let ref_str = extract_first_ref(&snapshot).expect("Should find button ref");

    // Modify the element's text content
    let result = eval_tool
        .execute(
            &json!({
                "function": "(el) => { el.textContent = 'Modified'; return el.textContent; }",
                "ref": ref_str,
                "element": "test button"
            }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_ok(),
        "Element-scoped evaluate modifying element should succeed: {:?}",
        result.err()
    );

    // Verify modification persisted
    let verify_result = eval_tool
        .execute(
            &json!({ "function": "() => document.getElementById('btn').textContent" }),
            &mut browser,
        )
        .await
        .unwrap();

    assert!(
        verify_result.contains("Modified"),
        "Element should have been modified. Got: {}",
        verify_result
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_evaluate_element_requires_element_description() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<button>Test</button>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let snapshot = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let ref_str = extract_first_ref(&snapshot).expect("Should find button ref");

    // Try to use ref without element description - should fail
    let result = eval_tool
        .execute(
            &json!({
                "function": "(el) => el.textContent",
                "ref": ref_str
                // Missing "element" field
            }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_err(),
        "Should fail when ref provided without element description"
    );
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("element") && err.contains("required"),
        "Error should mention element is required. Got: {}",
        err
    );

    browser.shutdown().await;
}
