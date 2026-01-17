# Change: Add Image Content Responses for Screenshots

## Why

The `browser_take_screenshot` tool currently returns a text description mentioning the base64 data but doesn't include the image in the MCP response. LLMs with vision capabilities cannot see the screenshot because:
1. The image is saved to a file but the path isn't always accessible/predictable
2. The MCP response only contains `TextContent`, not `ImageContent`
3. The LLM would need to make a separate file read call (if vision-enabled file reading is available)

Two approaches to solve this:
1. **File-based (default)**: Save to a known location so LLMs can read via file tools - avoids context bloat
2. **Inline images**: Return `ImageContent` in the MCP response (Playwright-MCP approach) - for LLMs that can't read files

## What Changes

- **ADDED** Default screenshot output directory: `.viewpoint-mcp-screenshots/` in current working directory
- **ADDED** `--screenshot-dir` CLI flag to override the default location
- **ADDED** `--image-responses` CLI flag with values `omit`, `file` (default), `inline`
- **MODIFIED** `browser_take_screenshot` to save files to the screenshot directory with predictable paths
- **MODIFIED** `ToolResult` type to support multiple content types (text and image)
- **MODIFIED** `ToolResultContent` to be an enum supporting both text and image content
- **MODIFIED** `browser_take_screenshot` to return `ImageContent` when `--image-responses=inline`
- **ADDED** Image scaling to fit LLM vision limits (max 1568px, 1.15MP as per Claude's guidelines)

## Impact

- Affected specs: `browser-tools`, `mcp-server`
- Affected code:
  - `crates/viewpoint-mcp/src/tools/traits.rs` - `ToolResult` type
  - `crates/viewpoint-mcp/src/server/protocol.rs` - `ToolResultContent`, response handling
  - `crates/viewpoint-mcp/src/tools/browser_take_screenshot.rs` - return image content, save to directory
  - `crates/viewpoint-mcp-cli/src/main.rs` - `--screenshot-dir` and `--image-responses` flags
