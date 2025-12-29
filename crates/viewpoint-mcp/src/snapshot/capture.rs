//! Main accessibility snapshot implementation

use std::collections::HashMap;

use viewpoint_core::page::locator::aria::AriaSnapshot as VpAriaSnapshot;
use viewpoint_core::Page;

use super::classification::{is_interactive_container, should_receive_ref};
use super::element::{CheckedState, SnapshotElement};
use super::error::{SnapshotError, SnapshotResult};
use super::format::SnapshotFormatter;
use super::reference::{ElementRef, RefGenerator};
use super::stale::StaleRefDetector;

/// Threshold for switching to compact mode
const COMPACT_MODE_THRESHOLD: usize = 100;

/// Options for snapshot capture
#[derive(Debug, Clone, Default)]
pub struct SnapshotOptions {
    /// Whether to include all refs (bypass compact mode)
    pub all_refs: bool,

    /// Context name for multi-context mode
    pub context: Option<String>,
}

/// The main accessibility snapshot for a page
#[derive(Debug)]
pub struct AccessibilitySnapshot {
    /// The root element of the snapshot tree
    root: SnapshotElement,

    /// Map from ref hash to element (for lookup)
    ref_map: HashMap<String, ElementRef>,

    /// Whether compact mode is active
    compact_mode: bool,

    /// The formatter used
    formatter: SnapshotFormatter,

    /// Stale reference detector
    stale_detector: StaleRefDetector,

    /// Context name
    context: Option<String>,
}

impl AccessibilitySnapshot {
    /// Capture an accessibility snapshot from a page
    ///
    /// This captures the full accessibility tree including iframe content.
    /// Frame boundaries are marked in the output for clarity.
    ///
    /// # Errors
    ///
    /// Returns an error if the accessibility tree cannot be captured
    pub async fn capture(page: &Page, options: SnapshotOptions) -> SnapshotResult<Self> {
        // Capture aria snapshot with frame content stitched in
        let aria_snapshot = page
            .aria_snapshot_with_frames()
            .await
            .map_err(|e| SnapshotError::CaptureError(e.to_string()))?;

        // Convert to our internal representation
        let ref_generator = match &options.context {
            Some(ctx) => RefGenerator::with_context(ctx),
            None => RefGenerator::new(),
        };

        let mut ref_map = HashMap::new();
        let root = Self::convert_aria_snapshot(
            &aria_snapshot,
            &ref_generator,
            &mut ref_map,
            false, // not in interactive container initially
            "",    // empty DOM path
            options.all_refs,
        );

        // Determine if we need compact mode
        let interactive_count = root.count_refs();
        let compact_mode = !options.all_refs && interactive_count > COMPACT_MODE_THRESHOLD;

        let formatter = SnapshotFormatter::new()
            .with_all_refs(options.all_refs)
            .with_compact_mode(compact_mode);

        let mut snapshot = Self {
            root,
            ref_map,
            compact_mode,
            formatter,
            stale_detector: StaleRefDetector::new(),
            context: options.context,
        };

        // Update stale detector with this snapshot
        snapshot.stale_detector.update(&snapshot.root);

        Ok(snapshot)
    }

    /// Convert a viewpoint `AriaSnapshot` to our internal representation
    fn convert_aria_snapshot(
        aria: &VpAriaSnapshot,
        ref_gen: &RefGenerator,
        ref_map: &mut HashMap<String, ElementRef>,
        in_interactive_container: bool,
        dom_path: &str,
        all_refs: bool,
    ) -> SnapshotElement {
        let role = aria.role.clone().unwrap_or_else(|| "none".to_string());
        let name = aria.name.clone();

        let mut element = SnapshotElement::new(&role);
        element.name.clone_from(&name);
        element.description.clone_from(&aria.description);
        element.disabled = aria.disabled.unwrap_or(false);
        element.expanded = aria.expanded;
        element.selected = aria.selected;
        element.pressed = aria.pressed;
        element.level = aria.level;
        element.value = aria.value_now;
        element.is_frame = aria.is_frame.unwrap_or(false);
        element.dom_path = dom_path.to_string();

        // Convert checked state
        if let Some(checked) = &aria.checked {
            element.checked = Some(match checked {
                viewpoint_core::page::locator::aria::AriaCheckedState::True => CheckedState::True,
                viewpoint_core::page::locator::aria::AriaCheckedState::False => CheckedState::False,
                viewpoint_core::page::locator::aria::AriaCheckedState::Mixed => CheckedState::Mixed,
            });
        }

        // Determine if this element should receive a ref
        let is_container = is_interactive_container(&role);
        let should_ref = all_refs || should_receive_ref(&role, in_interactive_container, false);

        if should_ref {
            // Generate ref
            let element_ref = ref_gen.generate(
                element.attributes.id.as_deref(),
                element.attributes.test_id.as_deref(),
                element.attributes.name.as_deref(),
                &role,
                name.as_deref(),
                dom_path,
            );
            ref_map.insert(element_ref.hash.clone(), element_ref.clone());
            element.element_ref = Some(element_ref);
        }

        // Process children
        let child_in_container = in_interactive_container || is_container;
        for (i, child) in aria.children.iter().enumerate() {
            let child_path = format!("{dom_path}/{i}");
            let child_element = Self::convert_aria_snapshot(
                child,
                ref_gen,
                ref_map,
                child_in_container,
                &child_path,
                all_refs,
            );
            element.children.push(child_element);
        }

        element
    }

    /// Format the snapshot as text for LLM consumption
    #[must_use]
    pub fn format(&self) -> String {
        self.formatter.format(&self.root)
    }

    /// Look up an element by its reference
    pub fn lookup(&self, ref_string: &str) -> SnapshotResult<&ElementRef> {
        let element_ref = ElementRef::parse(ref_string)
            .map_err(SnapshotError::InvalidRefFormat)?;

        // Validate against stale detector
        if let Err(stale_err) = self.stale_detector.validate_ref(&element_ref) {
            return Err(SnapshotError::StaleRef(stale_err.to_string()));
        }

        self.ref_map
            .get(&element_ref.hash)
            .ok_or_else(|| SnapshotError::RefNotFound(ref_string.to_string()))
    }

    /// Get the root element
    #[must_use]
    pub fn root(&self) -> &SnapshotElement {
        &self.root
    }

    /// Check if compact mode is active
    #[must_use]
    pub fn is_compact(&self) -> bool {
        self.compact_mode
    }

    /// Get the number of elements with refs
    #[must_use]
    pub fn ref_count(&self) -> usize {
        self.ref_map.len()
    }

    /// Get the total element count
    #[must_use]
    pub fn element_count(&self) -> usize {
        self.root.count_elements()
    }

    /// Get the context name
    #[must_use]
    pub fn context(&self) -> Option<&str> {
        self.context.as_deref()
    }
}
