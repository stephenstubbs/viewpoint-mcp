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
