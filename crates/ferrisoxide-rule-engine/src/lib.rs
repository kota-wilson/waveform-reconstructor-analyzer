//! Shared rule execution semantics for FerrisOxide Signal.
//!
//! This crate evaluates rule criteria over caller-provided time/sample slices.
//! It deliberately avoids CSV parsing, TOML parsing, report rendering,
//! plotting, file I/O, DAQ/controller I/O, hardware HALs, RTOS SDKs, and
//! certification claims.

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

fn is_time_dependent(criterion: &RuleCriterion) -> bool {
    matches!(
        &criterion.check,
        RuleCriterionCheck::PulseWidth { .. }
            | RuleCriterionCheck::TransientDuration { .. }
            | RuleCriterionCheck::TransientEvent { .. }
            | RuleCriterionCheck::StableStateDuration { .. }
            | RuleCriterionCheck::RiseFallTime { .. }
    ) || matches!(
        &criterion.check,
        RuleCriterionCheck::Measurement { measurement, .. } if measurement.is_time_dependent()
    )
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
            },
            tolerances,
        ),
        RuleCriterionCheck::TransientEvent {
            event_kind,
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
                event_kind,
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
        (value * 1_000_000_000.0).round() / 1_000_000_000.0
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
}

fn evaluate_transient_duration(
    waveform: RuleWaveform<'_>,
    channel: &RuleChannel<'_>,
    criterion: &RuleCriterion,
    spec: TransientDurationSpec<'_>,
    tolerances: RuleTolerances,
) -> Result<EvaluatedCriterion> {
    let transient_state = spec.expected_state.opposite();
    let longest = state_run_extremum(
        waveform.time,
        channel.samples,
        transient_state,
        spec.threshold_v,
        tolerances.voltage_v,
        RunSelection::Longest,
    )
    .map_err(measurement_error)?;

    let (measured, sample_index, timestamp) = longest
        .map(|run| (run.duration_s, run.start_index, run.start_time))
        .unwrap_or((0.0, 0, waveform.time[0]));
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

#[cfg(test)]
mod tests {
    use super::*;

    fn waveform<'a>(time: &'a [f64], samples: &'a [f64]) -> RuleWaveform<'a> {
        let channel = RuleChannel {
            name: "input_v",
            samples,
        };
        RuleWaveform {
            time,
            channels: Box::leak(Box::new([channel])),
        }
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

        let evaluation = evaluate_rule_set(
            waveform(&time, &samples),
            &criteria,
            RuleTolerances::default(),
        )
        .expect("rule set should evaluate");

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
                },
            },
        ];

        let evaluation = evaluate_rule_set(
            waveform(&time, &samples),
            &criteria,
            RuleTolerances::default(),
        )
        .expect("rule set should evaluate");

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

        let evaluation = evaluate_rule_set(
            waveform(&time, &samples),
            &criteria,
            RuleTolerances {
                voltage_v: 0.02,
                time_s: 0.0,
            },
        )
        .expect("rule set should evaluate");

        assert_eq!(evaluation.results[0].outcome, RuleOutcome::Pass);
        assert_eq!(evaluation.results[0].tolerance_used, 0.02);
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

        let error = evaluate_rule_set(
            waveform(&time, &samples),
            &criteria,
            RuleTolerances::default(),
        )
        .expect_err("non-monotonic time should fail");

        assert!(matches!(error, RuleEngineError::InvalidWaveform { .. }));
    }
}
