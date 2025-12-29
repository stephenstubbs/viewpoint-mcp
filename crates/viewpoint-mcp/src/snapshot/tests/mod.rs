//! Unit tests for accessibility snapshot system

use crate::snapshot::classification::{classify_role, should_receive_ref, ElementTier};
use crate::snapshot::element::SnapshotElement;
use crate::snapshot::format::SnapshotFormatter;
use crate::snapshot::reference::{ElementRef, RefGenerator};
use crate::snapshot::stale::{StaleRefDetector, StaleRefError};

// =============================================================================
// Classification Tests
// =============================================================================

#[test]
fn test_classify_tier1_roles() {
    assert_eq!(classify_role("button"), ElementTier::AlwaysInteractive);
    assert_eq!(classify_role("link"), ElementTier::AlwaysInteractive);
    assert_eq!(classify_role("textbox"), ElementTier::AlwaysInteractive);
    assert_eq!(classify_role("checkbox"), ElementTier::AlwaysInteractive);
    assert_eq!(classify_role("radio"), ElementTier::AlwaysInteractive);
    assert_eq!(classify_role("combobox"), ElementTier::AlwaysInteractive);
    assert_eq!(classify_role("slider"), ElementTier::AlwaysInteractive);
    assert_eq!(classify_role("menuitem"), ElementTier::AlwaysInteractive);
    assert_eq!(classify_role("tab"), ElementTier::AlwaysInteractive);
    assert_eq!(classify_role("switch"), ElementTier::AlwaysInteractive);
    assert_eq!(classify_role("searchbox"), ElementTier::AlwaysInteractive);
    assert_eq!(classify_role("spinbutton"), ElementTier::AlwaysInteractive);
}

#[test]
fn test_classify_tier2_roles() {
    assert_eq!(
        classify_role("listitem"),
        ElementTier::ContextuallyInteractive
    );
    assert_eq!(classify_role("option"), ElementTier::ContextuallyInteractive);
    assert_eq!(
        classify_role("treeitem"),
        ElementTier::ContextuallyInteractive
    );
    assert_eq!(classify_role("row"), ElementTier::ContextuallyInteractive);
    assert_eq!(classify_role("cell"), ElementTier::ContextuallyInteractive);
}

#[test]
fn test_classify_tier3_roles() {
    assert_eq!(classify_role("heading"), ElementTier::NonInteractive);
    assert_eq!(classify_role("paragraph"), ElementTier::NonInteractive);
    assert_eq!(classify_role("text"), ElementTier::NonInteractive);
    assert_eq!(classify_role("separator"), ElementTier::NonInteractive);
    assert_eq!(classify_role("img"), ElementTier::NonInteractive);
    assert_eq!(classify_role("main"), ElementTier::NonInteractive);
    assert_eq!(classify_role("navigation"), ElementTier::NonInteractive);
}

#[test]
fn test_classify_case_insensitive() {
    assert_eq!(classify_role("BUTTON"), ElementTier::AlwaysInteractive);
    assert_eq!(classify_role("Button"), ElementTier::AlwaysInteractive);
    assert_eq!(classify_role("LISTITEM"), ElementTier::ContextuallyInteractive);
}

#[test]
fn test_classify_unknown_role() {
    assert_eq!(classify_role("unknown"), ElementTier::NonInteractive);
    assert_eq!(classify_role("custom-element"), ElementTier::NonInteractive);
}

#[test]
fn test_should_receive_ref_tier1() {
    // Tier 1 elements always receive refs
    assert!(should_receive_ref("button", false, false));
    assert!(should_receive_ref("link", false, false));
    assert!(should_receive_ref("textbox", false, false));
}

#[test]
fn test_should_receive_ref_tier2_in_container() {
    // Tier 2 elements only receive refs in interactive containers
    assert!(!should_receive_ref("listitem", false, false));
    assert!(should_receive_ref("listitem", true, false));
    assert!(should_receive_ref("option", true, false));
}

#[test]
fn test_should_receive_ref_with_tabindex() {
    // Elements with tabindex always receive refs
    assert!(should_receive_ref("heading", false, true));
    assert!(should_receive_ref("paragraph", false, true));
    assert!(should_receive_ref("div", false, true));
}

// =============================================================================
// Reference Tests
// =============================================================================

#[test]
fn test_element_ref_format() {
    let element_ref = ElementRef::new("abc123");
    assert_eq!(element_ref.to_ref_string(), "eabc123");
}

#[test]
fn test_element_ref_with_context() {
    let element_ref = ElementRef::with_context("abc123", "clean");
    assert_eq!(element_ref.to_ref_string(), "clean:eabc123");
}

#[test]
fn test_element_ref_parse_simple() {
    let parsed = ElementRef::parse("e1a2b3").unwrap();
    assert_eq!(parsed.hash, "1a2b3");
    assert!(parsed.context.is_none());
}

#[test]
fn test_element_ref_parse_with_context() {
    let parsed = ElementRef::parse("clean:e1a2b3").unwrap();
    assert_eq!(parsed.hash, "1a2b3");
    assert_eq!(parsed.context, Some("clean".to_string()));
}

#[test]
fn test_element_ref_parse_invalid() {
    assert!(ElementRef::parse("invalid").is_err());
    assert!(ElementRef::parse("e").is_err());
    assert!(ElementRef::parse("").is_err());
    assert!(ElementRef::parse("ctx:invalid").is_err());
}

#[test]
fn test_ref_generator_hash_stability() {
    let generator = RefGenerator::new();

    // Same inputs should produce same hash
    let ref1 = generator.generate(Some("my-id"), None, None, "button", Some("Submit"), "/0/1");
    let ref2 = generator.generate(Some("my-id"), None, None, "button", Some("Submit"), "/0/1");
    assert_eq!(ref1.hash, ref2.hash);
}

#[test]
fn test_ref_generator_id_priority() {
    let generator = RefGenerator::new();

    // ID takes priority over other attributes
    let ref_with_id = generator.generate(
        Some("submit-btn"),
        Some("test-submit"),
        Some("submit"),
        "button",
        Some("Submit"),
        "/0/1",
    );

    let ref_without_id = generator.generate(
        None,
        Some("test-submit"),
        Some("submit"),
        "button",
        Some("Submit"),
        "/0/1",
    );

    assert_ne!(ref_with_id.hash, ref_without_id.hash);
}

#[test]
fn test_ref_generator_with_context() {
    let generator = RefGenerator::with_context("uk");

    let element_ref = generator.generate(Some("my-id"), None, None, "button", Some("Submit"), "/0/1");

    assert_eq!(element_ref.context, Some("uk".to_string()));
    assert!(element_ref.to_ref_string().starts_with("uk:e"));
}

// =============================================================================
// Formatting Tests
// =============================================================================

#[test]
fn test_format_simple_element() {
    let element = SnapshotElement::new("button").with_name("Submit");

    let formatter = SnapshotFormatter::new();
    let output = formatter.format(&element);

    assert!(output.contains("button"));
    assert!(output.contains("Submit"));
}

#[test]
fn test_format_element_with_ref() {
    let element =
        SnapshotElement::new("button")
            .with_name("Submit")
            .with_ref(ElementRef::new("abc123"));

    let formatter = SnapshotFormatter::new();
    let output = formatter.format(&element);

    assert!(output.contains("[ref=eabc123]"));
}

#[test]
fn test_format_nested_elements() {
    let child = SnapshotElement::new("button").with_name("Click me");
    let parent = SnapshotElement::new("main").with_child(child);

    let formatter = SnapshotFormatter::new();
    let output = formatter.format(&parent);

    assert!(output.contains("main"));
    assert!(output.contains("button"));
    assert!(output.contains("Click me"));
}

#[test]
fn test_format_compact_mode_indicator() {
    let element = SnapshotElement::new("document");

    let formatter = SnapshotFormatter::new().with_compact_mode(true);
    let output = formatter.format(&element);

    assert!(output.contains("[Note:"));
    assert!(output.contains("allRefs: true"));
}

// =============================================================================
// Stale Detection Tests
// =============================================================================

#[test]
fn test_stale_detector_valid_ref() {
    let mut detector = StaleRefDetector::new();

    let element =
        SnapshotElement::new("button")
            .with_name("Submit")
            .with_ref(ElementRef::new("abc123"));

    detector.update(&element);

    let result = detector.validate_ref(&ElementRef::new("abc123"));
    assert!(result.is_ok());
}

#[test]
fn test_stale_detector_removed_element() {
    let mut detector = StaleRefDetector::new();

    // First snapshot has the element
    let element1 =
        SnapshotElement::new("button")
            .with_name("Submit")
            .with_ref(ElementRef::new("abc123"));
    detector.update(&element1);

    // Second snapshot doesn't have it
    let element2 = SnapshotElement::new("document");
    detector.update(&element2);

    let result = detector.validate_ref(&ElementRef::new("abc123"));
    assert!(matches!(result, Err(StaleRefError::ElementRemoved { .. })));
}

#[test]
fn test_stale_detector_element_changed() {
    let mut detector = StaleRefDetector::new();

    // First snapshot
    let element1 =
        SnapshotElement::new("button")
            .with_name("Submit")
            .with_ref(ElementRef::new("abc123"));
    detector.update(&element1);

    // Second snapshot - same ref but role changed
    let mut element2 = SnapshotElement::new("link");
    element2.name = Some("Submit".to_string());
    element2.element_ref = Some(ElementRef::new("abc123"));
    detector.update(&element2);

    let result = detector.validate_ref(&ElementRef::new("abc123"));
    assert!(matches!(result, Err(StaleRefError::ElementChanged { .. })));
}

#[test]
fn test_stale_detector_minor_change() {
    let mut detector = StaleRefDetector::new();

    // First snapshot
    let element1 =
        SnapshotElement::new("button")
            .with_name("Submit (0)")
            .with_ref(ElementRef::new("abc123"));
    detector.update(&element1);

    // Second snapshot - same role but name changed
    let mut element2 = SnapshotElement::new("button");
    element2.name = Some("Submit (1)".to_string());
    element2.element_ref = Some(ElementRef::new("abc123"));
    detector.update(&element2);

    let result = detector.validate_ref(&ElementRef::new("abc123"));
    assert!(matches!(result, Err(StaleRefError::MinorChange { .. })));
}

// =============================================================================
// Element Tests
// =============================================================================

#[test]
fn test_element_count_refs() {
    let child1 = SnapshotElement::new("button")
        .with_name("Button 1")
        .with_ref(ElementRef::new("btn1"));
    let child2 = SnapshotElement::new("link")
        .with_name("Link 1")
        .with_ref(ElementRef::new("link1"));
    let child3 = SnapshotElement::new("heading").with_name("Title"); // no ref

    let parent = SnapshotElement::new("main")
        .with_child(child1)
        .with_child(child2)
        .with_child(child3);

    assert_eq!(parent.count_refs(), 2);
}

#[test]
fn test_element_count_total() {
    let child1 = SnapshotElement::new("button");
    let child2 = SnapshotElement::new("link");
    let grandchild = SnapshotElement::new("text");
    let child3 = SnapshotElement::new("heading").with_child(grandchild);

    let parent = SnapshotElement::new("main")
        .with_child(child1)
        .with_child(child2)
        .with_child(child3);

    assert_eq!(parent.count_elements(), 5); // parent + 3 children + 1 grandchild
}
