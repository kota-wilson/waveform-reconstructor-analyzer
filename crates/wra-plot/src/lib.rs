//! SVG plotting support for desktop waveform analysis.

use std::fmt;
use std::ops::Range;
use std::path::{Path, PathBuf};

use plotters::coord::Shift;
use plotters::prelude::*;
use wra_core::model::{Channel, Waveform};

const DEFAULT_WIDTH: u32 = 1024;
const DEFAULT_HEIGHT: u32 = 760;

#[derive(Debug, Clone, PartialEq)]
pub struct PlotOptions {
    pub output_path: PathBuf,
    pub title: String,
    pub channels: Vec<String>,
    pub z_channel: Option<String>,
    pub width: u32,
    pub height: u32,
}

impl PlotOptions {
    pub fn new(output_path: impl Into<PathBuf>, channels: Vec<String>) -> Self {
        Self {
            output_path: output_path.into(),
            title: "Waveform Plot".to_string(),
            channels,
            z_channel: None,
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlotError {
    EmptyWaveform,
    MissingChannel { channel: String },
    InvalidParameter { name: String, reason: String },
    InvalidOutputPath { path: PathBuf, reason: String },
    Render { message: String },
}

impl fmt::Display for PlotError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyWaveform => write!(formatter, "waveform contains no samples to plot"),
            Self::MissingChannel { channel } => {
                write!(formatter, "missing plot channel `{channel}`")
            }
            Self::InvalidParameter { name, reason } => {
                write!(formatter, "invalid plot parameter `{name}`: {reason}")
            }
            Self::InvalidOutputPath { path, reason } => {
                write!(
                    formatter,
                    "invalid plot output path `{}`: {reason}",
                    path.display()
                )
            }
            Self::Render { message } => write!(formatter, "failed to render plot: {message}"),
        }
    }
}

impl std::error::Error for PlotError {}

pub type Result<T> = std::result::Result<T, PlotError>;

pub fn render_svg(waveform: &Waveform, options: &PlotOptions) -> Result<()> {
    validate_output_path(&options.output_path)?;
    let area =
        SVGBackend::new(&options.output_path, (options.width, options.height)).into_drawing_area();
    draw_plot(area, waveform, options)
}

pub fn render_svg_string(waveform: &Waveform, options: &PlotOptions) -> Result<String> {
    let mut output = String::new();
    {
        let area = SVGBackend::with_string(&mut output, (options.width, options.height))
            .into_drawing_area();
        draw_plot(area, waveform, options)?;
    }
    Ok(output)
}

fn draw_plot<DB>(
    area: DrawingArea<DB, Shift>,
    waveform: &Waveform,
    options: &PlotOptions,
) -> Result<()>
where
    DB: DrawingBackend,
    DB::ErrorType: fmt::Debug,
{
    validate_options(waveform, options)?;
    area.fill(&WHITE).map_err(render_error)?;

    match options.z_channel.as_deref() {
        Some(z_channel) => draw_3d(area.clone(), waveform, options, z_channel)?,
        None => draw_2d(area.clone(), waveform, options)?,
    }

    area.present().map_err(render_error)
}

fn draw_2d<DB>(
    area: DrawingArea<DB, Shift>,
    waveform: &Waveform,
    options: &PlotOptions,
) -> Result<()>
where
    DB: DrawingBackend,
    DB::ErrorType: fmt::Debug,
{
    let channels = plot_channels(waveform, &options.channels)?;
    let x_range = padded_range(waveform.time.iter().copied(), "time")?;
    let y_range = padded_range(
        channels
            .iter()
            .flat_map(|channel| channel.samples.iter().copied()),
        "signal",
    )?;

    let mut chart = ChartBuilder::on(&area)
        .caption(options.title.as_str(), ("sans", 20))
        .margin(24)
        .x_label_area_size(40)
        .y_label_area_size(56)
        .build_cartesian_2d(x_range, y_range)
        .map_err(render_error)?;

    chart
        .configure_mesh()
        .x_desc(format!("time ({})", waveform.time_unit.name))
        .y_desc("signal")
        .light_line_style(BLACK.mix(0.15))
        .draw()
        .map_err(render_error)?;

    for (series_index, channel) in channels.iter().enumerate() {
        let color = series_color(series_index);
        let points = waveform
            .time
            .iter()
            .copied()
            .zip(channel.samples.iter().copied());
        chart
            .draw_series(LineSeries::new(points, color))
            .map_err(render_error)?
            .label(channel.name.clone())
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
    }

    chart
        .configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.8))
        .draw()
        .map_err(render_error)?;

    Ok(())
}

fn draw_3d<DB>(
    area: DrawingArea<DB, Shift>,
    waveform: &Waveform,
    options: &PlotOptions,
    z_channel_name: &str,
) -> Result<()>
where
    DB: DrawingBackend,
    DB::ErrorType: fmt::Debug,
{
    let channels = plot_channels(waveform, &options.channels)?;
    let z_channel = waveform
        .channel(z_channel_name)
        .ok_or_else(|| PlotError::MissingChannel {
            channel: z_channel_name.to_string(),
        })?;
    let x_range = padded_range(waveform.time.iter().copied(), "time")?;
    let y_range = padded_range(
        channels
            .iter()
            .flat_map(|channel| channel.samples.iter().copied()),
        "signal",
    )?;
    let z_range = padded_range(z_channel.samples.iter().copied(), "third_axis")?;

    let mut chart = ChartBuilder::on(&area)
        .caption(options.title.as_str(), ("sans", 20))
        .margin(24)
        .build_cartesian_3d(x_range, y_range, z_range)
        .map_err(render_error)?;

    chart.with_projection(|mut projection| {
        projection.yaw = 0.45;
        projection.scale = 0.86;
        projection.into_matrix()
    });

    chart
        .configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw()
        .map_err(render_error)?;

    for (series_index, channel) in channels.iter().enumerate() {
        let color = series_color(series_index);
        let points = waveform
            .time
            .iter()
            .copied()
            .zip(channel.samples.iter().copied())
            .zip(z_channel.samples.iter().copied())
            .map(|((time, sample), z_value)| (time, sample, z_value));
        chart
            .draw_series(LineSeries::new(points, color))
            .map_err(render_error)?
            .label(format!("{} vs {}", channel.name, z_channel.name))
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
    }

    chart
        .configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.8))
        .draw()
        .map_err(render_error)?;

    Ok(())
}

fn validate_options(waveform: &Waveform, options: &PlotOptions) -> Result<()> {
    if waveform.time.is_empty() {
        return Err(PlotError::EmptyWaveform);
    }
    if options.channels.is_empty() {
        return Err(PlotError::InvalidParameter {
            name: "channels".to_string(),
            reason: "at least one channel is required".to_string(),
        });
    }
    if options.width == 0 || options.height == 0 {
        return Err(PlotError::InvalidParameter {
            name: "dimensions".to_string(),
            reason: "width and height must be greater than zero".to_string(),
        });
    }
    if let Some(z_channel) = &options.z_channel {
        if options.channels.iter().any(|channel| channel == z_channel) {
            return Err(PlotError::InvalidParameter {
                name: "z_channel".to_string(),
                reason: "third-axis channel must be separate from plotted channels".to_string(),
            });
        }
    }
    Ok(())
}

fn validate_output_path(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty() {
        return Err(PlotError::InvalidOutputPath {
            path: path.to_path_buf(),
            reason: "path must not be empty".to_string(),
        });
    }

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            return Err(PlotError::InvalidOutputPath {
                path: path.to_path_buf(),
                reason: "parent directory does not exist".to_string(),
            });
        }
    }

    Ok(())
}

fn plot_channels<'a>(waveform: &'a Waveform, names: &[String]) -> Result<Vec<&'a Channel>> {
    names
        .iter()
        .map(|name| {
            waveform
                .channel(name)
                .ok_or_else(|| PlotError::MissingChannel {
                    channel: name.clone(),
                })
        })
        .collect()
}

fn padded_range(values: impl Iterator<Item = f64>, name: &str) -> Result<Range<f64>> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut found = false;

    for value in values {
        if !value.is_finite() {
            return Err(PlotError::InvalidParameter {
                name: name.to_string(),
                reason: "axis values must be finite".to_string(),
            });
        }
        min = min.min(value);
        max = max.max(value);
        found = true;
    }

    if !found {
        return Err(PlotError::EmptyWaveform);
    }

    if (max - min).abs() <= f64::EPSILON {
        let padding = if max.abs() > 1.0 {
            max.abs() * 0.1
        } else {
            1.0
        };
        Ok((min - padding)..(max + padding))
    } else {
        let padding = (max - min) * 0.05;
        Ok((min - padding)..(max + padding))
    }
}

fn series_color(index: usize) -> &'static RGBColor {
    const COLORS: [&RGBColor; 6] = [&BLUE, &RED, &GREEN, &MAGENTA, &CYAN, &BLACK];
    COLORS[index % COLORS.len()]
}

fn render_error<E: fmt::Debug>(error: E) -> PlotError {
    PlotError::Render {
        message: format!("{error:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wra_core::model::{Channel, Unit};

    fn waveform() -> Waveform {
        Waveform::new(
            vec![0.0, 0.001, 0.002, 0.003],
            vec![
                Channel::new("input_v", Unit::volts(), vec![0.0, 2.5, 5.0, 2.5]),
                Channel::new("temp_c", Unit::new("C"), vec![20.0, 21.0, 22.0, 23.0]),
            ],
        )
        .expect("test waveform should be valid")
    }

    #[test]
    fn renders_2d_svg_for_selected_channel() {
        let options = PlotOptions::new("unused.svg", vec!["input_v".to_string()]);

        let svg = render_svg_string(&waveform(), &options).expect("plot should render");

        assert!(svg.contains("<svg"));
        assert!(svg.contains("Waveform Plot"));
        assert!(svg.contains("input_v"));
    }

    #[test]
    fn renders_3d_svg_with_third_axis_channel() {
        let mut options = PlotOptions::new("unused.svg", vec!["input_v".to_string()]);
        options.z_channel = Some("temp_c".to_string());
        options.title = "3D Waveform Plot".to_string();

        let svg = render_svg_string(&waveform(), &options).expect("plot should render");

        assert!(svg.contains("<svg"));
        assert!(svg.contains("3D Waveform Plot"));
        assert!(svg.contains("input_v vs temp_c"));
    }

    #[test]
    fn rejects_missing_plot_channel() {
        let options = PlotOptions::new("unused.svg", vec!["missing_v".to_string()]);

        let result = render_svg_string(&waveform(), &options);

        assert_eq!(
            result,
            Err(PlotError::MissingChannel {
                channel: "missing_v".to_string()
            })
        );
    }

    #[test]
    fn rejects_z_channel_reuse_as_plot_channel() {
        let mut options = PlotOptions::new("unused.svg", vec!["input_v".to_string()]);
        options.z_channel = Some("input_v".to_string());

        let result = render_svg_string(&waveform(), &options);

        assert!(matches!(
            result,
            Err(PlotError::InvalidParameter { name, .. }) if name == "z_channel"
        ));
    }

    #[test]
    fn rejects_output_path_with_missing_parent_directory() {
        let output_path = std::env::temp_dir()
            .join(format!("wra-missing-parent-{}", std::process::id()))
            .join("plot.svg");
        let options = PlotOptions::new(output_path.clone(), vec!["input_v".to_string()]);

        let result = render_svg(&waveform(), &options);

        assert_eq!(
            result,
            Err(PlotError::InvalidOutputPath {
                path: output_path,
                reason: "parent directory does not exist".to_string(),
            })
        );
    }
}
