//! Tests for SSE transport and authentication

use axum::http::{HeaderMap, HeaderValue, StatusCode};

use crate::transport::SseConfig;

#[test]
fn test_sse_config_auto_generated_key() {
    let config = SseConfig::new(8080);

    assert_eq!(config.port, 8080);
    assert!(!config.api_key.is_empty());
    assert_eq!(config.api_key.len(), 64); // 32 bytes hex-encoded = 64 chars
}

#[test]
fn test_sse_config_custom_key() {
    let config = SseConfig::with_api_key(9000, "my-secret-key");

    assert_eq!(config.port, 9000);
    assert_eq!(config.api_key, "my-secret-key");
}

#[test]
fn test_sse_config_unique_keys() {
    let config1 = SseConfig::new(8080);
    let config2 = SseConfig::new(8080);

    // Each auto-generated key should be unique
    assert_ne!(config1.api_key, config2.api_key);
}

// Helper function for validating auth (duplicated from sse.rs for testing)
fn validate_auth(
    headers: &HeaderMap,
    expected_key: &str,
) -> Result<(), (StatusCode, &'static str)> {
    let auth_header = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Authentication required"))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid authorization format"))?;

    if token != expected_key {
        return Err((StatusCode::FORBIDDEN, "Invalid API key"));
    }

    Ok(())
}

#[test]
fn test_auth_valid_key() {
    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        HeaderValue::from_static("Bearer valid-key"),
    );

    let result = validate_auth(&headers, "valid-key");
    assert!(result.is_ok());
}

#[test]
fn test_auth_missing_header() {
    let headers = HeaderMap::new();

    let result = validate_auth(&headers, "any-key");
    assert!(result.is_err());

    let (status, _) = result.unwrap_err();
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[test]
fn test_auth_invalid_format() {
    let mut headers = HeaderMap::new();
    headers.insert("authorization", HeaderValue::from_static("Basic invalid"));

    let result = validate_auth(&headers, "any-key");
    assert!(result.is_err());

    let (status, _) = result.unwrap_err();
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[test]
fn test_auth_invalid_key() {
    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        HeaderValue::from_static("Bearer wrong-key"),
    );

    let result = validate_auth(&headers, "correct-key");
    assert!(result.is_err());

    let (status, _) = result.unwrap_err();
    assert_eq!(status, StatusCode::FORBIDDEN);
}
