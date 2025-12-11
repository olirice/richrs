//! Panel component for boxed content display.
//!
//! Panels render content within a decorative border with optional
//! title and subtitle.

use crate::box_chars::BoxChars;
use crate::errors::Result;
use crate::measure::{Measurable, MeasureOptions, Measurement, cell_len};
use crate::segment::{Segment, Segments};
use crate::style::Style;
use crate::text::Text;
use serde::{Deserialize, Serialize};

/// A panel that renders content within a border.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Panel {
    /// The content to display.
    content: Text,
    /// Box drawing characters to use.
    box_chars: BoxChars,
    /// Optional title displayed at the top.
    title: Option<Text>,
    /// Optional subtitle displayed at the bottom.
    subtitle: Option<Text>,
    /// Style for the border.
    border_style: Option<Style>,
    /// Style for the title.
    title_style: Option<Style>,
    /// Style for the subtitle.
    subtitle_style: Option<Style>,
    /// Whether to expand to full width.
    expand: bool,
    /// Padding inside the panel (left, top, right, bottom).
    padding: (usize, usize, usize, usize),
    /// Optional fixed width.
    width: Option<usize>,
    /// Whether to render a safe ASCII box.
    safe_box: bool,
}

impl Panel {
    /// Creates a new panel with the given content.
    #[must_use]
    pub fn new(content: impl Into<Text>) -> Self {
        Self {
            content: content.into(),
            box_chars: BoxChars::ROUNDED,
            title: None,
            subtitle: None,
            border_style: None,
            title_style: None,
            subtitle_style: None,
            expand: true,
            padding: (1, 0, 1, 0),
            width: None,
            safe_box: false,
        }
    }

    /// Creates a panel that fits its content width.
    #[must_use]
    pub fn fit(content: impl Into<Text>) -> Self {
        Self {
            content: content.into(),
            box_chars: BoxChars::ROUNDED,
            title: None,
            subtitle: None,
            border_style: None,
            title_style: None,
            subtitle_style: None,
            expand: false,
            padding: (1, 0, 1, 0),
            width: None,
            safe_box: false,
        }
    }

    /// Sets the box drawing characters.
    #[inline]
    #[must_use]
    pub fn box_chars(mut self, box_chars: BoxChars) -> Self {
        self.box_chars = box_chars;
        self
    }

    /// Sets the title.
    #[inline]
    #[must_use]
    pub fn title(mut self, title: impl Into<Text>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the subtitle.
    #[inline]
    #[must_use]
    pub fn subtitle(mut self, subtitle: impl Into<Text>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Sets the border style.
    #[inline]
    #[must_use]
    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = Some(style);
        self
    }

    /// Sets the title style.
    #[inline]
    #[must_use]
    pub fn title_style(mut self, style: Style) -> Self {
        self.title_style = Some(style);
        self
    }

    /// Sets the subtitle style.
    #[inline]
    #[must_use]
    pub fn subtitle_style(mut self, style: Style) -> Self {
        self.subtitle_style = Some(style);
        self
    }

    /// Sets whether to expand to full width.
    #[inline]
    #[must_use]
    pub const fn expand(mut self, expand: bool) -> Self {
        self.expand = expand;
        self
    }

    /// Sets the padding.
    #[inline]
    #[must_use]
    pub const fn padding(mut self, left: usize, top: usize, right: usize, bottom: usize) -> Self {
        self.padding = (left, top, right, bottom);
        self
    }

    /// Sets the width.
    #[inline]
    #[must_use]
    pub const fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets whether to use ASCII-safe box characters.
    #[inline]
    #[must_use]
    pub fn safe_box(mut self, safe: bool) -> Self {
        self.safe_box = safe;
        if safe {
            self.box_chars = BoxChars::ASCII;
        }
        self
    }

    /// Renders the panel to segments.
    #[must_use]
    pub fn render(&self, max_width: usize) -> Segments {
        let mut segments = Segments::new();

        // Calculate dimensions
        let content_width = self.calculate_content_width(max_width);
        let total_width = content_width
            .saturating_add(2)
            .saturating_add(self.padding.0)
            .saturating_add(self.padding.2);

        // Render top border with title
        self.render_top_border(&mut segments, total_width);

        // Render top padding
        for _ in 0..self.padding.1 {
            self.render_empty_line(&mut segments, total_width);
        }

        // Render content lines
        self.render_content(&mut segments, content_width, total_width);

        // Render bottom padding
        for _ in 0..self.padding.3 {
            self.render_empty_line(&mut segments, total_width);
        }

        // Render bottom border with subtitle
        self.render_bottom_border(&mut segments, total_width);

        segments
    }

    /// Calculates the content width.
    fn calculate_content_width(&self, max_width: usize) -> usize {
        if let Some(w) = self.width {
            return w
                .saturating_sub(2)
                .saturating_sub(self.padding.0)
                .saturating_sub(self.padding.2);
        }

        if self.expand {
            max_width
                .saturating_sub(2)
                .saturating_sub(self.padding.0)
                .saturating_sub(self.padding.2)
        } else {
            let content_width = cell_len(self.content.plain());
            let title_width = self
                .title
                .as_ref()
                .map(|t| cell_len(t.plain()))
                .unwrap_or(0);
            let subtitle_width = self
                .subtitle
                .as_ref()
                .map(|t| cell_len(t.plain()))
                .unwrap_or(0);

            let needed = content_width.max(title_width).max(subtitle_width);
            needed.min(
                max_width
                    .saturating_sub(2)
                    .saturating_sub(self.padding.0)
                    .saturating_sub(self.padding.2),
            )
        }
    }

    /// Renders the top border.
    fn render_top_border(&self, segments: &mut Segments, width: usize) {
        let inner_width = width.saturating_sub(2);

        // Start with left corner
        let left_corner = self.box_chars.top_left.to_string();
        if let Some(ref style) = self.border_style {
            segments.push(Segment::styled(left_corner, style.clone()));
        } else {
            segments.push(Segment::new(left_corner));
        }

        // Render title or horizontal line
        if let Some(ref title) = self.title {
            let title_text = title.plain();
            let title_len = cell_len(title_text);

            if title_len.saturating_add(4) <= inner_width {
                // Space for title with padding
                let left_line_len = 1;
                let right_line_len = inner_width
                    .saturating_sub(title_len)
                    .saturating_sub(left_line_len)
                    .saturating_sub(2);

                // Left line
                let left_line = self.box_chars.horizontal.to_string().repeat(left_line_len);
                if let Some(ref style) = self.border_style {
                    segments.push(Segment::styled(left_line, style.clone()));
                } else {
                    segments.push(Segment::new(left_line));
                }

                // Space before title
                segments.push(Segment::new(" "));

                // Title
                let title_segments = title.to_segments();
                if let Some(ref style) = self.title_style {
                    for seg in title_segments.iter() {
                        let combined_style = seg
                            .style
                            .clone()
                            .map(|s| s.combine(style))
                            .unwrap_or_else(|| style.clone());
                        segments.push(Segment::styled(seg.text.clone(), combined_style));
                    }
                } else {
                    segments.extend(title_segments);
                }

                // Space after title
                segments.push(Segment::new(" "));

                // Right line
                let right_line = self.box_chars.horizontal.to_string().repeat(right_line_len);
                if let Some(ref style) = self.border_style {
                    segments.push(Segment::styled(right_line, style.clone()));
                } else {
                    segments.push(Segment::new(right_line));
                }
            } else {
                // No room for title, just horizontal line
                let line = self.box_chars.horizontal.to_string().repeat(inner_width);
                if let Some(ref style) = self.border_style {
                    segments.push(Segment::styled(line, style.clone()));
                } else {
                    segments.push(Segment::new(line));
                }
            }
        } else {
            let line = self.box_chars.horizontal.to_string().repeat(inner_width);
            if let Some(ref style) = self.border_style {
                segments.push(Segment::styled(line, style.clone()));
            } else {
                segments.push(Segment::new(line));
            }
        }

        // Right corner
        let right_corner = self.box_chars.top_right.to_string();
        if let Some(ref style) = self.border_style {
            segments.push(Segment::styled(right_corner, style.clone()));
        } else {
            segments.push(Segment::new(right_corner));
        }

        segments.push(Segment::newline());
    }

    /// Renders the bottom border.
    fn render_bottom_border(&self, segments: &mut Segments, width: usize) {
        let inner_width = width.saturating_sub(2);

        // Left corner
        let left_corner = self.box_chars.bottom_left.to_string();
        if let Some(ref style) = self.border_style {
            segments.push(Segment::styled(left_corner, style.clone()));
        } else {
            segments.push(Segment::new(left_corner));
        }

        // Subtitle or horizontal line
        if let Some(ref subtitle) = self.subtitle {
            let subtitle_text = subtitle.plain();
            let subtitle_len = cell_len(subtitle_text);

            if subtitle_len.saturating_add(4) <= inner_width {
                let left_line_len = 1;
                let right_line_len = inner_width
                    .saturating_sub(subtitle_len)
                    .saturating_sub(left_line_len)
                    .saturating_sub(2);

                // Left line
                let left_line = self.box_chars.horizontal.to_string().repeat(left_line_len);
                if let Some(ref style) = self.border_style {
                    segments.push(Segment::styled(left_line, style.clone()));
                } else {
                    segments.push(Segment::new(left_line));
                }

                // Space before subtitle
                segments.push(Segment::new(" "));

                // Subtitle
                let subtitle_segments = subtitle.to_segments();
                if let Some(ref style) = self.subtitle_style {
                    for seg in subtitle_segments.iter() {
                        let combined_style = seg
                            .style
                            .clone()
                            .map(|s| s.combine(style))
                            .unwrap_or_else(|| style.clone());
                        segments.push(Segment::styled(seg.text.clone(), combined_style));
                    }
                } else {
                    segments.extend(subtitle_segments);
                }

                // Space after subtitle
                segments.push(Segment::new(" "));

                // Right line
                let right_line = self.box_chars.horizontal.to_string().repeat(right_line_len);
                if let Some(ref style) = self.border_style {
                    segments.push(Segment::styled(right_line, style.clone()));
                } else {
                    segments.push(Segment::new(right_line));
                }
            } else {
                let line = self.box_chars.horizontal.to_string().repeat(inner_width);
                if let Some(ref style) = self.border_style {
                    segments.push(Segment::styled(line, style.clone()));
                } else {
                    segments.push(Segment::new(line));
                }
            }
        } else {
            let line = self.box_chars.horizontal.to_string().repeat(inner_width);
            if let Some(ref style) = self.border_style {
                segments.push(Segment::styled(line, style.clone()));
            } else {
                segments.push(Segment::new(line));
            }
        }

        // Right corner
        let right_corner = self.box_chars.bottom_right.to_string();
        if let Some(ref style) = self.border_style {
            segments.push(Segment::styled(right_corner, style.clone()));
        } else {
            segments.push(Segment::new(right_corner));
        }

        segments.push(Segment::newline());
    }

    /// Renders an empty content line.
    fn render_empty_line(&self, segments: &mut Segments, width: usize) {
        let inner_width = width.saturating_sub(2);

        let vertical = self.box_chars.vertical.to_string();
        if let Some(ref style) = self.border_style {
            segments.push(Segment::styled(vertical.clone(), style.clone()));
        } else {
            segments.push(Segment::new(vertical.clone()));
        }

        segments.push(Segment::new(" ".repeat(inner_width)));

        if let Some(ref style) = self.border_style {
            segments.push(Segment::styled(vertical, style.clone()));
        } else {
            segments.push(Segment::new(vertical));
        }

        segments.push(Segment::newline());
    }

    /// Renders the content lines.
    fn render_content(&self, segments: &mut Segments, content_width: usize, _total_width: usize) {
        let lines = self.content.split_lines();

        for line in &lines {
            // Left border
            let vertical = self.box_chars.vertical.to_string();
            if let Some(ref style) = self.border_style {
                segments.push(Segment::styled(vertical.clone(), style.clone()));
            } else {
                segments.push(Segment::new(vertical.clone()));
            }

            // Left padding
            segments.push(Segment::new(" ".repeat(self.padding.0)));

            // Content
            let line_segments = line.to_segments();
            let line_width = line_segments.cell_length();

            for seg in line_segments.iter() {
                segments.push(seg.clone());
            }

            // Right padding to fill width
            let remaining = content_width.saturating_sub(line_width);
            if remaining > 0 {
                segments.push(Segment::new(" ".repeat(remaining)));
            }

            // Right padding
            segments.push(Segment::new(" ".repeat(self.padding.2)));

            // Right border
            if let Some(ref style) = self.border_style {
                segments.push(Segment::styled(vertical, style.clone()));
            } else {
                segments.push(Segment::new(vertical));
            }

            segments.push(Segment::newline());
        }
    }
}

impl From<&str> for Panel {
    #[inline]
    fn from(s: &str) -> Self {
        Self::new(Text::from_str(s))
    }
}

impl From<String> for Panel {
    #[inline]
    fn from(s: String) -> Self {
        Self::new(Text::from_str(s))
    }
}

impl Measurable for Panel {
    fn measure(&self, options: &MeasureOptions) -> Result<Measurement> {
        let content_measurement = self.content.measure(options)?;

        let title_width = self
            .title
            .as_ref()
            .map(|t| cell_len(t.plain()))
            .unwrap_or(0);
        let subtitle_width = self
            .subtitle
            .as_ref()
            .map(|t| cell_len(t.plain()))
            .unwrap_or(0);

        let border_overhead = 2usize
            .saturating_add(self.padding.0)
            .saturating_add(self.padding.2);

        let min_content = content_measurement
            .minimum
            .max(title_width)
            .max(subtitle_width);
        let max_content = content_measurement
            .maximum
            .max(title_width)
            .max(subtitle_width);

        Ok(Measurement::new(
            min_content.saturating_add(border_overhead),
            max_content.saturating_add(border_overhead),
        )
        .clamp_max(options.max_width))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{Color, StandardColor};

    #[test]
    fn test_panel_new() {
        let panel = Panel::new("Hello, World!");
        assert!(panel.expand);
        assert!(panel.title.is_none());
        assert!(panel.subtitle.is_none());
        assert!(panel.border_style.is_none());
        assert!(panel.title_style.is_none());
        assert!(panel.subtitle_style.is_none());
    }

    #[test]
    fn test_panel_fit() {
        let panel = Panel::fit("Hello");
        assert!(!panel.expand);
    }

    #[test]
    fn test_panel_expand() {
        let panel = Panel::new("Hello").expand(false);
        assert!(!panel.expand);

        let panel = Panel::fit("Hello").expand(true);
        assert!(panel.expand);
    }

    #[test]
    fn test_panel_with_title() {
        let panel = Panel::new("Content").title("Title");
        assert!(panel.title.is_some());
    }

    #[test]
    fn test_panel_with_subtitle() {
        let panel = Panel::new("Content").subtitle("Subtitle");
        assert!(panel.subtitle.is_some());
    }

    #[test]
    fn test_panel_with_title_and_subtitle() {
        let panel = Panel::new("Content").title("Title").subtitle("Subtitle");
        assert!(panel.title.is_some());
        assert!(panel.subtitle.is_some());
    }

    #[test]
    fn test_panel_border_style() {
        let style = Style::new().with_color(Color::Standard(StandardColor::Red));
        let panel = Panel::new("Content").border_style(style);
        assert!(panel.border_style.is_some());
    }

    #[test]
    fn test_panel_title_style() {
        let style = Style::new().bold();
        let panel = Panel::new("Content").title("Title").title_style(style);
        assert!(panel.title_style.is_some());
    }

    #[test]
    fn test_panel_subtitle_style() {
        let style = Style::new().italic();
        let panel = Panel::new("Content")
            .subtitle("Subtitle")
            .subtitle_style(style);
        assert!(panel.subtitle_style.is_some());
    }

    #[test]
    fn test_panel_padding() {
        let panel = Panel::new("Content").padding(2, 2, 2, 2);
        assert_eq!(panel.padding, (2, 2, 2, 2));
    }

    #[test]
    fn test_panel_render() {
        let panel = Panel::new("Hello");
        let segments = panel.render(40);
        assert!(!segments.is_empty());

        let text = segments.plain_text();
        assert!(text.contains("Hello"));
    }

    #[test]
    fn test_panel_render_with_title() {
        let panel = Panel::new("Content").title("Title");
        let segments = panel.render(40);
        let text = segments.plain_text();
        assert!(text.contains("Title"));
        assert!(text.contains("Content"));
    }

    #[test]
    fn test_panel_render_with_subtitle() {
        let panel = Panel::new("Content").subtitle("Subtitle");
        let segments = panel.render(40);
        let text = segments.plain_text();
        assert!(text.contains("Subtitle"));
        assert!(text.contains("Content"));
    }

    #[test]
    fn test_panel_render_with_title_and_subtitle() {
        let panel = Panel::new("Content").title("Title").subtitle("Subtitle");
        let segments = panel.render(40);
        let text = segments.plain_text();
        assert!(text.contains("Title"));
        assert!(text.contains("Subtitle"));
        assert!(text.contains("Content"));
    }

    #[test]
    fn test_panel_render_with_styled_title() {
        let panel = Panel::new("Content")
            .title("Title")
            .title_style(Style::new().bold());
        let segments = panel.render(40);
        let text = segments.plain_text();
        assert!(text.contains("Title"));
    }

    #[test]
    fn test_panel_render_with_styled_subtitle() {
        let panel = Panel::new("Content")
            .subtitle("Subtitle")
            .subtitle_style(Style::new().italic());
        let segments = panel.render(40);
        let text = segments.plain_text();
        assert!(text.contains("Subtitle"));
    }

    #[test]
    fn test_panel_from_str() {
        let panel: Panel = "Test".into();
        let segments = panel.render(40);
        let text = segments.plain_text();
        assert!(text.contains("Test"));
    }

    #[test]
    fn test_panel_from_string() {
        let panel: Panel = String::from("Test").into();
        let segments = panel.render(40);
        let text = segments.plain_text();
        assert!(text.contains("Test"));
    }

    #[test]
    fn test_panel_box_chars() {
        let panel = Panel::new("Test").box_chars(BoxChars::ASCII);
        let segments = panel.render(40);
        let text = segments.plain_text();
        assert!(text.contains('+'));
        assert!(text.contains('-'));
    }

    #[test]
    fn test_panel_safe_box() {
        let panel = Panel::new("Test").safe_box(true);
        let segments = panel.render(40);
        let text = segments.plain_text();
        assert!(text.contains('+'));
    }

    #[test]
    fn test_panel_safe_box_false() {
        let panel = Panel::new("Test").safe_box(false);
        let segments = panel.render(40);
        let text = segments.plain_text();
        // Should use unicode box chars
        assert!(text.contains('─') || text.contains('│'));
    }

    #[test]
    fn test_panel_measure() {
        let panel = Panel::new("Hello");
        let options = MeasureOptions::new(80);
        let measurement = panel.measure(&options).ok().unwrap_or_default();
        assert!(measurement.minimum > 0);
    }

    #[test]
    fn test_panel_measure_with_title() {
        let panel = Panel::new("Hi").title("A Very Long Title Here");
        let options = MeasureOptions::new(80);
        let measurement = panel.measure(&options).ok().unwrap_or_default();
        // Title should increase minimum width
        assert!(measurement.minimum > 5);
    }

    #[test]
    fn test_panel_measure_with_subtitle() {
        let panel = Panel::new("Hi").subtitle("A Very Long Subtitle");
        let options = MeasureOptions::new(80);
        let measurement = panel.measure(&options).ok().unwrap_or_default();
        assert!(measurement.minimum > 5);
    }

    #[test]
    fn test_panel_render_narrow() {
        let panel = Panel::new("A");
        let segments = panel.render(10);
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_panel_render_wide() {
        let panel = Panel::new("Test").expand(true);
        let segments = panel.render(100);
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_panel_render_with_padding() {
        let panel = Panel::new("Content").padding(2, 3, 2, 3);
        let segments = panel.render(40);
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_panel_render_with_border_style() {
        let style = Style::new().with_color(Color::Standard(StandardColor::Blue));
        let panel = Panel::new("Content").border_style(style);
        let segments = panel.render(40);
        // Should have styled border segments
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_panel_multiline_content() {
        let panel = Panel::new("Line 1\nLine 2\nLine 3");
        let segments = panel.render(40);
        let text = segments.plain_text();
        assert!(text.contains("Line 1"));
        assert!(text.contains("Line 2"));
        assert!(text.contains("Line 3"));
    }

    #[test]
    fn test_panel_empty_content() {
        let panel = Panel::new("");
        let segments = panel.render(40);
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_panel_fit_render() {
        let panel = Panel::fit("Short");
        let segments = panel.render(100);
        // Fit panel should not expand to full width
        assert!(!segments.is_empty());
    }
}
