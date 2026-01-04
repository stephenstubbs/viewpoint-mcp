//! Integration tests for optional capability tools (vision, PDF)
//!
//! Run with:
//! ```sh
//! cargo test --features integration -p viewpoint-mcp --test optional
//! ```
#![cfg(feature = "integration")]

mod optional {
    pub mod pdf_tests;
    pub mod vision_tests;

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
}
