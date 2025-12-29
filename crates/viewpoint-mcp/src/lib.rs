//! Viewpoint MCP Server Library
//!
//! This crate provides an MCP (Model Context Protocol) server for browser automation
//! using Viewpoint, a Rust-native Chromium automation library.

pub mod browser;
pub mod server;
pub mod snapshot;
pub mod tools;
pub mod transport;

pub use server::{McpServer, ServerConfig};
