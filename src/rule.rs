//! Rule component for horizontal dividers.
//!
//! Rules draw horizontal lines across the terminal, optionally
//! with a centered title.

use crate::errors::Result;
use crate::measure::{Measurable, MeasureOptions, Measurement, cell_len};
use crate::segment::{Segment, Segments};
use crate::style::Style;
use crate::text::{Justify, Text};
use serde::{Deserialize, Serialize};

/// A horizontal rule/divider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    /// Optional title displayed in the rule.
    title: Option<Text>,
    /// Character used for the rule line.
    character: char,
    /// Style for the rule line.
    style: Option<Style>,
    /// Style for the title.
    title_style: Option<Style>,
    /// Title alignment.
    align: Justify,
    /// End character (optional).
    end: Option<String>,
}

impl Rule {
    /// Creates a new rule without a title.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: None,
            character: '─',
            style: None,
            title_style: None,
            align: Justify::Center,
            end: None,
        }
    }

    /// Creates a rule with a title.
    #[inline]
    #[must_use]
    pub fn with_title(title: impl Into<Text>) -> Self {
        Self {
            title: Some(title.into()),
            character: '─',
            style: None,
            title_style: None,
            align: Justify::Center,
            end: None,
        }
    }

    /// Sets the rule character.
    #[inline]
    #[must_use]
    pub const fn character(mut self, ch: char) -> Self {
        self.character = ch;
        self
    }

    /// Sets the line style.
    #[inline]
    #[must_use]
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    /// Sets the title style.
    #[inline]
    #[must_use]
    pub fn title_style(mut self, style: Style) -> Self {
        self.title_style = Some(style);
        self
    }

    /// Sets the title alignment.
    #[inline]
    #[must_use]
    pub const fn align(mut self, align: Justify) -> Self {
        self.align = align;
        self
    }

    /// Sets the end string.
    #[inline]
    #[must_use]
    pub fn end(mut self, end: impl Into<String>) -> Self {
        self.end = Some(end.into());
        self
    }

    /// Renders the rule to segments.
    #[must_use]
    pub fn render(&self, width: usize) -> Segments {
        let mut segments = Segments::new();

        match &self.title {
            Some(title) => {
                let title_text = title.plain();
                let title_width = cell_len(title_text);

                // Space needed for title with padding
                let padding_width: usize = 2; // Space on each side of title
                let min_line_width: usize = 1; // Minimum line on each side

                if title_width
                    .saturating_add(padding_width.saturating_mul(2))
                    .saturating_add(min_line_width.saturating_mul(2))
                    > width
                {
                    // Not enough space, just render the line
                    self.render_line(&mut segments, width);
                } else {
                    // Calculate line widths based on alignment
                    let available = width
                        .saturating_sub(title_width)
                        .saturating_sub(padding_width.saturating_mul(2));
                    let (left_width, right_width) = match self.align {
                        Justify::Left | Justify::Default => {
                            (min_line_width, available.saturating_sub(min_line_width))
                        }
                        Justify::Right => {
                            (available.saturating_sub(min_line_width), min_line_width)
                        }
                        Justify::Center | Justify::Full => {
                            let half = available.checked_div(2).unwrap_or(0);
                            (half, available.saturating_sub(half))
                        }
                    };

                    // Left line
                    self.render_line(&mut segments, left_width);

                    // Space before title
                    segments.push(Segment::new(" "));

                    // Title
                    let title_segments = title.to_segments();
                    if let Some(ref style) = self.title_style {
                        for seg in title_segments.iter() {
                            let combined = seg
                                .style
                                .clone()
                                .map(|s| s.combine(style))
                                .unwrap_or_else(|| style.clone());
                            segments.push(Segment::styled(seg.text.clone(), combined));
                        }
                    } else {
                        segments.extend(title_segments);
                    }

                    // Space after title
                    segments.push(Segment::new(" "));

                    // Right line
                    self.render_line(&mut segments, right_width);
                }
            }
            None => {
                self.render_line(&mut segments, width);
            }
        }

        // End string (typically newline)
        if let Some(ref end_str) = self.end {
            segments.push(Segment::new(end_str.clone()));
        } else {
            segments.push(Segment::newline());
        }

        segments
    }

    /// Renders a line segment.
    fn render_line(&self, segments: &mut Segments, width: usize) {
        let line = self.character.to_string().repeat(width);
        if let Some(ref style) = self.style {
            segments.push(Segment::styled(line, style.clone()));
        } else {
            segments.push(Segment::new(line));
        }
    }
}

impl Default for Rule {
    fn default() -> Self {
        Self::new()
    }
}

impl Measurable for Rule {
    fn measure(&self, options: &MeasureOptions) -> Result<Measurement> {
        // Rules always expand to fill available width
        Ok(Measurement::fixed(options.max_width))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_new() {
        let rule = Rule::new();
        assert!(rule.title.is_none());
        assert_eq!(rule.character, '─');
    }

    #[test]
    fn test_rule_with_title() {
        let rule = Rule::with_title("Test");
        assert!(rule.title.is_some());
    }

    #[test]
    fn test_rule_render() {
        let rule = Rule::new();
        let segments = rule.render(40);
        let text = segments.plain_text();
        assert_eq!(text.trim(), "─".repeat(40));
    }

    #[test]
    fn test_rule_render_with_title() {
        let rule = Rule::with_title("Title");
        let segments = rule.render(40);
        let text = segments.plain_text();
        assert!(text.contains("Title"));
        assert!(text.contains('─'));
    }

    #[test]
    fn test_rule_character() {
        let rule = Rule::new().character('=');
        let segments = rule.render(20);
        let text = segments.plain_text();
        assert!(text.contains('='));
    }

    #[test]
    fn test_rule_align_left() {
        let rule = Rule::with_title("Title").align(Justify::Left);
        let segments = rule.render(40);
        let text = segments.plain_text();
        assert!(text.contains("Title"));
    }

    #[test]
    fn test_rule_measure() {
        let rule = Rule::new();
        let options = MeasureOptions::new(80);
        let measurement = rule.measure(&options).ok().unwrap_or_default();
        assert_eq!(measurement.minimum, 80);
        assert_eq!(measurement.maximum, 80);
    }
}
