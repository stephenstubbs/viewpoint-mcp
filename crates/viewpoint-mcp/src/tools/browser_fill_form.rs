//! Browser fill form tool for filling multiple form fields at once

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;
use crate::snapshot::{AccessibilitySnapshot, SnapshotOptions};

/// Browser fill form tool - fills multiple form fields at once
pub struct BrowserFillFormTool;

/// Input parameters for `browser_fill_form`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserFillFormInput {
    /// Fields to fill in
    pub fields: Vec<FormField>,
}

/// A single form field to fill
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormField {
    /// Human-readable field name
    pub name: String,

    /// Type of the field
    #[serde(rename = "type")]
    pub field_type: FieldType,

    /// Element reference from snapshot
    #[serde(rename = "ref")]
    pub element_ref: String,

    /// Value to fill in the field
    /// For checkbox: "true" or "false"
    /// For combobox: the text of the option to select
    /// For slider: the numeric value as string
    /// For textbox/radio: the text value
    pub value: String,
}

/// Type of form field
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    /// Text input field
    Textbox,
    /// Checkbox input
    Checkbox,
    /// Radio button
    Radio,
    /// Dropdown/select element
    Combobox,
    /// Range slider
    Slider,
}

impl BrowserFillFormTool {
    /// Create a new browser fill form tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserFillFormTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserFillFormTool {
    fn name(&self) -> &'static str {
        "browser_fill_form"
    }

    fn description(&self) -> &'static str {
        "Fill multiple form fields at once. Supports textbox, checkbox, radio, combobox (dropdown), \
         and slider field types. Each field requires a ref from browser_snapshot."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["fields"],
            "properties": {
                "fields": {
                    "type": "array",
                    "description": "Fields to fill in",
                    "items": {
                        "type": "object",
                        "required": ["name", "type", "ref", "value"],
                        "properties": {
                            "name": {
                                "type": "string",
                                "description": "Human-readable field name"
                            },
                            "type": {
                                "type": "string",
                                "enum": ["textbox", "checkbox", "radio", "combobox", "slider"],
                                "description": "Type of the field"
                            },
                            "ref": {
                                "type": "string",
                                "description": "Exact target field reference from the page snapshot"
                            },
                            "value": {
                                "type": "string",
                                "description": "Value to fill in the field. For checkbox, use 'true' or 'false'. For combobox, use the option text."
                            }
                        }
                    }
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserFillFormInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        if input.fields.is_empty() {
            return Err(ToolError::InvalidParams(
                "At least one field must be provided".to_string(),
            ));
        }

        // Ensure browser is initialized
        browser
            .initialize()
            .await
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        // Get active page
        let context = browser
            .active_context_mut()
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        let page = context
            .active_page()
            .ok_or_else(|| ToolError::BrowserNotAvailable("No active page".to_string()))?;

        // Capture current snapshot for validation
        let options = SnapshotOptions::default();
        let snapshot = AccessibilitySnapshot::capture(page, options)
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        let mut filled_fields = Vec::new();

        // Fill each field
        for field in &input.fields {
            // Validate the ref exists in the snapshot
            snapshot.lookup(&field.element_ref).map_err(|e| {
                ToolError::ElementNotFound(format!(
                    "Element ref '{}' for field '{}': {}",
                    field.element_ref, field.name, e
                ))
            })?;

            // Use native ref resolution API from viewpoint 0.2.9
            let locator = page.locator_from_ref(&field.element_ref);

            // Fill based on field type
            match field.field_type {
                FieldType::Textbox => {
                    locator.fill(&field.value).await.map_err(|e| {
                        ToolError::ExecutionFailed(format!(
                            "Failed to fill textbox '{}': {}",
                            field.name, e
                        ))
                    })?;
                }
                FieldType::Checkbox => {
                    let should_check = field.value.eq_ignore_ascii_case("true");
                    if should_check {
                        locator.check().await
                    } else {
                        locator.uncheck().await
                    }
                    .map_err(|e| {
                        ToolError::ExecutionFailed(format!(
                            "Failed to set checkbox '{}': {}",
                            field.name, e
                        ))
                    })?;
                }
                FieldType::Radio => {
                    locator.check().await.map_err(|e| {
                        ToolError::ExecutionFailed(format!(
                            "Failed to select radio '{}': {}",
                            field.name, e
                        ))
                    })?;
                }
                FieldType::Combobox => {
                    // Use the new builder API from viewpoint 0.2.10
                    locator
                        .select_option()
                        .value(&field.value)
                        .await
                        .map_err(|e| {
                            ToolError::ExecutionFailed(format!(
                                "Failed to select option in '{}': {}",
                                field.name, e
                            ))
                        })?;
                }
                FieldType::Slider => {
                    // For sliders, we fill the value which works for range inputs
                    locator.fill(&field.value).await.map_err(|e| {
                        ToolError::ExecutionFailed(format!(
                            "Failed to set slider '{}': {}",
                            field.name, e
                        ))
                    })?;
                }
            }

            filled_fields.push(field.name.clone());
        }

        // Invalidate cache after form interaction
        context.invalidate_cache();

        Ok(format!(
            "Filled {} field(s): {}",
            filled_fields.len(),
            filled_fields.join(", ")
        ))
    }
}
