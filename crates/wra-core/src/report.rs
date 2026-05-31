use crate::analysis::{AnalysisResult, Outcome};
use crate::error::{Result, WaveformError};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq)]
pub struct AnalysisReport {
    pub input_name: String,
    pub results: Vec<AnalysisResult>,
}

impl AnalysisReport {
    pub fn overall_outcome(&self) -> Outcome {
        if self
            .results
            .iter()
            .any(|result| result.outcome == Outcome::Fail)
        {
            Outcome::Fail
        } else {
            Outcome::Pass
        }
    }

    pub fn render_text(&self) -> String {
        let mut output = String::new();
        output.push_str("Waveform Analysis Report\n");
        output.push_str(&format!("Input: {}\n", self.input_name));
        output.push_str(&format!("Overall: {:?}\n", self.overall_outcome()));
        output.push_str("Criteria:\n");

        for result in &self.results {
            output.push_str(&format!(
                "- {}: {:?} channel={} measured={:.6} {} required={:.6} {} sample_index={} timestamp={:.6} reason={}\n",
                result.criterion_id,
                result.outcome,
                result.channel,
                result.measured_value,
                result.unit,
                result.required_value,
                result.unit,
                result.sample_index,
                result.timestamp,
                result.reason
            ));
        }

        output
    }

    pub fn render_json_pretty(&self) -> Result<String> {
        let document = JsonReport {
            input_name: &self.input_name,
            overall_outcome: self.overall_outcome(),
            results: &self.results,
        };
        serde_json::to_string_pretty(&document).map_err(|error| {
            WaveformError::ReportSerialization {
                message: error.to_string(),
            }
        })
    }
}

#[derive(Serialize)]
struct JsonReport<'a> {
    input_name: &'a str,
    overall_outcome: Outcome,
    results: &'a [AnalysisResult],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_text_report() {
        let report = AnalysisReport {
            input_name: "fixture.csv".to_string(),
            results: vec![AnalysisResult {
                criterion_id: "max".to_string(),
                outcome: Outcome::Pass,
                failed_criterion: None,
                channel: "input_v".to_string(),
                measured_value: 5.0,
                required_value: 5.5,
                unit: "V".to_string(),
                sample_index: 1,
                timestamp: 0.001,
                reason: "ok".to_string(),
            }],
        };

        let rendered = report.render_text();

        assert!(rendered.contains("Waveform Analysis Report"));
        assert!(rendered.contains("Overall: Pass"));
        assert!(rendered.contains("max"));
    }

    #[test]
    fn renders_json_report() {
        let report = AnalysisReport {
            input_name: "fixture.csv".to_string(),
            results: vec![AnalysisResult {
                criterion_id: "max".to_string(),
                outcome: Outcome::Pass,
                failed_criterion: None,
                channel: "input_v".to_string(),
                measured_value: 5.0,
                required_value: 5.5,
                unit: "V".to_string(),
                sample_index: 1,
                timestamp: 0.001,
                reason: "ok".to_string(),
            }],
        };

        let rendered = report.render_json_pretty().expect("json should render");

        assert!(rendered.contains("\"overall_outcome\": \"pass\""));
        assert!(rendered.contains("\"criterion_id\": \"max\""));
    }
}
