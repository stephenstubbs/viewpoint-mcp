# browser-tools Specification

## Purpose
This specification defines the browser automation tools exposed through the MCP protocol. These tools enable AI agents to navigate web pages, interact with elements, capture screenshots, manage browser contexts, and perform other browser automation tasks using element references from accessibility snapshots.
## Requirements
### Requirement: Navigation Tools
The system SHALL provide tools for page navigation.

#### Scenario: Navigate to URL
- **WHEN** `browser_navigate` is called with `url: "https://example.com"`
- **THEN** the page navigates to the URL
- **AND** waits for the page to load

#### Scenario: Navigate back
- **WHEN** `browser_navigate_back` is called
- **THEN** the page navigates to the previous history entry

#### Scenario: Navigate after all pages closed
- **WHEN** all pages have been closed via `browser_close`
- **AND** `browser_navigate` is called with a URL
- **THEN** a new page is automatically created in the active context
- **AND** the page navigates to the specified URL
- **AND** the tool returns success (not an error)

### Requirement: Element Interaction Tools
The system SHALL provide tools for interacting with page elements using accessibility refs, resolved via viewpoint's native `page.locator_from_ref()` API. After performing an action that may trigger navigation or network activity, the system SHALL wait for the page to stabilize before returning.

#### Scenario: Click element
- **WHEN** `browser_click` is called with `ref: "e1a2b3"` and `element: "Sign In button"`
- **THEN** the system calls `page.locator_from_ref("e1a2b3")`
- **AND** clicks the resolved DOM element
- **AND** waits for any triggered navigation or network activity to complete

#### Scenario: Click triggers navigation
- **WHEN** `browser_click` is called on a link or submit button that triggers navigation
- **THEN** the click is performed
- **AND** the system waits for the navigation to complete (up to navigation timeout)
- **AND** waits for the page to reach load state before returning

#### Scenario: Right-click element
- **WHEN** `browser_click` is called with `ref: "e1a2b3"` and `button: "right"`
- **THEN** the element is right-clicked via the locator
- **AND** context menu events are triggered

#### Scenario: Middle-click element
- **WHEN** `browser_click` is called with `ref: "e1a2b3"` and `button: "middle"`
- **THEN** the element is middle-clicked via the locator

#### Scenario: Click with modifier keys
- **WHEN** `browser_click` is called with `ref: "e1a2b3"` and `modifiers: ["Control"]`
- **THEN** the Control key is held during the click
- **AND** the click behavior reflects the modifier (e.g., Ctrl+click opens in new tab for links)

#### Scenario: Double click
- **WHEN** `browser_click` is called with `ref: "e1a2b3"` and `doubleClick: true`
- **THEN** the element is double-clicked via the locator
- **AND** waits for any triggered navigation or network activity to complete

#### Scenario: Type text
- **WHEN** `browser_type` is called with `ref: "e1a2b3"`, `element: "Email input"`, and `text: "user@example.com"`
- **THEN** the system calls `page.locator_from_ref("e1a2b3")`
- **AND** types the text into the resolved DOM element

#### Scenario: Type with submit
- **WHEN** `browser_type` is called with `submit: true`
- **THEN** Enter is pressed after typing
- **AND** the system waits for any triggered navigation to complete before returning

#### Scenario: Type with submit triggers navigation
- **WHEN** `browser_type` is called with `submit: true` on a search input (e.g., DuckDuckGo)
- **THEN** the text is typed and Enter is pressed
- **AND** the system waits for the search results page to load
- **AND** returns only after the navigation completes

#### Scenario: Fill form
- **WHEN** `browser_fill_form` is called with an array of field objects containing refs
- **THEN** the system calls `page.locator_from_ref()` for each field's ref
- **AND** each field is filled with its specified value
- **AND** waits for any triggered network activity to settle

#### Scenario: Hover element
- **WHEN** `browser_hover` is called with `ref: "e1a2b3"`
- **THEN** the system calls `page.locator_from_ref("e1a2b3")`
- **AND** hovers the mouse over the resolved DOM element

#### Scenario: Drag and drop
- **WHEN** `browser_drag` is called with `startRef` and `endRef`
- **THEN** the system calls `page.locator_from_ref()` for both refs
- **AND** a drag operation is performed from start to end element
- **AND** waits for any triggered navigation or network activity to complete

#### Scenario: Select dropdown option
- **WHEN** `browser_select_option` is called with `ref: "e1a2b3"` and `values: ["option1"]`
- **THEN** the system calls `page.locator_from_ref("e1a2b3")`
- **AND** selects the specified option in the resolved dropdown
- **AND** waits for any triggered navigation or network activity to complete

#### Scenario: Press key
- **WHEN** `browser_press_key` is called with `key: "Enter"`
- **THEN** the key is pressed on the focused element
- **AND** waits for any triggered navigation or network activity to complete

#### Scenario: Press Enter triggers form submission
- **WHEN** `browser_press_key` is called with `key: "Enter"` while focused on a form input
- **THEN** Enter is pressed
- **AND** the system waits for any form submission navigation to complete

#### Scenario: Upload files
- **WHEN** `browser_file_upload` is called with `paths: ["/path/to/file.pdf"]`
- **THEN** the files are uploaded to the active file input

#### Scenario: Upload files with hidden input
- **WHEN** `browser_file_upload` is called with file paths
- **AND** the file input element is hidden or dynamically created
- **THEN** the system SHALL locate the file input regardless of visibility state
- **AND** use viewpoint's file chooser handling to upload the files

#### Scenario: Upload files after click triggers file chooser
- **WHEN** a click action on a button triggers a file chooser dialog
- **AND** `browser_file_upload` is called with file paths
- **THEN** the system intercepts the file chooser event
- **AND** provides the specified files to the chooser

#### Scenario: Action without navigation returns promptly
- **WHEN** an action is performed that does not trigger navigation
- **THEN** the system waits briefly for any network activity to settle
- **AND** returns without unnecessary delay (no full navigation timeout)

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

### Requirement: JavaScript Evaluation
The system SHALL allow executing JavaScript in the page context.

#### Scenario: Evaluate expression
- **WHEN** `browser_evaluate` is called with `function: "() => document.title"`
- **THEN** the expression is evaluated and result returned

#### Scenario: Evaluate on element
- **WHEN** `browser_evaluate` is called with `ref: "c0p0f0e1"` and `function: "(el) => el.textContent"`
- **THEN** the system calls `page.locator_from_ref("c0p0f0e1")`
- **AND** the function is evaluated with the resolved element
- **AND** the result is properly serialized and returned

#### Scenario: Evaluate on element returns object
- **WHEN** `browser_evaluate` is called with `ref: "c0p0f0e1"` and `function: "(el) => ({ tag: el.tagName, id: el.id })"`
- **THEN** the function is evaluated with the element
- **AND** the object result is properly serialized as JSON

#### Scenario: Evaluate on element returns string
- **WHEN** `browser_evaluate` is called with `ref: "c0p0f0e1"` and `function: "(el) => el.textContent"`
- **THEN** the function is evaluated with the element
- **AND** the string result is returned directly

#### Scenario: Evaluate on element returns null
- **WHEN** `browser_evaluate` is called with `ref: "c0p0f0e1"` and `function: "(el) => el.getAttribute('nonexistent')"`
- **THEN** the function is evaluated with the element
- **AND** null is returned and properly formatted

### Requirement: Wait Conditions
The system SHALL support waiting for various conditions.

#### Scenario: Wait for text
- **WHEN** `browser_wait_for` is called with `text: "Loading complete"`
- **THEN** the tool waits until the text appears on the page

#### Scenario: Wait for text gone
- **WHEN** `browser_wait_for` is called with `textGone: "Loading..."`
- **THEN** the tool waits until the text disappears

#### Scenario: Wait for time
- **WHEN** `browser_wait_for` is called with `time: 2`
- **THEN** the tool waits for 2 seconds

### Requirement: Dialog Handling
The system SHALL handle browser dialogs (alert, confirm, prompt).

#### Scenario: Accept dialog
- **WHEN** `browser_handle_dialog` is called with `accept: true`
- **THEN** the next dialog is accepted

#### Scenario: Dismiss dialog
- **WHEN** `browser_handle_dialog` is called with `accept: false`
- **THEN** the next dialog is dismissed

#### Scenario: Prompt text
- **WHEN** `browser_handle_dialog` is called with `accept: true` and `promptText: "answer"`
- **THEN** the prompt is filled and accepted

### Requirement: Browser Management Tools
The system SHALL provide tools for browser/page lifecycle management.

#### Scenario: Close page
- **WHEN** `browser_close` is called
- **THEN** the current page is closed

#### Scenario: Resize viewport
- **WHEN** `browser_resize` is called with `width: 1920` and `height: 1080`
- **THEN** the viewport is resized

#### Scenario: List tabs
- **WHEN** `browser_tabs` is called with `action: "list"`
- **THEN** all open tabs are returned with their titles and URLs

#### Scenario: Create tab
- **WHEN** `browser_tabs` is called with `action: "new"`
- **THEN** a new tab is created and becomes active

#### Scenario: Close tab
- **WHEN** `browser_tabs` is called with `action: "close"` and `index: 2`
- **THEN** the tab at index 2 is closed

#### Scenario: Select tab
- **WHEN** `browser_tabs` is called with `action: "select"` and `index: 0`
- **THEN** the tab at index 0 becomes active

### Requirement: Browser Installation
The system SHALL provide a tool to install the browser if missing.

#### Scenario: Install browser
- **WHEN** `browser_install` is called
- **AND** Chromium is not installed
- **THEN** Chromium is downloaded and installed

#### Scenario: Browser already installed
- **WHEN** `browser_install` is called
- **AND** Chromium is already installed
- **THEN** a success message is returned indicating browser is ready

### Requirement: Vision Capabilities (Optional)
The system SHALL conditionally expose coordinate-based tools based on the `--caps=vision` flag.

#### Scenario: Vision tools hidden by default
- **WHEN** the server starts without `--caps=vision`
- **THEN** `tools/list` does NOT include `browser_mouse_click_xy`, `browser_mouse_move_xy`, `browser_mouse_drag_xy`
- **AND** calling these tools returns "Unknown tool" error (code `-32601`)

#### Scenario: Vision tools exposed when enabled
- **WHEN** the server starts with `--caps=vision`
- **THEN** `tools/list` includes `browser_mouse_click_xy`, `browser_mouse_move_xy`, `browser_mouse_drag_xy`
- **AND** these tools are fully functional

#### Scenario: Click at coordinates
- **WHEN** `browser_mouse_click_xy` is called with `x: 100` and `y: 200`
- **THEN** a click is performed at the specified viewport coordinates

#### Scenario: Move mouse
- **WHEN** `browser_mouse_move_xy` is called with `x` and `y` coordinates
- **THEN** the mouse moves to the specified viewport position

#### Scenario: Drag by coordinates
- **WHEN** `browser_mouse_drag_xy` is called with `startX`, `startY`, `endX`, `endY`
- **THEN** a drag operation is performed between the coordinates

#### Scenario: Mixed mode usage
- **WHEN** vision capabilities are enabled
- **THEN** ref-based tools (`browser_click`, etc.) remain available
- **AND** both ref-based and coordinate-based tools can be used in the same session

### Requirement: PDF Capabilities (Optional)
The system SHALL provide PDF tools when `--caps=pdf` is enabled.

#### Scenario: Save page as PDF
- **WHEN** `browser_pdf_save` is called
- **THEN** the current page is saved as a PDF file
- **AND** the file path is returned

### Requirement: Context Management Tools
The system SHALL provide tools for managing multiple isolated browser contexts.

#### Scenario: Create context
- **WHEN** `browser_context_create` is called with `name: "clean"`
- **THEN** a new isolated browser context is created
- **AND** the context becomes active
- **AND** the response confirms creation with context details

#### Scenario: Create context with proxy
- **WHEN** `browser_context_create` is called with:
  ```json
  {
    "name": "uk_proxy",
    "proxy": {
      "server": "socks5://proxy.example.com:1080",
      "username": "user",
      "password": "pass"
    }
  }
  ```
- **THEN** the context routes all traffic through the proxy
- **AND** proxy authentication is handled automatically

#### Scenario: Create context with storage state
- **WHEN** `browser_context_create` is called with `storageState: "/path/to/cookies.json"`
- **THEN** the context is initialized with saved cookies and localStorage
- **Note**: Implementation pending viewpoint-core storage state API support

#### Scenario: Switch context
- **WHEN** `browser_context_switch` is called with `name: "returning_user"`
- **THEN** that context becomes active
- **AND** subsequent tool calls operate on that context's pages

#### Scenario: List contexts
- **WHEN** `browser_context_list` is called
- **THEN** the response includes all contexts with:
  - `name`: context identifier
  - `isActive`: boolean indicating current context
  - `pageCount`: number of open pages
  - `currentUrl`: URL of active page (if any)
  - `proxy`: proxy server (if configured)

#### Scenario: Close context
- **WHEN** `browser_context_close` is called with `name: "temp"`
- **THEN** that context and all its pages are closed
- **AND** if it was active, "default" becomes active

#### Scenario: Close active context error
- **WHEN** `browser_context_close` is called with the name of the only remaining context
- **THEN** an error is returned: "Cannot close the only remaining context"

#### Scenario: Save context storage state
- **WHEN** `browser_context_save_storage` is called with `name: "logged_in"` and `path: "/path/to/save.json"`
- **THEN** cookies and localStorage are collected from the context
- **AND** the storage state is saved to the specified file path
- **AND** the response confirms successful save

#### Scenario: Save storage state handles page session errors
- **WHEN** `browser_context_save_storage` is called
- **AND** some pages have stale or invalid sessions
- **THEN** the system skips pages with invalid sessions
- **AND** collects storage from valid pages
- **AND** returns success with partial data (or appropriate error if no valid pages)

### Requirement: Scroll Into View Tool

The system SHALL provide a tool for scrolling elements into the visible viewport.

#### Scenario: Scroll element into view

- **WHEN** `browser_scroll_into_view` is called with `ref: "e1a2b3"` and `element: "Submit button"`
- **THEN** the system calls `page.locator_from_ref("e1a2b3")`
- **AND** scrolls the element into the center of the viewport
- **AND** returns success after the scroll completes

#### Scenario: Scroll element not found

- **WHEN** `browser_scroll_into_view` is called with a ref that does not exist
- **THEN** the system returns an error indicating the element was not found

#### Scenario: Scroll with stale ref

- **WHEN** `browser_scroll_into_view` is called with a ref from a previous snapshot
- **AND** the element no longer exists in the DOM
- **THEN** the system returns an error indicating the element may no longer exist

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

