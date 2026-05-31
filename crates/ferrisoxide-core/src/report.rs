use crate::analysis::{AnalysisResult, MeasurementRecord, Outcome};
use crate::error::{Result, WaveformError};
use crate::model::{TolerancePolicy, WaveformMetadata};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq)]
pub struct AnalysisReport {
    pub input_name: String,
    pub waveform_metadata: WaveformMetadata,
    pub evidence_context: ReportEvidenceContext,
    pub measurements: Vec<MeasurementRecord>,
    pub results: Vec<AnalysisResult>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ReportEvidenceContext {
    pub validation_profile: String,
    pub evidence_source: String,
    pub tolerance_policy: TolerancePolicy,
    pub confidence_notes: Vec<String>,
}

impl ReportEvidenceContext {
    pub fn engineering_validation(tolerance_policy: TolerancePolicy) -> Self {
        Self {
            validation_profile: "engineering_validation".to_string(),
            evidence_source: "local_file_analysis".to_string(),
            tolerance_policy,
            confidence_notes: vec![
                "software validation evidence only".to_string(),
                "not hardware qualification or certification evidence".to_string(),
            ],
        }
    }
}

impl Default for ReportEvidenceContext {
    fn default() -> Self {
        Self::engineering_validation(TolerancePolicy::default())
    }
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
        output.push_str(&format!(
            "Samples: {} Channels: {} Lineage: {:?}\n",
            self.waveform_metadata.sample_count,
            self.waveform_metadata.channel_count,
            self.waveform_metadata.lineage
        ));
        if let Some(interval) = &self.waveform_metadata.sample_interval {
            output.push_str(&format!(
                "Sample Interval: nominal={:.9} {} min={:.9} max={:.9} uniform={}\n",
                interval.nominal, interval.unit.name, interval.min, interval.max, interval.uniform
            ));
        }
        if let Some(sample_rate_hz) = self.waveform_metadata.nominal_sample_rate_hz {
            output.push_str(&format!("Nominal Sample Rate: {:.6} Hz\n", sample_rate_hz));
        }
        if !self.waveform_metadata.transform_history.is_empty() {
            output.push_str(&format!(
                "Transforms: {}\n",
                self.waveform_metadata.transform_history.join(" -> ")
            ));
        }
        output.push_str(&format!(
            "Validation Profile: {}\n",
            self.evidence_context.validation_profile
        ));
        output.push_str(&format!(
            "Evidence Source: {}\n",
            self.evidence_context.evidence_source
        ));
        output.push_str(&format!(
            "Tolerance Policy: voltage={:.6} V time={:.9} s\n",
            self.evidence_context.tolerance_policy.voltage_v,
            self.evidence_context.tolerance_policy.time_s
        ));
        if !self.evidence_context.confidence_notes.is_empty() {
            output.push_str(&format!(
                "Confidence Notes: {}\n",
                self.evidence_context.confidence_notes.join("; ")
            ));
        }
        output.push_str(&format!("Overall: {:?}\n", self.overall_outcome()));
        output.push_str("Measurements:\n");

        for measurement in &self.measurements {
            output.push_str(&format!(
                "- {}: method={} channel={} measured={:.6} {} sample_index={} timestamp={:.6}\n",
                measurement.id,
                measurement.method,
                measurement.channel,
                measurement.measured_value,
                measurement.unit,
                measurement.sample_index,
                measurement.timestamp
            ));
        }

        output.push_str("Criteria:\n");

        for result in &self.results {
            output.push_str(&format!(
                "- {}: {:?} measurement_id={} channel={} measured={:.6} {} required={:.6} {} tolerance={:.6} sample_index={} timestamp={:.6} reason={}\n",
                result.criterion_id,
                result.outcome,
                result.measurement_id,
                result.channel,
                result.measured_value,
                result.unit,
                result.required_value,
                result.unit,
                result.tolerance_used,
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
            waveform_metadata: &self.waveform_metadata,
            evidence_context: &self.evidence_context,
            overall_outcome: self.overall_outcome(),
            measurements: &self.measurements,
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
    waveform_metadata: &'a WaveformMetadata,
    evidence_context: &'a ReportEvidenceContext,
    overall_outcome: Outcome,
    measurements: &'a [MeasurementRecord],
    results: &'a [AnalysisResult],
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Channel, Unit, Waveform};

    fn metadata() -> WaveformMetadata {
        Waveform::new(
            vec![0.0, 0.001],
            vec![Channel::new("input_v", Unit::volts(), vec![0.0, 5.0])],
        )
        .expect("waveform should be valid")
        .metadata
    }

    fn measurement() -> MeasurementRecord {
        MeasurementRecord {
            id: "max_measurement".to_string(),
            channel: "input_v".to_string(),
            method: "maximum_sample".to_string(),
            measured_value: 5.0,
            unit: "V".to_string(),
            sample_index: 1,
            timestamp: 0.001,
            method_context: Default::default(),
        }
    }

    #[test]
    fn renders_text_report() {
        let report = AnalysisReport {
            input_name: "fixture.csv".to_string(),
            waveform_metadata: metadata(),
            evidence_context: ReportEvidenceContext::default(),
            measurements: vec![measurement()],
            results: vec![AnalysisResult {
                criterion_id: "max".to_string(),
                outcome: Outcome::Pass,
                failed_criterion: None,
                measurement_id: "max_measurement".to_string(),
                channel: "input_v".to_string(),
                measured_value: 5.0,
                required_value: 5.5,
                tolerance_used: 0.0,
                unit: "V".to_string(),
                sample_index: 1,
                timestamp: 0.001,
                reason: "ok".to_string(),
            }],
        };

        let rendered = report.render_text();

        assert!(rendered.contains("Waveform Analysis Report"));
        assert!(rendered.contains("Samples: 2 Channels: 1 Lineage: Raw"));
        assert!(rendered.contains("Nominal Sample Rate: 1000.000000 Hz"));
        assert!(rendered.contains("Validation Profile: engineering_validation"));
        assert!(rendered.contains("Tolerance Policy: voltage=0.000000 V time=0.000000000 s"));
        assert!(rendered.contains("Overall: Pass"));
        assert!(rendered.contains("Measurements:"));
        assert!(rendered.contains("max_measurement"));
        assert!(rendered.contains("max"));
    }

    #[test]
    fn renders_json_report() {
        let report = AnalysisReport {
            input_name: "fixture.csv".to_string(),
            waveform_metadata: metadata(),
            evidence_context: ReportEvidenceContext::default(),
            measurements: vec![measurement()],
            results: vec![AnalysisResult {
                criterion_id: "max".to_string(),
                outcome: Outcome::Pass,
                failed_criterion: None,
                measurement_id: "max_measurement".to_string(),
                channel: "input_v".to_string(),
                measured_value: 5.0,
                required_value: 5.5,
                tolerance_used: 0.0,
                unit: "V".to_string(),
                sample_index: 1,
                timestamp: 0.001,
                reason: "ok".to_string(),
            }],
        };

        let rendered = report.render_json_pretty().expect("json should render");

        assert!(rendered.contains("\"waveform_metadata\""));
        assert!(rendered.contains("\"evidence_context\""));
        assert!(rendered.contains("\"tolerance_policy\""));
        assert!(rendered.contains("\"sample_count\": 2"));
        assert!(rendered.contains("\"overall_outcome\": \"pass\""));
        assert!(rendered.contains("\"measurements\""));
        assert!(rendered.contains("\"measurement_id\": \"max_measurement\""));
        assert!(rendered.contains("\"criterion_id\": \"max\""));
    }
}
