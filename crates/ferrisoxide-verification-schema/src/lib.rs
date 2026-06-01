//! Versioned test verification config schema for FerrisOxide qualification workflows.
//!
//! This crate owns schema data structures and validation helpers only. It does
//! not evaluate criteria, parse waveform CSV, simulate controllers, render SVG
//! plots, export deployment packages, talk to DAQ hardware, bind HALs, or
//! integrate RTOS SDKs.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

pub const CURRENT_VERIFICATION_SCHEMA_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestVerificationConfig {
    pub package: VerificationPackageMetadata,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub production_control: Option<ProductionControlManifestLink>,
    pub approval: ApprovalMetadata,
    pub sample_timing: VerificationSampleTiming,
    pub channels: Vec<VerificationChannel>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timing_windows: Vec<TimingWindow>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub expected_transitions: Vec<ExpectedTransition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub voltage_limits: Vec<VoltageLimit>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pulse_widths: Vec<PulseWidthRequirement>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transient_limits: Vec<TransientLimit>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dropout_limits: Vec<DropoutLimit>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stable_state_requirements: Vec<StableStateRequirement>,
    pub evidence: EvidenceSettings,
    pub report: ReportSettings,
}

impl TestVerificationConfig {
    pub fn validate(&self) -> Result<(), VerificationConfigValidationReport> {
        let mut report = VerificationConfigValidationReport::new();

        validate_non_empty("package.name", &self.package.name, &mut report);
        validate_non_empty("package.version", &self.package.version, &mut report);
        if self.package.schema_version != CURRENT_VERIFICATION_SCHEMA_VERSION {
            report.push(VerificationConfigValidationError::new(
                "package.schema_version",
                VerificationConfigValidationErrorKind::SchemaVersionMismatch,
                format!(
                    "expected schema version `{CURRENT_VERIFICATION_SCHEMA_VERSION}`, got `{}`",
                    self.package.schema_version
                ),
            ));
        }

        if let Some(production_control) = &self.production_control {
            validate_production_control_link(production_control, &mut report);
        }
        validate_approval(&self.approval, &mut report);
        validate_sample_timing(&self.sample_timing, &mut report);

        if self.channels.is_empty() {
            report.push(VerificationConfigValidationError::new(
                "channels",
                VerificationConfigValidationErrorKind::MissingChannel,
                "at least one verification channel is required",
            ));
        }

        let channel_ids = collect_ids(
            "channels",
            &self.channels,
            VerificationChannel::id,
            &mut report,
        );
        for (index, channel) in self.channels.iter().enumerate() {
            validate_channel(index, channel, &mut report);
        }

        let window_ids = collect_ids(
            "timing_windows",
            &self.timing_windows,
            TimingWindow::id,
            &mut report,
        );
        for (index, window) in self.timing_windows.iter().enumerate() {
            validate_timing_window(index, window, &mut report);
        }

        let references = VerificationReferenceSets {
            channel_ids: &channel_ids,
            window_ids: &window_ids,
        };
        let mut criterion_ids = BTreeSet::new();
        validate_expected_transitions(
            &self.expected_transitions,
            &references,
            &mut criterion_ids,
            &mut report,
        );
        validate_voltage_limits(
            &self.voltage_limits,
            &references,
            &mut criterion_ids,
            &mut report,
        );
        validate_pulse_widths(
            &self.pulse_widths,
            &references,
            &mut criterion_ids,
            &mut report,
        );
        validate_transient_limits(
            &self.transient_limits,
            &references,
            &mut criterion_ids,
            &mut report,
        );
        validate_dropout_limits(
            &self.dropout_limits,
            &references,
            &mut criterion_ids,
            &mut report,
        );
        validate_stable_state_requirements(
            &self.stable_state_requirements,
            &references,
            &mut criterion_ids,
            &mut report,
        );

        if criterion_ids.is_empty() {
            report.push(VerificationConfigValidationError::new(
                "criteria",
                VerificationConfigValidationErrorKind::MissingCriterion,
                "at least one verification criterion is required",
            ));
        }

        validate_evidence_settings(&self.evidence, &references, &mut report);
        validate_report_settings(&self.report, &mut report);

        report.into_result()
    }
}

pub fn parse_verification_config_json(
    input: &str,
) -> Result<TestVerificationConfig, VerificationConfigValidationError> {
    serde_json::from_str(input).map_err(|error| {
        VerificationConfigValidationError::new(
            "test-verification-config.json",
            VerificationConfigValidationErrorKind::ParseError,
            error.to_string(),
        )
    })
}

pub fn parse_verification_config_toml(
    input: &str,
) -> Result<TestVerificationConfig, VerificationConfigValidationError> {
    toml::from_str(input).map_err(|error| {
        VerificationConfigValidationError::new(
            "test-verification-config.toml",
            VerificationConfigValidationErrorKind::ParseError,
            error.to_string(),
        )
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationPackageMetadata {
    pub name: String,
    pub version: String,
    pub schema_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl VerificationPackageMetadata {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            schema_version: CURRENT_VERIFICATION_SCHEMA_VERSION.to_string(),
            description: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductionControlManifestLink {
    pub package_name: String,
    pub package_version: String,
    pub schema_version: String,
    pub manifest_artifact: String,
    pub checksum: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalMetadata {
    pub status: ApprovalStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_at: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    Draft,
    Reviewed,
    Approved,
    Retired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerificationSampleTiming {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_rate_hz: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal_sample_period_s: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_time_gap_s: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerificationChannel {
    pub id: String,
    pub column: String,
    pub unit: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub low_threshold: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high_threshold: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl VerificationChannel {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimingWindow {
    pub id: String,
    pub start_s: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_s: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl TimingWindow {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpectedTransition {
    pub id: String,
    pub channel: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_state: Option<DigitalState>,
    pub to_state: DigitalState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_channel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_state: Option<DigitalState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_latency_s: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_latency_s: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
    #[serde(default = "default_required")]
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoltageLimit {
    pub id: String,
    pub channel: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_v: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_v: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PulseWidthRequirement {
    pub id: String,
    pub channel: String,
    pub state: DigitalState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_width_s: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_width_s: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientLimit {
    pub id: String,
    pub channel: String,
    pub event_kind: TransientEventKind,
    pub expected_state: DigitalState,
    pub max_duration_s: f64,
    pub allowed_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
    #[serde(default)]
    pub arm_after_first_expected_state: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DropoutLimit {
    pub id: String,
    pub channel: String,
    pub expected_state: DigitalState,
    pub max_duration_s: f64,
    pub allowed_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StableStateRequirement {
    pub id: String,
    pub channel: String,
    pub state: DigitalState,
    pub min_duration_s: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold_v: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DigitalState {
    Low,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransientEventKind {
    TransientEvent,
    SpuriousTransition,
    ContactBounce,
    FalseTransition,
    NoiseInducedTransition,
    ThresholdCrossingEvent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceSettings {
    pub include_passed_criteria: bool,
    pub include_failed_criteria: bool,
    pub include_measurements: bool,
    pub include_sample_index: bool,
    pub include_timestamp: bool,
    pub include_channel: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<EvidenceArtifactRequest>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceArtifactRequest {
    pub id: String,
    pub kind: EvidenceArtifactKind,
    pub path: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub channels: Vec<String>,
}

impl EvidenceArtifactRequest {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceArtifactKind {
    JsonReport,
    TextReport,
    SvgPlot,
    QualificationBundle,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportSettings {
    pub formats: Vec<ReportFormat>,
    pub include_overall_status: bool,
    pub include_failed_criterion: bool,
    pub include_measured_value: bool,
    pub include_required_value: bool,
    pub include_sample_index: bool,
    pub include_timestamp: bool,
    pub include_channel: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationConfigValidationReport {
    pub errors: Vec<VerificationConfigValidationError>,
}

impl VerificationConfigValidationReport {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn push(&mut self, error: VerificationConfigValidationError) {
        self.errors.push(error);
    }

    fn into_result(self) -> Result<(), Self> {
        if self.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl Default for VerificationConfigValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for VerificationConfigValidationReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for error in &self.errors {
            writeln!(
                formatter,
                "{}: {:?}: {}",
                error.field, error.kind, error.message
            )?;
        }
        Ok(())
    }
}

impl std::error::Error for VerificationConfigValidationReport {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationConfigValidationError {
    pub field: String,
    pub kind: VerificationConfigValidationErrorKind,
    pub message: String,
}

impl VerificationConfigValidationError {
    pub fn new(
        field: impl Into<String>,
        kind: VerificationConfigValidationErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            kind,
            message: message.into(),
        }
    }
}

impl fmt::Display for VerificationConfigValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.field, self.message)
    }
}

impl std::error::Error for VerificationConfigValidationError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationConfigValidationErrorKind {
    ParseError,
    SchemaVersionMismatch,
    EmptyField,
    DuplicateIdentifier,
    MissingChannel,
    MissingCriterion,
    UnknownChannel,
    UnknownTimingWindow,
    InvalidApproval,
    InvalidProductionControlLink,
    InvalidTiming,
    InvalidThreshold,
    InvalidLimit,
    InvalidEvidenceSettings,
    InvalidReportSettings,
}

#[derive(Clone, Copy)]
struct VerificationReferenceSets<'a> {
    channel_ids: &'a BTreeSet<String>,
    window_ids: &'a BTreeSet<String>,
}

fn default_required() -> bool {
    true
}

fn validate_production_control_link(
    link: &ProductionControlManifestLink,
    report: &mut VerificationConfigValidationReport,
) {
    validate_non_empty(
        "production_control.package_name",
        &link.package_name,
        report,
    );
    validate_non_empty(
        "production_control.package_version",
        &link.package_version,
        report,
    );
    validate_non_empty(
        "production_control.schema_version",
        &link.schema_version,
        report,
    );
    validate_non_empty(
        "production_control.manifest_artifact",
        &link.manifest_artifact,
        report,
    );
    validate_non_empty("production_control.checksum", &link.checksum, report);
    if !link.checksum.contains(':') {
        report.push(VerificationConfigValidationError::new(
            "production_control.checksum",
            VerificationConfigValidationErrorKind::InvalidProductionControlLink,
            "checksum should include an algorithm prefix such as `fnv1a64:`",
        ));
    }
}

fn validate_approval(approval: &ApprovalMetadata, report: &mut VerificationConfigValidationReport) {
    if approval.status == ApprovalStatus::Approved {
        if approval
            .approved_by
            .as_deref()
            .unwrap_or_default()
            .is_empty()
        {
            report.push(VerificationConfigValidationError::new(
                "approval.approved_by",
                VerificationConfigValidationErrorKind::InvalidApproval,
                "approved configs must identify an approver",
            ));
        }
        if approval
            .approved_at
            .as_deref()
            .unwrap_or_default()
            .is_empty()
        {
            report.push(VerificationConfigValidationError::new(
                "approval.approved_at",
                VerificationConfigValidationErrorKind::InvalidApproval,
                "approved configs must include an approval timestamp",
            ));
        }
    }
}

fn validate_sample_timing(
    timing: &VerificationSampleTiming,
    report: &mut VerificationConfigValidationReport,
) {
    validate_optional_positive_finite(
        "sample_timing.sample_rate_hz",
        timing.sample_rate_hz,
        report,
    );
    validate_optional_positive_finite(
        "sample_timing.nominal_sample_period_s",
        timing.nominal_sample_period_s,
        report,
    );
    validate_optional_positive_finite(
        "sample_timing.max_time_gap_s",
        timing.max_time_gap_s,
        report,
    );
    if timing.sample_rate_hz.is_none() && timing.nominal_sample_period_s.is_none() {
        report.push(VerificationConfigValidationError::new(
            "sample_timing",
            VerificationConfigValidationErrorKind::InvalidTiming,
            "sample_rate_hz or nominal_sample_period_s is required",
        ));
    }
    if timing.time_unit.as_deref() == Some("") {
        report.push(VerificationConfigValidationError::new(
            "sample_timing.time_unit",
            VerificationConfigValidationErrorKind::EmptyField,
            "time unit cannot be empty when provided",
        ));
    }
}

fn validate_channel(
    index: usize,
    channel: &VerificationChannel,
    report: &mut VerificationConfigValidationReport,
) {
    let field = format!("channels[{index}]");
    validate_non_empty(&format!("{field}.id"), &channel.id, report);
    validate_non_empty(&format!("{field}.column"), &channel.column, report);
    validate_non_empty(&format!("{field}.unit"), &channel.unit, report);
    validate_optional_finite(
        &format!("{field}.low_threshold"),
        channel.low_threshold,
        report,
    );
    validate_optional_finite(
        &format!("{field}.high_threshold"),
        channel.high_threshold,
        report,
    );
    if let (Some(low), Some(high)) = (channel.low_threshold, channel.high_threshold) {
        if low >= high {
            report.push(VerificationConfigValidationError::new(
                format!("{field}.low_threshold"),
                VerificationConfigValidationErrorKind::InvalidThreshold,
                "low_threshold must be lower than high_threshold",
            ));
        }
    }
}

fn validate_timing_window(
    index: usize,
    window: &TimingWindow,
    report: &mut VerificationConfigValidationReport,
) {
    let field = format!("timing_windows[{index}]");
    validate_non_empty(&format!("{field}.id"), &window.id, report);
    validate_non_negative_finite(&format!("{field}.start_s"), window.start_s, report);
    if let Some(end_s) = window.end_s {
        validate_positive_finite(&format!("{field}.end_s"), end_s, report);
        if end_s <= window.start_s {
            report.push(VerificationConfigValidationError::new(
                format!("{field}.end_s"),
                VerificationConfigValidationErrorKind::InvalidTiming,
                "end_s must be greater than start_s",
            ));
        }
    }
}

fn validate_expected_transitions(
    transitions: &[ExpectedTransition],
    references: &VerificationReferenceSets<'_>,
    criterion_ids: &mut BTreeSet<String>,
    report: &mut VerificationConfigValidationReport,
) {
    for (index, transition) in transitions.iter().enumerate() {
        let field = format!("expected_transitions[{index}]");
        validate_criterion_id(&field, &transition.id, criterion_ids, report);
        validate_channel_ref(
            &format!("{field}.channel"),
            &transition.channel,
            references,
            report,
        );
        validate_optional_channel_ref(
            &format!("{field}.reference_channel"),
            transition.reference_channel.as_deref(),
            references,
            report,
        );
        validate_optional_window_ref(
            &format!("{field}.window"),
            transition.window.as_deref(),
            references,
            report,
        );
        validate_optional_non_negative_finite(
            &format!("{field}.min_latency_s"),
            transition.min_latency_s,
            report,
        );
        validate_optional_positive_finite(
            &format!("{field}.max_latency_s"),
            transition.max_latency_s,
            report,
        );
        if let (Some(min), Some(max)) = (transition.min_latency_s, transition.max_latency_s) {
            if max < min {
                report.push(VerificationConfigValidationError::new(
                    format!("{field}.max_latency_s"),
                    VerificationConfigValidationErrorKind::InvalidTiming,
                    "max_latency_s must be greater than or equal to min_latency_s",
                ));
            }
        }
    }
}

fn validate_voltage_limits(
    limits: &[VoltageLimit],
    references: &VerificationReferenceSets<'_>,
    criterion_ids: &mut BTreeSet<String>,
    report: &mut VerificationConfigValidationReport,
) {
    for (index, limit) in limits.iter().enumerate() {
        let field = format!("voltage_limits[{index}]");
        validate_criterion_id(&field, &limit.id, criterion_ids, report);
        validate_channel_ref(
            &format!("{field}.channel"),
            &limit.channel,
            references,
            report,
        );
        validate_optional_window_ref(
            &format!("{field}.window"),
            limit.window.as_deref(),
            references,
            report,
        );
        validate_optional_finite(&format!("{field}.min_v"), limit.min_v, report);
        validate_optional_finite(&format!("{field}.max_v"), limit.max_v, report);
        if limit.min_v.is_none() && limit.max_v.is_none() {
            report.push(VerificationConfigValidationError::new(
                field,
                VerificationConfigValidationErrorKind::InvalidLimit,
                "voltage limit requires min_v, max_v, or both",
            ));
        }
        if let (Some(min), Some(max)) = (limit.min_v, limit.max_v) {
            if max < min {
                report.push(VerificationConfigValidationError::new(
                    format!("voltage_limits[{index}].max_v"),
                    VerificationConfigValidationErrorKind::InvalidLimit,
                    "max_v must be greater than or equal to min_v",
                ));
            }
        }
    }
}

fn validate_pulse_widths(
    pulse_widths: &[PulseWidthRequirement],
    references: &VerificationReferenceSets<'_>,
    criterion_ids: &mut BTreeSet<String>,
    report: &mut VerificationConfigValidationReport,
) {
    for (index, pulse_width) in pulse_widths.iter().enumerate() {
        let field = format!("pulse_widths[{index}]");
        validate_criterion_id(&field, &pulse_width.id, criterion_ids, report);
        validate_channel_ref(
            &format!("{field}.channel"),
            &pulse_width.channel,
            references,
            report,
        );
        validate_optional_window_ref(
            &format!("{field}.window"),
            pulse_width.window.as_deref(),
            references,
            report,
        );
        validate_optional_positive_finite(
            &format!("{field}.min_width_s"),
            pulse_width.min_width_s,
            report,
        );
        validate_optional_positive_finite(
            &format!("{field}.max_width_s"),
            pulse_width.max_width_s,
            report,
        );
        if pulse_width.min_width_s.is_none() && pulse_width.max_width_s.is_none() {
            report.push(VerificationConfigValidationError::new(
                field,
                VerificationConfigValidationErrorKind::InvalidLimit,
                "pulse width requires min_width_s, max_width_s, or both",
            ));
        }
        if let (Some(min), Some(max)) = (pulse_width.min_width_s, pulse_width.max_width_s) {
            if max < min {
                report.push(VerificationConfigValidationError::new(
                    format!("pulse_widths[{index}].max_width_s"),
                    VerificationConfigValidationErrorKind::InvalidLimit,
                    "max_width_s must be greater than or equal to min_width_s",
                ));
            }
        }
    }
}

fn validate_transient_limits(
    limits: &[TransientLimit],
    references: &VerificationReferenceSets<'_>,
    criterion_ids: &mut BTreeSet<String>,
    report: &mut VerificationConfigValidationReport,
) {
    for (index, limit) in limits.iter().enumerate() {
        let field = format!("transient_limits[{index}]");
        validate_criterion_id(&field, &limit.id, criterion_ids, report);
        validate_channel_ref(
            &format!("{field}.channel"),
            &limit.channel,
            references,
            report,
        );
        validate_optional_window_ref(
            &format!("{field}.window"),
            limit.window.as_deref(),
            references,
            report,
        );
        validate_positive_finite(
            &format!("{field}.max_duration_s"),
            limit.max_duration_s,
            report,
        );
    }
}

fn validate_dropout_limits(
    limits: &[DropoutLimit],
    references: &VerificationReferenceSets<'_>,
    criterion_ids: &mut BTreeSet<String>,
    report: &mut VerificationConfigValidationReport,
) {
    for (index, limit) in limits.iter().enumerate() {
        let field = format!("dropout_limits[{index}]");
        validate_criterion_id(&field, &limit.id, criterion_ids, report);
        validate_channel_ref(
            &format!("{field}.channel"),
            &limit.channel,
            references,
            report,
        );
        validate_optional_window_ref(
            &format!("{field}.window"),
            limit.window.as_deref(),
            references,
            report,
        );
        validate_positive_finite(
            &format!("{field}.max_duration_s"),
            limit.max_duration_s,
            report,
        );
    }
}

fn validate_stable_state_requirements(
    requirements: &[StableStateRequirement],
    references: &VerificationReferenceSets<'_>,
    criterion_ids: &mut BTreeSet<String>,
    report: &mut VerificationConfigValidationReport,
) {
    for (index, requirement) in requirements.iter().enumerate() {
        let field = format!("stable_state_requirements[{index}]");
        validate_criterion_id(&field, &requirement.id, criterion_ids, report);
        validate_channel_ref(
            &format!("{field}.channel"),
            &requirement.channel,
            references,
            report,
        );
        validate_optional_window_ref(
            &format!("{field}.window"),
            requirement.window.as_deref(),
            references,
            report,
        );
        validate_positive_finite(
            &format!("{field}.min_duration_s"),
            requirement.min_duration_s,
            report,
        );
        validate_optional_finite(
            &format!("{field}.threshold_v"),
            requirement.threshold_v,
            report,
        );
    }
}

fn validate_evidence_settings(
    evidence: &EvidenceSettings,
    references: &VerificationReferenceSets<'_>,
    report: &mut VerificationConfigValidationReport,
) {
    if !evidence.include_failed_criteria {
        report.push(VerificationConfigValidationError::new(
            "evidence.include_failed_criteria",
            VerificationConfigValidationErrorKind::InvalidEvidenceSettings,
            "failure evidence must include failed criteria",
        ));
    }
    if !evidence.include_measured_value_like_fields() {
        report.push(VerificationConfigValidationError::new(
            "evidence",
            VerificationConfigValidationErrorKind::InvalidEvidenceSettings,
            "evidence should include measurements, sample index, timestamp, and channel",
        ));
    }

    let _artifact_ids = collect_ids(
        "evidence.artifacts",
        &evidence.artifacts,
        EvidenceArtifactRequest::id,
        report,
    );
    for (index, artifact) in evidence.artifacts.iter().enumerate() {
        let field = format!("evidence.artifacts[{index}]");
        validate_non_empty(&format!("{field}.id"), &artifact.id, report);
        validate_non_empty(&format!("{field}.path"), &artifact.path, report);
        for (channel_index, channel) in artifact.channels.iter().enumerate() {
            validate_channel_ref(
                &format!("{field}.channels[{channel_index}]"),
                channel,
                references,
                report,
            );
        }
    }
}

impl EvidenceSettings {
    fn include_measured_value_like_fields(&self) -> bool {
        self.include_measurements
            && self.include_sample_index
            && self.include_timestamp
            && self.include_channel
    }
}

fn validate_report_settings(
    report_settings: &ReportSettings,
    report: &mut VerificationConfigValidationReport,
) {
    if report_settings.formats.is_empty() {
        report.push(VerificationConfigValidationError::new(
            "report.formats",
            VerificationConfigValidationErrorKind::InvalidReportSettings,
            "at least one report format is required",
        ));
    }
    if !report_settings.include_overall_status
        || !report_settings.include_failed_criterion
        || !report_settings.include_measured_value
        || !report_settings.include_required_value
        || !report_settings.include_sample_index
        || !report_settings.include_timestamp
        || !report_settings.include_channel
    {
        report.push(VerificationConfigValidationError::new(
            "report",
            VerificationConfigValidationErrorKind::InvalidReportSettings,
            "reports must include PASS/FAIL, failed criterion, measured value, required value, sample index, timestamp, and channel fields",
        ));
    }
}

fn validate_criterion_id(
    field: &str,
    id: &str,
    criterion_ids: &mut BTreeSet<String>,
    report: &mut VerificationConfigValidationReport,
) {
    validate_non_empty(&format!("{field}.id"), id, report);
    if !id.is_empty() && !criterion_ids.insert(id.to_string()) {
        report.push(VerificationConfigValidationError::new(
            format!("{field}.id"),
            VerificationConfigValidationErrorKind::DuplicateIdentifier,
            format!("duplicate criterion id `{id}`"),
        ));
    }
}

fn validate_channel_ref(
    field: &str,
    channel: &str,
    references: &VerificationReferenceSets<'_>,
    report: &mut VerificationConfigValidationReport,
) {
    validate_non_empty(field, channel, report);
    if !channel.is_empty() && !references.channel_ids.contains(channel) {
        report.push(VerificationConfigValidationError::new(
            field,
            VerificationConfigValidationErrorKind::UnknownChannel,
            format!("unknown channel `{channel}`"),
        ));
    }
}

fn validate_optional_channel_ref(
    field: &str,
    channel: Option<&str>,
    references: &VerificationReferenceSets<'_>,
    report: &mut VerificationConfigValidationReport,
) {
    if let Some(channel) = channel {
        validate_channel_ref(field, channel, references, report);
    }
}

fn validate_optional_window_ref(
    field: &str,
    window: Option<&str>,
    references: &VerificationReferenceSets<'_>,
    report: &mut VerificationConfigValidationReport,
) {
    if let Some(window) = window {
        validate_non_empty(field, window, report);
        if !window.is_empty() && !references.window_ids.contains(window) {
            report.push(VerificationConfigValidationError::new(
                field,
                VerificationConfigValidationErrorKind::UnknownTimingWindow,
                format!("unknown timing window `{window}`"),
            ));
        }
    }
}

fn collect_ids<T>(
    collection_name: &str,
    values: &[T],
    id: impl Fn(&T) -> &str,
    report: &mut VerificationConfigValidationReport,
) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for (index, value) in values.iter().enumerate() {
        let value_id = id(value);
        if value_id.is_empty() {
            continue;
        }
        if !ids.insert(value_id.to_string()) {
            report.push(VerificationConfigValidationError::new(
                format!("{collection_name}[{index}].id"),
                VerificationConfigValidationErrorKind::DuplicateIdentifier,
                format!("duplicate id `{value_id}` in {collection_name}"),
            ));
        }
    }
    ids
}

fn validate_non_empty(field: &str, value: &str, report: &mut VerificationConfigValidationReport) {
    if value.is_empty() {
        report.push(VerificationConfigValidationError::new(
            field,
            VerificationConfigValidationErrorKind::EmptyField,
            "value cannot be empty",
        ));
    }
}

fn validate_optional_finite(
    field: &str,
    value: Option<f64>,
    report: &mut VerificationConfigValidationReport,
) {
    if let Some(value) = value {
        if !value.is_finite() {
            report.push(VerificationConfigValidationError::new(
                field,
                VerificationConfigValidationErrorKind::InvalidLimit,
                "value must be finite",
            ));
        }
    }
}

fn validate_optional_positive_finite(
    field: &str,
    value: Option<f64>,
    report: &mut VerificationConfigValidationReport,
) {
    if let Some(value) = value {
        validate_positive_finite(field, value, report);
    }
}

fn validate_optional_non_negative_finite(
    field: &str,
    value: Option<f64>,
    report: &mut VerificationConfigValidationReport,
) {
    if let Some(value) = value {
        validate_non_negative_finite(field, value, report);
    }
}

fn validate_positive_finite(
    field: &str,
    value: f64,
    report: &mut VerificationConfigValidationReport,
) {
    if !value.is_finite() || value <= 0.0 {
        report.push(VerificationConfigValidationError::new(
            field,
            VerificationConfigValidationErrorKind::InvalidTiming,
            "value must be finite and greater than zero",
        ));
    }
}

fn validate_non_negative_finite(
    field: &str,
    value: f64,
    report: &mut VerificationConfigValidationReport,
) {
    if !value.is_finite() || value < 0.0 {
        report.push(VerificationConfigValidationError::new(
            field,
            VerificationConfigValidationErrorKind::InvalidTiming,
            "value must be finite and greater than or equal to zero",
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_and_validates_test_verification_config_schema() {
        let config = valid_config();

        assert!(config.validate().is_ok());
        assert_eq!(config.expected_transitions.len(), 1);
        assert_eq!(config.voltage_limits.len(), 1);
        assert_eq!(config.pulse_widths.len(), 1);
        assert_eq!(config.transient_limits.len(), 1);
        assert_eq!(config.dropout_limits.len(), 1);
        assert_eq!(config.stable_state_requirements.len(), 1);
    }

    #[test]
    fn parses_example_toml_and_json_round_trip() {
        let toml_input = include_str!(
            "../../../examples/test-verification-config/test-verification-config.toml"
        );
        let config = parse_verification_config_toml(toml_input).expect("example TOML parses");

        config.validate().expect("example config validates");

        let json = serde_json::to_string_pretty(&config).expect("serialize config");
        let parsed_json = parse_verification_config_json(&json).expect("parse serialized JSON");

        assert_eq!(config, parsed_json);
    }

    #[test]
    fn rejects_missing_references_and_invalid_values() {
        let mut config = valid_config();
        config.channels.push(VerificationChannel {
            id: "feedback".to_string(),
            column: "duplicate_feedback_v".to_string(),
            unit: "V".to_string(),
            low_threshold: Some(4.5),
            high_threshold: Some(0.5),
            description: None,
        });
        config.timing_windows[0].end_s = Some(0.5);
        config.expected_transitions[0].channel = "missing".to_string();
        config.expected_transitions[0].max_latency_s = Some(0.010);
        config.expected_transitions[0].min_latency_s = Some(0.020);
        config.voltage_limits[0].window = Some("missing-window".to_string());
        config.voltage_limits[0].min_v = Some(5.25);
        config.voltage_limits[0].max_v = Some(4.75);
        config.pulse_widths[0].min_width_s = None;
        config.pulse_widths[0].max_width_s = None;
        config.transient_limits[0].max_duration_s = 0.0;
        config.report.formats.clear();

        let report = config.validate().expect_err("invalid config should fail");
        let kinds: Vec<_> = report.errors.iter().map(|error| error.kind).collect();

        assert!(kinds.contains(&VerificationConfigValidationErrorKind::DuplicateIdentifier));
        assert!(kinds.contains(&VerificationConfigValidationErrorKind::InvalidThreshold));
        assert!(kinds.contains(&VerificationConfigValidationErrorKind::UnknownChannel));
        assert!(kinds.contains(&VerificationConfigValidationErrorKind::UnknownTimingWindow));
        assert!(kinds.contains(&VerificationConfigValidationErrorKind::InvalidLimit));
        assert!(kinds.contains(&VerificationConfigValidationErrorKind::InvalidTiming));
        assert!(kinds.contains(&VerificationConfigValidationErrorKind::InvalidReportSettings));
    }

    #[test]
    fn requires_approval_metadata_for_approved_configs() {
        let mut config = valid_config();
        config.approval.status = ApprovalStatus::Approved;
        config.approval.approved_by = None;
        config.approval.approved_at = None;

        let report = config
            .validate()
            .expect_err("approved config without metadata should fail");

        assert!(report
            .errors
            .iter()
            .any(|error| error.kind == VerificationConfigValidationErrorKind::InvalidApproval));
    }

    #[test]
    fn links_to_production_control_only_by_manifest_metadata() {
        let mut config = valid_config();
        config.production_control = Some(ProductionControlManifestLink {
            package_name: "heated-actuator-production-control".to_string(),
            package_version: "0.1.0".to_string(),
            schema_version: "0.1.0".to_string(),
            manifest_artifact: "deployment/manifest.json".to_string(),
            checksum: "missing-prefix".to_string(),
        });

        let report = config
            .validate()
            .expect_err("bad production control metadata should fail");

        assert!(report.errors.iter().any(|error| {
            error.kind == VerificationConfigValidationErrorKind::InvalidProductionControlLink
                && error.field == "production_control.checksum"
        }));
    }

    fn valid_config() -> TestVerificationConfig {
        TestVerificationConfig {
            package: VerificationPackageMetadata::new("heated-actuator-qualification", "0.1.0"),
            production_control: Some(ProductionControlManifestLink {
                package_name: "heated-actuator-production-control".to_string(),
                package_version: "0.1.0".to_string(),
                schema_version: "0.1.0".to_string(),
                manifest_artifact: "deployment/manifest.json".to_string(),
                checksum: "fnv1a64:0123456789abcdef".to_string(),
            }),
            approval: ApprovalMetadata {
                status: ApprovalStatus::Draft,
                approved_by: None,
                approved_at: None,
                evidence_refs: Vec::new(),
                notes: vec!["software-only schema fixture".to_string()],
            },
            sample_timing: VerificationSampleTiming {
                sample_rate_hz: Some(1_000.0),
                nominal_sample_period_s: Some(0.001),
                max_time_gap_s: Some(0.002),
                time_unit: Some("s".to_string()),
            },
            channels: vec![
                VerificationChannel {
                    id: "command".to_string(),
                    column: "command_v".to_string(),
                    unit: "V".to_string(),
                    low_threshold: Some(0.5),
                    high_threshold: Some(4.5),
                    description: None,
                },
                VerificationChannel {
                    id: "feedback".to_string(),
                    column: "actuator_feedback_v".to_string(),
                    unit: "V".to_string(),
                    low_threshold: Some(0.5),
                    high_threshold: Some(4.5),
                    description: None,
                },
                VerificationChannel {
                    id: "supply".to_string(),
                    column: "supply_v".to_string(),
                    unit: "V".to_string(),
                    low_threshold: None,
                    high_threshold: None,
                    description: None,
                },
            ],
            timing_windows: vec![TimingWindow {
                id: "commanded-open".to_string(),
                start_s: 1.0,
                end_s: Some(1.55),
                description: None,
            }],
            expected_transitions: vec![ExpectedTransition {
                id: "REQ-001".to_string(),
                channel: "feedback".to_string(),
                from_state: Some(DigitalState::Low),
                to_state: DigitalState::High,
                reference_channel: Some("command".to_string()),
                reference_state: Some(DigitalState::High),
                min_latency_s: Some(0.0),
                max_latency_s: Some(0.050),
                window: Some("commanded-open".to_string()),
                required: true,
            }],
            voltage_limits: vec![VoltageLimit {
                id: "REQ-004".to_string(),
                channel: "supply".to_string(),
                min_v: Some(4.75),
                max_v: Some(5.25),
                window: None,
            }],
            pulse_widths: vec![PulseWidthRequirement {
                id: "REQ-005".to_string(),
                channel: "feedback".to_string(),
                state: DigitalState::High,
                min_width_s: Some(0.500),
                max_width_s: None,
                window: Some("commanded-open".to_string()),
            }],
            transient_limits: vec![TransientLimit {
                id: "REQ-003".to_string(),
                channel: "feedback".to_string(),
                event_kind: TransientEventKind::SpuriousTransition,
                expected_state: DigitalState::High,
                max_duration_s: 0.001,
                allowed_count: 0,
                window: Some("commanded-open".to_string()),
                arm_after_first_expected_state: true,
            }],
            dropout_limits: vec![DropoutLimit {
                id: "REQ-006".to_string(),
                channel: "supply".to_string(),
                expected_state: DigitalState::High,
                max_duration_s: 0.001,
                allowed_count: 0,
                window: None,
            }],
            stable_state_requirements: vec![StableStateRequirement {
                id: "REQ-002".to_string(),
                channel: "feedback".to_string(),
                state: DigitalState::High,
                min_duration_s: 0.500,
                threshold_v: Some(2.5),
                window: Some("commanded-open".to_string()),
            }],
            evidence: EvidenceSettings {
                include_passed_criteria: true,
                include_failed_criteria: true,
                include_measurements: true,
                include_sample_index: true,
                include_timestamp: true,
                include_channel: true,
                artifacts: vec![EvidenceArtifactRequest {
                    id: "qualification-svg".to_string(),
                    kind: EvidenceArtifactKind::SvgPlot,
                    path: "qualification-evidence.svg".to_string(),
                    channels: vec![
                        "command".to_string(),
                        "feedback".to_string(),
                        "supply".to_string(),
                    ],
                }],
            },
            report: ReportSettings {
                formats: vec![ReportFormat::Text, ReportFormat::Json],
                include_overall_status: true,
                include_failed_criterion: true,
                include_measured_value: true,
                include_required_value: true,
                include_sample_index: true,
                include_timestamp: true,
                include_channel: true,
            },
        }
    }
}
