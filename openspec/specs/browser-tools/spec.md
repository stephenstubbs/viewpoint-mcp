# browser-tools Specification

## Purpose
TBD - created by archiving change add-core-browser-tools. Update Purpose after archive.
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

#### Scenario: Take screenshot
- **WHEN** `browser_take_screenshot` is called
- **THEN** a screenshot of the viewport is captured
- **AND** returned as base64-encoded image data

#### Scenario: Screenshot element
- **WHEN** `browser_take_screenshot` is called with `ref: "e1a2b3"`
- **THEN** the system calls `page.locator_from_ref("e1a2b3")`
- **AND** only the resolved element is captured

#### Scenario: Full page screenshot
- **WHEN** `browser_take_screenshot` is called with `fullPage: true`
- **THEN** the entire scrollable page is captured

#### Scenario: Get console messages
- **WHEN** `browser_console_messages` is called
- **THEN** console messages are returned filtered by level

#### Scenario: Get network requests
- **WHEN** `browser_network_requests` is called
- **THEN** network requests since page load are returned

### Requirement: JavaScript Evaluation
The system SHALL allow executing JavaScript in the page context.

#### Scenario: Evaluate expression
- **WHEN** `browser_evaluate` is called with `function: "() => document.title"`
- **THEN** the expression is evaluated and result returned

#### Scenario: Evaluate on element
- **WHEN** `browser_evaluate` is called with `ref: "e1a2b3"` and `function: "(el) => el.textContent"`
- **THEN** the system calls `page.locator_from_ref("e1a2b3")`
- **AND** the function is evaluated with the resolved element

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
- **THEN** the context's cookies and localStorage are saved to the file
- **AND** the file can be used with `storageState` in future context creation

