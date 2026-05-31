//! Versioned portable FerrisOxide Rule Package schema.
//!
//! This crate owns data structures only. It does not evaluate rules, parse CSV,
//! render reports, export deployment packages, or bind any controller, DAQ,
//! RTOS, SDK, or hardware HAL.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

pub const CURRENT_SCHEMA_VERSION: &str = "0.1.0";
pub const CURRENT_MANIFEST_VERSION: &str = "0.1.0";
pub const CHECKSUM_ALGORITHM: &str = "fnv1a64";
pub const CHECKSUM_FORMAT: &str = "hex";
pub const CHECKSUM_SCOPE: &str = "artifact_contents";
pub const CHECKSUM_SECURITY_NOTE: &str =
    "deterministic artifact drift detection only; not cryptographic signing or certification evidence";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulePackage {
    pub package: PackageMetadata,
    pub target: TargetProfile,
    pub sample_timing: SampleTimingAssumption,
    pub channels: Vec<ChannelDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<FilterDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub criteria: Vec<CriterionDefinition>,
}

impl RulePackage {
    pub fn new(
        package: PackageMetadata,
        target: TargetProfile,
        sample_timing: SampleTimingAssumption,
        channels: Vec<ChannelDefinition>,
        criteria: Vec<CriterionDefinition>,
    ) -> Self {
        Self {
            package,
            target,
            sample_timing,
            channels,
            filters: Vec::new(),
            criteria,
        }
    }

    pub fn validate(&self) -> Result<(), RulePackageValidationReport> {
        self.validate_with_expected_target(None)
    }

    pub fn validate_for_target(
        &self,
        expected: TargetProfileKind,
    ) -> Result<(), RulePackageValidationReport> {
        self.validate_with_expected_target(Some(expected))
    }

    fn validate_with_expected_target(
        &self,
        expected: Option<TargetProfileKind>,
    ) -> Result<(), RulePackageValidationReport> {
        let mut report = RulePackageValidationReport::new();

        validate_non_empty("package.name", &self.package.name, &mut report);
        validate_non_empty("package.version", &self.package.version, &mut report);
        if self.package.schema_version != CURRENT_SCHEMA_VERSION {
            report.push(RulePackageValidationError::new(
                "package.schema_version",
                RulePackageValidationErrorKind::SchemaVersionMismatch,
                format!(
                    "expected schema version `{CURRENT_SCHEMA_VERSION}`, got `{}`",
                    self.package.schema_version
                ),
            ));
        }

        if let Some(expected) = expected {
            if self.target.kind != expected {
                report.push(RulePackageValidationError::new(
                    "target.kind",
                    RulePackageValidationErrorKind::IncompatibleTargetProfile,
                    format!(
                        "expected target `{}`, got `{}`",
                        expected.as_str(),
                        self.target.kind.as_str()
                    ),
                ));
            }
        }

        validate_sample_timing(&self.sample_timing, &mut report);

        if self.channels.is_empty() {
            report.push(RulePackageValidationError::new(
                "channels",
                RulePackageValidationErrorKind::MissingChannel,
                "at least one channel is required",
            ));
        }

        let mut channel_names = BTreeSet::new();
        for (index, channel) in self.channels.iter().enumerate() {
            let field = format!("channels[{index}]");
            validate_non_empty(&format!("{field}.name"), &channel.name, &mut report);
            if !channel.name.is_empty() && !channel_names.insert(channel.name.clone()) {
                report.push(RulePackageValidationError::new(
                    format!("{field}.name"),
                    RulePackageValidationErrorKind::DuplicateIdentifier,
                    format!("duplicate channel `{}`", channel.name),
                ));
            }
            validate_optional_positive_finite(
                &format!("{field}.sample_rate_hz"),
                channel.sample_rate_hz,
                &mut report,
            );
            validate_thresholds(index, channel, &mut report);
        }

        let mut filter_ids = BTreeSet::new();
        for (index, filter) in self.filters.iter().enumerate() {
            validate_filter(index, filter, &channel_names, &mut filter_ids, &mut report);
        }

        let mut criterion_ids = BTreeSet::new();
        for (index, criterion) in self.criteria.iter().enumerate() {
            validate_criterion(
                index,
                criterion,
                &channel_names,
                &mut criterion_ids,
                &mut report,
            );
        }

        report.into_result()
    }
}

pub fn parse_rule_package_json(input: &str) -> Result<RulePackage, RulePackageValidationError> {
    serde_json::from_str(input).map_err(|error| classify_parse_error("rules.json", &error))
}

pub fn parse_rule_package_toml(input: &str) -> Result<RulePackage, RulePackageValidationError> {
    toml::from_str(input).map_err(|error| classify_parse_error("rules.toml", &error))
}

pub fn checksum_bytes(bytes: &[u8]) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }

    format!("{CHECKSUM_ALGORITHM}:{hash:016x}")
}

pub fn checksum_str(input: &str) -> String {
    checksum_bytes(input.as_bytes())
}

pub fn validate_checksum_match(
    expected: &str,
    actual: &str,
) -> Result<(), RulePackageValidationError> {
    if expected == actual {
        return Ok(());
    }

    Err(RulePackageValidationError::new(
        "checksum",
        RulePackageValidationErrorKind::ChecksumMismatch,
        "expected checksum does not match actual checksum",
    ))
}

pub fn validate_artifact_checksum(
    expected: &str,
    contents: &str,
) -> Result<(), RulePackageValidationError> {
    validate_checksum_match(expected, &checksum_str(contents))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RulePackageManifest {
    pub manifest_version: String,
    pub generated_by: String,
    pub schema_version: String,
    pub package: ManifestPackageMetadata,
    pub target: ManifestTargetProfile,
    pub sources: ManifestSources,
    pub validation: ManifestValidationEvidence,
    pub checksum: ChecksumMetadata,
    pub artifacts: Vec<ManifestArtifact>,
}

impl RulePackageManifest {
    pub fn new(
        package: &RulePackage,
        sources: ManifestSources,
        validation: ManifestValidationEvidence,
        artifacts: Vec<ManifestArtifact>,
    ) -> Self {
        Self {
            manifest_version: CURRENT_MANIFEST_VERSION.to_string(),
            generated_by: "ferrisoxide-signal".to_string(),
            schema_version: package.package.schema_version.clone(),
            package: ManifestPackageMetadata {
                name: package.package.name.clone(),
                version: package.package.version.clone(),
            },
            target: ManifestTargetProfile {
                kind: package.target.kind,
                identifier: package.target.identifier.clone(),
            },
            sources,
            validation,
            checksum: ChecksumMetadata::default(),
            artifacts,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestPackageMetadata {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestTargetProfile {
    pub kind: TargetProfileKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestSources {
    pub input: String,
    pub config: String,
}

impl ManifestSources {
    pub fn new(input: impl Into<String>, config: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            config: config.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestValidationEvidence {
    pub report_artifact: String,
    pub rule_package_validation: String,
}

impl ManifestValidationEvidence {
    pub fn passed(report_artifact: impl Into<String>) -> Self {
        Self {
            report_artifact: report_artifact.into(),
            rule_package_validation: "passed_before_export".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChecksumMetadata {
    pub algorithm: String,
    pub format: String,
    pub scope: String,
    pub security_note: String,
}

impl Default for ChecksumMetadata {
    fn default() -> Self {
        Self {
            algorithm: CHECKSUM_ALGORITHM.to_string(),
            format: CHECKSUM_FORMAT.to_string(),
            scope: CHECKSUM_SCOPE.to_string(),
            security_note: CHECKSUM_SECURITY_NOTE.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestArtifact {
    pub path: String,
    pub role: String,
    pub media_type: String,
    pub checksum: String,
    pub byte_length: usize,
}

impl ManifestArtifact {
    pub fn from_contents(
        path: impl Into<String>,
        role: impl Into<String>,
        media_type: impl Into<String>,
        contents: &str,
    ) -> Self {
        Self {
            path: path.into(),
            role: role.into(),
            media_type: media_type.into(),
            checksum: checksum_str(contents),
            byte_length: contents.len(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RulePackageValidationReport {
    pub errors: Vec<RulePackageValidationError>,
}

impl RulePackageValidationReport {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    fn push(&mut self, error: RulePackageValidationError) {
        self.errors.push(error);
    }

    fn into_result(self) -> Result<(), Self> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl Default for RulePackageValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for RulePackageValidationReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.errors.is_empty() {
            return write!(formatter, "rule package validation passed");
        }

        writeln!(
            formatter,
            "rule package validation failed with {} error(s):",
            self.errors.len()
        )?;
        for error in &self.errors {
            writeln!(formatter, "- {error}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RulePackageValidationError {
    pub field: String,
    pub kind: RulePackageValidationErrorKind,
    pub message: String,
}

impl RulePackageValidationError {
    pub fn new(
        field: impl Into<String>,
        kind: RulePackageValidationErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            kind,
            message: message.into(),
        }
    }
}

impl fmt::Display for RulePackageValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {} ({})",
            self.field,
            self.message,
            self.kind.as_str()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RulePackageValidationErrorKind {
    ParseError,
    MissingChannel,
    UnsupportedUnit,
    UnknownFilter,
    UnknownCriterion,
    InvalidTimingAssumption,
    ChecksumMismatch,
    IncompatibleTargetProfile,
    InvalidPackageMetadata,
    SchemaVersionMismatch,
    DuplicateIdentifier,
    InvalidFilter,
    InvalidCriterion,
    InvalidThreshold,
}

impl RulePackageValidationErrorKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::MissingChannel => "missing_channel",
            Self::UnsupportedUnit => "unsupported_unit",
            Self::UnknownFilter => "unknown_filter",
            Self::UnknownCriterion => "unknown_criterion",
            Self::InvalidTimingAssumption => "invalid_timing_assumption",
            Self::ChecksumMismatch => "checksum_mismatch",
            Self::IncompatibleTargetProfile => "incompatible_target_profile",
            Self::InvalidPackageMetadata => "invalid_package_metadata",
            Self::SchemaVersionMismatch => "schema_version_mismatch",
            Self::DuplicateIdentifier => "duplicate_identifier",
            Self::InvalidFilter => "invalid_filter",
            Self::InvalidCriterion => "invalid_criterion",
            Self::InvalidThreshold => "invalid_threshold",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub schema_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PackageMetadata {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            schema_version: CURRENT_SCHEMA_VERSION.to_string(),
            description: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetProfile {
    pub kind: TargetProfileKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

impl TargetProfile {
    pub fn new(kind: TargetProfileKind) -> Self {
        Self {
            kind,
            identifier: None,
            notes: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetProfileKind {
    DesktopAuthoring,
    EmbeddedRuntime,
    ControllerRuntime,
    TestVerification,
}

impl TargetProfileKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopAuthoring => "desktop_authoring",
            Self::EmbeddedRuntime => "embedded_runtime",
            Self::ControllerRuntime => "controller_runtime",
            Self::TestVerification => "test_verification",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SampleTimingAssumption {
    pub timestamp_unit: EngineeringUnit,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal_sample_rate_hz: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_rate_tolerance_hz: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal_sample_interval_s: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp_tolerance_s: Option<f64>,
}

impl SampleTimingAssumption {
    pub const fn seconds_at_hz(nominal_sample_rate_hz: f64) -> Self {
        Self {
            timestamp_unit: EngineeringUnit::Second,
            nominal_sample_rate_hz: Some(nominal_sample_rate_hz),
            sample_rate_tolerance_hz: None,
            nominal_sample_interval_s: None,
            timestamp_tolerance_s: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelDefinition {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    pub unit: EngineeringUnit,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_rate_hz: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub thresholds: Vec<ThresholdDefinition>,
}

impl ChannelDefinition {
    pub fn new(name: impl Into<String>, unit: EngineeringUnit) -> Self {
        Self {
            name: name.into(),
            source_name: None,
            unit,
            sample_rate_hz: None,
            thresholds: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThresholdDefinition {
    pub name: String,
    pub role: ThresholdRole,
    pub value: UnitValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThresholdRole {
    Low,
    High,
    Decision,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct UnitValue {
    pub value: f64,
    pub unit: EngineeringUnit,
}

impl UnitValue {
    pub const fn new(value: f64, unit: EngineeringUnit) -> Self {
        Self { value, unit }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EngineeringUnit {
    #[serde(rename = "V")]
    Volt,
    #[serde(rename = "s")]
    Second,
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "sample")]
    Sample,
    #[serde(rename = "Hz")]
    Hertz,
}

impl EngineeringUnit {
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Volt => "V",
            Self::Second => "s",
            Self::Count => "count",
            Self::Sample => "sample",
            Self::Hertz => "Hz",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FilterDefinition {
    MovingAverage {
        id: String,
        channel: String,
        window_samples: usize,
    },
    LowPass {
        id: String,
        channel: String,
        cutoff: UnitValue,
    },
    AdcQuantize {
        id: String,
        channel: String,
        bits: u8,
        min: UnitValue,
        max: UnitValue,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CriterionDefinition {
    pub id: String,
    pub channel: String,
    pub measurement: MeasurementDefinition,
    pub requirement: RequirementDefinition,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MeasurementDefinition {
    MinimumSample,
    MaximumSample,
    StateTransitionCount {
        threshold: UnitValue,
    },
    PulseWidth {
        state: SignalState,
        threshold: UnitValue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        selection: Option<RunSelection>,
    },
    StableStateDuration {
        state: SignalState,
        threshold: UnitValue,
    },
    TransientEventDuration {
        event_kind: TransientEventKind,
        expected_state: SignalState,
        threshold: UnitValue,
    },
    ResponseLatency {
        source_channel: String,
        source_threshold: UnitValue,
        target_threshold: UnitValue,
        source_state: SignalState,
        expected_target_state: SignalState,
    },
    RiseTime {
        low_threshold: UnitValue,
        high_threshold: UnitValue,
    },
    FallTime {
        low_threshold: UnitValue,
        high_threshold: UnitValue,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequirementDefinition {
    pub operator: ComparisonOperator,
    pub value: UnitValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    EqualTo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalState {
    High,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunSelection {
    Shortest,
    Longest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransientEventKind {
    TransientEvent,
    SpuriousTransition,
    ContactBounce,
    Dropout,
    NoiseInducedTransition,
    ThresholdCrossingEvent,
}

fn classify_parse_error(
    field: &'static str,
    error: &impl fmt::Display,
) -> RulePackageValidationError {
    let message = error.to_string();
    let kind = if message.contains("unknown variant")
        && message.contains("V")
        && message.contains("Hz")
        && message.contains("count")
    {
        RulePackageValidationErrorKind::UnsupportedUnit
    } else if message.contains("unknown variant")
        && message.contains("moving_average")
        && message.contains("adc_quantize")
    {
        RulePackageValidationErrorKind::UnknownFilter
    } else if message.contains("unknown variant")
        && message.contains("minimum_sample")
        && message.contains("transient_event_duration")
    {
        RulePackageValidationErrorKind::UnknownCriterion
    } else {
        RulePackageValidationErrorKind::ParseError
    };

    RulePackageValidationError::new(field, kind, message)
}

fn validate_non_empty(field: &str, value: &str, report: &mut RulePackageValidationReport) {
    if value.trim().is_empty() {
        report.push(RulePackageValidationError::new(
            field,
            RulePackageValidationErrorKind::InvalidPackageMetadata,
            "value must not be empty",
        ));
    }
}

fn validate_sample_timing(
    timing: &SampleTimingAssumption,
    report: &mut RulePackageValidationReport,
) {
    if timing.timestamp_unit != EngineeringUnit::Second {
        report.push(RulePackageValidationError::new(
            "sample_timing.timestamp_unit",
            RulePackageValidationErrorKind::InvalidTimingAssumption,
            "timestamp unit must be `s`",
        ));
    }

    validate_optional_positive_finite(
        "sample_timing.nominal_sample_rate_hz",
        timing.nominal_sample_rate_hz,
        report,
    );
    validate_optional_non_negative_finite(
        "sample_timing.sample_rate_tolerance_hz",
        timing.sample_rate_tolerance_hz,
        RulePackageValidationErrorKind::InvalidTimingAssumption,
        report,
    );
    validate_optional_positive_finite(
        "sample_timing.nominal_sample_interval_s",
        timing.nominal_sample_interval_s,
        report,
    );
    validate_optional_non_negative_finite(
        "sample_timing.timestamp_tolerance_s",
        timing.timestamp_tolerance_s,
        RulePackageValidationErrorKind::InvalidTimingAssumption,
        report,
    );

    if let (Some(rate), Some(interval)) = (
        timing.nominal_sample_rate_hz,
        timing.nominal_sample_interval_s,
    ) {
        if rate.is_finite() && rate > 0.0 && interval.is_finite() && interval > 0.0 {
            let expected_interval = 1.0 / rate;
            if (interval - expected_interval).abs() > expected_interval * 0.001 {
                report.push(RulePackageValidationError::new(
                    "sample_timing.nominal_sample_interval_s",
                    RulePackageValidationErrorKind::InvalidTimingAssumption,
                    "sample interval must match nominal sample rate within 0.1%",
                ));
            }
        }
    }
}

fn validate_thresholds(
    channel_index: usize,
    channel: &ChannelDefinition,
    report: &mut RulePackageValidationReport,
) {
    let mut names = BTreeSet::new();
    for (index, threshold) in channel.thresholds.iter().enumerate() {
        let field = format!("channels[{channel_index}].thresholds[{index}]");
        validate_non_empty(&format!("{field}.name"), &threshold.name, report);
        if !threshold.name.is_empty() && !names.insert(threshold.name.as_str()) {
            report.push(RulePackageValidationError::new(
                format!("{field}.name"),
                RulePackageValidationErrorKind::DuplicateIdentifier,
                format!("duplicate threshold `{}`", threshold.name),
            ));
        }
        validate_unit_value(
            &format!("{field}.value"),
            threshold.value,
            Some(channel.unit),
            RulePackageValidationErrorKind::InvalidThreshold,
            report,
        );
    }
}

fn validate_filter(
    index: usize,
    filter: &FilterDefinition,
    channel_names: &BTreeSet<String>,
    filter_ids: &mut BTreeSet<String>,
    report: &mut RulePackageValidationReport,
) {
    let field = format!("filters[{index}]");
    let id = filter.id();
    validate_non_empty(&format!("{field}.id"), id, report);
    if !id.is_empty() && !filter_ids.insert(id.to_string()) {
        report.push(RulePackageValidationError::new(
            format!("{field}.id"),
            RulePackageValidationErrorKind::DuplicateIdentifier,
            format!("duplicate filter `{id}`"),
        ));
    }
    validate_channel_reference(
        &format!("{field}.channel"),
        filter.channel(),
        channel_names,
        report,
    );

    match filter {
        FilterDefinition::MovingAverage { window_samples, .. } => {
            if *window_samples == 0 {
                report.push(RulePackageValidationError::new(
                    format!("{field}.window_samples"),
                    RulePackageValidationErrorKind::InvalidFilter,
                    "window_samples must be greater than zero",
                ));
            }
        }
        FilterDefinition::LowPass { cutoff, .. } => validate_unit_value(
            &format!("{field}.cutoff"),
            *cutoff,
            Some(EngineeringUnit::Hertz),
            RulePackageValidationErrorKind::InvalidFilter,
            report,
        ),
        FilterDefinition::AdcQuantize { bits, min, max, .. } => {
            if *bits == 0 || *bits > 32 {
                report.push(RulePackageValidationError::new(
                    format!("{field}.bits"),
                    RulePackageValidationErrorKind::InvalidFilter,
                    "bits must be between 1 and 32",
                ));
            }
            validate_unit_value(
                &format!("{field}.min"),
                *min,
                Some(EngineeringUnit::Volt),
                RulePackageValidationErrorKind::InvalidFilter,
                report,
            );
            validate_unit_value(
                &format!("{field}.max"),
                *max,
                Some(EngineeringUnit::Volt),
                RulePackageValidationErrorKind::InvalidFilter,
                report,
            );
            if min.unit == max.unit && min.value >= max.value {
                report.push(RulePackageValidationError::new(
                    format!("{field}.max"),
                    RulePackageValidationErrorKind::InvalidFilter,
                    "max must be greater than min",
                ));
            }
        }
    }
}

fn validate_criterion(
    index: usize,
    criterion: &CriterionDefinition,
    channel_names: &BTreeSet<String>,
    criterion_ids: &mut BTreeSet<String>,
    report: &mut RulePackageValidationReport,
) {
    let field = format!("criteria[{index}]");
    validate_non_empty(&format!("{field}.id"), &criterion.id, report);
    if !criterion.id.is_empty() && !criterion_ids.insert(criterion.id.clone()) {
        report.push(RulePackageValidationError::new(
            format!("{field}.id"),
            RulePackageValidationErrorKind::DuplicateIdentifier,
            format!("duplicate criterion `{}`", criterion.id),
        ));
    }
    validate_channel_reference(
        &format!("{field}.channel"),
        &criterion.channel,
        channel_names,
        report,
    );
    validate_measurement(
        &field,
        &criterion.measurement,
        &criterion.requirement,
        channel_names,
        report,
    );
}

fn validate_measurement(
    field: &str,
    measurement: &MeasurementDefinition,
    requirement: &RequirementDefinition,
    channel_names: &BTreeSet<String>,
    report: &mut RulePackageValidationReport,
) {
    match measurement {
        MeasurementDefinition::MinimumSample | MeasurementDefinition::MaximumSample => {
            validate_requirement(
                &format!("{field}.requirement"),
                requirement,
                EngineeringUnit::Volt,
                report,
            );
        }
        MeasurementDefinition::StateTransitionCount { threshold } => {
            validate_unit_value(
                &format!("{field}.measurement.threshold"),
                *threshold,
                Some(EngineeringUnit::Volt),
                RulePackageValidationErrorKind::InvalidCriterion,
                report,
            );
            validate_requirement(
                &format!("{field}.requirement"),
                requirement,
                EngineeringUnit::Count,
                report,
            );
        }
        MeasurementDefinition::PulseWidth {
            threshold,
            selection,
            ..
        } => {
            validate_unit_value(
                &format!("{field}.measurement.threshold"),
                *threshold,
                Some(EngineeringUnit::Volt),
                RulePackageValidationErrorKind::InvalidCriterion,
                report,
            );
            validate_requirement(
                &format!("{field}.requirement"),
                requirement,
                EngineeringUnit::Second,
                report,
            );
            if requirement.operator == ComparisonOperator::EqualTo && selection.is_none() {
                report.push(RulePackageValidationError::new(
                    format!("{field}.measurement.selection"),
                    RulePackageValidationErrorKind::InvalidCriterion,
                    "equal_to pulse_width criteria must select shortest or longest",
                ));
            }
        }
        MeasurementDefinition::StableStateDuration { threshold, .. }
        | MeasurementDefinition::TransientEventDuration { threshold, .. } => {
            validate_unit_value(
                &format!("{field}.measurement.threshold"),
                *threshold,
                Some(EngineeringUnit::Volt),
                RulePackageValidationErrorKind::InvalidCriterion,
                report,
            );
            validate_requirement(
                &format!("{field}.requirement"),
                requirement,
                EngineeringUnit::Second,
                report,
            );
        }
        MeasurementDefinition::ResponseLatency {
            source_channel,
            source_threshold,
            target_threshold,
            ..
        } => {
            validate_channel_reference(
                &format!("{field}.measurement.source_channel"),
                source_channel,
                channel_names,
                report,
            );
            validate_unit_value(
                &format!("{field}.measurement.source_threshold"),
                *source_threshold,
                Some(EngineeringUnit::Volt),
                RulePackageValidationErrorKind::InvalidCriterion,
                report,
            );
            validate_unit_value(
                &format!("{field}.measurement.target_threshold"),
                *target_threshold,
                Some(EngineeringUnit::Volt),
                RulePackageValidationErrorKind::InvalidCriterion,
                report,
            );
            validate_requirement(
                &format!("{field}.requirement"),
                requirement,
                EngineeringUnit::Second,
                report,
            );
        }
        MeasurementDefinition::RiseTime {
            low_threshold,
            high_threshold,
        }
        | MeasurementDefinition::FallTime {
            low_threshold,
            high_threshold,
        } => {
            validate_unit_value(
                &format!("{field}.measurement.low_threshold"),
                *low_threshold,
                Some(EngineeringUnit::Volt),
                RulePackageValidationErrorKind::InvalidCriterion,
                report,
            );
            validate_unit_value(
                &format!("{field}.measurement.high_threshold"),
                *high_threshold,
                Some(EngineeringUnit::Volt),
                RulePackageValidationErrorKind::InvalidCriterion,
                report,
            );
            if low_threshold.unit == high_threshold.unit
                && low_threshold.value >= high_threshold.value
            {
                report.push(RulePackageValidationError::new(
                    format!("{field}.measurement.high_threshold"),
                    RulePackageValidationErrorKind::InvalidCriterion,
                    "high_threshold must be greater than low_threshold",
                ));
            }
            validate_requirement(
                &format!("{field}.requirement"),
                requirement,
                EngineeringUnit::Second,
                report,
            );
        }
    }
}

fn validate_requirement(
    field: &str,
    requirement: &RequirementDefinition,
    expected_unit: EngineeringUnit,
    report: &mut RulePackageValidationReport,
) {
    validate_unit_value(
        &format!("{field}.value"),
        requirement.value,
        Some(expected_unit),
        RulePackageValidationErrorKind::InvalidCriterion,
        report,
    );

    if matches!(
        requirement.value.unit,
        EngineeringUnit::Count | EngineeringUnit::Sample
    ) && requirement.value.value.fract() != 0.0
    {
        report.push(RulePackageValidationError::new(
            format!("{field}.value"),
            RulePackageValidationErrorKind::InvalidCriterion,
            "count and sample requirements must be whole numbers",
        ));
    }

    if requirement.value.value < 0.0 {
        report.push(RulePackageValidationError::new(
            format!("{field}.value"),
            RulePackageValidationErrorKind::InvalidCriterion,
            "requirement value must be non-negative",
        ));
    }
}

fn validate_channel_reference(
    field: &str,
    channel: &str,
    channel_names: &BTreeSet<String>,
    report: &mut RulePackageValidationReport,
) {
    validate_non_empty(field, channel, report);
    if !channel.is_empty() && !channel_names.contains(channel) {
        report.push(RulePackageValidationError::new(
            field,
            RulePackageValidationErrorKind::MissingChannel,
            format!("channel `{channel}` is not defined"),
        ));
    }
}

fn validate_optional_positive_finite(
    field: &str,
    value: Option<f64>,
    report: &mut RulePackageValidationReport,
) {
    let Some(value) = value else {
        return;
    };

    if !value.is_finite() || value <= 0.0 {
        report.push(RulePackageValidationError::new(
            field,
            RulePackageValidationErrorKind::InvalidTimingAssumption,
            "value must be finite and greater than zero",
        ));
    }
}

fn validate_optional_non_negative_finite(
    field: &str,
    value: Option<f64>,
    kind: RulePackageValidationErrorKind,
    report: &mut RulePackageValidationReport,
) {
    let Some(value) = value else {
        return;
    };

    if !value.is_finite() || value < 0.0 {
        report.push(RulePackageValidationError::new(
            field,
            kind,
            "value must be finite and non-negative",
        ));
    }
}

fn validate_unit_value(
    field: &str,
    unit_value: UnitValue,
    expected_unit: Option<EngineeringUnit>,
    kind: RulePackageValidationErrorKind,
    report: &mut RulePackageValidationReport,
) {
    if !unit_value.value.is_finite() {
        report.push(RulePackageValidationError::new(
            field,
            kind,
            "value must be finite",
        ));
    }

    if let Some(expected_unit) = expected_unit {
        if unit_value.unit != expected_unit {
            report.push(RulePackageValidationError::new(
                format!("{field}.unit"),
                kind,
                format!(
                    "expected unit `{}`, got `{}`",
                    expected_unit.symbol(),
                    unit_value.unit.symbol()
                ),
            ));
        }
    }
}

impl FilterDefinition {
    fn id(&self) -> &str {
        match self {
            Self::MovingAverage { id, .. }
            | Self::LowPass { id, .. }
            | Self::AdcQuantize { id, .. } => id,
        }
    }

    fn channel(&self) -> &str {
        match self {
            Self::MovingAverage { channel, .. }
            | Self::LowPass { channel, .. }
            | Self::AdcQuantize { channel, .. } => channel,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_portable_rule_package_schema() {
        let package = RulePackage {
            package: PackageMetadata::new("switch-rule", "1.0.0"),
            target: TargetProfile {
                kind: TargetProfileKind::ControllerRuntime,
                identifier: Some("raspberry-pi-5-bare-metal".to_string()),
                notes: Vec::new(),
            },
            sample_timing: SampleTimingAssumption {
                timestamp_unit: EngineeringUnit::Second,
                nominal_sample_rate_hz: Some(10_000.0),
                sample_rate_tolerance_hz: Some(1.0),
                nominal_sample_interval_s: Some(0.0001),
                timestamp_tolerance_s: Some(0.000_001),
            },
            channels: vec![ChannelDefinition {
                name: "switch_v".to_string(),
                source_name: Some("daq_ai0".to_string()),
                unit: EngineeringUnit::Volt,
                sample_rate_hz: Some(10_000.0),
                thresholds: vec![
                    ThresholdDefinition {
                        name: "switch_low".to_string(),
                        role: ThresholdRole::Low,
                        value: UnitValue::new(0.5, EngineeringUnit::Volt),
                    },
                    ThresholdDefinition {
                        name: "switch_high".to_string(),
                        role: ThresholdRole::High,
                        value: UnitValue::new(4.5, EngineeringUnit::Volt),
                    },
                ],
            }],
            filters: vec![
                FilterDefinition::MovingAverage {
                    id: "filter_switch_average".to_string(),
                    channel: "switch_v".to_string(),
                    window_samples: 5,
                },
                FilterDefinition::LowPass {
                    id: "filter_switch_low_pass".to_string(),
                    channel: "switch_v".to_string(),
                    cutoff: UnitValue::new(250.0, EngineeringUnit::Hertz),
                },
                FilterDefinition::AdcQuantize {
                    id: "quantize_switch".to_string(),
                    channel: "switch_v".to_string(),
                    bits: 12,
                    min: UnitValue::new(0.0, EngineeringUnit::Volt),
                    max: UnitValue::new(5.0, EngineeringUnit::Volt),
                },
            ],
            criteria: vec![CriterionDefinition {
                id: "no_dropout_longer_than_1ms".to_string(),
                channel: "switch_v".to_string(),
                measurement: MeasurementDefinition::TransientEventDuration {
                    event_kind: TransientEventKind::Dropout,
                    expected_state: SignalState::High,
                    threshold: UnitValue::new(2.5, EngineeringUnit::Volt),
                },
                requirement: RequirementDefinition {
                    operator: ComparisonOperator::LessThanOrEqual,
                    value: UnitValue::new(0.001, EngineeringUnit::Second),
                },
            }],
        };

        assert_eq!(package.package.schema_version, CURRENT_SCHEMA_VERSION);
        assert_eq!(package.channels[0].unit.symbol(), "V");
        assert_eq!(package.channels[0].thresholds.len(), 2);
        assert_eq!(package.sample_timing.nominal_sample_rate_hz, Some(10_000.0));
        assert_eq!(package.criteria[0].channel, "switch_v");
        assert_eq!(
            package.criteria[0].requirement.value.unit,
            EngineeringUnit::Second
        );
    }

    #[test]
    fn serializes_and_deserializes_explicit_units() {
        let package = RulePackage::new(
            PackageMetadata::new("minimal", "0.1.0"),
            TargetProfile::new(TargetProfileKind::EmbeddedRuntime),
            SampleTimingAssumption::seconds_at_hz(1_000.0),
            vec![ChannelDefinition::new("control_v", EngineeringUnit::Volt)],
            vec![CriterionDefinition {
                id: "max_control".to_string(),
                channel: "control_v".to_string(),
                measurement: MeasurementDefinition::MaximumSample,
                requirement: RequirementDefinition {
                    operator: ComparisonOperator::LessThanOrEqual,
                    value: UnitValue::new(5.0, EngineeringUnit::Volt),
                },
            }],
        );

        let json = serde_json::to_string(&package).expect("schema should serialize");
        assert!(json.contains("\"schema_version\":\"0.1.0\""));
        assert!(json.contains("\"kind\":\"embedded_runtime\""));
        assert!(json.contains("\"unit\":\"V\""));
        assert!(json.contains("\"maximum_sample\""));
        assert!(!json.contains("csv"));
        assert!(!json.contains("plot"));
        assert!(!json.contains("hardware"));

        let round_trip: RulePackage =
            serde_json::from_str(&json).expect("schema should deserialize");
        assert_eq!(round_trip, package);
    }

    #[test]
    fn examples_rules_toml_and_json_describe_same_package() {
        let rules_toml = include_str!("../../../examples/rule-package/rules.toml");
        let rules_json = include_str!("../../../examples/rule-package/rules.json");

        let toml_package =
            parse_rule_package_toml(rules_toml).expect("rules.toml should match the schema");
        let json_package =
            parse_rule_package_json(rules_json).expect("rules.json should match the schema");

        assert_eq!(toml_package, json_package);
        assert_eq!(toml_package.package.schema_version, CURRENT_SCHEMA_VERSION);
        assert_eq!(toml_package.channels[0].name, "switch_signal");
        assert_eq!(toml_package.channels[0].thresholds.len(), 3);
        assert_eq!(toml_package.filters.len(), 2);
        assert_eq!(toml_package.criteria.len(), 3);
        assert_eq!(toml_package.validate(), Ok(()));
    }

    #[test]
    fn validates_accepted_package_for_expected_target() {
        let package = example_package();

        assert_eq!(package.validate(), Ok(()));
        assert_eq!(
            package.validate_for_target(TargetProfileKind::ControllerRuntime),
            Ok(())
        );
    }

    #[test]
    fn validates_response_latency_measurement_source_channel() {
        let mut package = RulePackage::new(
            PackageMetadata::new("heated-actuator", "0.1.0"),
            TargetProfile::new(TargetProfileKind::ControllerRuntime),
            SampleTimingAssumption::seconds_at_hz(1_000.0),
            vec![
                ChannelDefinition::new("command_v", EngineeringUnit::Volt),
                ChannelDefinition::new("feedback_v", EngineeringUnit::Volt),
            ],
            vec![CriterionDefinition {
                id: "response_latency".to_string(),
                channel: "feedback_v".to_string(),
                measurement: MeasurementDefinition::ResponseLatency {
                    source_channel: "command_v".to_string(),
                    source_threshold: UnitValue::new(2.5, EngineeringUnit::Volt),
                    target_threshold: UnitValue::new(2.5, EngineeringUnit::Volt),
                    source_state: SignalState::High,
                    expected_target_state: SignalState::High,
                },
                requirement: RequirementDefinition {
                    operator: ComparisonOperator::LessThanOrEqual,
                    value: UnitValue::new(0.050, EngineeringUnit::Second),
                },
            }],
        );

        assert_eq!(package.validate(), Ok(()));

        if let MeasurementDefinition::ResponseLatency { source_channel, .. } =
            &mut package.criteria[0].measurement
        {
            *source_channel = "missing_command".to_string();
        }
        let report = package
            .validate()
            .expect_err("missing response source channel should be rejected");

        assert!(report.errors.iter().any(|error| {
            error.kind == RulePackageValidationErrorKind::MissingChannel
                && error.field == "criteria[0].measurement.source_channel"
        }));
    }

    #[test]
    fn rejects_missing_channel_references() {
        let mut package = example_package();
        package.criteria[0].channel = "missing_signal".to_string();

        let report = package
            .validate()
            .expect_err("missing channel should be rejected");

        assert!(report.errors.iter().any(|error| {
            error.kind == RulePackageValidationErrorKind::MissingChannel
                && error.field == "criteria[0].channel"
        }));
    }

    #[test]
    fn classifies_unsupported_unit_during_parse() {
        let invalid = include_str!("../../../examples/rule-package/rules.toml")
            .replace("unit = \"V\"", "unit = \"mV\"");

        let error = parse_rule_package_toml(&invalid).expect_err("unsupported unit should fail");

        assert_eq!(error.kind, RulePackageValidationErrorKind::UnsupportedUnit);
    }

    #[test]
    fn classifies_unknown_filter_during_parse() {
        let invalid = include_str!("../../../examples/rule-package/rules.toml")
            .replace("type = \"moving_average\"", "type = \"median\"");

        let error = parse_rule_package_toml(&invalid).expect_err("unknown filter should fail");

        assert_eq!(error.kind, RulePackageValidationErrorKind::UnknownFilter);
    }

    #[test]
    fn classifies_unknown_criterion_measurement_during_parse() {
        let invalid = include_str!("../../../examples/rule-package/rules.toml").replace(
            "type = \"transient_event_duration\"",
            "type = \"glitch_duration\"",
        );

        let error = parse_rule_package_toml(&invalid).expect_err("unknown criterion should fail");

        assert_eq!(error.kind, RulePackageValidationErrorKind::UnknownCriterion);
    }

    #[test]
    fn rejects_invalid_timing_assumptions() {
        let mut package = example_package();
        package.sample_timing.nominal_sample_rate_hz = Some(0.0);
        package.sample_timing.sample_rate_tolerance_hz = Some(-1.0);

        let report = package
            .validate()
            .expect_err("invalid timing should be rejected");

        assert!(report.errors.iter().any(|error| {
            error.kind == RulePackageValidationErrorKind::InvalidTimingAssumption
                && error.field == "sample_timing.nominal_sample_rate_hz"
        }));
        assert!(report.errors.iter().any(|error| {
            error.kind == RulePackageValidationErrorKind::InvalidTimingAssumption
                && error.field == "sample_timing.sample_rate_tolerance_hz"
        }));
    }

    #[test]
    fn rejects_checksum_mismatch() {
        let error = validate_checksum_match("abc123", "def456")
            .expect_err("checksum mismatch should be structured");

        assert_eq!(error.kind, RulePackageValidationErrorKind::ChecksumMismatch);
        assert_eq!(error.field, "checksum");
    }

    #[test]
    fn produces_deterministic_artifact_checksums() {
        assert_eq!(
            checksum_str("FerrisOxide Signal\n"),
            "fnv1a64:427dde5372ab059f"
        );
        assert_eq!(
            checksum_str("FerrisOxide Signal\n"),
            checksum_bytes(b"FerrisOxide Signal\n")
        );
    }

    #[test]
    fn validates_artifact_checksum_with_clear_mismatch_error() {
        let expected = checksum_str("rules.toml contents\n");

        assert_eq!(
            validate_artifact_checksum(&expected, "rules.toml contents\n"),
            Ok(())
        );

        let error = validate_artifact_checksum(&expected, "changed rules.toml contents\n")
            .expect_err("artifact checksum mismatch should be structured");

        assert_eq!(error.kind, RulePackageValidationErrorKind::ChecksumMismatch);
        assert_eq!(error.field, "checksum");
        assert!(error.message.contains("expected checksum"));
    }

    #[test]
    fn builds_manifest_with_artifact_metadata() {
        let package = example_package();
        let manifest = RulePackageManifest::new(
            &package,
            ManifestSources::new("waveform.csv", "rules-config.toml"),
            ManifestValidationEvidence::passed("validation-report.json"),
            vec![ManifestArtifact::from_contents(
                "rules.toml",
                "rule_package_toml",
                "application/toml",
                "rules",
            )],
        );

        assert_eq!(manifest.manifest_version, CURRENT_MANIFEST_VERSION);
        assert_eq!(manifest.schema_version, CURRENT_SCHEMA_VERSION);
        assert_eq!(manifest.package.name, package.package.name);
        assert_eq!(manifest.target.kind, TargetProfileKind::ControllerRuntime);
        assert_eq!(manifest.sources.config, "rules-config.toml");
        assert_eq!(
            manifest.validation.rule_package_validation,
            "passed_before_export"
        );
        assert_eq!(manifest.checksum.algorithm, CHECKSUM_ALGORITHM);
        assert_eq!(manifest.artifacts[0].path, "rules.toml");
        assert_eq!(manifest.artifacts[0].byte_length, 5);
        assert_eq!(manifest.artifacts[0].checksum, checksum_str("rules"));
    }

    #[test]
    fn rejects_incompatible_target_profile() {
        let package = example_package();

        let report = package
            .validate_for_target(TargetProfileKind::EmbeddedRuntime)
            .expect_err("target mismatch should be rejected");

        assert!(report.errors.iter().any(|error| {
            error.kind == RulePackageValidationErrorKind::IncompatibleTargetProfile
                && error.field == "target.kind"
        }));
    }

    #[test]
    fn rejects_invalid_filter_and_criterion_parameters() {
        let mut package = example_package();
        package.filters[0] = FilterDefinition::MovingAverage {
            id: "switch_moving_average".to_string(),
            channel: "switch_signal".to_string(),
            window_samples: 0,
        };
        package.criteria[2].requirement.value = UnitValue::new(2.5, EngineeringUnit::Count);

        let report = package
            .validate()
            .expect_err("invalid filter and criterion should fail");

        assert!(report.errors.iter().any(|error| {
            error.kind == RulePackageValidationErrorKind::InvalidFilter
                && error.field == "filters[0].window_samples"
        }));
        assert!(report.errors.iter().any(|error| {
            error.kind == RulePackageValidationErrorKind::InvalidCriterion
                && error.field == "criteria[2].requirement.value"
        }));
    }

    fn example_package() -> RulePackage {
        parse_rule_package_toml(include_str!("../../../examples/rule-package/rules.toml"))
            .expect("example package should parse")
    }
}
