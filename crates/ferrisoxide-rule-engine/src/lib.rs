#![no_std]

//! Shared rule execution semantics for FerrisOxide Signal.
//!
//! This crate evaluates rule criteria over caller-provided time/sample slices.
//! It deliberately avoids CSV parsing, TOML parsing, report rendering,
//! plotting, file I/O, DAQ/controller I/O, hardware HALs, RTOS SDKs, and
//! certification claims.

extern crate alloc;

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::fmt;

use ferrisoxide_measurements::{
    count_state_transitions, maximum_sample, measure_fall_time, measure_rise_time, minimum_sample,
    state_run_extremum, EdgeDirection, MeasurementError, RunSelection, SignalState,
};

const FLOAT_TOLERANCE: f64 = 1.0e-12;

pub type Result<T> = core::result::Result<T, RuleEngineError>;

#[derive(Debug, Clone, PartialEq)]
pub enum RuleEngineError {
    EmptyInput,
    MissingChannel { channel: String },
    InvalidWaveform { reason: String },
    InvalidParameter { name: String, reason: String },
}

impl fmt::Display for RuleEngineError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(formatter, "empty input"),
            Self::MissingChannel { channel } => write!(formatter, "missing channel `{channel}`"),
            Self::InvalidWaveform { reason } => write!(formatter, "invalid waveform: {reason}"),
            Self::InvalidParameter { name, reason } => {
                write!(formatter, "invalid parameter `{name}`: {reason}")
            }
        }
    }
}

pub type BorrowedRuleResult<'a, T> = core::result::Result<T, BorrowedRuleError<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorrowedRuleError<'a> {
    EmptyInput,
    MissingChannel {
        channel: &'a str,
    },
    InvalidWaveform {
        reason: &'static str,
    },
    InvalidParameter {
        name: &'static str,
        reason: &'static str,
    },
}

impl<'a> fmt::Display for BorrowedRuleError<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(formatter, "empty input"),
            Self::MissingChannel { channel } => write!(formatter, "missing channel `{channel}`"),
            Self::InvalidWaveform { reason } => write!(formatter, "invalid waveform: {reason}"),
            Self::InvalidParameter { name, reason } => {
                write!(formatter, "invalid parameter `{name}`: {reason}")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleOutcome {
    Pass,
    Fail,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuleResult {
    pub criterion_id: String,
    pub outcome: RuleOutcome,
    pub failed_criterion: Option<String>,
    pub measurement_id: String,
    pub channel: String,
    pub measured_value: f64,
    pub required_value: f64,
    pub tolerance_used: f64,
    pub unit: String,
    pub sample_index: usize,
    pub timestamp: f64,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuleMeasurementRecord {
    pub id: String,
    pub channel: String,
    pub method: String,
    pub measured_value: f64,
    pub unit: String,
    pub sample_index: usize,
    pub timestamp: f64,
    pub method_context: RuleMeasurementMethodContext,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuleMeasurementMethodContext {
    pub source: String,
    pub threshold_v: Option<f64>,
    pub low_threshold_v: Option<f64>,
    pub high_threshold_v: Option<f64>,
    pub state: Option<String>,
    pub expected_state: Option<String>,
    pub event_kind: Option<String>,
    pub direction: Option<String>,
    pub selection: Option<String>,
}

impl Default for RuleMeasurementMethodContext {
    fn default() -> Self {
        Self {
            source: "ferrisoxide-measurements".to_string(),
            threshold_v: None,
            low_threshold_v: None,
            high_threshold_v: None,
            state: None,
            expected_state: None,
            event_kind: None,
            direction: None,
            selection: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuleEvaluation {
    pub results: Vec<RuleResult>,
    pub measurements: Vec<RuleMeasurementRecord>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RuleTolerances {
    pub voltage_v: f64,
    pub time_s: f64,
}

impl Default for RuleTolerances {
    fn default() -> Self {
        Self {
            voltage_v: 0.0,
            time_s: 0.0,
        }
    }
}

impl RuleTolerances {
    pub fn validate(self) -> Result<()> {
        if !self.voltage_v.is_finite() || self.voltage_v < 0.0 {
            return Err(RuleEngineError::InvalidParameter {
                name: "tolerances.voltage_v".to_string(),
                reason: "must be a finite non-negative value".to_string(),
            });
        }
        if !self.time_s.is_finite() || self.time_s < 0.0 {
            return Err(RuleEngineError::InvalidParameter {
                name: "tolerances.time_s".to_string(),
                reason: "must be a finite non-negative value".to_string(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RuleWaveform<'a> {
    pub time: &'a [f64],
    pub channels: &'a [RuleChannel<'a>],
}

impl<'a> RuleWaveform<'a> {
    pub fn channel(&self, name: &str) -> Option<&RuleChannel<'a>> {
        self.channels.iter().find(|channel| channel.name == name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RuleChannel<'a> {
    pub name: &'a str,
    pub samples: &'a [f64],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BorrowedRuleCriterion<'a> {
    pub id: &'a str,
    pub check: BorrowedRuleCriterionCheck<'a>,
}

impl<'a> BorrowedRuleCriterion<'a> {
    pub fn channel(&self) -> &'a str {
        self.check.channel()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BorrowedRuleCriterionCheck<'a> {
    MinimumVoltage {
        channel: &'a str,
        threshold_v: f64,
    },
    MaximumVoltage {
        channel: &'a str,
        threshold_v: f64,
    },
    StateTransitions {
        channel: &'a str,
        threshold_v: f64,
        expected_count: usize,
    },
    PulseWidth {
        channel: &'a str,
        state: SignalState,
        threshold_v: f64,
        min_width_s: Option<f64>,
        max_width_s: Option<f64>,
    },
    TransientDuration {
        channel: &'a str,
        expected_state: SignalState,
        threshold_v: f64,
        max_duration_s: f64,
    },
    TransientEvent {
        channel: &'a str,
        event_kind: &'a str,
        expected_state: SignalState,
        threshold_v: f64,
        max_duration_s: f64,
        start_time_s: Option<f64>,
        end_time_s: Option<f64>,
        arm_after_first_expected_state: bool,
    },
    StableStateDuration {
        channel: &'a str,
        state: SignalState,
        threshold_v: f64,
        min_duration_s: f64,
    },
    RiseFallTime {
        channel: &'a str,
        direction: EdgeDirection,
        low_threshold_v: f64,
        high_threshold_v: f64,
        max_duration_s: f64,
    },
    ResponseLatency {
        source_channel: &'a str,
        target_channel: &'a str,
        source_threshold_v: f64,
        target_threshold_v: f64,
        source_state: SignalState,
        expected_target_state: SignalState,
        max_latency_s: f64,
    },
}

impl<'a> BorrowedRuleCriterionCheck<'a> {
    pub fn channel(&self) -> &'a str {
        match self {
            Self::MinimumVoltage { channel, .. }
            | Self::MaximumVoltage { channel, .. }
            | Self::StateTransitions { channel, .. }
            | Self::PulseWidth { channel, .. }
            | Self::TransientDuration { channel, .. }
            | Self::TransientEvent { channel, .. }
            | Self::StableStateDuration { channel, .. }
            | Self::RiseFallTime { channel, .. } => channel,
            Self::ResponseLatency { target_channel, .. } => target_channel,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RuleSummary<'a> {
    pub criterion_id: &'a str,
    pub outcome: RuleOutcome,
    pub failed_criterion: Option<&'a str>,
    pub channel: &'a str,
    pub measured_value: f64,
    pub required_value: f64,
    pub tolerance_used: f64,
    pub unit: &'static str,
    pub sample_index: usize,
    pub timestamp: f64,
    pub method: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuleCriterion {
    pub id: String,
    pub check: RuleCriterionCheck,
}

impl RuleCriterion {
    pub fn channel(&self) -> &str {
        self.check.channel()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuleCriterionCheck {
    MinimumVoltage {
        channel: String,
        threshold_v: f64,
    },
    MaximumVoltage {
        channel: String,
        threshold_v: f64,
    },
    StateTransitions {
        channel: String,
        threshold_v: f64,
        expected_count: usize,
    },
    PulseWidth {
        channel: String,
        state: SignalState,
        threshold_v: f64,
        min_width_s: Option<f64>,
        max_width_s: Option<f64>,
    },
    TransientDuration {
        channel: String,
        expected_state: SignalState,
        threshold_v: f64,
        max_duration_s: f64,
    },
    TransientEvent {
        channel: String,
        event_kind: String,
        expected_state: SignalState,
        threshold_v: f64,
        max_duration_s: f64,
        start_time_s: Option<f64>,
        end_time_s: Option<f64>,
        arm_after_first_expected_state: bool,
    },
    StableStateDuration {
        channel: String,
        state: SignalState,
        threshold_v: f64,
        min_duration_s: f64,
    },
    RiseFallTime {
        channel: String,
        direction: EdgeDirection,
        low_threshold_v: f64,
        high_threshold_v: f64,
        max_duration_s: f64,
    },
    Measurement {
        channel: String,
        measurement: RuleMeasurementSpec,
        requirement: RuleMeasurementRequirement,
    },
    ResponseLatency {
        source_channel: String,
        target_channel: String,
        source_threshold_v: f64,
        target_threshold_v: f64,
        source_state: SignalState,
        expected_target_state: SignalState,
        max_latency_s: f64,
    },
}

impl RuleCriterionCheck {
    pub fn channel(&self) -> &str {
        match self {
            Self::MinimumVoltage { channel, .. }
            | Self::MaximumVoltage { channel, .. }
            | Self::StateTransitions { channel, .. }
            | Self::PulseWidth { channel, .. }
            | Self::TransientDuration { channel, .. }
            | Self::TransientEvent { channel, .. }
            | Self::StableStateDuration { channel, .. }
            | Self::RiseFallTime { channel, .. }
            | Self::Measurement { channel, .. } => channel,
            Self::ResponseLatency { target_channel, .. } => target_channel,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCriterionOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    EqualTo,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuleMeasurementSpec {
    MinimumSample,
    MaximumSample,
    StateTransitionCount {
        threshold_v: f64,
    },
    PulseWidth {
        state: SignalState,
        threshold_v: f64,
        selection: RuleRunSelectionConfig,
    },
    StableStateDuration {
        state: SignalState,
        threshold_v: f64,
    },
    TransientEventDuration {
        event_kind: String,
        expected_state: SignalState,
        threshold_v: f64,
    },
    RiseTime {
        low_threshold_v: f64,
        high_threshold_v: f64,
    },
    FallTime {
        low_threshold_v: f64,
        high_threshold_v: f64,
    },
}

impl RuleMeasurementSpec {
    pub fn is_time_dependent(&self) -> bool {
        matches!(
            self,
            Self::PulseWidth { .. }
                | Self::StableStateDuration { .. }
                | Self::TransientEventDuration { .. }
                | Self::RiseTime { .. }
                | Self::FallTime { .. }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RuleMeasurementRequirement {
    pub operator: RuleCriterionOperator,
    pub value: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleRunSelectionConfig {
    Shortest,
    Longest,
}

impl RuleRunSelectionConfig {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shortest => "shortest",
            Self::Longest => "longest",
        }
    }
}

pub fn evaluate_rule_set(
    waveform: RuleWaveform<'_>,
    criteria: &[RuleCriterion],
    tolerances: RuleTolerances,
) -> Result<RuleEvaluation> {
    tolerances.validate()?;
    validate_time_axis_for_criteria(waveform, criteria)?;
    let evaluated = criteria
        .iter()
        .map(|criterion| evaluate_criterion(waveform, criterion, tolerances))
        .collect::<Result<Vec<_>>>()?;
    Ok(RuleEvaluation {
        results: evaluated
            .iter()
            .map(|evaluation| evaluation.result.clone())
            .collect(),
        measurements: evaluated
            .into_iter()
            .map(|evaluation| evaluation.measurement)
            .collect(),
    })
}

pub fn evaluate_borrowed_rule<'a>(
    waveform: RuleWaveform<'_>,
    criterion: BorrowedRuleCriterion<'a>,
    tolerances: RuleTolerances,
) -> BorrowedRuleResult<'a, RuleSummary<'a>> {
    validate_borrowed_tolerances(tolerances)?;
    validate_time_axis_for_borrowed_rule(waveform, criterion)?;
    let channel =
        waveform
            .channel(criterion.channel())
            .ok_or(BorrowedRuleError::MissingChannel {
                channel: criterion.channel(),
            })?;

    match criterion.check {
        BorrowedRuleCriterionCheck::MinimumVoltage { threshold_v, .. } => {
            let measurement = minimum_sample(waveform.time, channel.samples)
                .map_err(borrowed_measurement_error)?;
            let outcome = if measurement.value + tolerances.voltage_v >= threshold_v {
                RuleOutcome::Pass
            } else {
                RuleOutcome::Fail
            };
            Ok(summary(
                criterion,
                outcome,
                SummaryEvidence {
                    measured_value: measurement.value,
                    required_value: threshold_v,
                    tolerance_used: tolerances.voltage_v,
                    unit: "V",
                    sample_index: measurement.sample_index,
                    timestamp: measurement.timestamp,
                    method: "minimum_sample",
                },
            ))
        }
        BorrowedRuleCriterionCheck::MaximumVoltage { threshold_v, .. } => {
            let measurement = maximum_sample(waveform.time, channel.samples)
                .map_err(borrowed_measurement_error)?;
            let outcome = if measurement.value - tolerances.voltage_v <= threshold_v {
                RuleOutcome::Pass
            } else {
                RuleOutcome::Fail
            };
            Ok(summary(
                criterion,
                outcome,
                SummaryEvidence {
                    measured_value: measurement.value,
                    required_value: threshold_v,
                    tolerance_used: tolerances.voltage_v,
                    unit: "V",
                    sample_index: measurement.sample_index,
                    timestamp: measurement.timestamp,
                    method: "maximum_sample",
                },
            ))
        }
        BorrowedRuleCriterionCheck::StateTransitions {
            threshold_v,
            expected_count,
            ..
        } => {
            let transitions = count_state_transitions(
                waveform.time,
                channel.samples,
                threshold_v,
                tolerances.voltage_v,
            )
            .map_err(borrowed_measurement_error)?;
            let measured = transitions.count;
            let outcome = if measured == expected_count {
                RuleOutcome::Pass
            } else {
                RuleOutcome::Fail
            };
            Ok(summary(
                criterion,
                outcome,
                SummaryEvidence {
                    measured_value: measured as f64,
                    required_value: expected_count as f64,
                    tolerance_used: 0.0,
                    unit: "transitions",
                    sample_index: transitions.first_index.unwrap_or(0),
                    timestamp: transitions.first_timestamp.unwrap_or(waveform.time[0]),
                    method: "state_transition_count",
                },
            ))
        }
        BorrowedRuleCriterionCheck::PulseWidth {
            state,
            threshold_v,
            min_width_s,
            max_width_s,
            ..
        } => evaluate_borrowed_pulse_width(
            waveform,
            channel,
            criterion,
            BorrowedPulseWidthSpec {
                state,
                threshold_v,
                min_width_s,
                max_width_s,
            },
            tolerances,
        ),
        BorrowedRuleCriterionCheck::TransientDuration {
            expected_state,
            threshold_v,
            max_duration_s,
            ..
        } => evaluate_borrowed_transient_duration(
            waveform,
            channel,
            criterion,
            BorrowedTransientDurationSpec {
                expected_state,
                threshold_v,
                max_duration_s,
                window: TimeWindow::full(),
                arm_after_first_expected_state: false,
            },
            tolerances,
        ),
        BorrowedRuleCriterionCheck::TransientEvent {
            expected_state,
            threshold_v,
            max_duration_s,
            start_time_s,
            end_time_s,
            arm_after_first_expected_state,
            ..
        } => evaluate_borrowed_transient_duration(
            waveform,
            channel,
            criterion,
            BorrowedTransientDurationSpec {
                expected_state,
                threshold_v,
                max_duration_s,
                window: TimeWindow {
                    start_time_s,
                    end_time_s,
                },
                arm_after_first_expected_state,
            },
            tolerances,
        ),
        BorrowedRuleCriterionCheck::StableStateDuration {
            state,
            threshold_v,
            min_duration_s,
            ..
        } => {
            let longest = state_run_extremum(
                waveform.time,
                channel.samples,
                state,
                threshold_v,
                tolerances.voltage_v,
                RunSelection::Longest,
            )
            .map_err(borrowed_measurement_error)?;
            let (measured, sample_index, timestamp) = longest
                .map(|run| (run.duration_s, run.start_index, run.start_time))
                .unwrap_or((0.0, 0, waveform.time[0]));
            let outcome = if measured + tolerances.time_s + FLOAT_TOLERANCE >= min_duration_s {
                RuleOutcome::Pass
            } else {
                RuleOutcome::Fail
            };
            Ok(summary(
                criterion,
                outcome,
                SummaryEvidence {
                    measured_value: measured,
                    required_value: min_duration_s,
                    tolerance_used: tolerances.time_s,
                    unit: "s",
                    sample_index,
                    timestamp,
                    method: "state_run_duration",
                },
            ))
        }
        BorrowedRuleCriterionCheck::RiseFallTime {
            direction,
            low_threshold_v,
            high_threshold_v,
            max_duration_s,
            ..
        } => evaluate_borrowed_rise_fall_time(
            waveform,
            channel,
            criterion,
            BorrowedRiseFallTimeSpec {
                direction,
                low_threshold_v,
                high_threshold_v,
                max_duration_s,
            },
            tolerances,
        ),
        BorrowedRuleCriterionCheck::ResponseLatency {
            source_channel,
            source_threshold_v,
            target_threshold_v,
            source_state,
            expected_target_state,
            max_latency_s,
            ..
        } => evaluate_borrowed_response_latency(
            waveform,
            channel,
            criterion,
            BorrowedResponseLatencySpec {
                source_channel,
                source_threshold_v,
                target_threshold_v,
                source_state,
                expected_target_state,
                max_latency_s,
            },
            tolerances,
        ),
    }
}

fn validate_time_axis_for_criteria(
    waveform: RuleWaveform<'_>,
    criteria: &[RuleCriterion],
) -> Result<()> {
    if !criteria.iter().any(is_time_dependent) {
        return Ok(());
    }

    for (index, pair) in waveform.time.windows(2).enumerate() {
        if pair[1] <= pair[0] {
            return Err(RuleEngineError::InvalidWaveform {
                reason: format!(
                    "time samples must be strictly increasing for duration criteria; sample {}={} is not greater than sample {}={}",
                    index + 1,
                    pair[1],
                    index,
                    pair[0]
                ),
            });
        }
    }
    Ok(())
}

fn validate_borrowed_tolerances<'a>(tolerances: RuleTolerances) -> BorrowedRuleResult<'a, ()> {
    if !tolerances.voltage_v.is_finite() || tolerances.voltage_v < 0.0 {
        return Err(BorrowedRuleError::InvalidParameter {
            name: "tolerances.voltage_v",
            reason: "must be a finite non-negative value",
        });
    }
    if !tolerances.time_s.is_finite() || tolerances.time_s < 0.0 {
        return Err(BorrowedRuleError::InvalidParameter {
            name: "tolerances.time_s",
            reason: "must be a finite non-negative value",
        });
    }
    Ok(())
}

fn validate_time_axis_for_borrowed_rule<'a>(
    waveform: RuleWaveform<'_>,
    criterion: BorrowedRuleCriterion<'_>,
) -> BorrowedRuleResult<'a, ()> {
    if !is_borrowed_time_dependent(criterion) {
        return Ok(());
    }

    for pair in waveform.time.windows(2) {
        if pair[1] <= pair[0] {
            return Err(BorrowedRuleError::InvalidWaveform {
                reason: "time samples must be strictly increasing for duration criteria",
            });
        }
    }
    Ok(())
}

fn is_time_dependent(criterion: &RuleCriterion) -> bool {
    matches!(
        &criterion.check,
        RuleCriterionCheck::PulseWidth { .. }
            | RuleCriterionCheck::TransientDuration { .. }
            | RuleCriterionCheck::TransientEvent { .. }
            | RuleCriterionCheck::StableStateDuration { .. }
            | RuleCriterionCheck::RiseFallTime { .. }
            | RuleCriterionCheck::ResponseLatency { .. }
    ) || matches!(
        &criterion.check,
        RuleCriterionCheck::Measurement { measurement, .. } if measurement.is_time_dependent()
    )
}

fn is_borrowed_time_dependent(criterion: BorrowedRuleCriterion<'_>) -> bool {
    matches!(
        criterion.check,
        BorrowedRuleCriterionCheck::PulseWidth { .. }
            | BorrowedRuleCriterionCheck::TransientDuration { .. }
            | BorrowedRuleCriterionCheck::TransientEvent { .. }
            | BorrowedRuleCriterionCheck::StableStateDuration { .. }
            | BorrowedRuleCriterionCheck::RiseFallTime { .. }
            | BorrowedRuleCriterionCheck::ResponseLatency { .. }
    )
}

struct SummaryEvidence {
    measured_value: f64,
    required_value: f64,
    tolerance_used: f64,
    unit: &'static str,
    sample_index: usize,
    timestamp: f64,
    method: &'static str,
}

fn summary<'a>(
    criterion: BorrowedRuleCriterion<'a>,
    outcome: RuleOutcome,
    evidence: SummaryEvidence,
) -> RuleSummary<'a> {
    RuleSummary {
        criterion_id: criterion.id,
        outcome,
        failed_criterion: (outcome == RuleOutcome::Fail).then_some(criterion.id),
        channel: criterion.channel(),
        measured_value: round_evidence(evidence.measured_value),
        required_value: round_evidence(evidence.required_value),
        tolerance_used: round_evidence(evidence.tolerance_used),
        unit: evidence.unit,
        sample_index: evidence.sample_index,
        timestamp: round_evidence(evidence.timestamp),
        method: evidence.method,
    }
}

#[derive(Debug, Clone, Copy)]
struct BorrowedPulseWidthSpec {
    state: SignalState,
    threshold_v: f64,
    min_width_s: Option<f64>,
    max_width_s: Option<f64>,
}

fn evaluate_borrowed_pulse_width<'a>(
    waveform: RuleWaveform<'_>,
    channel: &RuleChannel<'_>,
    criterion: BorrowedRuleCriterion<'a>,
    spec: BorrowedPulseWidthSpec,
    tolerances: RuleTolerances,
) -> BorrowedRuleResult<'a, RuleSummary<'a>> {
    if spec.min_width_s.is_none() && spec.max_width_s.is_none() {
        return Err(BorrowedRuleError::InvalidParameter {
            name: "criteria.pulse_width",
            reason: "min_width_s or max_width_s is required",
        });
    }

    let shortest = if spec.min_width_s.is_some() {
        state_run_extremum(
            waveform.time,
            channel.samples,
            spec.state,
            spec.threshold_v,
            tolerances.voltage_v,
            RunSelection::Shortest,
        )
        .map_err(borrowed_measurement_error)?
    } else {
        None
    };
    let longest = if spec.max_width_s.is_some() {
        state_run_extremum(
            waveform.time,
            channel.samples,
            spec.state,
            spec.threshold_v,
            tolerances.voltage_v,
            RunSelection::Longest,
        )
        .map_err(borrowed_measurement_error)?
    } else {
        None
    };

    if shortest.or(longest).is_none() {
        return Ok(summary(
            criterion,
            RuleOutcome::Fail,
            SummaryEvidence {
                measured_value: 0.0,
                required_value: spec.min_width_s.or(spec.max_width_s).unwrap_or_default(),
                tolerance_used: tolerances.time_s,
                unit: "s",
                sample_index: 0,
                timestamp: waveform.time[0],
                method: "state_run_duration",
            },
        ));
    }

    if let Some(min_width_s) = spec.min_width_s {
        let shortest = shortest.expect("state run should exist after empty check");
        if shortest.duration_s + tolerances.time_s + FLOAT_TOLERANCE < min_width_s {
            return Ok(summary(
                criterion,
                RuleOutcome::Fail,
                SummaryEvidence {
                    measured_value: shortest.duration_s,
                    required_value: min_width_s,
                    tolerance_used: tolerances.time_s,
                    unit: "s",
                    sample_index: shortest.start_index,
                    timestamp: shortest.start_time,
                    method: "state_run_duration",
                },
            ));
        }
    }

    if let Some(max_width_s) = spec.max_width_s {
        let longest = longest.expect("state run should exist after empty check");
        if longest.duration_s - tolerances.time_s - FLOAT_TOLERANCE > max_width_s {
            return Ok(summary(
                criterion,
                RuleOutcome::Fail,
                SummaryEvidence {
                    measured_value: longest.duration_s,
                    required_value: max_width_s,
                    tolerance_used: tolerances.time_s,
                    unit: "s",
                    sample_index: longest.start_index,
                    timestamp: longest.start_time,
                    method: "state_run_duration",
                },
            ));
        }
    }

    let measured = shortest
        .or(longest)
        .expect("state run should exist after empty check");
    Ok(summary(
        criterion,
        RuleOutcome::Pass,
        SummaryEvidence {
            measured_value: measured.duration_s,
            required_value: spec.min_width_s.or(spec.max_width_s).unwrap_or_default(),
            tolerance_used: tolerances.time_s,
            unit: "s",
            sample_index: measured.start_index,
            timestamp: measured.start_time,
            method: "state_run_duration",
        },
    ))
}

#[derive(Debug, Clone, Copy)]
struct TimeWindow {
    start_time_s: Option<f64>,
    end_time_s: Option<f64>,
}

impl TimeWindow {
    const fn full() -> Self {
        Self {
            start_time_s: None,
            end_time_s: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct StateRunEvidence {
    duration_s: f64,
    start_index: usize,
    start_time: f64,
}

#[derive(Debug, Clone, Copy)]
struct BorrowedTransientDurationSpec {
    expected_state: SignalState,
    threshold_v: f64,
    max_duration_s: f64,
    window: TimeWindow,
    arm_after_first_expected_state: bool,
}

fn evaluate_borrowed_transient_duration<'a>(
    waveform: RuleWaveform<'_>,
    channel: &RuleChannel<'_>,
    criterion: BorrowedRuleCriterion<'a>,
    spec: BorrowedTransientDurationSpec,
    tolerances: RuleTolerances,
) -> BorrowedRuleResult<'a, RuleSummary<'a>> {
    let transient_state = spec.expected_state.opposite();
    let window = if spec.arm_after_first_expected_state {
        arm_window_after_expected_state(
            waveform.time,
            channel.samples,
            spec.expected_state,
            spec.threshold_v,
            tolerances.voltage_v,
            spec.window,
        )
        .map_err(borrowed_measurement_error)?
    } else {
        spec.window
    };
    let longest = longest_state_run_in_window(
        waveform.time,
        channel.samples,
        transient_state,
        spec.threshold_v,
        tolerances.voltage_v,
        window,
    )
    .map_err(borrowed_measurement_error)?;
    let fallback = window_reference(waveform.time, window);
    let (measured, sample_index, timestamp) = longest
        .map(|run| (run.duration_s, run.start_index, run.start_time))
        .unwrap_or((0.0, fallback.0, fallback.1));
    let outcome = if measured <= spec.max_duration_s + tolerances.time_s + FLOAT_TOLERANCE {
        RuleOutcome::Pass
    } else {
        RuleOutcome::Fail
    };

    Ok(summary(
        criterion,
        outcome,
        SummaryEvidence {
            measured_value: measured,
            required_value: spec.max_duration_s,
            tolerance_used: tolerances.time_s,
            unit: "s",
            sample_index,
            timestamp,
            method: "state_run_duration",
        },
    ))
}

#[derive(Debug, Clone, Copy)]
struct BorrowedResponseLatencySpec<'a> {
    source_channel: &'a str,
    source_threshold_v: f64,
    target_threshold_v: f64,
    source_state: SignalState,
    expected_target_state: SignalState,
    max_latency_s: f64,
}

fn evaluate_borrowed_response_latency<'a>(
    waveform: RuleWaveform<'_>,
    target_channel: &RuleChannel<'_>,
    criterion: BorrowedRuleCriterion<'a>,
    spec: BorrowedResponseLatencySpec<'a>,
    tolerances: RuleTolerances,
) -> BorrowedRuleResult<'a, RuleSummary<'a>> {
    let source_channel =
        waveform
            .channel(spec.source_channel)
            .ok_or(BorrowedRuleError::MissingChannel {
                channel: spec.source_channel,
            })?;
    let evidence = measure_response_latency(
        waveform.time,
        source_channel.samples,
        target_channel.samples,
        ResponseLatencyMeasurementSpec {
            source_threshold_v: spec.source_threshold_v,
            target_threshold_v: spec.target_threshold_v,
            source_state: spec.source_state,
            expected_target_state: spec.expected_target_state,
        },
        tolerances.voltage_v,
    )
    .map_err(borrowed_measurement_error)?;
    let outcome = if evidence.observed
        && evidence.latency_s <= spec.max_latency_s + tolerances.time_s + FLOAT_TOLERANCE
    {
        RuleOutcome::Pass
    } else {
        RuleOutcome::Fail
    };

    Ok(summary(
        criterion,
        outcome,
        SummaryEvidence {
            measured_value: evidence.latency_s,
            required_value: spec.max_latency_s,
            tolerance_used: tolerances.time_s,
            unit: "s",
            sample_index: evidence.sample_index,
            timestamp: evidence.timestamp,
            method: "response_latency",
        },
    ))
}

#[derive(Debug, Clone, Copy)]
struct BorrowedRiseFallTimeSpec {
    direction: EdgeDirection,
    low_threshold_v: f64,
    high_threshold_v: f64,
    max_duration_s: f64,
}

fn evaluate_borrowed_rise_fall_time<'a>(
    waveform: RuleWaveform<'_>,
    channel: &RuleChannel<'_>,
    criterion: BorrowedRuleCriterion<'a>,
    spec: BorrowedRiseFallTimeSpec,
    tolerances: RuleTolerances,
) -> BorrowedRuleResult<'a, RuleSummary<'a>> {
    if spec.low_threshold_v >= spec.high_threshold_v {
        return Err(BorrowedRuleError::InvalidParameter {
            name: "criteria.low_threshold_v",
            reason: "must be lower than high_threshold_v",
        });
    }

    let measurement = match spec.direction {
        EdgeDirection::Rise => measure_rise_time(
            waveform.time,
            channel.samples,
            spec.low_threshold_v,
            spec.high_threshold_v,
            tolerances.voltage_v,
        ),
        EdgeDirection::Fall => measure_fall_time(
            waveform.time,
            channel.samples,
            spec.low_threshold_v,
            spec.high_threshold_v,
            tolerances.voltage_v,
        ),
    }
    .map_err(borrowed_measurement_error)?;
    let (measured, sample_index, timestamp, observed) = measurement
        .map(|transition| {
            (
                transition.duration_s,
                transition.end_index,
                transition.end_time,
                true,
            )
        })
        .unwrap_or((f64::INFINITY, 0, waveform.time[0], false));
    let outcome =
        if observed && measured <= spec.max_duration_s + tolerances.time_s + FLOAT_TOLERANCE {
            RuleOutcome::Pass
        } else {
            RuleOutcome::Fail
        };

    Ok(summary(
        criterion,
        outcome,
        SummaryEvidence {
            measured_value: measured,
            required_value: spec.max_duration_s,
            tolerance_used: tolerances.time_s,
            unit: "s",
            sample_index,
            timestamp,
            method: "edge_time",
        },
    ))
}

fn evaluate_criterion(
    waveform: RuleWaveform<'_>,
    criterion: &RuleCriterion,
    tolerances: RuleTolerances,
) -> Result<EvaluatedCriterion> {
    let channel =
        waveform
            .channel(criterion.channel())
            .ok_or_else(|| RuleEngineError::MissingChannel {
                channel: criterion.channel().to_string(),
            })?;

    match &criterion.check {
        RuleCriterionCheck::MinimumVoltage { threshold_v, .. } => {
            let measurement =
                minimum_sample(waveform.time, channel.samples).map_err(measurement_error)?;
            let outcome = if measurement.value + tolerances.voltage_v >= *threshold_v {
                RuleOutcome::Pass
            } else {
                RuleOutcome::Fail
            };
            Ok(result(
                criterion,
                outcome,
                Evidence {
                    measured_value: measurement.value,
                    required_value: *threshold_v,
                    tolerance_used: tolerances.voltage_v,
                    unit: "V",
                    sample_index: measurement.sample_index,
                    timestamp: measurement.timestamp,
                    reason: format!("minimum observed voltage was {:.6} V", measurement.value),
                    method: "minimum_sample",
                    method_context: RuleMeasurementMethodContext::default(),
                },
            ))
        }
        RuleCriterionCheck::MaximumVoltage { threshold_v, .. } => {
            let measurement =
                maximum_sample(waveform.time, channel.samples).map_err(measurement_error)?;
            let outcome = if measurement.value - tolerances.voltage_v <= *threshold_v {
                RuleOutcome::Pass
            } else {
                RuleOutcome::Fail
            };
            Ok(result(
                criterion,
                outcome,
                Evidence {
                    measured_value: measurement.value,
                    required_value: *threshold_v,
                    tolerance_used: tolerances.voltage_v,
                    unit: "V",
                    sample_index: measurement.sample_index,
                    timestamp: measurement.timestamp,
                    reason: format!("maximum observed voltage was {:.6} V", measurement.value),
                    method: "maximum_sample",
                    method_context: RuleMeasurementMethodContext::default(),
                },
            ))
        }
        RuleCriterionCheck::StateTransitions {
            threshold_v,
            expected_count,
            ..
        } => evaluate_state_transitions(
            waveform,
            channel,
            criterion,
            *threshold_v,
            *expected_count,
            tolerances,
        ),
        RuleCriterionCheck::PulseWidth {
            state,
            threshold_v,
            min_width_s,
            max_width_s,
            ..
        } => evaluate_pulse_width(
            waveform,
            channel,
            criterion,
            PulseWidthSpec {
                state: *state,
                threshold_v: *threshold_v,
                min_width_s: *min_width_s,
                max_width_s: *max_width_s,
            },
            tolerances,
        ),
        RuleCriterionCheck::TransientDuration {
            expected_state,
            threshold_v,
            max_duration_s,
            ..
        } => evaluate_transient_duration(
            waveform,
            channel,
            criterion,
            TransientDurationSpec {
                expected_state: *expected_state,
                threshold_v: *threshold_v,
                max_duration_s: *max_duration_s,
                event_kind: "transient",
                window: TimeWindow::full(),
                arm_after_first_expected_state: false,
            },
            tolerances,
        ),
        RuleCriterionCheck::TransientEvent {
            event_kind,
            expected_state,
            threshold_v,
            max_duration_s,
            start_time_s,
            end_time_s,
            arm_after_first_expected_state,
            ..
        } => evaluate_transient_duration(
            waveform,
            channel,
            criterion,
            TransientDurationSpec {
                expected_state: *expected_state,
                threshold_v: *threshold_v,
                max_duration_s: *max_duration_s,
                event_kind,
                window: TimeWindow {
                    start_time_s: *start_time_s,
                    end_time_s: *end_time_s,
                },
                arm_after_first_expected_state: *arm_after_first_expected_state,
            },
            tolerances,
        ),
        RuleCriterionCheck::StableStateDuration {
            state,
            threshold_v,
            min_duration_s,
            ..
        } => evaluate_stable_state_duration(
            waveform,
            channel,
            criterion,
            *state,
            *threshold_v,
            *min_duration_s,
            tolerances,
        ),
        RuleCriterionCheck::RiseFallTime {
            direction,
            low_threshold_v,
            high_threshold_v,
            max_duration_s,
            ..
        } => evaluate_rise_fall_time(
            waveform,
            channel,
            criterion,
            RiseFallTimeSpec {
                direction: *direction,
                low_threshold_v: *low_threshold_v,
                high_threshold_v: *high_threshold_v,
                max_duration_s: *max_duration_s,
            },
            tolerances,
        ),
        RuleCriterionCheck::Measurement {
            measurement,
            requirement,
            ..
        } => evaluate_measurement_criterion(
            waveform,
            channel,
            criterion,
            measurement,
            requirement,
            tolerances,
        ),
        RuleCriterionCheck::ResponseLatency {
            source_channel,
            source_threshold_v,
            target_threshold_v,
            source_state,
            expected_target_state,
            max_latency_s,
            ..
        } => evaluate_response_latency(
            waveform,
            channel,
            criterion,
            ResponseLatencySpec {
                source_channel,
                source_threshold_v: *source_threshold_v,
                target_threshold_v: *target_threshold_v,
                source_state: *source_state,
                expected_target_state: *expected_target_state,
                max_latency_s: *max_latency_s,
            },
            tolerances,
        ),
    }
}

#[derive(Debug, Clone, PartialEq)]
struct EvaluatedCriterion {
    result: RuleResult,
    measurement: RuleMeasurementRecord,
}

fn result(
    criterion: &RuleCriterion,
    outcome: RuleOutcome,
    evidence: Evidence,
) -> EvaluatedCriterion {
    let measurement_id = format!("{}_measurement", criterion.id);
    let measured_value = round_evidence(evidence.measured_value);
    let timestamp = round_evidence(evidence.timestamp);
    let result = RuleResult {
        criterion_id: criterion.id.clone(),
        outcome,
        failed_criterion: (outcome == RuleOutcome::Fail).then(|| criterion.id.clone()),
        measurement_id: measurement_id.clone(),
        channel: criterion.channel().to_string(),
        measured_value,
        required_value: round_evidence(evidence.required_value),
        tolerance_used: round_evidence(evidence.tolerance_used),
        unit: evidence.unit.to_string(),
        sample_index: evidence.sample_index,
        timestamp,
        reason: evidence.reason,
    };
    let measurement = RuleMeasurementRecord {
        id: measurement_id,
        channel: criterion.channel().to_string(),
        method: evidence.method.to_string(),
        measured_value,
        unit: evidence.unit.to_string(),
        sample_index: evidence.sample_index,
        timestamp,
        method_context: evidence.method_context,
    };
    EvaluatedCriterion {
        result,
        measurement,
    }
}

struct Evidence {
    measured_value: f64,
    required_value: f64,
    tolerance_used: f64,
    unit: &'static str,
    sample_index: usize,
    timestamp: f64,
    reason: String,
    method: &'static str,
    method_context: RuleMeasurementMethodContext,
}

fn round_evidence(value: f64) -> f64 {
    if value.is_finite() {
        let scaled = value * 1_000_000_000.0;
        let rounded = if scaled >= 0.0 {
            (scaled + 0.5) as i64
        } else {
            (scaled - 0.5) as i64
        };
        rounded as f64 / 1_000_000_000.0
    } else {
        value
    }
}

fn threshold_context(threshold_v: f64) -> RuleMeasurementMethodContext {
    RuleMeasurementMethodContext {
        threshold_v: Some(round_evidence(threshold_v)),
        ..RuleMeasurementMethodContext::default()
    }
}

fn state_run_context(
    threshold_v: f64,
    state: SignalState,
    selection: &'static str,
) -> RuleMeasurementMethodContext {
    RuleMeasurementMethodContext {
        threshold_v: Some(round_evidence(threshold_v)),
        state: Some(state.as_str().to_string()),
        selection: Some(selection.to_string()),
        ..RuleMeasurementMethodContext::default()
    }
}

fn transient_context(
    threshold_v: f64,
    transient_state: SignalState,
    expected_state: SignalState,
    event_kind: &str,
) -> RuleMeasurementMethodContext {
    RuleMeasurementMethodContext {
        threshold_v: Some(round_evidence(threshold_v)),
        state: Some(transient_state.as_str().to_string()),
        expected_state: Some(expected_state.as_str().to_string()),
        event_kind: Some(event_kind.to_string()),
        selection: Some("longest".to_string()),
        ..RuleMeasurementMethodContext::default()
    }
}

fn edge_context(
    low_threshold_v: f64,
    high_threshold_v: f64,
    direction: EdgeDirection,
) -> RuleMeasurementMethodContext {
    RuleMeasurementMethodContext {
        low_threshold_v: Some(round_evidence(low_threshold_v)),
        high_threshold_v: Some(round_evidence(high_threshold_v)),
        direction: Some(direction.as_str().to_string()),
        ..RuleMeasurementMethodContext::default()
    }
}

fn evaluate_state_transitions(
    waveform: RuleWaveform<'_>,
    channel: &RuleChannel<'_>,
    criterion: &RuleCriterion,
    threshold_v: f64,
    expected_count: usize,
    tolerances: RuleTolerances,
) -> Result<EvaluatedCriterion> {
    let transitions = count_state_transitions(
        waveform.time,
        channel.samples,
        threshold_v,
        tolerances.voltage_v,
    )
    .map_err(measurement_error)?;
    let measured = transitions.count;
    let sample_index = transitions.first_index.unwrap_or(0);
    let timestamp = transitions.first_timestamp.unwrap_or(waveform.time[0]);
    let outcome = if measured == expected_count {
        RuleOutcome::Pass
    } else {
        RuleOutcome::Fail
    };

    Ok(result(
        criterion,
        outcome,
        Evidence {
            measured_value: measured as f64,
            required_value: expected_count as f64,
            tolerance_used: 0.0,
            unit: "transitions",
            sample_index,
            timestamp,
            reason: format!("observed {measured} state transitions at {threshold_v:.6} V"),
            method: "state_transition_count",
            method_context: threshold_context(threshold_v),
        },
    ))
}

#[derive(Debug, Clone, Copy)]
struct PulseWidthSpec {
    state: SignalState,
    threshold_v: f64,
    min_width_s: Option<f64>,
    max_width_s: Option<f64>,
}

fn evaluate_pulse_width(
    waveform: RuleWaveform<'_>,
    channel: &RuleChannel<'_>,
    criterion: &RuleCriterion,
    spec: PulseWidthSpec,
    tolerances: RuleTolerances,
) -> Result<EvaluatedCriterion> {
    if spec.min_width_s.is_none() && spec.max_width_s.is_none() {
        return Err(RuleEngineError::InvalidParameter {
            name: "criteria.pulse_width".to_string(),
            reason: "min_width_s or max_width_s is required".to_string(),
        });
    }

    let shortest = if spec.min_width_s.is_some() {
        state_run_extremum(
            waveform.time,
            channel.samples,
            spec.state,
            spec.threshold_v,
            tolerances.voltage_v,
            RunSelection::Shortest,
        )
        .map_err(measurement_error)?
    } else {
        None
    };
    let longest = if spec.max_width_s.is_some() {
        state_run_extremum(
            waveform.time,
            channel.samples,
            spec.state,
            spec.threshold_v,
            tolerances.voltage_v,
            RunSelection::Longest,
        )
        .map_err(measurement_error)?
    } else {
        None
    };

    if shortest.or(longest).is_none() {
        return Ok(result(
            criterion,
            RuleOutcome::Fail,
            Evidence {
                measured_value: 0.0,
                required_value: spec.min_width_s.or(spec.max_width_s).unwrap_or_default(),
                tolerance_used: tolerances.time_s,
                unit: "s",
                sample_index: 0,
                timestamp: waveform.time[0],
                reason: format!("no {} pulse was observed", spec.state.as_str()),
                method: "state_run_duration",
                method_context: state_run_context(
                    spec.threshold_v,
                    spec.state,
                    if spec.min_width_s.is_some() {
                        "shortest"
                    } else {
                        "longest"
                    },
                ),
            },
        ));
    }

    if let Some(min_width_s) = spec.min_width_s {
        let shortest = shortest.expect("state run should exist after empty check");
        if shortest.duration_s + tolerances.time_s + FLOAT_TOLERANCE < min_width_s {
            return Ok(result(
                criterion,
                RuleOutcome::Fail,
                Evidence {
                    measured_value: shortest.duration_s,
                    required_value: min_width_s,
                    tolerance_used: tolerances.time_s,
                    unit: "s",
                    sample_index: shortest.start_index,
                    timestamp: shortest.start_time,
                    reason: format!(
                        "shortest {} pulse width was {:.6} s",
                        spec.state.as_str(),
                        shortest.duration_s
                    ),
                    method: "state_run_duration",
                    method_context: state_run_context(spec.threshold_v, spec.state, "shortest"),
                },
            ));
        }
    }

    if let Some(max_width_s) = spec.max_width_s {
        let longest = longest.expect("state run should exist after empty check");
        if longest.duration_s - tolerances.time_s - FLOAT_TOLERANCE > max_width_s {
            return Ok(result(
                criterion,
                RuleOutcome::Fail,
                Evidence {
                    measured_value: longest.duration_s,
                    required_value: max_width_s,
                    tolerance_used: tolerances.time_s,
                    unit: "s",
                    sample_index: longest.start_index,
                    timestamp: longest.start_time,
                    reason: format!(
                        "longest {} pulse width was {:.6} s",
                        spec.state.as_str(),
                        longest.duration_s
                    ),
                    method: "state_run_duration",
                    method_context: state_run_context(spec.threshold_v, spec.state, "longest"),
                },
            ));
        }
    }

    let measured = shortest
        .or(longest)
        .expect("state run should exist after empty check");
    Ok(result(
        criterion,
        RuleOutcome::Pass,
        Evidence {
            measured_value: measured.duration_s,
            required_value: spec.min_width_s.or(spec.max_width_s).unwrap_or_default(),
            tolerance_used: tolerances.time_s,
            unit: "s",
            sample_index: measured.start_index,
            timestamp: measured.start_time,
            reason: format!(
                "{} pulse width met configured limits; measured {:.6} s",
                spec.state.as_str(),
                measured.duration_s
            ),
            method: "state_run_duration",
            method_context: state_run_context(
                spec.threshold_v,
                spec.state,
                if spec.min_width_s.is_some() {
                    "shortest"
                } else {
                    "longest"
                },
            ),
        },
    ))
}

#[derive(Debug, Clone)]
struct TransientDurationSpec<'a> {
    expected_state: SignalState,
    threshold_v: f64,
    max_duration_s: f64,
    event_kind: &'a str,
    window: TimeWindow,
    arm_after_first_expected_state: bool,
}

fn evaluate_transient_duration(
    waveform: RuleWaveform<'_>,
    channel: &RuleChannel<'_>,
    criterion: &RuleCriterion,
    spec: TransientDurationSpec<'_>,
    tolerances: RuleTolerances,
) -> Result<EvaluatedCriterion> {
    let transient_state = spec.expected_state.opposite();
    let window = if spec.arm_after_first_expected_state {
        arm_window_after_expected_state(
            waveform.time,
            channel.samples,
            spec.expected_state,
            spec.threshold_v,
            tolerances.voltage_v,
            spec.window,
        )
        .map_err(measurement_error)?
    } else {
        spec.window
    };
    let longest = longest_state_run_in_window(
        waveform.time,
        channel.samples,
        transient_state,
        spec.threshold_v,
        tolerances.voltage_v,
        window,
    )
    .map_err(measurement_error)?;
    let fallback = window_reference(waveform.time, window);

    let (measured, sample_index, timestamp) = longest
        .map(|run| (run.duration_s, run.start_index, run.start_time))
        .unwrap_or((0.0, fallback.0, fallback.1));
    let outcome = if measured <= spec.max_duration_s + tolerances.time_s + FLOAT_TOLERANCE {
        RuleOutcome::Pass
    } else {
        RuleOutcome::Fail
    };

    Ok(result(
        criterion,
        outcome,
        Evidence {
            measured_value: measured,
            required_value: spec.max_duration_s,
            tolerance_used: tolerances.time_s,
            unit: "s",
            sample_index,
            timestamp,
            reason: format!(
                "longest unintended {} {} duration was {:.6} s",
                transient_state.as_str(),
                spec.event_kind,
                measured
            ),
            method: "state_run_duration",
            method_context: transient_context(
                spec.threshold_v,
                transient_state,
                spec.expected_state,
                spec.event_kind,
            ),
        },
    ))
}

#[derive(Debug, Clone, Copy)]
struct ResponseLatencySpec<'a> {
    source_channel: &'a str,
    source_threshold_v: f64,
    target_threshold_v: f64,
    source_state: SignalState,
    expected_target_state: SignalState,
    max_latency_s: f64,
}

fn evaluate_response_latency(
    waveform: RuleWaveform<'_>,
    target_channel: &RuleChannel<'_>,
    criterion: &RuleCriterion,
    spec: ResponseLatencySpec<'_>,
    tolerances: RuleTolerances,
) -> Result<EvaluatedCriterion> {
    let source_channel =
        waveform
            .channel(spec.source_channel)
            .ok_or_else(|| RuleEngineError::MissingChannel {
                channel: spec.source_channel.to_string(),
            })?;
    let latency = measure_response_latency(
        waveform.time,
        source_channel.samples,
        target_channel.samples,
        ResponseLatencyMeasurementSpec {
            source_threshold_v: spec.source_threshold_v,
            target_threshold_v: spec.target_threshold_v,
            source_state: spec.source_state,
            expected_target_state: spec.expected_target_state,
        },
        tolerances.voltage_v,
    )
    .map_err(measurement_error)?;
    let outcome = if latency.observed
        && latency.latency_s <= spec.max_latency_s + tolerances.time_s + FLOAT_TOLERANCE
    {
        RuleOutcome::Pass
    } else {
        RuleOutcome::Fail
    };

    Ok(result(
        criterion,
        outcome,
        Evidence {
            measured_value: latency.latency_s,
            required_value: spec.max_latency_s,
            tolerance_used: tolerances.time_s,
            unit: "s",
            sample_index: latency.sample_index,
            timestamp: latency.timestamp,
            reason: if latency.observed {
                format!(
                    "{} reached {} {:.6} s after {} reached {}",
                    criterion.channel(),
                    spec.expected_target_state.as_str(),
                    latency.latency_s,
                    spec.source_channel,
                    spec.source_state.as_str()
                )
            } else {
                format!(
                    "{} did not reach {} after {} reached {}",
                    criterion.channel(),
                    spec.expected_target_state.as_str(),
                    spec.source_channel,
                    spec.source_state.as_str()
                )
            },
            method: "response_latency",
            method_context: RuleMeasurementMethodContext {
                source: "ferrisoxide-rule-engine".to_string(),
                threshold_v: Some(round_evidence(spec.target_threshold_v)),
                state: Some(spec.source_state.as_str().to_string()),
                expected_state: Some(spec.expected_target_state.as_str().to_string()),
                selection: Some("first_response".to_string()),
                ..RuleMeasurementMethodContext::default()
            },
        },
    ))
}

fn evaluate_stable_state_duration(
    waveform: RuleWaveform<'_>,
    channel: &RuleChannel<'_>,
    criterion: &RuleCriterion,
    state: SignalState,
    threshold_v: f64,
    min_duration_s: f64,
    tolerances: RuleTolerances,
) -> Result<EvaluatedCriterion> {
    let longest = state_run_extremum(
        waveform.time,
        channel.samples,
        state,
        threshold_v,
        tolerances.voltage_v,
        RunSelection::Longest,
    )
    .map_err(measurement_error)?;
    let (measured, sample_index, timestamp) = longest
        .map(|run| (run.duration_s, run.start_index, run.start_time))
        .unwrap_or((0.0, 0, waveform.time[0]));
    let outcome = if measured + tolerances.time_s + FLOAT_TOLERANCE >= min_duration_s {
        RuleOutcome::Pass
    } else {
        RuleOutcome::Fail
    };

    Ok(result(
        criterion,
        outcome,
        Evidence {
            measured_value: measured,
            required_value: min_duration_s,
            tolerance_used: tolerances.time_s,
            unit: "s",
            sample_index,
            timestamp,
            reason: format!(
                "longest stable {} duration was {:.6} s",
                state.as_str(),
                measured
            ),
            method: "state_run_duration",
            method_context: state_run_context(threshold_v, state, "longest"),
        },
    ))
}

#[derive(Debug, Clone, Copy)]
struct RiseFallTimeSpec {
    direction: EdgeDirection,
    low_threshold_v: f64,
    high_threshold_v: f64,
    max_duration_s: f64,
}

fn evaluate_rise_fall_time(
    waveform: RuleWaveform<'_>,
    channel: &RuleChannel<'_>,
    criterion: &RuleCriterion,
    spec: RiseFallTimeSpec,
    tolerances: RuleTolerances,
) -> Result<EvaluatedCriterion> {
    if spec.low_threshold_v >= spec.high_threshold_v {
        return Err(RuleEngineError::InvalidParameter {
            name: "criteria.low_threshold_v".to_string(),
            reason: "must be lower than high_threshold_v".to_string(),
        });
    }

    let measurement = match spec.direction {
        EdgeDirection::Rise => measure_rise_time(
            waveform.time,
            channel.samples,
            spec.low_threshold_v,
            spec.high_threshold_v,
            tolerances.voltage_v,
        ),
        EdgeDirection::Fall => measure_fall_time(
            waveform.time,
            channel.samples,
            spec.low_threshold_v,
            spec.high_threshold_v,
            tolerances.voltage_v,
        ),
    }
    .map_err(measurement_error)?;

    let (measured, sample_index, timestamp, observed) = measurement
        .map(|transition| {
            (
                transition.duration_s,
                transition.end_index,
                transition.end_time,
                true,
            )
        })
        .unwrap_or((f64::INFINITY, 0, waveform.time[0], false));
    let outcome =
        if observed && measured <= spec.max_duration_s + tolerances.time_s + FLOAT_TOLERANCE {
            RuleOutcome::Pass
        } else {
            RuleOutcome::Fail
        };

    Ok(result(
        criterion,
        outcome,
        Evidence {
            measured_value: measured,
            required_value: spec.max_duration_s,
            tolerance_used: tolerances.time_s,
            unit: "s",
            sample_index,
            timestamp,
            reason: if observed {
                format!(
                    "{} time from {:.6} V to {:.6} V was {:.6} s",
                    spec.direction.as_str(),
                    spec.low_threshold_v,
                    spec.high_threshold_v,
                    measured
                )
            } else {
                format!("{} transition was not observed", spec.direction.as_str())
            },
            method: "edge_time",
            method_context: edge_context(
                spec.low_threshold_v,
                spec.high_threshold_v,
                spec.direction,
            ),
        },
    ))
}

fn evaluate_measurement_criterion(
    waveform: RuleWaveform<'_>,
    channel: &RuleChannel<'_>,
    criterion: &RuleCriterion,
    measurement: &RuleMeasurementSpec,
    requirement: &RuleMeasurementRequirement,
    tolerances: RuleTolerances,
) -> Result<EvaluatedCriterion> {
    let measured = measure_dsl_criterion(waveform, channel, measurement, tolerances)?;
    let outcome = if measured.observed
        && requirement_satisfied(
            measured.evidence.measured_value,
            requirement.value,
            requirement.operator,
            measured.evidence.tolerance_used,
        ) {
        RuleOutcome::Pass
    } else {
        RuleOutcome::Fail
    };

    Ok(result(
        criterion,
        outcome,
        Evidence {
            required_value: requirement.value,
            ..measured.evidence
        },
    ))
}

struct DslMeasurementEvidence {
    evidence: Evidence,
    observed: bool,
}

fn measure_dsl_criterion(
    waveform: RuleWaveform<'_>,
    channel: &RuleChannel<'_>,
    measurement: &RuleMeasurementSpec,
    tolerances: RuleTolerances,
) -> Result<DslMeasurementEvidence> {
    match measurement {
        RuleMeasurementSpec::MinimumSample => {
            let measurement =
                minimum_sample(waveform.time, channel.samples).map_err(measurement_error)?;
            Ok(DslMeasurementEvidence {
                evidence: Evidence {
                    measured_value: measurement.value,
                    required_value: 0.0,
                    tolerance_used: tolerances.voltage_v,
                    unit: "V",
                    sample_index: measurement.sample_index,
                    timestamp: measurement.timestamp,
                    reason: format!("minimum observed voltage was {:.6} V", measurement.value),
                    method: "minimum_sample",
                    method_context: RuleMeasurementMethodContext::default(),
                },
                observed: true,
            })
        }
        RuleMeasurementSpec::MaximumSample => {
            let measurement =
                maximum_sample(waveform.time, channel.samples).map_err(measurement_error)?;
            Ok(DslMeasurementEvidence {
                evidence: Evidence {
                    measured_value: measurement.value,
                    required_value: 0.0,
                    tolerance_used: tolerances.voltage_v,
                    unit: "V",
                    sample_index: measurement.sample_index,
                    timestamp: measurement.timestamp,
                    reason: format!("maximum observed voltage was {:.6} V", measurement.value),
                    method: "maximum_sample",
                    method_context: RuleMeasurementMethodContext::default(),
                },
                observed: true,
            })
        }
        RuleMeasurementSpec::StateTransitionCount { threshold_v } => {
            let transitions = count_state_transitions(
                waveform.time,
                channel.samples,
                *threshold_v,
                tolerances.voltage_v,
            )
            .map_err(measurement_error)?;
            let measured = transitions.count;
            Ok(DslMeasurementEvidence {
                evidence: Evidence {
                    measured_value: measured as f64,
                    required_value: 0.0,
                    tolerance_used: 0.0,
                    unit: "transitions",
                    sample_index: transitions.first_index.unwrap_or(0),
                    timestamp: transitions.first_timestamp.unwrap_or(waveform.time[0]),
                    reason: format!("observed {measured} state transitions at {threshold_v:.6} V"),
                    method: "state_transition_count",
                    method_context: threshold_context(*threshold_v),
                },
                observed: true,
            })
        }
        RuleMeasurementSpec::PulseWidth {
            state,
            threshold_v,
            selection,
        } => {
            let selection = *selection;
            let run = state_run_extremum(
                waveform.time,
                channel.samples,
                *state,
                *threshold_v,
                tolerances.voltage_v,
                run_selection(selection),
            )
            .map_err(measurement_error)?;

            let Some(run) = run else {
                return Ok(DslMeasurementEvidence {
                    evidence: Evidence {
                        measured_value: 0.0,
                        required_value: 0.0,
                        tolerance_used: tolerances.time_s,
                        unit: "s",
                        sample_index: 0,
                        timestamp: waveform.time[0],
                        reason: format!("no {} pulse was observed", state.as_str()),
                        method: "state_run_duration",
                        method_context: state_run_context(*threshold_v, *state, selection.as_str()),
                    },
                    observed: false,
                });
            };

            Ok(DslMeasurementEvidence {
                evidence: Evidence {
                    measured_value: run.duration_s,
                    required_value: 0.0,
                    tolerance_used: tolerances.time_s,
                    unit: "s",
                    sample_index: run.start_index,
                    timestamp: run.start_time,
                    reason: format!(
                        "{} pulse width met configured limits; measured {:.6} s",
                        state.as_str(),
                        run.duration_s
                    ),
                    method: "state_run_duration",
                    method_context: state_run_context(*threshold_v, *state, selection.as_str()),
                },
                observed: true,
            })
        }
        RuleMeasurementSpec::StableStateDuration { state, threshold_v } => {
            let longest = state_run_extremum(
                waveform.time,
                channel.samples,
                *state,
                *threshold_v,
                tolerances.voltage_v,
                RunSelection::Longest,
            )
            .map_err(measurement_error)?;
            let (measured, sample_index, timestamp) = longest
                .map(|run| (run.duration_s, run.start_index, run.start_time))
                .unwrap_or((0.0, 0, waveform.time[0]));
            Ok(DslMeasurementEvidence {
                evidence: Evidence {
                    measured_value: measured,
                    required_value: 0.0,
                    tolerance_used: tolerances.time_s,
                    unit: "s",
                    sample_index,
                    timestamp,
                    reason: format!(
                        "longest stable {} duration was {:.6} s",
                        state.as_str(),
                        measured
                    ),
                    method: "state_run_duration",
                    method_context: state_run_context(*threshold_v, *state, "longest"),
                },
                observed: true,
            })
        }
        RuleMeasurementSpec::TransientEventDuration {
            event_kind,
            expected_state,
            threshold_v,
        } => {
            let transient_state = expected_state.opposite();
            let longest = state_run_extremum(
                waveform.time,
                channel.samples,
                transient_state,
                *threshold_v,
                tolerances.voltage_v,
                RunSelection::Longest,
            )
            .map_err(measurement_error)?;

            let (measured, sample_index, timestamp) = longest
                .map(|run| (run.duration_s, run.start_index, run.start_time))
                .unwrap_or((0.0, 0, waveform.time[0]));
            Ok(DslMeasurementEvidence {
                evidence: Evidence {
                    measured_value: measured,
                    required_value: 0.0,
                    tolerance_used: tolerances.time_s,
                    unit: "s",
                    sample_index,
                    timestamp,
                    reason: format!(
                        "longest unintended {} {} duration was {:.6} s",
                        transient_state.as_str(),
                        event_kind,
                        measured
                    ),
                    method: "state_run_duration",
                    method_context: transient_context(
                        *threshold_v,
                        transient_state,
                        *expected_state,
                        event_kind,
                    ),
                },
                observed: true,
            })
        }
        RuleMeasurementSpec::RiseTime {
            low_threshold_v,
            high_threshold_v,
        } => measure_dsl_edge_time(
            waveform,
            channel,
            EdgeDirection::Rise,
            *low_threshold_v,
            *high_threshold_v,
            tolerances,
        ),
        RuleMeasurementSpec::FallTime {
            low_threshold_v,
            high_threshold_v,
        } => measure_dsl_edge_time(
            waveform,
            channel,
            EdgeDirection::Fall,
            *low_threshold_v,
            *high_threshold_v,
            tolerances,
        ),
    }
}

fn measure_dsl_edge_time(
    waveform: RuleWaveform<'_>,
    channel: &RuleChannel<'_>,
    direction: EdgeDirection,
    low_threshold_v: f64,
    high_threshold_v: f64,
    tolerances: RuleTolerances,
) -> Result<DslMeasurementEvidence> {
    if low_threshold_v >= high_threshold_v {
        return Err(RuleEngineError::InvalidParameter {
            name: "criteria.measurement.low_threshold".to_string(),
            reason: "must be lower than high_threshold".to_string(),
        });
    }

    let measurement = match direction {
        EdgeDirection::Rise => measure_rise_time(
            waveform.time,
            channel.samples,
            low_threshold_v,
            high_threshold_v,
            tolerances.voltage_v,
        ),
        EdgeDirection::Fall => measure_fall_time(
            waveform.time,
            channel.samples,
            low_threshold_v,
            high_threshold_v,
            tolerances.voltage_v,
        ),
    }
    .map_err(measurement_error)?;

    let (measured, sample_index, timestamp, observed) = measurement
        .map(|transition| {
            (
                transition.duration_s,
                transition.end_index,
                transition.end_time,
                true,
            )
        })
        .unwrap_or((f64::INFINITY, 0, waveform.time[0], false));

    Ok(DslMeasurementEvidence {
        evidence: Evidence {
            measured_value: measured,
            required_value: 0.0,
            tolerance_used: tolerances.time_s,
            unit: "s",
            sample_index,
            timestamp,
            reason: if observed {
                format!(
                    "{} time from {:.6} V to {:.6} V was {:.6} s",
                    direction.as_str(),
                    low_threshold_v,
                    high_threshold_v,
                    measured
                )
            } else {
                format!("{} transition was not observed", direction.as_str())
            },
            method: "edge_time",
            method_context: edge_context(low_threshold_v, high_threshold_v, direction),
        },
        observed,
    })
}

fn requirement_satisfied(
    measured: f64,
    required: f64,
    operator: RuleCriterionOperator,
    tolerance: f64,
) -> bool {
    match operator {
        RuleCriterionOperator::LessThan => {
            measured < required
                || (tolerance > 0.0 && measured - tolerance < required + FLOAT_TOLERANCE)
        }
        RuleCriterionOperator::LessThanOrEqual => {
            measured - tolerance <= required + FLOAT_TOLERANCE
        }
        RuleCriterionOperator::GreaterThan => {
            measured > required
                || (tolerance > 0.0 && measured + tolerance > required - FLOAT_TOLERANCE)
        }
        RuleCriterionOperator::GreaterThanOrEqual => {
            measured + tolerance + FLOAT_TOLERANCE >= required
        }
        RuleCriterionOperator::EqualTo => {
            (measured - required).abs() <= tolerance + FLOAT_TOLERANCE
        }
    }
}

fn run_selection(selection: RuleRunSelectionConfig) -> RunSelection {
    match selection {
        RuleRunSelectionConfig::Shortest => RunSelection::Shortest,
        RuleRunSelectionConfig::Longest => RunSelection::Longest,
    }
}

fn longest_state_run_in_window(
    time: &[f64],
    samples: &[f64],
    state: SignalState,
    threshold_v: f64,
    voltage_tolerance_v: f64,
    window: TimeWindow,
) -> core::result::Result<Option<StateRunEvidence>, MeasurementError> {
    validate_strict_time_and_sample_lengths(time, samples)?;

    let start_bound = window.start_time_s.unwrap_or(f64::NEG_INFINITY);
    let end_bound = window.end_time_s.unwrap_or(f64::INFINITY);
    let mut best = None;
    let mut current_start = None;
    let mut last_window_index = None;

    for index in 0..samples.len() {
        let timestamp = time[index];
        if timestamp < start_bound {
            continue;
        }
        if timestamp > end_bound {
            break;
        }
        last_window_index = Some(index);

        if sample_state(samples[index], threshold_v, voltage_tolerance_v) == state {
            if current_start.is_none() {
                current_start = Some(index);
            }
        } else if let Some(start_index) = current_start.take() {
            update_longest_run(
                &mut best,
                StateRunEvidence {
                    duration_s: timestamp - time[start_index],
                    start_index,
                    start_time: time[start_index],
                },
            );
        }
    }

    if let Some(start_index) = current_start {
        let last_index = last_window_index.expect("active run implies window sample");
        let end_time = time
            .get(last_index + 1)
            .copied()
            .filter(|next_time| *next_time <= end_bound)
            .unwrap_or_else(|| {
                if end_bound.is_finite() && end_bound >= time[start_index] {
                    end_bound
                } else {
                    time[last_index]
                }
            });
        update_longest_run(
            &mut best,
            StateRunEvidence {
                duration_s: end_time - time[start_index],
                start_index,
                start_time: time[start_index],
            },
        );
    }

    Ok(best)
}

fn update_longest_run(best: &mut Option<StateRunEvidence>, candidate: StateRunEvidence) {
    let replace = best
        .as_ref()
        .map(|current| candidate.duration_s >= current.duration_s)
        .unwrap_or(true);
    if replace {
        *best = Some(candidate);
    }
}

fn window_reference(time: &[f64], window: TimeWindow) -> (usize, f64) {
    let start_bound = window.start_time_s.unwrap_or(f64::NEG_INFINITY);
    let end_bound = window.end_time_s.unwrap_or(f64::INFINITY);
    time.iter()
        .copied()
        .enumerate()
        .find(|(_, timestamp)| *timestamp >= start_bound && *timestamp <= end_bound)
        .unwrap_or((0, time[0]))
}

fn arm_window_after_expected_state(
    time: &[f64],
    samples: &[f64],
    expected_state: SignalState,
    threshold_v: f64,
    voltage_tolerance_v: f64,
    window: TimeWindow,
) -> core::result::Result<TimeWindow, MeasurementError> {
    validate_strict_time_and_sample_lengths(time, samples)?;
    let (start_index, _) = window_reference(time, window);
    let Some((_, start_time)) = first_state_entry(
        samples,
        time,
        threshold_v,
        voltage_tolerance_v,
        expected_state,
        start_index,
    ) else {
        return Ok(window);
    };

    Ok(TimeWindow {
        start_time_s: Some(start_time),
        end_time_s: window.end_time_s,
    })
}

#[derive(Debug, Clone, Copy)]
struct ResponseLatencyMeasurementSpec {
    source_threshold_v: f64,
    target_threshold_v: f64,
    source_state: SignalState,
    expected_target_state: SignalState,
}

#[derive(Debug, Clone, Copy)]
struct ResponseLatencyMeasurement {
    latency_s: f64,
    sample_index: usize,
    timestamp: f64,
    observed: bool,
}

fn measure_response_latency(
    time: &[f64],
    source_samples: &[f64],
    target_samples: &[f64],
    spec: ResponseLatencyMeasurementSpec,
    voltage_tolerance_v: f64,
) -> core::result::Result<ResponseLatencyMeasurement, MeasurementError> {
    validate_strict_time_and_sample_lengths(time, source_samples)?;
    validate_strict_time_and_sample_lengths(time, target_samples)?;

    let Some((source_index, source_time)) = first_state_entry(
        source_samples,
        time,
        spec.source_threshold_v,
        voltage_tolerance_v,
        spec.source_state,
        0,
    ) else {
        return Ok(ResponseLatencyMeasurement {
            latency_s: 0.0,
            sample_index: 0,
            timestamp: time[0],
            observed: false,
        });
    };

    let Some((target_index, target_time)) = first_state_entry(
        target_samples,
        time,
        spec.target_threshold_v,
        voltage_tolerance_v,
        spec.expected_target_state,
        source_index,
    ) else {
        let last_index = time.len() - 1;
        return Ok(ResponseLatencyMeasurement {
            latency_s: time[last_index] - source_time,
            sample_index: last_index,
            timestamp: time[last_index],
            observed: false,
        });
    };

    Ok(ResponseLatencyMeasurement {
        latency_s: target_time - source_time,
        sample_index: target_index,
        timestamp: target_time,
        observed: true,
    })
}

fn first_state_entry(
    samples: &[f64],
    time: &[f64],
    threshold_v: f64,
    voltage_tolerance_v: f64,
    state: SignalState,
    start_index: usize,
) -> Option<(usize, f64)> {
    if sample_state(samples[start_index], threshold_v, voltage_tolerance_v) == state {
        return Some((start_index, time[start_index]));
    }

    let mut previous_state = sample_state(samples[start_index], threshold_v, voltage_tolerance_v);
    for index in start_index + 1..samples.len() {
        let next_state = sample_state(samples[index], threshold_v, voltage_tolerance_v);
        if previous_state != next_state && next_state == state {
            return Some((index, time[index]));
        }
        previous_state = next_state;
    }

    None
}

fn sample_state(value: f64, threshold_v: f64, voltage_tolerance_v: f64) -> SignalState {
    if value + voltage_tolerance_v >= threshold_v {
        SignalState::High
    } else {
        SignalState::Low
    }
}

fn validate_strict_time_and_sample_lengths(
    time: &[f64],
    samples: &[f64],
) -> core::result::Result<(), MeasurementError> {
    if time.is_empty() || samples.is_empty() {
        return Err(MeasurementError::EmptyInput);
    }
    if time.len() != samples.len() {
        return Err(MeasurementError::MismatchedLength);
    }
    if time.windows(2).any(|pair| pair[1] <= pair[0]) {
        return Err(MeasurementError::NonMonotonicTimeAxis);
    }
    Ok(())
}

fn measurement_error(error: MeasurementError) -> RuleEngineError {
    match error {
        MeasurementError::EmptyInput => RuleEngineError::EmptyInput,
        MeasurementError::MismatchedLength => RuleEngineError::InvalidWaveform {
            reason: "measurement time and sample arrays must have the same length".to_string(),
        },
        MeasurementError::NonMonotonicTimeAxis => RuleEngineError::InvalidWaveform {
            reason: "time samples must be strictly increasing for duration measurements"
                .to_string(),
        },
    }
}

fn borrowed_measurement_error<'a>(error: MeasurementError) -> BorrowedRuleError<'a> {
    match error {
        MeasurementError::EmptyInput => BorrowedRuleError::EmptyInput,
        MeasurementError::MismatchedLength => BorrowedRuleError::InvalidWaveform {
            reason: "measurement time and sample arrays must have the same length",
        },
        MeasurementError::NonMonotonicTimeAxis => BorrowedRuleError::InvalidWaveform {
            reason: "time samples must be strictly increasing for duration measurements",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evaluate_owned(
        time: &[f64],
        samples: &[f64],
        criteria: &[RuleCriterion],
        tolerances: RuleTolerances,
    ) -> RuleEvaluation {
        let channels = [RuleChannel {
            name: "input_v",
            samples,
        }];
        let waveform = RuleWaveform {
            time,
            channels: &channels,
        };
        evaluate_rule_set(waveform, criteria, tolerances).expect("rule set should evaluate")
    }

    #[test]
    fn evaluates_minimum_and_maximum_voltage() {
        let time = [0.0, 0.1, 0.2];
        let samples = [0.0, 3.3, 5.0];
        let criteria = [
            RuleCriterion {
                id: "min".to_string(),
                check: RuleCriterionCheck::MinimumVoltage {
                    channel: "input_v".to_string(),
                    threshold_v: 0.0,
                },
            },
            RuleCriterion {
                id: "max".to_string(),
                check: RuleCriterionCheck::MaximumVoltage {
                    channel: "input_v".to_string(),
                    threshold_v: 5.5,
                },
            },
        ];

        let evaluation = evaluate_owned(&time, &samples, &criteria, RuleTolerances::default());

        assert_eq!(evaluation.results[0].outcome, RuleOutcome::Pass);
        assert_eq!(evaluation.results[1].outcome, RuleOutcome::Pass);
        assert_eq!(evaluation.measurements[1].method, "maximum_sample");
        assert_eq!(
            evaluation.measurements[1].method_context.source,
            "ferrisoxide-measurements"
        );
    }

    #[test]
    fn detects_state_transitions_and_transient_events() {
        let time = [0.0, 0.001, 0.002, 0.003, 0.004, 0.005];
        let samples = [5.0, 5.0, 0.0, 0.0, 5.0, 5.0];
        let criteria = [
            RuleCriterion {
                id: "transitions".to_string(),
                check: RuleCriterionCheck::StateTransitions {
                    channel: "input_v".to_string(),
                    threshold_v: 2.5,
                    expected_count: 2,
                },
            },
            RuleCriterion {
                id: "dropout".to_string(),
                check: RuleCriterionCheck::TransientEvent {
                    channel: "input_v".to_string(),
                    event_kind: "dropout".to_string(),
                    expected_state: SignalState::High,
                    threshold_v: 2.5,
                    max_duration_s: 0.001,
                    start_time_s: None,
                    end_time_s: None,
                    arm_after_first_expected_state: false,
                },
            },
        ];

        let evaluation = evaluate_owned(&time, &samples, &criteria, RuleTolerances::default());

        assert_eq!(evaluation.results[0].outcome, RuleOutcome::Pass);
        assert_eq!(evaluation.results[1].outcome, RuleOutcome::Fail);
        assert_eq!(evaluation.results[1].measured_value, 0.002);
        assert_eq!(evaluation.results[1].sample_index, 2);
    }

    #[test]
    fn evaluates_measurement_backed_criteria_with_tolerance() {
        let time = [0.0, 0.001];
        let samples = [0.0, 5.01];
        let criteria = [RuleCriterion {
            id: "max".to_string(),
            check: RuleCriterionCheck::Measurement {
                channel: "input_v".to_string(),
                measurement: RuleMeasurementSpec::MaximumSample,
                requirement: RuleMeasurementRequirement {
                    operator: RuleCriterionOperator::LessThanOrEqual,
                    value: 5.0,
                },
            },
        }];

        let evaluation = evaluate_owned(
            &time,
            &samples,
            &criteria,
            RuleTolerances {
                voltage_v: 0.02,
                time_s: 0.0,
            },
        );

        assert_eq!(evaluation.results[0].outcome, RuleOutcome::Pass);
        assert_eq!(evaluation.results[0].tolerance_used, 0.02);
    }

    #[test]
    fn evaluates_response_latency_between_channels() {
        let time = [0.0, 0.999, 1.0, 1.02, 1.08];
        let command = [0.0, 0.0, 5.0, 5.0, 5.0];
        let feedback = [0.0, 0.0, 0.0, 5.0, 5.0];
        let channels = [
            RuleChannel {
                name: "command_v",
                samples: &command,
            },
            RuleChannel {
                name: "feedback_v",
                samples: &feedback,
            },
        ];
        let waveform = RuleWaveform {
            time: &time,
            channels: &channels,
        };
        let criteria = [RuleCriterion {
            id: "response".to_string(),
            check: RuleCriterionCheck::ResponseLatency {
                source_channel: "command_v".to_string(),
                target_channel: "feedback_v".to_string(),
                source_threshold_v: 2.5,
                target_threshold_v: 2.5,
                source_state: SignalState::High,
                expected_target_state: SignalState::High,
                max_latency_s: 0.05,
            },
        }];

        let evaluation = evaluate_rule_set(waveform, &criteria, RuleTolerances::default())
            .expect("response latency should evaluate");

        assert_eq!(evaluation.results[0].outcome, RuleOutcome::Pass);
        assert_eq!(evaluation.results[0].measured_value, 0.02);
        assert_eq!(evaluation.results[0].sample_index, 3);
        assert_eq!(evaluation.results[0].timestamp, 1.02);
        assert_eq!(evaluation.measurements[0].method, "response_latency");
    }

    #[test]
    fn transient_event_window_ignores_pre_window_state() {
        let time = [0.0, 1.0, 1.02, 1.20, 1.2024, 1.8];
        let samples = [0.0, 0.0, 5.0, 0.0, 5.0, 5.0];
        let criteria = [RuleCriterion {
            id: "windowed_transient".to_string(),
            check: RuleCriterionCheck::TransientEvent {
                channel: "input_v".to_string(),
                event_kind: "transient event".to_string(),
                expected_state: SignalState::High,
                threshold_v: 2.5,
                max_duration_s: 0.001,
                start_time_s: Some(1.02),
                end_time_s: None,
                arm_after_first_expected_state: false,
            },
        }];

        let evaluation = evaluate_owned(&time, &samples, &criteria, RuleTolerances::default());

        assert_eq!(evaluation.results[0].outcome, RuleOutcome::Fail);
        assert_eq!(evaluation.results[0].measured_value, 0.0024);
        assert_eq!(evaluation.results[0].sample_index, 3);
        assert_eq!(evaluation.results[0].timestamp, 1.2);
    }

    #[test]
    fn transient_event_can_arm_after_first_expected_state() {
        let time = [0.0, 1.0, 1.02, 1.07, 1.8];
        let samples = [0.0, 0.0, 0.0, 5.0, 5.0];
        let criteria = [RuleCriterion {
            id: "armed_transient".to_string(),
            check: RuleCriterionCheck::TransientEvent {
                channel: "input_v".to_string(),
                event_kind: "transient event".to_string(),
                expected_state: SignalState::High,
                threshold_v: 2.5,
                max_duration_s: 0.001,
                start_time_s: Some(1.02),
                end_time_s: None,
                arm_after_first_expected_state: true,
            },
        }];

        let evaluation = evaluate_owned(&time, &samples, &criteria, RuleTolerances::default());

        assert_eq!(evaluation.results[0].outcome, RuleOutcome::Pass);
        assert_eq!(evaluation.results[0].measured_value, 0.0);
        assert_eq!(evaluation.results[0].sample_index, 3);
        assert_eq!(evaluation.results[0].timestamp, 1.07);
    }

    #[test]
    fn rejects_decreasing_time_for_time_dependent_criteria() {
        let time = [0.0, 0.002, 0.001];
        let samples = [0.0, 5.0, 5.0];
        let criteria = [RuleCriterion {
            id: "pulse".to_string(),
            check: RuleCriterionCheck::PulseWidth {
                channel: "input_v".to_string(),
                state: SignalState::High,
                threshold_v: 2.5,
                min_width_s: Some(0.001),
                max_width_s: None,
            },
        }];

        let channels = [RuleChannel {
            name: "input_v",
            samples: &samples,
        }];
        let waveform = RuleWaveform {
            time: &time,
            channels: &channels,
        };
        let error = evaluate_rule_set(waveform, &criteria, RuleTolerances::default())
            .expect_err("non-monotonic time should fail");

        assert!(matches!(error, RuleEngineError::InvalidWaveform { .. }));
    }

    #[test]
    fn borrowed_summary_evaluates_basic_rule_without_owned_rule_data() {
        let time = [0.0, 0.001, 0.002, 0.003, 0.004];
        let samples = [0.0, 0.0, 5.0, 5.0, 0.0];
        let channels = [RuleChannel {
            name: "input_v",
            samples: &samples,
        }];
        let waveform = RuleWaveform {
            time: &time,
            channels: &channels,
        };
        let criterion = BorrowedRuleCriterion {
            id: "transition_count",
            check: BorrowedRuleCriterionCheck::StateTransitions {
                channel: "input_v",
                threshold_v: 2.5,
                expected_count: 2,
            },
        };

        let summary = evaluate_borrowed_rule(waveform, criterion, RuleTolerances::default())
            .expect("borrowed rule should evaluate");

        assert_eq!(summary.outcome, RuleOutcome::Pass);
        assert_eq!(summary.criterion_id, "transition_count");
        assert_eq!(summary.channel, "input_v");
        assert_eq!(summary.measured_value, 2.0);
        assert_eq!(summary.method, "state_transition_count");
    }

    #[test]
    fn borrowed_summary_detects_transient_event() {
        let time = [0.0, 0.001, 0.002, 0.003];
        let samples = [5.0, 0.0, 0.0, 5.0];
        let channels = [RuleChannel {
            name: "supply_v",
            samples: &samples,
        }];
        let waveform = RuleWaveform {
            time: &time,
            channels: &channels,
        };
        let criterion = BorrowedRuleCriterion {
            id: "dropout",
            check: BorrowedRuleCriterionCheck::TransientEvent {
                channel: "supply_v",
                event_kind: "dropout",
                expected_state: SignalState::High,
                threshold_v: 2.5,
                max_duration_s: 0.001,
                start_time_s: None,
                end_time_s: None,
                arm_after_first_expected_state: false,
            },
        };

        let summary = evaluate_borrowed_rule(waveform, criterion, RuleTolerances::default())
            .expect("borrowed transient rule should evaluate");

        assert_eq!(summary.outcome, RuleOutcome::Fail);
        assert_eq!(summary.failed_criterion, Some("dropout"));
        assert_eq!(summary.measured_value, 0.002);
        assert_eq!(summary.unit, "s");
    }

    #[test]
    fn borrowed_summary_errors_use_borrowed_static_data() {
        let time = [0.0, 0.001];
        let samples = [0.0, 5.0];
        let channels = [RuleChannel {
            name: "input_v",
            samples: &samples,
        }];
        let waveform = RuleWaveform {
            time: &time,
            channels: &channels,
        };
        let criterion = BorrowedRuleCriterion {
            id: "missing",
            check: BorrowedRuleCriterionCheck::MinimumVoltage {
                channel: "missing_v",
                threshold_v: 0.0,
            },
        };

        let error = evaluate_borrowed_rule(waveform, criterion, RuleTolerances::default())
            .expect_err("missing channel should fail");

        assert_eq!(
            error,
            BorrowedRuleError::MissingChannel {
                channel: "missing_v"
            }
        );
    }
}
