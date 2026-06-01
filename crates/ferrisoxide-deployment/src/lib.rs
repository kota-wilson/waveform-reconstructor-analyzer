//! Deployment package manifest format for controller/runtime workflows.
//!
//! This crate defines reviewable package metadata only. It does not export
//! packages, sign artifacts, load RTOS configs, bind HALs, talk to hardware, or
//! claim qualification/certification status.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

pub const CURRENT_DEPLOYMENT_FORMAT_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeploymentPackageManifest {
    pub manifest_version: String,
    pub package: DeploymentPackageMetadata,
    pub target: DeploymentTarget,
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
    DuplicateArtifactPath,
    MissingRequiredArtifact,
    ConfigsNotSeparate,
    ChecksumFileMissing,
}

impl DeploymentValidationErrorKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyField => "empty_field",
            Self::FormatVersionMismatch => "format_version_mismatch",
            Self::EmptyArtifacts => "empty_artifacts",
            Self::DuplicateArtifactPath => "duplicate_artifact_path",
            Self::MissingRequiredArtifact => "missing_required_artifact",
            Self::ConfigsNotSeparate => "configs_not_separate",
            Self::ChecksumFileMissing => "checksum_file_missing",
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
}
