use ferrisoxide_core::analysis::{evaluate_criteria_with_measurements, AnalysisResult};
use ferrisoxide_core::criteria::{
    Criterion, CriterionOperator, MeasurementRequirement, MeasurementSpec, RunSelectionConfig,
    SignalState, TransientEventKind,
};
use ferrisoxide_core::csv::{CsvParseOptions, SimpleCsvParser, WaveformParser};
use ferrisoxide_core::model::{TolerancePolicy, Waveform};
use ferrisoxide_rule_engine::{
    evaluate_borrowed_rule, BorrowedRuleCriterion, BorrowedRuleCriterionCheck, RuleChannel,
    RuleOutcome, RuleSummary, RuleTolerances, RuleWaveform,
};
use ferrisoxide_rule_schema::{
    ComparisonOperator, CriterionDefinition, EngineeringUnit, MeasurementDefinition,
    RequirementDefinition, RulePackage, RunSelection,
    TransientEventKind as SchemaTransientEventKind, UnitValue,
};
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
struct ParityReport {
    case_id: &'static str,
    waveform: &'static str,
    rule_package: &'static str,
    schema_note: &'static str,
    desktop_results: Vec<PortableEvidence>,
    embedded_results: Vec<PortableEvidence>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct PortableEvidence {
    criterion_id: String,
    outcome: &'static str,
    failed_criterion: Option<String>,
    measurement_id: String,
    method: String,
    channel: String,
    measured_value: f64,
    required_value: f64,
    tolerance_used: f64,
    unit: String,
    sample_index: usize,
    timestamp: f64,
}

#[test]
fn desktop_and_embedded_rule_paths_match_expected_evidence() {
    let package = ferrisoxide_rule_schema::parse_rule_package_toml(include_str!(
        "../../../tests/parity/rules_001.toml"
    ))
    .expect("rule package should parse");
    package.validate().expect("rule package should validate");

    let waveform = parse_parity_waveform(&package);
    let desktop_results = evaluate_desktop_path(&package, &waveform);
    let embedded_results = evaluate_embedded_compatible_path(&package, &waveform);

    assert_eq!(desktop_results, embedded_results);

    let report = ParityReport {
        case_id: "m8_008_rule_parity_001",
        waveform: "tests/parity/waveform_001.csv",
        rule_package: "tests/parity/rules_001.toml",
        schema_note: "Desktop reports include human-readable reason text and method context; parity compares the portable evidence fields shared by desktop analysis results and embedded-compatible rule summaries.",
        desktop_results,
        embedded_results,
    };
    let rendered = serde_json::to_string_pretty(&report).expect("parity report should render");

    assert_eq!(
        rendered,
        include_str!("../../../tests/parity/expected_result.json").trim_end()
    );
}

fn parse_parity_waveform(package: &RulePackage) -> Waveform {
    let channel_columns = package
        .channels
        .iter()
        .map(|channel| channel.name.clone())
        .collect::<Vec<_>>();
    let parser = SimpleCsvParser;
    parser
        .parse_str(
            include_str!("../../../tests/parity/waveform_001.csv"),
            &CsvParseOptions::new("time_s", channel_columns),
        )
        .expect("parity waveform should parse")
}

fn evaluate_desktop_path(package: &RulePackage, waveform: &Waveform) -> Vec<PortableEvidence> {
    let criteria = package
        .criteria
        .iter()
        .map(core_criterion)
        .collect::<Vec<_>>();
    let evaluation =
        evaluate_criteria_with_measurements(waveform, &criteria, TolerancePolicy::default())
            .expect("desktop path should evaluate");

    evaluation
        .results
        .iter()
        .map(|result| {
            let measurement = evaluation
                .measurements
                .iter()
                .find(|measurement| measurement.id == result.measurement_id)
                .expect("desktop result should link to a measurement");
            portable_from_desktop(result, measurement.method.as_str())
        })
        .collect()
}

fn evaluate_embedded_compatible_path(
    package: &RulePackage,
    waveform_data: &Waveform,
) -> Vec<PortableEvidence> {
    let channels = waveform_data
        .channels
        .iter()
        .map(|channel| RuleChannel {
            name: channel.name.as_str(),
            samples: channel.samples.as_slice(),
        })
        .collect::<Vec<_>>();
    let waveform = RuleWaveform {
        time: waveform_data.time.as_slice(),
        channels: &channels,
    };

    package
        .criteria
        .iter()
        .map(|criterion| {
            let borrowed = borrowed_criterion(criterion);
            let summary = evaluate_borrowed_rule(waveform, borrowed, RuleTolerances::default())
                .expect("embedded-compatible path should evaluate");
            portable_from_summary(summary)
        })
        .collect()
}

fn portable_from_desktop(result: &AnalysisResult, method: &str) -> PortableEvidence {
    PortableEvidence {
        criterion_id: result.criterion_id.clone(),
        outcome: match result.outcome {
            ferrisoxide_core::analysis::Outcome::Pass => "pass",
            ferrisoxide_core::analysis::Outcome::Fail => "fail",
        },
        failed_criterion: result.failed_criterion.clone(),
        measurement_id: result.measurement_id.clone(),
        method: method.to_string(),
        channel: result.channel.clone(),
        measured_value: result.measured_value,
        required_value: result.required_value,
        tolerance_used: result.tolerance_used,
        unit: result.unit.clone(),
        sample_index: result.sample_index,
        timestamp: result.timestamp,
    }
}

fn portable_from_summary(summary: RuleSummary<'_>) -> PortableEvidence {
    PortableEvidence {
        criterion_id: summary.criterion_id.to_string(),
        outcome: match summary.outcome {
            RuleOutcome::Pass => "pass",
            RuleOutcome::Fail => "fail",
        },
        failed_criterion: summary
            .failed_criterion
            .map(std::string::ToString::to_string),
        measurement_id: format!("{}_measurement", summary.criterion_id),
        method: summary.method.to_string(),
        channel: summary.channel.to_string(),
        measured_value: summary.measured_value,
        required_value: summary.required_value,
        tolerance_used: summary.tolerance_used,
        unit: summary.unit.to_string(),
        sample_index: summary.sample_index,
        timestamp: summary.timestamp,
    }
}

fn core_criterion(criterion: &CriterionDefinition) -> Criterion {
    Criterion::measurement(
        criterion.id.clone(),
        criterion.channel.clone(),
        core_measurement(&criterion.measurement),
        core_requirement(&criterion.requirement),
    )
}

fn core_measurement(measurement: &MeasurementDefinition) -> MeasurementSpec {
    match measurement {
        MeasurementDefinition::MinimumSample => MeasurementSpec::MinimumSample,
        MeasurementDefinition::MaximumSample => MeasurementSpec::MaximumSample,
        MeasurementDefinition::StateTransitionCount { threshold } => {
            MeasurementSpec::StateTransitionCount {
                threshold_v: volts(threshold),
            }
        }
        MeasurementDefinition::PulseWidth {
            state,
            threshold,
            selection,
        } => MeasurementSpec::PulseWidth {
            state: core_state(*state),
            threshold_v: volts(threshold),
            selection: core_selection(selection.unwrap_or(RunSelection::Shortest)),
        },
        MeasurementDefinition::StableStateDuration { state, threshold } => {
            MeasurementSpec::StableStateDuration {
                state: core_state(*state),
                threshold_v: volts(threshold),
            }
        }
        MeasurementDefinition::TransientEventDuration {
            event_kind,
            expected_state,
            threshold,
        } => MeasurementSpec::TransientEventDuration {
            event_kind: core_event_kind(*event_kind),
            expected_state: core_state(*expected_state),
            threshold_v: volts(threshold),
        },
        MeasurementDefinition::RiseTime {
            low_threshold,
            high_threshold,
        } => MeasurementSpec::RiseTime {
            low_threshold_v: volts(low_threshold),
            high_threshold_v: volts(high_threshold),
        },
        MeasurementDefinition::FallTime {
            low_threshold,
            high_threshold,
        } => MeasurementSpec::FallTime {
            low_threshold_v: volts(low_threshold),
            high_threshold_v: volts(high_threshold),
        },
    }
}

fn core_requirement(requirement: &RequirementDefinition) -> MeasurementRequirement {
    MeasurementRequirement {
        operator: core_operator(requirement.operator),
        value: requirement.value.value,
    }
}

fn borrowed_criterion(criterion: &CriterionDefinition) -> BorrowedRuleCriterion<'_> {
    BorrowedRuleCriterion {
        id: criterion.id.as_str(),
        check: match &criterion.measurement {
            MeasurementDefinition::MinimumSample => BorrowedRuleCriterionCheck::MinimumVoltage {
                channel: criterion.channel.as_str(),
                threshold_v: requirement_value(&criterion.requirement),
            },
            MeasurementDefinition::MaximumSample => BorrowedRuleCriterionCheck::MaximumVoltage {
                channel: criterion.channel.as_str(),
                threshold_v: requirement_value(&criterion.requirement),
            },
            MeasurementDefinition::StateTransitionCount { threshold } => {
                BorrowedRuleCriterionCheck::StateTransitions {
                    channel: criterion.channel.as_str(),
                    threshold_v: volts(threshold),
                    expected_count: requirement_value(&criterion.requirement) as usize,
                }
            }
            MeasurementDefinition::PulseWidth {
                state,
                threshold,
                selection,
            } => {
                let width = requirement_value(&criterion.requirement);
                match criterion.requirement.operator {
                    ComparisonOperator::GreaterThan | ComparisonOperator::GreaterThanOrEqual => {
                        BorrowedRuleCriterionCheck::PulseWidth {
                            channel: criterion.channel.as_str(),
                            state: core_state(*state),
                            threshold_v: volts(threshold),
                            min_width_s: Some(width),
                            max_width_s: None,
                        }
                    }
                    ComparisonOperator::LessThan | ComparisonOperator::LessThanOrEqual => {
                        BorrowedRuleCriterionCheck::PulseWidth {
                            channel: criterion.channel.as_str(),
                            state: core_state(*state),
                            threshold_v: volts(threshold),
                            min_width_s: None,
                            max_width_s: Some(width),
                        }
                    }
                    ComparisonOperator::EqualTo => {
                        let selection = selection.unwrap_or(RunSelection::Shortest);
                        BorrowedRuleCriterionCheck::PulseWidth {
                            channel: criterion.channel.as_str(),
                            state: core_state(*state),
                            threshold_v: volts(threshold),
                            min_width_s: (selection == RunSelection::Shortest).then_some(width),
                            max_width_s: (selection == RunSelection::Longest).then_some(width),
                        }
                    }
                }
            }
            MeasurementDefinition::StableStateDuration { state, threshold } => {
                BorrowedRuleCriterionCheck::StableStateDuration {
                    channel: criterion.channel.as_str(),
                    state: core_state(*state),
                    threshold_v: volts(threshold),
                    min_duration_s: requirement_value(&criterion.requirement),
                }
            }
            MeasurementDefinition::TransientEventDuration {
                event_kind,
                expected_state,
                threshold,
            } => BorrowedRuleCriterionCheck::TransientEvent {
                channel: criterion.channel.as_str(),
                event_kind: schema_event_kind_name(*event_kind),
                expected_state: core_state(*expected_state),
                threshold_v: volts(threshold),
                max_duration_s: requirement_value(&criterion.requirement),
            },
            MeasurementDefinition::RiseTime {
                low_threshold,
                high_threshold,
            } => BorrowedRuleCriterionCheck::RiseFallTime {
                channel: criterion.channel.as_str(),
                direction: ferrisoxide_core::criteria::EdgeDirection::Rise,
                low_threshold_v: volts(low_threshold),
                high_threshold_v: volts(high_threshold),
                max_duration_s: requirement_value(&criterion.requirement),
            },
            MeasurementDefinition::FallTime {
                low_threshold,
                high_threshold,
            } => BorrowedRuleCriterionCheck::RiseFallTime {
                channel: criterion.channel.as_str(),
                direction: ferrisoxide_core::criteria::EdgeDirection::Fall,
                low_threshold_v: volts(low_threshold),
                high_threshold_v: volts(high_threshold),
                max_duration_s: requirement_value(&criterion.requirement),
            },
        },
    }
}

fn core_operator(operator: ComparisonOperator) -> CriterionOperator {
    match operator {
        ComparisonOperator::LessThan => CriterionOperator::LessThan,
        ComparisonOperator::LessThanOrEqual => CriterionOperator::LessThanOrEqual,
        ComparisonOperator::GreaterThan => CriterionOperator::GreaterThan,
        ComparisonOperator::GreaterThanOrEqual => CriterionOperator::GreaterThanOrEqual,
        ComparisonOperator::EqualTo => CriterionOperator::EqualTo,
    }
}

fn core_state(state: ferrisoxide_rule_schema::SignalState) -> SignalState {
    match state {
        ferrisoxide_rule_schema::SignalState::High => SignalState::High,
        ferrisoxide_rule_schema::SignalState::Low => SignalState::Low,
    }
}

fn core_selection(selection: RunSelection) -> RunSelectionConfig {
    match selection {
        RunSelection::Shortest => RunSelectionConfig::Shortest,
        RunSelection::Longest => RunSelectionConfig::Longest,
    }
}

fn core_event_kind(kind: SchemaTransientEventKind) -> TransientEventKind {
    match kind {
        SchemaTransientEventKind::TransientEvent => TransientEventKind::TransientEvent,
        SchemaTransientEventKind::SpuriousTransition => TransientEventKind::SpuriousTransition,
        SchemaTransientEventKind::ContactBounce => TransientEventKind::ContactBounce,
        SchemaTransientEventKind::Dropout => TransientEventKind::Dropout,
        SchemaTransientEventKind::NoiseInducedTransition => {
            TransientEventKind::NoiseInducedTransition
        }
        SchemaTransientEventKind::ThresholdCrossingEvent => {
            TransientEventKind::ThresholdCrossingEvent
        }
    }
}

fn schema_event_kind_name(kind: SchemaTransientEventKind) -> &'static str {
    match kind {
        SchemaTransientEventKind::TransientEvent => "transient_event",
        SchemaTransientEventKind::SpuriousTransition => "spurious_transition",
        SchemaTransientEventKind::ContactBounce => "contact_bounce",
        SchemaTransientEventKind::Dropout => "dropout",
        SchemaTransientEventKind::NoiseInducedTransition => "noise_induced_transition",
        SchemaTransientEventKind::ThresholdCrossingEvent => "threshold_crossing_event",
    }
}

fn volts(value: &UnitValue) -> f64 {
    assert_eq!(value.unit, EngineeringUnit::Volt);
    value.value
}

fn requirement_value(requirement: &RequirementDefinition) -> f64 {
    requirement.value.value
}
