use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

use ferrisoxide_core::analysis::evaluate_criteria_with_measurements;
use ferrisoxide_core::config::AnalysisConfig;
use ferrisoxide_core::criteria::{
    Criterion, CriterionCheck, CriterionOperator, EdgeDirection, MeasurementSpec,
    RunSelectionConfig, SignalState, TransientEventKind,
};
use ferrisoxide_core::csv::{CsvParseOptions, SimpleCsvParser, WaveformParser};
use ferrisoxide_core::filter::{
    apply_filter_chain, AdcQuantizer, FilterStep, LowPassFilter, MovingAverageFilter,
};
use ferrisoxide_core::model::{MetadataContext, TolerancePolicy};
use ferrisoxide_core::report::{AnalysisReport, ReportEvidenceContext};
use ferrisoxide_plot::{evidence_overlays, render_svg, PlotOptions};
use ferrisoxide_rule_schema::{
    checksum_str, ChannelDefinition, ChecksumMetadata, ComparisonOperator, CriterionDefinition,
    EngineeringUnit, FilterDefinition, ManifestArtifact, ManifestSources,
    ManifestValidationEvidence, MeasurementDefinition, PackageMetadata, RequirementDefinition,
    RulePackage, RulePackageManifest, SampleTimingAssumption, TargetProfile, TargetProfileKind,
    ThresholdDefinition, ThresholdRole, UnitValue,
};

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
        Some("export-rule-package") => run_export_rule_package(&args),
        Some(other) => Err(format!(
            "expected subcommand `analyze`, `plot`, or `export-rule-package`, got `{other}`"
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
    let criteria = config
        .criteria()
        .map_err(|error| format!("invalid config criteria: {error}"))?;
    let evaluation = evaluate_criteria_with_measurements(&waveform, &criteria, config.tolerances)
        .map_err(|error| format!("analysis failed: {error}"))?;
    let report = AnalysisReport {
        input_name: input_path.to_string(),
        waveform_metadata: waveform.metadata.clone(),
        evidence_context: ReportEvidenceContext::engineering_validation(config.tolerances),
        measurements: evaluation.measurements,
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
            let id = match filter {
                FilterStep::MovingAverage(_) => format!("moving_average_{index}_{channel}"),
                FilterStep::LowPass(_) => format!("low_pass_{index}_{channel}"),
                FilterStep::AdcQuantize(_) => format!("adc_quantize_{index}_{channel}"),
            };
            schema_filters.push(match filter {
                FilterStep::MovingAverage(filter) => FilterDefinition::MovingAverage {
                    id,
                    channel: channel.clone(),
                    window_samples: filter.window_samples,
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
        "FerrisOxide Signal",
        "",
        "Usage:",
        "  ferrisoxide-signal analyze --input <csv> --config examples/basic-config.toml --format text",
        "  ferrisoxide-signal analyze --input <csv> --time-column time --channels input_v --moving-average 3 --low-pass 25 --adc-quantize 12:0.0:5.0 --min input_v:0.0 --max input_v:5.5 --format json",
        "  ferrisoxide-signal plot --input <csv> --time-column time --channels input_v --output waveform.svg",
        "  ferrisoxide-signal plot --input <csv> --config examples/basic-config.toml --output annotated.svg",
        "  ferrisoxide-signal plot --input <csv> --time-column time --channels input_v --z-column temp_c --output waveform-3d.svg",
        "  ferrisoxide-signal export-rule-package --input <csv> --config examples/basic-dsl-config.toml --output-dir deployment --package-name switch-rule --package-version 1.0.0 --target controller_runtime",
        "",
        "ADC quantization syntax: --adc-quantize bits:min_v:max_v",
        "Plot output is SVG. Use --config to add 2D criteria evidence overlays; use --z-column for an optional third axis.",
        "Rule package export writes rules.toml, rules.json, and validation-report.json without overwriting existing artifacts.",
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
