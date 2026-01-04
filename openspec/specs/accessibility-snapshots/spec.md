# accessibility-snapshots Specification

## Purpose
This specification defines how viewpoint-mcp captures and formats accessibility tree snapshots for LLM consumption. The accessibility snapshot system enables AI agents to understand page structure, identify interactive elements, and perform reliable element-based interactions through a consistent reference system.
## Requirements
### Requirement: Accessibility Tree Capture
The system SHALL capture the accessibility tree from the current page for LLM consumption.

#### Scenario: Capture full page accessibility tree
- **WHEN** `browser_snapshot` tool is called
- **THEN** the system retrieves the accessibility tree from the page
- **AND** returns a formatted text representation

#### Scenario: Handle empty page
- **WHEN** `browser_snapshot` is called on a blank page
- **THEN** the system returns a minimal tree with just the document node

#### Scenario: Handle frames
- **WHEN** the page contains iframes
- **THEN** the accessibility tree includes frame contents
- **AND** frame boundaries are clearly marked

#### Scenario: Handle null accessibility tree response
- **WHEN** `browser_snapshot` is called
- **AND** the browser returns a null or empty accessibility tree (e.g., minimal HTML without semantic elements)
- **THEN** the system SHALL NOT fail with a deserialization error
- **AND** the system returns a minimal document node representation
- **AND** an informational message indicates the page has no accessible content

### Requirement: Interactive Element Classification
The system SHALL classify elements into tiers to determine which receive refs.

#### Scenario: Tier 1 elements always receive refs
- **WHEN** the accessibility tree contains elements with roles: `button`, `link`, `textbox`, `checkbox`, `radio`, `combobox`, `slider`, `menuitem`, `menuitemcheckbox`, `menuitemradio`, `tab`, `switch`, `searchbox`, `spinbutton`
- **THEN** these elements always receive refs
- **AND** any element with `tabindex >= 0` receives a ref

#### Scenario: Tier 2 elements receive refs when actionable
- **WHEN** the accessibility tree contains `listitem`, `option`, `treeitem`, `row`, `cell`
- **AND** they are within interactive containers (selectable lists, comboboxes, trees, grids)
- **THEN** these elements receive refs

#### Scenario: Tier 3 elements never receive refs
- **WHEN** the accessibility tree contains structural roles: `heading`, `paragraph`, `text`, `separator`, `img`, `figure`, `main`, `navigation`, `banner`, `contentinfo`
- **AND** they are not interactive
- **THEN** these elements do NOT receive refs

#### Scenario: Compact output for complex pages
- **WHEN** a page has more than 100 interactive elements
- **THEN** only Tier 1 elements receive refs by default
- **AND** a note indicates additional interactive elements are available
- **AND** the user can request full refs with `browser_snapshot` parameter `allRefs: true`

### Requirement: Element Reference System
The system SHALL assign unique references to interactive elements using viewpoint-core's native CDP backend node identifiers.

**Implementation Note**: The spec originally described a hybrid identification strategy (id/data-testid/name prioritization) for stable refs. The current implementation uses viewpoint-core's native `node_ref` field which provides refs in the format `e{backendNodeId}`. This is a simpler approach but refs may change on page refresh. Enhanced stable identification would require viewpoint-core changes.

#### Scenario: Reference format
- **WHEN** refs are generated
- **THEN** the format is `e{backendNodeId}` where backendNodeId is the CDP backend node identifier
- **AND** in multi-context mode, refs are prefixed with context name (e.g., `clean:e12345`)

#### Scenario: Native ref from viewpoint-core
- **WHEN** viewpoint-core's `AriaSnapshot.node_ref` is present
- **THEN** the ref is used directly for element identification
- **AND** the ref can be passed to `page.locator_from_ref()` for reliable element resolution

#### Scenario: Reference validation on use
- **WHEN** a tool is called with ref `e{backendNodeId}`
- **THEN** the system looks up the element by ref in the current snapshot
- **AND** validates the element still exists
- **AND** verifies the role matches expectations

### Requirement: Stale Reference Detection
The system SHALL detect stale references and provide actionable recovery guidance.

#### Scenario: Reference from current snapshot
- **WHEN** a tool is called with a ref from the most recent snapshot
- **THEN** the action proceeds normally

#### Scenario: Reference exists but element changed
- **WHEN** a tool is called with a ref from a previous snapshot
- **AND** an element with matching ref exists but role or name changed significantly
- **THEN** the system returns an error with details:
  - "Element changed since snapshot. Was: button 'Submit', Now: button 'Loading...'"
  - "Take a new snapshot to get current element state."

#### Scenario: Reference no longer exists
- **WHEN** a tool is called with a ref that no longer matches any element
- **THEN** the system returns an error with recovery suggestions:
  - "Element 'Submit button' (ref: e12345) no longer exists."
  - "Take a new snapshot to see current page state."

#### Scenario: Graceful handling of minor changes
- **WHEN** a tool is called with a ref from a previous snapshot
- **AND** the element exists with only minor changes (e.g., text content updated)
- **THEN** the action proceeds with a note:
  - "Note: Element may have changed. Using current state."

#### Scenario: Snapshot history for comparison
- **WHEN** a new snapshot is taken
- **THEN** the previous snapshot is retained for staleness comparison
- **AND** only one previous snapshot is kept (not full history)

### Requirement: Snapshot Text Format
The system SHALL format accessibility snapshots as indented text for LLM readability.

#### Scenario: Format tree structure
- **WHEN** the accessibility tree is formatted
- **THEN** the output uses indentation to show hierarchy
- **AND** each line contains role, name, and ref (if interactive)

#### Scenario: Example output format
- **WHEN** a page with heading, button, and input is captured
- **THEN** the output resembles:
  ```
  - document:
    - heading "Welcome"
    - button "Sign In" [ref=e12345]
    - textbox "Email" [ref=e12346]
  ```

#### Scenario: Truncate long text content
- **WHEN** an element has text content exceeding 100 characters
- **THEN** the text is truncated with ellipsis ("...")
- **AND** the full text is available via element inspection

### Requirement: Element Lookup by Reference
The system SHALL resolve element references to DOM elements for tool actions using viewpoint's native ref resolution API.

#### Scenario: Resolve valid reference to locator
- **WHEN** a tool is called with `ref=e1a2b3`
- **AND** the reference exists in the current page
- **THEN** the system uses `page.locator_from_ref(ref)` to get a `Locator`
- **AND** the `Locator` provides auto-waiting capabilities for reliable interactions

#### Scenario: Handle invalid reference format
- **WHEN** a tool is called with a malformed `ref` value
- **THEN** the system returns an error with the expected format

#### Scenario: Handle stale reference
- **WHEN** a tool is called with a `ref` for an element no longer in the DOM
- **THEN** the system returns an error indicating the element no longer exists
- **AND** suggests taking a new snapshot

### Requirement: Snapshot Caching
The system SHALL cache accessibility snapshots to avoid redundant browser queries within the cache TTL window.

#### Scenario: Cache hit returns cached snapshot
- **WHEN** `browser_snapshot` is called
- **AND** a cached snapshot exists for the current page/URL
- **AND** the cache is less than 5 seconds old
- **AND** no page-modifying actions have occurred since caching
- **THEN** the cached snapshot is returned without querying the browser
- **AND** performance is significantly improved

#### Scenario: Cache miss captures fresh snapshot
- **WHEN** `browser_snapshot` is called
- **AND** no valid cached snapshot exists (expired, URL changed, or invalidated)
- **THEN** a fresh snapshot is captured from the browser
- **AND** the new snapshot is cached for subsequent requests

#### Scenario: Cache invalidated by mutations
- **WHEN** a page-modifying tool is called (click, type, navigate, etc.)
- **THEN** the snapshot cache is invalidated
- **AND** the next `browser_snapshot` call captures fresh data

#### Scenario: allRefs parameter bypasses cache
- **WHEN** `browser_snapshot` is called with `allRefs: true`
- **AND** a cached snapshot exists that was captured without `allRefs`
- **THEN** a fresh snapshot is captured with all refs
- **AND** the cache is NOT updated (to preserve default mode caching)

### Requirement: Performance Instrumentation
The system SHALL include tracing instrumentation for performance profiling.

#### Scenario: Tracing spans for snapshot phases
- **WHEN** `browser_snapshot` executes
- **THEN** the following phases are instrumented with tracing spans:
  - `browser_initialize` - browser startup (if needed)
  - `capture_snapshot` - accessibility tree retrieval
  - `format_snapshot` - tree-to-text formatting
- **AND** cache hit/miss is logged at debug level

