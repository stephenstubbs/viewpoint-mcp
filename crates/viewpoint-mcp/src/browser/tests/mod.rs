//! Unit tests for browser state management
//!
//! Browser integration tests that require Chromium are in `tests/browser_integration.rs`.
//! Run them with: `cargo test --features integration -p viewpoint-mcp --test browser_integration`

use crate::browser::{BrowserConfig, BrowserState, ProxyConfig, ViewportSize};

#[test]
fn test_viewport_parse_valid() {
    let viewport = ViewportSize::parse("1920x1080").unwrap();
    assert_eq!(viewport.width, 1920);
    assert_eq!(viewport.height, 1080);
}

#[test]
fn test_viewport_parse_small() {
    let viewport = ViewportSize::parse("800x600").unwrap();
    assert_eq!(viewport.width, 800);
    assert_eq!(viewport.height, 600);
}

#[test]
fn test_viewport_parse_invalid_format() {
    let result = ViewportSize::parse("1920-1080");
    assert!(result.is_err());
}

#[test]
fn test_viewport_parse_invalid_width() {
    let result = ViewportSize::parse("abcx1080");
    assert!(result.is_err());
}

#[test]
fn test_viewport_parse_invalid_height() {
    let result = ViewportSize::parse("1920xabc");
    assert!(result.is_err());
}

#[test]
fn test_proxy_config_simple() {
    let proxy = ProxyConfig::new("socks5://proxy:1080");
    assert_eq!(proxy.server, "socks5://proxy:1080");
    assert!(proxy.username.is_none());
    assert!(proxy.password.is_none());
    assert!(proxy.bypass.is_none());
}

#[test]
fn test_proxy_config_with_auth() {
    let proxy = ProxyConfig::new("http://proxy:8080")
        .with_auth("user", "pass")
        .with_bypass("localhost,127.0.0.1");

    assert_eq!(proxy.server, "http://proxy:8080");
    assert_eq!(proxy.username, Some("user".to_string()));
    assert_eq!(proxy.password, Some("pass".to_string()));
    assert_eq!(proxy.bypass, Some("localhost,127.0.0.1".to_string()));
}

// Unit test that doesn't require browser launch
#[tokio::test]
async fn test_browser_state_new() {
    let config = BrowserConfig::default();
    let state = BrowserState::new(config);

    assert!(!state.is_initialized());
    assert_eq!(state.active_context_name(), "default");
}

// Connection loss recovery tests

#[test]
fn test_is_connection_loss_error_websocket() {
    assert!(BrowserState::is_connection_loss_error(
        "WebSocket connection lost"
    ));
    assert!(BrowserState::is_connection_loss_error(
        "Error: WebSocket connection lost while waiting for response"
    ));
}

#[test]
fn test_is_connection_loss_error_variants() {
    // Various connection loss patterns
    assert!(BrowserState::is_connection_loss_error("ConnectionLost"));
    assert!(BrowserState::is_connection_loss_error("connection lost"));
    assert!(BrowserState::is_connection_loss_error("connection closed"));
    assert!(BrowserState::is_connection_loss_error("WebSocket error"));
    assert!(BrowserState::is_connection_loss_error("WebSocket closed"));
    assert!(BrowserState::is_connection_loss_error("channel closed"));
    assert!(BrowserState::is_connection_loss_error(
        "browser disconnected"
    ));
    assert!(BrowserState::is_connection_loss_error(
        "CDP connection failed"
    ));
}

#[test]
fn test_is_connection_loss_error_non_connection() {
    // These should NOT trigger connection loss recovery
    assert!(!BrowserState::is_connection_loss_error("Element not found"));
    assert!(!BrowserState::is_connection_loss_error(
        "Timeout waiting for selector"
    ));
    assert!(!BrowserState::is_connection_loss_error(
        "Navigation failed: 404"
    ));
    assert!(!BrowserState::is_connection_loss_error(
        "JavaScript error: undefined"
    ));
    assert!(!BrowserState::is_connection_loss_error("Invalid selector"));
}

#[test]
fn test_reset_on_connection_loss() {
    let config = BrowserConfig::default();
    let mut state = BrowserState::new(config);

    // Simulate initialized state (without actually launching browser)
    // We can't test with a real browser, but we can verify state transitions

    // State should start uninitialized
    assert!(!state.is_initialized());
    assert_eq!(state.active_context_name(), "default");

    // After reset, state should be ready for re-initialization
    state.reset_on_connection_loss();

    assert!(!state.is_initialized());
    assert!(state.browser().is_none());
    assert_eq!(state.active_context_name(), "default");
}

#[test]
fn test_handle_potential_connection_loss_triggers_reset() {
    let config = BrowserConfig::default();
    let mut state = BrowserState::new(config);

    // Connection loss error should trigger reset and return true
    let triggered = state.handle_potential_connection_loss("WebSocket connection lost");
    assert!(triggered);
    assert!(!state.is_initialized());
}

#[test]
fn test_handle_potential_connection_loss_ignores_other_errors() {
    let config = BrowserConfig::default();
    let mut state = BrowserState::new(config);

    // Non-connection errors should not trigger reset and return false
    let triggered = state.handle_potential_connection_loss("Element not found: #button");
    assert!(!triggered);
}
