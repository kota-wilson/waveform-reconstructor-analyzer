use std::fmt;

use crate::model::{
    TransformCapabilityStatus, TransformEvidenceLevel, TransformRuntimeProfile,
    TransformStepMetadata, WaveformMetadata,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformRuntimeValidationErrorKind {
    UnsupportedTransformRuntimeProfile,
    FutureGatedTransform,
    PlannedTransformNotImplemented,
    OfflineTransformNotStreamingSupported,
    MissingSampleTiming,
    InvalidSampleTiming,
    MissingNoStdEvidence,
    MissingMicroRuntimeEvidence,
    DependencyGateRequired,
    HardwareGateRequired,
    CertificationGateRequired,
}

impl TransformRuntimeValidationErrorKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedTransformRuntimeProfile => "unsupported_transform_runtime_profile",
            Self::FutureGatedTransform => "future_gated_transform",
            Self::PlannedTransformNotImplemented => "planned_transform_not_implemented",
            Self::OfflineTransformNotStreamingSupported => {
                "offline_transform_not_streaming_supported"
            }
            Self::MissingSampleTiming => "missing_sample_timing",
            Self::InvalidSampleTiming => "invalid_sample_timing",
            Self::MissingNoStdEvidence => "missing_no_std_evidence",
            Self::MissingMicroRuntimeEvidence => "missing_micro_runtime_evidence",
            Self::DependencyGateRequired => "dependency_gate_required",
            Self::HardwareGateRequired => "hardware_gate_required",
            Self::CertificationGateRequired => "certification_gate_required",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformRuntimeValidationError {
    pub field: String,
    pub kind: TransformRuntimeValidationErrorKind,
    pub transform_name: String,
    pub requested_profile: TransformRuntimeProfile,
    pub supported_profiles: Vec<TransformRuntimeProfile>,
    pub reason: String,
}

impl TransformRuntimeValidationError {
    fn new(
        field: impl Into<String>,
        kind: TransformRuntimeValidationErrorKind,
        transform: &TransformStepMetadata,
        requested_profile: TransformRuntimeProfile,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            kind,
            transform_name: transform.name.clone(),
            requested_profile,
            supported_profiles: transform.runtime_profiles.clone(),
            reason: reason.into(),
        }
    }
}

impl fmt::Display for TransformRuntimeValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {} for transform `{}` requested as `{}` ({})",
            self.field,
            self.reason,
            self.transform_name,
            self.requested_profile.as_str(),
            self.kind.as_str()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformRuntimeValidationReport {
    pub errors: Vec<TransformRuntimeValidationError>,
}

impl TransformRuntimeValidationReport {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn is_pass(&self) -> bool {
        self.errors.is_empty()
    }

    fn push(&mut self, error: TransformRuntimeValidationError) {
        self.errors.push(error);
    }

    fn into_result(self) -> Result<(), Self> {
        if self.is_pass() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl Default for TransformRuntimeValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TransformRuntimeValidationReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.errors.is_empty() {
            return write!(formatter, "transform runtime-profile validation passed");
        }

        writeln!(
            formatter,
            "transform runtime-profile validation failed with {} error(s):",
            self.errors.len()
        )?;
        for error in &self.errors {
            writeln!(formatter, "- {error}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransformRuntimeTimingEvidence {
    pub has_sample_timing: bool,
    pub timestamp_unit_is_seconds: bool,
    pub intervals_are_finite: bool,
    pub intervals_are_positive: bool,
    pub nominal_sample_rate_hz_present: bool,
}

impl TransformRuntimeTimingEvidence {
    pub const fn missing() -> Self {
        Self {
            has_sample_timing: false,
            timestamp_unit_is_seconds: false,
            intervals_are_finite: false,
            intervals_are_positive: false,
            nominal_sample_rate_hz_present: false,
        }
    }

    pub const fn valid_seconds() -> Self {
        Self {
            has_sample_timing: true,
            timestamp_unit_is_seconds: true,
            intervals_are_finite: true,
            intervals_are_positive: true,
            nominal_sample_rate_hz_present: true,
        }
    }

    pub fn from_waveform_metadata(metadata: &WaveformMetadata) -> Self {
        let Some(interval) = &metadata.sample_interval else {
            return Self::missing();
        };

        Self {
            has_sample_timing: true,
            timestamp_unit_is_seconds: metadata.time_unit.name == "s",
            intervals_are_finite: interval.min.is_finite()
                && interval.max.is_finite()
                && interval.nominal.is_finite(),
            intervals_are_positive: interval.min > 0.0
                && interval.max > 0.0
                && interval.nominal > 0.0,
            nominal_sample_rate_hz_present: metadata.nominal_sample_rate_hz.is_some(),
        }
    }

    fn invalid_reason(self) -> Option<&'static str> {
        if !self.has_sample_timing {
            return Some("sample timing metadata is missing");
        }
        if !self.timestamp_unit_is_seconds {
            return Some("timestamp unit must be seconds for sample-rate-required transforms");
        }
        if !self.intervals_are_finite {
            return Some("sample intervals must be finite");
        }
        if !self.intervals_are_positive {
            return Some("sample intervals must be positive");
        }
        if !self.nominal_sample_rate_hz_present {
            return Some("nominal sample rate in Hz is required");
        }
        None
    }
}

pub fn validate_waveform_metadata_runtime_profile(
    metadata: &WaveformMetadata,
    requested_profile: TransformRuntimeProfile,
) -> Result<(), TransformRuntimeValidationReport> {
    validate_transform_runtime_profile(
        &metadata.transform_steps,
        requested_profile,
        TransformRuntimeTimingEvidence::from_waveform_metadata(metadata),
    )
}

pub fn validate_transform_runtime_profile(
    transforms: &[TransformStepMetadata],
    requested_profile: TransformRuntimeProfile,
    timing: TransformRuntimeTimingEvidence,
) -> Result<(), TransformRuntimeValidationReport> {
    let mut report = TransformRuntimeValidationReport::new();
    for (index, transform) in transforms.iter().enumerate() {
        validate_transform_runtime_profile_step(
            transform,
            index,
            requested_profile,
            timing,
            &mut report,
        );
    }
    report.into_result()
}

fn validate_transform_runtime_profile_step(
    transform: &TransformStepMetadata,
    index: usize,
    requested_profile: TransformRuntimeProfile,
    timing: TransformRuntimeTimingEvidence,
    report: &mut TransformRuntimeValidationReport,
) {
    let field = format!("transform_steps[{index}].runtime_profiles");

    if requested_profile == TransformRuntimeProfile::FutureGated {
        report.push(TransformRuntimeValidationError::new(
            &field,
            TransformRuntimeValidationErrorKind::FutureGatedTransform,
            transform,
            requested_profile,
            "future_gated is not executable in current product paths",
        ));
    }

    if !transform.runtime_profiles.contains(&requested_profile) {
        report.push(TransformRuntimeValidationError::new(
            &field,
            TransformRuntimeValidationErrorKind::UnsupportedTransformRuntimeProfile,
            transform,
            requested_profile,
            format!(
                "transform supports [{}]",
                profile_list(&transform.runtime_profiles)
            ),
        ));
    }

    validate_capability_status(transform, index, requested_profile, report);
    validate_runtime_execution_flags(transform, index, requested_profile, report);
    validate_profile_evidence(transform, index, requested_profile, report);
    validate_timing(transform, index, requested_profile, timing, report);
}

fn validate_capability_status(
    transform: &TransformStepMetadata,
    index: usize,
    requested_profile: TransformRuntimeProfile,
    report: &mut TransformRuntimeValidationReport,
) {
    let (kind, reason) = match transform.capability_status {
        TransformCapabilityStatus::Implemented => return,
        TransformCapabilityStatus::Planned | TransformCapabilityStatus::Research => (
            TransformRuntimeValidationErrorKind::PlannedTransformNotImplemented,
            "transform is not implemented for executable exposure",
        ),
        TransformCapabilityStatus::DependencyGated => (
            TransformRuntimeValidationErrorKind::DependencyGateRequired,
            "dependency review is required before executable exposure",
        ),
        TransformCapabilityStatus::HardwareGated => (
            TransformRuntimeValidationErrorKind::HardwareGateRequired,
            "hardware and environment gates are required before exposure",
        ),
        TransformCapabilityStatus::CertificationGated => (
            TransformRuntimeValidationErrorKind::CertificationGateRequired,
            "certification evidence planning is required before claims",
        ),
    };

    report.push(TransformRuntimeValidationError::new(
        format!("transform_steps[{index}].capability_status"),
        kind,
        transform,
        requested_profile,
        reason,
    ));
}

fn validate_runtime_execution_flags(
    transform: &TransformStepMetadata,
    index: usize,
    requested_profile: TransformRuntimeProfile,
    report: &mut TransformRuntimeValidationReport,
) {
    if requested_profile != TransformRuntimeProfile::Desktop && transform.offline_only {
        report.push(TransformRuntimeValidationError::new(
            format!("transform_steps[{index}].offline_only"),
            TransformRuntimeValidationErrorKind::OfflineTransformNotStreamingSupported,
            transform,
            requested_profile,
            "offline-only transforms cannot be exposed to embedded or streaming runtime profiles",
        ));
    }
}

fn validate_profile_evidence(
    transform: &TransformStepMetadata,
    index: usize,
    requested_profile: TransformRuntimeProfile,
    report: &mut TransformRuntimeValidationReport,
) {
    match requested_profile {
        TransformRuntimeProfile::Pi5NoStdCandidate
            if transform
                .runtime_profiles
                .contains(&TransformRuntimeProfile::Pi5NoStdCandidate)
                && !matches!(
                    transform.evidence_level,
                    TransformEvidenceLevel::ParityTested | TransformEvidenceLevel::Validated
                ) =>
        {
            report.push(TransformRuntimeValidationError::new(
                format!("transform_steps[{index}].evidence_level"),
                TransformRuntimeValidationErrorKind::MissingNoStdEvidence,
                transform,
                requested_profile,
                "Pi 5 no_std candidate exposure requires parity or validation evidence",
            ));
        }
        TransformRuntimeProfile::Pico2Candidate
            if transform
                .runtime_profiles
                .contains(&TransformRuntimeProfile::Pico2Candidate)
                && transform.evidence_level != TransformEvidenceLevel::Validated =>
        {
            report.push(TransformRuntimeValidationError::new(
                format!("transform_steps[{index}].evidence_level"),
                TransformRuntimeValidationErrorKind::MissingMicroRuntimeEvidence,
                transform,
                requested_profile,
                "Pico 2 candidate exposure requires approved micro-runtime validation evidence",
            ));
        }
        _ => {}
    }
}

fn validate_timing(
    transform: &TransformStepMetadata,
    index: usize,
    requested_profile: TransformRuntimeProfile,
    timing: TransformRuntimeTimingEvidence,
    report: &mut TransformRuntimeValidationReport,
) {
    if !transform.sample_rate_required {
        return;
    }

    let Some(reason) = timing.invalid_reason() else {
        return;
    };
    let kind = if timing.has_sample_timing {
        TransformRuntimeValidationErrorKind::InvalidSampleTiming
    } else {
        TransformRuntimeValidationErrorKind::MissingSampleTiming
    };

    report.push(TransformRuntimeValidationError::new(
        format!("transform_steps[{index}].sample_rate_required"),
        kind,
        transform,
        requested_profile,
        reason,
    ));
}

fn profile_list(profiles: &[TransformRuntimeProfile]) -> String {
    profiles
        .iter()
        .map(|profile| profile.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::criteria::SignalState;
    use crate::event::{
        evaluate_event_pipeline, EdgeDirectionFilter, EdgeExtractionTransform, EventTransformStep,
        EventValidationStep, MissingPulseValidation, SchmittTriggerTransform,
    };
    use crate::filter::{apply_filter_chain, FilterStep, MovingAverageFilter};
    use crate::model::{
        Channel, TransformCategory, TransformParameterMetadata, TransformPhaseEffect,
        TransformStepMetadata, Unit, Waveform,
    };

    #[test]
    fn accepts_desktop_profile_for_current_waveform_metadata() {
        let raw = waveform_seconds();
        let derived = apply_filter_chain(
            &raw,
            &[FilterStep::MovingAverage(MovingAverageFilter {
                window_samples: 2,
            })],
        )
        .expect("moving average should apply");

        assert_eq!(
            validate_waveform_metadata_runtime_profile(
                &derived.metadata,
                TransformRuntimeProfile::Desktop
            ),
            Ok(())
        );
    }

    #[test]
    fn rejects_embedded_profile_for_current_waveform_metadata() {
        let raw = waveform_seconds();
        let derived = apply_filter_chain(
            &raw,
            &[FilterStep::MovingAverage(MovingAverageFilter {
                window_samples: 2,
            })],
        )
        .expect("moving average should apply");

        let report = validate_waveform_metadata_runtime_profile(
            &derived.metadata,
            TransformRuntimeProfile::Pi5NoStdCandidate,
        )
        .expect_err("desktop-only metadata should reject embedded profile");

        assert!(report.errors.iter().any(|error| {
            error.kind == TransformRuntimeValidationErrorKind::UnsupportedTransformRuntimeProfile
                && error.field == "transform_steps[0].runtime_profiles"
                && error.transform_name == "moving_average"
                && error.requested_profile == TransformRuntimeProfile::Pi5NoStdCandidate
                && error.supported_profiles == vec![TransformRuntimeProfile::Desktop]
        }));
    }

    #[test]
    fn rejects_future_gated_profile_for_current_waveform_metadata() {
        let raw = waveform_seconds();
        let derived = apply_filter_chain(
            &raw,
            &[FilterStep::MovingAverage(MovingAverageFilter {
                window_samples: 2,
            })],
        )
        .expect("moving average should apply");

        let report = validate_waveform_metadata_runtime_profile(
            &derived.metadata,
            TransformRuntimeProfile::FutureGated,
        )
        .expect_err("future_gated should not be executable");

        assert!(report.errors.iter().any(|error| {
            error.kind == TransformRuntimeValidationErrorKind::FutureGatedTransform
                && error.transform_name == "moving_average"
        }));
    }

    #[test]
    fn rejects_missing_timing_for_sample_rate_required_metadata() {
        let step = low_pass_metadata();

        let report = validate_transform_runtime_profile(
            &[step],
            TransformRuntimeProfile::Desktop,
            TransformRuntimeTimingEvidence::missing(),
        )
        .expect_err("low-pass runtime exposure should require timing");

        assert!(report.errors.iter().any(|error| {
            error.kind == TransformRuntimeValidationErrorKind::MissingSampleTiming
                && error.field == "transform_steps[0].sample_rate_required"
                && error.transform_name == "low_pass"
        }));
    }

    #[test]
    fn rejects_invalid_timing_for_sample_rate_required_metadata() {
        let invalid_cases = [
            TransformRuntimeTimingEvidence {
                timestamp_unit_is_seconds: false,
                ..TransformRuntimeTimingEvidence::valid_seconds()
            },
            TransformRuntimeTimingEvidence {
                intervals_are_finite: false,
                ..TransformRuntimeTimingEvidence::valid_seconds()
            },
            TransformRuntimeTimingEvidence {
                intervals_are_positive: false,
                ..TransformRuntimeTimingEvidence::valid_seconds()
            },
            TransformRuntimeTimingEvidence {
                nominal_sample_rate_hz_present: false,
                ..TransformRuntimeTimingEvidence::valid_seconds()
            },
        ];

        for timing in invalid_cases {
            let report = validate_transform_runtime_profile(
                &[low_pass_metadata()],
                TransformRuntimeProfile::Desktop,
                timing,
            )
            .expect_err("invalid timing should reject sample-rate-required metadata");

            assert!(report.errors.iter().any(|error| {
                error.kind == TransformRuntimeValidationErrorKind::InvalidSampleTiming
                    && error.transform_name == "low_pass"
            }));
        }
    }

    #[test]
    fn rejects_event_and_validation_metadata_for_embedded_profiles() {
        let waveform = Waveform::new(
            vec![0.0, 0.001, 0.002, 0.003],
            vec![Channel::new(
                "switch_v",
                Unit::volts(),
                vec![0.0, 5.0, 5.0, 5.0],
            )],
        )
        .expect("event waveform should build");
        let event_evaluation = evaluate_event_pipeline(
            &waveform,
            &[
                EventTransformStep::SchmittTrigger(SchmittTriggerTransform {
                    id: "state".to_string(),
                    channel: "switch_v".to_string(),
                    on_threshold_v: 3.0,
                    off_threshold_v: 2.0,
                    initial_state: SignalState::Low,
                }),
                EventTransformStep::EdgeExtraction(EdgeExtractionTransform {
                    id: "edges".to_string(),
                    channel: "switch_v".to_string(),
                }),
            ],
            &[EventValidationStep::MissingPulse(MissingPulseValidation {
                id: "missing".to_string(),
                channel: "switch_v".to_string(),
                direction: EdgeDirectionFilter::Rising,
                expected_count: 1,
            })],
        )
        .expect("event pipeline should evaluate");
        let mut metadata = event_evaluation
            .records
            .iter()
            .map(|record| record.transform_metadata.clone())
            .collect::<Vec<_>>();
        metadata.extend(
            event_evaluation
                .validations
                .iter()
                .map(|validation| validation.transform_metadata.clone()),
        );

        assert_eq!(
            validate_transform_runtime_profile(
                &metadata,
                TransformRuntimeProfile::Desktop,
                TransformRuntimeTimingEvidence::from_waveform_metadata(&waveform.metadata),
            ),
            Ok(())
        );

        let report = validate_transform_runtime_profile(
            &metadata,
            TransformRuntimeProfile::Pi5NoStdCandidate,
            TransformRuntimeTimingEvidence::from_waveform_metadata(&waveform.metadata),
        )
        .expect_err("desktop-only event metadata should reject embedded profile");

        assert!(report.errors.iter().any(|error| {
            error.kind == TransformRuntimeValidationErrorKind::UnsupportedTransformRuntimeProfile
                && error.transform_name == "schmitt_trigger"
        }));
        assert!(report.errors.iter().any(|error| {
            error.kind == TransformRuntimeValidationErrorKind::OfflineTransformNotStreamingSupported
                && error.transform_name == "missing_pulse"
        }));
    }

    fn waveform_seconds() -> Waveform {
        Waveform::new(
            vec![0.0, 0.001, 0.002, 0.003],
            vec![Channel::new(
                "input_v",
                Unit::volts(),
                vec![0.0, 1.0, 0.5, 1.5],
            )],
        )
        .expect("waveform should build")
    }

    fn low_pass_metadata() -> TransformStepMetadata {
        TransformStepMetadata::implemented_desktop(
            "low_pass(cutoff_hz=10)",
            "low_pass",
            TransformCategory::FrequencyFilter,
            vec![TransformParameterMetadata::float("cutoff_hz", 10.0, "Hz")],
            true,
            true,
            TransformPhaseEffect::Delay,
        )
    }
}
