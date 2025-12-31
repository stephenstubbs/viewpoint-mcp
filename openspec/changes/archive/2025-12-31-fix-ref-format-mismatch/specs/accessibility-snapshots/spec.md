## MODIFIED Requirements

### Requirement: Element Reference System
The system SHALL use viewpoint-core's native element references for stable, unique identification of interactive elements.

#### Scenario: Reference format
- **WHEN** refs are generated
- **THEN** the format is `e{backendNodeId}` where backendNodeId is the CDP backend node identifier provided by viewpoint-core
- **AND** in multi-context mode, refs MAY be prefixed with context name (e.g., `clean:e12345`)

#### Scenario: Reference from viewpoint-core
- **WHEN** an accessibility snapshot is captured
- **THEN** the system uses the `node_ref` field from viewpoint-core's `AriaSnapshot`
- **AND** refs are assigned only to elements where viewpoint-core provides a `node_ref`

#### Scenario: Reference validation on use
- **WHEN** a tool is called with ref `e{backendNodeId}`
- **THEN** the system passes the ref directly to `page.locator_from_ref(ref)`
- **AND** viewpoint-core resolves the ref to the DOM element
- **AND** the `Locator` provides auto-waiting capabilities for reliable interactions

### Requirement: Element Lookup by Reference
The system SHALL resolve element references to DOM elements for tool actions using viewpoint's native ref resolution API.

#### Scenario: Resolve valid reference to locator
- **WHEN** a tool is called with `ref=e12345`
- **AND** the reference exists in the current page
- **THEN** the system uses `page.locator_from_ref(ref)` to get a `Locator`
- **AND** the `Locator` provides auto-waiting capabilities for reliable interactions

#### Scenario: Handle invalid reference format
- **WHEN** a tool is called with a malformed `ref` value
- **THEN** the system returns an error with the expected format: `e{backendNodeId}`

#### Scenario: Handle stale reference
- **WHEN** a tool is called with a `ref` for an element no longer in the DOM
- **THEN** the system returns an error indicating the element no longer exists
- **AND** suggests taking a new snapshot

## REMOVED Requirements

### Requirement: Hash-based Reference Generation

**Reason:** The custom hash-based ref generation (using element attributes, role, name, DOM path) is incompatible with viewpoint-core's `locator_from_ref()` API which expects refs in the format `e{backendNodeId}`.

**Migration:** Use viewpoint-core's native `node_ref` field from `AriaSnapshot` which provides refs in the correct format. The following scenarios are superseded by using viewpoint-core's native refs:

- ~~Stable identifier from explicit IDs~~
- ~~Stable identifier from test attributes~~
- ~~Fallback to structural identification~~
