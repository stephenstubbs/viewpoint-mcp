//! SSE transport implementation
//!
//! HTTP server with Server-Sent Events for MCP communication.

use std::convert::Infallible;
use std::sync::Arc;

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use rand::RngCore;
use tokio::sync::Mutex;
use tokio_stream::wrappers::ReceiverStream;

use super::error::TransportError;
use crate::server::protocol::{JsonRpcRequest, JsonRpcResponse, McpServer};

/// SSE transport configuration
#[derive(Debug, Clone)]
pub struct SseConfig {
    /// Port to listen on
    pub port: u16,

    /// API key for authentication
    pub api_key: String,
}

impl SseConfig {
    /// Create a new SSE config with auto-generated API key
    #[must_use]
    pub fn new(port: u16) -> Self {
        Self {
            port,
            api_key: generate_api_key(),
        }
    }

    /// Create a new SSE config with specific API key
    #[must_use]
    pub fn with_api_key(port: u16, api_key: impl Into<String>) -> Self {
        Self {
            port,
            api_key: api_key.into(),
        }
    }
}

/// Generate a random API key
fn generate_api_key() -> String {
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    hex::encode(key)
}

/// Shared state for the SSE server
struct AppState {
    server: Arc<Mutex<McpServer>>,
    api_key: String,
}

/// SSE transport for MCP communication
pub struct SseTransport {
    config: SseConfig,
    server: Arc<Mutex<McpServer>>,
}

impl SseTransport {
    /// Create a new SSE transport
    #[must_use]
    pub fn new(server: McpServer, config: SseConfig) -> Self {
        Self {
            config,
            server: Arc::new(Mutex::new(server)),
        }
    }

    /// Get the API key
    #[must_use]
    pub fn api_key(&self) -> &str {
        &self.config.api_key
    }

    /// Run the SSE server
    ///
    /// # Errors
    ///
    /// Returns an error if the server fails to bind or start.
    pub async fn run(&self) -> super::Result<()> {
        let state = Arc::new(AppState {
            server: Arc::clone(&self.server),
            api_key: self.config.api_key.clone(),
        });

        let app = Router::new()
            .route("/mcp", get(handle_sse))
            .route("/mcp", post(handle_post))
            .with_state(state);

        let addr = format!("0.0.0.0:{}", self.config.port);
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(|e| TransportError::BindFailed(e.to_string()))?;

        tracing::info!(port = self.config.port, "SSE server listening");
        tracing::info!(api_key = %self.config.api_key, "API key for authentication");

        axum::serve(listener, app)
            .await
            .map_err(|e| TransportError::Io(std::io::Error::other(e.to_string())))?;

        Ok(())
    }
}

/// Validate the Authorization header
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

/// Handle SSE connection
async fn handle_sse(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, Response> {
    validate_auth(&headers, &state.api_key)
        .map_err(|(status, msg)| (status, msg).into_response())?;

    let (tx, rx) = tokio::sync::mpsc::channel::<Result<Event, Infallible>>(100);

    // Send initial connection event
    let _ = tx
        .send(Ok(Event::default().event("connected").data("ok")))
        .await;

    Ok(Sse::new(ReceiverStream::new(rx)).keep_alive(KeepAlive::default()))
}

/// Handle POST requests (JSON-RPC over HTTP)
async fn handle_post(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<JsonRpcRequest>,
) -> Result<Json<JsonRpcResponse>, Response> {
    validate_auth(&headers, &state.api_key)
        .map_err(|(status, msg)| (status, msg).into_response())?;

    let request_id = request.id.clone().unwrap_or(serde_json::Value::Null);
    let mut server = state.server.lock().await;

    let response = match server.handle_request(&request).await {
        Ok(result) => JsonRpcResponse::success(request_id, result),
        Err(e) => JsonRpcResponse::from_error(request_id, &e),
    };

    Ok(Json(response))
}
