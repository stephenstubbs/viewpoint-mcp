//! Snapshot element representation

use super::reference::ElementRef;

/// An element in the accessibility snapshot with reference information
#[derive(Debug, Clone)]
pub struct SnapshotElement {
    /// The ARIA role of the element
    pub role: String,

    /// The accessible name of the element
    pub name: Option<String>,

    /// The accessible description
    pub description: Option<String>,

    /// Whether this element is interactive and has a ref
    pub element_ref: Option<ElementRef>,

    /// Whether the element is disabled
    pub disabled: bool,

    /// Whether the element is expanded (for expandable elements)
    pub expanded: Option<bool>,

    /// Whether the element is selected
    pub selected: Option<bool>,

    /// Whether the element is checked (for checkboxes/radios)
    pub checked: Option<CheckedState>,

    /// Whether the element is pressed (for toggle buttons)
    pub pressed: Option<bool>,

    /// The level (for headings)
    pub level: Option<u32>,

    /// The value (for sliders, progress bars, etc.)
    pub value: Option<f64>,

    /// Whether this element is a frame boundary (iframe)
    pub is_frame: bool,

    /// Whether this element is an interactive container
    pub is_interactive_container: bool,

    /// Child elements
    pub children: Vec<Self>,
}

/// Checked state for checkboxes and similar elements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckedState {
    /// Checked state
    True,
    /// Unchecked state
    False,
    /// Mixed/indeterminate state
    Mixed,
}

impl SnapshotElement {
    /// Create a new snapshot element
    #[must_use]
    pub fn new(role: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            name: None,
            description: None,
            element_ref: None,
            disabled: false,
            expanded: None,
            selected: None,
            checked: None,
            pressed: None,
            level: None,
            value: None,
            is_frame: false,
            is_interactive_container: false,
            children: Vec::new(),
        }
    }

    /// Set the accessible name
    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the element reference
    #[must_use]
    pub fn with_ref(mut self, element_ref: ElementRef) -> Self {
        self.element_ref = Some(element_ref);
        self
    }

    /// Add a child element
    #[must_use]
    pub fn with_child(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }

    /// Check if this element has an assigned ref
    #[must_use]
    pub fn has_ref(&self) -> bool {
        self.element_ref.is_some()
    }

    /// Get the ref string if available
    #[must_use]
    pub fn ref_string(&self) -> Option<String> {
        self.element_ref.as_ref().map(super::reference::ElementRef::to_ref_string)
    }

    /// Count all elements with refs in this subtree
    #[must_use]
    pub fn count_refs(&self) -> usize {
        let self_count = usize::from(self.has_ref());
        let children_count: usize = self.children.iter().map(Self::count_refs).sum();
        self_count + children_count
    }

    /// Count all elements in this subtree
    #[must_use]
    pub fn count_elements(&self) -> usize {
        1 + self.children.iter().map(Self::count_elements).sum::<usize>()
    }

    /// Count both refs and elements in a single pass
    ///
    /// Returns `(ref_count, element_count)` for efficient counting.
    #[must_use]
    pub fn counts(&self) -> (usize, usize) {
        let self_refs = usize::from(self.has_ref());
        let (child_refs, child_elements): (usize, usize) = self
            .children
            .iter()
            .map(Self::counts)
            .fold((0, 0), |(r, e), (cr, ce)| (r + cr, e + ce));

        (self_refs + child_refs, 1 + child_elements)
    }
}
