//! Viewpoint MCP CLI
//!
//! Command-line interface for the Viewpoint MCP server.

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;
use viewpoint_mcp::browser::{BrowserConfig, BrowserType, ViewportSize};
use viewpoint_mcp::transport::{SseConfig, SseTransport, StdioTransport};
use viewpoint_mcp::{ImageResponseMode, McpServer, ServerConfig};

/// Viewpoint MCP Server - Browser automation for LLMs
#[derive(Parser, Debug)]
#[command(name = "viewpoint-mcp")]
#[command(version)]
#[command(about = "MCP server for browser automation via Viewpoint")]
struct Args {
    /// Run browser in headless mode
    #[arg(long, default_value_t = false)]
    headless: bool,

    /// Browser type (chromium or chrome)
    #[arg(long, default_value = "chromium")]
    browser: String,

    /// Viewport size (`WxH` format, e.g., "1280x720")
    #[arg(long, value_name = "WxH")]
    viewport_size: Option<String>,

    /// Connect to existing browser via CDP endpoint
    #[arg(long, value_name = "URL")]
    cdp_endpoint: Option<String>,

    /// User data directory for browser profile persistence
    #[arg(long, value_name = "PATH")]
    user_data_dir: Option<PathBuf>,

    /// Port for SSE transport (enables SSE mode instead of stdio)
    #[arg(long, value_name = "PORT")]
    port: Option<u16>,

    /// API key for SSE authentication (auto-generated if not provided)
    #[arg(long, value_name = "KEY")]
    api_key: Option<String>,

    /// Enable optional capabilities (comma-separated: vision,pdf)
    #[arg(long, value_name = "CAPS")]
    caps: Option<String>,

    /// Directory for saving screenshots (default: .viewpoint-mcp-screenshots/)
    #[arg(long, value_name = "PATH")]
    screenshot_dir: Option<PathBuf>,

    /// How screenshot images are included in responses: file (default), inline, or omit
    ///
    /// - file: Save to directory, return relative path in text
    /// - inline: Save to directory AND return base64 image in response
    /// - omit: Save to directory, return confirmation only
    #[arg(long, value_name = "MODE", default_value = "file")]
    image_responses: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .init();

    let args = Args::parse();

    // Parse viewport size
    let viewport = args.viewport_size.map(|s| {
        ViewportSize::parse(&s).unwrap_or_else(|e| {
            eprintln!("Warning: {e}, using default viewport");
            ViewportSize::new(1280, 720)
        })
    });

    // Parse browser type
    let browser_type = match args.browser.to_lowercase().as_str() {
        "chrome" => BrowserType::Chrome,
        _ => BrowserType::Chromium,
    };

    // Parse capabilities
    let capabilities: Vec<String> = args
        .caps
        .map(|s| s.split(',').map(str::trim).map(String::from).collect())
        .unwrap_or_default();

    // Parse image response mode
    let image_responses: ImageResponseMode = args.image_responses.parse().unwrap_or_else(|e| {
        eprintln!("Warning: {e}, using default 'file' mode");
        ImageResponseMode::File
    });

    // Build browser config
    let browser_config = BrowserConfig {
        headless: args.headless,
        browser_type,
        viewport,
        cdp_endpoint: args.cdp_endpoint,
        user_data_dir: args.user_data_dir,
        capabilities: capabilities.clone(),
    };

    // Build server config
    let server_config = ServerConfig {
        browser: browser_config,
        capabilities,
        screenshot_dir: args
            .screenshot_dir
            .unwrap_or_else(|| PathBuf::from(".viewpoint-mcp-screenshots")),
        image_responses,
        ..Default::default()
    };

    let server = McpServer::new(server_config);

    // Choose transport based on --port flag
    if let Some(port) = args.port {
        // SSE transport
        let sse_config = match args.api_key {
            Some(key) => SseConfig::with_api_key(port, key),
            None => {
                let config = SseConfig::new(port);
                eprintln!("Generated API key: {}", config.api_key);
                config
            }
        };

        let transport = SseTransport::new(server, sse_config);
        transport.run().await?;
    } else {
        // Stdio transport (default)
        // Note: --api-key is ignored in stdio mode
        if args.api_key.is_some() {
            tracing::warn!("--api-key is ignored in stdio mode");
        }

        let transport = StdioTransport::new(server);
        transport.run().await?;
    }

    Ok(())
}
