# code-conventions Specification

## Purpose
TBD - created by archiving change audit-convention-compliance. Update Purpose after archive.
## Requirements
### Requirement: JavaScript Generation with js! Macro

Tool implementations that generate JavaScript code for browser evaluation MUST use the `viewpoint_js::js!` macro instead of raw `format!` strings or string literals.

#### Scenario: JavaScript code uses js! macro

- **WHEN** a tool needs to generate JavaScript for `page.evaluate()` or `page.wait_for_function()`
- **THEN** the JavaScript MUST be generated using `viewpoint_js::js!`
- **AND** dynamic values MUST be interpolated using `@{}` syntax
- **AND** raw string interpolation (for user-provided code) MUST use `@{}` syntax

#### Scenario: Compile-time validation

- **WHEN** JavaScript is generated with the `js!` macro
- **THEN** syntax errors are caught at compile time
- **AND** proper escaping is handled automatically

#### Scenario: Consistent codebase style

- **WHEN** reviewing JavaScript generation across tools
- **THEN** all tools SHALL use the same `js!` macro approach
- **AND** no raw `format!` or string literals containing JavaScript SHALL exist in tool implementations

