use crate::error::{Result, WaveformError};
use serde::Serialize;

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

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WaveformMetadata {
    pub source_name: Option<String>,
    pub time_unit: Unit,
    pub sample_count: usize,
    pub channel_count: usize,
    pub channels: Vec<ChannelMetadata>,
    pub sample_interval: Option<SampleIntervalSummary>,
    pub nominal_sample_rate_hz: Option<f64>,
    pub lineage: WaveformLineage,
    pub transform_history: Vec<String>,
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

    pub fn as_derived_from(mut self, source: &Waveform, transform: impl Into<String>) -> Self {
        let mut history = source.metadata.transform_history.clone();
        history.push(transform.into());
        self.metadata.source_name = source.metadata.source_name.clone();
        self.metadata.lineage = WaveformLineage::Derived;
        self.metadata.transform_history = history;
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
