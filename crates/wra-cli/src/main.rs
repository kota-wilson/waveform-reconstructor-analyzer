use std::env;
use std::fs;
use std::process::ExitCode;

use wra_core::analysis::evaluate_criteria;
use wra_core::config::AnalysisConfig;
use wra_core::criteria::Criterion;
use wra_core::csv::{CsvParseOptions, SimpleCsvParser, WaveformParser};
use wra_core::filter::{
    apply_filter_chain, AdcQuantizer, FilterStep, LowPassFilter, MovingAverageFilter,
};
use wra_core::report::AnalysisReport;

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

    if args.first().map(String::as_str) != Some("analyze") {
        return Err("expected subcommand `analyze`".to_string());
    }

    let input_path = value_after(&args, "--input").ok_or("missing --input <path>")?;
    let output_format = value_after(&args, "--format").unwrap_or("text");
    let config = load_config(&args)?;
    let (options, filters, criteria) = match config {
        Some(config) => (
            config.csv_options(),
            config
                .filters()
                .map_err(|error| format!("invalid config filters: {error}"))?,
            config
                .criteria()
                .map_err(|error| format!("invalid config criteria: {error}"))?,
        ),
        None => {
            let time_column = value_after(&args, "--time-column").unwrap_or("time");
            let channels =
                value_after(&args, "--channels").ok_or("missing --channels <name[,name]>")?;
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
                parse_filters(&args)?,
                parse_criteria(&args)?,
            )
        }
    };

    let input = fs::read_to_string(input_path)
        .map_err(|error| format!("failed to read `{input_path}`: {error}"))?;
    let parser = SimpleCsvParser;
    let mut waveform = parser
        .parse_str(&input, &options)
        .map_err(|error| format!("failed to parse waveform: {error}"))?;

    waveform = apply_filter_chain(&waveform, &filters)
        .map_err(|error| format!("filter pipeline failed: {error}"))?;

    let results = evaluate_criteria(&waveform, &criteria)
        .map_err(|error| format!("analysis failed: {error}"))?;
    let report = AnalysisReport {
        input_name: input_path.to_string(),
        results,
    };

    render_report(&report, output_format)
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
        "",
        "ADC quantization syntax: --adc-quantize bits:min_v:max_v",
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
        assert!(output.contains("\"criterion_id\": \"input_max_voltage\""));
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
