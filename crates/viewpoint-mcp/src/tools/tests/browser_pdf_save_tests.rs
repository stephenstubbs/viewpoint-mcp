//! Tests for `browser_pdf_save` tool

use crate::tools::Tool;
use crate::tools::browser_pdf_save::{BrowserPdfSaveInput, BrowserPdfSaveTool, PaperFormat};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserPdfSaveTool::new();

    assert_eq!(tool.name(), "browser_pdf_save");
    assert!(tool.description().contains("PDF"));

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("path"))
    );
}

#[test]
fn test_input_parsing_minimal() {
    let input: BrowserPdfSaveInput = serde_json::from_value(json!({
        "path": "/tmp/test.pdf"
    }))
    .unwrap();

    assert_eq!(input.path, "/tmp/test.pdf");
    assert_eq!(input.format, PaperFormat::Letter);
    assert!(!input.landscape);
    assert!(!input.print_background);
    assert!(input.scale.is_none());
    assert!(input.page_ranges.is_none());
    assert!(input.margin.is_none());
}

#[test]
fn test_input_parsing_with_options() {
    let input: BrowserPdfSaveInput = serde_json::from_value(json!({
        "path": "/tmp/report.pdf",
        "format": "a4",
        "landscape": true,
        "printBackground": true,
        "scale": 0.8,
        "pageRanges": "1-5, 10",
        "margin": 0.5
    }))
    .unwrap();

    assert_eq!(input.path, "/tmp/report.pdf");
    assert_eq!(input.format, PaperFormat::A4);
    assert!(input.landscape);
    assert!(input.print_background);
    assert_eq!(input.scale, Some(0.8));
    assert_eq!(input.page_ranges, Some("1-5, 10".to_string()));
    assert_eq!(input.margin, Some(0.5));
}

#[test]
fn test_paper_format_conversion() {
    let letter: viewpoint_core::PaperFormat = PaperFormat::Letter.into();
    assert!(matches!(letter, viewpoint_core::PaperFormat::Letter));

    let a4: viewpoint_core::PaperFormat = PaperFormat::A4.into();
    assert!(matches!(a4, viewpoint_core::PaperFormat::A4));
}
