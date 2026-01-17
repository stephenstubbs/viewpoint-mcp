//! Tests for `browser_take_screenshot` tool

use crate::tools::Tool;
use crate::tools::browser_take_screenshot::{
    BrowserTakeScreenshotInput, BrowserTakeScreenshotTool, ImageFormat, MAX_INLINE_DIMENSION,
    MAX_INLINE_MEGAPIXELS, scale_image_for_inline,
};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserTakeScreenshotTool::new();

    assert_eq!(tool.name(), "browser_take_screenshot");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
}

#[test]
fn test_input_defaults() {
    let input: BrowserTakeScreenshotInput = serde_json::from_value(json!({})).unwrap();

    assert!(input.element_ref.is_none());
    assert!(!input.full_page);
    assert!(matches!(input.image_type, ImageFormat::Png));
}

#[test]
fn test_input_full_page() {
    let input: BrowserTakeScreenshotInput = serde_json::from_value(json!({
        "fullPage": true,
        "type": "jpeg"
    }))
    .unwrap();

    assert!(input.full_page);
    assert!(matches!(input.image_type, ImageFormat::Jpeg));
}

#[test]
fn test_input_element_screenshot() {
    let input: BrowserTakeScreenshotInput = serde_json::from_value(json!({
        "ref": "e1a2b3c",
        "element": "Login form"
    }))
    .unwrap();

    assert_eq!(input.element_ref, Some("e1a2b3c".to_string()));
    assert_eq!(input.element, Some("Login form".to_string()));
}

// =============================================================================
// Image scaling tests
// =============================================================================

/// Create a test image in PNG format with the given dimensions
fn create_test_png(width: u32, height: u32) -> Vec<u8> {
    use image::{ImageFormat, RgbImage};
    use std::io::Cursor;

    let img = RgbImage::new(width, height);
    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageFormat::Png).unwrap();
    buffer.into_inner()
}

#[test]
fn test_scale_small_image_unchanged_dimensions() {
    // A small image should not be scaled down (dimensions should fit)
    let png_bytes = create_test_png(800, 600);
    let result = scale_image_for_inline(&png_bytes).unwrap();

    // Result should be valid JPEG
    let decoded = image::load_from_memory(&result).unwrap();

    // Original is within limits, so dimensions should be preserved
    assert_eq!(decoded.width(), 800);
    assert_eq!(decoded.height(), 600);
}

#[test]
fn test_scale_wide_image() {
    // Image wider than MAX_INLINE_DIMENSION should be scaled
    let png_bytes = create_test_png(3000, 600);
    let result = scale_image_for_inline(&png_bytes).unwrap();

    let decoded = image::load_from_memory(&result).unwrap();

    // Width should be scaled to MAX_INLINE_DIMENSION
    assert!(
        decoded.width() <= MAX_INLINE_DIMENSION,
        "Width {} should be <= {}",
        decoded.width(),
        MAX_INLINE_DIMENSION
    );
    // Aspect ratio should be preserved (approximately)
    let expected_height = (600.0 * (decoded.width() as f64 / 3000.0)).round() as u32;
    assert!((decoded.height() as i32 - expected_height as i32).abs() <= 1);
}

#[test]
fn test_scale_tall_image() {
    // Image taller than MAX_INLINE_DIMENSION should be scaled
    let png_bytes = create_test_png(600, 3000);
    let result = scale_image_for_inline(&png_bytes).unwrap();

    let decoded = image::load_from_memory(&result).unwrap();

    // Height should be scaled to MAX_INLINE_DIMENSION
    assert!(
        decoded.height() <= MAX_INLINE_DIMENSION,
        "Height {} should be <= {}",
        decoded.height(),
        MAX_INLINE_DIMENSION
    );
}

#[test]
fn test_scale_large_megapixel_image() {
    // Image that exceeds megapixel limit but fits in dimensions
    // 1568x1568 = 2.46 megapixels, which exceeds 1.15MP limit
    let png_bytes = create_test_png(1568, 1568);
    let result = scale_image_for_inline(&png_bytes).unwrap();

    let decoded = image::load_from_memory(&result).unwrap();

    // Should be scaled to fit within megapixel limit
    let megapixels = (decoded.width() as f64 * decoded.height() as f64) / 1_000_000.0;
    assert!(
        megapixels <= MAX_INLINE_MEGAPIXELS + 0.01, // Small tolerance for rounding
        "Megapixels {} should be <= {}",
        megapixels,
        MAX_INLINE_MEGAPIXELS
    );
}

#[test]
fn test_scale_returns_jpeg() {
    // Result should always be JPEG format
    let png_bytes = create_test_png(100, 100);
    let result = scale_image_for_inline(&png_bytes).unwrap();

    // JPEG magic bytes: FF D8 FF
    assert!(result.len() >= 3, "Result too short");
    assert_eq!(result[0], 0xFF, "First byte should be FF");
    assert_eq!(result[1], 0xD8, "Second byte should be D8");
    assert_eq!(result[2], 0xFF, "Third byte should be FF");
}

#[test]
fn test_scale_invalid_image_data() {
    let invalid_bytes = b"not an image";
    let result = scale_image_for_inline(invalid_bytes);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to decode image"));
}

#[test]
fn test_scale_preserves_aspect_ratio() {
    // A 2000x1000 image should maintain 2:1 aspect ratio
    let png_bytes = create_test_png(2000, 1000);
    let result = scale_image_for_inline(&png_bytes).unwrap();

    let decoded = image::load_from_memory(&result).unwrap();

    let original_ratio = 2000.0 / 1000.0;
    let scaled_ratio = decoded.width() as f64 / decoded.height() as f64;

    // Allow small tolerance for rounding
    assert!(
        (original_ratio - scaled_ratio).abs() < 0.05,
        "Aspect ratio not preserved: original {}, scaled {}",
        original_ratio,
        scaled_ratio
    );
}
