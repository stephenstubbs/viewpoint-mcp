## MODIFIED Requirements

### Requirement: Page Inspection Tools
The system SHALL provide tools for inspecting page state.

#### Scenario: Capture accessibility snapshot
- **WHEN** `browser_snapshot` is called
- **THEN** the accessibility tree is captured and formatted
- **AND** element refs are assigned for interactive elements

#### Scenario: Take screenshot (file mode)
- **WHEN** `browser_take_screenshot` is called
- **AND** `--image-responses=file` or no flag is set
- **THEN** a screenshot of the viewport is captured
- **AND** saved to the screenshot directory (default: `.viewpoint-mcp-screenshots/`)
- **AND** returned as `TextContent` with the relative file path and description

#### Scenario: Take screenshot (inline mode)
- **WHEN** `browser_take_screenshot` is called
- **AND** `--image-responses=inline` is set
- **THEN** a screenshot of the viewport is captured
- **AND** saved to the screenshot directory
- **AND** the image is scaled to fit LLM vision limits (max 1568px dimension, max 1.15MP)
- **AND** the response includes both `TextContent` (relative path) and `ImageContent` (base64 image data)

#### Scenario: Take screenshot (omit mode)
- **WHEN** `browser_take_screenshot` is called
- **AND** `--image-responses=omit` is set
- **THEN** a screenshot of the viewport is captured
- **AND** saved to the screenshot directory
- **AND** the response only confirms the screenshot was taken (no path or image)

#### Scenario: Screenshot image scaling for inline response
- **WHEN** a screenshot exceeds 1568px in any dimension or 1.15 megapixels total
- **AND** `--image-responses=inline` is set
- **THEN** the inline image is scaled down proportionally before encoding
- **AND** JPEG format at quality 80 is used for smaller size
- **AND** the full-resolution image is still saved to file

#### Scenario: Screenshot element
- **WHEN** `browser_take_screenshot` is called with `ref: "c0p0f0e1"`
- **THEN** the system calls `page.locator_from_ref("c0p0f0e1")`
- **AND** only the resolved element is captured
- **AND** saved to the screenshot directory

#### Scenario: Full page screenshot
- **WHEN** `browser_take_screenshot` is called with `fullPage: true`
- **THEN** the entire scrollable page is captured
- **AND** saved to the screenshot directory

#### Scenario: Get console messages
- **WHEN** `browser_console_messages` is called
- **THEN** console messages captured since page load are returned
- **AND** messages are filtered by the specified level

#### Scenario: Console messages include all levels
- **WHEN** `browser_console_messages` is called with `level: "debug"`
- **THEN** messages from console.log, console.info, console.warn, console.error, and console.debug are returned

#### Scenario: Console messages filtered by level
- **WHEN** `browser_console_messages` is called with `level: "error"`
- **THEN** only console.error messages are returned

#### Scenario: Console message buffer limit
- **WHEN** more than 1000 console messages are logged
- **THEN** the oldest messages are evicted to maintain the 1000 message limit

#### Scenario: Get network requests
- **WHEN** `browser_network_requests` is called
- **THEN** network requests since page load are returned

## ADDED Requirements

### Requirement: MCP Content Types
The system SHALL support multiple content types in tool responses as defined by the MCP protocol.

#### Scenario: Text content response
- **WHEN** a tool returns text output
- **THEN** the response includes a content item with `type: "text"` and `text` field

#### Scenario: Image content response
- **WHEN** a tool returns image output
- **AND** `--image-responses=inline` is set
- **THEN** the response includes a content item with `type: "image"`, `data` (base64), and `mimeType` fields

#### Scenario: Mixed content response
- **WHEN** a tool returns both text and image output
- **THEN** the response `content` array includes both `TextContent` and `ImageContent` items
- **AND** the order is text first, then images

### Requirement: Screenshot Directory
The system SHALL save screenshots to a predictable directory location.

#### Scenario: Default screenshot directory
- **WHEN** a screenshot is taken
- **AND** no custom directory is configured
- **THEN** the screenshot is saved to `.viewpoint-mcp-screenshots/` in the current working directory

#### Scenario: Custom screenshot directory
- **WHEN** a screenshot is taken
- **AND** `--screenshot-dir` was specified at startup
- **THEN** the screenshot is saved to the specified directory

#### Scenario: Directory creation
- **WHEN** a screenshot is taken
- **AND** the screenshot directory does not exist
- **THEN** the directory is created automatically

#### Scenario: Relative path in response
- **WHEN** a screenshot is saved
- **AND** `--image-responses` is `file` or `inline`
- **THEN** the text response includes the relative file path
- **AND** the LLM can use the path with file reading tools

#### Scenario: Timestamp-based filenames
- **WHEN** a screenshot is saved
- **THEN** the filename follows the pattern `page-{ISO-timestamp}.{ext}`
- **AND** colons and dots in the timestamp are replaced with dashes
