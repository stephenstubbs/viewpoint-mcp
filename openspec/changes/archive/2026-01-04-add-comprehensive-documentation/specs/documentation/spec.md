## ADDED Requirements

### Requirement: Documentation Lint Enforcement

All crates in the workspace MUST enable the `missing_docs` lint to enforce documentation on public items.

#### Scenario: Missing docs produces warning

- **WHEN** a public item (function, struct, enum, trait, type alias, constant) lacks a doc comment
- **THEN** `cargo clippy` MUST produce a warning
- **AND** the warning MUST identify the undocumented item

### Requirement: Public Item Documentation

All public items MUST have rustdoc documentation that includes:
1. A summary line describing what the item does
2. An `# Examples` section with runnable code (where feasible)
3. An `# Errors` section for fallible functions
4. An `# Panics` section if the function can panic

#### Scenario: Struct with example

- **WHEN** a public struct is defined
- **THEN** its doc comment MUST include a summary
- **AND** MUST include an `# Examples` section showing construction and basic usage
- **AND** each public field or method MUST be documented

#### Scenario: Async function with error handling

- **WHEN** a public async function returns `Result`
- **THEN** its doc comment MUST include a summary
- **AND** MUST include an `# Errors` section listing error conditions
- **AND** SHOULD include an `# Examples` section (may use `no_run` if browser required)

### Requirement: Module-Level Documentation

Each module (`mod.rs` or `lib.rs`) MUST have a module-level doc comment (`//!`) that:
1. Describes the module's purpose
2. Lists key types and their relationships
3. Provides usage guidance or links to examples

#### Scenario: Module with submodules

- **WHEN** a module contains submodules
- **THEN** the module doc comment MUST describe the overall responsibility
- **AND** SHOULD mention key re-exported types

### Requirement: Crate README Files

Each crate in the workspace MUST have a `README.md` that:
1. Describes the crate's purpose
2. Shows basic usage examples
3. Links to generated rustdoc for detailed API reference

#### Scenario: Library crate README

- **WHEN** a library crate exists (e.g., `viewpoint-mcp`)
- **THEN** its README MUST include a "Quick Start" or "Usage" section
- **AND** MUST include code examples showing common use cases
- **AND** MUST link to `docs.rs` or local rustdoc

#### Scenario: Binary crate README

- **WHEN** a binary crate exists (e.g., `viewpoint-mcp-cli`)
- **THEN** its README MUST document CLI usage
- **AND** MAY defer to the root README for detailed options

### Requirement: Example Doctests

Documentation examples MUST be valid Rust code that compiles and runs as doctests where feasible.

#### Scenario: Doctest with setup

- **WHEN** an example requires setup (imports, async runtime)
- **THEN** the example MUST include the necessary setup code
- **AND** hidden lines (`# `) MAY be used to reduce noise

#### Scenario: Doctest requiring browser

- **WHEN** an example requires a real browser connection
- **THEN** the example MUST use `no_run` attribute
- **AND** the example MUST still compile successfully
