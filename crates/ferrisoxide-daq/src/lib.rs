//! DAQ input abstraction for deterministic controller-in-the-loop workflows.
//!
//! This crate intentionally starts with fixture and test-double sample sources.
//! It does not depend on vendor SDKs, drivers, live DAQ hardware, global setup,
//! HALs, RTOS bindings, or OS-specific device APIs.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DaqChannel {
    pub id: String,
    pub source: String,
    pub unit: String,
}

impl DaqChannel {
    pub fn new(id: impl Into<String>, source: impl Into<String>, unit: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            source: source.into(),
            unit: unit.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DaqSampleFrame {
    pub time_s: f64,
    pub values: BTreeMap<String, DaqSampleValue>,
}

impl DaqSampleFrame {
    pub fn new(time_s: f64) -> Self {
        Self {
            time_s,
            values: BTreeMap::new(),
        }
    }

    pub fn with_value(mut self, channel: impl Into<String>, value: DaqSampleValue) -> Self {
        self.values.insert(channel.into(), value);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DaqSampleValue {
    Analog { value: f64 },
    Digital { high: bool },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DaqSourceDescriptor {
    pub name: String,
    pub kind: DaqSourceKind,
    pub channels: Vec<DaqChannel>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DaqSourceKind {
    Fixture,
    TestDouble,
    FutureVendorSdk,
}

pub trait DaqSampleSource {
    fn descriptor(&self) -> &DaqSourceDescriptor;
    fn next_frame(&mut self) -> Result<Option<DaqSampleFrame>, DaqError>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FixtureDaqSource {
    descriptor: DaqSourceDescriptor,
    frames: Vec<DaqSampleFrame>,
    next_index: usize,
}

impl FixtureDaqSource {
    pub fn new(
        descriptor: DaqSourceDescriptor,
        frames: Vec<DaqSampleFrame>,
    ) -> Result<Self, DaqError> {
        validate_descriptor(&descriptor)?;
        validate_frames(&descriptor, &frames)?;
        Ok(Self {
            descriptor,
            frames,
            next_index: 0,
        })
    }

    pub fn reset(&mut self) {
        self.next_index = 0;
    }
}

impl DaqSampleSource for FixtureDaqSource {
    fn descriptor(&self) -> &DaqSourceDescriptor {
        &self.descriptor
    }

    fn next_frame(&mut self) -> Result<Option<DaqSampleFrame>, DaqError> {
        let Some(frame) = self.frames.get(self.next_index) else {
            return Ok(None);
        };
        self.next_index += 1;
        Ok(Some(frame.clone()))
    }
}

pub fn collect_frames(
    source: &mut impl DaqSampleSource,
    max_frames: usize,
) -> Result<Vec<DaqSampleFrame>, DaqError> {
    let mut frames = Vec::new();
    while frames.len() < max_frames {
        let Some(frame) = source.next_frame()? else {
            break;
        };
        frames.push(frame);
    }
    Ok(frames)
}

#[derive(Debug, Clone, PartialEq)]
pub enum DaqError {
    EmptyChannelList,
    EmptyChannelId {
        index: usize,
    },
    EmptySource {
        channel: String,
    },
    EmptyUnit {
        channel: String,
    },
    DuplicateChannel {
        channel: String,
    },
    EmptyFrames,
    NonMonotonicTime {
        previous_time_s: f64,
        current_time_s: f64,
    },
    MissingChannel {
        channel: String,
        sample_index: usize,
    },
    NonFiniteAnalogValue {
        channel: String,
        sample_index: usize,
    },
    NonFiniteTimestamp {
        sample_index: usize,
    },
}

impl fmt::Display for DaqError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyChannelList => write!(formatter, "DAQ source must define at least one channel"),
            Self::EmptyChannelId { index } => {
                write!(formatter, "DAQ channel at index {index} has an empty id")
            }
            Self::EmptySource { channel } => {
                write!(formatter, "DAQ channel `{channel}` has an empty source")
            }
            Self::EmptyUnit { channel } => {
                write!(formatter, "DAQ channel `{channel}` has an empty unit")
            }
            Self::DuplicateChannel { channel } => {
                write!(formatter, "duplicate DAQ channel `{channel}`")
            }
            Self::EmptyFrames => write!(formatter, "DAQ fixture source requires at least one frame"),
            Self::NonMonotonicTime {
                previous_time_s,
                current_time_s,
            } => write!(
                formatter,
                "DAQ frame timestamps must increase monotonically, previous={previous_time_s}, current={current_time_s}"
            ),
            Self::MissingChannel {
                channel,
                sample_index,
            } => write!(
                formatter,
                "DAQ frame {sample_index} is missing channel `{channel}`"
            ),
            Self::NonFiniteAnalogValue {
                channel,
                sample_index,
            } => write!(
                formatter,
                "DAQ frame {sample_index} channel `{channel}` has a non-finite analog value"
            ),
            Self::NonFiniteTimestamp { sample_index } => {
                write!(formatter, "DAQ frame {sample_index} has a non-finite timestamp")
            }
        }
    }
}

impl std::error::Error for DaqError {}

fn validate_descriptor(descriptor: &DaqSourceDescriptor) -> Result<(), DaqError> {
    if descriptor.channels.is_empty() {
        return Err(DaqError::EmptyChannelList);
    }
    let mut ids = BTreeSet::new();
    for (index, channel) in descriptor.channels.iter().enumerate() {
        if channel.id.is_empty() {
            return Err(DaqError::EmptyChannelId { index });
        }
        if channel.source.is_empty() {
            return Err(DaqError::EmptySource {
                channel: channel.id.clone(),
            });
        }
        if channel.unit.is_empty() {
            return Err(DaqError::EmptyUnit {
                channel: channel.id.clone(),
            });
        }
        if !ids.insert(channel.id.clone()) {
            return Err(DaqError::DuplicateChannel {
                channel: channel.id.clone(),
            });
        }
    }
    Ok(())
}

fn validate_frames(
    descriptor: &DaqSourceDescriptor,
    frames: &[DaqSampleFrame],
) -> Result<(), DaqError> {
    if frames.is_empty() {
        return Err(DaqError::EmptyFrames);
    }
    for (sample_index, frame) in frames.iter().enumerate() {
        if !frame.time_s.is_finite() {
            return Err(DaqError::NonFiniteTimestamp { sample_index });
        }
        if sample_index > 0 {
            let previous_time_s = frames[sample_index - 1].time_s;
            if frame.time_s <= previous_time_s {
                return Err(DaqError::NonMonotonicTime {
                    previous_time_s,
                    current_time_s: frame.time_s,
                });
            }
        }
        for channel in &descriptor.channels {
            let Some(value) = frame.values.get(&channel.id) else {
                return Err(DaqError::MissingChannel {
                    channel: channel.id.clone(),
                    sample_index,
                });
            };
            if let DaqSampleValue::Analog { value } = value {
                if !value.is_finite() {
                    return Err(DaqError::NonFiniteAnalogValue {
                        channel: channel.id.clone(),
                        sample_index,
                    });
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_source_yields_deterministic_sample_frames() {
        let descriptor = descriptor();
        let frames = frames();
        let mut source =
            FixtureDaqSource::new(descriptor.clone(), frames.clone()).expect("fixture source");

        assert_eq!(source.descriptor(), &descriptor);
        assert_eq!(
            source.next_frame().expect("first frame"),
            Some(frames[0].clone())
        );
        assert_eq!(
            source.next_frame().expect("second frame"),
            Some(frames[1].clone())
        );
        assert_eq!(source.next_frame().expect("end"), None);

        source.reset();
        assert_eq!(
            collect_frames(&mut source, 1).expect("collect one"),
            vec![frames[0].clone()]
        );
    }

    #[test]
    fn rejects_non_monotonic_or_incomplete_fixture_input() {
        let descriptor = descriptor();
        let bad_time = vec![frame(0.0, 0.0, false), frame(0.0, 1.0, true)];
        let error = FixtureDaqSource::new(descriptor.clone(), bad_time).expect_err("bad time");
        assert!(matches!(error, DaqError::NonMonotonicTime { .. }));

        let missing_feedback =
            vec![DaqSampleFrame::new(0.0)
                .with_value("command", DaqSampleValue::Analog { value: 0.0 })];
        let error =
            FixtureDaqSource::new(descriptor, missing_feedback).expect_err("missing channel");
        assert!(matches!(error, DaqError::MissingChannel { .. }));
    }

    #[test]
    fn rejects_duplicate_channels_and_non_finite_values() {
        let duplicate_descriptor = DaqSourceDescriptor {
            name: "duplicate".to_string(),
            kind: DaqSourceKind::TestDouble,
            channels: vec![
                DaqChannel::new("command", "fixture.command_v", "V"),
                DaqChannel::new("command", "fixture.command_copy_v", "V"),
            ],
        };
        let error =
            FixtureDaqSource::new(duplicate_descriptor, frames()).expect_err("duplicate channel");
        assert!(matches!(error, DaqError::DuplicateChannel { .. }));

        let descriptor = descriptor();
        let non_finite = vec![frame(0.0, f64::NAN, false)];
        let error = FixtureDaqSource::new(descriptor, non_finite).expect_err("non-finite");
        assert!(matches!(error, DaqError::NonFiniteAnalogValue { .. }));
    }

    fn descriptor() -> DaqSourceDescriptor {
        DaqSourceDescriptor {
            name: "heated-actuator-fixture".to_string(),
            kind: DaqSourceKind::Fixture,
            channels: vec![
                DaqChannel::new("command", "fixture.command_v", "V"),
                DaqChannel::new("feedback", "fixture.actuator_feedback_high", "bool"),
            ],
        }
    }

    fn frames() -> Vec<DaqSampleFrame> {
        vec![frame(0.0, 0.0, false), frame(0.001, 5.0, true)]
    }

    fn frame(time_s: f64, command_v: f64, feedback_high: bool) -> DaqSampleFrame {
        DaqSampleFrame::new(time_s)
            .with_value("command", DaqSampleValue::Analog { value: command_v })
            .with_value(
                "feedback",
                DaqSampleValue::Digital {
                    high: feedback_high,
                },
            )
    }
}
