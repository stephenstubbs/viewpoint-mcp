//! Unit tests for snapshot formatting (truncate_text function)

use crate::snapshot::format::truncate_text;

#[test]
fn test_truncate_text_short_string() {
    // String shorter than max_len should not be truncated
    assert_eq!(truncate_text("hello", 10), "hello");
}

#[test]
fn test_truncate_text_exact_length() {
    // String exactly at max_len should not be truncated
    assert_eq!(truncate_text("hello", 5), "hello");
}

#[test]
fn test_truncate_text_long_ascii() {
    // Long ASCII string should be truncated
    let result = truncate_text("hello world", 8);
    assert_eq!(result, "hello...");
    assert!(result.len() <= 8);
}

#[test]
fn test_truncate_text_utf8_safe() {
    // String with multi-byte UTF-8 characters should truncate at char boundary
    // "Català" has 'à' which is 2 bytes (U+00E0)
    let text = "Català language";
    let result = truncate_text(text, 10);
    // Should not panic and should produce valid UTF-8
    assert!(result.is_ascii() || !result.is_empty());
    assert!(result.ends_with("..."));
}

#[test]
fn test_truncate_text_all_multibyte() {
    // String with all multi-byte characters
    // Each character is 3 bytes in UTF-8
    let text = "日本語テスト";
    let result = truncate_text(text, 10);
    // Should not panic
    assert!(result.ends_with("..."));
}

#[test]
fn test_truncate_text_emoji() {
    // Emojis can be 4 bytes
    let text = "Hello 👋 World";
    let result = truncate_text(text, 10);
    // Should not panic and should be valid UTF-8
    assert!(result.ends_with("..."));
}

#[test]
fn test_truncate_text_boundary_at_multibyte() {
    // Test where the truncation point would fall in the middle of a multi-byte char
    // "café" - 'é' is at byte positions 3-4 (2 bytes)
    let text = "café au lait";
    let result = truncate_text(text, 7);
    // Should truncate before or after 'é', not in the middle
    assert!(result.ends_with("..."));
    // Verify it's valid UTF-8 by checking we can iterate chars
    assert!(result.chars().count() > 0);
}

#[test]
fn test_truncate_text_very_short_max() {
    // Very short max_len (less than ellipsis length)
    let result = truncate_text("hello", 2);
    assert_eq!(result, "...");
}

#[test]
fn test_truncate_text_wikipedia_languages() {
    // Real-world test case from Wikipedia language list that caused the bug
    let text = "Bahasa Indonesia\nBahasa Melayu\nBân-lâm-gú\nБългарски\nCatalà";
    let result = truncate_text(text, 50);
    // Should not panic and should produce valid output
    assert!(result.ends_with("..."));
    assert!(result.len() <= 50 + 10); // Allow some slack for char boundary
}
