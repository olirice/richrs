//! Rich protocol for custom renderables.
//!
//! The protocol defines traits that custom types can implement
//! to be rendered by the console.

use crate::errors::Result;
use crate::measure::{MeasureOptions, Measurement};
use crate::segment::Segments;

/// Options for rendering.
#[derive(Debug, Clone)]
pub struct RenderOptions {
    /// Maximum width available.
    pub max_width: usize,
    /// Whether highlighting is enabled.
    pub highlight: bool,
    /// Whether markup is enabled.
    pub markup: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            max_width: 80,
            highlight: false,
            markup: true,
        }
    }
}

impl RenderOptions {
    /// Creates new render options.
    #[inline]
    #[must_use]
    pub const fn new(max_width: usize) -> Self {
        Self {
            max_width,
            highlight: false,
            markup: true,
        }
    }
}

/// Trait for types that can be rendered to the console.
///
/// Implementing this trait allows custom types to be printed
/// using the Console's print methods.
pub trait Renderable {
    /// Renders this object to segments.
    ///
    /// # Errors
    ///
    /// Returns an error if rendering fails.
    fn render(&self, options: &RenderOptions) -> Result<Segments>;

    /// Measures this object's dimensions.
    ///
    /// # Errors
    ///
    /// Returns an error if measurement fails.
    fn measure(&self, options: &MeasureOptions) -> Result<Measurement> {
        // Default implementation renders and measures the result
        let render_opts = RenderOptions::new(options.max_width);
        let segments = self.render(&render_opts)?;
        let width = segments.cell_length();
        Ok(Measurement::fixed(width).clamp_max(options.max_width))
    }
}

/// Trait for types that produce rich representations.
///
/// This is similar to Python's `__rich__` method.
pub trait RichRepr {
    /// The renderable type this produces.
    type Output: Renderable;

    /// Returns a rich representation of this object.
    fn rich_repr(&self) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::segment::Segment;

    #[test]
    fn test_render_options_default() {
        let opts = RenderOptions::default();
        assert_eq!(opts.max_width, 80);
        assert!(!opts.highlight);
        assert!(opts.markup);
    }

    #[test]
    fn test_render_options_new() {
        let opts = RenderOptions::new(120);
        assert_eq!(opts.max_width, 120);
        assert!(!opts.highlight);
        assert!(opts.markup);
    }

    #[test]
    fn test_render_options_fields() {
        let mut opts = RenderOptions::new(100);
        opts.highlight = true;
        opts.markup = false;
        assert_eq!(opts.max_width, 100);
        assert!(opts.highlight);
        assert!(!opts.markup);
    }

    // Test a simple Renderable implementation
    struct SimpleRenderable {
        text: String,
    }

    impl Renderable for SimpleRenderable {
        fn render(&self, _options: &RenderOptions) -> Result<Segments> {
            let mut segments = Segments::new();
            segments.push(Segment::new(&self.text));
            Ok(segments)
        }
    }

    #[test]
    fn test_renderable_render() {
        let r = SimpleRenderable {
            text: "Hello".to_string(),
        };
        let opts = RenderOptions::new(80);
        let result = r.render(&opts);
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_renderable_measure_default() {
        let r = SimpleRenderable {
            text: "Hello".to_string(),
        };
        let opts = MeasureOptions::new(80);
        let result = r.measure(&opts);
        assert!(result.is_ok());
        let measurement = result.unwrap();
        assert!(measurement.minimum > 0);
    }
}
