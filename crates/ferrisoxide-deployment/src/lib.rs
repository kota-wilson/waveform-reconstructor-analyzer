//! Deployment package manifest format for controller/runtime workflows.
//!
//! This crate defines reviewable package metadata only. It does not export
//! packages, sign artifacts, load RTOS configs, bind HALs, talk to hardware, or
//! claim qualification/certification status.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

pub const CURRENT_DEPLOYMENT_FORMAT_VERSION: &str = "0.1.0";
pub const CURRENT_QUALIFICATION_EVIDENCE_REPORT_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeploymentPackageManifest {
    pub manifest_version: String,
    pub package: DeploymentPackageMetadata,
    pub target: DeploymentTarget,
    pub mode_profiles: Vec<DeploymentModeProfile>,
    pub generated_at: String,
    pub artifacts: Vec<DeploymentArtifact>,
    pub integrity: DeploymentIntegrity,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scope_notes: Vec<String>,
}

impl DeploymentPackageManifest {
    pub fn validate(&self) -> Result<(), DeploymentValidationReport> {
        let mut report = DeploymentValidationReport::new();

        validate_non_empty("manifest_version", &self.manifest_version, &mut report);
        if self.manifest_version != CURRENT_DEPLOYMENT_FORMAT_VERSION {
            report.push(DeploymentValidationError::new(
                "manifest_version",
                DeploymentValidationErrorKind::FormatVersionMismatch,
                format!(
                    "expected deployment format version `{CURRENT_DEPLOYMENT_FORMAT_VERSION}`, got `{}`",
                    self.manifest_version
                ),
            ));
        }
        validate_non_empty("package.name", &self.package.name, &mut report);
        validate_non_empty("package.version", &self.package.version, &mut report);
        validate_non_empty(
            "package.format_version",
            &self.package.format_version,
            &mut report,
        );
        if self.package.format_version != CURRENT_DEPLOYMENT_FORMAT_VERSION {
            report.push(DeploymentValidationError::new(
                "package.format_version",
                DeploymentValidationErrorKind::FormatVersionMismatch,
                format!(
                    "expected deployment package format version `{CURRENT_DEPLOYMENT_FORMAT_VERSION}`, got `{}`",
                    self.package.format_version
                ),
            ));
        }
        validate_non_empty("target.identifier", &self.target.identifier, &mut report);
        validate_non_empty("generated_at", &self.generated_at, &mut report);
        validate_non_empty(
            "integrity.checksum_file",
            &self.integrity.checksum_file,
            &mut report,
        );
        validate_non_empty(
            "integrity.algorithm",
            &self.integrity.algorithm,
            &mut report,
        );
        validate_non_empty("integrity.scope", &self.integrity.scope, &mut report);
        validate_non_empty(
            "integrity.security_note",
            &self.integrity.security_note,
            &mut report,
        );

        validate_artifacts(&self.artifacts, &self.integrity, &mut report);
        validate_mode_profiles(&self.mode_profiles, &self.artifacts, &mut report);

        report.into_result()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentPackageMetadata {
    pub name: String,
    pub version: String,
    pub format_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl DeploymentPackageMetadata {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            format_version: CURRENT_DEPLOYMENT_FORMAT_VERSION.to_string(),
            description: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentTarget {
    pub kind: DeploymentTargetKind,
    pub identifier: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentTargetKind {
    DesktopAuthoring,
    ControllerRuntime,
    EmbeddedRuntime,
    TestStand,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentModeProfile {
    pub id: String,
    pub purpose: DeploymentModePurpose,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control_mode: Option<String>,
    pub uses_artifacts: Vec<DeploymentArtifactRole>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentModePurpose {
    ProductionControl,
    TestVerification,
    SignalValidation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentArtifact {
    pub path: String,
    pub role: DeploymentArtifactRole,
    pub media_type: String,
    pub required: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl DeploymentArtifact {
    pub fn required(
        path: impl Into<String>,
        role: DeploymentArtifactRole,
        media_type: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            role,
            media_type: media_type.into(),
            required: true,
            description: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentArtifactRole {
    ProductionControlConfig,
    TestVerificationConfig,
    ChannelMap,
    PackageManifest,
    ChecksumIndex,
    QualificationReport,
    QualificationEvidenceSvg,
    GeneratedAt,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentIntegrity {
    pub checksum_file: String,
    pub algorithm: String,
    pub scope: String,
    pub security_note: String,
}

impl DeploymentIntegrity {
    pub fn drift_detection_only(checksum_file: impl Into<String>) -> Self {
        Self {
            checksum_file: checksum_file.into(),
            algorithm: "fnv1a64".to_string(),
            scope: "artifact drift detection only".to_string(),
            security_note:
                "non-cryptographic integrity index; not signing, authentication, certification, or tamper-proofing"
                    .to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualificationEvidenceReport {
    pub report_version: String,
    pub generated_at: String,
    pub overall_outcome: QualificationOutcome,
    pub qualification: QualificationRunMetadata,
    pub production_control_config: QualificationVersionedArtifactLink,
    pub test_verification_config: QualificationVersionedArtifactLink,
    pub channel_map: QualificationArtifactLink,
    pub simulation_trace: QualificationSimulationTrace,
    pub criteria_evidence: Vec<QualificationCriterionEvidence>,
    pub deployment_package: QualificationDeploymentPackageEvidence,
    pub checksum_evidence: QualificationChecksumEvidence,
    pub report_artifact: QualificationArtifactLink,
    pub visual_evidence: QualificationArtifactLink,
    pub generated_at_artifact: QualificationArtifactLink,
    pub scope_notes: Vec<String>,
}

impl QualificationEvidenceReport {
    pub fn validate(&self) -> Result<(), QualificationEvidenceValidationReport> {
        let mut report = QualificationEvidenceValidationReport::new();

        validate_qualification_non_empty("report_version", &self.report_version, &mut report);
        if self.report_version != CURRENT_QUALIFICATION_EVIDENCE_REPORT_VERSION {
            report.push(QualificationEvidenceValidationError::new(
                "report_version",
                QualificationEvidenceValidationErrorKind::FormatVersionMismatch,
                format!(
                    "expected qualification evidence report version `{CURRENT_QUALIFICATION_EVIDENCE_REPORT_VERSION}`, got `{}`",
                    self.report_version
                ),
            ));
        }
        validate_qualification_non_empty("generated_at", &self.generated_at, &mut report);
        validate_run_metadata(&self.qualification, &mut report);
        validate_versioned_artifact_link(
            "production_control_config",
            &self.production_control_config,
            &mut report,
        );
        validate_versioned_artifact_link(
            "test_verification_config",
            &self.test_verification_config,
            &mut report,
        );
        validate_artifact_link("channel_map", &self.channel_map, &mut report);
        validate_simulation_trace(&self.simulation_trace, &mut report);
        validate_criteria_evidence(&self.criteria_evidence, &mut report);
        validate_deployment_package_evidence(&self.deployment_package, &mut report);
        validate_checksum_evidence(self, &mut report);
        validate_artifact_link("report_artifact", &self.report_artifact, &mut report);
        validate_artifact_link("visual_evidence", &self.visual_evidence, &mut report);
        validate_artifact_link(
            "generated_at_artifact",
            &self.generated_at_artifact,
            &mut report,
        );
        validate_scope_notes(&self.scope_notes, &mut report);

        report.into_result()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationRunMetadata {
    pub case_id: String,
    pub workflow: String,
    pub input_waveform: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationVersionedArtifactLink {
    pub name: String,
    pub version: String,
    pub path: String,
    pub checksum: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationArtifactLink {
    pub path: String,
    pub checksum: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualificationSimulationTrace {
    pub engine: String,
    pub package_name: String,
    pub package_version: String,
    pub initial_mode: String,
    pub frames: Vec<QualificationStateTraceFrame>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualificationStateTraceFrame {
    pub sample_index: usize,
    pub timestamp_s: f64,
    pub mode: String,
    pub state_machines: Vec<QualificationStateMachineState>,
    pub transitions: Vec<QualificationTransitionEvidence>,
    pub outputs: Vec<QualificationOutputEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationStateMachineState {
    pub machine: String,
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationTransitionEvidence {
    pub transition: String,
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationOutputEvidence {
    pub output: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualificationCriterionEvidence {
    pub criterion_id: String,
    pub outcome: QualificationOutcome,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_criterion: Option<String>,
    pub measurement_id: String,
    pub method: String,
    pub channel: String,
    pub measured_value: f64,
    pub required_value: f64,
    pub tolerance_used: f64,
    pub sample_index: usize,
    pub timestamp_s: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationOutcome {
    Pass,
    Fail,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationDeploymentPackageEvidence {
    pub name: String,
    pub version: String,
    pub format_version: String,
    pub manifest_path: String,
    pub target_kind: DeploymentTargetKind,
    pub target_identifier: String,
    pub generated_at: String,
    pub modes: Vec<QualificationModeEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationModeEvidence {
    pub id: String,
    pub purpose: DeploymentModePurpose,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationChecksumEvidence {
    pub checksum_file: String,
    pub algorithm: String,
    pub scope: String,
    pub security_note: String,
    pub entries: Vec<QualificationChecksumEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationChecksumEntry {
    pub role: DeploymentArtifactRole,
    pub path: String,
    pub checksum: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualificationEvidenceValidationReport {
    pub errors: Vec<QualificationEvidenceValidationError>,
}

impl QualificationEvidenceValidationReport {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    fn push(&mut self, error: QualificationEvidenceValidationError) {
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

impl Default for QualificationEvidenceValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for QualificationEvidenceValidationReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.errors.is_empty() {
            return write!(formatter, "qualification evidence report validation passed");
        }

        writeln!(
            formatter,
            "qualification evidence report validation failed with {} error(s):",
            self.errors.len()
        )?;
        for error in &self.errors {
            writeln!(formatter, "- {error}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualificationEvidenceValidationError {
    pub field: String,
    pub kind: QualificationEvidenceValidationErrorKind,
    pub message: String,
}

impl QualificationEvidenceValidationError {
    pub fn new(
        field: impl Into<String>,
        kind: QualificationEvidenceValidationErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            kind,
            message: message.into(),
        }
    }
}

impl fmt::Display for QualificationEvidenceValidationError {
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
pub enum QualificationEvidenceValidationErrorKind {
    EmptyField,
    FormatVersionMismatch,
    EmptySimulationTrace,
    EmptyCriteriaEvidence,
    EmptyModeEvidence,
    EmptyChecksumEvidence,
    MissingChecksumEntry,
    MissingNonCertificationScopeNote,
    InvalidEvidenceValue,
}

impl QualificationEvidenceValidationErrorKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyField => "empty_field",
            Self::FormatVersionMismatch => "format_version_mismatch",
            Self::EmptySimulationTrace => "empty_simulation_trace",
            Self::EmptyCriteriaEvidence => "empty_criteria_evidence",
            Self::EmptyModeEvidence => "empty_mode_evidence",
            Self::EmptyChecksumEvidence => "empty_checksum_evidence",
            Self::MissingChecksumEntry => "missing_checksum_entry",
            Self::MissingNonCertificationScopeNote => "missing_non_certification_scope_note",
            Self::InvalidEvidenceValue => "invalid_evidence_value",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentValidationReport {
    pub errors: Vec<DeploymentValidationError>,
}

impl DeploymentValidationReport {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    fn push(&mut self, error: DeploymentValidationError) {
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

impl Default for DeploymentValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DeploymentValidationReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.errors.is_empty() {
            return write!(formatter, "deployment package validation passed");
        }

        writeln!(
            formatter,
            "deployment package validation failed with {} error(s):",
            self.errors.len()
        )?;
        for error in &self.errors {
            writeln!(formatter, "- {error}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentValidationError {
    pub field: String,
    pub kind: DeploymentValidationErrorKind,
    pub message: String,
}

impl DeploymentValidationError {
    pub fn new(
        field: impl Into<String>,
        kind: DeploymentValidationErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            kind,
            message: message.into(),
        }
    }
}

impl fmt::Display for DeploymentValidationError {
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
pub enum DeploymentValidationErrorKind {
    EmptyField,
    FormatVersionMismatch,
    EmptyArtifacts,
    EmptyModeProfiles,
    DuplicateArtifactPath,
    DuplicateModeProfile,
    MissingRequiredArtifact,
    MissingRequiredModePurpose,
    MissingModeArtifact,
    ConfigsNotSeparate,
    ChecksumFileMissing,
    InvalidModeProfile,
    InvalidModeArtifactCombination,
}

impl DeploymentValidationErrorKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyField => "empty_field",
            Self::FormatVersionMismatch => "format_version_mismatch",
            Self::EmptyArtifacts => "empty_artifacts",
            Self::EmptyModeProfiles => "empty_mode_profiles",
            Self::DuplicateArtifactPath => "duplicate_artifact_path",
            Self::DuplicateModeProfile => "duplicate_mode_profile",
            Self::MissingRequiredArtifact => "missing_required_artifact",
            Self::MissingRequiredModePurpose => "missing_required_mode_purpose",
            Self::MissingModeArtifact => "missing_mode_artifact",
            Self::ConfigsNotSeparate => "configs_not_separate",
            Self::ChecksumFileMissing => "checksum_file_missing",
            Self::InvalidModeProfile => "invalid_mode_profile",
            Self::InvalidModeArtifactCombination => "invalid_mode_artifact_combination",
        }
    }
}

pub fn required_artifact_roles() -> &'static [DeploymentArtifactRole] {
    &[
        DeploymentArtifactRole::ProductionControlConfig,
        DeploymentArtifactRole::TestVerificationConfig,
        DeploymentArtifactRole::ChannelMap,
        DeploymentArtifactRole::PackageManifest,
        DeploymentArtifactRole::ChecksumIndex,
        DeploymentArtifactRole::QualificationReport,
        DeploymentArtifactRole::QualificationEvidenceSvg,
        DeploymentArtifactRole::GeneratedAt,
    ]
}

pub fn required_mode_purposes() -> &'static [DeploymentModePurpose] {
    &[
        DeploymentModePurpose::ProductionControl,
        DeploymentModePurpose::TestVerification,
        DeploymentModePurpose::SignalValidation,
    ]
}

pub fn checksum_index_contents(
    metadata: &DeploymentIntegrity,
    entries: &[(String, String)],
) -> String {
    let mut output = String::new();
    output.push_str("# FerrisOxide deployment package checksums\n");
    output.push_str(&format!("algorithm={}\n", metadata.algorithm));
    output.push_str(&format!("scope={}\n", metadata.scope));
    output.push_str(&format!("security_note={}\n", metadata.security_note));
    output
        .push_str("# This is not signing, authentication, certification, or tamper-proofing.\n\n");
    for (path, checksum) in entries {
        output.push_str(checksum);
        output.push_str("  ");
        output.push_str(path);
        output.push('\n');
    }
    output
}

fn validate_run_metadata(
    metadata: &QualificationRunMetadata,
    report: &mut QualificationEvidenceValidationReport,
) {
    validate_qualification_non_empty("qualification.case_id", &metadata.case_id, report);
    validate_qualification_non_empty("qualification.workflow", &metadata.workflow, report);
    validate_qualification_non_empty(
        "qualification.input_waveform",
        &metadata.input_waveform,
        report,
    );
}

fn validate_versioned_artifact_link(
    field: &str,
    link: &QualificationVersionedArtifactLink,
    report: &mut QualificationEvidenceValidationReport,
) {
    validate_qualification_non_empty(&format!("{field}.name"), &link.name, report);
    validate_qualification_non_empty(&format!("{field}.version"), &link.version, report);
    validate_qualification_non_empty(&format!("{field}.path"), &link.path, report);
    validate_qualification_non_empty(&format!("{field}.checksum"), &link.checksum, report);
}

fn validate_artifact_link(
    field: &str,
    link: &QualificationArtifactLink,
    report: &mut QualificationEvidenceValidationReport,
) {
    validate_qualification_non_empty(&format!("{field}.path"), &link.path, report);
    validate_qualification_non_empty(&format!("{field}.checksum"), &link.checksum, report);
}

fn validate_simulation_trace(
    trace: &QualificationSimulationTrace,
    report: &mut QualificationEvidenceValidationReport,
) {
    validate_qualification_non_empty("simulation_trace.engine", &trace.engine, report);
    validate_qualification_non_empty("simulation_trace.package_name", &trace.package_name, report);
    validate_qualification_non_empty(
        "simulation_trace.package_version",
        &trace.package_version,
        report,
    );
    validate_qualification_non_empty("simulation_trace.initial_mode", &trace.initial_mode, report);
    if trace.frames.is_empty() {
        report.push(QualificationEvidenceValidationError::new(
            "simulation_trace.frames",
            QualificationEvidenceValidationErrorKind::EmptySimulationTrace,
            "qualification evidence must include at least one simulation trace frame",
        ));
    }

    for (frame_index, frame) in trace.frames.iter().enumerate() {
        let field = format!("simulation_trace.frames[{frame_index}]");
        validate_qualification_finite(&format!("{field}.timestamp_s"), frame.timestamp_s, report);
        validate_qualification_non_empty(&format!("{field}.mode"), &frame.mode, report);
        for (machine_index, machine) in frame.state_machines.iter().enumerate() {
            let machine_field = format!("{field}.state_machines[{machine_index}]");
            validate_qualification_non_empty(
                &format!("{machine_field}.machine"),
                &machine.machine,
                report,
            );
            validate_qualification_non_empty(
                &format!("{machine_field}.state"),
                &machine.state,
                report,
            );
        }
        for (transition_index, transition) in frame.transitions.iter().enumerate() {
            let transition_field = format!("{field}.transitions[{transition_index}]");
            validate_qualification_non_empty(
                &format!("{transition_field}.transition"),
                &transition.transition,
                report,
            );
            validate_qualification_non_empty(
                &format!("{transition_field}.from"),
                &transition.from,
                report,
            );
            validate_qualification_non_empty(
                &format!("{transition_field}.to"),
                &transition.to,
                report,
            );
        }
        for (output_index, output) in frame.outputs.iter().enumerate() {
            let output_field = format!("{field}.outputs[{output_index}]");
            validate_qualification_non_empty(
                &format!("{output_field}.output"),
                &output.output,
                report,
            );
            validate_qualification_non_empty(
                &format!("{output_field}.value"),
                &output.value,
                report,
            );
        }
    }
}

fn validate_criteria_evidence(
    criteria: &[QualificationCriterionEvidence],
    report: &mut QualificationEvidenceValidationReport,
) {
    if criteria.is_empty() {
        report.push(QualificationEvidenceValidationError::new(
            "criteria_evidence",
            QualificationEvidenceValidationErrorKind::EmptyCriteriaEvidence,
            "qualification evidence must include at least one criterion evidence record",
        ));
    }

    for (index, criterion) in criteria.iter().enumerate() {
        let field = format!("criteria_evidence[{index}]");
        validate_qualification_non_empty(
            &format!("{field}.criterion_id"),
            &criterion.criterion_id,
            report,
        );
        validate_qualification_non_empty(
            &format!("{field}.measurement_id"),
            &criterion.measurement_id,
            report,
        );
        validate_qualification_non_empty(&format!("{field}.method"), &criterion.method, report);
        validate_qualification_non_empty(&format!("{field}.channel"), &criterion.channel, report);
        validate_qualification_non_empty(&format!("{field}.unit"), &criterion.unit, report);
        validate_qualification_finite(
            &format!("{field}.measured_value"),
            criterion.measured_value,
            report,
        );
        validate_qualification_finite(
            &format!("{field}.required_value"),
            criterion.required_value,
            report,
        );
        validate_qualification_finite(
            &format!("{field}.tolerance_used"),
            criterion.tolerance_used,
            report,
        );
        validate_qualification_finite(
            &format!("{field}.timestamp_s"),
            criterion.timestamp_s,
            report,
        );
    }
}

fn validate_deployment_package_evidence(
    package: &QualificationDeploymentPackageEvidence,
    report: &mut QualificationEvidenceValidationReport,
) {
    validate_qualification_non_empty("deployment_package.name", &package.name, report);
    validate_qualification_non_empty("deployment_package.version", &package.version, report);
    validate_qualification_non_empty(
        "deployment_package.format_version",
        &package.format_version,
        report,
    );
    if package.format_version != CURRENT_DEPLOYMENT_FORMAT_VERSION {
        report.push(QualificationEvidenceValidationError::new(
            "deployment_package.format_version",
            QualificationEvidenceValidationErrorKind::FormatVersionMismatch,
            format!(
                "expected deployment package format version `{CURRENT_DEPLOYMENT_FORMAT_VERSION}`, got `{}`",
                package.format_version
            ),
        ));
    }
    validate_qualification_non_empty(
        "deployment_package.manifest_path",
        &package.manifest_path,
        report,
    );
    validate_qualification_non_empty(
        "deployment_package.target_identifier",
        &package.target_identifier,
        report,
    );
    validate_qualification_non_empty(
        "deployment_package.generated_at",
        &package.generated_at,
        report,
    );
    if package.modes.is_empty() {
        report.push(QualificationEvidenceValidationError::new(
            "deployment_package.modes",
            QualificationEvidenceValidationErrorKind::EmptyModeEvidence,
            "qualification evidence must link deployment mode profiles",
        ));
    }
    for (index, mode) in package.modes.iter().enumerate() {
        validate_qualification_non_empty(
            &format!("deployment_package.modes[{index}].id"),
            &mode.id,
            report,
        );
    }
}

fn validate_checksum_evidence(
    evidence_report: &QualificationEvidenceReport,
    report: &mut QualificationEvidenceValidationReport,
) {
    let checksum = &evidence_report.checksum_evidence;
    validate_qualification_non_empty(
        "checksum_evidence.checksum_file",
        &checksum.checksum_file,
        report,
    );
    validate_qualification_non_empty("checksum_evidence.algorithm", &checksum.algorithm, report);
    validate_qualification_non_empty("checksum_evidence.scope", &checksum.scope, report);
    validate_qualification_non_empty(
        "checksum_evidence.security_note",
        &checksum.security_note,
        report,
    );
    if checksum.entries.is_empty() {
        report.push(QualificationEvidenceValidationError::new(
            "checksum_evidence.entries",
            QualificationEvidenceValidationErrorKind::EmptyChecksumEvidence,
            "qualification evidence must include checksum entries for deployment artifacts",
        ));
    }
    for (index, entry) in checksum.entries.iter().enumerate() {
        let field = format!("checksum_evidence.entries[{index}]");
        validate_qualification_non_empty(&format!("{field}.path"), &entry.path, report);
        validate_qualification_non_empty(&format!("{field}.checksum"), &entry.checksum, report);
    }

    let required_links = [
        (
            DeploymentArtifactRole::ProductionControlConfig,
            evidence_report.production_control_config.path.as_str(),
        ),
        (
            DeploymentArtifactRole::TestVerificationConfig,
            evidence_report.test_verification_config.path.as_str(),
        ),
        (
            DeploymentArtifactRole::ChannelMap,
            evidence_report.channel_map.path.as_str(),
        ),
        (
            DeploymentArtifactRole::PackageManifest,
            evidence_report.deployment_package.manifest_path.as_str(),
        ),
        (
            DeploymentArtifactRole::ChecksumIndex,
            evidence_report.checksum_evidence.checksum_file.as_str(),
        ),
        (
            DeploymentArtifactRole::QualificationReport,
            evidence_report.report_artifact.path.as_str(),
        ),
        (
            DeploymentArtifactRole::QualificationEvidenceSvg,
            evidence_report.visual_evidence.path.as_str(),
        ),
        (
            DeploymentArtifactRole::GeneratedAt,
            evidence_report.generated_at_artifact.path.as_str(),
        ),
    ];

    for (role, path) in required_links {
        if !checksum
            .entries
            .iter()
            .any(|entry| entry.role == role && entry.path == path)
        {
            report.push(QualificationEvidenceValidationError::new(
                "checksum_evidence.entries",
                QualificationEvidenceValidationErrorKind::MissingChecksumEntry,
                format!(
                    "missing checksum entry for artifact role `{}` at `{path}`",
                    role_name(role)
                ),
            ));
        }
    }
}

fn validate_scope_notes(
    scope_notes: &[String],
    report: &mut QualificationEvidenceValidationReport,
) {
    if scope_notes.is_empty() {
        report.push(QualificationEvidenceValidationError::new(
            "scope_notes",
            QualificationEvidenceValidationErrorKind::MissingNonCertificationScopeNote,
            "qualification evidence must include explicit non-certification scope notes",
        ));
        return;
    }

    for (index, note) in scope_notes.iter().enumerate() {
        validate_qualification_non_empty(&format!("scope_notes[{index}]"), note, report);
    }

    if !scope_notes.iter().any(|note| {
        note.to_ascii_lowercase()
            .contains("not flight certification")
    }) {
        report.push(QualificationEvidenceValidationError::new(
            "scope_notes",
            QualificationEvidenceValidationErrorKind::MissingNonCertificationScopeNote,
            "scope notes must explicitly say the report is not flight certification evidence",
        ));
    }
}

fn validate_mode_profiles(
    mode_profiles: &[DeploymentModeProfile],
    artifacts: &[DeploymentArtifact],
    report: &mut DeploymentValidationReport,
) {
    if mode_profiles.is_empty() {
        report.push(DeploymentValidationError::new(
            "mode_profiles",
            DeploymentValidationErrorKind::EmptyModeProfiles,
            "deployment package must define production_control, test_verification, and signal_validation mode profiles",
        ));
        return;
    }

    let available_roles = artifacts
        .iter()
        .map(|artifact| artifact.role)
        .collect::<BTreeSet<_>>();
    let mut ids = BTreeSet::new();
    let mut purposes = BTreeSet::new();

    for (index, mode) in mode_profiles.iter().enumerate() {
        let field = format!("mode_profiles[{index}]");
        validate_non_empty(&format!("{field}.id"), &mode.id, report);
        if !ids.insert(mode.id.clone()) {
            report.push(DeploymentValidationError::new(
                format!("{field}.id"),
                DeploymentValidationErrorKind::DuplicateModeProfile,
                format!("duplicate mode profile id `{}`", mode.id),
            ));
        }

        purposes.insert(mode.purpose);
        validate_mode_artifact_list(&field, mode, &available_roles, report);
        validate_mode_purpose_policy(&field, mode, report);
    }

    for required_purpose in required_mode_purposes() {
        if !purposes.contains(required_purpose) {
            report.push(DeploymentValidationError::new(
                "mode_profiles",
                DeploymentValidationErrorKind::MissingRequiredModePurpose,
                format!(
                    "missing required mode purpose `{}`",
                    purpose_name(*required_purpose)
                ),
            ));
        }
    }
}

fn validate_mode_artifact_list(
    field: &str,
    mode: &DeploymentModeProfile,
    available_roles: &BTreeSet<DeploymentArtifactRole>,
    report: &mut DeploymentValidationReport,
) {
    if mode.uses_artifacts.is_empty() {
        report.push(DeploymentValidationError::new(
            format!("{field}.uses_artifacts"),
            DeploymentValidationErrorKind::InvalidModeProfile,
            "mode profile must declare at least one used artifact role",
        ));
    }

    let mut used_roles = BTreeSet::new();
    for (role_index, role) in mode.uses_artifacts.iter().enumerate() {
        let role_field = format!("{field}.uses_artifacts[{role_index}]");
        if *role == DeploymentArtifactRole::Other {
            report.push(DeploymentValidationError::new(
                role_field.clone(),
                DeploymentValidationErrorKind::InvalidModeProfile,
                "`other` artifact role is not allowed in mode profiles",
            ));
        }
        if !used_roles.insert(*role) {
            report.push(DeploymentValidationError::new(
                role_field.clone(),
                DeploymentValidationErrorKind::InvalidModeProfile,
                format!("duplicate mode artifact role `{}`", role_name(*role)),
            ));
        }
        if !available_roles.contains(role) {
            report.push(DeploymentValidationError::new(
                role_field,
                DeploymentValidationErrorKind::MissingModeArtifact,
                format!(
                    "mode profile references artifact role `{}` that is not listed in artifacts",
                    role_name(*role)
                ),
            ));
        }
    }
}

fn validate_mode_purpose_policy(
    field: &str,
    mode: &DeploymentModeProfile,
    report: &mut DeploymentValidationReport,
) {
    match mode.purpose {
        DeploymentModePurpose::ProductionControl => {
            validate_production_control_mode(field, mode, report);
        }
        DeploymentModePurpose::TestVerification => {
            validate_verification_mode(field, mode, "test_verification", report);
        }
        DeploymentModePurpose::SignalValidation => {
            validate_verification_mode(field, mode, "signal_validation", report);
        }
    }
}

fn validate_production_control_mode(
    field: &str,
    mode: &DeploymentModeProfile,
    report: &mut DeploymentValidationReport,
) {
    match mode.control_mode.as_deref() {
        Some(value) if !value.trim().is_empty() => {}
        _ => report.push(DeploymentValidationError::new(
            format!("{field}.control_mode"),
            DeploymentValidationErrorKind::InvalidModeProfile,
            "production_control mode must name a production control mode in control_mode",
        )),
    }
    require_mode_artifact(
        field,
        mode,
        DeploymentArtifactRole::ProductionControlConfig,
        report,
    );
    forbid_mode_artifact(
        field,
        mode,
        DeploymentArtifactRole::TestVerificationConfig,
        "production_control mode must not consume test verification config artifacts",
        report,
    );
    forbid_mode_artifact(
        field,
        mode,
        DeploymentArtifactRole::QualificationReport,
        "production_control mode must not consume qualification report artifacts",
        report,
    );
    forbid_mode_artifact(
        field,
        mode,
        DeploymentArtifactRole::QualificationEvidenceSvg,
        "production_control mode must not consume qualification evidence SVG artifacts",
        report,
    );
}

fn validate_verification_mode(
    field: &str,
    mode: &DeploymentModeProfile,
    purpose: &str,
    report: &mut DeploymentValidationReport,
) {
    if let Some(control_mode) = mode.control_mode.as_deref() {
        if !control_mode.trim().is_empty() {
            report.push(DeploymentValidationError::new(
                format!("{field}.control_mode"),
                DeploymentValidationErrorKind::InvalidModeArtifactCombination,
                format!("{purpose} mode must not select production control mode `{control_mode}`"),
            ));
        }
    }

    require_mode_artifact(
        field,
        mode,
        DeploymentArtifactRole::TestVerificationConfig,
        report,
    );
    require_mode_artifact(field, mode, DeploymentArtifactRole::ChannelMap, report);
    forbid_mode_artifact(
        field,
        mode,
        DeploymentArtifactRole::ProductionControlConfig,
        &format!("{purpose} mode must not consume production control config artifacts"),
        report,
    );
    forbid_mode_artifact(
        field,
        mode,
        DeploymentArtifactRole::QualificationReport,
        &format!("{purpose} mode must not consume pre-generated qualification report artifacts"),
        report,
    );
    forbid_mode_artifact(
        field,
        mode,
        DeploymentArtifactRole::QualificationEvidenceSvg,
        &format!(
            "{purpose} mode must not consume pre-generated qualification evidence SVG artifacts"
        ),
        report,
    );
}

fn require_mode_artifact(
    field: &str,
    mode: &DeploymentModeProfile,
    role: DeploymentArtifactRole,
    report: &mut DeploymentValidationReport,
) {
    if !mode_uses_artifact(mode, role) {
        report.push(DeploymentValidationError::new(
            format!("{field}.uses_artifacts"),
            DeploymentValidationErrorKind::MissingModeArtifact,
            format!(
                "{} mode must use artifact role `{}`",
                purpose_name(mode.purpose),
                role_name(role)
            ),
        ));
    }
}

fn forbid_mode_artifact(
    field: &str,
    mode: &DeploymentModeProfile,
    role: DeploymentArtifactRole,
    message: &str,
    report: &mut DeploymentValidationReport,
) {
    if mode_uses_artifact(mode, role) {
        report.push(DeploymentValidationError::new(
            format!("{field}.uses_artifacts"),
            DeploymentValidationErrorKind::InvalidModeArtifactCombination,
            message,
        ));
    }
}

fn mode_uses_artifact(mode: &DeploymentModeProfile, role: DeploymentArtifactRole) -> bool {
    mode.uses_artifacts.contains(&role)
}

fn validate_artifacts(
    artifacts: &[DeploymentArtifact],
    integrity: &DeploymentIntegrity,
    report: &mut DeploymentValidationReport,
) {
    if artifacts.is_empty() {
        report.push(DeploymentValidationError::new(
            "artifacts",
            DeploymentValidationErrorKind::EmptyArtifacts,
            "deployment package must list required artifacts",
        ));
        return;
    }

    let mut paths = BTreeSet::new();
    let mut roles = BTreeSet::new();
    for (index, artifact) in artifacts.iter().enumerate() {
        if artifact.path.trim().is_empty() {
            report.push(DeploymentValidationError::new(
                format!("artifacts[{index}].path"),
                DeploymentValidationErrorKind::EmptyField,
                "artifact path must not be empty",
            ));
        }
        if artifact.media_type.trim().is_empty() {
            report.push(DeploymentValidationError::new(
                format!("artifacts[{index}].media_type"),
                DeploymentValidationErrorKind::EmptyField,
                "artifact media_type must not be empty",
            ));
        }
        if !paths.insert(artifact.path.clone()) {
            report.push(DeploymentValidationError::new(
                format!("artifacts[{index}].path"),
                DeploymentValidationErrorKind::DuplicateArtifactPath,
                format!("duplicate artifact path `{}`", artifact.path),
            ));
        }
        if artifact.required {
            roles.insert(artifact.role);
        }
    }

    for required_role in required_artifact_roles() {
        if !roles.contains(required_role) {
            report.push(DeploymentValidationError::new(
                "artifacts",
                DeploymentValidationErrorKind::MissingRequiredArtifact,
                format!(
                    "missing required artifact role `{}`",
                    role_name(*required_role)
                ),
            ));
        }
    }

    let production_config =
        artifact_path(artifacts, DeploymentArtifactRole::ProductionControlConfig);
    let test_config = artifact_path(artifacts, DeploymentArtifactRole::TestVerificationConfig);
    if production_config.is_some() && production_config == test_config {
        report.push(DeploymentValidationError::new(
            "artifacts",
            DeploymentValidationErrorKind::ConfigsNotSeparate,
            "production control config and test verification config must be separate artifacts",
        ));
    }

    if !artifacts
        .iter()
        .any(|artifact| artifact.path == integrity.checksum_file)
    {
        report.push(DeploymentValidationError::new(
            "integrity.checksum_file",
            DeploymentValidationErrorKind::ChecksumFileMissing,
            format!(
                "checksum file `{}` must appear in artifacts",
                integrity.checksum_file
            ),
        ));
    }
}

fn artifact_path(artifacts: &[DeploymentArtifact], role: DeploymentArtifactRole) -> Option<&str> {
    artifacts
        .iter()
        .find(|artifact| artifact.required && artifact.role == role)
        .map(|artifact| artifact.path.as_str())
}

fn purpose_name(purpose: DeploymentModePurpose) -> &'static str {
    match purpose {
        DeploymentModePurpose::ProductionControl => "production_control",
        DeploymentModePurpose::TestVerification => "test_verification",
        DeploymentModePurpose::SignalValidation => "signal_validation",
    }
}

fn role_name(role: DeploymentArtifactRole) -> &'static str {
    match role {
        DeploymentArtifactRole::ProductionControlConfig => "production_control_config",
        DeploymentArtifactRole::TestVerificationConfig => "test_verification_config",
        DeploymentArtifactRole::ChannelMap => "channel_map",
        DeploymentArtifactRole::PackageManifest => "package_manifest",
        DeploymentArtifactRole::ChecksumIndex => "checksum_index",
        DeploymentArtifactRole::QualificationReport => "qualification_report",
        DeploymentArtifactRole::QualificationEvidenceSvg => "qualification_evidence_svg",
        DeploymentArtifactRole::GeneratedAt => "generated_at",
        DeploymentArtifactRole::Other => "other",
    }
}

fn validate_qualification_non_empty(
    field: &str,
    value: &str,
    report: &mut QualificationEvidenceValidationReport,
) {
    if value.trim().is_empty() {
        report.push(QualificationEvidenceValidationError::new(
            field,
            QualificationEvidenceValidationErrorKind::EmptyField,
            "field must not be empty",
        ));
    }
}

fn validate_qualification_finite(
    field: &str,
    value: f64,
    report: &mut QualificationEvidenceValidationReport,
) {
    if !value.is_finite() {
        report.push(QualificationEvidenceValidationError::new(
            field,
            QualificationEvidenceValidationErrorKind::InvalidEvidenceValue,
            "numeric evidence values must be finite",
        ));
    }
}

fn validate_non_empty(field: &str, value: &str, report: &mut DeploymentValidationReport) {
    if value.trim().is_empty() {
        report.push(DeploymentValidationError::new(
            field,
            DeploymentValidationErrorKind::EmptyField,
            "field must not be empty",
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_manifest_includes_required_artifacts_and_validates() {
        let manifest: DeploymentPackageManifest = serde_json::from_str(include_str!(
            "../../../examples/deployment-package/heated-actuator/manifest.json"
        ))
        .expect("example manifest should parse");

        assert_eq!(manifest.validate(), Ok(()));
        let roles = manifest
            .artifacts
            .iter()
            .map(|artifact| artifact.role)
            .collect::<BTreeSet<_>>();
        for required_role in required_artifact_roles() {
            assert!(roles.contains(required_role));
        }
    }

    #[test]
    fn validation_rejects_missing_required_artifact() {
        let mut manifest = valid_manifest();
        manifest
            .artifacts
            .retain(|artifact| artifact.role != DeploymentArtifactRole::QualificationReport);

        let report = manifest
            .validate()
            .expect_err("missing qualification report should fail");

        assert!(report.errors.iter().any(|error| {
            error.kind == DeploymentValidationErrorKind::MissingRequiredArtifact
                && error.message.contains("qualification_report")
        }));
    }

    #[test]
    fn validation_keeps_production_and_test_configs_separate() {
        let mut manifest = valid_manifest();
        for artifact in &mut manifest.artifacts {
            if artifact.role == DeploymentArtifactRole::TestVerificationConfig {
                artifact.path = "production-control-config.toml".to_string();
            }
        }

        let report = manifest
            .validate()
            .expect_err("conflated configs should fail");

        assert!(report
            .errors
            .iter()
            .any(|error| { error.kind == DeploymentValidationErrorKind::ConfigsNotSeparate }));
    }

    #[test]
    fn validation_requires_all_mode_purposes() {
        let mut manifest = valid_manifest();
        manifest
            .mode_profiles
            .retain(|mode| mode.purpose != DeploymentModePurpose::SignalValidation);

        let report = manifest
            .validate()
            .expect_err("missing signal validation mode should fail");

        assert!(report.errors.iter().any(|error| {
            error.kind == DeploymentValidationErrorKind::MissingRequiredModePurpose
                && error.message.contains("signal_validation")
        }));
    }

    #[test]
    fn validation_rejects_mixed_production_and_verification_mode_artifacts() {
        let mut manifest = valid_manifest();
        let mode = manifest
            .mode_profiles
            .iter_mut()
            .find(|mode| mode.purpose == DeploymentModePurpose::TestVerification)
            .expect("valid manifest should include test verification mode");
        mode.control_mode = Some("normal".to_string());
        mode.uses_artifacts
            .push(DeploymentArtifactRole::ProductionControlConfig);

        let report = manifest
            .validate()
            .expect_err("mixed production/test mode should fail");

        assert!(report.errors.iter().any(|error| {
            error.kind == DeploymentValidationErrorKind::InvalidModeArtifactCombination
                && error.message.contains("test_verification mode must not")
        }));
    }

    #[test]
    fn checksum_index_wording_disclaims_signing_and_certification() {
        let integrity = DeploymentIntegrity::drift_detection_only("checksum.txt");
        let contents = checksum_index_contents(
            &integrity,
            &[(
                "manifest.json".to_string(),
                "fnv1a64:0123456789abcdef".to_string(),
            )],
        );

        assert!(contents.contains("not signing"));
        assert!(contents.contains("certification"));
        assert!(contents.contains("manifest.json"));
    }

    #[test]
    fn example_qualification_evidence_report_validates_and_matches_exact_json() {
        let expected = include_str!(
            "../../../examples/deployment-package/heated-actuator/qualification-report.json"
        );
        let report: QualificationEvidenceReport =
            serde_json::from_str(expected).expect("example qualification report should parse");

        assert_eq!(report.validate(), Ok(()));

        let rendered = format!(
            "{}\n",
            serde_json::to_string_pretty(&report)
                .expect("qualification report should render deterministically")
        );
        assert_eq!(rendered, expected);
    }

    #[test]
    fn qualification_evidence_requires_non_certification_scope_note() {
        let mut report = valid_qualification_evidence_report();
        report.scope_notes = vec!["software evidence only".to_string()];

        let validation = report
            .validate()
            .expect_err("missing certification disclaimer should fail");

        assert!(validation.errors.iter().any(|error| {
            error.kind == QualificationEvidenceValidationErrorKind::MissingNonCertificationScopeNote
                && error.message.contains("not flight certification")
        }));
    }

    #[test]
    fn qualification_evidence_requires_checksum_links() {
        let mut report = valid_qualification_evidence_report();
        report
            .checksum_evidence
            .entries
            .retain(|entry| entry.role != DeploymentArtifactRole::TestVerificationConfig);

        let validation = report
            .validate()
            .expect_err("missing checksum link should fail");

        assert!(validation.errors.iter().any(|error| {
            error.kind == QualificationEvidenceValidationErrorKind::MissingChecksumEntry
                && error.message.contains("test_verification_config")
        }));
    }

    #[test]
    fn qualification_evidence_requires_trace_and_criteria_records() {
        let mut report = valid_qualification_evidence_report();
        report.simulation_trace.frames.clear();
        report.criteria_evidence.clear();

        let validation = report
            .validate()
            .expect_err("empty trace and criteria evidence should fail");

        assert!(validation.errors.iter().any(|error| {
            error.kind == QualificationEvidenceValidationErrorKind::EmptySimulationTrace
        }));
        assert!(validation.errors.iter().any(|error| {
            error.kind == QualificationEvidenceValidationErrorKind::EmptyCriteriaEvidence
        }));
    }

    fn valid_manifest() -> DeploymentPackageManifest {
        DeploymentPackageManifest {
            manifest_version: CURRENT_DEPLOYMENT_FORMAT_VERSION.to_string(),
            package: DeploymentPackageMetadata::new(
                "heated-actuator-controller-deployment",
                "0.1.0",
            ),
            target: DeploymentTarget {
                kind: DeploymentTargetKind::ControllerRuntime,
                identifier: "raspberry-pi-5-bare-metal".to_string(),
                notes: Vec::new(),
            },
            mode_profiles: valid_mode_profiles(),
            generated_at: "2026-06-01T00:00:00Z".to_string(),
            artifacts: required_artifact_roles()
                .iter()
                .map(|role| {
                    DeploymentArtifact::required(
                        match role {
                            DeploymentArtifactRole::ProductionControlConfig => {
                                "production-control-config.toml"
                            }
                            DeploymentArtifactRole::TestVerificationConfig => {
                                "test-verification-config.toml"
                            }
                            DeploymentArtifactRole::ChannelMap => "channel-map.toml",
                            DeploymentArtifactRole::PackageManifest => "manifest.json",
                            DeploymentArtifactRole::ChecksumIndex => "checksum.txt",
                            DeploymentArtifactRole::QualificationReport => {
                                "qualification-report.json"
                            }
                            DeploymentArtifactRole::QualificationEvidenceSvg => {
                                "qualification-evidence.svg"
                            }
                            DeploymentArtifactRole::GeneratedAt => "generated-at.txt",
                            DeploymentArtifactRole::Other => "other.txt",
                        },
                        *role,
                        "text/plain",
                    )
                })
                .collect(),
            integrity: DeploymentIntegrity::drift_detection_only("checksum.txt"),
            scope_notes: vec!["software evidence only".to_string()],
        }
    }

    fn valid_mode_profiles() -> Vec<DeploymentModeProfile> {
        vec![
            DeploymentModeProfile {
                id: "production-normal".to_string(),
                purpose: DeploymentModePurpose::ProductionControl,
                control_mode: Some("normal".to_string()),
                uses_artifacts: vec![DeploymentArtifactRole::ProductionControlConfig],
                notes: Vec::new(),
            },
            DeploymentModeProfile {
                id: "test-verification".to_string(),
                purpose: DeploymentModePurpose::TestVerification,
                control_mode: None,
                uses_artifacts: vec![
                    DeploymentArtifactRole::TestVerificationConfig,
                    DeploymentArtifactRole::ChannelMap,
                ],
                notes: Vec::new(),
            },
            DeploymentModeProfile {
                id: "signal-validation".to_string(),
                purpose: DeploymentModePurpose::SignalValidation,
                control_mode: None,
                uses_artifacts: vec![
                    DeploymentArtifactRole::TestVerificationConfig,
                    DeploymentArtifactRole::ChannelMap,
                ],
                notes: Vec::new(),
            },
        ]
    }

    fn valid_qualification_evidence_report() -> QualificationEvidenceReport {
        serde_json::from_str(include_str!(
            "../../../examples/deployment-package/heated-actuator/qualification-report.json"
        ))
        .expect("example qualification report should parse")
    }
}
