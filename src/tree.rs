//! Tree component for hierarchical data display.
//!
//! Trees render hierarchical data with guide lines connecting
//! parent and child nodes.

use crate::errors::Result;
use crate::measure::{cell_len, Measurable, MeasureOptions, Measurement};
use crate::segment::{Segment, Segments};
use crate::style::Style;
use crate::text::Text;
use serde::{Deserialize, Serialize};

/// Guide characters for tree rendering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TreeGuides {
    /// Vertical line.
    pub vertical: &'static str,
    /// Branch to child.
    pub branch: &'static str,
    /// Last child branch.
    pub last_branch: &'static str,
    /// Horizontal line.
    pub horizontal: &'static str,
    /// Spacing.
    pub space: &'static str,
}

impl TreeGuides {
    /// ASCII tree guides.
    pub const ASCII: Self = Self {
        vertical: "|",
        branch: "+--",
        last_branch: "+--",
        horizontal: "-",
        space: "   ",
    };

    /// Unicode tree guides (default).
    pub const UNICODE: Self = Self {
        vertical: "│",
        branch: "├──",
        last_branch: "└──",
        horizontal: "─",
        space: "   ",
    };

    /// Unicode tree guides with rounded corners.
    pub const ROUNDED: Self = Self {
        vertical: "│",
        branch: "├──",
        last_branch: "╰──",
        horizontal: "─",
        space: "   ",
    };

    /// Bold unicode tree guides.
    pub const BOLD: Self = Self {
        vertical: "┃",
        branch: "┣━━",
        last_branch: "┗━━",
        horizontal: "━",
        space: "   ",
    };

    /// Double-line tree guides.
    pub const DOUBLE: Self = Self {
        vertical: "║",
        branch: "╠══",
        last_branch: "╚══",
        horizontal: "═",
        space: "   ",
    };
}

impl Default for TreeGuides {
    fn default() -> Self {
        Self::UNICODE
    }
}

/// A node in the tree.
#[derive(Debug, Clone)]
pub struct TreeNode {
    /// The label for this node.
    label: Text,
    /// Child nodes.
    children: Vec<TreeNode>,
    /// Style for the label.
    style: Option<Style>,
    /// Style for the guide lines.
    guide_style: Option<Style>,
    /// Whether this node is expanded.
    expanded: bool,
}

impl TreeNode {
    /// Creates a new tree node with the given label.
    #[inline]
    #[must_use]
    pub fn new(label: impl Into<Text>) -> Self {
        Self {
            label: label.into(),
            children: Vec::new(),
            style: None,
            guide_style: None,
            expanded: true,
        }
    }

    /// Adds a child node.
    #[inline]
    pub fn add_child(&mut self, child: Self) {
        self.children.push(child);
    }

    /// Adds a child node and returns self for chaining.
    #[inline]
    #[must_use]
    pub fn with_child(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }

    /// Sets the style for this node.
    #[inline]
    #[must_use]
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    /// Sets the guide style for this node.
    #[inline]
    #[must_use]
    pub fn guide_style(mut self, style: Style) -> Self {
        self.guide_style = Some(style);
        self
    }

    /// Sets whether this node is expanded.
    #[inline]
    #[must_use]
    pub const fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Returns the label text.
    #[inline]
    #[must_use]
    pub fn label(&self) -> &Text {
        &self.label
    }

    /// Returns the children.
    #[inline]
    #[must_use]
    pub fn children(&self) -> &[Self] {
        &self.children
    }

    /// Returns whether this node has children.
    #[inline]
    #[must_use]
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

impl From<&str> for TreeNode {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for TreeNode {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

/// A tree for displaying hierarchical data.
#[derive(Debug, Clone)]
pub struct Tree {
    /// The root node.
    root: TreeNode,
    /// Tree guide characters.
    guides: TreeGuides,
    /// Style for guide lines.
    guide_style: Option<Style>,
    /// Whether to hide the root.
    hide_root: bool,
}

impl Tree {
    /// Creates a new tree with the given root label.
    #[inline]
    #[must_use]
    pub fn new(label: impl Into<Text>) -> Self {
        Self {
            root: TreeNode::new(label),
            guides: TreeGuides::default(),
            guide_style: None,
            hide_root: false,
        }
    }

    /// Creates a tree from an existing node.
    #[inline]
    #[must_use]
    pub const fn from_node(root: TreeNode) -> Self {
        Self {
            root,
            guides: TreeGuides::UNICODE,
            guide_style: None,
            hide_root: false,
        }
    }

    /// Sets the tree guides.
    #[inline]
    #[must_use]
    pub fn guides(mut self, guides: TreeGuides) -> Self {
        self.guides = guides;
        self
    }

    /// Sets the guide style.
    #[inline]
    #[must_use]
    pub fn guide_style(mut self, style: Style) -> Self {
        self.guide_style = Some(style);
        self
    }

    /// Sets whether to hide the root node.
    #[inline]
    #[must_use]
    pub const fn hide_root(mut self, hide: bool) -> Self {
        self.hide_root = hide;
        self
    }

    /// Returns a mutable reference to the root node.
    #[inline]
    pub fn root_mut(&mut self) -> &mut TreeNode {
        &mut self.root
    }

    /// Adds a child to the root node.
    #[inline]
    pub fn add(&mut self, child: TreeNode) {
        self.root.add_child(child);
    }

    /// Renders the tree to segments.
    #[must_use]
    pub fn render(&self) -> Segments {
        let mut segments = Segments::new();

        if !self.hide_root {
            // Render root label
            let label_segments = self.root.label.to_segments();
            if let Some(ref style) = self.root.style {
                for seg in label_segments.iter() {
                    let combined = seg
                        .style
                        .clone()
                        .map(|s| s.combine(style))
                        .unwrap_or_else(|| style.clone());
                    segments.push(Segment::styled(seg.text.clone(), combined));
                }
            } else {
                segments.extend(label_segments);
            }
            segments.push(Segment::newline());
        }

        // Render children
        let child_count = self.root.children.len();
        for (idx, child) in self.root.children.iter().enumerate() {
            let is_last = idx == child_count.saturating_sub(1);
            self.render_node(&mut segments, child, "", is_last);
        }

        segments
    }

    /// Renders a node and its children.
    fn render_node(&self, segments: &mut Segments, node: &TreeNode, prefix: &str, is_last: bool) {
        let guide_style = node.guide_style.clone().or_else(|| self.guide_style.clone());

        // Render the branch guide
        let branch = if is_last {
            self.guides.last_branch
        } else {
            self.guides.branch
        };

        if let Some(ref style) = guide_style {
            segments.push(Segment::styled(prefix.to_owned(), style.clone()));
            segments.push(Segment::styled(branch.to_owned(), style.clone()));
        } else {
            segments.push(Segment::new(prefix));
            segments.push(Segment::new(branch));
        }

        // Space after guide
        segments.push(Segment::new(" "));

        // Render label
        let label_segments = node.label.to_segments();
        if let Some(ref style) = node.style {
            for seg in label_segments.iter() {
                let combined = seg
                    .style
                    .clone()
                    .map(|s| s.combine(style))
                    .unwrap_or_else(|| style.clone());
                segments.push(Segment::styled(seg.text.clone(), combined));
            }
        } else {
            segments.extend(label_segments);
        }
        segments.push(Segment::newline());

        // Render children if expanded
        if node.expanded {
            let new_prefix = if is_last {
                format!("{}{}", prefix, self.guides.space)
            } else {
                format!("{}{} ", prefix, self.guides.vertical)
            };

            let child_count = node.children.len();
            for (idx, child) in node.children.iter().enumerate() {
                let child_is_last = idx == child_count.saturating_sub(1);
                self.render_node(segments, child, &new_prefix, child_is_last);
            }
        }
    }
}

impl Measurable for Tree {
    fn measure(&self, options: &MeasureOptions) -> Result<Measurement> {
        // Calculate the maximum line width
        fn max_width(node: &TreeNode, depth: usize, guides: &TreeGuides) -> usize {
            let guide_width = if depth > 0 {
                depth.saturating_mul(cell_len(guides.branch).saturating_add(1))
            } else {
                0
            };
            let label_width = cell_len(node.label.plain());
            let this_width = guide_width.saturating_add(label_width);

            let child_max = node
                .children
                .iter()
                .map(|c| max_width(c, depth.saturating_add(1), guides))
                .max()
                .unwrap_or(0);

            this_width.max(child_max)
        }

        let width = max_width(&self.root, 0, &self.guides);
        Ok(Measurement::fixed(width).clamp_max(options.max_width))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_new() {
        let tree = Tree::new("Root");
        assert!(!tree.hide_root);
    }

    #[test]
    fn test_tree_add() {
        let mut tree = Tree::new("Root");
        tree.add(TreeNode::new("Child 1"));
        tree.add(TreeNode::new("Child 2"));
        assert_eq!(tree.root.children.len(), 2);
    }

    #[test]
    fn test_tree_render() {
        let mut tree = Tree::new("Root");
        tree.add(TreeNode::new("Child 1"));
        tree.add(TreeNode::new("Child 2"));

        let segments = tree.render();
        let text = segments.plain_text();
        assert!(text.contains("Root"));
        assert!(text.contains("Child 1"));
        assert!(text.contains("Child 2"));
    }

    #[test]
    fn test_tree_nested() {
        let mut tree = Tree::new("Root");
        let child = TreeNode::new("Child").with_child(TreeNode::new("Grandchild"));
        tree.add(child);

        let segments = tree.render();
        let text = segments.plain_text();
        assert!(text.contains("Grandchild"));
    }

    #[test]
    fn test_tree_guides() {
        let tree = Tree::new("Root").guides(TreeGuides::ASCII);
        assert_eq!(tree.guides.branch, "+--");
    }

    #[test]
    fn test_tree_node_builder() {
        let style = Style::new().bold();
        let node = TreeNode::new("Label")
            .style(style.clone())
            .expanded(false);
        assert!(node.style.is_some());
        assert!(!node.expanded);
    }

    #[test]
    fn test_tree_from_node() {
        let node = TreeNode::new("Root").with_child(TreeNode::new("Child"));
        let tree = Tree::from_node(node);
        assert_eq!(tree.root.children.len(), 1);
    }

    #[test]
    fn test_tree_measure() {
        let mut tree = Tree::new("Root");
        tree.add(TreeNode::new("Child"));

        let options = MeasureOptions::new(80);
        let measurement = tree.measure(&options).ok().unwrap_or_default();
        assert!(measurement.minimum > 0);
    }
}
