## MODIFIED Requirements

### Requirement: Integration Test Entry Points

Integration test subdirectories in `tests/` MUST have a corresponding entry point `.rs` file in the `tests/` directory root that declares the subdirectory as a module.

#### Scenario: Subdirectory tests are discovered

- **WHEN** a test subdirectory exists with a `mod.rs` file (e.g., `tests/context/mod.rs`)
- **THEN** a corresponding entry point file MUST exist (e.g., `tests/context.rs`)
- **AND** the entry point file MUST declare the module (e.g., `mod context;`)
- **AND** running `cargo test --features integration` MUST discover and run tests from the subdirectory

#### Scenario: File size limits apply to tests

- **WHEN** a test file exceeds 500 lines
- **THEN** the file MUST be split into smaller, focused test modules
- **AND** each new module SHOULD group related tests by functionality
- **AND** a parent module MUST declare all sub-modules
