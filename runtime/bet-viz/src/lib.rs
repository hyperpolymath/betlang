// SPDX-License-Identifier: MIT OR Apache-2.0
//! Visualization and plotting for Betlang
//!
//! Provides SVG plot generation and terminal-based visualization.

#![forbid(unsafe_code)]
use bet_rt::value::Value;
use im::Vector;
use plotters::prelude::*;
use std::collections::HashMap;

mod terminal;
pub use terminal::*;

/// Visualization error types
#[derive(Debug, Clone)]
pub enum VizError {
    InvalidData(String),
    RenderError(String),
    IoError(String),
}

impl std::fmt::Display for VizError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VizError::InvalidData(s) => write!(f, "Invalid data: {}", s),
            VizError::RenderError(s) => write!(f, "Render error: {}", s),
            VizError::IoError(s) => write!(f, "I/O error: {}", s),
        }
    }
}

impl std::error::Error for VizError {}

pub type VizResult<T> = Result<T, VizError>;

// ============================================================================
// Plot Configuration
// ============================================================================

/// Configuration for plots
#[derive(Debug, Clone)]
pub struct PlotConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub background: RGBColor,
    pub line_color: RGBColor,
    pub fill_color: Option<RGBColor>,
    pub margin: u32,
}

impl Default for PlotConfig {
    fn default() -> Self {
        PlotConfig {
            width: 800,
            height: 600,
            title: String::new(),
            x_label: "x".to_string(),
            y_label: "y".to_string(),
            background: WHITE,
            line_color: BLUE,
            fill_color: None,
            margin: 40,
        }
    }
}

impl PlotConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn with_labels(mut self, x: &str, y: &str) -> Self {
        self.x_label = x.to_string();
        self.y_label = y.to_string();
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_color(mut self, color: RGBColor) -> Self {
        self.line_color = color;
        self
    }
}

// ============================================================================
// Data Extraction
// ============================================================================

/// Extract numeric data from a list of Values
pub fn extract_floats(data: &Vector<Value>) -> Vec<f64> {
    data.iter()
        .filter_map(|v| match v {
            Value::Int(i) => Some(*i as f64),
            Value::Float(f) => Some(*f),
            _ => None,
        })
        .collect()
}

/// Extract x,y pairs from a list of tuples or maps
pub fn extract_points(data: &Vector<Value>) -> Vec<(f64, f64)> {
    data.iter()
        .filter_map(|v| match v {
            Value::Tuple(t) if t.len() >= 2 => {
                let x = match &t[0] {
                    Value::Int(i) => *i as f64,
                    Value::Float(f) => *f,
                    _ => return None,
                };
                let y = match &t[1] {
                    Value::Int(i) => *i as f64,
                    Value::Float(f) => *f,
                    _ => return None,
                };
                Some((x, y))
            }
            Value::Map(m) => {
                let x = match m.get("x") {
                    Some(Value::Int(i)) => *i as f64,
                    Some(Value::Float(f)) => *f,
                    _ => return None,
                };
                let y = match m.get("y") {
                    Some(Value::Int(i)) => *i as f64,
                    Some(Value::Float(f)) => *f,
                    _ => return None,
                };
                Some((x, y))
            }
            _ => None,
        })
        .collect()
}

// ============================================================================
// Line Plot
// ============================================================================

/// Generate a line plot as SVG
pub fn line_plot(data: &Vector<Value>, config: &PlotConfig) -> VizResult<String> {
    let points = extract_points(data);
    if points.is_empty() {
        return Err(VizError::InvalidData("No valid points found".to_string()));
    }

    let mut svg = String::new();
    {
        let root = SVGBackend::with_string(&mut svg, (config.width, config.height))
            .into_drawing_area();

        root.fill(&config.background)
            .map_err(|e| VizError::RenderError(e.to_string()))?;

        // Find data bounds
        let x_min = points.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min);
        let x_max = points.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
        let y_min = points.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
        let y_max = points.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);

        // Add margins
        let x_margin = (x_max - x_min) * 0.05;
        let y_margin = (y_max - y_min) * 0.05;

        let mut chart = ChartBuilder::on(&root)
            .caption(&config.title, ("sans-serif", 20))
            .margin(config.margin)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(
                (x_min - x_margin)..(x_max + x_margin),
                (y_min - y_margin)..(y_max + y_margin),
            )
            .map_err(|e| VizError::RenderError(e.to_string()))?;

        chart
            .configure_mesh()
            .x_desc(&config.x_label)
            .y_desc(&config.y_label)
            .draw()
            .map_err(|e| VizError::RenderError(e.to_string()))?;

        chart
            .draw_series(LineSeries::new(points, &config.line_color))
            .map_err(|e| VizError::RenderError(e.to_string()))?;

        root.present()
            .map_err(|e| VizError::RenderError(e.to_string()))?;
    }

    Ok(svg)
}

/// Generate a scatter plot as SVG
pub fn scatter_plot(data: &Vector<Value>, config: &PlotConfig) -> VizResult<String> {
    let points = extract_points(data);
    if points.is_empty() {
        return Err(VizError::InvalidData("No valid points found".to_string()));
    }

    let mut svg = String::new();
    {
        let root = SVGBackend::with_string(&mut svg, (config.width, config.height))
            .into_drawing_area();

        root.fill(&config.background)
            .map_err(|e| VizError::RenderError(e.to_string()))?;

        let x_min = points.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min);
        let x_max = points.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
        let y_min = points.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
        let y_max = points.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);

        let x_margin = (x_max - x_min) * 0.1;
        let y_margin = (y_max - y_min) * 0.1;

        let mut chart = ChartBuilder::on(&root)
            .caption(&config.title, ("sans-serif", 20))
            .margin(config.margin)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(
                (x_min - x_margin)..(x_max + x_margin),
                (y_min - y_margin)..(y_max + y_margin),
            )
            .map_err(|e| VizError::RenderError(e.to_string()))?;

        chart
            .configure_mesh()
            .x_desc(&config.x_label)
            .y_desc(&config.y_label)
            .draw()
            .map_err(|e| VizError::RenderError(e.to_string()))?;

        chart
            .draw_series(points.iter().map(|(x, y)| Circle::new((*x, *y), 3, config.line_color.filled())))
            .map_err(|e| VizError::RenderError(e.to_string()))?;

        root.present()
            .map_err(|e| VizError::RenderError(e.to_string()))?;
    }

    Ok(svg)
}

// ============================================================================
// Histogram
// ============================================================================

/// Generate a histogram as SVG
pub fn histogram(data: &Vector<Value>, bins: usize, config: &PlotConfig) -> VizResult<String> {
    let values = extract_floats(data);
    if values.is_empty() {
        return Err(VizError::InvalidData("No valid numeric data found".to_string()));
    }

    // Calculate bin edges
    let min_val = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let bin_width = (max_val - min_val) / bins as f64;

    // Count values in each bin
    let mut counts = vec![0u32; bins];
    for v in &values {
        let bin_idx = ((v - min_val) / bin_width).floor() as usize;
        let bin_idx = bin_idx.min(bins - 1);
        counts[bin_idx] += 1;
    }

    let max_count = *counts.iter().max().unwrap_or(&1);

    let mut svg = String::new();
    {
        let root = SVGBackend::with_string(&mut svg, (config.width, config.height))
            .into_drawing_area();

        root.fill(&config.background)
            .map_err(|e| VizError::RenderError(e.to_string()))?;

        let mut chart = ChartBuilder::on(&root)
            .caption(&config.title, ("sans-serif", 20))
            .margin(config.margin)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(
                (min_val)..(max_val + bin_width),
                0u32..(max_count + 1),
            )
            .map_err(|e| VizError::RenderError(e.to_string()))?;

        chart
            .configure_mesh()
            .x_desc(&config.x_label)
            .y_desc("Count")
            .draw()
            .map_err(|e| VizError::RenderError(e.to_string()))?;

        chart
            .draw_series(
                Histogram::vertical(&chart)
                    .style(config.line_color.filled())
                    .margin(1)
                    .data(
                        counts
                            .iter()
                            .enumerate()
                            .map(|(i, &c)| (min_val + i as f64 * bin_width, c)),
                    ),
            )
            .map_err(|e| VizError::RenderError(e.to_string()))?;

        root.present()
            .map_err(|e| VizError::RenderError(e.to_string()))?;
    }

    Ok(svg)
}

// ============================================================================
// Bar Chart
// ============================================================================

/// Generate a bar chart as SVG
pub fn bar_chart(
    labels: &[String],
    values: &[f64],
    config: &PlotConfig,
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

    let mut svg = String::new();
    {
        let root = SVGBackend::with_string(&mut svg, (config.width, config.height))
            .into_drawing_area();

        root.fill(&config.background)
            .map_err(|e| VizError::RenderError(e.to_string()))?;

        let mut chart = ChartBuilder::on(&root)
            .caption(&config.title, ("sans-serif", 20))
            .margin(config.margin)
            .x_label_area_size(60)
            .y_label_area_size(50)
            .build_cartesian_2d(0..labels.len(), 0.0..(max_val * 1.1))
            .map_err(|e| VizError::RenderError(e.to_string()))?;

        chart
            .configure_mesh()
            .y_desc(&config.y_label)
            .x_label_formatter(&|idx| {
                labels.get(*idx).cloned().unwrap_or_default()
            })
            .draw()
            .map_err(|e| VizError::RenderError(e.to_string()))?;

        chart
            .draw_series(
                values
                    .iter()
                    .enumerate()
                    .map(|(i, &v)| {
                        Rectangle::new(
                            [(i, 0.0), (i + 1, v)],
                            config.line_color.filled(),
                        )
                    }),
            )
            .map_err(|e| VizError::RenderError(e.to_string()))?;

        root.present()
            .map_err(|e| VizError::RenderError(e.to_string()))?;
    }

    Ok(svg)
}

// ============================================================================
// Box Plot
// ============================================================================

/// Statistics for box plot
#[derive(Debug, Clone)]
pub struct BoxStats {
    pub min: f64,
    pub q1: f64,
    pub median: f64,
    pub q3: f64,
    pub max: f64,
}

impl BoxStats {
    pub fn from_data(data: &[f64]) -> Option<Self> {
        if data.is_empty() {
            return None;
        }

        let mut sorted = data.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let n = sorted.len();
        let min = sorted[0];
        let max = sorted[n - 1];

        let median = if n % 2 == 0 {
            (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
        } else {
            sorted[n / 2]
        };

        let q1_idx = n / 4;
        let q3_idx = 3 * n / 4;

        Some(BoxStats {
            min,
            q1: sorted[q1_idx],
            median,
            q3: sorted[q3_idx],
            max,
        })
    }
}

// ============================================================================
// Distribution Visualization
// ============================================================================

/// Plot probability distribution samples
pub fn distribution_plot(
    samples: &Vector<Value>,
    bins: usize,
    config: &PlotConfig,
) -> VizResult<String> {
    // Use histogram with normalized counts (density)
    histogram(samples, bins, config)
}

/// Plot multiple distributions overlaid
pub fn multi_distribution_plot(
    distributions: &[(&str, Vector<Value>)],
    bins: usize,
    config: &PlotConfig,
) -> VizResult<String> {
    if distributions.is_empty() {
        return Err(VizError::InvalidData("No distributions provided".to_string()));
    }

    // Simple implementation: just plot the first one for now
    // A full implementation would overlay multiple histograms
    let (name, data) = &distributions[0];
    let mut cfg = config.clone();
    cfg.title = format!("{}: {}", config.title, name);
    histogram(data, bins, &cfg)
}

// ============================================================================
// Time Series
// ============================================================================

/// Plot time series data
pub fn time_series(
    times: &[f64],
    values: &[f64],
    config: &PlotConfig,
) -> VizResult<String> {
    if times.len() != values.len() {
        return Err(VizError::InvalidData(
            "Times and values must have same length".to_string(),
        ));
    }

    let points: Vector<Value> = times
        .iter()
        .zip(values.iter())
        .map(|(t, v)| {
            Value::Tuple(std::sync::Arc::new(vec![
                Value::Float(*t),
                Value::Float(*v),
            ]))
        })
        .collect();

    line_plot(&points, config)
}

// ============================================================================
// Heatmap
// ============================================================================

/// Generate a heatmap as SVG
pub fn heatmap(
    data: &[Vec<f64>],
    x_labels: Option<&[String]>,
    y_labels: Option<&[String]>,
    config: &PlotConfig,
) -> VizResult<String> {
    if data.is_empty() || data[0].is_empty() {
        return Err(VizError::InvalidData("Empty data matrix".to_string()));
    }

    let rows = data.len();
    let cols = data[0].len();

    // Find min/max for color scaling
    let min_val = data
        .iter()
        .flat_map(|row| row.iter())
        .cloned()
        .fold(f64::INFINITY, f64::min);
    let max_val = data
        .iter()
        .flat_map(|row| row.iter())
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    let mut svg = String::new();
    {
        let root = SVGBackend::with_string(&mut svg, (config.width, config.height))
            .into_drawing_area();

        root.fill(&config.background)
            .map_err(|e| VizError::RenderError(e.to_string()))?;

        let mut chart = ChartBuilder::on(&root)
            .caption(&config.title, ("sans-serif", 20))
            .margin(config.margin)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0..cols, 0..rows)
            .map_err(|e| VizError::RenderError(e.to_string()))?;

        chart
            .configure_mesh()
            .disable_mesh()
            .draw()
            .map_err(|e| VizError::RenderError(e.to_string()))?;

        // Draw cells
        for (row_idx, row) in data.iter().enumerate() {
            for (col_idx, &val) in row.iter().enumerate() {
                let normalized = (val - min_val) / (max_val - min_val);
                let intensity = (normalized * 255.0) as u8;
                let color = RGBColor(255 - intensity, 255 - intensity, 255);

                chart
                    .draw_series(std::iter::once(Rectangle::new(
                        [(col_idx, row_idx), (col_idx + 1, row_idx + 1)],
                        color.filled(),
                    )))
                    .map_err(|e| VizError::RenderError(e.to_string()))?;
            }
        }

        root.present()
            .map_err(|e| VizError::RenderError(e.to_string()))?;
    }

    Ok(svg)
}

// ============================================================================
// File Output
// ============================================================================

/// Save SVG to file
pub fn save_svg(svg: &str, path: &str) -> VizResult<()> {
    std::fs::write(path, svg).map_err(|e| VizError::IoError(e.to_string()))
}

// ============================================================================
// Native function bindings
// ============================================================================

use bet_rt::value::NativeFunction;
use std::sync::Arc;

/// Get all visualization native functions
pub fn native_functions() -> Vec<NativeFunction> {
    vec![
        NativeFunction {
            name: "histogram",
            arity: 2,
            func: |args| {
                if args.len() >= 2 {
                    if let (Value::List(data), Value::Int(bins)) = (&args[0], &args[1]) {
                        match histogram(data, *bins as usize, &PlotConfig::default()) {
                            Ok(svg) => Ok(Value::String(Arc::new(svg))),
                            Err(e) => Ok(Value::Error(Arc::new(e.to_string()))),
                        }
                    } else {
                        Err("histogram expects (list, int)".to_string())
                    }
                } else {
                    Err("histogram expects 2 arguments".to_string())
                }
            },
        },
        NativeFunction {
            name: "line_plot",
            arity: 1,
            func: |args| {
                if let Some(Value::List(data)) = args.first() {
                    match line_plot(data, &PlotConfig::default()) {
                        Ok(svg) => Ok(Value::String(Arc::new(svg))),
                        Err(e) => Ok(Value::Error(Arc::new(e.to_string()))),
                    }
                } else {
                    Err("line_plot expects a list".to_string())
                }
            },
        },
        NativeFunction {
            name: "scatter_plot",
            arity: 1,
            func: |args| {
                if let Some(Value::List(data)) = args.first() {
                    match scatter_plot(data, &PlotConfig::default()) {
                        Ok(svg) => Ok(Value::String(Arc::new(svg))),
                        Err(e) => Ok(Value::Error(Arc::new(e.to_string()))),
                    }
                } else {
                    Err("scatter_plot expects a list".to_string())
                }
            },
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_floats() {
        let data: Vector<Value> = vec![
            Value::Int(1),
            Value::Float(2.5),
            Value::Int(3),
            Value::String(Arc::new("ignore".to_string())),
        ]
        .into_iter()
        .collect();

        let floats = extract_floats(&data);
        assert_eq!(floats, vec![1.0, 2.5, 3.0]);
    }

    #[test]
    fn test_extract_points() {
        let data: Vector<Value> = vec![
            Value::Tuple(Arc::new(vec![Value::Float(1.0), Value::Float(2.0)])),
            Value::Tuple(Arc::new(vec![Value::Float(3.0), Value::Float(4.0)])),
        ]
        .into_iter()
        .collect();

        let points = extract_points(&data);
        assert_eq!(points, vec![(1.0, 2.0), (3.0, 4.0)]);
    }

    #[test]
    fn test_box_stats() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let stats = BoxStats::from_data(&data).unwrap();

        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 10.0);
        assert_eq!(stats.median, 5.5);
    }
}
