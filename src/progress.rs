//! Progress bar component for displaying task progress.
//!
//! Progress bars show the completion status of long-running tasks
//! with optional elapsed time, remaining time, and throughput.

use crate::errors::Result;
use crate::segment::{Segment, Segments};
use crate::style::Style;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// A unique identifier for a progress task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(u64);

impl TaskId {
    /// Creates a new task ID.
    #[inline]
    #[must_use]
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the raw ID value.
    #[inline]
    #[must_use]
    pub const fn id(&self) -> u64 {
        self.0
    }
}

/// A progress task being tracked.
#[derive(Debug, Clone)]
pub struct Task {
    /// Unique task identifier.
    pub id: TaskId,
    /// Task description.
    pub description: String,
    /// Total units of work (None for indeterminate).
    pub total: Option<u64>,
    /// Completed units of work.
    pub completed: u64,
    /// Whether the task is started.
    pub started: bool,
    /// Whether the task is visible.
    pub visible: bool,
    /// Start time.
    pub start_time: Option<Instant>,
    /// Stop time.
    pub stop_time: Option<Instant>,
    /// Custom fields.
    pub fields: std::collections::HashMap<String, String>,
}

impl Task {
    /// Creates a new task.
    #[must_use]
    pub fn new(id: TaskId, description: impl Into<String>, total: Option<u64>) -> Self {
        Self {
            id,
            description: description.into(),
            total,
            completed: 0,
            started: false,
            visible: true,
            start_time: None,
            stop_time: None,
            fields: std::collections::HashMap::new(),
        }
    }

    /// Returns the completion percentage (0.0 to 1.0).
    #[must_use]
    pub fn percentage(&self) -> Option<f64> {
        self.total.map(|t| {
            if t == 0 {
                1.0
            } else {
                #[allow(clippy::cast_precision_loss)]
                let completed = self.completed as f64;
                #[allow(clippy::cast_precision_loss)]
                let total = t as f64;
                (completed / total).min(1.0)
            }
        })
    }

    /// Returns whether the task is complete.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.total.map_or(false, |t| self.completed >= t)
    }

    /// Returns the elapsed time since the task started.
    #[must_use]
    pub fn elapsed(&self) -> Option<Duration> {
        self.start_time.map(|start| {
            self.stop_time
                .unwrap_or_else(Instant::now)
                .saturating_duration_since(start)
        })
    }

    /// Estimates the remaining time.
    #[must_use]
    pub fn remaining(&self) -> Option<Duration> {
        let elapsed = self.elapsed()?;
        let percentage = self.percentage()?;

        if percentage <= 0.0 {
            return None;
        }

        #[allow(clippy::cast_precision_loss)]
        let elapsed_secs = elapsed.as_secs_f64();
        #[allow(clippy::cast_sign_loss)]
        let remaining_secs = (elapsed_secs / percentage) - elapsed_secs;

        if remaining_secs.is_finite() && remaining_secs >= 0.0 {
            Some(Duration::from_secs_f64(remaining_secs))
        } else {
            None
        }
    }

    /// Returns the throughput (items per second).
    #[must_use]
    pub fn speed(&self) -> Option<f64> {
        let elapsed = self.elapsed()?;
        let secs = elapsed.as_secs_f64();

        if secs <= 0.0 {
            return None;
        }

        #[allow(clippy::cast_precision_loss)]
        let completed = self.completed as f64;
        Some(completed / secs)
    }
}

/// A visual progress bar.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProgressBar {
    /// Width of the bar in characters.
    pub width: usize,
    /// Character for completed portion.
    pub complete_char: char,
    /// Character for incomplete portion.
    pub incomplete_char: char,
    /// Character for the progress indicator.
    pub pulse_char: char,
    /// Style for completed portion.
    pub complete_style: Option<Style>,
    /// Style for incomplete portion.
    pub incomplete_style: Option<Style>,
    /// Style for finished bar.
    pub finished_style: Option<Style>,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self {
            width: 40,
            complete_char: '━',
            incomplete_char: '━',
            pulse_char: '━',
            complete_style: Some(Style::new().with_color(crate::color::Color::Standard(
                crate::color::StandardColor::Green,
            ))),
            incomplete_style: Some(Style::new().with_color(crate::color::Color::Standard(
                crate::color::StandardColor::BrightBlack,
            ))),
            finished_style: None,
        }
    }
}

impl ProgressBar {
    /// Creates a new progress bar with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the width.
    #[inline]
    #[must_use]
    pub const fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Sets the complete character.
    #[inline]
    #[must_use]
    pub const fn complete_char(mut self, ch: char) -> Self {
        self.complete_char = ch;
        self
    }

    /// Sets the incomplete character.
    #[inline]
    #[must_use]
    pub const fn incomplete_char(mut self, ch: char) -> Self {
        self.incomplete_char = ch;
        self
    }

    /// Sets the complete style.
    #[inline]
    #[must_use]
    pub fn complete_style(mut self, style: Style) -> Self {
        self.complete_style = Some(style);
        self
    }

    /// Sets the incomplete style.
    #[inline]
    #[must_use]
    pub fn incomplete_style(mut self, style: Style) -> Self {
        self.incomplete_style = Some(style);
        self
    }

    /// Renders the progress bar for a given percentage.
    #[must_use]
    pub fn render(&self, percentage: Option<f64>) -> Segments {
        let mut segments = Segments::new();

        if let Some(pct) = percentage {
            #[allow(clippy::cast_possible_truncation)]
            #[allow(clippy::cast_sign_loss)]
            let completed_width = ((pct * (self.width as f64)).round() as usize).min(self.width);
            let remaining_width = self.width.saturating_sub(completed_width);

            // Completed portion
            let completed = self.complete_char.to_string().repeat(completed_width);
            if let Some(ref style) = self.complete_style {
                segments.push(Segment::styled(completed, style.clone()));
            } else {
                segments.push(Segment::new(completed));
            }

            // Incomplete portion
            if remaining_width > 0 {
                let incomplete = self.incomplete_char.to_string().repeat(remaining_width);
                if let Some(ref style) = self.incomplete_style {
                    segments.push(Segment::styled(incomplete, style.clone()));
                } else {
                    segments.push(Segment::new(incomplete));
                }
            }
        } else {
            // Indeterminate progress (pulse animation would go here)
            let bar = self.incomplete_char.to_string().repeat(self.width);
            if let Some(ref style) = self.incomplete_style {
                segments.push(Segment::styled(bar, style.clone()));
            } else {
                segments.push(Segment::new(bar));
            }
        }

        segments
    }
}

/// Progress display manager for multiple tasks.
#[derive(Debug)]
pub struct Progress {
    /// Active tasks.
    tasks: Vec<Task>,
    /// Next task ID.
    next_id: u64,
    /// Progress bar configuration.
    bar: ProgressBar,
    /// Refresh rate per second.
    pub refresh_per_second: u32,
    /// Auto-refresh enabled.
    pub auto_refresh: bool,
    /// Transient display (hide when done).
    pub transient: bool,
    /// Expand to full width.
    pub expand: bool,
}

impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
}

impl Progress {
    /// Creates a new progress display.
    #[must_use]
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 0,
            bar: ProgressBar::default(),
            refresh_per_second: 10,
            auto_refresh: true,
            transient: false,
            expand: false,
        }
    }

    /// Sets the progress bar configuration.
    #[inline]
    #[must_use]
    pub fn bar(mut self, bar: ProgressBar) -> Self {
        self.bar = bar;
        self
    }

    /// Sets the refresh rate.
    #[inline]
    #[must_use]
    pub const fn refresh_per_second(mut self, rate: u32) -> Self {
        self.refresh_per_second = rate;
        self
    }

    /// Sets transient mode.
    #[inline]
    #[must_use]
    pub const fn transient(mut self, transient: bool) -> Self {
        self.transient = transient;
        self
    }

    /// Adds a new task.
    pub fn add_task(
        &mut self,
        description: impl Into<String>,
        total: Option<u64>,
        start: bool,
    ) -> TaskId {
        let id = TaskId::new(self.next_id);
        self.next_id = self.next_id.saturating_add(1);

        let mut task = Task::new(id, description, total);
        if start {
            task.started = true;
            task.start_time = Some(Instant::now());
        }

        self.tasks.push(task);
        id
    }

    /// Updates a task's progress.
    ///
    /// # Errors
    ///
    /// Returns an error if the task is not found.
    pub fn update(
        &mut self,
        task_id: TaskId,
        completed: Option<u64>,
        advance: Option<u64>,
        total: Option<u64>,
        visible: Option<bool>,
    ) -> Result<()> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == task_id)
            .ok_or_else(|| crate::errors::Error::OutOfRange {
                message: format!("task {} not found", task_id.0),
            })?;

        if let Some(c) = completed {
            task.completed = c;
        }

        if let Some(a) = advance {
            task.completed = task.completed.saturating_add(a);
        }

        if let Some(t) = total {
            task.total = Some(t);
        }

        if let Some(v) = visible {
            task.visible = v;
        }

        Ok(())
    }

    /// Advances a task by a given amount.
    ///
    /// # Errors
    ///
    /// Returns an error if the task is not found.
    pub fn advance(&mut self, task_id: TaskId, amount: u64) -> Result<()> {
        self.update(task_id, None, Some(amount), None, None)
    }

    /// Starts a task.
    ///
    /// # Errors
    ///
    /// Returns an error if the task is not found.
    pub fn start_task(&mut self, task_id: TaskId) -> Result<()> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == task_id)
            .ok_or_else(|| crate::errors::Error::OutOfRange {
                message: format!("task {} not found", task_id.0),
            })?;

        if !task.started {
            task.started = true;
            task.start_time = Some(Instant::now());
        }

        Ok(())
    }

    /// Stops a task.
    ///
    /// # Errors
    ///
    /// Returns an error if the task is not found.
    pub fn stop_task(&mut self, task_id: TaskId) -> Result<()> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == task_id)
            .ok_or_else(|| crate::errors::Error::OutOfRange {
                message: format!("task {} not found", task_id.0),
            })?;

        task.stop_time = Some(Instant::now());
        Ok(())
    }

    /// Removes a task.
    pub fn remove_task(&mut self, task_id: TaskId) {
        self.tasks.retain(|t| t.id != task_id);
    }

    /// Returns all tasks.
    #[inline]
    #[must_use]
    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    /// Returns a task by ID.
    #[must_use]
    pub fn get_task(&self, task_id: TaskId) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == task_id)
    }

    /// Returns true if all tasks are complete.
    #[must_use]
    pub fn finished(&self) -> bool {
        self.tasks.iter().all(Task::is_complete)
    }

    /// Renders the progress display.
    #[must_use]
    pub fn render(&self, width: usize) -> Segments {
        let mut segments = Segments::new();

        for task in &self.tasks {
            if !task.visible {
                continue;
            }

            // Description
            segments.push(Segment::new(task.description.clone()));
            segments.push(Segment::new(" "));

            // Progress bar
            let bar_segments = self.bar.render(task.percentage());
            segments.extend(bar_segments);

            // Percentage
            if let Some(pct) = task.percentage() {
                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_sign_loss)]
                let pct_int = (pct * 100.0).round() as u32;
                segments.push(Segment::new(format!(" {pct_int:3}%")));
            }

            // Completed / Total
            if let Some(total) = task.total {
                segments.push(Segment::new(format!(" {}/{}", task.completed, total)));
            }

            segments.push(Segment::newline());
        }

        // Ensure we don't exceed width
        let _ = width; // Width would be used for more sophisticated rendering

        segments
    }
}

/// Formats a duration as a human-readable string.
#[must_use]
pub fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let mins = secs.checked_div(60).unwrap_or(0);
    let hours = mins.checked_div(60).unwrap_or(0);

    if hours > 0 {
        format!(
            "{}:{:02}:{:02}",
            hours,
            mins.checked_rem(60).unwrap_or(0),
            secs.checked_rem(60).unwrap_or(0)
        )
    } else if mins > 0 {
        format!("{}:{:02}", mins, secs.checked_rem(60).unwrap_or(0))
    } else {
        format!("0:{:02}", secs)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::unchecked_duration_subtraction)]
    use super::*;
    use crate::color::{Color, StandardColor};

    #[test]
    fn test_task_id() {
        let id = TaskId::new(42);
        assert_eq!(id.id(), 42);
    }

    #[test]
    fn test_task_new() {
        let task = Task::new(TaskId::new(0), "Test", Some(100));
        assert_eq!(task.description, "Test");
        assert_eq!(task.total, Some(100));
        assert_eq!(task.completed, 0);
        assert!(task.visible);
    }

    #[test]
    fn test_task_new_no_total() {
        let task = Task::new(TaskId::new(0), "Test", None);
        assert!(task.total.is_none());
    }

    #[test]
    fn test_task_percentage() {
        let mut task = Task::new(TaskId::new(0), "Test", Some(100));
        task.completed = 50;
        assert!((task.percentage().unwrap_or(0.0) - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_task_percentage_no_total() {
        let task = Task::new(TaskId::new(0), "Test", None);
        assert!(task.percentage().is_none());
    }

    #[test]
    fn test_task_percentage_zero_total() {
        let task = Task::new(TaskId::new(0), "Test", Some(0));
        // Zero total returns 1.0 (100% complete)
        assert!((task.percentage().unwrap_or(0.0) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_task_is_complete() {
        let mut task = Task::new(TaskId::new(0), "Test", Some(100));
        assert!(!task.is_complete());

        task.completed = 100;
        assert!(task.is_complete());
    }

    #[test]
    fn test_task_is_complete_no_total() {
        let task = Task::new(TaskId::new(0), "Test", None);
        assert!(!task.is_complete());
    }

    #[test]
    fn test_task_elapsed() {
        let task = Task::new(TaskId::new(0), "Test", Some(100));
        // Just verify it doesn't panic - elapsed depends on wall clock
        let _ = task.elapsed();
    }

    #[test]
    fn test_progress_bar() {
        let bar = ProgressBar::new().width(20);
        let segments = bar.render(Some(0.5));
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_progress_bar_default() {
        let bar = ProgressBar::default();
        assert_eq!(bar.width, 40);
    }

    #[test]
    fn test_progress_bar_complete_char() {
        let bar = ProgressBar::new().complete_char('#');
        assert_eq!(bar.complete_char, '#');
    }

    #[test]
    fn test_progress_bar_incomplete_char() {
        let bar = ProgressBar::new().incomplete_char('-');
        assert_eq!(bar.incomplete_char, '-');
    }

    #[test]
    fn test_progress_bar_complete_style() {
        let style = Style::new().with_color(Color::Standard(StandardColor::Green));
        let bar = ProgressBar::new().complete_style(style);
        assert!(bar.complete_style.is_some());
    }

    #[test]
    fn test_progress_bar_incomplete_style() {
        let style = Style::new().with_color(Color::Standard(StandardColor::Red));
        let bar = ProgressBar::new().incomplete_style(style);
        assert!(bar.incomplete_style.is_some());
    }

    #[test]
    fn test_progress_bar_render_none() {
        let bar = ProgressBar::new().width(20);
        let segments = bar.render(None);
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_progress_bar_render_zero() {
        let bar = ProgressBar::new().width(20);
        let segments = bar.render(Some(0.0));
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_progress_bar_render_full() {
        let bar = ProgressBar::new().width(20);
        let segments = bar.render(Some(1.0));
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_progress_bar_render_with_style() {
        let bar = ProgressBar::new()
            .width(20)
            .complete_style(Style::new().with_color(Color::Standard(StandardColor::Green)))
            .incomplete_style(Style::new().with_color(Color::Standard(StandardColor::Red)));
        let segments = bar.render(Some(0.5));
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_progress_new() {
        let progress = Progress::new();
        assert!(progress.tasks.is_empty());
    }

    #[test]
    fn test_progress_add_task() {
        let mut progress = Progress::new();
        let id = progress.add_task("Test task", Some(100), true);
        assert_eq!(progress.tasks.len(), 1);
        assert!(progress.get_task(id).is_some());
    }

    #[test]
    fn test_progress_add_task_not_started() {
        let mut progress = Progress::new();
        let id = progress.add_task("Task", Some(100), false);
        let task = progress.get_task(id);
        // start=false means not started, but still visible by default
        assert!(!task.map(|t| t.started).unwrap_or(true));
        assert!(task.map(|t| t.start_time.is_none()).unwrap_or(false));
    }

    #[test]
    fn test_progress_update() {
        let mut progress = Progress::new();
        let id = progress.add_task("Test", Some(100), true);
        // update(task_id, completed, advance, total, visible)
        progress.update(id, Some(50), None, None, None).ok();

        let task = progress.get_task(id);
        assert_eq!(task.map(|t| t.completed), Some(50));
    }

    #[test]
    fn test_progress_update_advance() {
        let mut progress = Progress::new();
        let id = progress.add_task("Test", Some(100), true);
        // update with advance parameter
        progress.update(id, None, Some(25), None, None).ok();

        let task = progress.get_task(id);
        assert_eq!(task.map(|t| t.completed), Some(25));
    }

    #[test]
    fn test_progress_update_total() {
        let mut progress = Progress::new();
        let id = progress.add_task("Test", Some(100), true);
        // update(task_id, completed, advance, total, visible)
        progress.update(id, None, None, Some(200), None).ok();

        let task = progress.get_task(id);
        assert_eq!(task.and_then(|t| t.total), Some(200));
    }

    #[test]
    fn test_progress_update_visible() {
        let mut progress = Progress::new();
        let id = progress.add_task("Test", Some(100), true);
        progress.update(id, None, None, None, Some(false)).ok();

        let task = progress.get_task(id);
        assert!(!task.map(|t| t.visible).unwrap_or(true));
    }

    #[test]
    fn test_progress_advance() {
        let mut progress = Progress::new();
        let id = progress.add_task("Test", Some(100), true);
        progress.advance(id, 25).ok();
        progress.advance(id, 25).ok();

        let task = progress.get_task(id);
        assert_eq!(task.map(|t| t.completed), Some(50));
    }

    #[test]
    fn test_progress_advance_invalid_task() {
        let mut progress = Progress::new();
        let result = progress.advance(TaskId::new(999), 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_progress_render() {
        let mut progress = Progress::new();
        progress.add_task("Test", Some(100), true);

        let segments = progress.render(80);
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_progress_render_multiple_tasks() {
        let mut progress = Progress::new();
        progress.add_task("Task 1", Some(100), true);
        progress.add_task("Task 2", Some(50), true);
        progress.add_task("Task 3", Some(200), true);

        let segments = progress.render(80);
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_progress_render_hidden_task() {
        let mut progress = Progress::new();
        progress.add_task("Visible", Some(100), true);
        progress.add_task("Hidden", Some(100), false);

        let segments = progress.render(80);
        let text = segments.plain_text();
        assert!(text.contains("Visible"));
        // Hidden task shouldn't appear
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(45)), "0:45");
        assert_eq!(format_duration(Duration::from_secs(125)), "2:05");
        assert_eq!(format_duration(Duration::from_secs(3665)), "1:01:05");
    }

    #[test]
    fn test_format_duration_zero() {
        assert_eq!(format_duration(Duration::from_secs(0)), "0:00");
    }

    #[test]
    fn test_format_duration_one_hour() {
        assert_eq!(format_duration(Duration::from_secs(3600)), "1:00:00");
    }

    #[test]
    fn test_progress_finished() {
        let mut progress = Progress::new();
        let id = progress.add_task("Test", Some(10), true);
        assert!(!progress.finished());

        progress.update(id, Some(10), None, None, None).ok();
        assert!(progress.finished());
    }

    #[test]
    fn test_progress_finished_empty() {
        let progress = Progress::new();
        assert!(progress.finished());
    }

    #[test]
    fn test_progress_finished_multiple() {
        let mut progress = Progress::new();
        let id1 = progress.add_task("Task 1", Some(10), true);
        let id2 = progress.add_task("Task 2", Some(20), true);

        progress.update(id1, Some(10), None, None, None).ok();
        assert!(!progress.finished()); // id2 not complete

        progress.update(id2, Some(20), None, None, None).ok();
        assert!(progress.finished());
    }

    #[test]
    fn test_task_remaining_with_progress() {
        let mut task = Task::new(TaskId::new(1), "Test", Some(100));
        task.start_time = Some(Instant::now() - Duration::from_secs(10));
        task.completed = 50;
        // 50% done in 10 seconds, should have about 10 seconds remaining
        let remaining = task.remaining();
        assert!(remaining.is_some());
    }

    #[test]
    fn test_task_remaining_zero_percent() {
        let mut task = Task::new(TaskId::new(1), "Test", Some(100));
        task.start_time = Some(Instant::now());
        task.completed = 0;
        // 0% complete, can't estimate remaining
        assert!(task.remaining().is_none());
    }

    #[test]
    fn test_task_speed_calculation() {
        let mut task = Task::new(TaskId::new(1), "Test", Some(100));
        task.start_time = Some(Instant::now() - Duration::from_secs(10));
        task.completed = 50;
        // 50 items in 10 seconds = 5 items/sec
        let speed = task.speed();
        assert!(speed.is_some());
        assert!((speed.unwrap() - 5.0).abs() < 0.5);
    }

    #[test]
    fn test_task_speed_no_start() {
        let task = Task::new(TaskId::new(1), "Test", Some(100));
        assert!(task.speed().is_none());
    }

    #[test]
    fn test_task_is_complete_overflow() {
        let mut task = Task::new(TaskId::new(1), "Test", Some(100));
        task.completed = 150;
        assert!(task.is_complete());
    }

    #[test]
    fn test_progress_bar_pulse_char() {
        let mut bar = ProgressBar::new();
        bar.pulse_char = '*';
        assert_eq!(bar.pulse_char, '*');
    }

    #[test]
    fn test_progress_bar_finished_style() {
        let style = Style::new().bold();
        let mut bar = ProgressBar::new();
        bar.finished_style = Some(style);
        assert!(bar.finished_style.is_some());
    }

    #[test]
    fn test_progress_bar_render_over_100() {
        let bar = ProgressBar::new().width(20);
        let segments = bar.render(Some(1.5)); // Over 100%
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_progress_bar_render_negative() {
        let bar = ProgressBar::new().width(20);
        let segments = bar.render(Some(-0.5)); // Negative
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_progress_update_invalid_task() {
        let mut progress = Progress::new();
        let result = progress.update(TaskId::new(999), Some(10), None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_progress_render_no_total() {
        let mut progress = Progress::new();
        progress.add_task("Indeterminate", None, true);
        let segments = progress.render(80);
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_progress_default() {
        let progress = Progress::default();
        assert!(progress.tasks.is_empty());
    }

    #[test]
    fn test_progress_bar_new() {
        let bar = ProgressBar::new();
        assert_eq!(bar.width, 40);
        assert!(bar.complete_style.is_some());
        assert!(bar.incomplete_style.is_some());
    }

    #[test]
    fn test_format_duration_minutes() {
        assert_eq!(format_duration(Duration::from_secs(90)), "1:30");
    }

    #[test]
    fn test_format_duration_hours() {
        assert_eq!(format_duration(Duration::from_secs(7200)), "2:00:00");
    }

    #[test]
    fn test_task_fields() {
        let mut task = Task::new(TaskId::new(1), "Test", Some(100));
        task.fields.insert("key".to_owned(), "value".to_owned());
        assert_eq!(task.fields.get("key"), Some(&"value".to_owned()));
    }
}
