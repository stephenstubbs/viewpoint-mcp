//! Integration tests for optional capability tools (vision, PDF)
//!
//! Run with:
//! ```sh
//! cargo test --features integration -p viewpoint-mcp --test tools_optional
//! ```
#![cfg(feature = "integration")]

use serde_json::json;
use std::sync::Arc;
use tempfile::TempDir;
use viewpoint_mcp::browser::{BrowserConfig, BrowserState};
use viewpoint_mcp::tools::{
    BrowserMouseClickXyTool, BrowserMouseDragXyTool, BrowserMouseMoveXyTool,
    BrowserNavigateTool, BrowserPdfSaveTool, Capability, Tool, ToolRegistry,
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
// Capability-Based Tool Filtering Tests
// =============================================================================

#[test]
fn test_registry_without_capabilities() {
    let mut registry = ToolRegistry::new();

    // Register tools
    registry.register(Arc::new(BrowserNavigateTool::new()));
    registry.register(Arc::new(BrowserMouseClickXyTool::new()));
    registry.register(Arc::new(BrowserMouseMoveXyTool::new()));
    registry.register(Arc::new(BrowserMouseDragXyTool::new()));
    registry.register(Arc::new(BrowserPdfSaveTool::new()));

    // Without capabilities, vision and PDF tools should be hidden
    let available = registry.list();
    let names: Vec<_> = available.iter().map(|t| t.name()).collect();

    assert!(names.contains(&"browser_navigate"));
    assert!(!names.contains(&"browser_mouse_click_xy"));
    assert!(!names.contains(&"browser_mouse_move_xy"));
    assert!(!names.contains(&"browser_mouse_drag_xy"));
    assert!(!names.contains(&"browser_pdf_save"));
}

#[test]
fn test_registry_with_vision_capability() {
    let mut registry = ToolRegistry::with_capabilities([Capability::Vision]);

    registry.register(Arc::new(BrowserNavigateTool::new()));
    registry.register(Arc::new(BrowserMouseClickXyTool::new()));
    registry.register(Arc::new(BrowserMouseMoveXyTool::new()));
    registry.register(Arc::new(BrowserMouseDragXyTool::new()));
    registry.register(Arc::new(BrowserPdfSaveTool::new()));

    let available = registry.list();
    let names: Vec<_> = available.iter().map(|t| t.name()).collect();

    // Vision tools should be available
    assert!(names.contains(&"browser_navigate"));
    assert!(names.contains(&"browser_mouse_click_xy"));
    assert!(names.contains(&"browser_mouse_move_xy"));
    assert!(names.contains(&"browser_mouse_drag_xy"));
    // PDF should still be hidden
    assert!(!names.contains(&"browser_pdf_save"));
}

#[test]
fn test_registry_with_pdf_capability() {
    let mut registry = ToolRegistry::with_capabilities([Capability::Pdf]);

    registry.register(Arc::new(BrowserNavigateTool::new()));
    registry.register(Arc::new(BrowserMouseClickXyTool::new()));
    registry.register(Arc::new(BrowserPdfSaveTool::new()));

    let available = registry.list();
    let names: Vec<_> = available.iter().map(|t| t.name()).collect();

    assert!(names.contains(&"browser_navigate"));
    assert!(names.contains(&"browser_pdf_save"));
    // Vision should be hidden
    assert!(!names.contains(&"browser_mouse_click_xy"));
}

#[test]
fn test_registry_with_all_capabilities() {
    let mut registry = ToolRegistry::with_capabilities([Capability::Vision, Capability::Pdf]);

    registry.register(Arc::new(BrowserNavigateTool::new()));
    registry.register(Arc::new(BrowserMouseClickXyTool::new()));
    registry.register(Arc::new(BrowserMouseMoveXyTool::new()));
    registry.register(Arc::new(BrowserMouseDragXyTool::new()));
    registry.register(Arc::new(BrowserPdfSaveTool::new()));

    let available = registry.list();
    assert_eq!(available.len(), 5); // All tools available
}

#[test]
fn test_get_unavailable_tool() {
    let mut registry = ToolRegistry::new();
    registry.register(Arc::new(BrowserMouseClickXyTool::new()));

    // get() should return None for capability-gated tools
    assert!(registry.get("browser_mouse_click_xy").is_none());

    // get_unchecked() should still return the tool
    assert!(registry.get_unchecked("browser_mouse_click_xy").is_some());
}

#[test]
fn test_required_capability_vision_tools() {
    assert_eq!(
        BrowserMouseClickXyTool::new().required_capability(),
        Some(Capability::Vision)
    );
    assert_eq!(
        BrowserMouseMoveXyTool::new().required_capability(),
        Some(Capability::Vision)
    );
    assert_eq!(
        BrowserMouseDragXyTool::new().required_capability(),
        Some(Capability::Vision)
    );
}

#[test]
fn test_required_capability_pdf_tool() {
    assert_eq!(
        BrowserPdfSaveTool::new().required_capability(),
        Some(Capability::Pdf)
    );
}

#[test]
fn test_required_capability_standard_tool() {
    assert_eq!(BrowserNavigateTool::new().required_capability(), None);
}

// =============================================================================
// browser_mouse_click_xy Tests
// =============================================================================

#[tokio::test]
async fn test_mouse_click_xy_basic() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let click_tool = BrowserMouseClickXyTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<button style='position:absolute;left:100px;top:100px;width:100px;height:50px'>Click</button>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = click_tool
        .execute(
            &json!({ "x": 150, "y": 125, "element": "button" }),
            &mut browser,
        )
        .await;

    assert!(result.is_ok(), "Click at coordinates should succeed: {:?}", result.err());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_mouse_click_xy_double_click() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let click_tool = BrowserMouseClickXyTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<div>Double click me</div>" }), &mut browser)
        .await
        .unwrap();

    let result = click_tool
        .execute(
            &json!({ "x": 100, "y": 100, "clickCount": 2 }),
            &mut browser,
        )
        .await;

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Double"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_mouse_click_xy_right_button() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let click_tool = BrowserMouseClickXyTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<div>Right click me</div>" }), &mut browser)
        .await
        .unwrap();

    let result = click_tool
        .execute(
            &json!({ "x": 100, "y": 100, "button": "right" }),
            &mut browser,
        )
        .await;

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("right"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_mouse_click_xy_middle_button() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let click_tool = BrowserMouseClickXyTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<div>Middle click</div>" }), &mut browser)
        .await
        .unwrap();

    let result = click_tool
        .execute(
            &json!({ "x": 100, "y": 100, "button": "middle" }),
            &mut browser,
        )
        .await;

    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_mouse_click_xy_negative_coordinates() {
    let mut browser = create_browser().await;
    let click_tool = BrowserMouseClickXyTool::new();

    let result = click_tool
        .execute(&json!({ "x": -10, "y": 100 }), &mut browser)
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("non-negative") || err.contains("invalid"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_mouse_click_xy_edge_of_viewport() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let click_tool = BrowserMouseClickXyTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<div>Test</div>" }), &mut browser)
        .await
        .unwrap();

    // Click at corner
    let result = click_tool
        .execute(&json!({ "x": 0, "y": 0 }), &mut browser)
        .await;

    assert!(result.is_ok());

    browser.shutdown().await;
}

// =============================================================================
// browser_mouse_move_xy Tests
// =============================================================================

#[tokio::test]
async fn test_mouse_move_xy_basic() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let move_tool = BrowserMouseMoveXyTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<div>Hover target</div>" }), &mut browser)
        .await
        .unwrap();

    let result = move_tool
        .execute(&json!({ "x": 100, "y": 100 }), &mut browser)
        .await;

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("100"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_mouse_move_xy_with_steps() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let move_tool = BrowserMouseMoveXyTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<div>Test</div>" }), &mut browser)
        .await
        .unwrap();

    let result = move_tool
        .execute(&json!({ "x": 200, "y": 200, "steps": 10 }), &mut browser)
        .await;

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("10 steps"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_mouse_move_xy_instant() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let move_tool = BrowserMouseMoveXyTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<div>Test</div>" }), &mut browser)
        .await
        .unwrap();

    // steps=1 means instant move
    let result = move_tool
        .execute(&json!({ "x": 50, "y": 50, "steps": 1 }), &mut browser)
        .await;

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.contains("steps")); // Instant move doesn't mention steps

    browser.shutdown().await;
}

// =============================================================================
// browser_mouse_drag_xy Tests
// =============================================================================

#[tokio::test]
async fn test_mouse_drag_xy_basic() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let drag_tool = BrowserMouseDragXyTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<div draggable='true' style='width:50px;height:50px;background:blue'>Drag me</div>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = drag_tool
        .execute(
            &json!({
                "startX": 25,
                "startY": 25,
                "endX": 200,
                "endY": 200
            }),
            &mut browser,
        )
        .await;

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Dragged"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_mouse_drag_xy_with_steps() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let drag_tool = BrowserMouseDragXyTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<div>Test</div>" }), &mut browser)
        .await
        .unwrap();

    let result = drag_tool
        .execute(
            &json!({
                "startX": 50,
                "startY": 50,
                "endX": 300,
                "endY": 300,
                "steps": 25
            }),
            &mut browser,
        )
        .await;

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("25 steps"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_mouse_drag_xy_negative_coordinates() {
    let mut browser = create_browser().await;
    let drag_tool = BrowserMouseDragXyTool::new();

    let result = drag_tool
        .execute(
            &json!({
                "startX": -10,
                "startY": 50,
                "endX": 100,
                "endY": 100
            }),
            &mut browser,
        )
        .await;

    assert!(result.is_err());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_mouse_drag_xy_same_position() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let drag_tool = BrowserMouseDragXyTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<div>Test</div>" }), &mut browser)
        .await
        .unwrap();

    // Drag to same position
    let result = drag_tool
        .execute(
            &json!({
                "startX": 100,
                "startY": 100,
                "endX": 100,
                "endY": 100
            }),
            &mut browser,
        )
        .await;

    assert!(result.is_ok()); // Should succeed, just no actual movement

    browser.shutdown().await;
}

// =============================================================================
// browser_pdf_save Tests
// =============================================================================

#[tokio::test]
async fn test_pdf_save_basic() {
    let temp_dir = TempDir::new().unwrap();
    let pdf_path = temp_dir.path().join("test.pdf");

    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let pdf_tool = BrowserPdfSaveTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>PDF Test</h1>" }), &mut browser)
        .await
        .unwrap();

    let result = pdf_tool
        .execute(&json!({ "path": pdf_path.to_str().unwrap() }), &mut browser)
        .await;

    assert!(result.is_ok(), "PDF save should succeed: {:?}", result.err());
    assert!(pdf_path.exists(), "PDF file should exist");

    // Check it's a valid PDF (starts with %PDF)
    let content = std::fs::read(&pdf_path).unwrap();
    assert!(content.starts_with(b"%PDF"), "Should be valid PDF");

    browser.shutdown().await;
}

#[tokio::test]
async fn test_pdf_save_a4_format() {
    let temp_dir = TempDir::new().unwrap();
    let pdf_path = temp_dir.path().join("a4.pdf");

    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let pdf_tool = BrowserPdfSaveTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>A4 PDF</h1>" }), &mut browser)
        .await
        .unwrap();

    let result = pdf_tool
        .execute(
            &json!({
                "path": pdf_path.to_str().unwrap(),
                "format": "a4"
            }),
            &mut browser,
        )
        .await;

    assert!(result.is_ok());
    assert!(pdf_path.exists());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_pdf_save_landscape() {
    let temp_dir = TempDir::new().unwrap();
    let pdf_path = temp_dir.path().join("landscape.pdf");

    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let pdf_tool = BrowserPdfSaveTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Landscape</h1>" }), &mut browser)
        .await
        .unwrap();

    let result = pdf_tool
        .execute(
            &json!({
                "path": pdf_path.to_str().unwrap(),
                "landscape": true
            }),
            &mut browser,
        )
        .await;

    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_pdf_save_with_background() {
    let temp_dir = TempDir::new().unwrap();
    let pdf_path = temp_dir.path().join("background.pdf");

    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let pdf_tool = BrowserPdfSaveTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<body style='background:blue'><h1 style='color:white'>Background</h1></body>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = pdf_tool
        .execute(
            &json!({
                "path": pdf_path.to_str().unwrap(),
                "printBackground": true
            }),
            &mut browser,
        )
        .await;

    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_pdf_save_with_scale() {
    let temp_dir = TempDir::new().unwrap();
    let pdf_path = temp_dir.path().join("scaled.pdf");

    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let pdf_tool = BrowserPdfSaveTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Scaled</h1>" }), &mut browser)
        .await
        .unwrap();

    let result = pdf_tool
        .execute(
            &json!({
                "path": pdf_path.to_str().unwrap(),
                "scale": 0.5
            }),
            &mut browser,
        )
        .await;

    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_pdf_save_invalid_scale() {
    let temp_dir = TempDir::new().unwrap();
    let pdf_path = temp_dir.path().join("invalid.pdf");

    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let pdf_tool = BrowserPdfSaveTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Test</h1>" }), &mut browser)
        .await
        .unwrap();

    // Scale out of range
    let result = pdf_tool
        .execute(
            &json!({
                "path": pdf_path.to_str().unwrap(),
                "scale": 5.0
            }),
            &mut browser,
        )
        .await;

    assert!(result.is_err());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_pdf_save_with_margin() {
    let temp_dir = TempDir::new().unwrap();
    let pdf_path = temp_dir.path().join("margin.pdf");

    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let pdf_tool = BrowserPdfSaveTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Margins</h1>" }), &mut browser)
        .await
        .unwrap();

    let result = pdf_tool
        .execute(
            &json!({
                "path": pdf_path.to_str().unwrap(),
                "margin": 1.0
            }),
            &mut browser,
        )
        .await;

    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_pdf_save_empty_path() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let pdf_tool = BrowserPdfSaveTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Test</h1>" }), &mut browser)
        .await
        .unwrap();

    let result = pdf_tool.execute(&json!({ "path": "" }), &mut browser).await;

    assert!(result.is_err());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_pdf_save_all_formats() {
    let temp_dir = TempDir::new().unwrap();

    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let pdf_tool = BrowserPdfSaveTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Format Test</h1>" }), &mut browser)
        .await
        .unwrap();

    let formats = ["letter", "legal", "tabloid", "a3", "a4", "a5"];

    for format in formats {
        let pdf_path = temp_dir.path().join(format!("{}.pdf", format));
        let result = pdf_tool
            .execute(
                &json!({
                    "path": pdf_path.to_str().unwrap(),
                    "format": format
                }),
                &mut browser,
            )
            .await;

        assert!(result.is_ok(), "Format {} should work", format);
        assert!(pdf_path.exists(), "PDF for {} should exist", format);
    }

    browser.shutdown().await;
}
