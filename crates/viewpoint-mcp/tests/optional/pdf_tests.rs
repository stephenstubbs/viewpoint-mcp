//! PDF save tool integration tests

use serde_json::json;
use tempfile::TempDir;
use viewpoint_mcp::tools::{BrowserNavigateTool, BrowserPdfSaveTool, Tool};

use super::create_browser;

#[tokio::test]
async fn test_pdf_save_basic() {
    let temp_dir = TempDir::new().unwrap();
    let pdf_path = temp_dir.path().join("test.pdf");

    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let pdf_tool = BrowserPdfSaveTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>PDF Test</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = pdf_tool
        .execute(&json!({ "path": pdf_path.to_str().unwrap() }), &mut browser)
        .await;

    assert!(
        result.is_ok(),
        "PDF save should succeed: {:?}",
        result.err()
    );
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
        .execute(
            &json!({ "url": "data:text/html,<h1>A4 PDF</h1>" }),
            &mut browser,
        )
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
        .execute(
            &json!({ "url": "data:text/html,<h1>Landscape</h1>" }),
            &mut browser,
        )
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
        .execute(
            &json!({ "url": "data:text/html,<h1>Scaled</h1>" }),
            &mut browser,
        )
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
        .execute(
            &json!({ "url": "data:text/html,<h1>Test</h1>" }),
            &mut browser,
        )
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
        .execute(
            &json!({ "url": "data:text/html,<h1>Margins</h1>" }),
            &mut browser,
        )
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
        .execute(
            &json!({ "url": "data:text/html,<h1>Test</h1>" }),
            &mut browser,
        )
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
        .execute(
            &json!({ "url": "data:text/html,<h1>Format Test</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let formats = ["letter", "legal", "tabloid", "a3", "a4", "a5"];

    for format in formats {
        let pdf_path = temp_dir.path().join(format!("{format}.pdf"));
        let result = pdf_tool
            .execute(
                &json!({
                    "path": pdf_path.to_str().unwrap(),
                    "format": format
                }),
                &mut browser,
            )
            .await;

        assert!(result.is_ok(), "Format {format} should work");
        assert!(pdf_path.exists(), "PDF for {format} should exist");
    }

    browser.shutdown().await;
}
