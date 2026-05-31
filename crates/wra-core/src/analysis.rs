use crate::criteria::{Criterion, CriterionCheck, EdgeDirection, SignalState};
use crate::error::{Result, WaveformError};
use crate::model::{Channel, TolerancePolicy, Waveform};
use serde::Serialize;
use wra_measurements::{
    count_state_transitions, maximum_sample, measure_fall_time, measure_rise_time, minimum_sample,
    state_run_extremum, MeasurementError, RunSelection,
};

const FLOAT_TOLERANCE: f64 = 1.0e-12;

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
    pub channel: String,
    pub measured_value: f64,
    pub required_value: f64,
    pub tolerance_used: f64,
    pub unit: String,
    pub sample_index: usize,
    pub timestamp: f64,
    pub reason: String,
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
    tolerances.validate()?;
    validate_time_axis_for_criteria(waveform, criteria)?;
    criteria
        .iter()
        .map(|criterion| evaluate_criterion(waveform, criterion, tolerances))
        .collect()
}

fn validate_time_axis_for_criteria(waveform: &Waveform, criteria: &[Criterion]) -> Result<()> {
    if !criteria.iter().any(is_time_dependent) {
        return Ok(());
    }

    for (index, pair) in waveform.time.windows(2).enumerate() {
        if pair[1] <= pair[0] {
            return Err(WaveformError::InvalidWaveform {
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

fn is_time_dependent(criterion: &Criterion) -> bool {
    matches!(
        &criterion.check,
        CriterionCheck::PulseWidth { .. }
            | CriterionCheck::TransientDuration { .. }
            | CriterionCheck::TransientEvent { .. }
            | CriterionCheck::StableStateDuration { .. }
            | CriterionCheck::RiseFallTime { .. }
    )
}

fn evaluate_criterion(
    waveform: &Waveform,
    criterion: &Criterion,
    tolerances: TolerancePolicy,
) -> Result<AnalysisResult> {
    let channel =
        waveform
            .channel(criterion.channel())
            .ok_or_else(|| WaveformError::MissingColumn {
                column: criterion.channel().to_string(),
            })?;

    match &criterion.check {
        CriterionCheck::MinimumVoltage { threshold_v, .. } => {
            let measurement =
                minimum_sample(&waveform.time, &channel.samples).map_err(measurement_error)?;
            let outcome = if measurement.value + tolerances.voltage_v >= *threshold_v {
                Outcome::Pass
            } else {
                Outcome::Fail
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
                },
            ))
        }
        CriterionCheck::MaximumVoltage { threshold_v, .. } => {
            let measurement =
                maximum_sample(&waveform.time, &channel.samples).map_err(measurement_error)?;
            let outcome = if measurement.value - tolerances.voltage_v <= *threshold_v {
                Outcome::Pass
            } else {
                Outcome::Fail
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
                },
            ))
        }
        CriterionCheck::StateTransitions {
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
        CriterionCheck::PulseWidth {
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
        CriterionCheck::TransientDuration {
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
        CriterionCheck::TransientEvent {
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
                event_kind: event_kind.as_str(),
            },
            tolerances,
        ),
        CriterionCheck::StableStateDuration {
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
        CriterionCheck::RiseFallTime {
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
    }
}

fn result(criterion: &Criterion, outcome: Outcome, evidence: Evidence) -> AnalysisResult {
    AnalysisResult {
        criterion_id: criterion.id.clone(),
        outcome,
        failed_criterion: (outcome == Outcome::Fail).then(|| criterion.id.clone()),
        channel: criterion.channel().to_string(),
        measured_value: round_evidence(evidence.measured_value),
        required_value: round_evidence(evidence.required_value),
        tolerance_used: round_evidence(evidence.tolerance_used),
        unit: evidence.unit.to_string(),
        sample_index: evidence.sample_index,
        timestamp: round_evidence(evidence.timestamp),
        reason: evidence.reason,
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
}

fn round_evidence(value: f64) -> f64 {
    if value.is_finite() {
        (value * 1_000_000_000.0).round() / 1_000_000_000.0
    } else {
        value
    }
}

fn evaluate_state_transitions(
    waveform: &Waveform,
    channel: &Channel,
    criterion: &Criterion,
    threshold_v: f64,
    expected_count: usize,
    tolerances: TolerancePolicy,
) -> Result<AnalysisResult> {
    let transitions = count_state_transitions(
        &waveform.time,
        &channel.samples,
        threshold_v,
        tolerances.voltage_v,
    )
    .map_err(measurement_error)?;
    let measured = transitions.count;
    let sample_index = transitions.first_index.unwrap_or(0);
    let timestamp = transitions.first_timestamp.unwrap_or(waveform.time[0]);
    let outcome = if measured == expected_count {
        Outcome::Pass
    } else {
        Outcome::Fail
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
    waveform: &Waveform,
    channel: &Channel,
    criterion: &Criterion,
    spec: PulseWidthSpec,
    tolerances: TolerancePolicy,
) -> Result<AnalysisResult> {
    if spec.min_width_s.is_none() && spec.max_width_s.is_none() {
        return Err(WaveformError::InvalidParameter {
            name: "criteria.pulse_width".to_string(),
            reason: "min_width_s or max_width_s is required".to_string(),
        });
    }

    let shortest = if spec.min_width_s.is_some() {
        state_run_extremum(
            &waveform.time,
            &channel.samples,
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
            &waveform.time,
            &channel.samples,
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
            Outcome::Fail,
            Evidence {
                measured_value: 0.0,
                required_value: spec.min_width_s.or(spec.max_width_s).unwrap_or_default(),
                tolerance_used: tolerances.time_s,
                unit: "s",
                sample_index: 0,
                timestamp: waveform.time[0],
                reason: format!("no {} pulse was observed", spec.state.as_str()),
            },
        ));
    }

    if let Some(min_width_s) = spec.min_width_s {
        let shortest = shortest.expect("state run should exist after empty check");
        if shortest.duration_s + tolerances.time_s + FLOAT_TOLERANCE < min_width_s {
            return Ok(result(
                criterion,
                Outcome::Fail,
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
                },
            ));
        }
    }

    if let Some(max_width_s) = spec.max_width_s {
        let longest = longest.expect("state run should exist after empty check");
        if longest.duration_s - tolerances.time_s - FLOAT_TOLERANCE > max_width_s {
            return Ok(result(
                criterion,
                Outcome::Fail,
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
                },
            ));
        }
    }

    let measured = shortest
        .or(longest)
        .expect("state run should exist after empty check");
    Ok(result(
        criterion,
        Outcome::Pass,
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
        },
    ))
}

#[derive(Debug, Clone, Copy)]
struct TransientDurationSpec<'a> {
    expected_state: SignalState,
    threshold_v: f64,
    max_duration_s: f64,
    event_kind: &'a str,
}

fn evaluate_transient_duration(
    waveform: &Waveform,
    channel: &Channel,
    criterion: &Criterion,
    spec: TransientDurationSpec<'_>,
    tolerances: TolerancePolicy,
) -> Result<AnalysisResult> {
    let transient_state = opposite_state(spec.expected_state);
    let longest = state_run_extremum(
        &waveform.time,
        &channel.samples,
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
        Outcome::Pass
    } else {
        Outcome::Fail
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
        },
    ))
}

fn evaluate_stable_state_duration(
    waveform: &Waveform,
    channel: &Channel,
    criterion: &Criterion,
    state: SignalState,
    threshold_v: f64,
    min_duration_s: f64,
    tolerances: TolerancePolicy,
) -> Result<AnalysisResult> {
    let longest = state_run_extremum(
        &waveform.time,
        &channel.samples,
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
        Outcome::Pass
    } else {
        Outcome::Fail
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
    waveform: &Waveform,
    channel: &Channel,
    criterion: &Criterion,
    spec: RiseFallTimeSpec,
    tolerances: TolerancePolicy,
) -> Result<AnalysisResult> {
    if spec.low_threshold_v >= spec.high_threshold_v {
        return Err(WaveformError::InvalidParameter {
            name: "criteria.low_threshold_v".to_string(),
            reason: "must be lower than high_threshold_v".to_string(),
        });
    }

    let measurement = match spec.direction {
        EdgeDirection::Rise => measure_rise_time(
            &waveform.time,
            &channel.samples,
            spec.low_threshold_v,
            spec.high_threshold_v,
            tolerances.voltage_v,
        ),
        EdgeDirection::Fall => measure_fall_time(
            &waveform.time,
            &channel.samples,
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
            Outcome::Pass
        } else {
            Outcome::Fail
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
        },
    ))
}

fn measurement_error(error: MeasurementError) -> WaveformError {
    match error {
        MeasurementError::EmptyInput => WaveformError::EmptyInput,
        MeasurementError::MismatchedLength => WaveformError::InvalidWaveform {
            reason: "measurement time and sample arrays must have the same length".to_string(),
        },
        MeasurementError::NonMonotonicTimeAxis => WaveformError::InvalidWaveform {
            reason: "time samples must be strictly increasing for duration measurements"
                .to_string(),
        },
    }
}

fn opposite_state(state: SignalState) -> SignalState {
    state.opposite()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::criteria::{EdgeDirection, SignalState, TransientEventKind};
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
