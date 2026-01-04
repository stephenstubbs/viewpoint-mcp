## ADDED Requirements

### Requirement: Integration Test Entry Points

Integration test subdirectories in `tests/` MUST have a corresponding entry point `.rs` file in the `tests/` directory root that declares the subdirectory as a module.

#### Scenario: Subdirectory tests are discovered

- **WHEN** a test subdirectory exists with a `mod.rs` file (e.g., `tests/context/mod.rs`)
- **THEN** a corresponding entry point file MUST exist (e.g., `tests/context.rs`)
- **AND** the entry point file MUST declare the module (e.g., `mod context;`)
- **AND** running `cargo test --features integration` MUST discover and run tests from the subdirectory

### Requirement: No Inline Test Modules

Source files MUST NOT contain inline test modules (`#[cfg(test)] mod tests { ... }` blocks). Tests MUST be placed in a `tests/` subdirectory within the module folder.

#### Scenario: Tests in separate folder

- **WHEN** a module needs unit tests
- **THEN** tests MUST be placed in `module/tests/*.rs`
- **AND** the module's `mod.rs` MUST reference them with `#[cfg(test)] mod tests;`
- **AND** the source file MUST NOT contain `mod tests { ... }` blocks
