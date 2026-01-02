//! Integration tests for context management tools
//!
//! Run with:
//! ```sh
//! cargo test --features integration -p viewpoint-mcp --test context
//! ```
#![cfg(feature = "integration")]

mod close_tests;
mod create_tests;
mod integration_tests;
mod list_tests;
mod storage_tests;
mod switch_tests;

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
