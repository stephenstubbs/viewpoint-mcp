//! Integration tests for interaction tools (click, type, hover, etc.)
//!
//! NOTE: Many ref-based interaction tools use placeholder implementations
//! that rely on [data-ref] CSS selectors which don't exist in actual pages.
//! These tests verify the tool infrastructure works, but actual element
//! interaction requires proper ref-to-element resolution implementation.
//!
//! Run with:
//! ```sh
//! cargo test --features integration -p viewpoint-mcp --test interaction
//! ```
#![cfg(feature = "integration")]

mod interaction {
    pub mod click_tests;
    pub mod drag_tests;
    pub mod form_tests;
    pub mod iframe_tests;
    pub mod key_tests;
    pub mod scroll_tests;
    pub mod type_tests;

    use viewpoint_mcp::browser::{BrowserConfig, BrowserState};

    /// Helper to create a headless browser state
    pub async fn create_browser() -> BrowserState {
        let config = BrowserConfig {
            headless: true,
            ..Default::default()
        };
        let mut state = BrowserState::new(config);
        state
            .initialize()
            .await
            .expect("Failed to initialize browser");
        state
    }

    /// Helper to extract first ref from snapshot
    pub fn extract_first_ref(snapshot: &str) -> Option<String> {
        let re = regex::Regex::new(r"\[ref=(c\d+p\d+f\d+e\d+)\]").unwrap();
        re.captures(snapshot)
            .map(|c| c.get(1).unwrap().as_str().to_string())
    }
}
