use ferrisoxide_core::analysis::evaluate_criteria_with_measurements;
use ferrisoxide_core::config::AnalysisConfig;
use ferrisoxide_core::csv::{SimpleCsvParser, WaveformParser};
use ferrisoxide_core::report::{AnalysisReport, ReportEvidenceContext};

fn render_report(input_name: &str, csv_input: &str, config_input: &str) -> String {
    let config: AnalysisConfig = toml::from_str(config_input).expect("config should parse");
    config.validate().expect("config should validate");

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
        results: evaluation.results,
    };

    report.render_json_pretty().expect("json should render")
}

fn assert_heated_actuator_report(input_file: &str, expected_file: &str) {
    let rendered = render_report(
        &format!("tests/e2e/heated_actuator/input/{input_file}"),
        match input_file {
            "passing_run.csv" => {
                include_str!("../../../tests/e2e/heated_actuator/input/passing_run.csv")
            }
            "failing_late_response.csv" => {
                include_str!("../../../tests/e2e/heated_actuator/input/failing_late_response.csv")
            }
            "failing_transient_event.csv" => {
                include_str!("../../../tests/e2e/heated_actuator/input/failing_transient_event.csv")
            }
            "failing_supply_dropout.csv" => {
                include_str!("../../../tests/e2e/heated_actuator/input/failing_supply_dropout.csv")
            }
            _ => panic!("unknown heated actuator fixture {input_file}"),
        },
        include_str!("../../../tests/e2e/heated_actuator/configs/test-verification-config.toml"),
    );

    let expected = match expected_file {
        "passing_report.json" => {
            include_str!("../../../tests/e2e/heated_actuator/expected/passing_report.json")
        }
        "failing_late_response_report.json" => include_str!(
            "../../../tests/e2e/heated_actuator/expected/failing_late_response_report.json"
        ),
        "failing_transient_event_report.json" => include_str!(
            "../../../tests/e2e/heated_actuator/expected/failing_transient_event_report.json"
        ),
        "failing_supply_dropout_report.json" => include_str!(
            "../../../tests/e2e/heated_actuator/expected/failing_supply_dropout_report.json"
        ),
        _ => panic!("unknown heated actuator expected report {expected_file}"),
    };

    assert_eq!(rendered, expected.trim_end());
}

#[test]
fn heated_actuator_passing_run_matches_golden_report() {
    assert_heated_actuator_report("passing_run.csv", "passing_report.json");
}

#[test]
fn heated_actuator_late_response_matches_golden_report() {
    assert_heated_actuator_report(
        "failing_late_response.csv",
        "failing_late_response_report.json",
    );
}

#[test]
fn heated_actuator_transient_event_matches_golden_report() {
    assert_heated_actuator_report(
        "failing_transient_event.csv",
        "failing_transient_event_report.json",
    );
}

#[test]
fn heated_actuator_supply_dropout_matches_golden_report() {
    assert_heated_actuator_report(
        "failing_supply_dropout.csv",
        "failing_supply_dropout_report.json",
    );
}
