## MODIFIED Requirements

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

#### Scenario: Action without navigation returns promptly
- **WHEN** an action is performed that does not trigger navigation
- **THEN** the system waits briefly for any network activity to settle
- **AND** returns without unnecessary delay (no full navigation timeout)

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
- **THEN** the system returns an error indicating storage state export is not yet available
- **Note**: Full implementation pending viewpoint-core storage state export API
