use crate::criteria::{
    Criterion, CriterionCheck, CriterionOperator, MeasurementRequirement, MeasurementSpec,
    RunSelectionConfig,
};
use crate::error::{Result, WaveformError};
use crate::model::{TolerancePolicy, Waveform};
use ferrisoxide_rule_engine::{
    evaluate_rule_set, RuleChannel, RuleCriterion, RuleCriterionCheck, RuleCriterionOperator,
    RuleEngineError, RuleMeasurementMethodContext, RuleMeasurementRecord,
    RuleMeasurementRequirement, RuleMeasurementSpec, RuleOutcome, RuleRunSelectionConfig,
    RuleTolerances, RuleWaveform,
};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Pass,
    Fail,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AnalysisResult {
    pub criterion_id: String,
    pub outcome: Outcome,
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

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MeasurementRecord {
    pub id: String,
    pub channel: String,
    pub method: String,
    pub measured_value: f64,
    pub unit: String,
    pub sample_index: usize,
    pub timestamp: f64,
    pub method_context: MeasurementMethodContext,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MeasurementMethodContext {
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

impl Default for MeasurementMethodContext {
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
pub struct CriteriaEvaluation {
    pub results: Vec<AnalysisResult>,
    pub measurements: Vec<MeasurementRecord>,
}

pub fn evaluate_criteria(
    waveform: &Waveform,
    criteria: &[Criterion],
) -> Result<Vec<AnalysisResult>> {
    evaluate_criteria_with_tolerances(waveform, criteria, TolerancePolicy::default())
}

pub fn evaluate_criteria_with_tolerances(
    waveform: &Waveform,
    criteria: &[Criterion],
    tolerances: TolerancePolicy,
) -> Result<Vec<AnalysisResult>> {
    Ok(evaluate_criteria_with_measurements(waveform, criteria, tolerances)?.results)
}

pub fn evaluate_criteria_with_measurements(
    waveform: &Waveform,
    criteria: &[Criterion],
    tolerances: TolerancePolicy,
) -> Result<CriteriaEvaluation> {
    let rule_channels = waveform
        .channels
        .iter()
        .map(|channel| RuleChannel {
            name: channel.name.as_str(),
            samples: channel.samples.as_slice(),
        })
        .collect::<Vec<_>>();
    let rule_criteria = criteria.iter().map(to_rule_criterion).collect::<Vec<_>>();
    let rule_waveform = RuleWaveform {
        time: waveform.time.as_slice(),
        channels: rule_channels.as_slice(),
    };
    let rule_tolerances = RuleTolerances {
        voltage_v: tolerances.voltage_v,
        time_s: tolerances.time_s,
    };

    let evaluation = evaluate_rule_set(rule_waveform, rule_criteria.as_slice(), rule_tolerances)
        .map_err(rule_engine_error)?;

    Ok(CriteriaEvaluation {
        results: evaluation
            .results
            .into_iter()
            .map(from_rule_result)
            .collect(),
        measurements: evaluation
            .measurements
            .into_iter()
            .map(from_rule_measurement)
            .collect(),
    })
}

fn to_rule_criterion(criterion: &Criterion) -> RuleCriterion {
    RuleCriterion {
        id: criterion.id.clone(),
        check: to_rule_check(&criterion.check),
    }
}

fn to_rule_check(check: &CriterionCheck) -> RuleCriterionCheck {
    match check {
        CriterionCheck::MinimumVoltage {
            channel,
            threshold_v,
        } => RuleCriterionCheck::MinimumVoltage {
            channel: channel.clone(),
            threshold_v: *threshold_v,
        },
        CriterionCheck::MaximumVoltage {
            channel,
            threshold_v,
        } => RuleCriterionCheck::MaximumVoltage {
            channel: channel.clone(),
            threshold_v: *threshold_v,
        },
        CriterionCheck::StateTransitions {
            channel,
            threshold_v,
            expected_count,
        } => RuleCriterionCheck::StateTransitions {
            channel: channel.clone(),
            threshold_v: *threshold_v,
            expected_count: *expected_count,
        },
        CriterionCheck::PulseWidth {
            channel,
            state,
            threshold_v,
            min_width_s,
            max_width_s,
        } => RuleCriterionCheck::PulseWidth {
            channel: channel.clone(),
            state: *state,
            threshold_v: *threshold_v,
            min_width_s: *min_width_s,
            max_width_s: *max_width_s,
        },
        CriterionCheck::TransientDuration {
            channel,
            expected_state,
            threshold_v,
            max_duration_s,
        } => RuleCriterionCheck::TransientDuration {
            channel: channel.clone(),
            expected_state: *expected_state,
            threshold_v: *threshold_v,
            max_duration_s: *max_duration_s,
        },
        CriterionCheck::TransientEvent {
            channel,
            event_kind,
            expected_state,
            threshold_v,
            max_duration_s,
        } => RuleCriterionCheck::TransientEvent {
            channel: channel.clone(),
            event_kind: event_kind.as_str().to_string(),
            expected_state: *expected_state,
            threshold_v: *threshold_v,
            max_duration_s: *max_duration_s,
        },
        CriterionCheck::StableStateDuration {
            channel,
            state,
            threshold_v,
            min_duration_s,
        } => RuleCriterionCheck::StableStateDuration {
            channel: channel.clone(),
            state: *state,
            threshold_v: *threshold_v,
            min_duration_s: *min_duration_s,
        },
        CriterionCheck::RiseFallTime {
            channel,
            direction,
            low_threshold_v,
            high_threshold_v,
            max_duration_s,
        } => RuleCriterionCheck::RiseFallTime {
            channel: channel.clone(),
            direction: *direction,
            low_threshold_v: *low_threshold_v,
            high_threshold_v: *high_threshold_v,
            max_duration_s: *max_duration_s,
        },
        CriterionCheck::Measurement {
            channel,
            measurement,
            requirement,
        } => RuleCriterionCheck::Measurement {
            channel: channel.clone(),
            measurement: to_rule_measurement(measurement),
            requirement: to_rule_requirement(requirement),
        },
    }
}

fn to_rule_measurement(measurement: &MeasurementSpec) -> RuleMeasurementSpec {
    match measurement {
        MeasurementSpec::MinimumSample => RuleMeasurementSpec::MinimumSample,
        MeasurementSpec::MaximumSample => RuleMeasurementSpec::MaximumSample,
        MeasurementSpec::StateTransitionCount { threshold_v } => {
            RuleMeasurementSpec::StateTransitionCount {
                threshold_v: *threshold_v,
            }
        }
        MeasurementSpec::PulseWidth {
            state,
            threshold_v,
            selection,
        } => RuleMeasurementSpec::PulseWidth {
            state: *state,
            threshold_v: *threshold_v,
            selection: to_rule_selection(*selection),
        },
        MeasurementSpec::StableStateDuration { state, threshold_v } => {
            RuleMeasurementSpec::StableStateDuration {
                state: *state,
                threshold_v: *threshold_v,
            }
        }
        MeasurementSpec::TransientEventDuration {
            event_kind,
            expected_state,
            threshold_v,
        } => RuleMeasurementSpec::TransientEventDuration {
            event_kind: event_kind.as_str().to_string(),
            expected_state: *expected_state,
            threshold_v: *threshold_v,
        },
        MeasurementSpec::RiseTime {
            low_threshold_v,
            high_threshold_v,
        } => RuleMeasurementSpec::RiseTime {
            low_threshold_v: *low_threshold_v,
            high_threshold_v: *high_threshold_v,
        },
        MeasurementSpec::FallTime {
            low_threshold_v,
            high_threshold_v,
        } => RuleMeasurementSpec::FallTime {
            low_threshold_v: *low_threshold_v,
            high_threshold_v: *high_threshold_v,
        },
    }
}

fn to_rule_requirement(requirement: &MeasurementRequirement) -> RuleMeasurementRequirement {
    RuleMeasurementRequirement {
        operator: to_rule_operator(requirement.operator),
        value: requirement.value,
    }
}

fn to_rule_operator(operator: CriterionOperator) -> RuleCriterionOperator {
    match operator {
        CriterionOperator::LessThan => RuleCriterionOperator::LessThan,
        CriterionOperator::LessThanOrEqual => RuleCriterionOperator::LessThanOrEqual,
        CriterionOperator::GreaterThan => RuleCriterionOperator::GreaterThan,
        CriterionOperator::GreaterThanOrEqual => RuleCriterionOperator::GreaterThanOrEqual,
        CriterionOperator::EqualTo => RuleCriterionOperator::EqualTo,
    }
}

fn to_rule_selection(selection: RunSelectionConfig) -> RuleRunSelectionConfig {
    match selection {
        RunSelectionConfig::Shortest => RuleRunSelectionConfig::Shortest,
        RunSelectionConfig::Longest => RuleRunSelectionConfig::Longest,
    }
}

fn from_rule_result(result: ferrisoxide_rule_engine::RuleResult) -> AnalysisResult {
    AnalysisResult {
        criterion_id: result.criterion_id,
        outcome: from_rule_outcome(result.outcome),
        failed_criterion: result.failed_criterion,
        measurement_id: result.measurement_id,
        channel: result.channel,
        measured_value: result.measured_value,
        required_value: result.required_value,
        tolerance_used: result.tolerance_used,
        unit: result.unit,
        sample_index: result.sample_index,
        timestamp: result.timestamp,
        reason: result.reason,
    }
}

fn from_rule_outcome(outcome: RuleOutcome) -> Outcome {
    match outcome {
        RuleOutcome::Pass => Outcome::Pass,
        RuleOutcome::Fail => Outcome::Fail,
    }
}

fn from_rule_measurement(record: RuleMeasurementRecord) -> MeasurementRecord {
    MeasurementRecord {
        id: record.id,
        channel: record.channel,
        method: record.method,
        measured_value: record.measured_value,
        unit: record.unit,
        sample_index: record.sample_index,
        timestamp: record.timestamp,
        method_context: from_rule_context(record.method_context),
    }
}

fn from_rule_context(context: RuleMeasurementMethodContext) -> MeasurementMethodContext {
    MeasurementMethodContext {
        source: context.source,
        threshold_v: context.threshold_v,
        low_threshold_v: context.low_threshold_v,
        high_threshold_v: context.high_threshold_v,
        state: context.state,
        expected_state: context.expected_state,
        event_kind: context.event_kind,
        direction: context.direction,
        selection: context.selection,
    }
}

fn rule_engine_error(error: RuleEngineError) -> WaveformError {
    match error {
        RuleEngineError::EmptyInput => WaveformError::EmptyInput,
        RuleEngineError::MissingChannel { channel } => {
            WaveformError::MissingColumn { column: channel }
        }
        RuleEngineError::InvalidWaveform { reason } => WaveformError::InvalidWaveform { reason },
        RuleEngineError::InvalidParameter { name, reason } => {
            WaveformError::InvalidParameter { name, reason }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::criteria::{
        CriterionOperator, EdgeDirection, MeasurementRequirement, MeasurementSpec, SignalState,
        TransientEventKind,
    };
    use crate::model::{Channel, Unit};

    fn waveform() -> Waveform {
        Waveform::new(
            vec![0.0, 0.1, 0.2],
            vec![Channel::new("input_v", Unit::volts(), vec![0.0, 3.3, 5.0])],
        )
        .expect("test waveform should be valid")
    }

    #[test]
    fn evaluates_minimum_and_maximum_voltage_criteria() {
        let results = evaluate_criteria(
            &waveform(),
            &[
                Criterion::minimum_voltage("min", "input_v", 0.0),
                Criterion::maximum_voltage("max", "input_v", 5.5),
            ],
        )
        .expect("criteria should evaluate");

        assert_eq!(results[0].outcome, Outcome::Pass);
        assert_eq!(results[1].outcome, Outcome::Pass);
    }

    #[test]
    fn fails_when_voltage_exceeds_threshold() {
        let results = evaluate_criteria(
            &waveform(),
            &[Criterion::maximum_voltage("max", "input_v", 4.5)],
        )
        .expect("criteria should evaluate");

        assert_eq!(results[0].outcome, Outcome::Fail);
        assert_eq!(results[0].measured_value, 5.0);
        assert_eq!(results[0].sample_index, 2);
        assert_eq!(results[0].timestamp, 0.2);
        assert_eq!(results[0].channel, "input_v");
        assert_eq!(results[0].failed_criterion, Some("max".to_string()));
    }

    #[test]
    fn returns_measurements_with_stable_result_links() {
        let evaluation = evaluate_criteria_with_measurements(
            &waveform(),
            &[Criterion::maximum_voltage("max", "input_v", 5.5)],
            TolerancePolicy::default(),
        )
        .expect("criteria should evaluate");

        assert_eq!(evaluation.results.len(), 1);
        assert_eq!(evaluation.measurements.len(), 1);
        assert_eq!(evaluation.results[0].measurement_id, "max_measurement");
        assert_eq!(evaluation.measurements[0].id, "max_measurement");
        assert_eq!(evaluation.measurements[0].method, "maximum_sample");
        assert_eq!(evaluation.measurements[0].measured_value, 5.0);
    }

    #[test]
    fn detects_state_transitions_and_pulse_width() {
        let waveform = Waveform::new(
            vec![0.0, 0.001, 0.002, 0.003, 0.004, 0.005],
            vec![Channel::new(
                "switch_v",
                Unit::volts(),
                vec![0.0, 0.0, 5.0, 5.0, 0.0, 0.0],
            )],
        )
        .expect("test waveform should be valid");
        let results = evaluate_criteria(
            &waveform,
            &[
                Criterion::state_transitions("transitions", "switch_v", 2.5, 2),
                Criterion::pulse_width(
                    "high_width",
                    "switch_v",
                    SignalState::High,
                    2.5,
                    Some(0.002),
                    Some(0.003),
                ),
            ],
        )
        .expect("criteria should evaluate");

        assert_eq!(results[0].outcome, Outcome::Pass);
        assert_eq!(results[0].measured_value, 2.0);
        assert_eq!(results[1].outcome, Outcome::Pass);
        assert_eq!(results[1].measured_value, 0.002);
    }

    #[test]
    fn fails_when_transient_event_duration_exceeds_limit() {
        let waveform = Waveform::new(
            vec![0.0, 0.001, 0.002, 0.003, 0.004, 0.005],
            vec![Channel::new(
                "supply_v",
                Unit::volts(),
                vec![5.0, 5.0, 0.0, 0.0, 5.0, 5.0],
            )],
        )
        .expect("test waveform should be valid");
        let results = evaluate_criteria(
            &waveform,
            &[Criterion::transient_event(
                "dropout",
                "supply_v",
                TransientEventKind::Dropout,
                SignalState::High,
                2.5,
                0.001,
            )],
        )
        .expect("criteria should evaluate");

        assert_eq!(results[0].outcome, Outcome::Fail);
        assert_eq!(results[0].measured_value, 0.002);
        assert_eq!(results[0].sample_index, 2);
        assert_eq!(results[0].timestamp, 0.002);
    }

    #[test]
    fn evaluates_stable_state_and_rise_time() {
        let waveform = Waveform::new(
            vec![0.0, 0.001, 0.002, 0.003, 0.004],
            vec![Channel::new(
                "signal_v",
                Unit::volts(),
                vec![0.0, 0.5, 2.5, 4.5, 5.0],
            )],
        )
        .expect("test waveform should be valid");
        let results = evaluate_criteria(
            &waveform,
            &[
                Criterion::stable_state_duration(
                    "stable_high",
                    "signal_v",
                    SignalState::High,
                    2.5,
                    0.001,
                ),
                Criterion::rise_fall_time("rise", "signal_v", EdgeDirection::Rise, 0.5, 4.5, 0.003),
            ],
        )
        .expect("criteria should evaluate");

        assert_eq!(results[0].outcome, Outcome::Pass);
        assert_eq!(results[1].outcome, Outcome::Pass);
        assert_eq!(results[1].measured_value, 0.002);
    }

    #[test]
    fn applies_voltage_and_time_tolerances() {
        let waveform = Waveform::new(
            vec![0.0, 0.001, 0.002, 0.003],
            vec![Channel::new(
                "signal_v",
                Unit::volts(),
                vec![0.0, 0.0, 5.01, 5.01],
            )],
        )
        .expect("test waveform should be valid");
        let tolerances = TolerancePolicy {
            voltage_v: 0.02,
            time_s: 0.0005,
        };
        let results = evaluate_criteria_with_tolerances(
            &waveform,
            &[
                Criterion::maximum_voltage("max", "signal_v", 5.0),
                Criterion::pulse_width(
                    "high_width",
                    "signal_v",
                    SignalState::High,
                    2.5,
                    Some(0.0015),
                    None,
                ),
            ],
            tolerances,
        )
        .expect("criteria should evaluate");

        assert_eq!(results[0].outcome, Outcome::Pass);
        assert_eq!(results[0].tolerance_used, 0.02);
        assert_eq!(results[1].outcome, Outcome::Pass);
        assert_eq!(results[1].tolerance_used, 0.0005);
    }

    #[test]
    fn dsl_measurement_criteria_apply_explicit_operator_semantics() {
        let waveform = waveform();
        let strict = Criterion::measurement(
            "strict_max",
            "input_v",
            MeasurementSpec::MaximumSample,
            MeasurementRequirement {
                operator: CriterionOperator::LessThan,
                value: 5.0,
            },
        );
        let inclusive = Criterion::measurement(
            "inclusive_max",
            "input_v",
            MeasurementSpec::MaximumSample,
            MeasurementRequirement {
                operator: CriterionOperator::LessThanOrEqual,
                value: 5.0,
            },
        );

        let results = evaluate_criteria(&waveform, &[strict, inclusive])
            .expect("DSL measurement criteria should evaluate");

        assert_eq!(results[0].outcome, Outcome::Fail);
        assert_eq!(results[0].measured_value, 5.0);
        assert_eq!(results[0].required_value, 5.0);
        assert_eq!(results[1].outcome, Outcome::Pass);
        assert_eq!(results[1].measurement_id, "inclusive_max_measurement");
    }

    #[test]
    fn still_fails_beyond_configured_tolerance() {
        let waveform = Waveform::new(
            vec![0.0, 0.001],
            vec![Channel::new("signal_v", Unit::volts(), vec![0.0, 5.05])],
        )
        .expect("test waveform should be valid");
        let results = evaluate_criteria_with_tolerances(
            &waveform,
            &[Criterion::maximum_voltage("max", "signal_v", 5.0)],
            TolerancePolicy {
                voltage_v: 0.02,
                time_s: 0.0,
            },
        )
        .expect("criteria should evaluate");

        assert_eq!(results[0].outcome, Outcome::Fail);
    }

    #[test]
    fn rejects_duplicate_or_decreasing_time_for_duration_criteria() {
        for time in [vec![0.0, 0.001, 0.001], vec![0.0, 0.002, 0.001]] {
            let waveform = Waveform::new(
                time,
                vec![Channel::new("signal_v", Unit::volts(), vec![0.0, 5.0, 5.0])],
            )
            .expect("waveform construction still allows raw timestamps");
            let result = evaluate_criteria(
                &waveform,
                &[Criterion::pulse_width(
                    "high_width",
                    "signal_v",
                    SignalState::High,
                    2.5,
                    Some(0.001),
                    None,
                )],
            );

            assert!(matches!(result, Err(WaveformError::InvalidWaveform { .. })));
        }
    }

    #[test]
    fn allows_non_uniform_but_increasing_time_axis() {
        let waveform = Waveform::new(
            vec![0.0, 0.001, 0.003, 0.006],
            vec![Channel::new(
                "signal_v",
                Unit::volts(),
                vec![0.0, 5.0, 5.0, 0.0],
            )],
        )
        .expect("test waveform should be valid");

        let results = evaluate_criteria(
            &waveform,
            &[Criterion::pulse_width(
                "high_width",
                "signal_v",
                SignalState::High,
                2.5,
                Some(0.005),
                None,
            )],
        )
        .expect("criteria should evaluate on non-uniform timestamps");

        assert_eq!(results[0].outcome, Outcome::Pass);
        assert_eq!(results[0].measured_value, 0.005);
        assert!(
            !waveform
                .metadata
                .sample_interval
                .as_ref()
                .expect("interval summary should exist")
                .uniform
        );
    }
}
