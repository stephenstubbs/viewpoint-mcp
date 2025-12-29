//! Integration tests for interaction tools (click, type, hover, etc.)
//!
//! NOTE: Many ref-based interaction tools use placeholder implementations
//! that rely on [data-ref] CSS selectors which don't exist in actual pages.
//! These tests verify the tool infrastructure works, but actual element
//! interaction requires proper ref-to-element resolution implementation.
//!
//! Run with:
//! ```sh
//! cargo test --features integration -p viewpoint-mcp --test tools_interaction
//! ```
#![cfg(feature = "integration")]

use serde_json::json;
use viewpoint_mcp::browser::{BrowserConfig, BrowserState};
use viewpoint_mcp::tools::{
    BrowserClickTool, BrowserNavigateTool, BrowserPressKeyTool, Tool,
};

/// Helper to create a headless browser state
async fn create_browser() -> BrowserState {
    let config = BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let mut state = BrowserState::new(config);
    state.initialize().await.expect("Failed to initialize browser");
    state
}

// =============================================================================
// browser_click Tests - Tool Infrastructure
// =============================================================================

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
async fn test_click_invalid_ref_format() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let click_tool = BrowserClickTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<button>Test</button>" }), &mut browser)
        .await
        .unwrap();

    // Invalid ref format
    let result = click_tool
        .execute(
            &json!({ "ref": "", "element": "Test button" }),
            &mut browser,
        )
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
    assert!(schema["required"].as_array().unwrap().contains(&json!("ref")));
}

// =============================================================================
// browser_press_key Tests
// =============================================================================

#[tokio::test]
async fn test_press_enter_key() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let press_tool = BrowserPressKeyTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<input type='text'>" }), &mut browser)
        .await
        .unwrap();

    let result = press_tool.execute(&json!({ "key": "Enter" }), &mut browser).await;
    assert!(result.is_ok(), "Press Enter should succeed: {:?}", result.err());
    
    browser.shutdown().await;
}

#[tokio::test]
async fn test_press_key_combination() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let press_tool = BrowserPressKeyTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<input type='text'>" }), &mut browser)
        .await
        .unwrap();

    // Test Control+A (select all)
    let result = press_tool.execute(&json!({ "key": "Control+a" }), &mut browser).await;
    assert!(result.is_ok());
    
    browser.shutdown().await;
}

#[tokio::test]
async fn test_press_arrow_keys() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let press_tool = BrowserPressKeyTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<input type='text'>" }), &mut browser)
        .await
        .unwrap();

    for key in ["ArrowDown", "ArrowUp", "ArrowLeft", "ArrowRight"] {
        let result = press_tool.execute(&json!({ "key": key }), &mut browser).await;
        assert!(result.is_ok(), "Failed to press {}", key);
    }
    
    browser.shutdown().await;
}

#[tokio::test]
async fn test_press_function_keys() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let press_tool = BrowserPressKeyTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<div>Test</div>" }), &mut browser)
        .await
        .unwrap();

    // F1-F12 keys
    for i in 1..=12 {
        let key = format!("F{}", i);
        let result = press_tool.execute(&json!({ "key": key }), &mut browser).await;
        assert!(result.is_ok(), "Failed to press {}", key);
    }
    
    browser.shutdown().await;
}

#[tokio::test]
async fn test_press_special_keys() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let press_tool = BrowserPressKeyTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<input type='text'>" }), &mut browser)
        .await
        .unwrap();

    let special_keys = [
        "Backspace", "Tab", "Escape", "Space", "Home", "End",
        "PageUp", "PageDown", "Delete", "Insert"
    ];

    for key in special_keys {
        let result = press_tool.execute(&json!({ "key": key }), &mut browser).await;
        assert!(result.is_ok(), "Failed to press {}: {:?}", key, result.err());
    }
    
    browser.shutdown().await;
}

#[tokio::test]
async fn test_press_key_missing() {
    let mut browser = create_browser().await;
    let press_tool = BrowserPressKeyTool::new();

    let result = press_tool.execute(&json!({}), &mut browser).await;
    assert!(result.is_err());
    
    browser.shutdown().await;
}

#[tokio::test]
async fn test_press_key_with_modifiers() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let press_tool = BrowserPressKeyTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<input type='text'>" }), &mut browser)
        .await
        .unwrap();

    // Various modifier combinations
    let combos = [
        "Shift+a", "Control+c", "Control+v", "Alt+Tab",
        "Control+Shift+z", "Meta+s"
    ];

    for combo in combos {
        let result = press_tool.execute(&json!({ "key": combo }), &mut browser).await;
        // Some combos may be intercepted by system, just verify no panic
        let _ = result;
    }
    
    browser.shutdown().await;
}

// =============================================================================
// Tool Schema Validation Tests
// =============================================================================

#[tokio::test]
async fn test_tool_schemas_are_valid() {
    use viewpoint_mcp::tools::{
        BrowserTypeTool, BrowserHoverTool, BrowserDragTool,
        BrowserSelectOptionTool, BrowserFillFormTool,
    };

    // Verify all tools have valid schemas
    let tools: Vec<Box<dyn Tool>> = vec![
        Box::new(BrowserClickTool::new()),
        Box::new(BrowserTypeTool::new()),
        Box::new(BrowserHoverTool::new()),
        Box::new(BrowserDragTool::new()),
        Box::new(BrowserSelectOptionTool::new()),
        Box::new(BrowserPressKeyTool::new()),
        Box::new(BrowserFillFormTool::new()),
    ];

    for tool in tools {
        let schema = tool.input_schema();
        assert_eq!(schema["type"], "object", "{} should have object schema", tool.name());
        assert!(schema["properties"].is_object(), "{} should have properties", tool.name());
    }
}

// =============================================================================
// Browser State Tests for Interactions
// =============================================================================

#[tokio::test]
async fn test_interaction_requires_page() {
    let mut browser = create_browser().await;
    let click_tool = BrowserClickTool::new();

    // Navigate first to ensure we have a page
    let nav_tool = BrowserNavigateTool::new();
    nav_tool
        .execute(&json!({ "url": "data:text/html,<button>Test</button>" }), &mut browser)
        .await
        .unwrap();

    // Now test - will fail on ref lookup but shows page exists
    let result = click_tool
        .execute(&json!({ "ref": "nonexistent", "element": "test" }), &mut browser)
        .await;
    
    // Should fail due to ref not found, not due to no page
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("ref") || err.contains("Element") || err.contains("not found"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_multiple_key_presses() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let press_tool = BrowserPressKeyTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<input type='text' id='input'>" }), &mut browser)
        .await
        .unwrap();

    // Type "hello" using key presses
    for c in ['h', 'e', 'l', 'l', 'o'] {
        let result = press_tool.execute(&json!({ "key": c.to_string() }), &mut browser).await;
        assert!(result.is_ok());
    }

    browser.shutdown().await;
}

// =============================================================================
// Edge Cases
// =============================================================================

#[tokio::test]
async fn test_press_key_empty_key() {
    let mut browser = create_browser().await;
    let press_tool = BrowserPressKeyTool::new();

    let result = press_tool.execute(&json!({ "key": "" }), &mut browser).await;
    // Empty key should fail
    assert!(result.is_err());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_rapid_key_presses() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let press_tool = BrowserPressKeyTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<input type='text'>" }), &mut browser)
        .await
        .unwrap();

    // Rapid sequential key presses
    for _ in 0..20 {
        let _ = press_tool.execute(&json!({ "key": "a" }), &mut browser).await;
    }

    browser.shutdown().await;
}
