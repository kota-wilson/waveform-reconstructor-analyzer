use std::env;
use std::fs;
use std::process::ExitCode;

use wra_core::analysis::evaluate_criteria_with_measurements;
use wra_core::config::AnalysisConfig;
use wra_core::criteria::Criterion;
use wra_core::csv::{CsvParseOptions, SimpleCsvParser, WaveformParser};
use wra_core::filter::{
    apply_filter_chain, AdcQuantizer, FilterStep, LowPassFilter, MovingAverageFilter,
};
use wra_core::model::{MetadataContext, TolerancePolicy};
use wra_core::report::{AnalysisReport, ReportEvidenceContext};
use wra_plot::{evidence_overlays, render_svg, PlotOptions};

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}

fn run(args: Vec<String>) -> Result<String, String> {
    if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        return Ok(help());
    }

    match args.first().map(String::as_str) {
        Some("analyze") => run_analyze(&args),
        Some("plot") => run_plot(&args),
        Some(other) => Err(format!(
            "expected subcommand `analyze` or `plot`, got `{other}`"
        )),
        None => Ok(help()),
    }
}

fn run_analyze(args: &[String]) -> Result<String, String> {
    let input_path = value_after(args, "--input").ok_or("missing --input <path>")?;
    let output_format = value_after(args, "--format").unwrap_or("text");
    let config = load_config(args)?;
    let (options, filters, criteria, tolerances, metadata) = match config {
        Some(config) => (
            config.csv_options(),
            config
                .filters()
                .map_err(|error| format!("invalid config filters: {error}"))?,
            config
                .criteria()
                .map_err(|error| format!("invalid config criteria: {error}"))?,
            config.tolerances,
            config.metadata,
        ),
        None => {
            let time_column = value_after(args, "--time-column").unwrap_or("time");
            let channels =
                value_after(args, "--channels").ok_or("missing --channels <name[,name]>")?;
            let channel_columns = channels
                .split(',')
                .map(str::trim)
                .filter(|channel| !channel.is_empty())
                .map(str::to_string)
                .collect::<Vec<_>>();
            if channel_columns.is_empty() {
                return Err("--channels must include at least one channel".to_string());
            }

            (
                CsvParseOptions::new(time_column, channel_columns),
                parse_filters(args)?,
                parse_criteria(args)?,
                TolerancePolicy::default(),
                MetadataContext::default(),
            )
        }
    };

    let input = fs::read_to_string(input_path)
        .map_err(|error| format!("failed to read `{input_path}`: {error}"))?;
    let parser = SimpleCsvParser;
    let mut waveform = parser
        .parse_str(&input, &options)
        .map_err(|error| format!("failed to parse waveform: {error}"))?
        .with_source_name(input_path.to_string())
        .with_metadata_context(&metadata)
        .with_tolerance_policy(tolerances);

    waveform = apply_filter_chain(&waveform, &filters)
        .map_err(|error| format!("filter pipeline failed: {error}"))?;

    let evaluation = evaluate_criteria_with_measurements(&waveform, &criteria, tolerances)
        .map_err(|error| format!("analysis failed: {error}"))?;
    let report = AnalysisReport {
        input_name: input_path.to_string(),
        waveform_metadata: waveform.metadata.clone(),
        evidence_context: ReportEvidenceContext::engineering_validation(tolerances),
        measurements: evaluation.measurements,
        results: evaluation.results,
    };

    render_report(&report, output_format)
}

fn run_plot(args: &[String]) -> Result<String, String> {
    let input_path = value_after(args, "--input").ok_or("missing --input <path>")?;
    let output_path = value_after(args, "--output").ok_or("missing --output <svg>")?;
    let z_column = value_after(args, "--z-column").map(str::to_string);
    let title = value_after(args, "--title").unwrap_or("Waveform Plot");
    let width = parse_optional_u32(args, "--width", 1024)?;
    let height = parse_optional_u32(args, "--height", 760)?;
    let config = load_config(args)?;

    let input = fs::read_to_string(input_path)
        .map_err(|error| format!("failed to read `{input_path}`: {error}"))?;

    let (waveform, channel_columns, overlays) = match config {
        Some(config) => {
            let channel_columns = match value_after(args, "--channels") {
                Some(channels) => parse_channel_list(channels)?,
                None => config.input.channels.clone(),
            };
            let mut csv_options = config.csv_options();
            include_channels(&mut csv_options.channel_columns, &channel_columns);
            if let Some(z_column) = &z_column {
                include_channels(
                    &mut csv_options.channel_columns,
                    std::slice::from_ref(z_column),
                );
            }

            let parser = SimpleCsvParser;
            let mut waveform = parser
                .parse_str(&input, &csv_options)
                .map_err(|error| format!("failed to parse waveform: {error}"))?
                .with_source_name(input_path.to_string())
                .with_metadata_context(&config.metadata)
                .with_tolerance_policy(config.tolerances);
            let filters = config
                .filters()
                .map_err(|error| format!("invalid config filters: {error}"))?;
            waveform = apply_filter_chain(&waveform, &filters)
                .map_err(|error| format!("filter pipeline failed: {error}"))?;
            let criteria = config
                .criteria()
                .map_err(|error| format!("invalid config criteria: {error}"))?;
            let evaluation =
                evaluate_criteria_with_measurements(&waveform, &criteria, config.tolerances)
                    .map_err(|error| format!("analysis failed: {error}"))?;
            let overlays = evidence_overlays(&evaluation.measurements, &evaluation.results);
            (waveform, channel_columns, overlays)
        }
        None => {
            let time_column = value_after(args, "--time-column").unwrap_or("time");
            let channel_columns = parse_channel_list(
                value_after(args, "--channels").ok_or("missing --channels <name[,name]>")?,
            )?;
            let mut parser_channels = channel_columns.clone();
            if let Some(z_column) = &z_column {
                include_channels(&mut parser_channels, std::slice::from_ref(z_column));
            }
            let parser = SimpleCsvParser;
            let waveform = parser
                .parse_str(&input, &CsvParseOptions::new(time_column, parser_channels))
                .map_err(|error| format!("failed to parse waveform: {error}"))?
                .with_source_name(input_path.to_string());
            (waveform, channel_columns, Vec::new())
        }
    };

    let mut options = PlotOptions::new(output_path, channel_columns);
    options.title = title.to_string();
    options.z_channel = z_column;
    options.evidence_overlays = overlays;
    options.width = width;
    options.height = height;

    render_svg(&waveform, &options).map_err(|error| format!("failed to render plot: {error}"))?;

    Ok(format!("Plot written to {output_path}"))
}

fn include_channels(columns: &mut Vec<String>, required: &[String]) {
    for channel in required {
        if !columns.iter().any(|existing| existing == channel) {
            columns.push(channel.clone());
        }
    }
}

fn value_after<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|window| window[0] == flag)
        .map(|window| window[1].as_str())
}

fn load_config(args: &[String]) -> Result<Option<AnalysisConfig>, String> {
    let Some(path) = value_after(args, "--config") else {
        return Ok(None);
    };

    let input =
        fs::read_to_string(path).map_err(|error| format!("failed to read `{path}`: {error}"))?;
    let config = toml::from_str::<AnalysisConfig>(&input)
        .map_err(|error| format!("failed to parse config `{path}`: {error}"))?;

    if config.input.channels.is_empty() {
        return Err("config input.channels must include at least one channel".to_string());
    }
    if config.criteria.is_empty() {
        return Err("config must include at least one [[criteria]] entry".to_string());
    }
    config
        .validate()
        .map_err(|error| format!("invalid config: {error}"))?;

    Ok(Some(config))
}

fn parse_filters(args: &[String]) -> Result<Vec<FilterStep>, String> {
    let mut filters = Vec::new();
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--moving-average" => {
                let value = args
                    .get(index + 1)
                    .ok_or("missing value after --moving-average")?;
                let window_samples = value
                    .parse::<usize>()
                    .map_err(|_| format!("invalid moving average window `{value}`"))?;
                filters.push(FilterStep::MovingAverage(MovingAverageFilter {
                    window_samples,
                }));
                index += 2;
            }
            "--low-pass" => {
                let value = args
                    .get(index + 1)
                    .ok_or("missing value after --low-pass")?;
                let cutoff_hz = value
                    .parse::<f64>()
                    .map_err(|_| format!("invalid low-pass cutoff `{value}`"))?;
                filters.push(FilterStep::LowPass(LowPassFilter { cutoff_hz }));
                index += 2;
            }
            "--adc-quantize" => {
                let value = args
                    .get(index + 1)
                    .ok_or("missing value after --adc-quantize")?;
                filters.push(parse_adc_quantize(value)?);
                index += 2;
            }
            _ => index += 1,
        }
    }
    Ok(filters)
}

fn parse_adc_quantize(value: &str) -> Result<FilterStep, String> {
    let parts = value.split(':').collect::<Vec<_>>();
    if parts.len() != 3 {
        return Err("ADC quantization must use bits:min_v:max_v syntax".to_string());
    }

    let bits = parts[0]
        .parse::<u8>()
        .map_err(|_| format!("invalid ADC bit depth `{}`", parts[0]))?;
    let min_v = parts[1]
        .parse::<f64>()
        .map_err(|_| format!("invalid ADC min voltage `{}`", parts[1]))?;
    let max_v = parts[2]
        .parse::<f64>()
        .map_err(|_| format!("invalid ADC max voltage `{}`", parts[2]))?;

    Ok(FilterStep::AdcQuantize(AdcQuantizer { bits, min_v, max_v }))
}

fn parse_optional_u32(args: &[String], flag: &str, default: u32) -> Result<u32, String> {
    match value_after(args, flag) {
        Some(value) => value
            .parse::<u32>()
            .map_err(|_| format!("invalid {flag} value `{value}`")),
        None => Ok(default),
    }
}

fn parse_channel_list(channels: &str) -> Result<Vec<String>, String> {
    let channel_columns = channels
        .split(',')
        .map(str::trim)
        .filter(|channel| !channel.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();
    if channel_columns.is_empty() {
        return Err("--channels must include at least one channel".to_string());
    }
    Ok(channel_columns)
}

fn render_report(report: &AnalysisReport, output_format: &str) -> Result<String, String> {
    match output_format {
        "text" => Ok(report.render_text()),
        "json" => report
            .render_json_pretty()
            .map_err(|error| format!("failed to render json report: {error}")),
        _ => Err(format!(
            "unsupported --format `{output_format}`; use text or json"
        )),
    }
}

fn parse_criteria(args: &[String]) -> Result<Vec<Criterion>, String> {
    let mut criteria = Vec::new();
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--min" => {
                let value = args.get(index + 1).ok_or("missing value after --min")?;
                let (channel, threshold) = parse_channel_threshold(value)?;
                criteria.push(Criterion::minimum_voltage(
                    format!("min_voltage_{channel}"),
                    channel,
                    threshold,
                ));
                index += 2;
            }
            "--max" => {
                let value = args.get(index + 1).ok_or("missing value after --max")?;
                let (channel, threshold) = parse_channel_threshold(value)?;
                criteria.push(Criterion::maximum_voltage(
                    format!("max_voltage_{channel}"),
                    channel,
                    threshold,
                ));
                index += 2;
            }
            _ => index += 1,
        }
    }

    if criteria.is_empty() {
        return Err(
            "provide at least one criterion with --min channel:value or --max channel:value"
                .to_string(),
        );
    }
    Ok(criteria)
}

fn parse_channel_threshold(value: &str) -> Result<(&str, f64), String> {
    let (channel, threshold) = value
        .split_once(':')
        .ok_or("criterion must use channel:value syntax")?;
    let threshold = threshold
        .parse::<f64>()
        .map_err(|_| format!("invalid threshold `{threshold}`"))?;
    Ok((channel, threshold))
}

fn help() -> String {
    [
        "Waveform Reconstructor and Analyzer",
        "",
        "Usage:",
        "  wra analyze --input <csv> --config examples/basic-config.toml --format text",
        "  wra analyze --input <csv> --time-column time --channels input_v --moving-average 3 --low-pass 25 --adc-quantize 12:0.0:5.0 --min input_v:0.0 --max input_v:5.5 --format json",
        "  wra plot --input <csv> --time-column time --channels input_v --output waveform.svg",
        "  wra plot --input <csv> --config examples/basic-config.toml --output annotated.svg",
        "  wra plot --input <csv> --time-column time --channels input_v --z-column temp_c --output waveform-3d.svg",
        "",
        "ADC quantization syntax: --adc-quantize bits:min_v:max_v",
        "Plot output is SVG. Use --config to add 2D criteria evidence overlays; use --z-column for an optional third axis.",
        "Formats: text, json",
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_filters_in_command_order() {
        let args = vec![
            "analyze".to_string(),
            "--low-pass".to_string(),
            "10.5".to_string(),
            "--adc-quantize".to_string(),
            "2:0.0:3.0".to_string(),
            "--moving-average".to_string(),
            "3".to_string(),
        ];

        let filters = parse_filters(&args).expect("filters should parse");

        assert_eq!(
            filters,
            vec![
                FilterStep::LowPass(LowPassFilter { cutoff_hz: 10.5 }),
                FilterStep::AdcQuantize(AdcQuantizer {
                    bits: 2,
                    min_v: 0.0,
                    max_v: 3.0,
                }),
                FilterStep::MovingAverage(MovingAverageFilter { window_samples: 3 }),
            ]
        );
    }

    fn unique_plot_path(name: &str) -> String {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time should be available")
            .as_nanos();
        std::env::temp_dir()
            .join(format!("wra-{name}-{}-{nonce}.svg", std::process::id()))
            .to_string_lossy()
            .into_owned()
    }

    #[test]
    fn runs_analysis_with_explicit_cli_arguments() {
        let input_path = format!(
            "{}/../../examples/basic-waveform.csv",
            env!("CARGO_MANIFEST_DIR")
        );
        let output = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            input_path,
            "--time-column".to_string(),
            "time".to_string(),
            "--channels".to_string(),
            "input_v".to_string(),
            "--moving-average".to_string(),
            "2".to_string(),
            "--min".to_string(),
            "input_v:0.0".to_string(),
            "--max".to_string(),
            "input_v:5.5".to_string(),
        ])
        .expect("analysis should run");

        assert!(output.contains("Waveform Analysis Report"));
        assert!(output.contains("Overall: Pass"));
        assert!(output.contains("Measurements:"));
        assert!(output.contains("max_voltage_input_v"));
    }

    #[test]
    fn runs_analysis_with_adc_quantization_before_criteria() {
        let input_path = format!(
            "{}/../../examples/basic-waveform.csv",
            env!("CARGO_MANIFEST_DIR")
        );
        let output = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            input_path,
            "--time-column".to_string(),
            "time".to_string(),
            "--channels".to_string(),
            "input_v".to_string(),
            "--adc-quantize".to_string(),
            "2:0.0:3.0".to_string(),
            "--max".to_string(),
            "input_v:4.0".to_string(),
        ])
        .expect("analysis should run");

        assert!(output.contains("Overall: Pass"));
        assert!(output.contains("measured=3.000000 V required=4.000000 V"));
    }

    #[test]
    fn runs_analysis_with_config_and_json_output() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = format!("{manifest_dir}/../../examples/basic-waveform.csv");
        let config_path = format!("{manifest_dir}/../../examples/basic-config.toml");

        let output = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            input_path,
            "--config".to_string(),
            config_path,
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("analysis should run");

        assert!(output.contains("\"overall_outcome\": \"pass\""));
        assert!(output.contains("\"measurements\""));
        assert!(output.contains("\"measurement_id\""));
        assert!(output.contains("\"criterion_id\": \"input_max_voltage\""));
    }

    #[test]
    fn renders_2d_plot_to_svg_file() {
        let input_path = format!(
            "{}/../../examples/basic-waveform.csv",
            env!("CARGO_MANIFEST_DIR")
        );
        let output_path = unique_plot_path("plot-2d");

        let output = run(vec![
            "plot".to_string(),
            "--input".to_string(),
            input_path,
            "--time-column".to_string(),
            "time".to_string(),
            "--channels".to_string(),
            "input_v,output_v".to_string(),
            "--output".to_string(),
            output_path.clone(),
        ])
        .expect("plot should render");

        let svg = fs::read_to_string(&output_path).expect("svg should be written");
        assert!(output.contains("Plot written to"));
        assert!(svg.contains("<svg"));
        assert!(svg.contains("input_v"));
        assert!(svg.contains("output_v"));
        let _ = fs::remove_file(output_path);
    }

    #[test]
    fn renders_2d_plot_with_configured_evidence_overlays() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = format!("{manifest_dir}/../../tests/fixtures/dropout_event.csv");
        let config_path =
            format!("{manifest_dir}/../../tests/configs/transient-event-dropout-fail.toml");
        let output_path = unique_plot_path("plot-evidence");

        run(vec![
            "plot".to_string(),
            "--input".to_string(),
            input_path,
            "--config".to_string(),
            config_path,
            "--output".to_string(),
            output_path.clone(),
        ])
        .expect("annotated plot should render");

        let svg = fs::read_to_string(&output_path).expect("svg should be written");
        assert!(svg.contains("Evidence status: FAIL"));
        assert!(svg.contains("supply_dropout_max_1ms threshold 2.500000 V"));
        assert!(svg.contains("FAIL supply_dropout_max_1ms sample_index=3"));
        let _ = fs::remove_file(output_path);
    }

    #[test]
    fn renders_3d_plot_with_z_column_to_svg_file() {
        let input_path = format!(
            "{}/../../tests/fixtures/plot_three_axis.csv",
            env!("CARGO_MANIFEST_DIR")
        );
        let output_path = unique_plot_path("plot-3d");

        run(vec![
            "plot".to_string(),
            "--input".to_string(),
            input_path,
            "--time-column".to_string(),
            "time_s".to_string(),
            "--channels".to_string(),
            "signal_v".to_string(),
            "--z-column".to_string(),
            "temperature_c".to_string(),
            "--output".to_string(),
            output_path.clone(),
            "--title".to_string(),
            "Three Axis Validation Plot".to_string(),
        ])
        .expect("3d plot should render");

        let svg = fs::read_to_string(&output_path).expect("svg should be written");
        assert!(svg.contains("<svg"));
        assert!(svg.contains("Three Axis Validation Plot"));
        assert!(svg.contains("signal_v vs temperature_c"));
        let _ = fs::remove_file(output_path);
    }

    #[test]
    fn plot_reports_missing_z_column() {
        let input_path = format!(
            "{}/../../examples/basic-waveform.csv",
            env!("CARGO_MANIFEST_DIR")
        );
        let output_path = unique_plot_path("plot-missing-z");

        let error = run(vec![
            "plot".to_string(),
            "--input".to_string(),
            input_path,
            "--time-column".to_string(),
            "time".to_string(),
            "--channels".to_string(),
            "input_v".to_string(),
            "--z-column".to_string(),
            "temperature_c".to_string(),
            "--output".to_string(),
            output_path,
        ])
        .expect_err("missing z column should fail");

        assert!(error.contains("missing required column `temperature_c`"));
    }

    #[test]
    fn invalid_config_syntax_returns_clear_error() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = format!("{manifest_dir}/../../examples/basic-waveform.csv");
        let config_path = format!("{manifest_dir}/../../tests/configs/invalid-bad-syntax.toml");

        let error = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            input_path,
            "--config".to_string(),
            config_path.clone(),
        ])
        .expect_err("bad config should fail");

        assert!(error.contains(&format!("failed to parse config `{config_path}`")));
    }

    #[test]
    fn invalid_config_semantics_return_clear_errors() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = format!("{manifest_dir}/../../examples/basic-waveform.csv");

        for (config_file, expected) in [
            (
                "invalid-empty-channels.toml",
                "config input.channels must include at least one channel",
            ),
            (
                "invalid-missing-criteria.toml",
                "config must include at least one [[criteria]] entry",
            ),
            (
                "invalid-unsupported-criterion.toml",
                "unsupported criterion type `aerospace_magic`",
            ),
            (
                "invalid-missing-transient-event-field.toml",
                "invalid parameter `criteria.max_duration_s`",
            ),
            (
                "invalid-missing-adc-field.toml",
                "invalid config filters: invalid parameter `filters.max_v`",
            ),
            (
                "invalid-negative-tolerance.toml",
                "invalid config: invalid parameter `tolerances.time_s`",
            ),
            (
                "invalid-mixed-legacy-dsl-criterion.toml",
                "invalid config: invalid parameter `criteria.mixed_shape`",
            ),
            (
                "invalid-dsl-unknown-operator.toml",
                "invalid config: invalid parameter `criteria.dsl_bad_operator.requirement.operator`",
            ),
            (
                "invalid-dsl-missing-requirement-unit.toml",
                "invalid config: invalid parameter `criteria.dsl_missing_unit.requirement.unit`",
            ),
        ] {
            let config_path = format!("{manifest_dir}/../../tests/configs/{config_file}");
            let error = run(vec![
                "analyze".to_string(),
                "--input".to_string(),
                input_path.clone(),
                "--config".to_string(),
                config_path,
            ])
            .expect_err("invalid config should fail");

            assert!(
                error.contains(expected),
                "expected `{error}` to contain `{expected}`"
            );
        }
    }
}
