//! Tests for `browser_fill_form` tool

use crate::tools::Tool;
use crate::tools::browser_fill_form::{BrowserFillFormInput, BrowserFillFormTool, FieldType};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserFillFormTool::new();

    assert_eq!(tool.name(), "browser_fill_form");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("fields"))
    );
}

#[test]
fn test_input_parsing() {
    let input: BrowserFillFormInput = serde_json::from_value(json!({
        "fields": [
            {
                "name": "Email",
                "type": "textbox",
                "ref": "c0p0f0e1",
                "value": "user@example.com"
            }
        ]
    }))
    .unwrap();

    assert_eq!(input.fields.len(), 1);
    assert_eq!(input.fields[0].name, "Email");
    assert_eq!(input.fields[0].field_type, FieldType::Textbox);
    assert_eq!(input.fields[0].element_ref, "c0p0f0e1");
    assert_eq!(input.fields[0].value, "user@example.com");
}

#[test]
fn test_multiple_fields() {
    let input: BrowserFillFormInput = serde_json::from_value(json!({
        "fields": [
            {
                "name": "Username",
                "type": "textbox",
                "ref": "c0p0f0e1",
                "value": "johndoe"
            },
            {
                "name": "Remember me",
                "type": "checkbox",
                "ref": "c0p0f0e2",
                "value": "true"
            },
            {
                "name": "Country",
                "type": "combobox",
                "ref": "c0p0f0e3",
                "value": "United States"
            }
        ]
    }))
    .unwrap();

    assert_eq!(input.fields.len(), 3);
    assert_eq!(input.fields[0].field_type, FieldType::Textbox);
    assert_eq!(input.fields[1].field_type, FieldType::Checkbox);
    assert_eq!(input.fields[2].field_type, FieldType::Combobox);
}

#[test]
fn test_all_field_types() {
    let input: BrowserFillFormInput = serde_json::from_value(json!({
        "fields": [
            { "name": "Text", "type": "textbox", "ref": "c0p0f0e1", "value": "test" },
            { "name": "Check", "type": "checkbox", "ref": "c0p0f0e2", "value": "false" },
            { "name": "Radio", "type": "radio", "ref": "c0p0f0e3", "value": "option1" },
            { "name": "Select", "type": "combobox", "ref": "c0p0f0e4", "value": "Option A" },
            { "name": "Range", "type": "slider", "ref": "c0p0f0e5", "value": "50" }
        ]
    }))
    .unwrap();

    assert_eq!(input.fields.len(), 5);
    assert_eq!(input.fields[0].field_type, FieldType::Textbox);
    assert_eq!(input.fields[1].field_type, FieldType::Checkbox);
    assert_eq!(input.fields[2].field_type, FieldType::Radio);
    assert_eq!(input.fields[3].field_type, FieldType::Combobox);
    assert_eq!(input.fields[4].field_type, FieldType::Slider);
}
