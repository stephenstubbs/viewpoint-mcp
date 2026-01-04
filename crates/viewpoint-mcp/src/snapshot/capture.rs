//! Main accessibility snapshot implementation

use std::collections::HashMap;

use viewpoint_core::Page;
use viewpoint_core::page::locator::aria::AriaSnapshot as VpAriaSnapshot;

use super::classification::is_interactive_container;
use super::element::{CheckedState, SnapshotElement};
use super::error::{SnapshotError, SnapshotResult};
use super::format::SnapshotFormatter;
use super::reference::ElementRef;
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

    /// Map from ref string (e.g., "c0p0f0e1") to `ElementRef` (for lookup)
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
    /// For pages with empty or minimal accessibility trees (e.g., blank pages,
    /// pages still loading), returns a minimal document node.
    ///
    /// # Errors
    ///
    /// Returns an error if the accessibility tree cannot be captured
    pub async fn capture(page: &Page, options: SnapshotOptions) -> SnapshotResult<Self> {
        // Capture aria snapshot with refs, including iframe content
        // Handle empty accessibility trees gracefully
        let aria_snapshot = match page.aria_snapshot_with_frames().await {
            Ok(snapshot) => snapshot,
            Err(e) => {
                let error_msg = e.to_string();
                // Check if this is the null/empty accessibility tree error
                if error_msg.contains("invalid type: null")
                    || error_msg.contains("expected struct AriaSnapshot")
                {
                    // Return empty snapshot for empty pages
                    return Ok(Self::empty_snapshot(options.context));
                }
                return Err(SnapshotError::CaptureError(error_msg));
            }
        };

        let mut ref_map = HashMap::new();
        let root =
            Self::convert_aria_snapshot(&aria_snapshot, &mut ref_map, options.context.as_deref());

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

    /// Create an empty snapshot for pages with no accessibility tree
    ///
    /// This returns a minimal document node, which is the expected output
    /// for blank pages or pages with no accessible content.
    fn empty_snapshot(context: Option<String>) -> Self {
        let root = SnapshotElement::new("document");
        let formatter = SnapshotFormatter::new();

        Self {
            root,
            ref_map: HashMap::new(),
            compact_mode: false,
            formatter,
            stale_detector: StaleRefDetector::new(),
            context,
        }
    }

    /// Convert a viewpoint `AriaSnapshot` to our internal representation
    ///
    /// Uses viewpoint-core's native `node_ref` field which provides refs in the
    /// correct format (`e{backendNodeId}`) for use with `locator_from_ref()`.
    fn convert_aria_snapshot(
        aria: &VpAriaSnapshot,
        ref_map: &mut HashMap<String, ElementRef>,
        context: Option<&str>,
    ) -> SnapshotElement {
        let role = aria.role.clone().unwrap_or_else(|| "none".to_string());

        let mut element = SnapshotElement::new(&role);
        element.name.clone_from(&aria.name);
        element.description.clone_from(&aria.description);
        element.disabled = aria.disabled.unwrap_or(false);
        element.expanded = aria.expanded;
        element.selected = aria.selected;
        element.pressed = aria.pressed;
        element.level = aria.level;
        element.value = aria.value_now;
        element.is_frame = aria.is_frame.unwrap_or(false);

        // Convert checked state
        if let Some(checked) = &aria.checked {
            element.checked = Some(match checked {
                viewpoint_core::page::locator::aria::AriaCheckedState::True => CheckedState::True,
                viewpoint_core::page::locator::aria::AriaCheckedState::False => CheckedState::False,
                viewpoint_core::page::locator::aria::AriaCheckedState::Mixed => CheckedState::Mixed,
            });
        }

        // Use viewpoint-core's native ref if available
        // The node_ref field provides refs in the format `e{backendNodeId}`
        // which is exactly what `locator_from_ref()` expects
        if let Some(ref_string) = &aria.node_ref {
            let element_ref = match context {
                Some(ctx) => ElementRef::with_context(ref_string, ctx),
                None => ElementRef::new(ref_string),
            };
            ref_map.insert(ref_string.clone(), element_ref.clone());
            element.element_ref = Some(element_ref);
        }

        // Process children recursively
        let is_container = is_interactive_container(&role);
        for child in &aria.children {
            let child_element = Self::convert_aria_snapshot(child, ref_map, context);
            element.children.push(child_element);
        }

        // Track container status for potential future use
        element.is_interactive_container = is_container;

        element
    }

    /// Format the snapshot as text for LLM consumption
    #[must_use]
    pub fn format(&self) -> String {
        self.formatter.format(&self.root)
    }

    /// Look up an element by its reference
    pub fn lookup(&self, ref_str: &str) -> SnapshotResult<&ElementRef> {
        let element_ref = ElementRef::parse(ref_str).map_err(SnapshotError::InvalidRefFormat)?;

        // Validate against stale detector
        if let Err(stale_err) = self.stale_detector.validate_ref(&element_ref) {
            return Err(SnapshotError::StaleRef(stale_err.to_string()));
        }

        // Look up by the raw ref string (e.g., "c0p0f0e1")
        self.ref_map
            .get(element_ref.ref_string())
            .ok_or_else(|| SnapshotError::RefNotFound(ref_str.to_string()))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_snapshot_has_document_root() {
        let snapshot = AccessibilitySnapshot::empty_snapshot(None);

        assert_eq!(snapshot.root().role, "document");
        assert!(snapshot.root().children.is_empty());
        assert_eq!(snapshot.ref_count(), 0);
        assert_eq!(snapshot.element_count(), 1);
        assert!(!snapshot.is_compact());
    }

    #[test]
    fn test_empty_snapshot_with_context() {
        let snapshot = AccessibilitySnapshot::empty_snapshot(Some("test-context".to_string()));

        assert_eq!(snapshot.context(), Some("test-context"));
        assert_eq!(snapshot.root().role, "document");
    }

    #[test]
    fn test_empty_snapshot_format() {
        let snapshot = AccessibilitySnapshot::empty_snapshot(None);
        let output = snapshot.format();

        // Should contain "document" since that's the root role
        assert!(output.contains("document"));
    }
}
