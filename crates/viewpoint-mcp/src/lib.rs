//! # Viewpoint MCP Server Library
//!
//! This crate provides an MCP (Model Context Protocol) server for browser automation
//! using [Viewpoint](https://crates.io/crates/viewpoint-core), a Rust-native Chromium
//! automation library.
//!
//! ## Overview
//!
//! Viewpoint MCP enables LLMs to control web browsers through the Model Context Protocol,
//! providing tools for:
//!
//! - **Navigation**: Navigate to URLs, go back, handle redirects
//! - **Interaction**: Click elements, type text, fill forms, drag and drop
//! - **Inspection**: Capture accessibility snapshots, screenshots, console logs
//! - **Context Management**: Multiple isolated browser contexts with proxy support
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use viewpoint_mcp::{McpServer, ServerConfig};
//! use viewpoint_mcp::transport::StdioTransport;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), viewpoint_mcp::transport::TransportError> {
//!     // Create server with default configuration
//!     let config = ServerConfig::default();
//!     let server = McpServer::new(config);
//!
//!     // Run with stdio transport (for CLI usage)
//!     let transport = StdioTransport::new(server);
//!     transport.run().await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## SSE Transport Example
//!
//! For HTTP-based MCP clients, use the SSE transport:
//!
//! ```rust,ignore
//! use viewpoint_mcp::{McpServer, ServerConfig};
//! use viewpoint_mcp::transport::{SseTransport, SseConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), viewpoint_mcp::transport::TransportError> {
//!     let server = McpServer::new(ServerConfig::default());
//!
//!     // Run SSE server on port 3000 with auto-generated API key
//!     let sse_config = SseConfig::new(3000);
//!     println!("API Key: {}", sse_config.api_key);
//!
//!     let transport = SseTransport::new(server, sse_config);
//!     transport.run().await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Module Structure
//!
//! - [`browser`]: Browser state and configuration management
//! - [`server`]: MCP protocol implementation and server logic
//! - [`snapshot`]: Accessibility tree capture and element references
//! - [`tools`]: MCP tool definitions for browser automation
//! - [`transport`]: Communication layers (stdio, SSE)
//!
//! ## Optional Capabilities
//!
//! Some tools require opt-in capabilities:
//!
//! - `vision`: Enables coordinate-based mouse tools for visual automation
//! - `pdf`: Enables PDF generation from pages
//!
//! Enable via [`ServerConfig::capabilities`] or the CLI `--caps` flag.

pub mod browser;
pub mod server;
pub mod snapshot;
pub mod tools;
pub mod transport;

pub use server::{ImageResponseMode, McpServer, ServerConfig};
