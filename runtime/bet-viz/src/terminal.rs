// SPDX-License-Identifier: MIT OR Apache-2.0
//! Terminal-based visualization for Betlang
//!
//! Provides ASCII/Unicode plots for terminal output.

use bet_rt::value::Value;
use im::Vector;
use textplots::{Chart, Plot, Shape};

use super::{extract_floats, VizError, VizResult};

// ============================================================================
// Terminal Plot Configuration
// ============================================================================

/// Configuration for terminal plots
#[derive(Debug, Clone)]
pub struct TermPlotConfig {
    pub width: u32,
    pub height: u32,
    pub title: Option<String>,
}

impl Default for TermPlotConfig {
    fn default() -> Self {
        TermPlotConfig {
            width: 80,
            height: 20,
            title: None,
        }
    }
}

impl TermPlotConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }
}

// ============================================================================
// Terminal Line Plot
// ============================================================================

/// Print a line plot to the terminal
pub fn term_line_plot(x: &[f32], y: &[f32], config: &TermPlotConfig) -> VizResult<String> {
    if x.len() != y.len() {
        return Err(VizError::InvalidData("x and y must have same length".to_string()));
    }

    if x.is_empty() {
        return Err(VizError::InvalidData("No data provided".to_string()));
    }

    let points: Vec<(f32, f32)> = x.iter().cloned().zip(y.iter().cloned()).collect();

    let mut output = String::new();

    if let Some(title) = &config.title {
        output.push_str(&format!("{}\n", title));
        output.push_str(&"─".repeat(config.width as usize));
        output.push('\n');
    }

    // Use textplots for ASCII rendering
    let chart = Chart::new(config.width * 2, config.height, points[0].0, points.last().unwrap().0)
        .lineplot(&Shape::Lines(&points));

    output.push_str(&format!("{}", chart));

    Ok(output)
}

/// Print a simple plot of values (indexed by position)
pub fn term_plot(data: &Vector<Value>, config: &TermPlotConfig) -> VizResult<String> {
    let values = extract_floats(data);
    if values.is_empty() {
        return Err(VizError::InvalidData("No numeric data found".to_string()));
    }

    let x: Vec<f32> = (0..values.len()).map(|i| i as f32).collect();
    let y: Vec<f32> = values.iter().map(|v| *v as f32).collect();

    term_line_plot(&x, &y, config)
}

// ============================================================================
// Terminal Histogram
// ============================================================================

/// Print a histogram to the terminal
pub fn term_histogram(data: &Vector<Value>, bins: usize, config: &TermPlotConfig) -> VizResult<String> {
    let values = extract_floats(data);
    if values.is_empty() {
        return Err(VizError::InvalidData("No numeric data found".to_string()));
    }

    let min_val = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let bin_width = (max_val - min_val) / bins as f64;

    // Count values in each bin
    let mut counts = vec![0usize; bins];
    for v in &values {
        let bin_idx = ((v - min_val) / bin_width).floor() as usize;
        let bin_idx = bin_idx.min(bins - 1);
        counts[bin_idx] += 1;
    }

    let max_count = *counts.iter().max().unwrap_or(&1);

    let mut output = String::new();

    if let Some(title) = &config.title {
        output.push_str(&format!("{}\n", title));
        output.push_str(&"─".repeat(config.width as usize));
        output.push('\n');
    }

    // Calculate bar width
    let bar_max_width = config.width as usize - 15; // Leave room for labels

    for (i, &count) in counts.iter().enumerate() {
        let bin_start = min_val + i as f64 * bin_width;
        let bin_end = bin_start + bin_width;
        let bar_width = (count as f64 / max_count as f64 * bar_max_width as f64) as usize;

        output.push_str(&format!(
            "{:>6.2}-{:<6.2} │{}",
            bin_start,
            bin_end,
            "█".repeat(bar_width)
        ));

        if bar_width < bar_max_width {
            output.push_str(&format!(" ({})", count));
        }
        output.push('\n');
    }

    Ok(output)
}

// ============================================================================
// Terminal Scatter Plot
// ============================================================================

/// Print a scatter plot to the terminal
pub fn term_scatter(x: &[f32], y: &[f32], config: &TermPlotConfig) -> VizResult<String> {
    if x.len() != y.len() {
        return Err(VizError::InvalidData("x and y must have same length".to_string()));
    }

    if x.is_empty() {
        return Err(VizError::InvalidData("No data provided".to_string()));
    }

    let points: Vec<(f32, f32)> = x.iter().cloned().zip(y.iter().cloned()).collect();

    let mut output = String::new();

    if let Some(title) = &config.title {
        output.push_str(&format!("{}\n", title));
        output.push_str(&"─".repeat(config.width as usize));
        output.push('\n');
    }

    let x_min = x.iter().cloned().fold(f32::INFINITY, f32::min);
    let x_max = x.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    let chart = Chart::new(config.width * 2, config.height, x_min, x_max)
        .lineplot(&Shape::Points(&points));

    output.push_str(&format!("{}", chart));

    Ok(output)
}

// ============================================================================
// Terminal Bar Chart
// ============================================================================

/// Print a horizontal bar chart to the terminal
pub fn term_bar_chart(
    labels: &[String],
    values: &[f64],
    config: &TermPlotConfig,
) -> VizResult<String> {
    if labels.len() != values.len() {
        return Err(VizError::InvalidData(
            "Labels and values must have same length".to_string(),
        ));
    }

    if labels.is_empty() {
        return Err(VizError::InvalidData("No data provided".to_string()));
    }

    let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let max_label_len = labels.iter().map(|l| l.len()).max().unwrap_or(0);
    let bar_max_width = config.width as usize - max_label_len - 10;

    let mut output = String::new();

    if let Some(title) = &config.title {
        output.push_str(&format!("{}\n", title));
        output.push_str(&"─".repeat(config.width as usize));
        output.push('\n');
    }

    for (label, &value) in labels.iter().zip(values.iter()) {
        let bar_width = (value / max_val * bar_max_width as f64) as usize;
        output.push_str(&format!(
            "{:>width$} │{} {:.2}\n",
            label,
            "█".repeat(bar_width),
            value,
            width = max_label_len
        ));
    }

    Ok(output)
}

// ============================================================================
// Sparkline
// ============================================================================

/// Generate a sparkline (mini inline chart)
pub fn sparkline(data: &[f64]) -> String {
    if data.is_empty() {
        return String::new();
    }

    let min_val = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max_val - min_val;

    // Sparkline characters (8 levels)
    const CHARS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    data.iter()
        .map(|&v| {
            let normalized = if range > 0.0 {
                (v - min_val) / range
            } else {
                0.5
            };
            let idx = (normalized * 7.0).round() as usize;
            CHARS[idx.min(7)]
        })
        .collect()
}

/// Generate sparkline from betlang values
pub fn value_sparkline(data: &Vector<Value>) -> String {
    let floats = extract_floats(data);
    sparkline(&floats)
}

// ============================================================================
// Progress Bar
// ============================================================================

/// Generate a progress bar
pub fn progress_bar(current: f64, total: f64, width: usize) -> String {
    let ratio = (current / total).clamp(0.0, 1.0);
    let filled = (ratio * width as f64) as usize;
    let empty = width - filled;

    format!(
        "[{}{}] {:.1}%",
        "█".repeat(filled),
        "░".repeat(empty),
        ratio * 100.0
    )
}

// ============================================================================
// ASCII Box Drawing
// ============================================================================

/// Draw a box around text
pub fn boxed(content: &str, width: usize) -> String {
    let mut output = String::new();

    // Top border
    output.push('┌');
    output.push_str(&"─".repeat(width));
    output.push('┐');
    output.push('\n');

    // Content lines
    for line in content.lines() {
        let padding = width.saturating_sub(line.chars().count());
        output.push('│');
        output.push_str(line);
        output.push_str(&" ".repeat(padding));
        output.push('│');
        output.push('\n');
    }

    // Bottom border
    output.push('└');
    output.push_str(&"─".repeat(width));
    output.push('┘');

    output
}

/// Draw a simple table
pub fn table(headers: &[&str], rows: &[Vec<String>], col_widths: &[usize]) -> String {
    let mut output = String::new();

    // Header separator
    let separator: String = col_widths
        .iter()
        .map(|w| "─".repeat(*w + 2))
        .collect::<Vec<_>>()
        .join("┼");

    // Top border
    output.push('┌');
    output.push_str(&separator.replace('┼', "┬"));
    output.push('┐');
    output.push('\n');

    // Headers
    output.push('│');
    for (header, &width) in headers.iter().zip(col_widths.iter()) {
        output.push_str(&format!(" {:^width$} │", header, width = width));
    }
    output.push('\n');

    // Header separator
    output.push('├');
    output.push_str(&separator);
    output.push('┤');
    output.push('\n');

    // Rows
    for row in rows {
        output.push('│');
        for (cell, &width) in row.iter().zip(col_widths.iter()) {
            let truncated: String = cell.chars().take(width).collect();
            output.push_str(&format!(" {:width$} │", truncated, width = width));
        }
        output.push('\n');
    }

    // Bottom border
    output.push('└');
    output.push_str(&separator.replace('┼', "┴"));
    output.push('┘');

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparkline() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        let line = sparkline(&data);
        assert_eq!(line.chars().count(), 9);
    }

    #[test]
    fn test_progress_bar() {
        let bar = progress_bar(50.0, 100.0, 20);
        assert!(bar.contains("50.0%"));
        assert!(bar.contains("█"));
        assert!(bar.contains("░"));
    }

    #[test]
    fn test_boxed() {
        let output = boxed("Hello\nWorld", 10);
        assert!(output.contains("┌"));
        assert!(output.contains("└"));
        assert!(output.contains("Hello"));
    }
}
