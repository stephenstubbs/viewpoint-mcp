# testing Specification Delta

## ADDED Requirements

### Requirement: Module Directory Structure Hygiene

Module directories MUST NOT contain empty, unused subdirectories. When folder modules are used, only the module folder with actual content (mod.rs + submodules) should exist.

#### Scenario: No empty stub directories

- **WHEN** a module uses the folder module pattern (e.g., `server/mod.rs`)
- **AND** the module declares submodules as files (e.g., `server/protocol.rs`)
- **THEN** there MUST NOT be empty directories with the same name as file-based submodules (e.g., no empty `server/protocol/`)
- **AND** removing such empty directories has no functional impact
