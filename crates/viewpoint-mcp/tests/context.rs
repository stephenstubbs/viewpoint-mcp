//! Integration tests for context management tools
//!
//! Run with:
//! ```sh
//! cargo test --features integration -p viewpoint-mcp --test context
//! ```
#![cfg(feature = "integration")]

mod context {
    pub mod close_tests;
    pub mod create_tests;
    pub mod integration_tests;
    pub mod list_tests;
    pub mod storage_tests;
    pub mod switch_tests;

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
