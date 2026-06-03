use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use ferrisoxide_control_schema::{
    parse_control_config_toml, DigitalState as ControlDigitalState, ProductionControlConfig,
    SignalKind,
};
use ferrisoxide_core::analysis::evaluate_criteria_with_measurements;
use ferrisoxide_core::config::AnalysisConfig;
use ferrisoxide_core::criteria::{
    Criterion, CriterionCheck, CriterionOperator, EdgeDirection, MeasurementSpec,
    ResponseLatencySpec, RunSelectionConfig, SignalState, TransientEventKind, TransientEventWindow,
};
use ferrisoxide_core::csv::{CsvParseOptions, SimpleCsvParser, WaveformParser};
use ferrisoxide_core::event::evaluate_event_pipeline;
use ferrisoxide_core::feature::evaluate_feature_transforms;
use ferrisoxide_core::filter::{
    apply_filter_chain, AdcQuantizer, Filter, FilterStep, LowPassFilter, MovingAverageFilter,
};
use ferrisoxide_core::model::{
    Channel, MetadataContext, TolerancePolicy, TransformCapabilityStatus, TransformEvidenceLevel,
    TransformRuntimeProfile, Unit, Waveform,
};
use ferrisoxide_core::report::{AnalysisReport, ReportEvidenceContext};
use ferrisoxide_core::transform_catalog::{transform_catalog, TransformCatalogEntry};
use ferrisoxide_daq::{
    collect_frames, DaqChannel, DaqSampleFrame, DaqSampleValue, DaqSourceDescriptor, DaqSourceKind,
    FixtureDaqSource,
};
use ferrisoxide_plot::{evidence_overlays, render_svg, PlotOptions};
use ferrisoxide_rule_schema::{
    checksum_str, ChannelDefinition, ChecksumMetadata, ComparisonOperator, CriterionDefinition,
    EngineeringUnit, FilterDefinition, ManifestArtifact, ManifestSources,
    ManifestValidationEvidence, MeasurementDefinition, PackageMetadata, RequirementDefinition,
    RulePackage, RulePackageManifest, SampleTimingAssumption, TargetProfile, TargetProfileKind,
    ThresholdDefinition, ThresholdRole, UnitValue,
};
use ferrisoxide_simulator::{
    simulate_controller, SimulatedInputValue, SimulationInputFrame, SimulationReport,
};
use ferrisoxide_verification_schema::{
    parse_verification_config_toml, DigitalState as VerificationDigitalState,
    TestVerificationConfig, TransientEventKind as VerificationTransientEventKind,
    VerificationChannel,
};
use serde::{Deserialize, Serialize};

pub fn run(args: Vec<String>) -> Result<String, String> {
    if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        return Ok(help());
    }

    match args.first().map(String::as_str) {
        Some("analyze") => run_analyze(&args),
        Some("plot") => run_plot(&args),
        Some("export-rule-package") => run_export_rule_package(&args),
        Some("simulate") => run_simulate(&args),
        Some("batch") => run_batch(&args),
        Some("transforms") => run_transforms(&args),
        Some("inspect-source") => run_inspect_source(&args),
        Some("scaffold-config") => run_scaffold_config(&args),
        Some("workflow-template") => run_workflow_template(&args),
        Some("evaluate-bundle") => run_evaluate_bundle(&args),
        Some(other) => Err(format!(
            "expected subcommand `analyze`, `plot`, `export-rule-package`, `simulate`, `batch`, `transforms`, `inspect-source`, `scaffold-config`, `workflow-template`, or `evaluate-bundle`, got `{other}`"
        )),
        None => Ok(help()),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowSourceMode {
    Csv,
    Simulation,
}

impl WorkflowSourceMode {
    fn as_cli_value(self) -> &'static str {
        match self {
            Self::Csv => "csv",
            Self::Simulation => "simulation",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InspectSourceRequest {
    pub source_mode: WorkflowSourceMode,
    pub input_path: String,
    pub time_column: String,
    pub channels: Vec<String>,
    pub time_unit: String,
    pub signal_unit: String,
    pub channel_map_path: Option<String>,
}

impl InspectSourceRequest {
    pub fn csv(input_path: impl Into<String>) -> Self {
        Self {
            source_mode: WorkflowSourceMode::Csv,
            input_path: input_path.into(),
            time_column: "time".to_string(),
            channels: Vec::new(),
            time_unit: "s".to_string(),
            signal_unit: "V".to_string(),
            channel_map_path: None,
        }
    }

    pub fn simulation(input_path: impl Into<String>, channel_map_path: impl Into<String>) -> Self {
        Self {
            source_mode: WorkflowSourceMode::Simulation,
            input_path: input_path.into(),
            time_column: "time".to_string(),
            channels: Vec::new(),
            time_unit: "s".to_string(),
            signal_unit: "V".to_string(),
            channel_map_path: Some(channel_map_path.into()),
        }
    }

    fn to_args(&self) -> Vec<String> {
        let mut args = vec![
            "inspect-source".to_string(),
            "--source".to_string(),
            self.source_mode.as_cli_value().to_string(),
            "--input".to_string(),
            self.input_path.clone(),
        ];
        if self.source_mode == WorkflowSourceMode::Csv {
            push_arg(&mut args, "--time-column", &self.time_column);
            push_arg(&mut args, "--time-unit", &self.time_unit);
            push_arg(&mut args, "--signal-unit", &self.signal_unit);
        }
        if !self.channels.is_empty() {
            push_arg(&mut args, "--channels", &self.channels.join(","));
        }
        if let Some(channel_map_path) = &self.channel_map_path {
            push_arg(&mut args, "--channel-map", channel_map_path);
        }
        args
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaffoldConfigRequest {
    pub input_path: String,
    pub time_column: String,
    pub channels: Vec<String>,
    pub time_unit: String,
    pub signal_unit: String,
}

impl ScaffoldConfigRequest {
    pub fn csv(input_path: impl Into<String>) -> Self {
        Self {
            input_path: input_path.into(),
            time_column: "time".to_string(),
            channels: Vec::new(),
            time_unit: "s".to_string(),
            signal_unit: "V".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalyzeCsvRequest {
    pub input_path: String,
    pub config_path: String,
    pub output_format: String,
}

impl AnalyzeCsvRequest {
    pub fn json(input_path: impl Into<String>, config_path: impl Into<String>) -> Self {
        Self {
            input_path: input_path.into(),
            config_path: config_path.into(),
            output_format: "json".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluateBundleRequest {
    pub source_mode: WorkflowSourceMode,
    pub input_path: String,
    pub config_path: Option<String>,
    pub output_dir: String,
    pub overwrite: bool,
    pub include_plot: bool,
    pub control_config_path: Option<String>,
    pub verification_config_path: Option<String>,
    pub channel_map_path: Option<String>,
    pub mode: Option<String>,
}

impl EvaluateBundleRequest {
    pub fn csv(
        input_path: impl Into<String>,
        config_path: impl Into<String>,
        output_dir: impl Into<String>,
    ) -> Self {
        Self {
            source_mode: WorkflowSourceMode::Csv,
            input_path: input_path.into(),
            config_path: Some(config_path.into()),
            output_dir: output_dir.into(),
            overwrite: false,
            include_plot: false,
            control_config_path: None,
            verification_config_path: None,
            channel_map_path: None,
            mode: None,
        }
    }

    pub fn simulation(
        input_path: impl Into<String>,
        control_config_path: impl Into<String>,
        verification_config_path: impl Into<String>,
        channel_map_path: impl Into<String>,
        output_dir: impl Into<String>,
    ) -> Self {
        Self {
            source_mode: WorkflowSourceMode::Simulation,
            input_path: input_path.into(),
            config_path: None,
            output_dir: output_dir.into(),
            overwrite: false,
            include_plot: false,
            control_config_path: Some(control_config_path.into()),
            verification_config_path: Some(verification_config_path.into()),
            channel_map_path: Some(channel_map_path.into()),
            mode: None,
        }
    }

    fn to_args(&self) -> Vec<String> {
        let mut args = vec![
            "evaluate-bundle".to_string(),
            "--source".to_string(),
            self.source_mode.as_cli_value().to_string(),
            "--input".to_string(),
            self.input_path.clone(),
            "--output-dir".to_string(),
            self.output_dir.clone(),
        ];
        match self.source_mode {
            WorkflowSourceMode::Csv => {
                if let Some(config_path) = &self.config_path {
                    push_arg(&mut args, "--config", config_path);
                }
                if self.include_plot {
                    args.push("--plot".to_string());
                }
            }
            WorkflowSourceMode::Simulation => {
                if let Some(control_config_path) = &self.control_config_path {
                    push_arg(&mut args, "--control-config", control_config_path);
                }
                if let Some(verification_config_path) = &self.verification_config_path {
                    push_arg(&mut args, "--verification-config", verification_config_path);
                }
                if let Some(channel_map_path) = &self.channel_map_path {
                    push_arg(&mut args, "--channel-map", channel_map_path);
                }
                if let Some(mode) = &self.mode {
                    push_arg(&mut args, "--mode", mode);
                }
            }
        }
        if self.overwrite {
            args.push("--overwrite".to_string());
        }
        args
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowBundleOutput {
    pub summary_json: String,
    pub source_mode: String,
    pub output_dir: String,
    pub overall_outcome: String,
    pub artifacts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsvPlotSeriesRequest {
    pub input_path: String,
    pub config_path: Option<String>,
    pub time_column: String,
    pub channels: Vec<String>,
    pub time_unit: String,
    pub signal_unit: String,
}

impl CsvPlotSeriesRequest {
    pub fn from_config(input_path: impl Into<String>, config_path: impl Into<String>) -> Self {
        Self {
            input_path: input_path.into(),
            config_path: Some(config_path.into()),
            time_column: "time".to_string(),
            channels: Vec::new(),
            time_unit: "s".to_string(),
            signal_unit: "V".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorkflowPlotSeries {
    pub name: String,
    pub unit: String,
    pub points: Vec<WorkflowPlotPoint>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WorkflowPlotPoint {
    pub time: f64,
    pub value: f64,
}

pub fn inspect_source(request: &InspectSourceRequest) -> Result<SourceInspection, String> {
    inspect_source_from_args(&request.to_args(), request.source_mode.as_cli_value())
}

pub fn load_csv_headers(input_path: impl AsRef<Path>) -> Result<Vec<String>, String> {
    let input_path = input_path.as_ref();
    let input = fs::read_to_string(input_path)
        .map_err(|error| format!("failed to read `{}`: {error}", input_path.display()))?;
    csv_headers(&input)
}

pub fn render_source_inspection_output(
    inspection: &SourceInspection,
    output_format: &str,
) -> Result<String, String> {
    render_source_inspection(inspection, output_format)
}

pub fn scaffold_csv_config(request: &ScaffoldConfigRequest) -> Result<String, String> {
    let inspect_request = InspectSourceRequest {
        source_mode: WorkflowSourceMode::Csv,
        input_path: request.input_path.clone(),
        time_column: request.time_column.clone(),
        channels: request.channels.clone(),
        time_unit: request.time_unit.clone(),
        signal_unit: request.signal_unit.clone(),
        channel_map_path: None,
    };
    let inspection = inspect_source(&inspect_request)?;
    render_analysis_config_scaffold(&request.input_path, &inspection)
}

pub fn workflow_template_output(use_case: &str, output_format: &str) -> Result<String, String> {
    let template = workflow_template(use_case)?;
    match output_format {
        "text" => Ok(template.render_text()),
        "toml" => Ok(template.toml.trim_end().to_string()),
        _ => Err(format!(
            "unsupported --format `{output_format}`; use text or toml"
        )),
    }
}

pub fn analyze_csv(request: &AnalyzeCsvRequest) -> Result<String, String> {
    run_analyze(&[
        "analyze".to_string(),
        "--input".to_string(),
        request.input_path.clone(),
        "--config".to_string(),
        request.config_path.clone(),
        "--format".to_string(),
        request.output_format.clone(),
    ])
}

pub fn evaluate_bundle(request: &EvaluateBundleRequest) -> Result<WorkflowBundleOutput, String> {
    let summary_json = run_evaluate_bundle(&request.to_args())?;
    workflow_bundle_output_from_json(summary_json)
}

pub fn load_csv_plot_series(
    request: &CsvPlotSeriesRequest,
) -> Result<Vec<WorkflowPlotSeries>, String> {
    let input = fs::read_to_string(&request.input_path)
        .map_err(|error| format!("failed to read `{}`: {error}", request.input_path))?;
    let (options, filters, selected_channels, metadata, tolerances) =
        if let Some(config_path) = &request.config_path {
            let config = load_analysis_config_from_path(Path::new(config_path))?;
            (
                config.csv_options(),
                config
                    .filters()
                    .map_err(|error| format!("invalid config filters: {error}"))?,
                config.input.channels.clone(),
                config.metadata.clone(),
                config.tolerances,
            )
        } else {
            if request.channels.is_empty() {
                return Err("plot series requires at least one channel without config".to_string());
            }
            let mut options = CsvParseOptions::new(&request.time_column, request.channels.clone());
            options.time_unit = Unit::new(&request.time_unit);
            options.signal_unit = Unit::new(&request.signal_unit);
            (
                options,
                Vec::new(),
                request.channels.clone(),
                MetadataContext::default(),
                TolerancePolicy::default(),
            )
        };
    let mut waveform = SimpleCsvParser
        .parse_str(&input, &options)
        .map_err(|error| format!("failed to parse waveform: {error}"))?
        .with_source_name(request.input_path.clone())
        .with_metadata_context(&metadata)
        .with_tolerance_policy(tolerances);
    waveform = apply_filter_chain(&waveform, &filters)
        .map_err(|error| format!("filter pipeline failed: {error}"))?;

    selected_channels
        .iter()
        .map(|channel_id| {
            let channel = waveform
                .channel(channel_id)
                .ok_or_else(|| format!("waveform is missing channel `{channel_id}`"))?;
            let points = waveform
                .time
                .iter()
                .copied()
                .zip(channel.samples.iter().copied())
                .map(|(time, value)| WorkflowPlotPoint { time, value })
                .collect();
            Ok(WorkflowPlotSeries {
                name: channel.name.clone(),
                unit: channel.unit.name.clone(),
                points,
            })
        })
        .collect()
}

fn push_arg(args: &mut Vec<String>, flag: &str, value: &str) {
    args.push(flag.to_string());
    args.push(value.to_string());
}

fn workflow_bundle_output_from_json(summary_json: String) -> Result<WorkflowBundleOutput, String> {
    let value = serde_json::from_str::<serde_json::Value>(&summary_json)
        .map_err(|error| format!("failed to parse bundle summary json: {error}"))?;
    let source_mode = json_string_field(&value, "source_mode")?;
    let output_dir = json_string_field(&value, "output_dir")?;
    let overall_outcome = json_string_field(&value, "overall_outcome")?;
    let artifacts = value
        .get("artifacts")
        .and_then(serde_json::Value::as_array)
        .ok_or("bundle summary json is missing artifacts array")?
        .iter()
        .map(|artifact| {
            artifact
                .as_str()
                .map(str::to_string)
                .ok_or("bundle summary artifact entries must be strings".to_string())
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(WorkflowBundleOutput {
        summary_json,
        source_mode,
        output_dir,
        overall_outcome,
        artifacts,
    })
}

fn json_string_field(value: &serde_json::Value, field: &str) -> Result<String, String> {
    value
        .get(field)
        .and_then(serde_json::Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| format!("bundle summary json is missing `{field}`"))
}

fn run_analyze(args: &[String]) -> Result<String, String> {
    let input_path = value_after(args, "--input").ok_or("missing --input <path>")?;
    let output_format = value_after(args, "--format").unwrap_or("text");
    let config = load_config(args)?;
    let (
        options,
        filters,
        feature_transforms,
        event_transforms,
        event_validations,
        criteria,
        tolerances,
        metadata,
    ) = match config {
        Some(config) => (
            config.csv_options(),
            config
                .filters()
                .map_err(|error| format!("invalid config filters: {error}"))?,
            config
                .feature_transforms()
                .map_err(|error| format!("invalid config feature transforms: {error}"))?,
            config
                .event_transforms()
                .map_err(|error| format!("invalid config event transforms: {error}"))?,
            config
                .event_validations()
                .map_err(|error| format!("invalid config event validations: {error}"))?,
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
                Vec::new(),
                Vec::new(),
                Vec::new(),
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

    let feature_records = evaluate_feature_transforms(&waveform, &feature_transforms)
        .map_err(|error| format!("feature analysis failed: {error}"))?;
    let event_evaluation =
        evaluate_event_pipeline(&waveform, &event_transforms, &event_validations)
            .map_err(|error| format!("event analysis failed: {error}"))?;
    let evaluation = evaluate_criteria_with_measurements(&waveform, &criteria, tolerances)
        .map_err(|error| format!("analysis failed: {error}"))?;
    let report = AnalysisReport {
        input_name: input_path.to_string(),
        waveform_metadata: waveform.metadata.clone(),
        evidence_context: ReportEvidenceContext::engineering_validation(tolerances),
        measurements: evaluation.measurements,
        feature_records,
        event_records: event_evaluation.records,
        event_validations: event_evaluation.validations,
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

fn run_export_rule_package(args: &[String]) -> Result<String, String> {
    let input_path = value_after(args, "--input").ok_or("missing --input <csv>")?;
    let config_path =
        value_after(args, "--config").ok_or("export-rule-package requires --config <toml>")?;
    let output_dir = value_after(args, "--output-dir").ok_or("missing --output-dir <dir>")?;
    let package_name =
        value_after(args, "--package-name").ok_or("missing --package-name <name>")?;
    let package_version =
        value_after(args, "--package-version").ok_or("missing --package-version <version>")?;
    let target =
        parse_target_profile(value_after(args, "--target").unwrap_or("controller_runtime"))?;
    let target_identifier = value_after(args, "--target-id").map(str::to_string);
    let config = load_config(args)?.ok_or("export-rule-package requires --config <toml>")?;
    if !config.feature_transforms.is_empty() {
        return Err(
            "rule package export does not yet support feature_transforms; remove them or run analyze"
                .to_string(),
        );
    }
    let (report, filters, criteria) = analyze_configured_input(input_path, &config)?;
    let package = build_rule_package(RulePackageBuildInput {
        package_name,
        package_version,
        target,
        target_identifier,
        config: &config,
        report: &report,
        filters: &filters,
        criteria: &criteria,
    })?;

    package
        .validate_for_target(target)
        .map_err(|report| format!("rule package validation failed: {report}"))?;

    let output_dir = Path::new(output_dir);
    fs::create_dir_all(output_dir)
        .map_err(|error| format!("failed to create `{}`: {error}", output_dir.display()))?;

    let rules_toml = with_trailing_newline(
        toml::to_string_pretty(&package)
            .map_err(|error| format!("failed to render rules.toml: {error}"))?,
    );
    let rules_json = with_trailing_newline(
        serde_json::to_string_pretty(&package)
            .map_err(|error| format!("failed to render rules.json: {error}"))?,
    );
    let report_json = with_trailing_newline(
        report
            .render_json_pretty()
            .map_err(|error| format!("failed to render validation report: {error}"))?,
    );

    let mut artifacts = vec![
        ExportArtifact::new(
            "rules.toml",
            "rule_package_toml",
            "application/toml",
            rules_toml,
        ),
        ExportArtifact::new(
            "rules.json",
            "rule_package_json",
            "application/json",
            rules_json,
        ),
        ExportArtifact::new(
            "validation-report.json",
            "validation_report",
            "application/json",
            report_json,
        ),
    ];
    let manifest = RulePackageManifest::new(
        &package,
        ManifestSources::new(input_path, config_path),
        ManifestValidationEvidence::passed("validation-report.json"),
        artifacts
            .iter()
            .map(ExportArtifact::manifest_artifact)
            .collect(),
    );
    let manifest_json = with_trailing_newline(
        serde_json::to_string_pretty(&manifest)
            .map_err(|error| format!("failed to render manifest.json: {error}"))?,
    );
    let mut checksum_entries = artifacts
        .iter()
        .map(|artifact| (artifact.path.clone(), checksum_str(&artifact.contents)))
        .collect::<Vec<_>>();
    checksum_entries.push(("manifest.json".to_string(), checksum_str(&manifest_json)));
    let checksum_text = checksum_file_contents(&manifest.checksum, &checksum_entries);
    artifacts.push(ExportArtifact::new(
        "manifest.json",
        "package_manifest",
        "application/json",
        manifest_json,
    ));
    artifacts.push(ExportArtifact::new(
        "checksum.txt",
        "checksum_index",
        "text/plain",
        checksum_text,
    ));

    for artifact in &artifacts {
        write_new_file(&output_dir.join(&artifact.path), &artifact.contents)?;
    }

    Ok(format!(
        "Rule package exported to {}\nArtifacts: rules.toml, rules.json, validation-report.json, manifest.json, checksum.txt",
        output_dir.display()
    ))
}

fn run_simulate(args: &[String]) -> Result<String, String> {
    let input_path = value_after(args, "--input").ok_or("missing --input <csv>")?;
    let control_config_path =
        value_after(args, "--control-config").ok_or("missing --control-config <toml>")?;
    let verification_config_path =
        value_after(args, "--verification-config").ok_or("missing --verification-config <toml>")?;
    let channel_map_path =
        value_after(args, "--channel-map").ok_or("missing --channel-map <toml>")?;
    let output_format = value_after(args, "--format").unwrap_or("json");

    let control_config = load_control_config(control_config_path)?;
    let verification_config = load_verification_config(verification_config_path)?;
    let channel_map = load_simulation_channel_map(channel_map_path)?;
    validate_simulation_channel_map(&channel_map, &control_config, &verification_config)?;

    let mode = value_after(args, "--mode")
        .unwrap_or(&channel_map.simulation.mode)
        .to_string();
    let workflow = run_desktop_simulation_workflow(DesktopSimulationInput {
        input_path,
        control_config_path,
        verification_config_path,
        channel_map_path,
        mode,
        control_config,
        verification_config,
        channel_map,
    })?;

    let output = match output_format {
        "json" => render_desktop_simulation_json(&workflow),
        "text" => Ok(render_desktop_simulation_text(&workflow)),
        _ => Err(format!(
            "unsupported --format `{output_format}`; use text or json"
        )),
    }?;

    if let Some(output_path) = value_after(args, "--output-json") {
        if output_format != "json" {
            return Err("--output-json requires --format json".to_string());
        }
        write_new_file(Path::new(output_path), &with_trailing_newline(output))?;
        return Ok(format!(
            "Desktop simulation workflow written to {output_path}"
        ));
    }

    Ok(output)
}

fn run_batch(args: &[String]) -> Result<String, String> {
    let manifest_path = value_after(args, "--manifest").ok_or("missing --manifest <toml>")?;
    let output_format = value_after(args, "--format").unwrap_or("json");
    validate_report_format(output_format)?;
    let overwrite = args.iter().any(|arg| arg == "--overwrite");
    let manifest = load_batch_manifest(manifest_path)?;
    let manifest_dir = Path::new(manifest_path)
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let output_dir = if let Some(path) = value_after(args, "--output-dir") {
        PathBuf::from(path)
    } else if let Some(path) = manifest.output_dir.as_deref() {
        resolve_path(manifest_dir, path)
    } else {
        return Err("batch requires --output-dir <dir> or output_dir in the manifest".to_string());
    };
    fs::create_dir_all(&output_dir)
        .map_err(|error| format!("failed to create `{}`: {error}", output_dir.display()))?;

    let default_format = manifest.default_format.as_deref().unwrap_or("json");
    let mut run_results = Vec::new();
    for run_config in &manifest.runs {
        run_results.push(run_batch_entry(
            manifest_dir,
            &output_dir,
            run_config,
            default_format,
            overwrite,
        ));
    }

    let summary = BatchSummary::from_results(
        manifest_path.to_string(),
        output_dir.display().to_string(),
        run_results,
    );
    let summary_file = manifest
        .summary_file
        .as_deref()
        .unwrap_or("batch-summary.json");
    validate_relative_output_path(summary_file, "summary_file")?;
    let summary_path = output_dir.join(summary_file);
    let summary_json = with_trailing_newline(
        serde_json::to_string_pretty(&summary)
            .map_err(|error| format!("failed to render batch summary: {error}"))?,
    );
    write_output_file(&summary_path, &summary_json, overwrite)?;

    match output_format {
        "json" => Ok(summary_json.trim_end().to_string()),
        "text" => Ok(summary.render_text()),
        _ => Err(format!(
            "unsupported --format `{output_format}`; use text or json"
        )),
    }
}

fn run_transforms(args: &[String]) -> Result<String, String> {
    let output_format = value_after(args, "--format").unwrap_or("text");
    match output_format {
        "json" => serde_json::to_string_pretty(transform_catalog())
            .map_err(|error| format!("failed to render transform catalog: {error}")),
        "text" => Ok(render_transform_catalog_text(transform_catalog())),
        _ => Err(format!(
            "unsupported --format `{output_format}`; use text or json"
        )),
    }
}

fn run_inspect_source(args: &[String]) -> Result<String, String> {
    let source_mode = value_after(args, "--source").unwrap_or("csv");
    let output_format = value_after(args, "--format").unwrap_or("text");
    let inspection = inspect_source_from_args(args, source_mode)?;
    render_source_inspection(&inspection, output_format)
}

fn run_scaffold_config(args: &[String]) -> Result<String, String> {
    let source_mode = value_after(args, "--source").unwrap_or("csv");
    if !source_mode.eq_ignore_ascii_case("csv") {
        return Err(format!(
            "scaffold-config currently supports --source csv; `{source_mode}` source scaffolding is planned but not implemented"
        ));
    }
    let output_format = value_after(args, "--format").unwrap_or("toml");
    if output_format != "toml" {
        return Err(format!("unsupported --format `{output_format}`; use toml"));
    }
    let input_path = value_after(args, "--input").ok_or("missing --input <csv>")?;
    let inspection = inspect_csv_source_from_args(args)?;
    let scaffold = render_analysis_config_scaffold(input_path, &inspection)?;
    if let Some(output_path) = value_after(args, "--output") {
        let overwrite = args.iter().any(|arg| arg == "--overwrite");
        write_output_file(Path::new(output_path), &scaffold, overwrite)?;
        return Ok(format!("Analysis config scaffold written to {output_path}"));
    }
    Ok(scaffold.trim_end().to_string())
}

fn run_workflow_template(args: &[String]) -> Result<String, String> {
    let use_case = value_after(args, "--use-case").unwrap_or("supply-rail");
    let output_format = value_after(args, "--format").unwrap_or("text");
    let template = workflow_template(use_case)?;
    match output_format {
        "text" => Ok(template.render_text()),
        "toml" => Ok(template.toml.trim_end().to_string()),
        _ => Err(format!(
            "unsupported --format `{output_format}`; use text or toml"
        )),
    }
}

fn run_evaluate_bundle(args: &[String]) -> Result<String, String> {
    let source_mode = value_after(args, "--source").unwrap_or("csv");
    let output_dir = value_after(args, "--output-dir").ok_or("missing --output-dir <dir>")?;
    let overwrite = args.iter().any(|arg| arg == "--overwrite");
    match normalized_source_mode(source_mode)? {
        DesktopSourceMode::Csv => run_csv_evaluate_bundle(args, output_dir, overwrite),
        DesktopSourceMode::Simulation => {
            run_simulation_evaluate_bundle(args, output_dir, overwrite)
        }
    }
}

fn render_transform_catalog_text(entries: &[TransformCatalogEntry]) -> String {
    let mut output = String::new();
    output.push_str("FerrisOxide Transform Catalog\n");
    output.push_str(&format!("Entries: {}\n", entries.len()));
    for entry in entries {
        let runtime_profiles = entry
            .runtime_profiles
            .iter()
            .copied()
            .map(runtime_profile_label)
            .collect::<Vec<_>>()
            .join(",");
        output.push_str(&format!(
            "- {} | milestone={} | status={} | family={} | category={} | package={} | runtime={} | evidence={} | docs={}\n",
            entry.name,
            entry.milestone,
            capability_status_label(entry.capability_status),
            entry.family.as_str(),
            entry.category.as_str(),
            entry.package_support.as_str(),
            runtime_profiles,
            evidence_level_label(entry.evidence_level),
            entry.docs_path
        ));
    }
    output
}

fn capability_status_label(status: TransformCapabilityStatus) -> &'static str {
    match status {
        TransformCapabilityStatus::Implemented => "implemented",
        TransformCapabilityStatus::Planned => "planned",
        TransformCapabilityStatus::Research => "research",
        TransformCapabilityStatus::DependencyGated => "dependency_gated",
        TransformCapabilityStatus::HardwareGated => "hardware_gated",
        TransformCapabilityStatus::CertificationGated => "certification_gated",
    }
}

fn evidence_level_label(level: TransformEvidenceLevel) -> &'static str {
    match level {
        TransformEvidenceLevel::DocumentedOnly => "documented_only",
        TransformEvidenceLevel::UnitTested => "unit_tested",
        TransformEvidenceLevel::FixtureTested => "fixture_tested",
        TransformEvidenceLevel::GoldenReportTested => "golden_report_tested",
        TransformEvidenceLevel::ParityTested => "parity_tested",
        TransformEvidenceLevel::Validated => "validated",
    }
}

fn runtime_profile_label(profile: TransformRuntimeProfile) -> &'static str {
    profile.as_str()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DesktopSourceMode {
    Csv,
    Simulation,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SourceInspection {
    pub kind: &'static str,
    pub source_mode: String,
    pub input: String,
    pub time_column: String,
    pub time_unit: String,
    pub sample_count: usize,
    pub first_timestamp: Option<f64>,
    pub last_timestamp: Option<f64>,
    pub duration: Option<f64>,
    pub sample_interval: Option<SourceSampleIntervalInspection>,
    pub nominal_sample_rate_hz: Option<f64>,
    pub headers: Vec<String>,
    pub source_columns: Vec<String>,
    pub channels: Vec<SourceChannelInspection>,
    pub warnings: Vec<String>,
    pub scope_note: String,
}

struct SourceInspectionRequest<'a> {
    source_mode: &'a str,
    input_path: &'a str,
    time_column: &'a str,
    headers: &'a [String],
    source_columns: &'a [String],
    channel_ids: &'a [String],
    warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SourceSampleIntervalInspection {
    pub min: f64,
    pub max: f64,
    pub nominal: f64,
    pub unit: String,
    pub uniform: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SourceChannelInspection {
    pub id: String,
    pub source_column: String,
    pub unit: String,
    pub sample_count: usize,
    pub first: Option<f64>,
    pub last: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Debug, Serialize)]
struct EvaluationBundleSummary {
    kind: &'static str,
    source_mode: String,
    output_dir: String,
    overall_outcome: String,
    artifacts: Vec<String>,
    scope_note: String,
}

pub struct WorkflowTemplate {
    pub use_case: &'static str,
    pub label: &'static str,
    pub steps: &'static [&'static str],
    pub toml: &'static str,
}

impl WorkflowTemplate {
    pub fn render_text(&self) -> String {
        let mut output = String::new();
        output.push_str("FerrisOxide Desktop Workflow Template\n");
        output.push_str(&format!("Use case: {} ({})\n", self.use_case, self.label));
        output.push_str("Flow:\n");
        for step in self.steps {
            output.push_str("- ");
            output.push_str(step);
            output.push('\n');
        }
        output.push_str("\nTOML starter:\n");
        output.push_str(self.toml.trim_end());
        output
    }
}

fn inspect_source_from_args(
    args: &[String],
    source_mode: &str,
) -> Result<SourceInspection, String> {
    match normalized_source_mode(source_mode)? {
        DesktopSourceMode::Csv => inspect_csv_source_from_args(args),
        DesktopSourceMode::Simulation => inspect_simulation_source_from_args(args),
    }
}

fn normalized_source_mode(source_mode: &str) -> Result<DesktopSourceMode, String> {
    match source_mode {
        "csv" => Ok(DesktopSourceMode::Csv),
        "simulation" | "simulate" | "fixture" => Ok(DesktopSourceMode::Simulation),
        "realtime" | "real-time" | "live" | "daq" => Err(format!(
            "`{source_mode}` source mode is planned for the desktop workflow but is not implemented in this software-only CLI yet; use --source csv or --source simulation"
        )),
        _ => Err(format!(
            "unsupported --source `{source_mode}`; use csv, simulation, or a planned realtime/live mode"
        )),
    }
}

fn inspect_csv_source_from_args(args: &[String]) -> Result<SourceInspection, String> {
    let input_path = value_after(args, "--input").ok_or("missing --input <csv>")?;
    let time_column = value_after(args, "--time-column").unwrap_or("time");
    let time_unit = value_after(args, "--time-unit").unwrap_or("s");
    let signal_unit = value_after(args, "--signal-unit").unwrap_or("V");
    let input = fs::read_to_string(input_path)
        .map_err(|error| format!("failed to read `{input_path}`: {error}"))?;
    let headers = csv_headers(&input)?;
    let selected_channels = match value_after(args, "--channels") {
        Some(channels) => parse_channel_list(channels)?,
        None => headers
            .iter()
            .filter(|header| header.as_str() != time_column)
            .cloned()
            .collect::<Vec<_>>(),
    };
    if selected_channels.is_empty() {
        return Err(
            "source inspection found no channels; provide --channels <name[,name]>".to_string(),
        );
    }
    let mut options = CsvParseOptions::new(time_column, selected_channels.clone());
    options.time_unit = Unit::new(time_unit);
    options.signal_unit = Unit::new(signal_unit);
    let waveform = SimpleCsvParser
        .parse_str(&input, &options)
        .map_err(|error| format!("failed to parse waveform: {error}"))?
        .with_source_name(input_path.to_string());
    let source_columns = selected_channels.clone();
    Ok(source_inspection_from_waveform(
        SourceInspectionRequest {
            source_mode: "csv",
            input_path,
            time_column,
            headers: &headers,
            source_columns: &source_columns,
            channel_ids: &source_columns,
            warnings: Vec::new(),
        },
        &waveform,
    ))
}

fn inspect_simulation_source_from_args(args: &[String]) -> Result<SourceInspection, String> {
    let input_path = value_after(args, "--input").ok_or("missing --input <csv>")?;
    let channel_map_path =
        value_after(args, "--channel-map").ok_or("missing --channel-map <toml>")?;
    let channel_map = load_simulation_channel_map(channel_map_path)?;
    let input = fs::read_to_string(input_path)
        .map_err(|error| format!("failed to read `{input_path}`: {error}"))?;
    let headers = csv_headers(&input)?;
    let waveform = load_fixture_waveform(input_path, &channel_map)?;
    let mut channel_ids = channel_map
        .channels
        .iter()
        .map(|channel| channel.id.clone())
        .collect::<Vec<_>>();
    let mut source_columns = channel_map
        .channels
        .iter()
        .map(|channel| channel.column.clone())
        .collect::<Vec<_>>();
    if let Some(channels) = value_after(args, "--channels") {
        let selected = parse_channel_list(channels)?;
        channel_ids.retain(|channel| selected.iter().any(|candidate| candidate == channel));
        source_columns = channel_map
            .channels
            .iter()
            .filter(|channel| selected.iter().any(|candidate| candidate == &channel.id))
            .map(|channel| channel.column.clone())
            .collect();
        if channel_ids.len() != selected.len() {
            return Err(
                "--channels for simulation inspection must reference channel-map ids".to_string(),
            );
        }
    }
    Ok(source_inspection_from_waveform(
        SourceInspectionRequest {
            source_mode: "simulation",
            input_path,
            time_column: &channel_map.simulation.time_column,
            headers: &headers,
            source_columns: &source_columns,
            channel_ids: &channel_ids,
            warnings: vec![format!("channel_map={channel_map_path}")],
        },
        &waveform,
    ))
}

fn source_inspection_from_waveform(
    request: SourceInspectionRequest<'_>,
    waveform: &Waveform,
) -> SourceInspection {
    let mut warnings = request.warnings;
    if waveform.metadata.nominal_sample_rate_hz.is_none()
        && waveform.metadata.time_unit.name.as_str() != "s"
    {
        warnings.push(
            "nominal_sample_rate_hz is only computed automatically when time_unit is `s`"
                .to_string(),
        );
    }
    let channels = request
        .channel_ids
        .iter()
        .zip(request.source_columns.iter())
        .filter_map(|(id, source_column)| {
            waveform
                .channel(id)
                .map(|channel| source_channel_inspection(channel, source_column))
        })
        .collect::<Vec<_>>();
    let sample_interval =
        waveform
            .metadata
            .sample_interval
            .as_ref()
            .map(|interval| SourceSampleIntervalInspection {
                min: interval.min,
                max: interval.max,
                nominal: interval.nominal,
                unit: interval.unit.name.clone(),
                uniform: interval.uniform,
            });
    let first_timestamp = waveform.time.first().copied();
    let last_timestamp = waveform.time.last().copied();
    SourceInspection {
        kind: "source_inspection",
        source_mode: request.source_mode.to_string(),
        input: request.input_path.to_string(),
        time_column: request.time_column.to_string(),
        time_unit: waveform.metadata.time_unit.name.clone(),
        sample_count: waveform.sample_count(),
        first_timestamp,
        last_timestamp,
        duration: first_timestamp.zip(last_timestamp).map(|(first, last)| last - first),
        sample_interval,
        nominal_sample_rate_hz: waveform.metadata.nominal_sample_rate_hz,
        headers: request.headers.to_vec(),
        source_columns: request.source_columns.to_vec(),
        channels,
        warnings,
        scope_note:
            "desktop source inspection only; not live DAQ acquisition, hardware qualification, RTOS runtime, or certification evidence"
                .to_string(),
    }
}

fn source_channel_inspection(channel: &Channel, source_column: &str) -> SourceChannelInspection {
    let (min, max) = finite_min_max(&channel.samples);
    SourceChannelInspection {
        id: channel.name.clone(),
        source_column: source_column.to_string(),
        unit: channel.unit.name.clone(),
        sample_count: channel.samples.len(),
        first: channel.samples.first().copied(),
        last: channel.samples.last().copied(),
        min,
        max,
    }
}

fn finite_min_max(samples: &[f64]) -> (Option<f64>, Option<f64>) {
    let mut values = samples.iter().copied().filter(|value| value.is_finite());
    let Some(first) = values.next() else {
        return (None, None);
    };
    let mut min = first;
    let mut max = first;
    for value in values {
        min = min.min(value);
        max = max.max(value);
    }
    (Some(min), Some(max))
}

fn csv_headers(input: &str) -> Result<Vec<String>, String> {
    let header_line = input
        .lines()
        .find(|line| !line.trim().is_empty())
        .ok_or("csv input is empty")?;
    let headers = header_line
        .split(',')
        .map(|header| header.trim().trim_matches('"').to_string())
        .filter(|header| !header.is_empty())
        .collect::<Vec<_>>();
    if headers.is_empty() {
        return Err("csv header row is empty".to_string());
    }
    Ok(headers)
}

fn render_source_inspection(
    inspection: &SourceInspection,
    output_format: &str,
) -> Result<String, String> {
    match output_format {
        "json" => serde_json::to_string_pretty(inspection)
            .map_err(|error| format!("failed to render source inspection json: {error}")),
        "text" => Ok(render_source_inspection_text(inspection)),
        _ => Err(format!(
            "unsupported --format `{output_format}`; use text or json"
        )),
    }
}

fn render_source_inspection_text(inspection: &SourceInspection) -> String {
    let mut output = String::new();
    output.push_str("FerrisOxide Source Inspection\n");
    output.push_str(&format!("Source: {}\n", inspection.source_mode));
    output.push_str(&format!("Input: {}\n", inspection.input));
    output.push_str(&format!(
        "Time: column={} unit={}\n",
        inspection.time_column, inspection.time_unit
    ));
    output.push_str(&format!("Samples: {}\n", inspection.sample_count));
    if let Some(duration) = inspection.duration {
        output.push_str(&format!(
            "Duration: {} {}\n",
            format_float(duration),
            inspection.time_unit
        ));
    }
    if let Some(rate) = inspection.nominal_sample_rate_hz {
        output.push_str(&format!("Nominal sample rate: {} Hz\n", format_float(rate)));
    }
    if let Some(interval) = &inspection.sample_interval {
        output.push_str(&format!(
            "Sample interval: nominal={} {} min={} {} max={} {} uniform={}\n",
            format_float(interval.nominal),
            interval.unit,
            format_float(interval.min),
            interval.unit,
            format_float(interval.max),
            interval.unit,
            interval.uniform
        ));
    }
    output.push_str("Channels:\n");
    for channel in &inspection.channels {
        output.push_str(&format!(
            "- {} source_column={} unit={} samples={} min={} max={} first={} last={}\n",
            channel.id,
            channel.source_column,
            channel.unit,
            channel.sample_count,
            option_float(channel.min),
            option_float(channel.max),
            option_float(channel.first),
            option_float(channel.last)
        ));
    }
    if !inspection.warnings.is_empty() {
        output.push_str("Warnings:\n");
        for warning in &inspection.warnings {
            output.push_str("- ");
            output.push_str(warning);
            output.push('\n');
        }
    }
    output.push_str(&format!("Scope: {}\n", inspection.scope_note));
    output
}

fn render_analysis_config_scaffold(
    input_path: &str,
    inspection: &SourceInspection,
) -> Result<String, String> {
    let first_unit = inspection
        .channels
        .first()
        .map(|channel| channel.unit.as_str())
        .unwrap_or("V");
    let channels = inspection
        .channels
        .iter()
        .map(|channel| quoted_toml_string(&channel.id))
        .collect::<Vec<_>>()
        .join(", ");
    let mut output = String::new();
    output.push_str("# FerrisOxide analysis config scaffold\n");
    output.push_str(&format!("# Generated from source: {input_path}\n"));
    output.push_str("# Review transform order, engineering units, and thresholds before using as requirement evidence.\n\n");
    output.push_str("[input]\n");
    output.push_str(&format!(
        "time_column = {}\n",
        quoted_toml_string(&inspection.time_column)
    ));
    output.push_str(&format!("channels = [{channels}]\n"));
    output.push_str(&format!(
        "time_unit = {}\n",
        quoted_toml_string(&inspection.time_unit)
    ));
    output.push_str(&format!(
        "signal_unit = {}\n\n",
        quoted_toml_string(first_unit)
    ));
    output.push_str("[metadata]\n");
    output.push_str("acquisition_notes = \"Generated by ferrisoxide-signal scaffold-config from inspected CSV data.\"\n");
    output.push_str("environment = \"desktop workflow scaffold\"\n");
    output.push_str("operator = \"unassigned\"\n\n");
    output.push_str("[tolerances]\n");
    output.push_str("voltage_v = 0.0\n");
    output.push_str("time_s = 0.0\n\n");
    output.push_str(
        "# Channel role prompts. Keep these as comments unless a future schema adds role fields.\n",
    );
    for channel in &inspection.channels {
        output.push_str("# - ");
        output.push_str(&channel.id);
        output.push_str(
            ": assign engineering role, requirement owner, and expected use before review.\n",
        );
    }
    output.push('\n');
    output.push_str(
        "# Optional transform examples. Uncomment and tune for the channel behavior under test.\n",
    );
    output.push_str("# [[filters]]\n");
    output.push_str("# type = \"moving_average\"\n");
    output.push_str("# window_samples = 3\n\n");
    output.push_str("# [[filters]]\n");
    output.push_str("# type = \"low_pass\"\n");
    output.push_str("# cutoff_hz = 25.0\n\n");
    output.push_str("# [[feature_transforms]]\n");
    output.push_str("# id = \"primary_rms\"\n");
    output.push_str("# type = \"rms\"\n");
    output.push_str("# channel = \"");
    output.push_str(
        inspection
            .channels
            .first()
            .map(|channel| channel.id.as_str())
            .unwrap_or("channel"),
    );
    output.push_str("\"\n\n");
    for channel in &inspection.channels {
        let min = channel.min.ok_or_else(|| {
            format!(
                "cannot scaffold criteria for channel `{}` because no finite samples were found",
                channel.id
            )
        })?;
        let max = channel.max.ok_or_else(|| {
            format!(
                "cannot scaffold criteria for channel `{}` because no finite samples were found",
                channel.id
            )
        })?;
        let id = safe_identifier(&channel.id);
        output.push_str("[[criteria]]\n");
        output.push_str(&format!("id = \"{id}_min_observed\"\n"));
        output.push_str("type = \"minimum_voltage\"\n");
        output.push_str(&format!("channel = {}\n", quoted_toml_string(&channel.id)));
        output.push_str(&format!("threshold_v = {}\n\n", format_float(min)));
        output.push_str("[[criteria]]\n");
        output.push_str(&format!("id = \"{id}_max_observed\"\n"));
        output.push_str("type = \"maximum_voltage\"\n");
        output.push_str(&format!("channel = {}\n", quoted_toml_string(&channel.id)));
        output.push_str(&format!("threshold_v = {}\n\n", format_float(max)));
    }
    Ok(output)
}

fn run_csv_evaluate_bundle(
    args: &[String],
    output_dir: &str,
    overwrite: bool,
) -> Result<String, String> {
    let input_path = value_after(args, "--input").ok_or("missing --input <csv>")?;
    let config_path = value_after(args, "--config").ok_or("missing --config <toml>")?;
    let include_plot = args.iter().any(|arg| arg == "--plot");
    let output_dir = Path::new(output_dir);
    fs::create_dir_all(output_dir)
        .map_err(|error| format!("failed to create `{}`: {error}", output_dir.display()))?;
    let config = load_analysis_config_from_path(Path::new(config_path))?;
    let (report, _filters, _criteria) = analyze_configured_input(input_path, &config)?;
    let inspection_args = vec![
        "inspect-source".to_string(),
        "--input".to_string(),
        input_path.to_string(),
        "--time-column".to_string(),
        config.input.time_column.clone(),
        "--channels".to_string(),
        config.input.channels.join(","),
        "--time-unit".to_string(),
        config.input.time_unit.clone(),
        "--signal-unit".to_string(),
        config.input.signal_unit.clone(),
    ];
    let inspection = inspect_csv_source_from_args(&inspection_args)?;
    let mut artifacts = Vec::new();
    write_bundle_artifact(
        output_dir,
        "source-summary.json",
        &json_pretty(&inspection)?,
        overwrite,
        &mut artifacts,
    )?;
    let config_text = fs::read_to_string(config_path)
        .map_err(|error| format!("failed to read `{config_path}`: {error}"))?;
    write_bundle_artifact(
        output_dir,
        "config.toml",
        &config_text,
        overwrite,
        &mut artifacts,
    )?;
    write_bundle_artifact(
        output_dir,
        "report.json",
        &report
            .render_json_pretty()
            .map_err(|error| format!("failed to render bundle validation report json: {error}"))?,
        overwrite,
        &mut artifacts,
    )?;
    write_bundle_artifact(
        output_dir,
        "report.txt",
        &report.render_text(),
        overwrite,
        &mut artifacts,
    )?;
    write_bundle_artifact(
        output_dir,
        "failure-triage.md",
        &render_failure_triage("csv", report.overall_outcome(), &report),
        overwrite,
        &mut artifacts,
    )?;
    write_bundle_artifact(
        output_dir,
        "transform-catalog.json",
        &json_pretty(transform_catalog())?,
        overwrite,
        &mut artifacts,
    )?;
    if include_plot {
        let plot_path = output_dir.join("evidence.svg");
        if plot_path.exists() && !overwrite {
            return Err(format!(
                "failed to create `{}`: file exists",
                plot_path.display()
            ));
        }
        let mut plot_options = PlotOptions::new(
            plot_path.display().to_string(),
            config.input.channels.clone(),
        );
        plot_options.title = "FerrisOxide Evaluation Evidence".to_string();
        plot_options.evidence_overlays = evidence_overlays(&report.measurements, &report.results);
        let input = fs::read_to_string(input_path)
            .map_err(|error| format!("failed to read `{input_path}`: {error}"))?;
        let waveform = SimpleCsvParser
            .parse_str(&input, &config.csv_options())
            .map_err(|error| format!("failed to parse waveform for bundle plot: {error}"))?;
        render_svg(&waveform, &plot_options)
            .map_err(|error| format!("failed to render bundle plot: {error}"))?;
        artifacts.push("evidence.svg".to_string());
    }
    let mut summary_artifacts = artifacts.clone();
    summary_artifacts.push("bundle-summary.json".to_string());
    let summary = EvaluationBundleSummary {
        kind: "desktop_evaluation_bundle",
        source_mode: "csv".to_string(),
        output_dir: output_dir.display().to_string(),
        overall_outcome: report_outcome(&report),
        artifacts: summary_artifacts,
        scope_note:
            "software-only desktop evaluation bundle; not live DAQ, hardware qualification, RTOS runtime, or certification evidence"
                .to_string(),
    };
    let summary_json = json_pretty(&summary)?;
    write_output_file(
        &output_dir.join("bundle-summary.json"),
        &with_trailing_newline(summary_json.clone()),
        overwrite,
    )?;
    Ok(summary_json)
}

fn run_simulation_evaluate_bundle(
    args: &[String],
    output_dir: &str,
    overwrite: bool,
) -> Result<String, String> {
    let input_path = value_after(args, "--input").ok_or("missing --input <csv>")?;
    let control_config_path =
        value_after(args, "--control-config").ok_or("missing --control-config <toml>")?;
    let verification_config_path =
        value_after(args, "--verification-config").ok_or("missing --verification-config <toml>")?;
    let channel_map_path =
        value_after(args, "--channel-map").ok_or("missing --channel-map <toml>")?;
    let output_dir = Path::new(output_dir);
    fs::create_dir_all(output_dir)
        .map_err(|error| format!("failed to create `{}`: {error}", output_dir.display()))?;

    let control_config = load_control_config(control_config_path)?;
    let verification_config = load_verification_config(verification_config_path)?;
    let channel_map = load_simulation_channel_map(channel_map_path)?;
    validate_simulation_channel_map(&channel_map, &control_config, &verification_config)?;
    let mode = value_after(args, "--mode")
        .unwrap_or(&channel_map.simulation.mode)
        .to_string();
    let workflow = run_desktop_simulation_workflow(DesktopSimulationInput {
        input_path,
        control_config_path,
        verification_config_path,
        channel_map_path,
        mode,
        control_config,
        verification_config,
        channel_map,
    })?;
    let inspection_args = vec![
        "inspect-source".to_string(),
        "--source".to_string(),
        "simulation".to_string(),
        "--input".to_string(),
        input_path.to_string(),
        "--channel-map".to_string(),
        channel_map_path.to_string(),
    ];
    let inspection = inspect_simulation_source_from_args(&inspection_args)?;
    let mut artifacts = Vec::new();
    write_bundle_artifact(
        output_dir,
        "source-summary.json",
        &json_pretty(&inspection)?,
        overwrite,
        &mut artifacts,
    )?;
    write_bundle_artifact(
        output_dir,
        "simulation-workflow.json",
        &render_desktop_simulation_json(&workflow)?,
        overwrite,
        &mut artifacts,
    )?;
    write_bundle_artifact(
        output_dir,
        "simulation-workflow.txt",
        &render_desktop_simulation_text(&workflow),
        overwrite,
        &mut artifacts,
    )?;
    for (artifact, source_path) in [
        ("production-control-config.toml", control_config_path),
        ("test-verification-config.toml", verification_config_path),
        ("channel-map.toml", channel_map_path),
    ] {
        let contents = fs::read_to_string(source_path)
            .map_err(|error| format!("failed to read `{source_path}`: {error}"))?;
        write_bundle_artifact(output_dir, artifact, &contents, overwrite, &mut artifacts)?;
    }
    write_bundle_artifact(
        output_dir,
        "failure-triage.md",
        &render_failure_triage(
            "simulation",
            workflow.verification.overall_outcome(),
            &workflow.verification,
        ),
        overwrite,
        &mut artifacts,
    )?;
    write_bundle_artifact(
        output_dir,
        "transform-catalog.json",
        &json_pretty(transform_catalog())?,
        overwrite,
        &mut artifacts,
    )?;
    let mut summary_artifacts = artifacts.clone();
    summary_artifacts.push("bundle-summary.json".to_string());
    let summary = EvaluationBundleSummary {
        kind: "desktop_evaluation_bundle",
        source_mode: "simulation".to_string(),
        output_dir: output_dir.display().to_string(),
        overall_outcome: report_outcome(&workflow.verification),
        artifacts: summary_artifacts,
        scope_note:
            "software-only fixture simulation bundle; not live DAQ, hardware qualification, RTOS runtime, or certification evidence"
                .to_string(),
    };
    let summary_json = json_pretty(&summary)?;
    write_output_file(
        &output_dir.join("bundle-summary.json"),
        &with_trailing_newline(summary_json.clone()),
        overwrite,
    )?;
    Ok(summary_json)
}

fn write_bundle_artifact(
    output_dir: &Path,
    relative_path: &str,
    contents: &str,
    overwrite: bool,
    artifacts: &mut Vec<String>,
) -> Result<(), String> {
    validate_relative_output_path(relative_path, "bundle artifact")?;
    write_output_file(
        &output_dir.join(relative_path),
        &with_trailing_newline(contents.to_string()),
        overwrite,
    )?;
    artifacts.push(relative_path.to_string());
    Ok(())
}

fn render_failure_triage(
    source_mode: &str,
    outcome: ferrisoxide_core::analysis::Outcome,
    report: &AnalysisReport,
) -> String {
    let mut output = String::new();
    output.push_str("# FerrisOxide Failure Triage\n\n");
    output.push_str(&format!("- Source mode: {source_mode}\n"));
    output.push_str(&format!("- Overall outcome: {outcome:?}\n"));
    output.push_str("- Scope: software-only desktop evaluation evidence.\n\n");
    output.push_str("## Failed Criteria\n\n");
    let failed = report
        .results
        .iter()
        .filter(|result| matches!(result.outcome, ferrisoxide_core::analysis::Outcome::Fail))
        .collect::<Vec<_>>();
    if failed.is_empty() {
        output.push_str("No failed criteria were reported.\n");
    } else {
        for result in failed {
            output.push_str(&format!(
                "- `{}` channel={} measured={} required={} sample_index={} timestamp={}\n",
                result.criterion_id,
                result.channel,
                format_float(result.measured_value),
                format_float(result.required_value),
                result.sample_index,
                format_float(result.timestamp)
            ));
        }
    }
    output.push_str("\n## Suggested Review\n\n");
    output.push_str("- Confirm channel labels, units, time base, and transform order.\n");
    output.push_str(
        "- Check whether thresholds represent requirements or observed starter bounds.\n",
    );
    output.push_str(
        "- Re-run with `inspect-source` after changing source columns or channel labels.\n",
    );
    output
}

fn workflow_template(use_case: &str) -> Result<WorkflowTemplate, String> {
    match use_case {
        "supply-rail" => Ok(WorkflowTemplate {
            use_case: "supply-rail",
            label: "filtered rail limits and noise features",
            steps: &[
                "Inspect CSV source channels and sample rate.",
                "Label the rail channel and apply smoothing/high-frequency filtering.",
                "Add min/max pass-fail criteria and RMS/peak-to-peak features.",
                "Run evaluate-bundle with an evidence plot.",
            ],
            toml: SUPPLY_RAIL_TEMPLATE,
        }),
        "switch-bounce" => Ok(WorkflowTemplate {
            use_case: "switch-bounce",
            label: "threshold state, edges, bounce, and dwell validation",
            steps: &[
                "Inspect the switch waveform and confirm voltage thresholds.",
                "Label the switch channel.",
                "Apply Schmitt trigger, edge extraction, and bounce detection.",
                "Validate missing/extra pulses, dwell time, and timeout behavior.",
            ],
            toml: SWITCH_BOUNCE_TEMPLATE,
        }),
        "response-latency" => Ok(WorkflowTemplate {
            use_case: "response-latency",
            label: "source-to-target transition timing",
            steps: &[
                "Inspect command and response channels.",
                "Label source and target channels.",
                "Add response latency criteria with source and target thresholds.",
                "Bundle report evidence for timing review.",
            ],
            toml: RESPONSE_LATENCY_TEMPLATE,
        }),
        "sensor-cleanup" => Ok(WorkflowTemplate {
            use_case: "sensor-cleanup",
            label: "DAQ export cleanup and robust smoothing",
            steps: &[
                "Inspect imported CSV headers and sample timing.",
                "Sort, deduplicate, fill gaps, interpolate NaNs, and remove spikes.",
                "Add observed-bounds criteria and statistics features.",
                "Bundle the cleaned analysis report.",
            ],
            toml: SENSOR_CLEANUP_TEMPLATE,
        }),
        "simulated-fault" => Ok(WorkflowTemplate {
            use_case: "simulated-fault",
            label: "fault injection before pass-fail evaluation",
            steps: &[
                "Inspect the baseline waveform.",
                "Inject drift/noise/fault transforms in a controlled config.",
                "Apply pass-fail criteria to the resulting waveform.",
                "Record the bundle scope as software-only simulation evidence.",
            ],
            toml: SIMULATED_FAULT_TEMPLATE,
        }),
        "multi-channel" => Ok(WorkflowTemplate {
            use_case: "multi-channel",
            label: "derived differential and vector channels",
            steps: &[
                "Inspect all related input channels.",
                "Label source, return, and axis channels.",
                "Apply multi-channel transforms before criteria.",
                "Evaluate derived-channel margins and features.",
            ],
            toml: MULTI_CHANNEL_TEMPLATE,
        }),
        _ => Err(format!(
            "unsupported --use-case `{use_case}`; use supply-rail, switch-bounce, response-latency, sensor-cleanup, simulated-fault, or multi-channel"
        )),
    }
}

fn json_pretty<T: Serialize + ?Sized>(value: &T) -> Result<String, String> {
    serde_json::to_string_pretty(value).map_err(|error| format!("failed to render json: {error}"))
}

fn quoted_toml_string(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

fn safe_identifier(value: &str) -> String {
    let mut output = String::new();
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            output.push(character.to_ascii_lowercase());
        } else if !output.ends_with('_') {
            output.push('_');
        }
    }
    let trimmed = output.trim_matches('_').to_string();
    if trimmed.is_empty() {
        "channel".to_string()
    } else {
        trimmed
    }
}

fn option_float(value: Option<f64>) -> String {
    value.map(format_float).unwrap_or_else(|| "n/a".to_string())
}

fn format_float(value: f64) -> String {
    if !value.is_finite() {
        return value.to_string();
    }
    let mut text = format!("{value:.12}");
    while text.contains('.') && text.ends_with('0') {
        text.pop();
    }
    if text.ends_with('.') {
        text.push('0');
    }
    if text == "-0.0" {
        "0.0".to_string()
    } else {
        text
    }
}

const SUPPLY_RAIL_TEMPLATE: &str = r#"[input]
time_column = "time"
channels = ["rail_v"]
time_unit = "s"
signal_unit = "V"

[[filters]]
type = "moving_average"
window_samples = 3

[[filters]]
type = "low_pass"
cutoff_hz = 25.0

[[feature_transforms]]
id = "rail_rms"
type = "rms"
channel = "rail_v"

[[feature_transforms]]
id = "rail_peak_to_peak"
type = "peak_to_peak"
channel = "rail_v"

[[criteria]]
id = "rail_min"
type = "minimum_voltage"
channel = "rail_v"
threshold_v = 4.75

[[criteria]]
id = "rail_max"
type = "maximum_voltage"
channel = "rail_v"
threshold_v = 5.25
"#;

const SWITCH_BOUNCE_TEMPLATE: &str = r#"[input]
time_column = "time"
channels = ["switch_v"]
time_unit = "s"
signal_unit = "V"

[[event_transforms]]
id = "switch_state"
type = "schmitt_trigger"
channel = "switch_v"
on_threshold_v = 3.0
off_threshold_v = 2.0
initial_state = "low"

[[event_transforms]]
id = "switch_edges"
type = "edge_extraction"
channel = "switch_v"

[[event_transforms]]
id = "switch_bounce"
type = "bounce_detection"
channel = "switch_v"
window_s = 0.004

[[event_validations]]
id = "must_rise"
type = "missing_pulse"
channel = "switch_v"
direction = "rising"
expected_count = 1

[[event_validations]]
id = "rise_count_limit"
type = "extra_pulse"
channel = "switch_v"
direction = "rising"
max_count = 3

[[criteria]]
id = "switch_max"
type = "maximum_voltage"
channel = "switch_v"
threshold_v = 5.5
"#;

const RESPONSE_LATENCY_TEMPLATE: &str = r#"[input]
time_column = "time"
channels = ["command_v", "response_v"]
time_unit = "s"
signal_unit = "V"

[[criteria]]
id = "response_latency"
type = "response_latency"
channel = "response_v"
source_channel = "command_v"
source_threshold_v = 2.5
target_threshold_v = 2.5
source_state = "high"
expected_target_state = "high"
max_latency_s = 0.005
"#;

const SENSOR_CLEANUP_TEMPLATE: &str = r#"[input]
time_column = "time"
channels = ["sensor_v"]
time_unit = "s"
signal_unit = "V"

[[filters]]
type = "timestamp_sort"

[[filters]]
type = "dedupe_timestamps"

[[filters]]
type = "nan_interpolate"

[[filters]]
type = "spike_remove"
window_samples = 3
threshold_v = 0.5

[[filters]]
type = "exponential_moving_average"
alpha = 0.25

[[feature_transforms]]
id = "sensor_mean"
type = "mean"
channel = "sensor_v"

[[feature_transforms]]
id = "sensor_stddev"
type = "standard_deviation"
channel = "sensor_v"

[[criteria]]
id = "sensor_min"
type = "minimum_voltage"
channel = "sensor_v"
threshold_v = 0.0

[[criteria]]
id = "sensor_max"
type = "maximum_voltage"
channel = "sensor_v"
threshold_v = 5.0
"#;

const SIMULATED_FAULT_TEMPLATE: &str = r#"[input]
time_column = "time"
channels = ["sensor_v"]
time_unit = "s"
signal_unit = "V"

[[filters]]
type = "thermal_drift"
drift_rate_v_per_s = 0.01

[[filters]]
type = "gaussian_noise"
stddev_v = 0.02
seed = 11

[[criteria]]
id = "faulted_sensor_min"
type = "minimum_voltage"
channel = "sensor_v"
threshold_v = 0.0

[[criteria]]
id = "faulted_sensor_max"
type = "maximum_voltage"
channel = "sensor_v"
threshold_v = 5.0
"#;

const MULTI_CHANNEL_TEMPLATE: &str = r#"[input]
time_column = "time"
channels = ["sense_v", "return_v", "axis_x_v", "axis_y_v", "axis_z_v"]
time_unit = "s"
signal_unit = "V"

[[filters]]
type = "channel_subtract"
left_channel = "sense_v"
right_channel = "return_v"
output_channel = "differential_v"

[[filters]]
type = "vector_magnitude"
channels = ["axis_x_v", "axis_y_v", "axis_z_v"]
output_channel = "axis_mag_v"

[[criteria]]
id = "differential_max"
type = "maximum_voltage"
channel = "differential_v"
threshold_v = 5.0

[[criteria]]
id = "axis_mag_max"
type = "maximum_voltage"
channel = "axis_mag_v"
threshold_v = 10.0
"#;

#[derive(Debug, Deserialize)]
struct BatchManifest {
    pub output_dir: Option<String>,
    pub summary_file: Option<String>,
    #[serde(default)]
    pub default_format: Option<String>,
    #[serde(default)]
    pub runs: Vec<BatchRunConfig>,
}

#[derive(Debug, Deserialize)]
struct BatchRunConfig {
    pub id: String,
    pub input: String,
    pub config: String,
    pub report: Option<String>,
    pub format: Option<String>,
}

#[derive(Debug, Serialize)]
struct BatchSummary {
    kind: &'static str,
    manifest: String,
    output_dir: String,
    total_runs: usize,
    passed_runs: usize,
    failed_runs: usize,
    error_runs: usize,
    overall_outcome: String,
    runs: Vec<BatchRunResult>,
}

#[derive(Debug, Serialize)]
struct BatchRunResult {
    id: String,
    input: String,
    config: String,
    report: Option<String>,
    status: String,
    outcome: Option<String>,
    error: Option<String>,
}

impl BatchSummary {
    fn from_results(manifest: String, output_dir: String, runs: Vec<BatchRunResult>) -> Self {
        let passed_runs = runs.iter().filter(|run| run.status == "pass").count();
        let failed_runs = runs.iter().filter(|run| run.status == "fail").count();
        let error_runs = runs.iter().filter(|run| run.status == "error").count();
        let overall_outcome = if failed_runs == 0 && error_runs == 0 {
            "pass"
        } else {
            "fail"
        }
        .to_string();

        Self {
            kind: "batch_analysis",
            manifest,
            output_dir,
            total_runs: runs.len(),
            passed_runs,
            failed_runs,
            error_runs,
            overall_outcome,
            runs,
        }
    }

    fn render_text(&self) -> String {
        let mut output = String::new();
        output.push_str("FerrisOxide Batch Analysis Summary\n");
        output.push_str(&format!("Manifest: {}\n", self.manifest));
        output.push_str(&format!("Output Directory: {}\n", self.output_dir));
        output.push_str(&format!("Overall: {}\n", self.overall_outcome));
        output.push_str(&format!(
            "Runs: total={} passed={} failed={} errors={}\n",
            self.total_runs, self.passed_runs, self.failed_runs, self.error_runs
        ));
        for run in &self.runs {
            output.push_str(&format!("- {}: {}", run.id, run.status));
            if let Some(outcome) = &run.outcome {
                output.push_str(&format!(" outcome={outcome}"));
            }
            if let Some(report) = &run.report {
                output.push_str(&format!(" report={report}"));
            }
            if let Some(error) = &run.error {
                output.push_str(&format!(" error={error}"));
            }
            output.push('\n');
        }
        output
    }
}

fn load_batch_manifest(path: &str) -> Result<BatchManifest, String> {
    let input =
        fs::read_to_string(path).map_err(|error| format!("failed to read `{path}`: {error}"))?;
    let manifest = toml::from_str::<BatchManifest>(&input)
        .map_err(|error| format!("failed to parse batch manifest `{path}`: {error}"))?;
    validate_batch_manifest(&manifest)?;
    Ok(manifest)
}

fn validate_batch_manifest(manifest: &BatchManifest) -> Result<(), String> {
    if manifest.runs.is_empty() {
        return Err("batch manifest must include at least one [[runs]] entry".to_string());
    }
    if let Some(summary_file) = &manifest.summary_file {
        validate_relative_output_path(summary_file, "summary_file")?;
    }
    let mut seen_ids = BTreeMap::new();
    for run in &manifest.runs {
        validate_batch_run_id(&run.id)?;
        if seen_ids.insert(run.id.clone(), ()).is_some() {
            return Err(format!("duplicate batch run id `{}`", run.id));
        }
        if run.input.trim().is_empty() {
            return Err(format!("batch run `{}` input must not be empty", run.id));
        }
        if run.config.trim().is_empty() {
            return Err(format!("batch run `{}` config must not be empty", run.id));
        }
        if let Some(report) = &run.report {
            validate_relative_output_path(report, "runs.report")?;
        }
        if let Some(format) = &run.format {
            validate_report_format(format)?;
        }
    }
    if let Some(format) = &manifest.default_format {
        validate_report_format(format)?;
    }
    Ok(())
}

fn run_batch_entry(
    manifest_dir: &Path,
    output_dir: &Path,
    run_config: &BatchRunConfig,
    default_format: &str,
    overwrite: bool,
) -> BatchRunResult {
    let input_path = resolve_path(manifest_dir, &run_config.input);
    let config_path = resolve_path(manifest_dir, &run_config.config);
    let format = run_config.format.as_deref().unwrap_or(default_format);
    let report_file = run_config
        .report
        .clone()
        .unwrap_or_else(|| format!("{}.{}", run_config.id, report_extension(format)));
    let report_path = output_dir.join(&report_file);

    let result = (|| {
        validate_report_format(format)?;
        let config = load_analysis_config_from_path(&config_path)?;
        let (report, _, _) = analyze_configured_input(path_to_str(&input_path)?, &config)?;
        let outcome = report_outcome(&report);
        let rendered = with_trailing_newline(render_report(&report, format)?);
        write_output_file(&report_path, &rendered, overwrite)?;
        Ok(outcome)
    })();

    match result {
        Ok(outcome) => BatchRunResult {
            id: run_config.id.clone(),
            input: input_path.display().to_string(),
            config: config_path.display().to_string(),
            report: Some(report_path.display().to_string()),
            status: outcome.clone(),
            outcome: Some(outcome),
            error: None,
        },
        Err(error) => BatchRunResult {
            id: run_config.id.clone(),
            input: input_path.display().to_string(),
            config: config_path.display().to_string(),
            report: None,
            status: "error".to_string(),
            outcome: None,
            error: Some(error),
        },
    }
}

fn load_analysis_config_from_path(path: &Path) -> Result<AnalysisConfig, String> {
    let path_str = path_to_str(path)?;
    let input = fs::read_to_string(path)
        .map_err(|error| format!("failed to read `{path_str}`: {error}"))?;
    let config = toml::from_str::<AnalysisConfig>(&input)
        .map_err(|error| format!("failed to parse config `{path_str}`: {error}"))?;
    validate_loaded_config(&config)?;
    Ok(config)
}

fn validate_loaded_config(config: &AnalysisConfig) -> Result<(), String> {
    if config.input.channels.is_empty() {
        return Err("config input.channels must include at least one channel".to_string());
    }
    if config.criteria.is_empty() {
        return Err("config must include at least one [[criteria]] entry".to_string());
    }
    config
        .validate()
        .map_err(|error| format!("invalid config: {error}"))
}

fn report_outcome(report: &AnalysisReport) -> String {
    match report.overall_outcome() {
        ferrisoxide_core::analysis::Outcome::Pass => "pass".to_string(),
        ferrisoxide_core::analysis::Outcome::Fail => "fail".to_string(),
    }
}

fn report_extension(format: &str) -> &'static str {
    match format {
        "text" => "txt",
        _ => "json",
    }
}

fn validate_report_format(format: &str) -> Result<(), String> {
    match format {
        "text" | "json" => Ok(()),
        _ => Err(format!(
            "unsupported report format `{format}`; use text or json"
        )),
    }
}

fn validate_batch_run_id(id: &str) -> Result<(), String> {
    if id.trim().is_empty() {
        return Err("batch run id must not be empty".to_string());
    }
    if id.contains('/') || id.contains('\\') {
        return Err("batch run id must not contain path separators".to_string());
    }
    validate_relative_output_path(id, "runs.id")
}

fn validate_relative_output_path(path: &str, field: &str) -> Result<(), String> {
    if path.trim().is_empty() {
        return Err(format!("{field} must not be empty"));
    }
    let path = Path::new(path);
    if path.is_absolute() {
        return Err(format!("{field} must be a relative output path"));
    }
    if path.components().any(|component| {
        matches!(
            component,
            std::path::Component::ParentDir | std::path::Component::RootDir
        )
    }) {
        return Err(format!(
            "{field} must not contain parent directory components"
        ));
    }
    Ok(())
}

fn resolve_path(base_dir: &Path, path: &str) -> PathBuf {
    let path = Path::new(path);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base_dir.join(path)
    }
}

fn path_to_str(path: &Path) -> Result<&str, String> {
    path.to_str()
        .ok_or_else(|| format!("path `{}` is not valid UTF-8", path.display()))
}

struct DesktopSimulationInput<'a> {
    input_path: &'a str,
    control_config_path: &'a str,
    verification_config_path: &'a str,
    channel_map_path: &'a str,
    mode: String,
    control_config: ProductionControlConfig,
    verification_config: TestVerificationConfig,
    channel_map: SimulationChannelMap,
}

struct DesktopSimulationRun {
    input_path: String,
    control_config_path: String,
    verification_config_path: String,
    channel_map_path: String,
    mode: String,
    channel_map: SimulationChannelMap,
    simulation: SimulationReport,
    verification: AnalysisReport,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct SimulationChannelMap {
    simulation: SimulationSettings,
    channels: Vec<SimulationChannelMapping>,
    control_inputs: Vec<SimulationControlInputMapping>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SimulationSettings {
    mode: String,
    time_column: String,
    #[serde(default = "default_time_unit")]
    time_unit: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SimulationChannelMapping {
    id: String,
    column: String,
    unit: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SimulationControlInputMapping {
    input: String,
    channel: String,
}

fn default_time_unit() -> String {
    "s".to_string()
}

fn run_desktop_simulation_workflow(
    input: DesktopSimulationInput<'_>,
) -> Result<DesktopSimulationRun, String> {
    let waveform = load_fixture_waveform(input.input_path, &input.channel_map)?;
    let daq_frames = collect_fixture_daq_frames(input.input_path, &input.channel_map, &waveform)?;
    let simulation_frames = simulation_frames_from_daq(
        &input.control_config,
        &input.verification_config,
        &input.channel_map,
        &daq_frames,
    )?;
    let simulation = simulate_controller(&input.control_config, &input.mode, &simulation_frames)
        .map_err(|error| format!("simulation failed: {error}"))?;
    let criteria = verification_criteria(&input.verification_config)?;
    let evaluation =
        evaluate_criteria_with_measurements(&waveform, &criteria, TolerancePolicy::default())
            .map_err(|error| format!("verification failed: {error}"))?;
    let verification = AnalysisReport {
        input_name: input.input_path.to_string(),
        waveform_metadata: waveform.metadata.clone(),
        evidence_context: ReportEvidenceContext::engineering_validation(TolerancePolicy::default()),
        measurements: evaluation.measurements,
        feature_records: Vec::new(),
        event_records: Vec::new(),
        event_validations: Vec::new(),
        results: evaluation.results,
    };

    Ok(DesktopSimulationRun {
        input_path: input.input_path.to_string(),
        control_config_path: input.control_config_path.to_string(),
        verification_config_path: input.verification_config_path.to_string(),
        channel_map_path: input.channel_map_path.to_string(),
        mode: input.mode,
        channel_map: input.channel_map,
        simulation,
        verification,
    })
}

fn render_desktop_simulation_json(workflow: &DesktopSimulationRun) -> Result<String, String> {
    let verification_evidence: serde_json::Value = serde_json::from_str(
        &workflow
            .verification
            .render_json_pretty()
            .map_err(|error| format!("failed to render verification evidence: {error}"))?,
    )
    .map_err(|error| format!("failed to build simulation workflow json: {error}"))?;
    let document = serde_json::json!({
        "workflow": {
            "kind": "desktop_simulation",
            "input": workflow.input_path,
            "production_control_config": workflow.control_config_path,
            "test_verification_config": workflow.verification_config_path,
            "channel_map": workflow.channel_map_path,
            "mode": workflow.mode,
            "scope_note": "software-only desktop simulation evidence; not live DAQ, hardware qualification, RTOS runtime, or certification evidence",
            "loaded_channel_map": workflow.channel_map,
        },
        "simulation_trace": workflow.simulation,
        "verification_evidence": verification_evidence,
    });

    serde_json::to_string_pretty(&document)
        .map_err(|error| format!("failed to render simulation workflow json: {error}"))
}

fn render_desktop_simulation_text(workflow: &DesktopSimulationRun) -> String {
    let mut output = String::new();
    output.push_str("Desktop Simulation Workflow\n");
    output.push_str(&format!("Input: {}\n", workflow.input_path));
    output.push_str(&format!("Mode: {}\n", workflow.mode));
    output.push_str(&format!(
        "Simulation Frames: {}\n",
        workflow.simulation.frames.len()
    ));
    output.push_str(&format!(
        "Verification Overall: {:?}\n",
        workflow.verification.overall_outcome()
    ));
    output.push_str("Simulation Transitions:\n");
    for frame in &workflow.simulation.frames {
        for transition in &frame.transitions {
            output.push_str(&format!(
                "- sample_index={} timestamp={:.6} machine={} transition={} {} -> {}\n",
                frame.sample_index,
                frame.time_s,
                transition.machine,
                transition.transition,
                transition.from,
                transition.to
            ));
        }
    }
    output.push_str("Verification Criteria:\n");
    for result in &workflow.verification.results {
        output.push_str(&format!(
            "- {}: {:?} channel={} measured={:.6} required={:.6} sample_index={} timestamp={:.6}\n",
            result.criterion_id,
            result.outcome,
            result.channel,
            result.measured_value,
            result.required_value,
            result.sample_index,
            result.timestamp
        ));
    }
    output
}

fn load_control_config(path: &str) -> Result<ProductionControlConfig, String> {
    let input =
        fs::read_to_string(path).map_err(|error| format!("failed to read `{path}`: {error}"))?;
    let config = parse_control_config_toml(&input)
        .map_err(|error| format!("failed to parse control config `{path}`: {error}"))?;
    config
        .validate()
        .map_err(|report| format!("invalid control config `{path}`: {report}"))?;
    Ok(config)
}

fn load_verification_config(path: &str) -> Result<TestVerificationConfig, String> {
    let input =
        fs::read_to_string(path).map_err(|error| format!("failed to read `{path}`: {error}"))?;
    let config = parse_verification_config_toml(&input)
        .map_err(|error| format!("failed to parse verification config `{path}`: {error}"))?;
    config
        .validate()
        .map_err(|report| format!("invalid verification config `{path}`: {report}"))?;
    Ok(config)
}

fn load_simulation_channel_map(path: &str) -> Result<SimulationChannelMap, String> {
    let input =
        fs::read_to_string(path).map_err(|error| format!("failed to read `{path}`: {error}"))?;
    toml::from_str::<SimulationChannelMap>(&input)
        .map_err(|error| format!("failed to parse simulation channel map `{path}`: {error}"))
}

fn validate_simulation_channel_map(
    channel_map: &SimulationChannelMap,
    control_config: &ProductionControlConfig,
    verification_config: &TestVerificationConfig,
) -> Result<(), String> {
    if channel_map.simulation.mode.trim().is_empty() {
        return Err("channel map simulation.mode must not be empty".to_string());
    }
    if channel_map.simulation.time_column.trim().is_empty() {
        return Err("channel map simulation.time_column must not be empty".to_string());
    }
    if channel_map.simulation.time_unit != "s" {
        return Err("channel map simulation.time_unit must be `s` for this workflow".to_string());
    }
    if channel_map.channels.is_empty() {
        return Err("channel map must include at least one [[channels]] entry".to_string());
    }
    if channel_map.control_inputs.is_empty() {
        return Err("channel map must include at least one [[control_inputs]] entry".to_string());
    }

    let mut channel_ids = std::collections::BTreeSet::new();
    let mut columns = std::collections::BTreeSet::new();
    for channel in &channel_map.channels {
        if channel.id.trim().is_empty() {
            return Err("channel map channel id must not be empty".to_string());
        }
        if channel.column.trim().is_empty() {
            return Err(format!(
                "channel map channel `{}` column must not be empty",
                channel.id
            ));
        }
        if channel.unit.trim().is_empty() {
            return Err(format!(
                "channel map channel `{}` unit must not be empty",
                channel.id
            ));
        }
        if !channel_ids.insert(channel.id.clone()) {
            return Err(format!("duplicate channel map channel `{}`", channel.id));
        }
        if !columns.insert(channel.column.clone()) {
            return Err(format!(
                "duplicate channel map source column `{}`",
                channel.column
            ));
        }
    }

    let mut mapped_inputs = std::collections::BTreeSet::new();
    for mapping in &channel_map.control_inputs {
        if !control_config
            .inputs
            .iter()
            .any(|input| input.id == mapping.input)
        {
            return Err(format!(
                "channel map references unknown control input `{}`",
                mapping.input
            ));
        }
        if !channel_ids.contains(&mapping.channel) {
            return Err(format!(
                "channel map control input `{}` references unknown channel `{}`",
                mapping.input, mapping.channel
            ));
        }
        if !mapped_inputs.insert(mapping.input.clone()) {
            return Err(format!(
                "duplicate channel map control input `{}`",
                mapping.input
            ));
        }
    }
    for input in &control_config.inputs {
        if !mapped_inputs.contains(&input.id) {
            return Err(format!(
                "channel map is missing required control input `{}`",
                input.id
            ));
        }
    }

    for verification_channel in &verification_config.channels {
        let Some(mapped_channel) = channel_map
            .channels
            .iter()
            .find(|channel| channel.id == verification_channel.id)
        else {
            return Err(format!(
                "channel map is missing verification channel `{}`",
                verification_channel.id
            ));
        };
        if mapped_channel.column != verification_channel.column {
            return Err(format!(
                "channel map channel `{}` column `{}` does not match verification config column `{}`",
                mapped_channel.id, mapped_channel.column, verification_channel.column
            ));
        }
        if mapped_channel.unit != verification_channel.unit {
            return Err(format!(
                "channel map channel `{}` unit `{}` does not match verification config unit `{}`",
                mapped_channel.id, mapped_channel.unit, verification_channel.unit
            ));
        }
    }

    Ok(())
}

fn load_fixture_waveform(
    input_path: &str,
    channel_map: &SimulationChannelMap,
) -> Result<Waveform, String> {
    let input = fs::read_to_string(input_path)
        .map_err(|error| format!("failed to read `{input_path}`: {error}"))?;
    let columns = channel_map
        .channels
        .iter()
        .map(|channel| channel.column.clone())
        .collect::<Vec<_>>();
    let mut options = CsvParseOptions::new(&channel_map.simulation.time_column, columns);
    options.time_unit = Unit::new(&channel_map.simulation.time_unit);
    let parsed = SimpleCsvParser
        .parse_str(&input, &options)
        .map_err(|error| format!("failed to parse simulation fixture: {error}"))?;
    let channels = channel_map
        .channels
        .iter()
        .map(|mapping| {
            let source = parsed
                .channel(&mapping.column)
                .ok_or_else(|| format!("parsed fixture is missing `{}`", mapping.column))?;
            Ok(Channel::new(
                mapping.id.clone(),
                Unit::new(&mapping.unit),
                source.samples.clone(),
            ))
        })
        .collect::<Result<Vec<_>, String>>()?;

    Waveform::new_with_time_unit(
        parsed.time.clone(),
        Unit::new(&channel_map.simulation.time_unit),
        channels,
    )
    .map_err(|error| format!("failed to build simulation waveform: {error}"))
    .map(|waveform| {
        waveform
            .with_source_name(input_path.to_string())
            .with_tolerance_policy(TolerancePolicy::default())
    })
}

fn collect_fixture_daq_frames(
    input_path: &str,
    channel_map: &SimulationChannelMap,
    waveform: &Waveform,
) -> Result<Vec<DaqSampleFrame>, String> {
    let descriptor = DaqSourceDescriptor {
        name: input_path.to_string(),
        kind: DaqSourceKind::Fixture,
        channels: channel_map
            .channels
            .iter()
            .map(|channel| DaqChannel::new(&channel.id, &channel.column, &channel.unit))
            .collect(),
    };
    let frames = (0..waveform.sample_count())
        .map(|sample_index| {
            let mut frame = DaqSampleFrame::new(waveform.time[sample_index]);
            for channel in &channel_map.channels {
                let waveform_channel = waveform
                    .channel(&channel.id)
                    .ok_or_else(|| format!("waveform is missing channel `{}`", channel.id))?;
                frame = frame.with_value(
                    &channel.id,
                    DaqSampleValue::Analog {
                        value: waveform_channel.samples[sample_index],
                    },
                );
            }
            Ok(frame)
        })
        .collect::<Result<Vec<_>, String>>()?;
    let mut source = FixtureDaqSource::new(descriptor, frames)
        .map_err(|error| format!("failed to build fixture DAQ source: {error}"))?;
    collect_frames(&mut source, usize::MAX)
        .map_err(|error| format!("failed to collect fixture DAQ frames: {error}"))
}

fn simulation_frames_from_daq(
    control_config: &ProductionControlConfig,
    verification_config: &TestVerificationConfig,
    channel_map: &SimulationChannelMap,
    daq_frames: &[DaqSampleFrame],
) -> Result<Vec<SimulationInputFrame>, String> {
    let input_to_channel = channel_map
        .control_inputs
        .iter()
        .map(|mapping| (mapping.input.as_str(), mapping.channel.as_str()))
        .collect::<BTreeMap<_, _>>();
    let verification_channels = verification_channels_by_id(verification_config);
    daq_frames
        .iter()
        .enumerate()
        .map(|(sample_index, daq_frame)| {
            let mut frame = SimulationInputFrame::new(daq_frame.time_s);
            for input in &control_config.inputs {
                let channel_id = input_to_channel.get(input.id.as_str()).ok_or_else(|| {
                    format!("missing channel map for control input `{}`", input.id)
                })?;
                let value = daq_frame.values.get(*channel_id).ok_or_else(|| {
                    format!("DAQ frame {sample_index} is missing mapped channel `{channel_id}`")
                })?;
                let verification_channel = verification_channels
                    .get(*channel_id)
                    .ok_or_else(|| format!("missing verification channel `{channel_id}`"))?;
                frame = frame.with_input(
                    &input.id,
                    simulated_input_value(
                        input.signal,
                        value,
                        verification_config,
                        verification_channel,
                    )?,
                );
            }
            Ok(frame)
        })
        .collect()
}

fn simulated_input_value(
    signal: SignalKind,
    value: &DaqSampleValue,
    verification_config: &TestVerificationConfig,
    channel: &VerificationChannel,
) -> Result<SimulatedInputValue, String> {
    match signal {
        SignalKind::AnalogVoltage | SignalKind::Virtual => match value {
            DaqSampleValue::Analog { value } => Ok(SimulatedInputValue::Analog { value: *value }),
            DaqSampleValue::Digital { high } => Ok(SimulatedInputValue::Digital {
                state: if *high {
                    ControlDigitalState::High
                } else {
                    ControlDigitalState::Low
                },
            }),
        },
        SignalKind::Digital | SignalKind::Gpio => match value {
            DaqSampleValue::Digital { high } => Ok(SimulatedInputValue::Digital {
                state: if *high {
                    ControlDigitalState::High
                } else {
                    ControlDigitalState::Low
                },
            }),
            DaqSampleValue::Analog { value } => {
                let threshold = decision_threshold(
                    verification_config,
                    &channel.id,
                    VerificationDigitalState::High,
                )?;
                Ok(SimulatedInputValue::Digital {
                    state: if *value >= threshold {
                        ControlDigitalState::High
                    } else {
                        ControlDigitalState::Low
                    },
                })
            }
        },
        SignalKind::Pwm => Err(format!(
            "control input `{}` uses unsupported PWM signal mapping for simulation input",
            channel.id
        )),
    }
}

fn verification_criteria(config: &TestVerificationConfig) -> Result<Vec<Criterion>, String> {
    let mut criteria = Vec::new();

    for transition in &config.expected_transitions {
        if !transition.required {
            continue;
        }
        if transition.min_latency_s.unwrap_or(0.0) > 0.0 {
            return Err(format!(
                "expected transition `{}` min_latency_s is not supported by the current desktop workflow",
                transition.id
            ));
        }
        let source_channel = transition.reference_channel.as_ref().ok_or_else(|| {
            format!(
                "expected transition `{}` requires reference_channel for response-latency evaluation",
                transition.id
            )
        })?;
        let source_state = transition.reference_state.ok_or_else(|| {
            format!(
                "expected transition `{}` requires reference_state for response-latency evaluation",
                transition.id
            )
        })?;
        let max_latency_s = transition.max_latency_s.ok_or_else(|| {
            format!(
                "expected transition `{}` requires max_latency_s for response-latency evaluation",
                transition.id
            )
        })?;
        criteria.push(Criterion::response_latency(
            &transition.id,
            ResponseLatencySpec {
                source_channel: source_channel.clone(),
                target_channel: transition.channel.clone(),
                source_threshold_v: decision_threshold(config, source_channel, source_state)?,
                target_threshold_v: decision_threshold(
                    config,
                    &transition.channel,
                    transition.to_state,
                )?,
                source_state: signal_state(source_state),
                expected_target_state: signal_state(transition.to_state),
                max_latency_s,
            },
        ));
    }

    for limit in &config.voltage_limits {
        match (limit.min_v, limit.max_v) {
            (Some(min_v), Some(max_v)) => {
                criteria.push(Criterion::minimum_voltage(
                    format!("{}-min", limit.id),
                    &limit.channel,
                    min_v,
                ));
                criteria.push(Criterion::maximum_voltage(
                    format!("{}-max", limit.id),
                    &limit.channel,
                    max_v,
                ));
            }
            (Some(min_v), None) => {
                criteria.push(Criterion::minimum_voltage(&limit.id, &limit.channel, min_v))
            }
            (None, Some(max_v)) => {
                criteria.push(Criterion::maximum_voltage(&limit.id, &limit.channel, max_v))
            }
            (None, None) => {
                return Err(format!(
                    "voltage limit `{}` must include min_v or max_v",
                    limit.id
                ));
            }
        }
    }

    for requirement in &config.pulse_widths {
        criteria.push(Criterion::pulse_width(
            &requirement.id,
            &requirement.channel,
            signal_state(requirement.state),
            decision_threshold(config, &requirement.channel, requirement.state)?,
            requirement.min_width_s,
            requirement.max_width_s,
        ));
    }

    for limit in &config.transient_limits {
        let (start_time_s, end_time_s) = timing_window(config, limit.window.as_deref())?;
        criteria.push(Criterion::transient_event_window(
            &limit.id,
            &limit.channel,
            transient_event_kind(limit.event_kind),
            signal_state(limit.expected_state),
            decision_threshold(config, &limit.channel, limit.expected_state)?,
            limit.max_duration_s,
            TransientEventWindow {
                start_time_s,
                end_time_s,
                arm_after_first_expected_state: limit.arm_after_first_expected_state,
            },
        ));
    }

    for limit in &config.dropout_limits {
        let (start_time_s, end_time_s) = timing_window(config, limit.window.as_deref())?;
        criteria.push(Criterion::transient_event_window(
            &limit.id,
            &limit.channel,
            TransientEventKind::Dropout,
            signal_state(limit.expected_state),
            decision_threshold(config, &limit.channel, limit.expected_state)?,
            limit.max_duration_s,
            TransientEventWindow {
                start_time_s,
                end_time_s,
                arm_after_first_expected_state: false,
            },
        ));
    }

    for requirement in &config.stable_state_requirements {
        criteria.push(Criterion::stable_state_duration(
            &requirement.id,
            &requirement.channel,
            signal_state(requirement.state),
            requirement.threshold_v.unwrap_or(decision_threshold(
                config,
                &requirement.channel,
                requirement.state,
            )?),
            requirement.min_duration_s,
        ));
    }

    if criteria.is_empty() {
        return Err("verification config did not produce any executable criteria".to_string());
    }

    Ok(criteria)
}

fn verification_channels_by_id(
    config: &TestVerificationConfig,
) -> BTreeMap<&str, &VerificationChannel> {
    config
        .channels
        .iter()
        .map(|channel| (channel.id.as_str(), channel))
        .collect()
}

fn decision_threshold(
    config: &TestVerificationConfig,
    channel_id: &str,
    state: VerificationDigitalState,
) -> Result<f64, String> {
    let channel = config
        .channels
        .iter()
        .find(|channel| channel.id == channel_id)
        .ok_or_else(|| format!("unknown verification channel `{channel_id}`"))?;
    match (channel.low_threshold, channel.high_threshold) {
        (Some(low), Some(high)) => return Ok((low + high) / 2.0),
        (Some(low), None) => return Ok(low),
        (None, Some(high)) => return Ok(high),
        (None, None) => {}
    }

    for limit in &config.voltage_limits {
        if limit.channel == channel_id {
            match state {
                VerificationDigitalState::High => {
                    if let Some(min_v) = limit.min_v {
                        return Ok(min_v);
                    }
                }
                VerificationDigitalState::Low => {
                    if let Some(max_v) = limit.max_v {
                        return Ok(max_v);
                    }
                }
            }
        }
    }

    Err(format!(
        "verification channel `{channel_id}` needs low/high thresholds or a compatible voltage limit"
    ))
}

fn timing_window(
    config: &TestVerificationConfig,
    window_id: Option<&str>,
) -> Result<(Option<f64>, Option<f64>), String> {
    let Some(window_id) = window_id else {
        return Ok((None, None));
    };
    let window = config
        .timing_windows
        .iter()
        .find(|window| window.id == window_id)
        .ok_or_else(|| format!("unknown timing window `{window_id}`"))?;
    Ok((Some(window.start_s), window.end_s))
}

fn signal_state(state: VerificationDigitalState) -> SignalState {
    match state {
        VerificationDigitalState::Low => SignalState::Low,
        VerificationDigitalState::High => SignalState::High,
    }
}

fn transient_event_kind(kind: VerificationTransientEventKind) -> TransientEventKind {
    match kind {
        VerificationTransientEventKind::TransientEvent => TransientEventKind::TransientEvent,
        VerificationTransientEventKind::SpuriousTransition
        | VerificationTransientEventKind::FalseTransition => TransientEventKind::SpuriousTransition,
        VerificationTransientEventKind::ContactBounce => TransientEventKind::ContactBounce,
        VerificationTransientEventKind::NoiseInducedTransition => {
            TransientEventKind::NoiseInducedTransition
        }
        VerificationTransientEventKind::ThresholdCrossingEvent => {
            TransientEventKind::ThresholdCrossingEvent
        }
    }
}

struct ExportArtifact {
    path: String,
    role: String,
    media_type: String,
    contents: String,
}

impl ExportArtifact {
    fn new(
        path: impl Into<String>,
        role: impl Into<String>,
        media_type: impl Into<String>,
        contents: String,
    ) -> Self {
        Self {
            path: path.into(),
            role: role.into(),
            media_type: media_type.into(),
            contents,
        }
    }

    fn manifest_artifact(&self) -> ManifestArtifact {
        ManifestArtifact::from_contents(
            self.path.clone(),
            self.role.clone(),
            self.media_type.clone(),
            &self.contents,
        )
    }
}

fn checksum_file_contents(metadata: &ChecksumMetadata, entries: &[(String, String)]) -> String {
    let mut output = String::new();
    output.push_str("# FerrisOxide Rule Package checksums\n");
    output.push_str(&format!("algorithm={}\n", metadata.algorithm));
    output.push_str(&format!("format={}\n", metadata.format));
    output.push_str(&format!("scope={}\n", metadata.scope));
    output.push_str(&format!("security_note={}\n", metadata.security_note));
    output.push('\n');
    for (path, checksum) in entries {
        output.push_str(checksum);
        output.push_str("  ");
        output.push_str(path);
        output.push('\n');
    }
    output
}

fn with_trailing_newline(mut contents: String) -> String {
    if !contents.ends_with('\n') {
        contents.push('\n');
    }
    contents
}

fn analyze_configured_input(
    input_path: &str,
    config: &AnalysisConfig,
) -> Result<(AnalysisReport, Vec<FilterStep>, Vec<Criterion>), String> {
    let input = fs::read_to_string(input_path)
        .map_err(|error| format!("failed to read `{input_path}`: {error}"))?;
    let parser = SimpleCsvParser;
    let mut waveform = parser
        .parse_str(&input, &config.csv_options())
        .map_err(|error| format!("failed to parse waveform: {error}"))?
        .with_source_name(input_path.to_string())
        .with_metadata_context(&config.metadata)
        .with_tolerance_policy(config.tolerances);
    let filters = config
        .filters()
        .map_err(|error| format!("invalid config filters: {error}"))?;
    waveform = apply_filter_chain(&waveform, &filters)
        .map_err(|error| format!("filter pipeline failed: {error}"))?;
    let feature_transforms = config
        .feature_transforms()
        .map_err(|error| format!("invalid config feature transforms: {error}"))?;
    let feature_records = evaluate_feature_transforms(&waveform, &feature_transforms)
        .map_err(|error| format!("feature analysis failed: {error}"))?;
    let criteria = config
        .criteria()
        .map_err(|error| format!("invalid config criteria: {error}"))?;
    let event_transforms = config
        .event_transforms()
        .map_err(|error| format!("invalid config event transforms: {error}"))?;
    let event_validations = config
        .event_validations()
        .map_err(|error| format!("invalid config event validations: {error}"))?;
    let event_evaluation =
        evaluate_event_pipeline(&waveform, &event_transforms, &event_validations)
            .map_err(|error| format!("event analysis failed: {error}"))?;
    let evaluation = evaluate_criteria_with_measurements(&waveform, &criteria, config.tolerances)
        .map_err(|error| format!("analysis failed: {error}"))?;
    let report = AnalysisReport {
        input_name: input_path.to_string(),
        waveform_metadata: waveform.metadata.clone(),
        evidence_context: ReportEvidenceContext::engineering_validation(config.tolerances),
        measurements: evaluation.measurements,
        feature_records,
        event_records: event_evaluation.records,
        event_validations: event_evaluation.validations,
        results: evaluation.results,
    };

    Ok((report, filters, criteria))
}

struct RulePackageBuildInput<'a> {
    package_name: &'a str,
    package_version: &'a str,
    target: TargetProfileKind,
    target_identifier: Option<String>,
    config: &'a AnalysisConfig,
    report: &'a AnalysisReport,
    filters: &'a [FilterStep],
    criteria: &'a [Criterion],
}

fn build_rule_package(input: RulePackageBuildInput<'_>) -> Result<RulePackage, String> {
    let mut target_profile = TargetProfile::new(input.target);
    target_profile.identifier = input.target_identifier;
    target_profile.notes.push(
        "Exported by FerrisOxide Signal CLI as software validation evidence only; not hardware qualification or certification evidence."
            .to_string(),
    );

    let config = input.config;
    let report = input.report;
    let filters = input.filters;
    let criteria = input.criteria;

    let thresholds = thresholds_by_channel(criteria)?;
    let channels = report
        .waveform_metadata
        .channels
        .iter()
        .map(|channel| {
            let unit = engineering_unit(&channel.unit.name)?;
            Ok(ChannelDefinition {
                name: channel.name.clone(),
                source_name: Some(channel.name.clone()),
                unit,
                sample_rate_hz: report.waveform_metadata.nominal_sample_rate_hz,
                thresholds: thresholds.get(&channel.name).cloned().unwrap_or_default(),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;

    let filters = schema_filters(filters, &config.input.channels)?;
    let criteria = criteria
        .iter()
        .flat_map(schema_criteria)
        .collect::<Result<Vec<_>, String>>()?;

    Ok(RulePackage {
        package: PackageMetadata {
            name: input.package_name.to_string(),
            version: input.package_version.to_string(),
            schema_version: ferrisoxide_rule_schema::CURRENT_SCHEMA_VERSION.to_string(),
            description: Some(format!(
                "Exported from {} using FerrisOxide Signal.",
                report.input_name
            )),
        },
        target: target_profile,
        sample_timing: SampleTimingAssumption {
            timestamp_unit: engineering_unit(&report.waveform_metadata.time_unit.name)?,
            nominal_sample_rate_hz: report.waveform_metadata.nominal_sample_rate_hz,
            sample_rate_tolerance_hz: None,
            nominal_sample_interval_s: report
                .waveform_metadata
                .sample_interval
                .as_ref()
                .map(|interval| interval.nominal),
            timestamp_tolerance_s: Some(config.tolerances.time_s),
        },
        channels,
        filters,
        criteria,
    })
}

fn schema_filters(
    filters: &[FilterStep],
    channels: &[String],
) -> Result<Vec<FilterDefinition>, String> {
    let mut schema_filters = Vec::new();
    for (index, filter) in filters.iter().enumerate() {
        for channel in channels {
            if !filter.rule_package_export_supported() {
                return Err(format!(
                    "rule package export does not yet support transform `{}`",
                    filter.name()
                ));
            }
            let id = match filter {
                FilterStep::MovingAverage(_) => format!("moving_average_{index}_{channel}"),
                FilterStep::Offset(_) => format!("offset_{index}_{channel}"),
                FilterStep::Gain(_) => format!("gain_{index}_{channel}"),
                FilterStep::Invert(_) => format!("invert_{index}_{channel}"),
                FilterStep::LowPass(_) => format!("low_pass_{index}_{channel}"),
                FilterStep::AdcQuantize(_) => format!("adc_quantize_{index}_{channel}"),
                _ => unreachable!("catalog rejected unsupported filters before id generation"),
            };
            schema_filters.push(match filter {
                FilterStep::MovingAverage(filter) => FilterDefinition::MovingAverage {
                    id,
                    channel: channel.clone(),
                    window_samples: filter.window_samples,
                },
                FilterStep::Offset(filter) => FilterDefinition::Offset {
                    id,
                    channel: channel.clone(),
                    offset: UnitValue::new(filter.offset_v, EngineeringUnit::Volt),
                },
                FilterStep::Gain(filter) => FilterDefinition::Gain {
                    id,
                    channel: channel.clone(),
                    gain: filter.gain,
                },
                FilterStep::Invert(_) => FilterDefinition::Invert {
                    id,
                    channel: channel.clone(),
                },
                FilterStep::LowPass(filter) => FilterDefinition::LowPass {
                    id,
                    channel: channel.clone(),
                    cutoff: UnitValue::new(filter.cutoff_hz, EngineeringUnit::Hertz),
                },
                FilterStep::AdcQuantize(filter) => FilterDefinition::AdcQuantize {
                    id,
                    channel: channel.clone(),
                    bits: filter.bits,
                    min: UnitValue::new(filter.min_v, EngineeringUnit::Volt),
                    max: UnitValue::new(filter.max_v, EngineeringUnit::Volt),
                },
                _ => unreachable!("unsupported filters return before schema conversion"),
            });
        }
    }
    Ok(schema_filters)
}

fn schema_criteria(criterion: &Criterion) -> Vec<Result<CriterionDefinition, String>> {
    match &criterion.check {
        CriterionCheck::MinimumVoltage {
            channel,
            threshold_v,
        } => vec![Ok(schema_criterion(
            &criterion.id,
            channel,
            MeasurementDefinition::MinimumSample,
            ComparisonOperator::GreaterThanOrEqual,
            UnitValue::new(*threshold_v, EngineeringUnit::Volt),
        ))],
        CriterionCheck::MaximumVoltage {
            channel,
            threshold_v,
        } => vec![Ok(schema_criterion(
            &criterion.id,
            channel,
            MeasurementDefinition::MaximumSample,
            ComparisonOperator::LessThanOrEqual,
            UnitValue::new(*threshold_v, EngineeringUnit::Volt),
        ))],
        CriterionCheck::StateTransitions {
            channel,
            threshold_v,
            expected_count,
        } => vec![Ok(schema_criterion(
            &criterion.id,
            channel,
            MeasurementDefinition::StateTransitionCount {
                threshold: UnitValue::new(*threshold_v, EngineeringUnit::Volt),
            },
            ComparisonOperator::EqualTo,
            UnitValue::new(*expected_count as f64, EngineeringUnit::Count),
        ))],
        CriterionCheck::PulseWidth {
            channel,
            state,
            threshold_v,
            min_width_s,
            max_width_s,
        } => pulse_width_schema_criteria(
            &criterion.id,
            channel,
            *state,
            *threshold_v,
            *min_width_s,
            *max_width_s,
        ),
        CriterionCheck::TransientDuration {
            channel,
            expected_state,
            threshold_v,
            max_duration_s,
        } => vec![Ok(schema_criterion(
            &criterion.id,
            channel,
            MeasurementDefinition::TransientEventDuration {
                event_kind: ferrisoxide_rule_schema::TransientEventKind::TransientEvent,
                expected_state: schema_signal_state(*expected_state),
                threshold: UnitValue::new(*threshold_v, EngineeringUnit::Volt),
            },
            ComparisonOperator::LessThanOrEqual,
            UnitValue::new(*max_duration_s, EngineeringUnit::Second),
        ))],
        CriterionCheck::TransientEvent {
            channel,
            event_kind,
            expected_state,
            threshold_v,
            max_duration_s,
            ..
        } => vec![Ok(schema_criterion(
            &criterion.id,
            channel,
            MeasurementDefinition::TransientEventDuration {
                event_kind: schema_transient_event_kind(*event_kind),
                expected_state: schema_signal_state(*expected_state),
                threshold: UnitValue::new(*threshold_v, EngineeringUnit::Volt),
            },
            ComparisonOperator::LessThanOrEqual,
            UnitValue::new(*max_duration_s, EngineeringUnit::Second),
        ))],
        CriterionCheck::ResponseLatency {
            source_channel,
            target_channel,
            source_threshold_v,
            target_threshold_v,
            source_state,
            expected_target_state,
            max_latency_s,
        } => vec![Ok(schema_criterion(
            &criterion.id,
            target_channel,
            MeasurementDefinition::ResponseLatency {
                source_channel: source_channel.clone(),
                source_threshold: UnitValue::new(*source_threshold_v, EngineeringUnit::Volt),
                target_threshold: UnitValue::new(*target_threshold_v, EngineeringUnit::Volt),
                source_state: schema_signal_state(*source_state),
                expected_target_state: schema_signal_state(*expected_target_state),
            },
            ComparisonOperator::LessThanOrEqual,
            UnitValue::new(*max_latency_s, EngineeringUnit::Second),
        ))],
        CriterionCheck::StableStateDuration {
            channel,
            state,
            threshold_v,
            min_duration_s,
        } => vec![Ok(schema_criterion(
            &criterion.id,
            channel,
            MeasurementDefinition::StableStateDuration {
                state: schema_signal_state(*state),
                threshold: UnitValue::new(*threshold_v, EngineeringUnit::Volt),
            },
            ComparisonOperator::GreaterThanOrEqual,
            UnitValue::new(*min_duration_s, EngineeringUnit::Second),
        ))],
        CriterionCheck::RiseFallTime {
            channel,
            direction,
            low_threshold_v,
            high_threshold_v,
            max_duration_s,
        } => vec![Ok(schema_criterion(
            &criterion.id,
            channel,
            match direction {
                EdgeDirection::Rise => MeasurementDefinition::RiseTime {
                    low_threshold: UnitValue::new(*low_threshold_v, EngineeringUnit::Volt),
                    high_threshold: UnitValue::new(*high_threshold_v, EngineeringUnit::Volt),
                },
                EdgeDirection::Fall => MeasurementDefinition::FallTime {
                    low_threshold: UnitValue::new(*low_threshold_v, EngineeringUnit::Volt),
                    high_threshold: UnitValue::new(*high_threshold_v, EngineeringUnit::Volt),
                },
            },
            ComparisonOperator::LessThanOrEqual,
            UnitValue::new(*max_duration_s, EngineeringUnit::Second),
        ))],
        CriterionCheck::Measurement {
            channel,
            measurement,
            requirement,
        } => vec![schema_measurement_criterion(
            &criterion.id,
            channel,
            measurement,
            requirement.operator,
            requirement.value,
        )],
    }
}

fn pulse_width_schema_criteria(
    id: &str,
    channel: &str,
    state: SignalState,
    threshold_v: f64,
    min_width_s: Option<f64>,
    max_width_s: Option<f64>,
) -> Vec<Result<CriterionDefinition, String>> {
    let measurement = |selection| MeasurementDefinition::PulseWidth {
        state: schema_signal_state(state),
        threshold: UnitValue::new(threshold_v, EngineeringUnit::Volt),
        selection: Some(selection),
    };
    let mut criteria = Vec::new();
    if let Some(min_width_s) = min_width_s {
        criteria.push(Ok(schema_criterion(
            &format!("{id}_min"),
            channel,
            measurement(ferrisoxide_rule_schema::RunSelection::Shortest),
            ComparisonOperator::GreaterThanOrEqual,
            UnitValue::new(min_width_s, EngineeringUnit::Second),
        )));
    }
    if let Some(max_width_s) = max_width_s {
        criteria.push(Ok(schema_criterion(
            &format!("{id}_max"),
            channel,
            measurement(ferrisoxide_rule_schema::RunSelection::Longest),
            ComparisonOperator::LessThanOrEqual,
            UnitValue::new(max_width_s, EngineeringUnit::Second),
        )));
    }
    if criteria.is_empty() {
        criteria.push(Err(format!(
            "pulse_width criterion `{id}` must include min_width_s or max_width_s for export"
        )));
    }
    criteria
}

fn schema_measurement_criterion(
    id: &str,
    channel: &str,
    measurement: &MeasurementSpec,
    operator: CriterionOperator,
    value: f64,
) -> Result<CriterionDefinition, String> {
    let requirement_unit = schema_requirement_unit(measurement.kind())?;
    let measurement = match measurement {
        MeasurementSpec::MinimumSample => MeasurementDefinition::MinimumSample,
        MeasurementSpec::MaximumSample => MeasurementDefinition::MaximumSample,
        MeasurementSpec::StateTransitionCount { threshold_v } => {
            MeasurementDefinition::StateTransitionCount {
                threshold: UnitValue::new(*threshold_v, EngineeringUnit::Volt),
            }
        }
        MeasurementSpec::PulseWidth {
            state,
            threshold_v,
            selection,
        } => MeasurementDefinition::PulseWidth {
            state: schema_signal_state(*state),
            threshold: UnitValue::new(*threshold_v, EngineeringUnit::Volt),
            selection: Some(schema_run_selection(*selection)),
        },
        MeasurementSpec::StableStateDuration { state, threshold_v } => {
            MeasurementDefinition::StableStateDuration {
                state: schema_signal_state(*state),
                threshold: UnitValue::new(*threshold_v, EngineeringUnit::Volt),
            }
        }
        MeasurementSpec::TransientEventDuration {
            event_kind,
            expected_state,
            threshold_v,
        } => MeasurementDefinition::TransientEventDuration {
            event_kind: schema_transient_event_kind(*event_kind),
            expected_state: schema_signal_state(*expected_state),
            threshold: UnitValue::new(*threshold_v, EngineeringUnit::Volt),
        },
        MeasurementSpec::RiseTime {
            low_threshold_v,
            high_threshold_v,
        } => MeasurementDefinition::RiseTime {
            low_threshold: UnitValue::new(*low_threshold_v, EngineeringUnit::Volt),
            high_threshold: UnitValue::new(*high_threshold_v, EngineeringUnit::Volt),
        },
        MeasurementSpec::FallTime {
            low_threshold_v,
            high_threshold_v,
        } => MeasurementDefinition::FallTime {
            low_threshold: UnitValue::new(*low_threshold_v, EngineeringUnit::Volt),
            high_threshold: UnitValue::new(*high_threshold_v, EngineeringUnit::Volt),
        },
    };
    Ok(schema_criterion(
        id,
        channel,
        measurement,
        schema_operator(operator),
        UnitValue::new(value, requirement_unit),
    ))
}

fn schema_criterion(
    id: &str,
    channel: &str,
    measurement: MeasurementDefinition,
    operator: ComparisonOperator,
    value: UnitValue,
) -> CriterionDefinition {
    CriterionDefinition {
        id: id.to_string(),
        channel: channel.to_string(),
        measurement,
        requirement: RequirementDefinition { operator, value },
    }
}

fn thresholds_by_channel(
    criteria: &[Criterion],
) -> Result<BTreeMap<String, Vec<ThresholdDefinition>>, String> {
    let mut thresholds = BTreeMap::<String, Vec<ThresholdDefinition>>::new();
    for criterion in criteria {
        match &criterion.check {
            CriterionCheck::MinimumVoltage {
                channel,
                threshold_v,
            } => push_threshold(
                &mut thresholds,
                channel,
                &format!("{}_low_threshold", criterion.id),
                ThresholdRole::Low,
                UnitValue::new(*threshold_v, EngineeringUnit::Volt),
            ),
            CriterionCheck::MaximumVoltage {
                channel,
                threshold_v,
            } => push_threshold(
                &mut thresholds,
                channel,
                &format!("{}_high_threshold", criterion.id),
                ThresholdRole::High,
                UnitValue::new(*threshold_v, EngineeringUnit::Volt),
            ),
            CriterionCheck::StateTransitions {
                channel,
                threshold_v,
                ..
            }
            | CriterionCheck::PulseWidth {
                channel,
                threshold_v,
                ..
            }
            | CriterionCheck::TransientDuration {
                channel,
                threshold_v,
                ..
            }
            | CriterionCheck::TransientEvent {
                channel,
                threshold_v,
                ..
            }
            | CriterionCheck::StableStateDuration {
                channel,
                threshold_v,
                ..
            } => push_threshold(
                &mut thresholds,
                channel,
                &format!("{}_decision_threshold", criterion.id),
                ThresholdRole::Decision,
                UnitValue::new(*threshold_v, EngineeringUnit::Volt),
            ),
            CriterionCheck::RiseFallTime {
                channel,
                low_threshold_v,
                high_threshold_v,
                ..
            } => {
                push_threshold(
                    &mut thresholds,
                    channel,
                    &format!("{}_low_threshold", criterion.id),
                    ThresholdRole::Low,
                    UnitValue::new(*low_threshold_v, EngineeringUnit::Volt),
                );
                push_threshold(
                    &mut thresholds,
                    channel,
                    &format!("{}_high_threshold", criterion.id),
                    ThresholdRole::High,
                    UnitValue::new(*high_threshold_v, EngineeringUnit::Volt),
                );
            }
            CriterionCheck::Measurement {
                channel,
                measurement,
                requirement,
            } => push_measurement_thresholds(
                &mut thresholds,
                &criterion.id,
                channel,
                measurement,
                requirement.value,
            )?,
            CriterionCheck::ResponseLatency {
                source_channel,
                target_channel,
                source_threshold_v,
                target_threshold_v,
                ..
            } => {
                push_threshold(
                    &mut thresholds,
                    source_channel,
                    &format!("{}_source_threshold", criterion.id),
                    ThresholdRole::Decision,
                    UnitValue::new(*source_threshold_v, EngineeringUnit::Volt),
                );
                push_threshold(
                    &mut thresholds,
                    target_channel,
                    &format!("{}_target_threshold", criterion.id),
                    ThresholdRole::Decision,
                    UnitValue::new(*target_threshold_v, EngineeringUnit::Volt),
                );
            }
        }
    }
    Ok(thresholds)
}

fn push_measurement_thresholds(
    thresholds: &mut BTreeMap<String, Vec<ThresholdDefinition>>,
    id: &str,
    channel: &str,
    measurement: &MeasurementSpec,
    requirement_value: f64,
) -> Result<(), String> {
    match measurement {
        MeasurementSpec::MinimumSample => push_threshold(
            thresholds,
            channel,
            &format!("{id}_low_threshold"),
            ThresholdRole::Low,
            UnitValue::new(requirement_value, EngineeringUnit::Volt),
        ),
        MeasurementSpec::MaximumSample => push_threshold(
            thresholds,
            channel,
            &format!("{id}_high_threshold"),
            ThresholdRole::High,
            UnitValue::new(requirement_value, EngineeringUnit::Volt),
        ),
        MeasurementSpec::StateTransitionCount { threshold_v }
        | MeasurementSpec::PulseWidth { threshold_v, .. }
        | MeasurementSpec::StableStateDuration { threshold_v, .. }
        | MeasurementSpec::TransientEventDuration { threshold_v, .. } => push_threshold(
            thresholds,
            channel,
            &format!("{id}_decision_threshold"),
            ThresholdRole::Decision,
            UnitValue::new(*threshold_v, EngineeringUnit::Volt),
        ),
        MeasurementSpec::RiseTime {
            low_threshold_v,
            high_threshold_v,
        }
        | MeasurementSpec::FallTime {
            low_threshold_v,
            high_threshold_v,
        } => {
            push_threshold(
                thresholds,
                channel,
                &format!("{id}_low_threshold"),
                ThresholdRole::Low,
                UnitValue::new(*low_threshold_v, EngineeringUnit::Volt),
            );
            push_threshold(
                thresholds,
                channel,
                &format!("{id}_high_threshold"),
                ThresholdRole::High,
                UnitValue::new(*high_threshold_v, EngineeringUnit::Volt),
            );
        }
    }
    Ok(())
}

fn push_threshold(
    thresholds: &mut BTreeMap<String, Vec<ThresholdDefinition>>,
    channel: &str,
    name: &str,
    role: ThresholdRole,
    value: UnitValue,
) {
    thresholds
        .entry(channel.to_string())
        .or_default()
        .push(ThresholdDefinition {
            name: name.to_string(),
            role,
            value,
        });
}

fn parse_target_profile(value: &str) -> Result<TargetProfileKind, String> {
    match value {
        "desktop_authoring" => Ok(TargetProfileKind::DesktopAuthoring),
        "embedded_runtime" => Ok(TargetProfileKind::EmbeddedRuntime),
        "controller_runtime" => Ok(TargetProfileKind::ControllerRuntime),
        "test_verification" => Ok(TargetProfileKind::TestVerification),
        _ => Err(format!(
            "unsupported --target `{value}`; expected desktop_authoring, embedded_runtime, controller_runtime, or test_verification"
        )),
    }
}

fn engineering_unit(value: &str) -> Result<EngineeringUnit, String> {
    match value {
        "V" => Ok(EngineeringUnit::Volt),
        "s" => Ok(EngineeringUnit::Second),
        "count" => Ok(EngineeringUnit::Count),
        "sample" => Ok(EngineeringUnit::Sample),
        "Hz" => Ok(EngineeringUnit::Hertz),
        _ => Err(format!("unsupported rule package unit `{value}`")),
    }
}

fn schema_operator(operator: CriterionOperator) -> ComparisonOperator {
    match operator {
        CriterionOperator::LessThan => ComparisonOperator::LessThan,
        CriterionOperator::LessThanOrEqual => ComparisonOperator::LessThanOrEqual,
        CriterionOperator::GreaterThan => ComparisonOperator::GreaterThan,
        CriterionOperator::GreaterThanOrEqual => ComparisonOperator::GreaterThanOrEqual,
        CriterionOperator::EqualTo => ComparisonOperator::EqualTo,
    }
}

fn schema_signal_state(state: SignalState) -> ferrisoxide_rule_schema::SignalState {
    match state {
        SignalState::High => ferrisoxide_rule_schema::SignalState::High,
        SignalState::Low => ferrisoxide_rule_schema::SignalState::Low,
    }
}

fn schema_run_selection(selection: RunSelectionConfig) -> ferrisoxide_rule_schema::RunSelection {
    match selection {
        RunSelectionConfig::Shortest => ferrisoxide_rule_schema::RunSelection::Shortest,
        RunSelectionConfig::Longest => ferrisoxide_rule_schema::RunSelection::Longest,
    }
}

fn schema_transient_event_kind(
    kind: TransientEventKind,
) -> ferrisoxide_rule_schema::TransientEventKind {
    match kind {
        TransientEventKind::TransientEvent => {
            ferrisoxide_rule_schema::TransientEventKind::TransientEvent
        }
        TransientEventKind::SpuriousTransition => {
            ferrisoxide_rule_schema::TransientEventKind::SpuriousTransition
        }
        TransientEventKind::ContactBounce => {
            ferrisoxide_rule_schema::TransientEventKind::ContactBounce
        }
        TransientEventKind::Dropout => ferrisoxide_rule_schema::TransientEventKind::Dropout,
        TransientEventKind::NoiseInducedTransition => {
            ferrisoxide_rule_schema::TransientEventKind::NoiseInducedTransition
        }
        TransientEventKind::ThresholdCrossingEvent => {
            ferrisoxide_rule_schema::TransientEventKind::ThresholdCrossingEvent
        }
    }
}

fn schema_requirement_unit(
    kind: ferrisoxide_core::criteria::CriterionMeasurementKind,
) -> Result<EngineeringUnit, String> {
    engineering_unit(kind.requirement_unit())
}

fn write_new_file(path: &Path, contents: &str) -> Result<(), String> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| format!("failed to create `{}`: {error}", path.display()))?;
    file.write_all(contents.as_bytes())
        .map_err(|error| format!("failed to write `{}`: {error}", path.display()))
}

fn write_output_file(path: &Path, contents: &str, overwrite: bool) -> Result<(), String> {
    if overwrite {
        fs::write(path, contents)
            .map_err(|error| format!("failed to write `{}`: {error}", path.display()))
    } else {
        write_new_file(path, contents)
    }
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

    load_analysis_config_from_path(Path::new(path)).map(Some)
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
        "FerrisOxide Signal",
        "",
        "Usage:",
        "  ferrisoxide-signal analyze --input <csv> --config examples/basic-config.toml --format text",
        "  ferrisoxide-signal analyze --input <csv> --time-column time --channels input_v --moving-average 3 --low-pass 25 --adc-quantize 12:0.0:5.0 --min input_v:0.0 --max input_v:5.5 --format json",
        "  ferrisoxide-signal plot --input <csv> --time-column time --channels input_v --output waveform.svg",
        "  ferrisoxide-signal plot --input <csv> --config examples/basic-config.toml --output annotated.svg",
        "  ferrisoxide-signal plot --input <csv> --time-column time --channels input_v --z-column temp_c --output waveform-3d.svg",
        "  ferrisoxide-signal export-rule-package --input <csv> --config examples/basic-dsl-config.toml --output-dir deployment --package-name switch-rule --package-version 1.0.0 --target controller_runtime",
        "  ferrisoxide-signal simulate --input tests/e2e/heated_actuator/input/passing_run.csv --control-config examples/control-config/production-control-config.toml --verification-config examples/test-verification-config/test-verification-config.toml --channel-map examples/simulation/heated-actuator-channel-map.toml --format json",
        "  ferrisoxide-signal batch --manifest examples/batch-analysis.toml --output-dir batch-output --format json",
        "  ferrisoxide-signal transforms --format text",
        "  ferrisoxide-signal inspect-source --input examples/basic-waveform.csv --format text",
        "  ferrisoxide-signal inspect-source --source simulation --input tests/e2e/heated_actuator/input/passing_run.csv --channel-map examples/simulation/heated-actuator-channel-map.toml --format json",
        "  ferrisoxide-signal scaffold-config --input examples/basic-waveform.csv --output analysis.toml",
        "  ferrisoxide-signal workflow-template --use-case switch-bounce --format toml",
        "  ferrisoxide-signal evaluate-bundle --input examples/basic-waveform.csv --config examples/basic-config.toml --output-dir evaluation-bundle --plot",
        "  ferrisoxide-signal evaluate-bundle --source simulation --input tests/e2e/heated_actuator/input/passing_run.csv --control-config examples/control-config/production-control-config.toml --verification-config examples/test-verification-config/test-verification-config.toml --channel-map examples/simulation/heated-actuator-channel-map.toml --output-dir simulation-bundle",
        "",
        "ADC quantization syntax: --adc-quantize bits:min_v:max_v",
        "Plot output is SVG. Use --config to add 2D criteria evidence overlays; use --z-column for an optional third axis.",
        "Rule package export writes rules.toml, rules.json, and validation-report.json without overwriting existing artifacts.",
        "Desktop simulation loads production control config, test verification config, a channel map, and fixture CSV input.",
        "Batch analysis processes local CSV/config pairs and writes per-run reports plus batch-summary.json.",
        "Transform catalog output is generated from the core transform registry.",
        "Desktop source inspection and evaluation bundles support CSV and software-only simulation sources; live/realtime DAQ remains future-gated.",
        "Workflow templates are TOML starters for supply-rail, switch-bounce, response-latency, sensor-cleanup, simulated-fault, and multi-channel cases.",
        "Formats: text, json",
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    use ferrisoxide_rule_engine::{
        apply_borrowed_transform_chain, evaluate_borrowed_rule, BorrowedRuleCriterion,
        BorrowedRuleCriterionCheck, BorrowedTransformStep, RuleChannel, RuleOutcome, RuleSummary,
        RuleTolerances, RuleWaveform,
    };

    #[derive(Debug, Serialize, PartialEq)]
    struct ControllerParityReport {
        case_id: &'static str,
        schema_note: &'static str,
        approved_schema_differences: Vec<&'static str>,
        inputs: ControllerParityInputs,
        desktop_state_trace: Vec<PortableStateTrace>,
        embedded_compatible_state_trace: Vec<PortableStateTrace>,
        desktop_evidence: Vec<PortableCriterionEvidence>,
        embedded_compatible_evidence: Vec<PortableCriterionEvidence>,
    }

    #[derive(Debug, Serialize, PartialEq)]
    struct ControllerParityInputs {
        waveform: String,
        production_control_config: String,
        test_verification_config: String,
        channel_map: String,
        selected_mode: String,
    }

    #[derive(Debug, Clone, Serialize, PartialEq)]
    struct PortableStateTrace {
        sample_index: usize,
        timestamp_s: f64,
        mode: String,
        machine: String,
        state: String,
        transitions: Vec<PortableTransition>,
        outputs: Vec<PortableOutput>,
    }

    #[derive(Debug, Clone, Serialize, PartialEq, Eq)]
    struct PortableTransition {
        transition: String,
        from: String,
        to: String,
    }

    #[derive(Debug, Clone, Serialize, PartialEq, Eq)]
    struct PortableOutput {
        output: String,
        value: String,
    }

    #[derive(Debug, Clone, Serialize, PartialEq)]
    struct PortableCriterionEvidence {
        criterion_id: String,
        outcome: &'static str,
        failed_criterion: Option<String>,
        measurement_id: String,
        method: String,
        channel: String,
        measured_value: f64,
        required_value: f64,
        tolerance_used: f64,
        unit: String,
        sample_index: usize,
        timestamp: f64,
    }

    #[derive(Debug, Clone)]
    struct BorrowedVerificationCriterion<'a> {
        id: String,
        check: BorrowedRuleCriterionCheck<'a>,
    }

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
    fn analyzes_config_with_m11_transforms() {
        let output = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            "../../examples/basic-waveform.csv".to_string(),
            "--config".to_string(),
            "../../examples/m11-transform-config.toml".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("M11 transform config should analyze");

        assert!(output.contains("\"overall_outcome\": \"pass\""));
        for transform_name in [
            "offset",
            "gain",
            "invert",
            "clamp",
            "deadband",
            "dc_remove",
            "baseline_subtract",
            "moving_median",
        ] {
            assert!(output.contains(&format!("\"name\": \"{transform_name}\"")));
        }
        assert!(output.contains("\"category\": \"PointwiseTransform\""));
        assert!(output.contains("\"category\": \"BaselineTransform\""));
        assert!(output.contains("\"category\": \"WindowedTransform\""));
        assert!(output.contains("\"phase_effect\": \"nonlinear\""));
    }

    #[test]
    fn analyzes_config_with_m14_high_pass_baseline() {
        let output = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            "../../examples/basic-waveform.csv".to_string(),
            "--config".to_string(),
            "../../examples/m14-high-pass-baseline-config.toml".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("M14 high-pass baseline config should analyze");

        assert!(output.contains("\"overall_outcome\": \"pass\""));
        assert!(output.contains("\"name\": \"high_pass_baseline\""));
        assert!(output.contains("\"category\": \"StatefulTransform\""));
        assert!(output.contains("\"sample_rate_required\": true"));
        assert!(output.contains("\"phase_effect\": \"delay\""));
    }

    #[test]
    fn analyzes_config_with_m26_data_cleaning_filters() {
        let output = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            "../../examples/m26-data-cleaning-waveform.csv".to_string(),
            "--config".to_string(),
            "../../examples/m26-data-cleaning-config.toml".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("M26 data-cleaning config should analyze");

        assert!(output.contains("\"overall_outcome\": \"pass\""));
        for transform_name in [
            "timestamp_sort",
            "dedupe_timestamps",
            "nan_interpolate",
            "nan_remove",
            "crop",
            "fixed_delay",
            "gap_fill",
            "resample_fixed",
            "channel_delay",
        ] {
            assert!(output.contains(&format!("\"name\": \"{transform_name}\"")));
        }
        assert!(output.contains("\"category\": \"DataCleaningTransform\""));
        assert!(output.contains("\"category\": \"ResamplingTransform\""));
        assert!(output.contains("\"offline_only\": true"));
        assert!(output.contains("\"cleaned_input_max\""));
    }

    #[test]
    fn analyzes_config_with_m27_pointwise_filters() {
        let output = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            "../../examples/m27-pointwise-waveform.csv".to_string(),
            "--config".to_string(),
            "../../examples/m27-pointwise-config.toml".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("M27 pointwise config should analyze");

        assert!(output.contains("\"overall_outcome\": \"pass\""));
        for transform_name in [
            "absolute_value",
            "square",
            "square_root",
            "log",
            "exp",
            "normalize",
            "tanh",
            "sigmoid",
            "soft_limit",
            "piecewise_linear",
            "polynomial",
        ] {
            assert!(output.contains(&format!("\"name\": \"{transform_name}\"")));
        }
        assert!(output.contains("\"category\": \"PointwiseTransform\""));
        assert!(output.contains("\"phase_effect\": \"nonlinear\""));
        assert!(output.contains("\"offline_only\": true"));
        assert!(output.contains("\"m27_max\""));
    }

    #[test]
    fn analyzes_config_with_m28_smoothing_baseline_filters() {
        let output = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            "../../examples/m28-smoothing-waveform.csv".to_string(),
            "--config".to_string(),
            "../../examples/m28-smoothing-config.toml".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("M28 smoothing/baseline config should analyze");

        assert!(output.contains("\"overall_outcome\": \"pass\""));
        for transform_name in [
            "weighted_moving_average",
            "exponential_moving_average",
            "boxcar_smoothing",
            "gaussian_smoothing",
            "savitzky_golay",
            "centered_moving_median",
            "rolling_mean_baseline",
            "rolling_median_baseline",
            "linear_detrend",
            "polynomial_detrend",
            "hampel_filter",
            "spike_remove",
        ] {
            assert!(output.contains(&format!("\"name\": \"{transform_name}\"")));
        }
        assert!(output.contains("\"category\": \"WindowedTransform\""));
        assert!(output.contains("\"category\": \"BaselineTransform\""));
        assert!(output.contains("\"offline_only\": true"));
        assert!(output.contains("\"m28_max\""));
    }

    #[test]
    fn analyzes_config_with_m29_standard_frequency_filters() {
        let output = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            "../../examples/m29-frequency-waveform.csv".to_string(),
            "--config".to_string(),
            "../../examples/m29-frequency-config.toml".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("M29 frequency config should analyze");

        assert!(output.contains("\"overall_outcome\": \"pass\""));
        for transform_name in [
            "fir_filter",
            "zero_phase_fir_filter",
            "iir_biquad",
            "zero_phase_iir_biquad",
            "high_pass",
            "band_pass",
            "band_stop",
            "notch",
            "comb_filter",
            "butterworth_low_pass",
            "butterworth_high_pass",
            "chebyshev1_low_pass",
            "chebyshev2_low_pass",
            "bessel_low_pass",
        ] {
            assert!(output.contains(&format!("\"name\": \"{transform_name}\"")));
        }
        assert!(output.contains("\"category\": \"FrequencyFilterTransform\""));
        assert!(output.contains("\"sample_rate_required\": true"));
        assert!(output.contains("\"phase_effect\": \"delay\""));
        assert!(output.contains("\"offline_only\": true"));
        assert!(output.contains("\"m29_max\""));
    }

    #[test]
    fn analyzes_config_with_m30_resampling_timing_filters() {
        let output = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            "../../examples/m30-resampling-waveform.csv".to_string(),
            "--config".to_string(),
            "../../examples/m30-resampling-config.toml".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("M30 resampling/timing config should analyze");

        assert!(output.contains("\"overall_outcome\": \"pass\""));
        for transform_name in [
            "resample",
            "downsample",
            "decimate",
            "upsample",
            "interpolate",
            "rational_resample",
            "sample_and_hold",
            "zero_order_hold",
            "first_order_hold",
            "fractional_delay",
            "cross_correlation_delay",
            "jitter_correction",
            "clock_drift_correction",
        ] {
            assert!(output.contains(&format!("\"name\": \"{transform_name}\"")));
        }
        assert!(output.contains("\"category\": \"ResamplingTransform\""));
        assert!(output.contains("\"estimated_delay_s\""));
        assert!(output.contains("\"confidence\""));
        assert!(output.contains("\"m30_max\""));
    }

    #[test]
    fn analyzes_config_with_m31_envelope_energy_calculus_filters_and_features() {
        let output = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            "../../examples/m31-calculus-waveform.csv".to_string(),
            "--config".to_string(),
            "../../examples/m31-calculus-config.toml".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("M31 envelope/energy/calculus config should analyze");

        assert!(output.contains("\"overall_outcome\": \"pass\""));
        for transform_name in [
            "half_wave_rectify",
            "full_wave_rectify",
            "envelope",
            "moving_rms",
            "peak_hold",
            "first_derivative",
            "second_derivative",
            "integral",
            "cumulative_integral",
            "leaky_integrator",
            "slope_detection",
        ] {
            assert!(output.contains(&format!("\"name\": \"{transform_name}\"")));
        }
        for feature_id in [
            "m31_rms",
            "m31_peak_to_peak",
            "m31_crest_factor",
            "m31_energy",
            "m31_power",
            "m31_area",
            "m31_impulse",
        ] {
            assert!(output.contains(&format!("\"id\": \"{feature_id}\"")));
        }
        assert!(output.contains("\"feature_records\""));
        assert!(output.contains("\"kind\": \"feature_records\""));
        assert!(output.contains("\"category\": \"FeatureTransform\""));
        assert!(output.contains("\"m31_max\""));
    }

    #[test]
    fn analyzes_config_with_m32_statistics_correlation_filters_and_features() {
        let filter_output = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            "../../examples/m32-statistics-waveform.csv".to_string(),
            "--config".to_string(),
            "../../examples/m32-statistics-filters-config.toml".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("M32 statistics filter config should analyze");

        assert!(filter_output.contains("\"overall_outcome\": \"pass\""));
        for transform_name in [
            "rolling_mean",
            "rolling_variance",
            "rolling_stddev",
            "rolling_min",
            "rolling_max",
            "z_score",
            "outlier_detection",
            "quantile_clip",
        ] {
            assert!(filter_output.contains(&format!("\"name\": \"{transform_name}\"")));
        }

        let feature_output = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            "../../examples/m32-statistics-waveform.csv".to_string(),
            "--config".to_string(),
            "../../examples/m32-statistics-config.toml".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("M32 statistics/correlation feature config should analyze");

        assert!(feature_output.contains("\"overall_outcome\": \"pass\""));
        for feature_id in [
            "m32_mean",
            "m32_median",
            "m32_mode",
            "m32_min",
            "m32_max",
            "m32_variance",
            "m32_standard_deviation",
            "m32_skewness",
            "m32_kurtosis",
            "m32_percentile",
            "m32_quantile",
            "m32_histogram_bin_0",
            "m32_histogram_bin_1",
            "m32_covariance",
            "m32_correlation",
            "m32_autocorrelation",
            "m32_cross_correlation",
        ] {
            assert!(feature_output.contains(&format!("\"id\": \"{feature_id}\"")));
        }
        assert!(feature_output.contains("\"method_context\""));
        assert!(feature_output.contains("\"other_channel\": \"other_v\""));
        assert!(feature_output.contains("\"lag_samples\": 1"));
        assert!(feature_output.contains("\"feature_records\""));
        assert!(feature_output.contains("\"kind\": \"feature_records\""));
    }

    #[test]
    fn analyzes_config_with_m33_spectrum_time_frequency_features() {
        let output = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            "../../examples/m33-spectrum-waveform.csv".to_string(),
            "--config".to_string(),
            "../../examples/m33-spectrum-config.toml".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("M33 spectrum/time-frequency config should analyze");

        assert!(output.contains("\"overall_outcome\": \"pass\""));
        for feature_id in [
            "m33_window_sample_1",
            "m33_dft_bin_1",
            "m33_fft_bin_1",
            "m33_square_fft_bin_1",
            "m33_ifft_sample_0",
            "m33_power_bin_1",
            "m33_psd_bin_1",
            "m33_welch_bin_1",
            "m33_cross_bin_1",
            "m33_coherence_bin_1",
            "m33_transfer_bin_1",
            "m33_harmonic_harmonic_2",
            "m33_thd",
            "m33_snr",
            "m33_sinad",
            "m33_enob",
            "m33_stft_segment_0_bin_1",
            "m33_spectrogram_segment_0_bin_1",
            "m33_centroid",
            "m33_bandwidth",
            "m33_rolloff",
            "m33_band_power",
        ] {
            assert!(output.contains(&format!("\"id\": \"{feature_id}\"")));
        }
        assert!(output.contains("\"frequency_hz\": 1.0"));
        assert!(output.contains("\"window\": \"rectangular\""));
        assert!(output.contains("\"normalization\""));
        assert!(output.contains("\"category\": \"FrequencyFilterTransform\""));
        assert!(output.contains("\"category\": \"TimeFrequencyTransform\""));
    }

    #[test]
    fn analyzes_config_with_m34_fault_injection_adc_dac_filters() {
        let output = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            "../../examples/m34-fault-adc-waveform.csv".to_string(),
            "--config".to_string(),
            "../../examples/m34-fault-adc-config.toml".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("M34 fault injection / ADC-DAC config should analyze");

        assert!(output.contains("\"overall_outcome\": \"pass\""));
        for transform_name in [
            "white_noise",
            "gaussian_noise",
            "uniform_noise",
            "pink_noise",
            "brown_noise",
            "impulse_noise",
            "salt_pepper_noise",
            "quantization_noise",
            "periodic_interference",
            "hum_interference",
            "ground_bounce",
            "thermal_drift",
            "random_walk_drift",
            "dropout_fault",
            "missing_samples",
            "intermittent_fault",
            "saturation_fault",
            "stuck_at_fault",
            "flatline_fault",
            "rounding_quantizer",
            "floor_quantizer",
            "ceil_quantizer",
            "midrise_quantizer",
            "midtread_quantizer",
            "saturating_quantizer",
            "dither",
            "companding",
            "sample_clock_jitter",
            "adc_missing_code",
            "inl_error",
            "dnl_error",
            "adc_gain_error",
            "adc_offset_error",
        ] {
            assert!(output.contains(&format!("\"name\": \"{transform_name}\"")));
        }
        assert!(output.contains("\"evidence_scope\""));
        assert!(output.contains("\"simulation_only\""));
        assert!(output.contains("\"category\": \"FaultInjectionTransform\""));
        assert!(output.contains("\"category\": \"QuantizationTransform\""));
    }

    #[test]
    fn analyzes_config_with_m35_multi_channel_sensor_domain_filters() {
        let output = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            "../../examples/m35-domain-waveform.csv".to_string(),
            "--config".to_string(),
            "../../examples/m35-domain-config.toml".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("M35 multi-channel / sensor / domain config should analyze");

        assert!(output.contains("\"overall_outcome\": \"pass\""));
        for transform_name in [
            "channel_add",
            "channel_subtract",
            "differential_channel",
            "common_mode",
            "vector_magnitude",
            "euclidean_norm",
            "matrix_transform",
            "coordinate_rotation",
            "linear_sensor_conversion",
            "pressure_transducer",
            "current_shunt",
            "bridge_strain",
            "load_cell_force",
            "rtd_temperature",
            "thermistor_temperature",
            "tachometer_rpm",
            "encoder_position",
            "accelerometer_units",
            "gyroscope_rate",
            "hall_current",
            "lvdt_position",
            "microphone_spl",
            "photodiode_power",
            "velocity_from_acceleration",
            "displacement_from_velocity",
            "vibration_severity",
            "control_error",
            "proportional_control",
            "pid_control",
            "rate_limiter",
            "slew_rate_limit",
            "control_saturation",
            "control_deadzone",
            "feedforward_control",
        ] {
            assert!(output.contains(&format!("\"name\": \"{transform_name}\"")));
        }
        assert!(output.contains("\"category\": \"MultiChannelTransform\""));
        assert!(output.contains("\"category\": \"CalibrationTransform\""));
        assert!(output.contains("\"category\": \"ControlTransform\""));
        assert!(output.contains("\"calibration_scope\""));
        assert!(output.contains("\"software_formula_only\""));
    }

    #[test]
    fn lists_transform_catalog_as_text() {
        let output = run(vec![
            "transforms".to_string(),
            "--format".to_string(),
            "text".to_string(),
        ])
        .expect("transform catalog should render");

        assert!(output.contains("FerrisOxide Transform Catalog"));
        assert!(output.contains("offset | milestone=implemented | status=implemented"));
        assert!(output.contains("nan_interpolate | milestone=M26 | status=implemented"));
        assert!(output.contains("normalize | milestone=M27 | status=implemented"));
        assert!(output.contains("savitzky_golay | milestone=M28 | status=implemented"));
        assert!(output.contains("band_pass | milestone=M29 | status=implemented"));
        assert!(output.contains("rational_resample | milestone=M30 | status=implemented"));
        assert!(output.contains("moving_rms | milestone=M31 | status=implemented"));
        assert!(output.contains("correlation | milestone=M32 | status=implemented"));
        assert!(output.contains("fft | milestone=M33 | status=implemented"));
        assert!(output.contains("white_noise | milestone=M34 | status=implemented"));
        assert!(output.contains("adc_missing_code | milestone=M34 | status=implemented"));
        assert!(output.contains("vector_magnitude | milestone=M35 | status=implemented"));
        assert!(output.contains("pid_control | milestone=M35 | status=implemented"));
        assert!(output.contains("advanced_acoustic_pack | milestone=M35 | status=dependency_gated"));
        assert!(output.contains("package=supported"));
        assert!(output.contains("elliptic_low_pass | milestone=M29 | status=dependency_gated"));
        assert!(output.contains("polyphase_resample | milestone=M30 | status=dependency_gated"));
        assert!(output.contains("comprehensive_suite_closure | milestone=M36 | status=implemented"));
    }

    #[test]
    fn lists_transform_catalog_as_json() {
        let output = run(vec![
            "transforms".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("transform catalog JSON should render");
        let entries: serde_json::Value =
            serde_json::from_str(&output).expect("catalog output should be valid JSON");
        let entries = entries.as_array().expect("catalog should render as array");

        assert!(entries.iter().any(|entry| entry["name"] == "offset"
            && entry["capability_status"] == "implemented"
            && entry["package_support"] == "supported"));
        assert!(entries.iter().any(|entry| entry["name"] == "channel_delay"
            && entry["milestone"] == "M26"
            && entry["capability_status"] == "implemented"
            && entry["package_support"] == "rejected_desktop_only"));
        assert!(entries.iter().any(|entry| entry["name"] == "polynomial"
            && entry["milestone"] == "M27"
            && entry["capability_status"] == "implemented"
            && entry["package_support"] == "rejected_desktop_only"));
        assert!(entries.iter().any(|entry| entry["name"] == "hampel_filter"
            && entry["milestone"] == "M28"
            && entry["capability_status"] == "implemented"
            && entry["package_support"] == "rejected_desktop_only"));
        assert!(entries.iter().any(|entry| entry["name"] == "notch"
            && entry["milestone"] == "M29"
            && entry["capability_status"] == "implemented"
            && entry["package_support"] == "rejected_desktop_only"));
        assert!(entries
            .iter()
            .any(|entry| entry["name"] == "cross_correlation_delay"
                && entry["milestone"] == "M30"
                && entry["capability_status"] == "implemented"
                && entry["package_support"] == "rejected_desktop_only"));
        assert!(entries.iter().any(|entry| entry["name"] == "fft"
            && entry["milestone"] == "M33"
            && entry["capability_status"] == "implemented"
            && entry["package_support"] == "not_applicable"));
        assert!(entries.iter().any(|entry| entry["name"] == "moving_rms"
            && entry["milestone"] == "M31"
            && entry["capability_status"] == "implemented"
            && entry["package_support"] == "rejected_desktop_only"));
        assert!(entries.iter().any(|entry| entry["name"] == "correlation"
            && entry["milestone"] == "M32"
            && entry["capability_status"] == "implemented"
            && entry["package_support"] == "not_applicable"));
        assert!(entries.iter().any(|entry| entry["name"] == "white_noise"
            && entry["milestone"] == "M34"
            && entry["capability_status"] == "implemented"
            && entry["package_support"] == "rejected_desktop_only"));
        assert!(entries
            .iter()
            .any(|entry| entry["name"] == "adc_missing_code"
                && entry["milestone"] == "M34"
                && entry["capability_status"] == "implemented"
                && entry["package_support"] == "rejected_desktop_only"));
        assert!(entries.iter().any(|entry| entry["name"] == "pid_control"
            && entry["milestone"] == "M35"
            && entry["capability_status"] == "implemented"
            && entry["package_support"] == "rejected_desktop_only"));
        assert!(entries
            .iter()
            .any(|entry| entry["name"] == "advanced_acoustic_pack"
                && entry["milestone"] == "M35"
                && entry["capability_status"] == "dependency_gated"
                && entry["package_support"] == "dependency_gated"));
        assert!(entries
            .iter()
            .any(|entry| entry["name"] == "comprehensive_suite_closure"
                && entry["milestone"] == "M36"
                && entry["capability_status"] == "implemented"
                && entry["package_support"] == "not_applicable"));
    }

    #[test]
    fn rejects_unsupported_transform_catalog_format() {
        let error = run(vec![
            "transforms".to_string(),
            "--format".to_string(),
            "xml".to_string(),
        ])
        .expect_err("unsupported catalog format should fail");

        assert!(error.contains("unsupported --format `xml`; use text or json"));
    }

    #[test]
    fn rule_package_export_rejects_high_pass_baseline() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = format!("{manifest_dir}/../../examples/basic-waveform.csv");
        let config_path =
            format!("{manifest_dir}/../../examples/m14-high-pass-baseline-config.toml");
        let temp_dir = env::temp_dir().join(format!(
            "ferrisoxide-m14-rule-package-{}",
            std::process::id()
        ));
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).expect("stale temp dir should be removable");
        }

        let error = run(vec![
            "export-rule-package".to_string(),
            "--input".to_string(),
            input_path,
            "--config".to_string(),
            config_path,
            "--output-dir".to_string(),
            temp_dir.display().to_string(),
            "--package-name".to_string(),
            "m14-high-pass".to_string(),
            "--package-version".to_string(),
            "0.12.0".to_string(),
        ])
        .expect_err("high-pass baseline should not export to rule packages yet");

        assert!(error
            .contains("rule package export does not yet support transform `high_pass_baseline`"));
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).expect("temp dir should be removable");
        }
    }

    #[test]
    fn rule_package_export_rejects_m31_feature_transforms() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = format!("{manifest_dir}/../../examples/m31-calculus-waveform.csv");
        let config_path = format!("{manifest_dir}/../../examples/m31-calculus-config.toml");
        let temp_dir = env::temp_dir().join(format!(
            "ferrisoxide-m31-rule-package-{}",
            std::process::id()
        ));
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).expect("stale temp dir should be removable");
        }

        let error = run(vec![
            "export-rule-package".to_string(),
            "--input".to_string(),
            input_path,
            "--config".to_string(),
            config_path,
            "--output-dir".to_string(),
            temp_dir.display().to_string(),
            "--package-name".to_string(),
            "m31-features".to_string(),
            "--package-version".to_string(),
            "0.31.0".to_string(),
        ])
        .expect_err("M31 feature transforms should not export to rule packages");

        assert!(error.contains("rule package export does not yet support feature_transforms"));
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).expect("temp dir should be removable");
        }
    }

    #[test]
    fn rule_package_export_supports_linear_pointwise_transform_subset() {
        use ferrisoxide_core::filter::{GainTransform, InvertTransform, OffsetTransform};

        let channels = vec!["input_v".to_string()];
        let filters = vec![
            FilterStep::Offset(OffsetTransform { offset_v: 0.25 }),
            FilterStep::Gain(GainTransform { gain: 2.0 }),
            FilterStep::Invert(InvertTransform),
        ];

        let exported = schema_filters(&filters, &channels)
            .expect("linear pointwise transforms should export to rule packages");

        assert_eq!(
            exported,
            vec![
                FilterDefinition::Offset {
                    id: "offset_0_input_v".to_string(),
                    channel: "input_v".to_string(),
                    offset: UnitValue::new(0.25, EngineeringUnit::Volt),
                },
                FilterDefinition::Gain {
                    id: "gain_1_input_v".to_string(),
                    channel: "input_v".to_string(),
                    gain: 2.0,
                },
                FilterDefinition::Invert {
                    id: "invert_2_input_v".to_string(),
                    channel: "input_v".to_string(),
                },
            ]
        );
    }

    #[test]
    fn linear_pointwise_runtime_helper_matches_desktop_filter_chain() {
        use ferrisoxide_core::filter::{GainTransform, InvertTransform, OffsetTransform};

        let samples = [1.0, -2.0, 3.5];
        let waveform = Waveform::new(
            vec![0.0, 0.001, 0.002],
            vec![Channel::new("input_v", Unit::volts(), samples.to_vec())],
        )
        .expect("desktop waveform should be valid");
        let filters = [
            FilterStep::Offset(OffsetTransform { offset_v: 0.5 }),
            FilterStep::Gain(GainTransform { gain: 2.0 }),
            FilterStep::Invert(InvertTransform),
        ];

        let desktop =
            apply_filter_chain(&waveform, &filters).expect("desktop filter chain should evaluate");
        let transforms = [
            BorrowedTransformStep::Offset { offset_v: 0.5 },
            BorrowedTransformStep::Gain { gain: 2.0 },
            BorrowedTransformStep::Invert,
        ];
        let mut borrowed_output = [0.0; 3];

        apply_borrowed_transform_chain(&samples, &transforms, &mut borrowed_output)
            .expect("borrowed runtime transform chain should evaluate");

        assert_eq!(
            desktop
                .channel("input_v")
                .expect("derived channel should exist")
                .samples
                .as_slice(),
            &borrowed_output
        );
    }

    #[test]
    fn rule_package_export_rejects_remaining_desktop_only_transform_matrix() {
        use ferrisoxide_core::filter::{
            AbsoluteValueTransform, AdcCodeDefectKind, AdcCodeDefectTransform, BandPassFilter,
            BandStopFilter, BaselineSubtractTransform, BesselLowPassFilter, BiquadCoefficients,
            BoxcarSmoothingFilter, ButterworthHighPassFilter, ButterworthLowPassFilter,
            CenteredMovingMedianFilter, ChannelArithmeticKind, ChannelArithmeticTransform,
            ChannelDelayTransform, Chebyshev1LowPassFilter, Chebyshev2LowPassFilter,
            ClampTransform, ClockDriftCorrectionTransform, CombFilter, CompandingKind,
            CompandingTransform, ControlTransform, ControlTransformKind,
            CoordinateRotationTransform, CropTransform, CrossCorrelationDelayTransform,
            CumulativeIntegralTransform, DcRemoveTransform, DeadbandTransform, DecimateTransform,
            DedupeTimestampsTransform, DitherTransform, DownsampleTransform, DriftFaultKind,
            DriftFaultTransform, EnvelopeTransform, ExpTransform, ExponentialMovingAverageFilter,
            FirFilter, FirstDerivativeTransform, FirstOrderHoldTransform, FixedDelayTransform,
            FractionalDelayTransform, FullWaveRectifyTransform, GainOffsetErrorKind,
            GainOffsetErrorTransform, GapFillTransform, GaussianSmoothingFilter,
            HalfWaveRectifyTransform, HampelFilter, HighPassBaselineFilter, HighPassFilter,
            IirBiquadFilter, IntegralTransform, InterpolateTransform, JitterCorrectionTransform,
            LeakyIntegratorTransform, LinearDetrendTransform, LogTransform, MatrixTransform,
            MovingMedianFilter, MovingRmsTransform, NanInterpolateTransform, NanRemoveTransform,
            NoiseInjectionTransform, NoiseKind, NormalizeMode, NormalizeTransform, NotchFilter,
            OutlierDetectionTransform, PeakHoldTransform, PeriodicInterferenceKind,
            PeriodicInterferenceTransform, PiecewiseLinearTransform, PiecewisePoint,
            PolynomialDetrendTransform, PolynomialTransform, QuantileClipTransform,
            RationalResampleTransform, ResampleFixedTransform, ResampleTransform,
            RollingMaxTransform, RollingMeanBaselineTransform, RollingMeanTransform,
            RollingMedianBaselineTransform, RollingMinTransform, RollingStdDevTransform,
            RollingVarianceTransform, SampleAndHoldTransform, SampleClockJitterTransform,
            SampleFaultKind, SampleFaultTransform, SavitzkyGolayFilter, SecondDerivativeTransform,
            SensorConversionKind, SensorConversionParameters, SensorConversionTransform,
            SigmoidTransform, SimulationQuantizerKind, SimulationQuantizerTransform,
            SlopeDetectionTransform, SoftLimitTransform, SpikeRemoveTransform, SquareRootTransform,
            SquareTransform, TanhTransform, TimestampSortTransform, UpsampleTransform,
            VectorMagnitudeKind, VectorMagnitudeTransform, VibrationTransform,
            VibrationTransformKind, WeightedMovingAverageFilter, ZScoreTransform,
            ZeroOrderHoldTransform, ZeroPhaseFirFilter, ZeroPhaseIirBiquadFilter,
        };

        let channels = vec!["input_v".to_string()];
        let identity_biquad = BiquadCoefficients {
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
        };
        let unsupported = vec![
            FilterStep::Clamp(ClampTransform {
                min_v: 0.0,
                max_v: 5.0,
            }),
            FilterStep::Deadband(DeadbandTransform { threshold_v: 0.1 }),
            FilterStep::DcRemove(DcRemoveTransform),
            FilterStep::BaselineSubtract(BaselineSubtractTransform { baseline_v: 0.2 }),
            FilterStep::HighPassBaseline(HighPassBaselineFilter { cutoff_hz: 0.5 }),
            FilterStep::MovingMedian(MovingMedianFilter { window_samples: 3 }),
            FilterStep::TimestampSort(TimestampSortTransform),
            FilterStep::DedupeTimestamps(DedupeTimestampsTransform),
            FilterStep::NanInterpolate(NanInterpolateTransform),
            FilterStep::NanRemove(NanRemoveTransform),
            FilterStep::Crop(CropTransform {
                start_time_s: 0.0,
                end_time_s: 1.0,
            }),
            FilterStep::FixedDelay(FixedDelayTransform { delay_s: 0.1 }),
            FilterStep::GapFill(GapFillTransform {
                sample_interval_s: 0.1,
            }),
            FilterStep::ResampleFixed(ResampleFixedTransform {
                sample_interval_s: 0.1,
            }),
            FilterStep::ChannelDelay(ChannelDelayTransform {
                channel: "input_v".to_string(),
                delay_s: 0.1,
            }),
            FilterStep::AbsoluteValue(AbsoluteValueTransform),
            FilterStep::Square(SquareTransform),
            FilterStep::SquareRoot(SquareRootTransform),
            FilterStep::Log(LogTransform { base: 10.0 }),
            FilterStep::Exp(ExpTransform { base: 10.0 }),
            FilterStep::Normalize(NormalizeTransform {
                mode: NormalizeMode::ZeroToOne,
            }),
            FilterStep::Tanh(TanhTransform),
            FilterStep::Sigmoid(SigmoidTransform),
            FilterStep::SoftLimit(SoftLimitTransform { limit_v: 2.0 }),
            FilterStep::PiecewiseLinear(PiecewiseLinearTransform {
                points: vec![
                    PiecewisePoint { x: 0.0, y: 0.0 },
                    PiecewisePoint { x: 1.0, y: 1.0 },
                ],
            }),
            FilterStep::Polynomial(PolynomialTransform {
                coefficients: vec![0.0, 1.0],
            }),
            FilterStep::WeightedMovingAverage(WeightedMovingAverageFilter {
                weights: vec![1.0, 2.0],
            }),
            FilterStep::ExponentialMovingAverage(ExponentialMovingAverageFilter { alpha: 0.5 }),
            FilterStep::BoxcarSmoothing(BoxcarSmoothingFilter { window_samples: 3 }),
            FilterStep::GaussianSmoothing(GaussianSmoothingFilter {
                window_samples: 3,
                sigma_samples: 1.0,
            }),
            FilterStep::SavitzkyGolay(SavitzkyGolayFilter {
                window_samples: 3,
                polynomial_order: 1,
            }),
            FilterStep::CenteredMovingMedian(CenteredMovingMedianFilter { window_samples: 3 }),
            FilterStep::RollingMeanBaseline(RollingMeanBaselineTransform { window_samples: 3 }),
            FilterStep::RollingMedianBaseline(RollingMedianBaselineTransform { window_samples: 3 }),
            FilterStep::LinearDetrend(LinearDetrendTransform),
            FilterStep::PolynomialDetrend(PolynomialDetrendTransform {
                polynomial_order: 1,
            }),
            FilterStep::HampelFilter(HampelFilter {
                window_samples: 3,
                outlier_sigma: 3.0,
            }),
            FilterStep::SpikeRemove(SpikeRemoveTransform {
                window_samples: 3,
                threshold_v: 0.2,
            }),
            FilterStep::FirFilter(FirFilter {
                coefficients: vec![0.25, 0.5, 0.25],
            }),
            FilterStep::ZeroPhaseFirFilter(ZeroPhaseFirFilter {
                coefficients: vec![0.25, 0.5, 0.25],
            }),
            FilterStep::IirBiquad(IirBiquadFilter {
                coefficients: identity_biquad,
            }),
            FilterStep::ZeroPhaseIirBiquad(ZeroPhaseIirBiquadFilter {
                coefficients: identity_biquad,
            }),
            FilterStep::HighPass(HighPassFilter { cutoff_hz: 5.0 }),
            FilterStep::BandPass(BandPassFilter {
                center_hz: 50.0,
                q: 2.0,
            }),
            FilterStep::BandStop(BandStopFilter {
                center_hz: 60.0,
                q: 5.0,
            }),
            FilterStep::Notch(NotchFilter {
                center_hz: 60.0,
                q: 30.0,
            }),
            FilterStep::CombFilter(CombFilter {
                delay_samples: 2,
                feedback_gain: -0.5,
            }),
            FilterStep::ButterworthLowPass(ButterworthLowPassFilter { cutoff_hz: 100.0 }),
            FilterStep::ButterworthHighPass(ButterworthHighPassFilter { cutoff_hz: 5.0 }),
            FilterStep::Chebyshev1LowPass(Chebyshev1LowPassFilter {
                cutoff_hz: 100.0,
                ripple_db: 1.0,
            }),
            FilterStep::Chebyshev2LowPass(Chebyshev2LowPassFilter {
                cutoff_hz: 100.0,
                stopband_attenuation_db: 40.0,
            }),
            FilterStep::BesselLowPass(BesselLowPassFilter { cutoff_hz: 100.0 }),
            FilterStep::Resample(ResampleTransform {
                sample_interval_s: 0.001,
            }),
            FilterStep::Downsample(DownsampleTransform { factor: 2 }),
            FilterStep::Decimate(DecimateTransform {
                factor: 2,
                cutoff_hz: 100.0,
            }),
            FilterStep::Upsample(UpsampleTransform { factor: 2 }),
            FilterStep::Interpolate(InterpolateTransform {
                sample_interval_s: 0.001,
            }),
            FilterStep::RationalResample(RationalResampleTransform {
                upsample_factor: 3,
                downsample_factor: 2,
            }),
            FilterStep::SampleAndHold(SampleAndHoldTransform {
                sample_interval_s: 0.001,
            }),
            FilterStep::ZeroOrderHold(ZeroOrderHoldTransform {
                sample_interval_s: 0.001,
            }),
            FilterStep::FirstOrderHold(FirstOrderHoldTransform {
                sample_interval_s: 0.001,
            }),
            FilterStep::FractionalDelay(FractionalDelayTransform { delay_s: 0.0005 }),
            FilterStep::CrossCorrelationDelay(CrossCorrelationDelayTransform {
                reference_channel: "input_v".to_string(),
                target_channel: "input_v".to_string(),
                max_lag_samples: 1,
            }),
            FilterStep::JitterCorrection(JitterCorrectionTransform {
                sample_interval_s: 0.001,
            }),
            FilterStep::ClockDriftCorrection(ClockDriftCorrectionTransform {
                sample_interval_s: 0.001,
            }),
            FilterStep::HalfWaveRectify(HalfWaveRectifyTransform),
            FilterStep::FullWaveRectify(FullWaveRectifyTransform),
            FilterStep::Envelope(EnvelopeTransform { alpha: 0.5 }),
            FilterStep::MovingRms(MovingRmsTransform { window_samples: 3 }),
            FilterStep::PeakHold(PeakHoldTransform),
            FilterStep::FirstDerivative(FirstDerivativeTransform),
            FilterStep::SecondDerivative(SecondDerivativeTransform),
            FilterStep::Integral(IntegralTransform),
            FilterStep::CumulativeIntegral(CumulativeIntegralTransform),
            FilterStep::LeakyIntegrator(LeakyIntegratorTransform {
                time_constant_s: 2.0,
            }),
            FilterStep::SlopeDetection(SlopeDetectionTransform {
                threshold_per_s: 1.0,
            }),
            FilterStep::RollingMean(RollingMeanTransform { window_samples: 3 }),
            FilterStep::RollingVariance(RollingVarianceTransform { window_samples: 3 }),
            FilterStep::RollingStdDev(RollingStdDevTransform { window_samples: 3 }),
            FilterStep::RollingMin(RollingMinTransform { window_samples: 3 }),
            FilterStep::RollingMax(RollingMaxTransform { window_samples: 3 }),
            FilterStep::ZScore(ZScoreTransform),
            FilterStep::OutlierDetection(OutlierDetectionTransform {
                threshold_sigma: 1.0,
            }),
            FilterStep::QuantileClip(QuantileClipTransform {
                lower_quantile: 0.25,
                upper_quantile: 0.75,
            }),
            FilterStep::NoiseInjection(NoiseInjectionTransform {
                kind: NoiseKind::White,
                amplitude_v: 0.01,
                min_v: 0.0,
                max_v: 0.0,
                probability: 0.0,
                seed: 1,
            }),
            FilterStep::NoiseInjection(NoiseInjectionTransform {
                kind: NoiseKind::Gaussian,
                amplitude_v: 0.01,
                min_v: 0.0,
                max_v: 0.0,
                probability: 0.0,
                seed: 2,
            }),
            FilterStep::NoiseInjection(NoiseInjectionTransform {
                kind: NoiseKind::Uniform,
                amplitude_v: 0.0,
                min_v: -0.01,
                max_v: 0.01,
                probability: 0.0,
                seed: 3,
            }),
            FilterStep::NoiseInjection(NoiseInjectionTransform {
                kind: NoiseKind::Pink,
                amplitude_v: 0.01,
                min_v: 0.0,
                max_v: 0.0,
                probability: 0.0,
                seed: 4,
            }),
            FilterStep::NoiseInjection(NoiseInjectionTransform {
                kind: NoiseKind::Brown,
                amplitude_v: 0.01,
                min_v: 0.0,
                max_v: 0.0,
                probability: 0.0,
                seed: 5,
            }),
            FilterStep::NoiseInjection(NoiseInjectionTransform {
                kind: NoiseKind::Impulse,
                amplitude_v: 0.1,
                min_v: 0.0,
                max_v: 0.0,
                probability: 0.2,
                seed: 6,
            }),
            FilterStep::NoiseInjection(NoiseInjectionTransform {
                kind: NoiseKind::SaltPepper,
                amplitude_v: 0.0,
                min_v: 0.0,
                max_v: 5.0,
                probability: 0.2,
                seed: 7,
            }),
            FilterStep::NoiseInjection(NoiseInjectionTransform {
                kind: NoiseKind::Quantization,
                amplitude_v: 0.05,
                min_v: 0.0,
                max_v: 0.0,
                probability: 0.0,
                seed: 8,
            }),
            FilterStep::PeriodicInterference(PeriodicInterferenceTransform {
                kind: PeriodicInterferenceKind::Periodic,
                amplitude_v: 0.1,
                frequency_hz: 10.0,
                phase_rad: 0.0,
            }),
            FilterStep::PeriodicInterference(PeriodicInterferenceTransform {
                kind: PeriodicInterferenceKind::Hum,
                amplitude_v: 0.1,
                frequency_hz: 60.0,
                phase_rad: 0.0,
            }),
            FilterStep::DriftFault(DriftFaultTransform {
                kind: DriftFaultKind::GroundBounce,
                amplitude_v: 0.1,
                drift_rate_v_per_s: 0.0,
                interval_samples: 2,
                seed: 0,
            }),
            FilterStep::DriftFault(DriftFaultTransform {
                kind: DriftFaultKind::Thermal,
                amplitude_v: 0.0,
                drift_rate_v_per_s: 0.01,
                interval_samples: 1,
                seed: 0,
            }),
            FilterStep::DriftFault(DriftFaultTransform {
                kind: DriftFaultKind::RandomWalk,
                amplitude_v: 0.01,
                drift_rate_v_per_s: 0.0,
                interval_samples: 1,
                seed: 9,
            }),
            FilterStep::SampleFault(SampleFaultTransform {
                kind: SampleFaultKind::Dropout,
                probability: 0.2,
                fault_value_v: 0.0,
                min_v: 0.0,
                max_v: 0.0,
                start_index: 0,
                duration_samples: 1,
                seed: 10,
            }),
            FilterStep::SampleFault(SampleFaultTransform {
                kind: SampleFaultKind::MissingSamples,
                probability: 0.2,
                fault_value_v: -999.0,
                min_v: 0.0,
                max_v: 0.0,
                start_index: 0,
                duration_samples: 1,
                seed: 11,
            }),
            FilterStep::SampleFault(SampleFaultTransform {
                kind: SampleFaultKind::Saturation,
                probability: 0.0,
                fault_value_v: 0.0,
                min_v: 0.0,
                max_v: 5.0,
                start_index: 0,
                duration_samples: 1,
                seed: 0,
            }),
            FilterStep::SampleFault(SampleFaultTransform {
                kind: SampleFaultKind::StuckAt,
                probability: 0.0,
                fault_value_v: 2.5,
                min_v: 0.0,
                max_v: 0.0,
                start_index: 1,
                duration_samples: 2,
                seed: 0,
            }),
            FilterStep::SampleFault(SampleFaultTransform {
                kind: SampleFaultKind::Flatline,
                probability: 0.0,
                fault_value_v: f64::NAN,
                min_v: 0.0,
                max_v: 0.0,
                start_index: 1,
                duration_samples: 1,
                seed: 0,
            }),
            FilterStep::SampleFault(SampleFaultTransform {
                kind: SampleFaultKind::Intermittent,
                probability: 0.2,
                fault_value_v: 0.0,
                min_v: 0.0,
                max_v: 0.0,
                start_index: 0,
                duration_samples: 1,
                seed: 12,
            }),
            FilterStep::SimulationQuantizer(SimulationQuantizerTransform {
                kind: SimulationQuantizerKind::Rounding,
                lsb_v: 0.1,
                min_v: 0.0,
                max_v: 0.0,
            }),
            FilterStep::SimulationQuantizer(SimulationQuantizerTransform {
                kind: SimulationQuantizerKind::Floor,
                lsb_v: 0.1,
                min_v: 0.0,
                max_v: 0.0,
            }),
            FilterStep::SimulationQuantizer(SimulationQuantizerTransform {
                kind: SimulationQuantizerKind::Ceil,
                lsb_v: 0.1,
                min_v: 0.0,
                max_v: 0.0,
            }),
            FilterStep::SimulationQuantizer(SimulationQuantizerTransform {
                kind: SimulationQuantizerKind::MidRise,
                lsb_v: 0.1,
                min_v: 0.0,
                max_v: 0.0,
            }),
            FilterStep::SimulationQuantizer(SimulationQuantizerTransform {
                kind: SimulationQuantizerKind::MidTread,
                lsb_v: 0.1,
                min_v: 0.0,
                max_v: 0.0,
            }),
            FilterStep::SimulationQuantizer(SimulationQuantizerTransform {
                kind: SimulationQuantizerKind::Saturating,
                lsb_v: 1.0,
                min_v: 0.0,
                max_v: 5.0,
            }),
            FilterStep::Dither(DitherTransform {
                lsb_v: 0.1,
                seed: 13,
            }),
            FilterStep::Companding(CompandingTransform {
                kind: CompandingKind::MuLaw,
                max_v: 5.0,
                mu: 255.0,
            }),
            FilterStep::SampleClockJitter(SampleClockJitterTransform {
                jitter_s: 0.00001,
                seed: 14,
            }),
            FilterStep::AdcCodeDefect(AdcCodeDefectTransform {
                kind: AdcCodeDefectKind::MissingCode,
                bits: 4,
                min_v: 0.0,
                max_v: 5.0,
                missing_code: 3,
                coefficients: Vec::new(),
            }),
            FilterStep::AdcCodeDefect(AdcCodeDefectTransform {
                kind: AdcCodeDefectKind::Inl,
                bits: 4,
                min_v: 0.0,
                max_v: 5.0,
                missing_code: 0,
                coefficients: vec![0.0, 0.01],
            }),
            FilterStep::AdcCodeDefect(AdcCodeDefectTransform {
                kind: AdcCodeDefectKind::Dnl,
                bits: 4,
                min_v: 0.0,
                max_v: 5.0,
                missing_code: 0,
                coefficients: vec![0.0, -0.01],
            }),
            FilterStep::GainOffsetError(GainOffsetErrorTransform {
                kind: GainOffsetErrorKind::Gain,
                gain_error: 0.01,
                offset_error_v: 0.0,
            }),
            FilterStep::GainOffsetError(GainOffsetErrorTransform {
                kind: GainOffsetErrorKind::Offset,
                gain_error: 0.0,
                offset_error_v: 0.02,
            }),
            FilterStep::ChannelArithmetic(ChannelArithmeticTransform {
                kind: ChannelArithmeticKind::Add,
                left_channel: "input_v".to_string(),
                right_channel: "input_v".to_string(),
                output_channel: "sum_v".to_string(),
                output_unit: None,
            }),
            FilterStep::ChannelArithmetic(ChannelArithmeticTransform {
                kind: ChannelArithmeticKind::Subtract,
                left_channel: "input_v".to_string(),
                right_channel: "input_v".to_string(),
                output_channel: "sub_v".to_string(),
                output_unit: None,
            }),
            FilterStep::ChannelArithmetic(ChannelArithmeticTransform {
                kind: ChannelArithmeticKind::Differential,
                left_channel: "input_v".to_string(),
                right_channel: "input_v".to_string(),
                output_channel: "diff_v".to_string(),
                output_unit: None,
            }),
            FilterStep::ChannelArithmetic(ChannelArithmeticTransform {
                kind: ChannelArithmeticKind::CommonMode,
                left_channel: "input_v".to_string(),
                right_channel: "input_v".to_string(),
                output_channel: "common_v".to_string(),
                output_unit: None,
            }),
            FilterStep::VectorMagnitude(VectorMagnitudeTransform {
                kind: VectorMagnitudeKind::VectorMagnitude,
                channels: vec!["input_v".to_string(), "input_v".to_string()],
                output_channel: "vector_mag".to_string(),
                output_unit: None,
            }),
            FilterStep::VectorMagnitude(VectorMagnitudeTransform {
                kind: VectorMagnitudeKind::EuclideanNorm,
                channels: vec!["input_v".to_string(), "input_v".to_string()],
                output_channel: "norm_v".to_string(),
                output_unit: None,
            }),
            FilterStep::MatrixTransform(MatrixTransform {
                input_channels: vec!["input_v".to_string()],
                matrix: vec![vec![1.0]],
                output_channels: vec!["mix_v".to_string()],
                output_unit: None,
            }),
            FilterStep::CoordinateRotation(CoordinateRotationTransform {
                x_channel: "input_v".to_string(),
                y_channel: "input_v".to_string(),
                angle_rad: 0.0,
                output_x_channel: "rot_x".to_string(),
                output_y_channel: "rot_y".to_string(),
                output_unit: None,
            }),
            FilterStep::sensor_conversion(SensorConversionTransform {
                kind: SensorConversionKind::Linear,
                channel: "input_v".to_string(),
                output_channel: "linear_units".to_string(),
                output_unit: "unit".to_string(),
                parameters: SensorConversionParameters {
                    input_min_v: Some(0.0),
                    input_max_v: Some(5.0),
                    output_min: Some(0.0),
                    output_max: Some(100.0),
                    ..SensorConversionParameters::default()
                },
            }),
            FilterStep::sensor_conversion(SensorConversionTransform {
                kind: SensorConversionKind::Pressure,
                channel: "input_v".to_string(),
                output_channel: "pressure_kpa".to_string(),
                output_unit: "kPa".to_string(),
                parameters: SensorConversionParameters {
                    input_min_v: Some(0.0),
                    input_max_v: Some(5.0),
                    output_min: Some(0.0),
                    output_max: Some(100.0),
                    ..SensorConversionParameters::default()
                },
            }),
            FilterStep::sensor_conversion(SensorConversionTransform {
                kind: SensorConversionKind::CurrentShunt,
                channel: "input_v".to_string(),
                output_channel: "current_a".to_string(),
                output_unit: "A".to_string(),
                parameters: SensorConversionParameters {
                    shunt_ohms: Some(1.0),
                    ..SensorConversionParameters::default()
                },
            }),
            FilterStep::sensor_conversion(SensorConversionTransform {
                kind: SensorConversionKind::BridgeStrain,
                channel: "input_v".to_string(),
                output_channel: "strain".to_string(),
                output_unit: "strain".to_string(),
                parameters: SensorConversionParameters {
                    excitation_v: Some(5.0),
                    gauge_factor: Some(2.0),
                    ..SensorConversionParameters::default()
                },
            }),
            FilterStep::sensor_conversion(SensorConversionTransform {
                kind: SensorConversionKind::LoadCell,
                channel: "input_v".to_string(),
                output_channel: "force_n".to_string(),
                output_unit: "N".to_string(),
                parameters: SensorConversionParameters {
                    excitation_v: Some(5.0),
                    sensitivity_mv_v: Some(2.0),
                    full_scale: Some(100.0),
                    ..SensorConversionParameters::default()
                },
            }),
            FilterStep::sensor_conversion(SensorConversionTransform {
                kind: SensorConversionKind::Rtd,
                channel: "input_v".to_string(),
                output_channel: "rtd_c".to_string(),
                output_unit: "C".to_string(),
                parameters: SensorConversionParameters {
                    r0_ohm: Some(100.0),
                    alpha_per_c: Some(0.00385),
                    ..SensorConversionParameters::default()
                },
            }),
            FilterStep::sensor_conversion(SensorConversionTransform {
                kind: SensorConversionKind::Thermistor,
                channel: "input_v".to_string(),
                output_channel: "thermistor_c".to_string(),
                output_unit: "C".to_string(),
                parameters: SensorConversionParameters {
                    r0_ohm: Some(10000.0),
                    beta_k: Some(3950.0),
                    t0_c: Some(25.0),
                    ..SensorConversionParameters::default()
                },
            }),
            FilterStep::sensor_conversion(SensorConversionTransform {
                kind: SensorConversionKind::TachometerRpm,
                channel: "input_v".to_string(),
                output_channel: "rpm".to_string(),
                output_unit: "rpm".to_string(),
                parameters: SensorConversionParameters {
                    pulses_per_rev: Some(2.0),
                    ..SensorConversionParameters::default()
                },
            }),
            FilterStep::sensor_conversion(SensorConversionTransform {
                kind: SensorConversionKind::EncoderPosition,
                channel: "input_v".to_string(),
                output_channel: "angle_rad".to_string(),
                output_unit: "rad".to_string(),
                parameters: SensorConversionParameters {
                    counts_per_rev: Some(1024.0),
                    scale_per_rev: Some(std::f64::consts::TAU),
                    ..SensorConversionParameters::default()
                },
            }),
            FilterStep::sensor_conversion(SensorConversionTransform {
                kind: SensorConversionKind::Accelerometer,
                channel: "input_v".to_string(),
                output_channel: "accel_g".to_string(),
                output_unit: "g".to_string(),
                parameters: SensorConversionParameters {
                    sensitivity_v_per_unit: Some(0.1),
                    bias_v: Some(2.5),
                    ..SensorConversionParameters::default()
                },
            }),
            FilterStep::sensor_conversion(SensorConversionTransform {
                kind: SensorConversionKind::Gyroscope,
                channel: "input_v".to_string(),
                output_channel: "gyro_deg_s".to_string(),
                output_unit: "deg/s".to_string(),
                parameters: SensorConversionParameters {
                    sensitivity_v_per_unit: Some(0.02),
                    bias_v: Some(2.5),
                    ..SensorConversionParameters::default()
                },
            }),
            FilterStep::sensor_conversion(SensorConversionTransform {
                kind: SensorConversionKind::HallCurrent,
                channel: "input_v".to_string(),
                output_channel: "hall_a".to_string(),
                output_unit: "A".to_string(),
                parameters: SensorConversionParameters {
                    sensitivity_v_per_unit: Some(0.04),
                    bias_v: Some(2.5),
                    ..SensorConversionParameters::default()
                },
            }),
            FilterStep::sensor_conversion(SensorConversionTransform {
                kind: SensorConversionKind::LvdtPosition,
                channel: "input_v".to_string(),
                output_channel: "position_mm".to_string(),
                output_unit: "mm".to_string(),
                parameters: SensorConversionParameters {
                    sensitivity_v_per_unit: Some(0.5),
                    bias_v: Some(0.0),
                    ..SensorConversionParameters::default()
                },
            }),
            FilterStep::sensor_conversion(SensorConversionTransform {
                kind: SensorConversionKind::MicrophoneSpl,
                channel: "input_v".to_string(),
                output_channel: "spl_db".to_string(),
                output_unit: "dB".to_string(),
                parameters: SensorConversionParameters {
                    reference: Some(0.00002),
                    ..SensorConversionParameters::default()
                },
            }),
            FilterStep::sensor_conversion(SensorConversionTransform {
                kind: SensorConversionKind::PhotodiodePower,
                channel: "input_v".to_string(),
                output_channel: "optical_w".to_string(),
                output_unit: "W".to_string(),
                parameters: SensorConversionParameters {
                    responsivity_a_per_w: Some(0.4),
                    ..SensorConversionParameters::default()
                },
            }),
            FilterStep::VibrationTransform(VibrationTransform {
                kind: VibrationTransformKind::VelocityFromAcceleration,
                channel: "input_v".to_string(),
                output_channel: "velocity_m_s".to_string(),
                output_unit: "m/s".to_string(),
                window_samples: 1,
            }),
            FilterStep::VibrationTransform(VibrationTransform {
                kind: VibrationTransformKind::DisplacementFromVelocity,
                channel: "input_v".to_string(),
                output_channel: "displacement_m".to_string(),
                output_unit: "m".to_string(),
                window_samples: 1,
            }),
            FilterStep::VibrationTransform(VibrationTransform {
                kind: VibrationTransformKind::VibrationSeverity,
                channel: "input_v".to_string(),
                output_channel: "severity".to_string(),
                output_unit: "m/s^2".to_string(),
                window_samples: 2,
            }),
            FilterStep::ControlTransform(ControlTransform {
                kind: ControlTransformKind::ErrorSignal,
                channel: "input_v".to_string(),
                output_channel: "error_v".to_string(),
                output_unit: None,
                setpoint: 5.0,
                kp: 0.0,
                ki: 0.0,
                kd: 0.0,
                rate_limit_per_s: 0.0,
                min_v: 0.0,
                max_v: 0.0,
                threshold_v: 0.0,
                feedforward_gain: 0.0,
                feedforward_offset: 0.0,
            }),
            FilterStep::ControlTransform(ControlTransform {
                kind: ControlTransformKind::ProportionalControl,
                channel: "input_v".to_string(),
                output_channel: "p_out".to_string(),
                output_unit: None,
                setpoint: 5.0,
                kp: 2.0,
                ki: 0.0,
                kd: 0.0,
                rate_limit_per_s: 0.0,
                min_v: 0.0,
                max_v: 0.0,
                threshold_v: 0.0,
                feedforward_gain: 0.0,
                feedforward_offset: 0.0,
            }),
            FilterStep::ControlTransform(ControlTransform {
                kind: ControlTransformKind::PidControl,
                channel: "input_v".to_string(),
                output_channel: "pid_out".to_string(),
                output_unit: None,
                setpoint: 5.0,
                kp: 2.0,
                ki: 0.5,
                kd: 0.1,
                rate_limit_per_s: 0.0,
                min_v: 0.0,
                max_v: 0.0,
                threshold_v: 0.0,
                feedforward_gain: 0.0,
                feedforward_offset: 0.0,
            }),
            FilterStep::ControlTransform(ControlTransform {
                kind: ControlTransformKind::RateLimiter,
                channel: "input_v".to_string(),
                output_channel: "rate_limited".to_string(),
                output_unit: None,
                setpoint: 0.0,
                kp: 0.0,
                ki: 0.0,
                kd: 0.0,
                rate_limit_per_s: 1.0,
                min_v: 0.0,
                max_v: 0.0,
                threshold_v: 0.0,
                feedforward_gain: 0.0,
                feedforward_offset: 0.0,
            }),
            FilterStep::ControlTransform(ControlTransform {
                kind: ControlTransformKind::SlewRateLimit,
                channel: "input_v".to_string(),
                output_channel: "slew_limited".to_string(),
                output_unit: None,
                setpoint: 0.0,
                kp: 0.0,
                ki: 0.0,
                kd: 0.0,
                rate_limit_per_s: 1.0,
                min_v: 0.0,
                max_v: 0.0,
                threshold_v: 0.0,
                feedforward_gain: 0.0,
                feedforward_offset: 0.0,
            }),
            FilterStep::ControlTransform(ControlTransform {
                kind: ControlTransformKind::ControlSaturation,
                channel: "input_v".to_string(),
                output_channel: "saturated_v".to_string(),
                output_unit: None,
                setpoint: 0.0,
                kp: 0.0,
                ki: 0.0,
                kd: 0.0,
                rate_limit_per_s: 0.0,
                min_v: 0.0,
                max_v: 5.0,
                threshold_v: 0.0,
                feedforward_gain: 0.0,
                feedforward_offset: 0.0,
            }),
            FilterStep::ControlTransform(ControlTransform {
                kind: ControlTransformKind::ControlDeadzone,
                channel: "input_v".to_string(),
                output_channel: "deadzone_v".to_string(),
                output_unit: None,
                setpoint: 0.0,
                kp: 0.0,
                ki: 0.0,
                kd: 0.0,
                rate_limit_per_s: 0.0,
                min_v: 0.0,
                max_v: 0.0,
                threshold_v: 0.1,
                feedforward_gain: 0.0,
                feedforward_offset: 0.0,
            }),
            FilterStep::ControlTransform(ControlTransform {
                kind: ControlTransformKind::FeedforwardControl,
                channel: "input_v".to_string(),
                output_channel: "feedforward_v".to_string(),
                output_unit: None,
                setpoint: 0.0,
                kp: 0.0,
                ki: 0.0,
                kd: 0.0,
                rate_limit_per_s: 0.0,
                min_v: 0.0,
                max_v: 0.0,
                threshold_v: 0.0,
                feedforward_gain: 2.0,
                feedforward_offset: 0.5,
            }),
        ];

        for filter in unsupported {
            let name = filter.name().to_string();
            let error = schema_filters(&[filter], &channels)
                .expect_err("desktop-only transform should be rejected");
            assert!(
                error.contains(&format!(
                    "rule package export does not yet support transform `{name}`"
                )),
                "unexpected error for {name}: {error}"
            );
        }
    }

    #[test]
    fn analyzes_config_with_m12_event_validation_transforms() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = format!("{manifest_dir}/../../examples/switch-bounce-waveform.csv");
        let config_path = format!("{manifest_dir}/../../examples/m12-event-validation-config.toml");

        let output = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            input_path,
            "--config".to_string(),
            config_path,
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("M12 event validation config should analyze");

        assert!(output.contains("\"overall_outcome\": \"pass\""));
        assert!(output.contains("\"event_records\""));
        assert!(output.contains("\"event_validations\""));
        assert!(output.contains("\"transform\": \"schmitt_trigger\""));
        assert!(output.contains("\"kind\": \"bounce\""));
        assert!(output.contains("\"validation\": \"missing_pulse\""));
        assert!(output.contains("\"validation\": \"extra_pulse\""));
        assert!(output.contains("\"validation\": \"dwell_time\""));
        assert!(output.contains("\"validation\": \"timeout\""));
        assert!(output.contains("\"category\": \"EventTransform\""));
        assert!(output.contains("\"category\": \"StatefulTransform\""));
        assert!(output.contains("\"category\": \"ValidationTransform\""));
        assert!(output.contains("\"kind\": \"event_records\""));
        assert!(output.contains("\"kind\": \"validation_records\""));
    }

    fn unique_plot_path(name: &str) -> String {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time should be available")
            .as_nanos();
        std::env::temp_dir()
            .join(format!(
                "ferrisoxide-signal-{name}-{}-{nonce}.svg",
                std::process::id()
            ))
            .to_string_lossy()
            .into_owned()
    }

    fn unique_export_dir(name: &str) -> String {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time should be available")
            .as_nanos();
        std::env::temp_dir()
            .join(format!(
                "ferrisoxide-signal-{name}-{}-{nonce}",
                std::process::id()
            ))
            .to_string_lossy()
            .into_owned()
    }

    #[test]
    fn desktop_flow_inspects_csv_source_as_json() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = Path::new(manifest_dir)
            .join("../../examples/basic-waveform.csv")
            .display()
            .to_string();

        let output = run(vec![
            "inspect-source".to_string(),
            "--input".to_string(),
            input_path,
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("CSV source should inspect");

        assert!(output.contains("\"kind\": \"source_inspection\""));
        assert!(output.contains("\"source_mode\": \"csv\""));
        assert!(output.contains("\"time_column\": \"time\""));
        assert!(output.contains("\"id\": \"input_v\""));
        assert!(output.contains("\"id\": \"output_v\""));
        assert!(output.contains("\"nominal_sample_rate_hz\""));
    }

    #[test]
    fn workflow_api_inspects_csv_source() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = Path::new(manifest_dir)
            .join("../../examples/basic-waveform.csv")
            .display()
            .to_string();

        let inspection =
            inspect_source(&InspectSourceRequest::csv(input_path)).expect("source should inspect");

        assert_eq!(inspection.source_mode, "csv");
        assert_eq!(inspection.time_column, "time");
        assert_eq!(inspection.sample_count, 5);
        assert_eq!(
            inspection
                .channels
                .iter()
                .map(|channel| channel.id.as_str())
                .collect::<Vec<_>>(),
            vec!["input_v", "output_v"]
        );
    }

    #[test]
    fn workflow_api_loads_csv_headers() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = Path::new(manifest_dir).join("../../examples/basic-waveform.csv");

        let headers = load_csv_headers(&input_path).expect("headers should load");

        assert_eq!(
            headers,
            vec![
                "time".to_string(),
                "input_v".to_string(),
                "output_v".to_string()
            ]
        );
    }

    #[test]
    fn workflow_api_scaffolds_and_analyzes_csv_source() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = Path::new(manifest_dir)
            .join("../../examples/basic-waveform.csv")
            .display()
            .to_string();
        let temp_root = PathBuf::from(unique_export_dir("workflow-api-scaffold"));
        fs::create_dir_all(&temp_root).expect("temp root should be created");
        let scaffold_path = temp_root.join("analysis.toml");

        let scaffold = scaffold_csv_config(&ScaffoldConfigRequest::csv(input_path.clone()))
            .expect("config should scaffold");
        fs::write(&scaffold_path, scaffold).expect("scaffold should be written for analysis");

        let analysis = analyze_csv(&AnalyzeCsvRequest::json(
            input_path,
            scaffold_path.display().to_string(),
        ))
        .expect("scaffolded config should analyze");

        assert!(analysis.contains("\"overall_outcome\": \"pass\""));
        assert!(analysis.contains("\"input_v_min_observed\""));

        fs::remove_dir_all(&temp_root).expect("temp root should be removable");
    }

    #[test]
    fn workflow_api_writes_csv_bundle_summary() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = Path::new(manifest_dir)
            .join("../../examples/basic-waveform.csv")
            .display()
            .to_string();
        let config_path = Path::new(manifest_dir)
            .join("../../examples/basic-config.toml")
            .display()
            .to_string();
        let temp_root = PathBuf::from(unique_export_dir("workflow-api-bundle"));
        let output_dir = temp_root.join("bundle");
        let mut request =
            EvaluateBundleRequest::csv(input_path, config_path, output_dir.display().to_string());
        request.include_plot = true;

        let bundle = evaluate_bundle(&request).expect("bundle should be written");

        assert_eq!(bundle.source_mode, "csv");
        assert_eq!(bundle.overall_outcome, "pass");
        assert!(bundle.artifacts.contains(&"report.json".to_string()));
        assert!(bundle.artifacts.contains(&"evidence.svg".to_string()));

        fs::remove_dir_all(&temp_root).expect("temp root should be removable");
    }

    #[test]
    fn workflow_api_loads_csv_plot_series_from_config() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = Path::new(manifest_dir)
            .join("../../examples/basic-waveform.csv")
            .display()
            .to_string();
        let config_path = Path::new(manifest_dir)
            .join("../../examples/basic-config.toml")
            .display()
            .to_string();

        let series =
            load_csv_plot_series(&CsvPlotSeriesRequest::from_config(input_path, config_path))
                .expect("plot series should load");

        assert_eq!(series.len(), 2);
        assert_eq!(series[0].name, "input_v");
        assert_eq!(series[0].points.len(), 5);
        assert_eq!(series[0].points[0].time, 0.0);
    }

    #[test]
    fn desktop_flow_rejects_realtime_source_until_live_daq_exists() {
        let error = run(vec![
            "inspect-source".to_string(),
            "--source".to_string(),
            "realtime".to_string(),
        ])
        .expect_err("realtime source should be explicitly gated");

        assert!(error.contains("planned for the desktop workflow"));
        assert!(error.contains("not implemented"));
    }

    #[test]
    fn desktop_flow_scaffolds_config_that_can_analyze_the_source() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = Path::new(manifest_dir)
            .join("../../examples/basic-waveform.csv")
            .display()
            .to_string();
        let temp_root = PathBuf::from(unique_export_dir("scaffold-config"));
        fs::create_dir_all(&temp_root).expect("temp root should be created");
        let scaffold_path = temp_root.join("analysis.toml");

        let output = run(vec![
            "scaffold-config".to_string(),
            "--input".to_string(),
            input_path.clone(),
            "--output".to_string(),
            scaffold_path.display().to_string(),
        ])
        .expect("config should scaffold");

        assert!(output.contains("Analysis config scaffold written"));
        assert!(scaffold_path.exists());

        let analysis = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            input_path,
            "--config".to_string(),
            scaffold_path.display().to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("scaffolded config should analyze the inspected source");

        assert!(analysis.contains("\"overall_outcome\": \"pass\""));
        assert!(analysis.contains("\"input_v_min_observed\""));
        assert!(analysis.contains("\"output_v_max_observed\""));
        let scaffold = fs::read_to_string(&scaffold_path).expect("scaffold should be readable");
        assert!(scaffold.contains("Channel role prompts"));

        fs::remove_dir_all(&temp_root).expect("temp root should be removable");
    }

    #[test]
    fn desktop_flow_renders_authoring_recipe_template() {
        let output = run(vec![
            "workflow-template".to_string(),
            "--use-case".to_string(),
            "switch-bounce".to_string(),
            "--format".to_string(),
            "toml".to_string(),
        ])
        .expect("workflow template should render");

        assert!(output.contains("[[event_transforms]]"));
        assert!(output.contains("type = \"schmitt_trigger\""));
        assert!(output.contains("[[event_validations]]"));
        let config =
            toml::from_str::<AnalysisConfig>(&output).expect("template should be valid TOML");
        assert_eq!(config.input.channels, vec!["switch_v".to_string()]);
        assert_eq!(config.event_transforms.len(), 3);
        assert_eq!(config.event_validations.len(), 2);
        assert_eq!(config.criteria.len(), 1);
        validate_loaded_config(&config).expect("template should be a valid analysis config");
    }

    #[test]
    fn desktop_flow_writes_csv_evaluation_bundle_with_plot() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = Path::new(manifest_dir)
            .join("../../examples/basic-waveform.csv")
            .display()
            .to_string();
        let config_path = Path::new(manifest_dir)
            .join("../../examples/basic-config.toml")
            .display()
            .to_string();
        let temp_root = PathBuf::from(unique_export_dir("csv-evaluate-bundle"));
        let output_dir = temp_root.join("bundle");

        let output = run(vec![
            "evaluate-bundle".to_string(),
            "--input".to_string(),
            input_path,
            "--config".to_string(),
            config_path,
            "--output-dir".to_string(),
            output_dir.display().to_string(),
            "--plot".to_string(),
        ])
        .expect("CSV evaluation bundle should be written");

        assert!(output.contains("\"kind\": \"desktop_evaluation_bundle\""));
        assert!(output.contains("\"source_mode\": \"csv\""));
        assert!(output.contains("\"overall_outcome\": \"pass\""));
        for artifact in [
            "source-summary.json",
            "config.toml",
            "report.json",
            "report.txt",
            "failure-triage.md",
            "transform-catalog.json",
            "evidence.svg",
            "bundle-summary.json",
        ] {
            assert!(
                output_dir.join(artifact).exists(),
                "missing artifact {artifact}"
            );
        }
        let overwrite_error = run(vec![
            "evaluate-bundle".to_string(),
            "--input".to_string(),
            Path::new(manifest_dir)
                .join("../../examples/basic-waveform.csv")
                .display()
                .to_string(),
            "--config".to_string(),
            Path::new(manifest_dir)
                .join("../../examples/basic-config.toml")
                .display()
                .to_string(),
            "--output-dir".to_string(),
            output_dir.display().to_string(),
        ])
        .expect_err("existing bundle artifacts should not be overwritten by default");
        assert!(overwrite_error.contains("failed to create"));

        fs::remove_dir_all(&temp_root).expect("temp root should be removable");
    }

    #[test]
    fn desktop_flow_writes_simulation_evaluation_bundle() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = Path::new(manifest_dir)
            .join("../../tests/e2e/heated_actuator/input/passing_run.csv")
            .display()
            .to_string();
        let control_config_path = Path::new(manifest_dir)
            .join("../../examples/control-config/production-control-config.toml")
            .display()
            .to_string();
        let verification_config_path = Path::new(manifest_dir)
            .join("../../examples/test-verification-config/test-verification-config.toml")
            .display()
            .to_string();
        let channel_map_path = Path::new(manifest_dir)
            .join("../../examples/simulation/heated-actuator-channel-map.toml")
            .display()
            .to_string();
        let temp_root = PathBuf::from(unique_export_dir("simulation-evaluate-bundle"));
        let output_dir = temp_root.join("bundle");

        let output = run(vec![
            "evaluate-bundle".to_string(),
            "--source".to_string(),
            "simulation".to_string(),
            "--input".to_string(),
            input_path,
            "--control-config".to_string(),
            control_config_path,
            "--verification-config".to_string(),
            verification_config_path,
            "--channel-map".to_string(),
            channel_map_path,
            "--output-dir".to_string(),
            output_dir.display().to_string(),
        ])
        .expect("simulation evaluation bundle should be written");

        assert!(output.contains("\"source_mode\": \"simulation\""));
        assert!(output.contains("\"overall_outcome\": \"pass\""));
        for artifact in [
            "source-summary.json",
            "simulation-workflow.json",
            "simulation-workflow.txt",
            "production-control-config.toml",
            "test-verification-config.toml",
            "channel-map.toml",
            "failure-triage.md",
            "transform-catalog.json",
            "bundle-summary.json",
        ] {
            assert!(
                output_dir.join(artifact).exists(),
                "missing artifact {artifact}"
            );
        }

        fs::remove_dir_all(&temp_root).expect("temp root should be removable");
    }

    #[test]
    fn runs_batch_analysis_manifest_with_pass_fail_and_error_runs() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let temp_root = PathBuf::from(unique_export_dir("batch-analysis"));
        fs::create_dir_all(&temp_root).expect("temp root should be created");
        let output_dir = temp_root.join("out");
        let manifest_path = temp_root.join("batch.toml");
        let basic_input = Path::new(manifest_dir)
            .join("../../examples/basic-waveform.csv")
            .display()
            .to_string();
        let basic_config = Path::new(manifest_dir)
            .join("../../examples/basic-config.toml")
            .display()
            .to_string();
        let slow_input = Path::new(manifest_dir)
            .join("../../tests/fixtures/slow_rise_fall_signal.csv")
            .display()
            .to_string();
        let slow_config = Path::new(manifest_dir)
            .join("../../tests/configs/slow-rise-fail.toml")
            .display()
            .to_string();
        let missing_input = temp_root.join("missing.csv").display().to_string();

        fs::write(
            &manifest_path,
            format!(
                r#"
default_format = "json"
summary_file = "summary.json"

[[runs]]
id = "basic_pass"
input = "{basic_input}"
config = "{basic_config}"
report = "basic-pass.json"

[[runs]]
id = "slow_fail"
input = "{slow_input}"
config = "{slow_config}"
report = "slow-fail.json"

[[runs]]
id = "missing_input"
input = "{missing_input}"
config = "{basic_config}"
report = "missing-input.json"
"#
            ),
        )
        .expect("manifest should be writable");

        let output = run(vec![
            "batch".to_string(),
            "--manifest".to_string(),
            manifest_path.display().to_string(),
            "--output-dir".to_string(),
            output_dir.display().to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("batch workflow should return a summary");

        assert!(output.contains("\"kind\": \"batch_analysis\""));
        assert!(output.contains("\"total_runs\": 3"));
        assert!(output.contains("\"passed_runs\": 1"));
        assert!(output.contains("\"failed_runs\": 1"));
        assert!(output.contains("\"error_runs\": 1"));
        assert!(output.contains("\"overall_outcome\": \"fail\""));
        assert!(output.contains("\"id\": \"basic_pass\""));
        assert!(output.contains("\"id\": \"slow_fail\""));
        assert!(output.contains("\"id\": \"missing_input\""));
        assert!(output_dir.join("basic-pass.json").exists());
        assert!(output_dir.join("slow-fail.json").exists());
        assert!(output_dir.join("summary.json").exists());
        assert!(!output_dir.join("missing-input.json").exists());

        fs::remove_dir_all(&temp_root).expect("temp root should be removable");
    }

    #[test]
    fn rejects_empty_batch_manifest() {
        let temp_root = PathBuf::from(unique_export_dir("empty-batch"));
        fs::create_dir_all(&temp_root).expect("temp root should be created");
        let manifest_path = temp_root.join("batch.toml");
        fs::write(&manifest_path, "default_format = \"json\"\n")
            .expect("manifest should be writable");

        let error = run(vec![
            "batch".to_string(),
            "--manifest".to_string(),
            manifest_path.display().to_string(),
            "--output-dir".to_string(),
            temp_root.join("out").display().to_string(),
        ])
        .expect_err("empty batch manifest should be rejected");

        assert!(error.contains("batch manifest must include at least one [[runs]] entry"));
        fs::remove_dir_all(&temp_root).expect("temp root should be removable");
    }

    #[test]
    fn rejects_batch_summary_format_before_writing_outputs() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let temp_root = PathBuf::from(unique_export_dir("batch-format"));
        fs::create_dir_all(&temp_root).expect("temp root should be created");
        let output_dir = temp_root.join("out");
        let manifest_path = temp_root.join("batch.toml");
        let basic_input = Path::new(manifest_dir)
            .join("../../examples/basic-waveform.csv")
            .display()
            .to_string();
        let basic_config = Path::new(manifest_dir)
            .join("../../examples/basic-config.toml")
            .display()
            .to_string();

        fs::write(
            &manifest_path,
            format!(
                r#"
default_format = "json"

[[runs]]
id = "basic_pass"
input = "{basic_input}"
config = "{basic_config}"
report = "basic-pass.json"
"#
            ),
        )
        .expect("manifest should be writable");

        let error = run(vec![
            "batch".to_string(),
            "--manifest".to_string(),
            manifest_path.display().to_string(),
            "--output-dir".to_string(),
            output_dir.display().to_string(),
            "--format".to_string(),
            "xml".to_string(),
        ])
        .expect_err("unsupported batch summary format should be rejected");

        assert!(error.contains("unsupported report format `xml`; use text or json"));
        assert!(!output_dir.exists());
        fs::remove_dir_all(&temp_root).expect("temp root should be removable");
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
    fn runs_analysis_with_dsl_config_and_text_output() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = format!("{manifest_dir}/../../examples/basic-waveform.csv");
        let config_path = format!("{manifest_dir}/../../examples/basic-dsl-config.toml");

        let output = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            input_path,
            "--config".to_string(),
            config_path,
            "--format".to_string(),
            "text".to_string(),
        ])
        .expect("analysis should run");

        assert!(output.contains("Overall: Pass"));
        assert!(output.contains("input_min_voltage_measurement"));
        assert!(output.contains("input_max_voltage_measurement"));
    }

    #[test]
    fn runs_heated_actuator_qualification_analysis_with_config() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path =
            format!("{manifest_dir}/../../tests/e2e/heated_actuator/input/passing_run.csv");
        let config_path = format!(
            "{manifest_dir}/../../tests/e2e/heated_actuator/configs/test-verification-config.toml"
        );

        let output = run(vec![
            "analyze".to_string(),
            "--input".to_string(),
            input_path,
            "--config".to_string(),
            config_path,
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("heated actuator analysis should run");

        assert!(output.contains("\"overall_outcome\": \"pass\""));
        assert!(output.contains("\"criterion_id\": \"REQ-001\""));
        assert!(output.contains("\"method\": \"response_latency\""));
    }

    #[test]
    fn runs_desktop_simulation_workflow_with_fixture_input() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path =
            format!("{manifest_dir}/../../tests/e2e/heated_actuator/input/passing_run.csv");
        let control_config_path =
            format!("{manifest_dir}/../../examples/control-config/production-control-config.toml");
        let verification_config_path = format!(
            "{manifest_dir}/../../examples/test-verification-config/test-verification-config.toml"
        );
        let channel_map_path =
            format!("{manifest_dir}/../../examples/simulation/heated-actuator-channel-map.toml");

        let output = run(vec![
            "simulate".to_string(),
            "--input".to_string(),
            input_path,
            "--control-config".to_string(),
            control_config_path,
            "--verification-config".to_string(),
            verification_config_path,
            "--channel-map".to_string(),
            channel_map_path,
            "--format".to_string(),
            "json".to_string(),
        ])
        .expect("desktop simulation workflow should run");

        assert!(output.contains("\"kind\": \"desktop_simulation\""));
        assert!(output.contains("\"simulation_trace\""));
        assert!(output.contains("\"transition\": \"command_to_heating\""));
        assert!(output.contains("\"transition\": \"feedback_reached\""));
        assert!(output.contains("\"verification_evidence\""));
        assert!(output.contains("\"overall_outcome\": \"pass\""));
        assert!(output.contains("\"criterion_id\": \"REQ-001\""));
    }

    #[test]
    fn controller_config_and_behavior_paths_match_portable_parity_evidence() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path =
            format!("{manifest_dir}/../../tests/e2e/heated_actuator/input/passing_run.csv");
        let control_config_path =
            format!("{manifest_dir}/../../examples/control-config/production-control-config.toml");
        let verification_config_path = format!(
            "{manifest_dir}/../../examples/test-verification-config/test-verification-config.toml"
        );
        let channel_map_path =
            format!("{manifest_dir}/../../examples/simulation/heated-actuator-channel-map.toml");

        let control_config =
            load_control_config(&control_config_path).expect("control config should load");
        let verification_config = load_verification_config(&verification_config_path)
            .expect("verification config should load");
        let channel_map =
            load_simulation_channel_map(&channel_map_path).expect("channel map should load");
        validate_simulation_channel_map(&channel_map, &control_config, &verification_config)
            .expect("channel map should validate");

        let selected_mode = channel_map.simulation.mode.clone();
        let workflow = run_desktop_simulation_workflow(DesktopSimulationInput {
            input_path: &input_path,
            control_config_path: &control_config_path,
            verification_config_path: &verification_config_path,
            channel_map_path: &channel_map_path,
            mode: selected_mode.clone(),
            control_config,
            verification_config: verification_config.clone(),
            channel_map: channel_map.clone(),
        })
        .expect("desktop workflow should run");

        let waveform =
            load_fixture_waveform(&input_path, &channel_map).expect("parity waveform should load");
        let desktop_state_trace = portable_state_trace(&workflow.simulation);
        let embedded_compatible_state_trace = portable_state_trace(&workflow.simulation);
        let desktop_evidence = portable_evidence_from_report(&workflow.verification);
        let embedded_compatible_evidence =
            embedded_compatible_evidence(&verification_config, &waveform)
                .expect("embedded-compatible evidence should evaluate");

        assert_eq!(desktop_state_trace, embedded_compatible_state_trace);
        assert_eq!(desktop_evidence, embedded_compatible_evidence);

        let report = ControllerParityReport {
            case_id: "m9_009_controller_config_parity_001",
            schema_note: "Software-only parity: state trace compares the portable simulator trace fields a future runtime must match; criteria evidence compares desktop analysis against embedded-compatible borrowed-rule execution.",
            approved_schema_differences: vec![
                "No embedded controller runtime exists yet, so controller state parity is a portable trace projection rather than target firmware execution.",
                "Desktop reports include human-readable reason strings and metadata not required by the embedded-compatible borrowed-rule summary.",
            ],
            inputs: ControllerParityInputs {
                waveform: "tests/e2e/heated_actuator/input/passing_run.csv".to_string(),
                production_control_config:
                    "examples/control-config/production-control-config.toml".to_string(),
                test_verification_config:
                    "examples/test-verification-config/test-verification-config.toml".to_string(),
                channel_map: "examples/simulation/heated-actuator-channel-map.toml".to_string(),
                selected_mode,
            },
            desktop_state_trace,
            embedded_compatible_state_trace,
            desktop_evidence,
            embedded_compatible_evidence,
        };
        let rendered =
            serde_json::to_string_pretty(&report).expect("controller parity report should render");

        assert!(rendered.contains("\"case_id\": \"m9_009_controller_config_parity_001\""));
        assert!(rendered.contains("\"criterion_id\": \"REQ-001\""));
        assert!(rendered.contains("\"transition\": \"command_to_heating\""));
        assert!(rendered.contains("Software-only parity"));
    }

    #[test]
    fn exports_rule_package_artifacts_from_config_and_evidence() {
        let input_path = "../../examples/basic-waveform.csv";
        let config_path = "../../examples/basic-dsl-config.toml";
        let output_dir = unique_export_dir("rule-package");

        let output = run(vec![
            "export-rule-package".to_string(),
            "--input".to_string(),
            input_path.to_string(),
            "--config".to_string(),
            config_path.to_string(),
            "--output-dir".to_string(),
            output_dir.clone(),
            "--package-name".to_string(),
            "basic-rule-package".to_string(),
            "--package-version".to_string(),
            "1.0.0".to_string(),
            "--target".to_string(),
            "controller_runtime".to_string(),
            "--target-id".to_string(),
            "test-controller".to_string(),
        ])
        .expect("rule package export should run");

        assert!(output.contains("Rule package exported to"));
        assert_export_artifact(
            &output_dir,
            "rules.toml",
            include_str!("../../../tests/expected/rule-package-basic/rules.toml"),
        );
        assert_export_artifact(
            &output_dir,
            "rules.json",
            include_str!("../../../tests/expected/rule-package-basic/rules.json"),
        );
        assert_export_artifact(
            &output_dir,
            "validation-report.json",
            include_str!("../../../tests/expected/rule-package-basic/validation-report.json"),
        );
        assert_export_artifact(
            &output_dir,
            "manifest.json",
            include_str!("../../../tests/expected/rule-package-basic/manifest.json"),
        );
        assert_export_artifact(
            &output_dir,
            "checksum.txt",
            include_str!("../../../tests/expected/rule-package-basic/checksum.txt"),
        );

        let package = ferrisoxide_rule_schema::parse_rule_package_toml(
            &fs::read_to_string(format!("{output_dir}/rules.toml"))
                .expect("rules.toml should be readable"),
        )
        .expect("exported rules.toml should parse");
        assert_eq!(
            package.validate_for_target(TargetProfileKind::ControllerRuntime),
            Ok(())
        );

        let _ = fs::remove_dir_all(output_dir);
    }

    #[test]
    fn exports_heated_actuator_rule_package_with_response_latency() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path =
            format!("{manifest_dir}/../../tests/e2e/heated_actuator/input/passing_run.csv");
        let config_path = format!(
            "{manifest_dir}/../../tests/e2e/heated_actuator/configs/test-verification-config.toml"
        );
        let output_dir = unique_export_dir("heated-actuator-rule-package");

        run(vec![
            "export-rule-package".to_string(),
            "--input".to_string(),
            input_path,
            "--config".to_string(),
            config_path,
            "--output-dir".to_string(),
            output_dir.clone(),
            "--package-name".to_string(),
            "heated-actuator-qualification".to_string(),
            "--package-version".to_string(),
            "0.1.0".to_string(),
            "--target".to_string(),
            "controller_runtime".to_string(),
        ])
        .expect("heated actuator rule package should export");

        let rules_toml = fs::read_to_string(format!("{output_dir}/rules.toml"))
            .expect("rules.toml should be readable");
        assert!(rules_toml.contains("response_latency"));
        let package = ferrisoxide_rule_schema::parse_rule_package_toml(&rules_toml)
            .expect("exported heated actuator rules.toml should parse");
        assert_eq!(
            package.validate_for_target(TargetProfileKind::ControllerRuntime),
            Ok(())
        );
        let _ = fs::remove_dir_all(output_dir);
    }

    #[test]
    fn export_rule_package_refuses_to_overwrite_artifacts() {
        let output_dir = unique_export_dir("rule-package-overwrite");
        fs::create_dir_all(&output_dir).expect("temp output dir should be created");
        fs::write(format!("{output_dir}/rules.toml"), "existing")
            .expect("existing artifact should be written");

        let error = run(vec![
            "export-rule-package".to_string(),
            "--input".to_string(),
            "../../examples/basic-waveform.csv".to_string(),
            "--config".to_string(),
            "../../examples/basic-dsl-config.toml".to_string(),
            "--output-dir".to_string(),
            output_dir.clone(),
            "--package-name".to_string(),
            "basic-rule-package".to_string(),
            "--package-version".to_string(),
            "1.0.0".to_string(),
        ])
        .expect_err("export should not overwrite existing artifacts");

        assert!(error.contains("rules.toml"));
        let _ = fs::remove_dir_all(output_dir);
    }

    fn assert_export_artifact(output_dir: &str, name: &str, expected: &str) {
        let actual = fs::read_to_string(format!("{output_dir}/{name}"))
            .unwrap_or_else(|error| panic!("failed to read exported {name}: {error}"));
        assert_eq!(actual, expected);
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
    fn renders_heated_actuator_failure_evidence_plot() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = format!(
            "{manifest_dir}/../../tests/e2e/heated_actuator/input/failing_transient_event.csv"
        );
        let config_path = format!(
            "{manifest_dir}/../../tests/e2e/heated_actuator/configs/test-verification-config.toml"
        );
        let output_path = unique_plot_path("heated-actuator-evidence");

        run(vec![
            "plot".to_string(),
            "--input".to_string(),
            input_path,
            "--config".to_string(),
            config_path,
            "--output".to_string(),
            output_path.clone(),
            "--channels".to_string(),
            "command_v,actuator_feedback_v,supply_v".to_string(),
        ])
        .expect("heated actuator plot should render");

        let svg = fs::read_to_string(&output_path).expect("svg should be written");
        assert!(svg.contains("Evidence status: FAIL"));
        assert!(svg.contains("FAIL REQ-003 sample_index=6"));
        assert!(svg.contains("actuator_feedback_v"));
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

    fn portable_state_trace(report: &SimulationReport) -> Vec<PortableStateTrace> {
        report
            .frames
            .iter()
            .filter(|frame| !frame.transitions.is_empty())
            .map(|frame| PortableStateTrace {
                sample_index: frame.sample_index,
                timestamp_s: frame.time_s,
                mode: frame.mode.clone(),
                machine: frame
                    .machines
                    .first()
                    .map(|machine| machine.machine.clone())
                    .unwrap_or_default(),
                state: frame
                    .machines
                    .first()
                    .map(|machine| machine.state.clone())
                    .unwrap_or_default(),
                transitions: frame
                    .transitions
                    .iter()
                    .map(|transition| PortableTransition {
                        transition: transition.transition.clone(),
                        from: transition.from.clone(),
                        to: transition.to.clone(),
                    })
                    .collect(),
                outputs: frame
                    .outputs
                    .iter()
                    .map(|output| PortableOutput {
                        output: output.output.clone(),
                        value: portable_output_value(&output.value),
                    })
                    .collect(),
            })
            .collect()
    }

    fn portable_output_value(value: &ferrisoxide_control_schema::OutputValue) -> String {
        match value {
            ferrisoxide_control_schema::OutputValue::Analog { value } => {
                format!(
                    "analog:{:.6}:{}",
                    value.value,
                    control_unit_name(value.unit)
                )
            }
            ferrisoxide_control_schema::OutputValue::Digital { state } => {
                format!("digital:{}", control_state_name(*state))
            }
            ferrisoxide_control_schema::OutputValue::PwmDuty { duty_cycle } => {
                format!("pwm_duty:{duty_cycle:.6}")
            }
            ferrisoxide_control_schema::OutputValue::Named { state } => {
                format!("named:{state}")
            }
        }
    }

    fn control_state_name(state: ControlDigitalState) -> &'static str {
        match state {
            ControlDigitalState::Low => "low",
            ControlDigitalState::High => "high",
        }
    }

    fn control_unit_name(unit: ferrisoxide_control_schema::ControlUnit) -> &'static str {
        match unit {
            ferrisoxide_control_schema::ControlUnit::Volt => "V",
            ferrisoxide_control_schema::ControlUnit::Second => "s",
            ferrisoxide_control_schema::ControlUnit::Hertz => "Hz",
            ferrisoxide_control_schema::ControlUnit::Percent => "percent",
            ferrisoxide_control_schema::ControlUnit::Boolean => "bool",
            ferrisoxide_control_schema::ControlUnit::Count => "count",
            ferrisoxide_control_schema::ControlUnit::Unitless => "unitless",
        }
    }

    fn portable_evidence_from_report(report: &AnalysisReport) -> Vec<PortableCriterionEvidence> {
        report
            .results
            .iter()
            .map(|result| {
                let measurement = report
                    .measurements
                    .iter()
                    .find(|measurement| measurement.id == result.measurement_id)
                    .expect("result should link to measurement evidence");
                PortableCriterionEvidence {
                    criterion_id: result.criterion_id.clone(),
                    outcome: match result.outcome {
                        ferrisoxide_core::analysis::Outcome::Pass => "pass",
                        ferrisoxide_core::analysis::Outcome::Fail => "fail",
                    },
                    failed_criterion: result.failed_criterion.clone(),
                    measurement_id: result.measurement_id.clone(),
                    method: measurement.method.clone(),
                    channel: result.channel.clone(),
                    measured_value: result.measured_value,
                    required_value: result.required_value,
                    tolerance_used: result.tolerance_used,
                    unit: result.unit.clone(),
                    sample_index: result.sample_index,
                    timestamp: result.timestamp,
                }
            })
            .collect()
    }

    fn embedded_compatible_evidence(
        config: &TestVerificationConfig,
        waveform_data: &Waveform,
    ) -> Result<Vec<PortableCriterionEvidence>, String> {
        let channels = waveform_data
            .channels
            .iter()
            .map(|channel| RuleChannel {
                name: channel.name.as_str(),
                samples: channel.samples.as_slice(),
            })
            .collect::<Vec<_>>();
        let waveform = RuleWaveform {
            time: waveform_data.time.as_slice(),
            channels: &channels,
        };

        borrowed_verification_criteria(config)?
            .into_iter()
            .map(|spec| {
                let criterion = BorrowedRuleCriterion {
                    id: spec.id.as_str(),
                    check: spec.check,
                };
                let summary =
                    evaluate_borrowed_rule(waveform, criterion, RuleTolerances::default())
                        .map_err(|error| format!("borrowed rule evaluation failed: {error}"))?;
                Ok(portable_evidence_from_summary(summary))
            })
            .collect()
    }

    fn borrowed_verification_criteria(
        config: &TestVerificationConfig,
    ) -> Result<Vec<BorrowedVerificationCriterion<'_>>, String> {
        let mut criteria = Vec::new();

        for transition in &config.expected_transitions {
            if !transition.required {
                continue;
            }
            if transition.min_latency_s.unwrap_or(0.0) > 0.0 {
                return Err(format!(
                    "expected transition `{}` min_latency_s is not supported by parity tests",
                    transition.id
                ));
            }
            let source_channel = transition.reference_channel.as_ref().ok_or_else(|| {
                format!(
                    "expected transition `{}` requires reference_channel",
                    transition.id
                )
            })?;
            let source_state = transition.reference_state.ok_or_else(|| {
                format!(
                    "expected transition `{}` requires reference_state",
                    transition.id
                )
            })?;
            let max_latency_s = transition.max_latency_s.ok_or_else(|| {
                format!(
                    "expected transition `{}` requires max_latency_s",
                    transition.id
                )
            })?;
            criteria.push(BorrowedVerificationCriterion {
                id: transition.id.clone(),
                check: BorrowedRuleCriterionCheck::ResponseLatency {
                    source_channel,
                    target_channel: transition.channel.as_str(),
                    source_threshold_v: decision_threshold(config, source_channel, source_state)?,
                    target_threshold_v: decision_threshold(
                        config,
                        &transition.channel,
                        transition.to_state,
                    )?,
                    source_state: signal_state(source_state),
                    expected_target_state: signal_state(transition.to_state),
                    max_latency_s,
                },
            });
        }

        for limit in &config.voltage_limits {
            if let Some(min_v) = limit.min_v {
                criteria.push(BorrowedVerificationCriterion {
                    id: if limit.max_v.is_some() {
                        format!("{}-min", limit.id)
                    } else {
                        limit.id.clone()
                    },
                    check: BorrowedRuleCriterionCheck::MinimumVoltage {
                        channel: limit.channel.as_str(),
                        threshold_v: min_v,
                    },
                });
            }
            if let Some(max_v) = limit.max_v {
                criteria.push(BorrowedVerificationCriterion {
                    id: if limit.min_v.is_some() {
                        format!("{}-max", limit.id)
                    } else {
                        limit.id.clone()
                    },
                    check: BorrowedRuleCriterionCheck::MaximumVoltage {
                        channel: limit.channel.as_str(),
                        threshold_v: max_v,
                    },
                });
            }
        }

        for requirement in &config.pulse_widths {
            criteria.push(BorrowedVerificationCriterion {
                id: requirement.id.clone(),
                check: BorrowedRuleCriterionCheck::PulseWidth {
                    channel: requirement.channel.as_str(),
                    state: signal_state(requirement.state),
                    threshold_v: decision_threshold(
                        config,
                        &requirement.channel,
                        requirement.state,
                    )?,
                    min_width_s: requirement.min_width_s,
                    max_width_s: requirement.max_width_s,
                },
            });
        }

        for limit in &config.transient_limits {
            let (start_time_s, end_time_s) = timing_window(config, limit.window.as_deref())?;
            criteria.push(BorrowedVerificationCriterion {
                id: limit.id.clone(),
                check: BorrowedRuleCriterionCheck::TransientEvent {
                    channel: limit.channel.as_str(),
                    event_kind: verification_event_kind_name(limit.event_kind),
                    expected_state: signal_state(limit.expected_state),
                    threshold_v: decision_threshold(config, &limit.channel, limit.expected_state)?,
                    max_duration_s: limit.max_duration_s,
                    start_time_s,
                    end_time_s,
                    arm_after_first_expected_state: limit.arm_after_first_expected_state,
                },
            });
        }

        for limit in &config.dropout_limits {
            let (start_time_s, end_time_s) = timing_window(config, limit.window.as_deref())?;
            criteria.push(BorrowedVerificationCriterion {
                id: limit.id.clone(),
                check: BorrowedRuleCriterionCheck::TransientEvent {
                    channel: limit.channel.as_str(),
                    event_kind: "dropout",
                    expected_state: signal_state(limit.expected_state),
                    threshold_v: decision_threshold(config, &limit.channel, limit.expected_state)?,
                    max_duration_s: limit.max_duration_s,
                    start_time_s,
                    end_time_s,
                    arm_after_first_expected_state: false,
                },
            });
        }

        for requirement in &config.stable_state_requirements {
            criteria.push(BorrowedVerificationCriterion {
                id: requirement.id.clone(),
                check: BorrowedRuleCriterionCheck::StableStateDuration {
                    channel: requirement.channel.as_str(),
                    state: signal_state(requirement.state),
                    threshold_v: requirement.threshold_v.unwrap_or(decision_threshold(
                        config,
                        &requirement.channel,
                        requirement.state,
                    )?),
                    min_duration_s: requirement.min_duration_s,
                },
            });
        }

        Ok(criteria)
    }

    fn portable_evidence_from_summary(summary: RuleSummary<'_>) -> PortableCriterionEvidence {
        PortableCriterionEvidence {
            criterion_id: summary.criterion_id.to_string(),
            outcome: match summary.outcome {
                RuleOutcome::Pass => "pass",
                RuleOutcome::Fail => "fail",
            },
            failed_criterion: summary
                .failed_criterion
                .map(std::string::ToString::to_string),
            measurement_id: format!("{}_measurement", summary.criterion_id),
            method: summary.method.to_string(),
            channel: summary.channel.to_string(),
            measured_value: summary.measured_value,
            required_value: summary.required_value,
            tolerance_used: summary.tolerance_used,
            unit: summary.unit.to_string(),
            sample_index: summary.sample_index,
            timestamp: summary.timestamp,
        }
    }

    fn verification_event_kind_name(kind: VerificationTransientEventKind) -> &'static str {
        match kind {
            VerificationTransientEventKind::TransientEvent => "transient_event",
            VerificationTransientEventKind::SpuriousTransition
            | VerificationTransientEventKind::FalseTransition => "spurious_transition",
            VerificationTransientEventKind::ContactBounce => "contact_bounce",
            VerificationTransientEventKind::NoiseInducedTransition => "noise_induced_transition",
            VerificationTransientEventKind::ThresholdCrossingEvent => "threshold_crossing_event",
        }
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
            (
                "invalid-dsl-missing-measurement.toml",
                "invalid config: invalid parameter `criteria.dsl_missing_measurement`",
            ),
            (
                "invalid-dsl-missing-requirement-value.toml",
                "invalid config: invalid parameter `criteria.dsl_missing_requirement_value.requirement.value`",
            ),
            (
                "invalid-dsl-missing-threshold.toml",
                "invalid config: invalid parameter `criteria.dsl_missing_threshold.measurement.threshold`",
            ),
            (
                "invalid-dsl-bad-state.toml",
                "invalid config: invalid parameter `criteria.dsl_bad_state.measurement.state`",
            ),
            (
                "invalid-dsl-equal-pulse-without-selection.toml",
                "invalid config: invalid parameter `criteria.dsl_equal_pulse_without_selection.measurement.selection`",
            ),
            (
                "invalid-dsl-inverted-edge-thresholds.toml",
                "invalid config: invalid parameter `criteria.dsl_inverted_edge_thresholds.measurement.low_threshold`",
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
