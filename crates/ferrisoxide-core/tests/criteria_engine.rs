use ferrisoxide_core::analysis::{
    evaluate_criteria_with_measurements, evaluate_criteria_with_tolerances,
};
use ferrisoxide_core::config::AnalysisConfig;
use ferrisoxide_core::csv::{SimpleCsvParser, WaveformParser};
use ferrisoxide_core::report::{AnalysisReport, ReportEvidenceContext};

fn render_report(input_name: &str, csv_input: &str, config_input: &str) -> String {
    let config: AnalysisConfig = toml::from_str(config_input).expect("config should parse");
    let parser = SimpleCsvParser;
    let waveform = parser
        .parse_str(csv_input, &config.csv_options())
        .expect("waveform should parse")
        .with_source_name(input_name.to_string())
        .with_metadata_context(&config.metadata)
        .with_tolerance_policy(config.tolerances);
    let criteria = config.criteria().expect("criteria should convert");
    let evaluation = evaluate_criteria_with_measurements(&waveform, &criteria, config.tolerances)
        .expect("criteria should evaluate");
    let report = AnalysisReport {
        input_name: input_name.to_string(),
        waveform_metadata: waveform.metadata.clone(),
        evidence_context: ReportEvidenceContext::engineering_validation(config.tolerances),
        measurements: evaluation.measurements,
        feature_records: Vec::new(),
        event_records: Vec::new(),
        event_validations: Vec::new(),
        results: evaluation.results,
    };

    report.render_json_pretty().expect("json should render")
}

fn assert_legacy_and_dsl_reports_match(
    input_name: &str,
    csv_input: &str,
    legacy_config_input: &str,
    dsl_config_input: &str,
    expected_json: &str,
) {
    let legacy = render_report(input_name, csv_input, legacy_config_input);
    let dsl = render_report(input_name, csv_input, dsl_config_input);

    assert_eq!(legacy, expected_json.trim_end());
    assert_eq!(dsl, legacy);
}

#[test]
fn clean_square_wave_matches_golden_report() {
    let rendered = render_report(
        "tests/fixtures/clean_square_wave.csv",
        include_str!("../../../tests/fixtures/clean_square_wave.csv"),
        include_str!("../../../tests/configs/criteria-engine-pass.toml"),
    );

    assert_eq!(
        rendered,
        include_str!("../../../tests/golden/criteria_engine_pass.json").trim_end()
    );
}

#[test]
fn clean_square_wave_dsl_matches_legacy_golden_report() {
    assert_legacy_and_dsl_reports_match(
        "tests/fixtures/clean_square_wave.csv",
        include_str!("../../../tests/fixtures/clean_square_wave.csv"),
        include_str!("../../../tests/configs/criteria-engine-pass.toml"),
        include_str!("../../../tests/configs/criteria-engine-pass-dsl.toml"),
        include_str!("../../../tests/golden/criteria_engine_pass.json"),
    );
}

#[test]
fn dropout_transient_event_matches_golden_report() {
    let rendered = render_report(
        "tests/fixtures/dropout_event.csv",
        include_str!("../../../tests/fixtures/dropout_event.csv"),
        include_str!("../../../tests/configs/transient-event-dropout-fail.toml"),
    );

    assert_eq!(
        rendered,
        include_str!("../../../tests/golden/transient_event_dropout_fail.json").trim_end()
    );
}

#[test]
fn dropout_transient_event_dsl_matches_legacy_golden_report() {
    assert_legacy_and_dsl_reports_match(
        "tests/fixtures/dropout_event.csv",
        include_str!("../../../tests/fixtures/dropout_event.csv"),
        include_str!("../../../tests/configs/transient-event-dropout-fail.toml"),
        include_str!("../../../tests/configs/transient-event-dropout-fail-dsl.toml"),
        include_str!("../../../tests/golden/transient_event_dropout_fail.json"),
    );
}

#[test]
fn slow_rise_fall_matches_golden_report() {
    let rendered = render_report(
        "tests/fixtures/slow_rise_fall_signal.csv",
        include_str!("../../../tests/fixtures/slow_rise_fall_signal.csv"),
        include_str!("../../../tests/configs/slow-rise-fail.toml"),
    );

    assert_eq!(
        rendered,
        include_str!("../../../tests/golden/slow_rise_fail.json").trim_end()
    );
}

#[test]
fn slow_rise_fall_dsl_matches_legacy_golden_report() {
    assert_legacy_and_dsl_reports_match(
        "tests/fixtures/slow_rise_fall_signal.csv",
        include_str!("../../../tests/fixtures/slow_rise_fall_signal.csv"),
        include_str!("../../../tests/configs/slow-rise-fail.toml"),
        include_str!("../../../tests/configs/slow-rise-fail-dsl.toml"),
        include_str!("../../../tests/golden/slow_rise_fail.json"),
    );
}

#[test]
fn multi_channel_fixture_targets_configured_channels() {
    let config: AnalysisConfig =
        toml::from_str(include_str!("../../../tests/configs/multi-channel.toml"))
            .expect("config should parse");
    let parser = SimpleCsvParser;
    let waveform = parser
        .parse_str(
            include_str!("../../../tests/fixtures/multi_channel.csv"),
            &config.csv_options(),
        )
        .expect("multi-channel waveform should parse");
    let criteria = config.criteria().expect("criteria should convert");
    let results = evaluate_criteria_with_tolerances(&waveform, &criteria, config.tolerances)
        .expect("criteria should evaluate");

    assert_eq!(waveform.channels.len(), 3);
    assert_eq!(results[0].channel, "supply_v");
    assert_eq!(
        results[0].failed_criterion,
        Some("supply_dropout_transient_event".to_string())
    );
    assert_eq!(results[1].channel, "control_v");
    assert_eq!(results[2].channel, "output_v");
}

#[test]
fn bounce_fixture_detects_short_transient() {
    let rendered = render_report(
        "tests/fixtures/analog_switch_bounce_transients.csv",
        include_str!("../../../tests/fixtures/analog_switch_bounce_transients.csv"),
        include_str!("../../../tests/configs/bounce-transient.toml"),
    );

    assert!(rendered.contains("\"overall_outcome\": \"fail\""));
    assert!(rendered.contains("\"criterion_id\": \"switch_bounce_transient_max\""));
    assert!(rendered.contains("\"sample_index\": 4"));
}

#[test]
fn noisy_square_wave_transition_count_is_stable() {
    let config: AnalysisConfig = toml::from_str(include_str!(
        "../../../tests/configs/criteria-engine-pass.toml"
    ))
    .expect("config should parse");
    let parser = SimpleCsvParser;
    let waveform = parser
        .parse_str(
            include_str!("../../../tests/fixtures/noisy_square_wave.csv"),
            &config.csv_options(),
        )
        .expect("noisy waveform should parse");
    let criteria = config.criteria().expect("criteria should convert");
    let results = evaluate_criteria_with_tolerances(&waveform, &criteria, config.tolerances)
        .expect("criteria should evaluate");

    assert_eq!(results[0].measured_value, 4.0);
    assert_eq!(results[0].failed_criterion, None);
}

#[test]
fn validation_known_answer_square_wave_matches_expected_report() {
    let rendered = render_report(
        "validation/known_answer/square_wave_tolerance.csv",
        include_str!("../../../validation/known_answer/square_wave_tolerance.csv"),
        include_str!("../../../validation/known_answer/square_wave_tolerance.toml"),
    );

    assert_eq!(
        rendered,
        include_str!("../../../validation/reports/square_wave_tolerance.json").trim_end()
    );
}

#[test]
fn validation_dropout_environmental_case_matches_expected_report() {
    let rendered = render_report(
        "validation/environmental_cases/dropout_event.csv",
        include_str!("../../../validation/environmental_cases/dropout_event.csv"),
        include_str!("../../../validation/environmental_cases/dropout_event.toml"),
    );

    assert_eq!(
        rendered,
        include_str!("../../../validation/reports/environmental_dropout_fail.json").trim_end()
    );
}

#[test]
fn validation_contact_bounce_environmental_case_matches_expected_report() {
    let rendered = render_report(
        "validation/environmental_cases/contact_bounce.csv",
        include_str!("../../../validation/environmental_cases/contact_bounce.csv"),
        include_str!("../../../validation/environmental_cases/contact_bounce.toml"),
    );

    assert_eq!(
        rendered,
        include_str!("../../../validation/reports/environmental_contact_bounce_fail.json")
            .trim_end()
    );
}

#[test]
fn validation_measurement_engine_known_answer_matches_expected_report() {
    let rendered = render_report(
        "validation/measurement_engine/known_answer_measurements.csv",
        include_str!("../../../validation/measurement_engine/known_answer_measurements.csv"),
        include_str!("../../../validation/measurement_engine/known_answer_measurements.toml"),
    );

    assert_eq!(
        rendered,
        include_str!("../../../validation/reports/measurement_engine_known_answer.json").trim_end()
    );
}

#[test]
fn validation_measurement_engine_dsl_matches_legacy_golden_report() {
    assert_legacy_and_dsl_reports_match(
        "validation/measurement_engine/known_answer_measurements.csv",
        include_str!("../../../validation/measurement_engine/known_answer_measurements.csv"),
        include_str!("../../../validation/measurement_engine/known_answer_measurements.toml"),
        include_str!("../../../validation/measurement_engine/known_answer_measurements_dsl.toml"),
        include_str!("../../../validation/reports/measurement_engine_known_answer.json"),
    );
}

#[test]
fn dsl_criteria_evaluate_through_measurement_records() {
    let config: AnalysisConfig = toml::from_str(
        r#"
[input]
time_column = "time_s"
channels = ["control_v", "supply_v", "rise_v", "fall_v"]
time_unit = "s"
signal_unit = "V"

[tolerances]
voltage_v = 0.0
time_s = 0.0005

[[criteria]]
id = "dsl_control_min"
channel = "control_v"

[criteria.measurement]
type = "minimum_sample"

[criteria.requirement]
operator = "greater_than_or_equal"
value = 0.0
unit = "V"

[[criteria]]
id = "dsl_supply_max"
channel = "supply_v"

[criteria.measurement]
type = "maximum_sample"

[criteria.requirement]
operator = "less_than_or_equal"
value = 5.0
unit = "V"

[[criteria]]
id = "dsl_control_transition_count"
channel = "control_v"

[criteria.measurement]
type = "state_transition_count"
threshold = { value = 2.5, unit = "V" }

[criteria.requirement]
operator = "equal_to"
value = 4
unit = "count"

[[criteria]]
id = "dsl_control_high_pulse_width"
channel = "control_v"

[criteria.measurement]
type = "pulse_width"
threshold = { value = 2.5, unit = "V" }
state = "high"

[criteria.requirement]
operator = "greater_than_or_equal"
value = 0.0015
unit = "s"

[[criteria]]
id = "dsl_control_stable_low"
channel = "control_v"

[criteria.measurement]
type = "stable_state_duration"
threshold = { value = 2.5, unit = "V" }
state = "low"

[criteria.requirement]
operator = "greater_than_or_equal"
value = 0.002
unit = "s"

[[criteria]]
id = "dsl_supply_dropout"
channel = "supply_v"

[criteria.measurement]
type = "transient_event_duration"
threshold = { value = 2.5, unit = "V" }
expected_state = "high"
event_kind = "dropout"

[criteria.requirement]
operator = "less_than_or_equal"
value = 0.0015
unit = "s"

[[criteria]]
id = "dsl_rise_time"
channel = "rise_v"

[criteria.measurement]
type = "rise_time"
low_threshold = { value = 0.5, unit = "V" }
high_threshold = { value = 4.5, unit = "V" }

[criteria.requirement]
operator = "less_than_or_equal"
value = 0.0015
unit = "s"

[[criteria]]
id = "dsl_fall_time"
channel = "fall_v"

[criteria.measurement]
type = "fall_time"
low_threshold = { value = 0.5, unit = "V" }
high_threshold = { value = 4.5, unit = "V" }

[criteria.requirement]
operator = "less_than_or_equal"
value = 0.0015
unit = "s"
"#,
    )
    .expect("DSL config should parse");
    let parser = SimpleCsvParser;
    let waveform = parser
        .parse_str(
            include_str!("../../../validation/measurement_engine/known_answer_measurements.csv"),
            &config.csv_options(),
        )
        .expect("waveform should parse");
    let criteria = config.criteria().expect("DSL criteria should convert");
    let evaluation = evaluate_criteria_with_measurements(&waveform, &criteria, config.tolerances)
        .expect("DSL criteria should evaluate");

    assert_eq!(evaluation.results.len(), 8);
    assert_eq!(evaluation.measurements.len(), 8);
    for result in &evaluation.results {
        assert_eq!(result.failed_criterion, None);
        assert_eq!(
            result.measurement_id,
            format!("{}_measurement", result.criterion_id)
        );
    }

    let methods = evaluation
        .measurements
        .iter()
        .map(|measurement| measurement.method.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        methods,
        vec![
            "minimum_sample",
            "maximum_sample",
            "state_transition_count",
            "state_run_duration",
            "state_run_duration",
            "state_run_duration",
            "edge_time",
            "edge_time",
        ]
    );
    assert_eq!(evaluation.results[2].measured_value, 4.0);
    assert_eq!(evaluation.results[2].required_value, 4.0);
    assert_eq!(evaluation.results[2].sample_index, 2);
    assert_eq!(evaluation.results[2].timestamp, 0.002);
    assert_eq!(evaluation.results[2].channel, "control_v");
}
