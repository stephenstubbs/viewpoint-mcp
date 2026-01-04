//! Stdio transport implementation
//!
//! Reads JSON-RPC requests from stdin and writes responses to stdout.
//! This is the default transport for CLI-based MCP clients.

use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;

use crate::server::protocol::{JsonRpcRequest, JsonRpcResponse, McpServer};

/// Stdio transport for MCP communication.
///
/// Implements the MCP transport protocol over stdin/stdout, suitable
/// for integration with CLI-based MCP clients like Claude Code.
///
/// # Examples
///
/// ```rust,ignore
/// use viewpoint_mcp::{McpServer, ServerConfig};
/// use viewpoint_mcp::transport::StdioTransport;
///
/// #[tokio::main]
/// async fn main() -> Result<(), viewpoint_mcp::transport::TransportError> {
///     let server = McpServer::new(ServerConfig::default());
///     let transport = StdioTransport::new(server);
///
///     // Run until stdin closes
///     transport.run().await?;
///     Ok(())
/// }
/// ```
pub struct StdioTransport {
    server: Arc<Mutex<McpServer>>,
}

impl StdioTransport {
    /// Create a new stdio transport
    #[must_use]
    pub fn new(server: McpServer) -> Self {
        Self {
            server: Arc::new(Mutex::new(server)),
        }
    }

    /// Run the transport, processing requests until stdin closes
    ///
    /// # Errors
    ///
    /// Returns an error if I/O operations fail.
    pub async fn run(&self) -> super::Result<()> {
        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut reader = BufReader::new(stdin);

        let mut line = String::new();

        loop {
            line.clear();
            let bytes_read = reader.read_line(&mut line).await?;

            if bytes_read == 0 {
                // EOF - stdin closed
                tracing::info!("Stdin closed, shutting down");
                break;
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Parse the request
            let request: JsonRpcRequest = match serde_json::from_str(trimmed) {
                Ok(req) => req,
                Err(e) => {
                    let response = JsonRpcResponse::error(
                        serde_json::Value::Null,
                        -32700,
                        format!("Parse error: {e}"),
                    );
                    let response_json = serde_json::to_string(&response)?;
                    stdout.write_all(response_json.as_bytes()).await?;
                    stdout.write_all(b"\n").await?;
                    stdout.flush().await?;
                    continue;
                }
            };

            // Handle the request
            let request_id = request.id.clone().unwrap_or(serde_json::Value::Null);
            let mut server = self.server.lock().await;

            let response = match server.handle_request(&request).await {
                Ok(result) => JsonRpcResponse::success(request_id, result),
                Err(e) => JsonRpcResponse::from_error(request_id, &e),
            };

            // Skip response for notifications (no id)
            if request.id.is_some() {
                let response_json = serde_json::to_string(&response)?;
                stdout.write_all(response_json.as_bytes()).await?;
                stdout.write_all(b"\n").await?;
                stdout.flush().await?;
            }
        }

        Ok(())
    }
}
