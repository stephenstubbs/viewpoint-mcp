## 1. CLI and Configuration

- [x] 1.1 Add `--screenshot-dir` flag to CLI with default `.viewpoint-mcp-screenshots/`
- [x] 1.2 Add `--image-responses` flag with values `omit`, `file` (default), `inline`
- [x] 1.3 Add `screenshot_dir: PathBuf` to `ServerConfig`
- [x] 1.4 Add `image_responses: ImageResponseMode` enum to `ServerConfig`
- [x] 1.5 Pass screenshot config to `BrowserState` or tool context

## 2. Core Types

- [x] 2.1 Add `ContentItem` enum to `server/protocol.rs` with `Text` and `Image` variants
- [x] 2.2 Add `ToolOutput` struct with `content: Vec<ContentItem>` field
- [x] 2.3 Add `ToolOutput::text(s: impl Into<String>)` helper for simple text responses
- [x] 2.4 Update `ToolResult` type alias to `Result<ToolOutput, ToolError>`
- [x] 2.5 Update `ToolCallResult` to use `Vec<ContentItem>` instead of `Vec<ToolResultContent>`

## 3. Tool Updates

- [x] 3.1 Update `browser_navigate` to return `ToolOutput::text(...)`
- [x] 3.2 Update `browser_navigate_back` to return `ToolOutput::text(...)`
- [x] 3.3 Update `browser_click` to return `ToolOutput::text(...)`
- [x] 3.4 Update `browser_type` to return `ToolOutput::text(...)`
- [x] 3.5 Update `browser_hover` to return `ToolOutput::text(...)`
- [x] 3.6 Update `browser_drag` to return `ToolOutput::text(...)`
- [x] 3.7 Update `browser_select_option` to return `ToolOutput::text(...)`
- [x] 3.8 Update `browser_press_key` to return `ToolOutput::text(...)`
- [x] 3.9 Update `browser_fill_form` to return `ToolOutput::text(...)`
- [x] 3.10 Update `browser_file_upload` to return `ToolOutput::text(...)`
- [x] 3.11 Update `browser_snapshot` to return `ToolOutput::text(...)`
- [x] 3.12 Update `browser_console_messages` to return `ToolOutput::text(...)`
- [x] 3.13 Update `browser_network_requests` to return `ToolOutput::text(...)`
- [x] 3.14 Update `browser_evaluate` to return `ToolOutput::text(...)`
- [x] 3.15 Update `browser_wait_for` to return `ToolOutput::text(...)`
- [x] 3.16 Update `browser_handle_dialog` to return `ToolOutput::text(...)`
- [x] 3.17 Update `browser_close` to return `ToolOutput::text(...)`
- [x] 3.18 Update `browser_resize` to return `ToolOutput::text(...)`
- [x] 3.19 Update `browser_tabs` to return `ToolOutput::text(...)`
- [x] 3.20 Update `browser_install` to return `ToolOutput::text(...)`
- [x] 3.21 Update `browser_context_*` tools to return `ToolOutput::text(...)`
- [x] 3.22 Update vision tools (`browser_mouse_*_xy`) to return `ToolOutput::text(...)`
- [x] 3.23 Update `browser_pdf_save` to return `ToolOutput::text(...)`

## 4. Screenshot Tool Updates

- [x] 4.1 Update `browser_take_screenshot` to save to screenshot directory
- [x] 4.2 Create screenshot directory if it doesn't exist
- [x] 4.3 Use timestamp-based filenames (`page-{ISO-timestamp}.{ext}`)
- [x] 4.4 Return relative file path in text response (file/inline modes)
- [x] 4.5 Return confirmation only in text response (omit mode)
- [x] 4.6 Add image scaling utility function (max 1568px, 1.15MP, JPEG quality 80)
- [x] 4.7 Add `image` crate dependency for image processing
- [x] 4.8 Accept `image_responses` mode from tool context
- [x] 4.9 Return `ImageContent` when mode is `inline` (scaled, JPEG)
- [x] 4.10 Keep full-resolution image saved to file in all modes

## 5. Protocol Handling

- [x] 5.1 Update `handle_tools_call` to serialize `ContentItem` correctly
- [x] 5.2 Pass image response mode and screenshot directory to tool execution context
- [x] 5.3 Verify MCP protocol compliance with `content` array format

## 6. Testing

- [x] 6.1 Add unit tests for `ContentItem` serialization
- [x] 6.2 Add unit tests for image scaling logic
- [x] 6.3 Update existing tool tests to use `ToolOutput`
- [x] 6.4 Add integration test for screenshot saving to directory
- [x] 6.5 Add integration test for `--image-responses=file` (default)
- [x] 6.6 Add integration test for `--image-responses=inline`
- [x] 6.7 Add integration test for `--image-responses=omit`
- [x] 6.8 Verify response format matches MCP spec

## 7. Documentation

- [x] 7.1 Update CLI help text for `--screenshot-dir` flag
- [x] 7.2 Update CLI help text for `--image-responses` flag
- [x] 7.3 Update tool descriptions to mention screenshot directory and response modes
