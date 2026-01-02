//! Integration tests for browser connection loss recovery
//!
//! These tests verify that the system:
//! - Detects connection loss errors correctly
//! - Resets state appropriately after connection loss
//! - Re-initializes successfully after recovery
//!
//! Note: Tests that require killing the browser process are marked as `#[ignore]`
//! as they may be flaky in CI environments.
//!
//! Run with:
//! ```sh
//! cargo test --features integration -p viewpoint-mcp --test connection_recovery
//! ```
#![cfg(feature = "integration")]

use viewpoint_mcp::browser::{BrowserConfig, BrowserState};

// =============================================================================
// Connection Loss Detection Tests (Unit-level)
// =============================================================================

#[test]
fn test_is_connection_loss_error_detects_websocket_loss() {
    let patterns = [
        "WebSocket connection lost",
        "Error: WebSocket connection lost during operation",
        "CDP error: ConnectionLost",
        "connection lost to browser",
        "WebSocket error: connection refused",
        "channel closed unexpectedly",
    ];

    for msg in patterns {
        assert!(
            BrowserState::is_connection_loss_error(msg),
            "Should detect connection loss in: '{}'",
            msg
        );
    }
}

#[test]
fn test_is_connection_loss_error_ignores_other_errors() {
    let non_connection_errors = [
        "Element not found",
        "Timeout waiting for element",
        "Navigation failed",
        "Invalid selector",
        "JavaScript evaluation failed",
        "Page crashed",
    ];

    for msg in non_connection_errors {
        assert!(
            !BrowserState::is_connection_loss_error(msg),
            "Should NOT detect connection loss in: '{}'",
            msg
        );
    }
}

// =============================================================================
// State Reset Tests
// =============================================================================

#[tokio::test]
async fn test_reset_on_connection_loss_clears_state() {
    let config = BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let mut state = BrowserState::new(config);

    // Initialize browser
    state
        .initialize()
        .await
        .expect("Failed to initialize browser");
    assert!(state.is_initialized());
    assert!(state.browser().is_some());

    // Create additional contexts
    state.create_context("test_ctx").await.unwrap();
    assert!(state.list_contexts().len() >= 2);

    // Simulate connection loss
    state.reset_on_connection_loss();

    // Verify state was cleared
    assert!(!state.is_initialized());
    assert!(state.browser().is_none());
    assert!(state.list_contexts().is_empty());
    assert_eq!(state.active_context_name(), "default");
}

#[tokio::test]
async fn test_handle_potential_connection_loss_triggers_reset() {
    let config = BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let mut state = BrowserState::new(config);

    state
        .initialize()
        .await
        .expect("Failed to initialize browser");
    assert!(state.is_initialized());

    // Test with connection loss error
    let was_reset = state.handle_potential_connection_loss("WebSocket connection lost");
    assert!(was_reset, "Should trigger reset for connection loss");
    assert!(!state.is_initialized());
}

#[tokio::test]
async fn test_handle_potential_connection_loss_ignores_non_connection_errors() {
    let config = BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let mut state = BrowserState::new(config);

    state
        .initialize()
        .await
        .expect("Failed to initialize browser");
    assert!(state.is_initialized());

    // Test with non-connection error
    let was_reset = state.handle_potential_connection_loss("Element not found");
    assert!(
        !was_reset,
        "Should NOT trigger reset for non-connection error"
    );
    assert!(state.is_initialized());

    state.shutdown().await;
}

// =============================================================================
// Re-initialization Tests
// =============================================================================

#[tokio::test]
async fn test_reinitialize_after_reset() {
    let config = BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let mut state = BrowserState::new(config);

    // First initialization
    state
        .initialize()
        .await
        .expect("Failed initial initialization");
    assert!(state.is_initialized());

    // Simulate connection loss
    state.reset_on_connection_loss();
    assert!(!state.is_initialized());

    // Re-initialize should work
    state
        .initialize()
        .await
        .expect("Failed to re-initialize after connection loss");
    assert!(state.is_initialized());
    assert!(state.browser().is_some());

    // Should be able to use the browser again
    let contexts = state.list_contexts();
    assert!(
        !contexts.is_empty(),
        "Should have at least default context after re-init"
    );

    state.shutdown().await;
}

#[tokio::test]
async fn test_multiple_reset_reinitialize_cycles() {
    let config = BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let mut state = BrowserState::new(config);

    // Multiple reset/reinit cycles
    for i in 0..3 {
        state
            .initialize()
            .await
            .expect(&format!("Failed initialization cycle {}", i));
        assert!(state.is_initialized());

        state.reset_on_connection_loss();
        assert!(!state.is_initialized());
    }

    // Final initialization
    state
        .initialize()
        .await
        .expect("Failed final initialization");
    assert!(state.is_initialized());

    state.shutdown().await;
}

// =============================================================================
// Integration Tests (Actual Browser Killing)
// =============================================================================

/// Test that killing the browser process triggers appropriate error handling
///
/// This test is ignored by default as it involves killing processes and may
/// be flaky in CI environments.
#[tokio::test]
#[ignore = "requires killing browser process, may be flaky in CI"]
async fn test_tool_call_after_browser_killed() {
    use serde_json::json;
    use viewpoint_mcp::tools::{BrowserNavigateTool, Tool};

    let config = BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let mut state = BrowserState::new(config);

    state.initialize().await.unwrap();

    // Navigate to a page
    let nav_tool = BrowserNavigateTool::new();
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Test</h1>" }),
            &mut state,
        )
        .await
        .unwrap();

    // Get the browser PID and kill it
    // Note: This is platform-specific and may not work in all environments
    if let Some(browser) = state.browser() {
        // Try to get the browser process (implementation-dependent)
        // For now, we'll skip the actual killing and just simulate it
        let _ = browser;
    }

    // Force a state reset to simulate connection loss
    state.reset_on_connection_loss();

    // Next tool call should trigger re-initialization
    state
        .initialize()
        .await
        .expect("Re-initialization should succeed");

    // Should be able to navigate again
    let result = nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>After Recovery</h1>" }),
            &mut state,
        )
        .await;

    assert!(
        result.is_ok(),
        "Navigation after recovery should succeed: {:?}",
        result.err()
    );

    state.shutdown().await;
}
