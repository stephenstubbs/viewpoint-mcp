//! Snapshot formatting for LLM consumption

use std::fmt::Write;
use std::sync::LazyLock;

use super::element::SnapshotElement;

/// Maximum text content length before truncation
const MAX_TEXT_LENGTH: usize = 100;

/// Default indentation string
const INDENT: &str = "  ";

/// Maximum pre-allocated depth for indent strings
const MAX_PREALLOC_DEPTH: usize = 32;

/// Estimated bytes per element for buffer pre-allocation
const ESTIMATED_BYTES_PER_ELEMENT: usize = 80;

/// Pre-allocated indent strings for common depths (0 to `MAX_PREALLOC_DEPTH`)
static INDENT_CACHE: LazyLock<Vec<String>> = LazyLock::new(|| {
    (0..=MAX_PREALLOC_DEPTH)
        .map(|depth| INDENT.repeat(depth))
        .collect()
});

/// Formatter for accessibility snapshots
#[derive(Debug, Default)]
pub struct SnapshotFormatter {
    /// Whether to show all refs (including Tier 2)
    pub all_refs: bool,

    /// Maximum depth to format (-1 for unlimited)
    pub max_depth: i32,

    /// Whether we're in compact mode (>100 interactive elements)
    pub compact_mode: bool,
}

impl SnapshotFormatter {
    /// Create a new formatter with default settings
    #[must_use]
    pub fn new() -> Self {
        Self {
            all_refs: false,
            max_depth: -1,
            compact_mode: false,
        }
    }

    /// Enable all refs output
    #[must_use]
    pub fn with_all_refs(mut self, all_refs: bool) -> Self {
        self.all_refs = all_refs;
        self
    }

    /// Set compact mode
    #[must_use]
    pub fn with_compact_mode(mut self, compact: bool) -> Self {
        self.compact_mode = compact;
        self
    }

    /// Format a snapshot element tree as indented text
    #[must_use]
    pub fn format(&self, root: &SnapshotElement) -> String {
        self.format_with_hint(root, None)
    }

    /// Format a snapshot element tree with an optional element count hint for buffer sizing
    #[must_use]
    pub fn format_with_hint(
        &self,
        root: &SnapshotElement,
        element_count_hint: Option<usize>,
    ) -> String {
        // Pre-allocate output buffer based on element count
        let capacity = element_count_hint.map_or(1024, |count| count * ESTIMATED_BYTES_PER_ELEMENT);
        let mut output = String::with_capacity(capacity);

        self.format_element(&mut output, root, 0);

        if self.compact_mode {
            output.push_str("\n[Note: Page has many interactive elements. ");
            output.push_str("Use browser_snapshot with allRefs: true for complete refs.]");
        }

        output
    }

    /// Format a single element and its children
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn format_element(&self, output: &mut String, element: &SnapshotElement, depth: usize) {
        // Check depth limit
        if self.max_depth >= 0 && depth as i32 > self.max_depth {
            return;
        }

        // Use pre-allocated indent string if available, otherwise compute it
        let indent: std::borrow::Cow<'static, str> = if depth <= MAX_PREALLOC_DEPTH {
            std::borrow::Cow::Borrowed(&INDENT_CACHE[depth])
        } else {
            std::borrow::Cow::Owned(INDENT.repeat(depth))
        };

        // Format the element line
        output.push_str(&indent);
        output.push_str("- ");
        output.push_str(&element.role);

        // Add accessible name if present
        if let Some(name) = &element.name {
            let truncated = truncate_text(name, MAX_TEXT_LENGTH);
            let _ = write!(output, " \"{truncated}\"");
        }

        // Add frame boundary marker
        if element.is_frame {
            output.push_str(" [frame-boundary]");
        }

        // Add state indicators
        Self::format_state(output, element);

        // Add ref if present
        if let Some(ref_str) = element.ref_string() {
            let _ = write!(output, " [ref={ref_str}]");
        }

        output.push('\n');

        // Format children
        for child in &element.children {
            self.format_element(output, child, depth + 1);
        }
    }

    /// Format element state indicators
    fn format_state(output: &mut String, element: &SnapshotElement) {
        if element.disabled {
            output.push_str(" (disabled)");
        }

        if let Some(expanded) = element.expanded {
            if expanded {
                output.push_str(" (expanded)");
            } else {
                output.push_str(" (collapsed)");
            }
        }

        if let Some(selected) = element.selected
            && selected
        {
            output.push_str(" (selected)");
        }

        if let Some(checked) = &element.checked {
            match checked {
                super::element::CheckedState::True => output.push_str(" (checked)"),
                super::element::CheckedState::False => output.push_str(" (unchecked)"),
                super::element::CheckedState::Mixed => output.push_str(" (mixed)"),
            }
        }

        if let Some(pressed) = element.pressed
            && pressed
        {
            output.push_str(" (pressed)");
        }

        if let Some(level) = element.level {
            let _ = write!(output, " (level {level})");
        }

        if let Some(value) = element.value {
            let _ = write!(output, " (value: {value})");
        }
    }
}

/// Truncate text to a maximum length with ellipsis
fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len - 3])
    }
}
