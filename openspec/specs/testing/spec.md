# testing Specification

## Purpose
Define the test organization patterns and conventions for the viewpoint-mcp codebase, including integration test structure, module organization, and directory hygiene rules.
## Requirements
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

### Requirement: No Inline Test Modules

Source files MUST NOT contain inline test modules (`#[cfg(test)] mod tests { ... }` blocks). Tests MUST be placed in a `tests/` subdirectory within the module folder.

#### Scenario: Tests in separate folder

- **WHEN** a module needs unit tests
- **THEN** tests MUST be placed in `module/tests/*.rs`
- **AND** the module's `mod.rs` MUST reference them with `#[cfg(test)] mod tests;`
- **AND** the source file MUST NOT contain `mod tests { ... }` blocks

### Requirement: Module Directory Structure Hygiene

Module directories MUST NOT contain empty, unused subdirectories. When folder modules are used, only the module folder with actual content (mod.rs + submodules) should exist.

#### Scenario: No empty stub directories

- **WHEN** a module uses the folder module pattern (e.g., `server/mod.rs`)
- **AND** the module declares submodules as files (e.g., `server/protocol.rs`)
- **THEN** there MUST NOT be empty directories with the same name as file-based submodules (e.g., no empty `server/protocol/`)
- **AND** removing such empty directories has no functional impact

