# accessibility-snapshots Specification

## Purpose
TBD - created by archiving change add-accessibility-snapshots. Update Purpose after archive.
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
The system SHALL assign stable, unique references to interactive elements using a hybrid identification strategy.

#### Scenario: Reference format
- **WHEN** refs are generated
- **THEN** the format is `e{hash}` where hash is a stable 4-6 character identifier
- **AND** in multi-context mode, refs are prefixed with context name (e.g., `clean:e1a2b`)

#### Scenario: Stable identifier from explicit IDs
- **WHEN** an element has a unique `id` attribute
- **THEN** the hash is derived primarily from the id
- **AND** the ref remains stable across page refreshes

#### Scenario: Stable identifier from test attributes
- **WHEN** an element has `data-testid`, `data-test`, or `name` attribute
- **THEN** the hash incorporates that attribute
- **AND** these attributes take precedence over positional identification

#### Scenario: Fallback to structural identification
- **WHEN** an element has no unique identifier attributes
- **THEN** the hash is derived from: role + accessible name + DOM path
- **AND** the ref may change if page structure changes significantly

#### Scenario: Reference validation on use
- **WHEN** a tool is called with ref `e{hash}`
- **THEN** the system looks up the element by hash in the current snapshot
- **AND** validates the element still exists
- **AND** verifies the role matches expectations

### Requirement: Stale Reference Detection
The system SHALL detect stale references and provide actionable recovery guidance.

#### Scenario: Reference from current snapshot
- **WHEN** a tool is called with a ref from the most recent snapshot
- **THEN** the action proceeds normally

#### Scenario: Reference exists but element changed
- **WHEN** a tool is called with a ref from a previous snapshot
- **AND** an element with matching hash exists but role or name changed significantly
- **THEN** the system returns an error with details:
  - "Element changed since snapshot. Was: button 'Submit', Now: button 'Loading...'"
  - "Take a new snapshot to get current element state."

#### Scenario: Reference no longer exists
- **WHEN** a tool is called with a ref that no longer matches any element
- **THEN** the system returns an error with recovery suggestions:
  - "Element 'Submit button' (ref: e1a2b) no longer exists."
  - "Similar elements on page: [lists up to 3 close matches with their refs]"
  - "Take a new snapshot to see current page state."

#### Scenario: Graceful handling of minor changes
- **WHEN** a tool is called with a ref from a previous snapshot
- **AND** the element exists with only minor changes (e.g., text count updated)
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
    - button "Sign In" [ref=s1e1]
    - textbox "Email" [ref=s1e2]
  ```

#### Scenario: Truncate long text content
- **WHEN** an element has text content exceeding 100 characters
- **THEN** the text is truncated with ellipsis
- **AND** the full text is available via element inspection

### Requirement: Element Lookup by Reference
The system SHALL resolve element references to DOM elements for tool actions.

#### Scenario: Resolve valid reference
- **WHEN** a tool is called with `ref=s1e42`
- **AND** the reference exists in the current snapshot
- **THEN** the system returns the corresponding DOM element

#### Scenario: Handle invalid reference format
- **WHEN** a tool is called with a malformed `ref` value
- **THEN** the system returns an error with the expected format

#### Scenario: Handle non-existent reference
- **WHEN** a tool is called with a `ref` that doesn't exist
- **THEN** the system returns an error indicating the element was not found
- **AND** suggests taking a new snapshot

