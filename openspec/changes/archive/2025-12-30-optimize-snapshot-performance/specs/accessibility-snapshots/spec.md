## ADDED Requirements

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
