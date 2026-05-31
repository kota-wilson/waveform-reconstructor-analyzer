use wra_core::analysis::evaluate_criteria_with_tolerances;
use wra_core::config::AnalysisConfig;
use wra_core::csv::{SimpleCsvParser, WaveformParser};
use wra_core::report::{AnalysisReport, ReportEvidenceContext};

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
    let results = evaluate_criteria_with_tolerances(&waveform, &criteria, config.tolerances)
        .expect("criteria should evaluate");
    let report = AnalysisReport {
        input_name: input_name.to_string(),
        waveform_metadata: waveform.metadata.clone(),
        evidence_context: ReportEvidenceContext::engineering_validation(config.tolerances),
        results,
    };

    report.render_json_pretty().expect("json should render")
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
