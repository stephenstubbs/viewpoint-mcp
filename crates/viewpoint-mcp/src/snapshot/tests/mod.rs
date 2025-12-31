//! Unit tests for accessibility snapshot system

use crate::snapshot::classification::{classify_role, should_receive_ref, ElementTier};
use crate::snapshot::element::SnapshotElement;
use crate::snapshot::format::SnapshotFormatter;
use crate::snapshot::reference::ElementRef;
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
    // ElementRef now stores the full ref string as provided by viewpoint-core
    let element_ref = ElementRef::new("e12345");
    assert_eq!(element_ref.to_ref_string(), "e12345");
    assert_eq!(element_ref.ref_string(), "e12345");
}

#[test]
fn test_element_ref_with_context() {
    let element_ref = ElementRef::with_context("e12345", "clean");
    assert_eq!(element_ref.to_ref_string(), "clean:e12345");
    assert_eq!(element_ref.ref_string(), "e12345");
    assert_eq!(element_ref.context(), Some("clean"));
}

#[test]
fn test_element_ref_parse_simple() {
    let parsed = ElementRef::parse("e12345").unwrap();
    assert_eq!(parsed.ref_string(), "e12345");
    assert!(parsed.context().is_none());
}

#[test]
fn test_element_ref_parse_with_context() {
    let parsed = ElementRef::parse("clean:e12345").unwrap();
    assert_eq!(parsed.ref_string(), "e12345");
    assert_eq!(parsed.context(), Some("clean"));
}

#[test]
fn test_element_ref_parse_invalid() {
    // Invalid: must start with 'e' followed by digits
    assert!(ElementRef::parse("invalid").is_err());
    assert!(ElementRef::parse("e").is_err());
    assert!(ElementRef::parse("").is_err());
    assert!(ElementRef::parse("ctx:invalid").is_err());
    // Invalid: hash-based refs are no longer supported
    assert!(ElementRef::parse("eabc123").is_err());
    assert!(ElementRef::parse("e1a2b3").is_err());
}

#[test]
fn test_element_ref_parse_valid_formats() {
    // Valid viewpoint-core format: e{backendNodeId}
    assert!(ElementRef::parse("e1").is_ok());
    assert!(ElementRef::parse("e12345").is_ok());
    assert!(ElementRef::parse("e999999").is_ok());
    assert!(ElementRef::parse("main:e42").is_ok());
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
            .with_ref(ElementRef::new("e12345"));

    let formatter = SnapshotFormatter::new();
    let output = formatter.format(&element);

    assert!(output.contains("[ref=e12345]"));
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
            .with_ref(ElementRef::new("e12345"));

    detector.update(&element);

    let result = detector.validate_ref(&ElementRef::new("e12345"));
    assert!(result.is_ok());
}

#[test]
fn test_stale_detector_removed_element() {
    let mut detector = StaleRefDetector::new();

    // First snapshot has the element
    let element1 =
        SnapshotElement::new("button")
            .with_name("Submit")
            .with_ref(ElementRef::new("e12345"));
    detector.update(&element1);

    // Second snapshot doesn't have it
    let element2 = SnapshotElement::new("document");
    detector.update(&element2);

    let result = detector.validate_ref(&ElementRef::new("e12345"));
    assert!(matches!(result, Err(StaleRefError::ElementRemoved { .. })));
}

#[test]
fn test_stale_detector_element_changed() {
    let mut detector = StaleRefDetector::new();

    // First snapshot
    let element1 =
        SnapshotElement::new("button")
            .with_name("Submit")
            .with_ref(ElementRef::new("e12345"));
    detector.update(&element1);

    // Second snapshot - same ref but role changed
    let mut element2 = SnapshotElement::new("link");
    element2.name = Some("Submit".to_string());
    element2.element_ref = Some(ElementRef::new("e12345"));
    detector.update(&element2);

    let result = detector.validate_ref(&ElementRef::new("e12345"));
    assert!(matches!(result, Err(StaleRefError::ElementChanged { .. })));
}

#[test]
fn test_stale_detector_minor_change() {
    let mut detector = StaleRefDetector::new();

    // First snapshot
    let element1 =
        SnapshotElement::new("button")
            .with_name("Submit (0)")
            .with_ref(ElementRef::new("e12345"));
    detector.update(&element1);

    // Second snapshot - same role but name changed
    let mut element2 = SnapshotElement::new("button");
    element2.name = Some("Submit (1)".to_string());
    element2.element_ref = Some(ElementRef::new("e12345"));
    detector.update(&element2);

    let result = detector.validate_ref(&ElementRef::new("e12345"));
    assert!(matches!(result, Err(StaleRefError::MinorChange { .. })));
}

// =============================================================================
// Element Tests
// =============================================================================

#[test]
fn test_element_count_refs() {
    let child1 = SnapshotElement::new("button")
        .with_name("Button 1")
        .with_ref(ElementRef::new("e1"));
    let child2 = SnapshotElement::new("link")
        .with_name("Link 1")
        .with_ref(ElementRef::new("e2"));
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

#[test]
fn test_element_counts_single_pass() {
    let child1 = SnapshotElement::new("button")
        .with_name("Button 1")
        .with_ref(ElementRef::new("e1"));
    let child2 = SnapshotElement::new("link")
        .with_name("Link 1")
        .with_ref(ElementRef::new("e2"));
    let child3 = SnapshotElement::new("heading").with_name("Title"); // no ref
    let grandchild = SnapshotElement::new("text"); // no ref

    let parent = SnapshotElement::new("main")
        .with_child(child1)
        .with_child(child2)
        .with_child(child3.with_child(grandchild));

    let (ref_count, element_count) = parent.counts();

    // Verify single-pass counts match individual method results
    assert_eq!(ref_count, parent.count_refs());
    assert_eq!(element_count, parent.count_elements());

    // Also verify specific values
    assert_eq!(ref_count, 2); // button and link have refs
    assert_eq!(element_count, 5); // parent + 3 children + 1 grandchild
}
