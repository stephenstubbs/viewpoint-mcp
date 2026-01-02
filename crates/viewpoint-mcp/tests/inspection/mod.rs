//! Integration tests for inspection tools (snapshot, screenshot, console, network)
//!
//! Run with:
//! ```sh
//! cargo test --features integration -p viewpoint-mcp --test inspection
//! ```
#![cfg(feature = "integration")]

mod console_network_tests;
mod screenshot_tests;
mod snapshot_tests;

use serde_json::json;
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
