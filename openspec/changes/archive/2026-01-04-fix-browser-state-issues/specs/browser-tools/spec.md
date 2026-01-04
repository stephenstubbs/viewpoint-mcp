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
