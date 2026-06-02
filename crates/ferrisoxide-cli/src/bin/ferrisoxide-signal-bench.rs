use std::env;
use std::fs;
use std::process::ExitCode;
use std::time::Instant;

use ferrisoxide_core::analysis::evaluate_criteria_with_measurements;
use ferrisoxide_core::config::AnalysisConfig;
use ferrisoxide_core::csv::{SimpleCsvParser, WaveformParser};
use ferrisoxide_core::feature::evaluate_feature_transforms;
use ferrisoxide_core::filter::apply_filter_chain;
use ferrisoxide_core::report::{AnalysisReport, ReportEvidenceContext};

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

    let input_path = value_after(&args, "--input").ok_or("missing --input <path>")?;
    let config_path = value_after(&args, "--config").ok_or("missing --config <path>")?;
    let iterations = value_after(&args, "--iterations")
        .unwrap_or("1")
        .parse::<usize>()
        .map_err(|_| "invalid --iterations value".to_string())?;
    if iterations == 0 {
        return Err("--iterations must be greater than zero".to_string());
    }

    let config_input = fs::read_to_string(config_path)
        .map_err(|error| format!("failed to read `{config_path}`: {error}"))?;
    let config = toml::from_str::<AnalysisConfig>(&config_input)
        .map_err(|error| format!("failed to parse config `{config_path}`: {error}"))?;
    config
        .validate()
        .map_err(|error| format!("invalid config tolerances: {error}"))?;
    if config.input.channels.is_empty() {
        return Err("config input.channels must include at least one channel".to_string());
    }
    if config.criteria.is_empty() {
        return Err("config must include at least one [[criteria]] entry".to_string());
    }

    let options = config.csv_options();
    let filters = config
        .filters()
        .map_err(|error| format!("invalid config filters: {error}"))?;
    let feature_transforms = config
        .feature_transforms()
        .map_err(|error| format!("invalid config feature transforms: {error}"))?;
    let criteria = config
        .criteria()
        .map_err(|error| format!("invalid config criteria: {error}"))?;
    let parser = SimpleCsvParser;

    let mut totals = TimingTotals::default();
    let mut samples = 0;
    let mut channels = 0;
    let mut report_bytes = 0;

    for _ in 0..iterations {
        let total_start = Instant::now();

        let read_start = Instant::now();
        let csv_input = fs::read_to_string(input_path)
            .map_err(|error| format!("failed to read `{input_path}`: {error}"))?;
        totals.read_ms += elapsed_ms(read_start);

        let parse_start = Instant::now();
        let waveform = parser
            .parse_str(&csv_input, &options)
            .map_err(|error| format!("failed to parse waveform: {error}"))?
            .with_source_name(input_path.to_string())
            .with_metadata_context(&config.metadata)
            .with_tolerance_policy(config.tolerances);
        totals.parse_ms += elapsed_ms(parse_start);

        let transform_start = Instant::now();
        let waveform = apply_filter_chain(&waveform, &filters)
            .map_err(|error| format!("filter pipeline failed: {error}"))?;
        totals.transform_ms += elapsed_ms(transform_start);

        let criteria_start = Instant::now();
        let feature_records = evaluate_feature_transforms(&waveform, &feature_transforms)
            .map_err(|error| format!("feature analysis failed: {error}"))?;
        let evaluation =
            evaluate_criteria_with_measurements(&waveform, &criteria, config.tolerances)
                .map_err(|error| format!("analysis failed: {error}"))?;
        totals.criteria_ms += elapsed_ms(criteria_start);

        let report_start = Instant::now();
        let report = AnalysisReport {
            input_name: input_path.to_string(),
            waveform_metadata: waveform.metadata.clone(),
            evidence_context: ReportEvidenceContext::engineering_validation(config.tolerances),
            measurements: evaluation.measurements,
            feature_records,
            event_records: Vec::new(),
            event_validations: Vec::new(),
            results: evaluation.results,
        };
        let rendered = report
            .render_json_pretty()
            .map_err(|error| format!("failed to render json report: {error}"))?;
        totals.report_ms += elapsed_ms(report_start);

        totals.total_ms += elapsed_ms(total_start);
        samples = waveform.sample_count();
        channels = waveform.channels.len();
        report_bytes = rendered.len();
    }

    Ok(format!(
        "\
ferrisoxide_signal_benchmark
input={input_path}
config={config_path}
iterations={iterations}
samples={samples}
channels={channels}
report_bytes={report_bytes}
read_ms_avg={:.3}
parse_ms_avg={:.3}
transform_ms_avg={:.3}
criteria_ms_avg={:.3}
report_ms_avg={:.3}
total_ms_avg={:.3}",
        totals.read_ms / iterations as f64,
        totals.parse_ms / iterations as f64,
        totals.transform_ms / iterations as f64,
        totals.criteria_ms / iterations as f64,
        totals.report_ms / iterations as f64,
        totals.total_ms / iterations as f64,
    ))
}

#[derive(Default)]
struct TimingTotals {
    read_ms: f64,
    parse_ms: f64,
    transform_ms: f64,
    criteria_ms: f64,
    report_ms: f64,
    total_ms: f64,
}

fn elapsed_ms(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}

fn value_after<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|window| window[0] == flag)
        .map(|window| window[1].as_str())
}

fn help() -> String {
    [
        "FerrisOxide Signal benchmark helper",
        "",
        "Usage:",
        "  ferrisoxide-signal-bench --input <csv> --config <toml> --iterations 3",
        "",
        "The helper reports average read, parse, transform, criteria, report, and total timings.",
    ]
    .join("\n")
}
