use crate::error::{Result, WaveformError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct Unit {
    pub name: String,
}

impl Unit {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn seconds() -> Self {
        Self::new("s")
    }

    pub fn volts() -> Self {
        Self::new("V")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Channel {
    pub name: String,
    pub unit: Unit,
    pub samples: Vec<f64>,
}

impl Channel {
    pub fn new(name: impl Into<String>, unit: Unit, samples: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            unit,
            samples,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ChannelMetadata {
    pub name: String,
    pub unit: Unit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WaveformLineage {
    Raw,
    Derived,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SampleIntervalSummary {
    pub min: f64,
    pub max: f64,
    pub nominal: f64,
    pub unit: Unit,
    pub uniform: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TolerancePolicy {
    pub voltage_v: f64,
    pub time_s: f64,
}

impl TolerancePolicy {
    pub fn validate(&self) -> Result<()> {
        validate_tolerance("tolerances.voltage_v", self.voltage_v)?;
        validate_tolerance("tolerances.time_s", self.time_s)
    }
}

impl Default for TolerancePolicy {
    fn default() -> Self {
        Self {
            voltage_v: 0.0,
            time_s: 0.0,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MetadataContext {
    pub test_run_id: Option<String>,
    pub acquisition_notes: Option<String>,
    pub environment: Option<String>,
    pub operator: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WaveformMetadata {
    pub source_name: Option<String>,
    pub test_run_id: Option<String>,
    pub acquisition_notes: Option<String>,
    pub environment: Option<String>,
    pub operator: Option<String>,
    pub time_unit: Unit,
    pub sample_count: usize,
    pub channel_count: usize,
    pub channels: Vec<ChannelMetadata>,
    pub sample_interval: Option<SampleIntervalSummary>,
    pub nominal_sample_rate_hz: Option<f64>,
    pub lineage: WaveformLineage,
    pub transform_history: Vec<String>,
    pub tolerance_policy: Option<TolerancePolicy>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Waveform {
    pub time_unit: Unit,
    pub time: Vec<f64>,
    pub channels: Vec<Channel>,
    pub metadata: WaveformMetadata,
}

impl Waveform {
    pub fn new(time: Vec<f64>, channels: Vec<Channel>) -> Result<Self> {
        Self::new_with_time_unit(time, Unit::seconds(), channels)
    }

    pub fn new_with_time_unit(
        time: Vec<f64>,
        time_unit: Unit,
        channels: Vec<Channel>,
    ) -> Result<Self> {
        if time.is_empty() {
            return Err(WaveformError::EmptyInput);
        }
        for channel in &channels {
            if channel.samples.len() != time.len() {
                return Err(WaveformError::MismatchedSampleCount {
                    expected: time.len(),
                    actual: channel.samples.len(),
                });
            }
        }
        let metadata = WaveformMetadata::new(None, time_unit.clone(), &time, &channels);
        Ok(Self {
            time_unit,
            time,
            channels,
            metadata,
        })
    }

    pub fn sample_count(&self) -> usize {
        self.time.len()
    }

    pub fn channel(&self, name: &str) -> Option<&Channel> {
        self.channels.iter().find(|channel| channel.name == name)
    }

    pub fn with_source_name(mut self, source_name: impl Into<String>) -> Self {
        self.metadata.source_name = Some(source_name.into());
        self
    }

    pub fn with_metadata_context(mut self, context: &MetadataContext) -> Self {
        self.metadata.test_run_id = context.test_run_id.clone();
        self.metadata.acquisition_notes = context.acquisition_notes.clone();
        self.metadata.environment = context.environment.clone();
        self.metadata.operator = context.operator.clone();
        self
    }

    pub fn with_tolerance_policy(mut self, policy: TolerancePolicy) -> Self {
        self.metadata.tolerance_policy = Some(policy);
        self
    }

    pub fn as_derived_from(mut self, source: &Waveform, transform: impl Into<String>) -> Self {
        let mut history = source.metadata.transform_history.clone();
        history.push(transform.into());
        self.metadata.source_name = source.metadata.source_name.clone();
        self.metadata.test_run_id = source.metadata.test_run_id.clone();
        self.metadata.acquisition_notes = source.metadata.acquisition_notes.clone();
        self.metadata.environment = source.metadata.environment.clone();
        self.metadata.operator = source.metadata.operator.clone();
        self.metadata.lineage = WaveformLineage::Derived;
        self.metadata.transform_history = history;
        self.metadata.tolerance_policy = source.metadata.tolerance_policy;
        self
    }
}

impl WaveformMetadata {
    fn new(
        source_name: Option<String>,
        time_unit: Unit,
        time: &[f64],
        channels: &[Channel],
    ) -> Self {
        let sample_interval = SampleIntervalSummary::from_time(time, time_unit.clone());
        let nominal_sample_rate_hz = sample_interval
            .as_ref()
            .and_then(|summary| sample_rate_hz(summary, &time_unit));
        Self {
            source_name,
            test_run_id: None,
            acquisition_notes: None,
            environment: None,
            operator: None,
            time_unit,
            sample_count: time.len(),
            channel_count: channels.len(),
            channels: channels
                .iter()
                .map(|channel| ChannelMetadata {
                    name: channel.name.clone(),
                    unit: channel.unit.clone(),
                })
                .collect(),
            sample_interval,
            nominal_sample_rate_hz,
            lineage: WaveformLineage::Raw,
            transform_history: Vec::new(),
            tolerance_policy: None,
        }
    }
}

impl SampleIntervalSummary {
    fn from_time(time: &[f64], unit: Unit) -> Option<Self> {
        let mut intervals = time.windows(2).map(|pair| pair[1] - pair[0]);
        let first = intervals.next()?;
        let mut min = first;
        let mut max = first;
        let mut sum = first;
        let mut count = 1_usize;

        for interval in intervals {
            min = min.min(interval);
            max = max.max(interval);
            sum += interval;
            count += 1;
        }

        let nominal = sum / count as f64;
        Some(Self {
            min: round_metadata_value(min),
            max: round_metadata_value(max),
            nominal: round_metadata_value(nominal),
            unit,
            uniform: (max - min).abs() <= 1.0e-12,
        })
    }
}

fn round_metadata_value(value: f64) -> f64 {
    if value.is_finite() {
        (value * 1_000_000_000_000.0).round() / 1_000_000_000_000.0
    } else {
        value
    }
}

fn sample_rate_hz(summary: &SampleIntervalSummary, time_unit: &Unit) -> Option<f64> {
    if time_unit.name == "s" && summary.nominal > 0.0 {
        Some(1.0 / summary.nominal)
    } else {
        None
    }
}

fn validate_tolerance(name: &str, value: f64) -> Result<()> {
    if !value.is_finite() {
        return Err(WaveformError::InvalidParameter {
            name: name.to_string(),
            reason: "must be finite".to_string(),
        });
    }
    if value < 0.0 {
        return Err(WaveformError::InvalidParameter {
            name: name.to_string(),
            reason: "must be greater than or equal to zero".to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_aligned_channel_lengths() {
        let waveform = Waveform::new(
            vec![0.0, 0.1],
            vec![Channel::new("input_v", Unit::volts(), vec![0.0, 5.0])],
        )
        .expect("waveform should be valid");

        assert_eq!(waveform.sample_count(), 2);
        assert_eq!(waveform.metadata.sample_count, 2);
        assert_eq!(waveform.metadata.channel_count, 1);
        assert_eq!(waveform.metadata.channels[0].name, "input_v");
        assert_eq!(waveform.metadata.channels[0].unit, Unit::volts());
        assert_eq!(waveform.metadata.lineage, WaveformLineage::Raw);
        assert_eq!(waveform.metadata.transform_history, Vec::<String>::new());
        assert_eq!(waveform.metadata.tolerance_policy, None);
    }

    #[test]
    fn computes_sample_interval_and_rate_metadata() {
        let waveform = Waveform::new(
            vec![0.0, 0.001, 0.002, 0.003],
            vec![Channel::new(
                "input_v",
                Unit::volts(),
                vec![0.0, 1.0, 2.0, 3.0],
            )],
        )
        .expect("waveform should be valid");

        let interval = waveform
            .metadata
            .sample_interval
            .expect("sample interval should be summarized");
        assert_eq!(interval.min, 0.001);
        assert_eq!(interval.max, 0.001);
        assert_eq!(interval.nominal, 0.001);
        assert!(interval.uniform);
        assert_eq!(waveform.metadata.nominal_sample_rate_hz, Some(1000.0));
    }

    #[test]
    fn tracks_source_name_and_derived_transform_history() {
        let raw = Waveform::new(
            vec![0.0, 0.1],
            vec![Channel::new("input_v", Unit::volts(), vec![0.0, 5.0])],
        )
        .expect("waveform should be valid")
        .with_source_name("fixture.csv");
        let derived = Waveform::new(
            raw.time.clone(),
            vec![Channel::new("input_v", Unit::volts(), vec![1.0, 4.0])],
        )
        .expect("derived waveform should be valid")
        .as_derived_from(&raw, "moving_average(window_samples=2)");

        assert_eq!(derived.metadata.source_name.as_deref(), Some("fixture.csv"));
        assert_eq!(derived.metadata.lineage, WaveformLineage::Derived);
        assert_eq!(
            derived.metadata.transform_history,
            vec!["moving_average(window_samples=2)"]
        );
    }

    #[test]
    fn stores_optional_validation_context_and_tolerances() {
        let context = MetadataContext {
            test_run_id: Some("run-42".to_string()),
            acquisition_notes: Some("known-answer synthetic case".to_string()),
            environment: Some("desktop validation".to_string()),
            operator: Some("automation".to_string()),
        };
        let policy = TolerancePolicy {
            voltage_v: 0.01,
            time_s: 0.0005,
        };
        let waveform = Waveform::new(
            vec![0.0, 0.1],
            vec![Channel::new("input_v", Unit::volts(), vec![0.0, 5.0])],
        )
        .expect("waveform should be valid")
        .with_metadata_context(&context)
        .with_tolerance_policy(policy);

        assert_eq!(waveform.metadata.test_run_id.as_deref(), Some("run-42"));
        assert_eq!(waveform.metadata.tolerance_policy, Some(policy));
    }

    #[test]
    fn rejects_invalid_tolerance_values() {
        let invalid = TolerancePolicy {
            voltage_v: -0.1,
            time_s: 0.0,
        };

        assert!(matches!(
            invalid.validate(),
            Err(WaveformError::InvalidParameter { .. })
        ));
    }

    #[test]
    fn rejects_mismatched_channel_lengths() {
        let result = Waveform::new(
            vec![0.0, 0.1],
            vec![Channel::new("input_v", Unit::volts(), vec![0.0])],
        );

        assert_eq!(
            result,
            Err(WaveformError::MismatchedSampleCount {
                expected: 2,
                actual: 1
            })
        );
    }
}
