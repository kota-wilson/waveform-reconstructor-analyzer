#![no_std]

//! `no_std` signal-analysis primitives for embedded and RTOS adapters.
//!
//! This crate intentionally avoids file I/O, parsing, reporting, dynamic
//! allocation, and RTOS bindings. It provides the smallest reusable analysis
//! core that desktop and embedded wrappers can share.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sample {
    pub timestamp_s: f32,
    pub value: f32,
}

impl Sample {
    pub const fn new(timestamp_s: f32, value: f32) -> Self {
        Self { timestamp_s, value }
    }
}

impl Default for Sample {
    fn default() -> Self {
        Self {
            timestamp_s: 0.0,
            value: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalError {
    BufferFull,
    EmptyInput,
    InvalidThresholds,
    InvalidDuration,
    NonMonotonicTimestamp,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FixedSampleBuffer<const N: usize> {
    samples: [Sample; N],
    len: usize,
}

impl<const N: usize> FixedSampleBuffer<N> {
    pub fn new() -> Self {
        Self {
            samples: [Sample::default(); N],
            len: 0,
        }
    }

    pub fn push(&mut self, sample: Sample) -> Result<(), SignalError> {
        if self.len == N {
            return Err(SignalError::BufferFull);
        }
        if self
            .last()
            .is_some_and(|last| sample.timestamp_s <= last.timestamp_s)
        {
            return Err(SignalError::NonMonotonicTimestamp);
        }

        self.samples[self.len] = sample;
        self.len += 1;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn capacity(&self) -> usize {
        N
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub const fn is_full(&self) -> bool {
        self.len == N
    }

    pub fn as_slice(&self) -> &[Sample] {
        &self.samples[..self.len]
    }

    pub fn last(&self) -> Option<Sample> {
        self.len.checked_sub(1).map(|index| self.samples[index])
    }
}

impl<const N: usize> Default for FixedSampleBuffer<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThresholdLimits {
    pub min: Option<f32>,
    pub max: Option<f32>,
}

impl ThresholdLimits {
    pub const fn new(min: Option<f32>, max: Option<f32>) -> Self {
        Self { min, max }
    }

    pub fn evaluate(&self, samples: &[Sample]) -> Result<ThresholdEvaluation, SignalError> {
        let mut tracker = ThresholdTracker::new(*self)?;
        for sample in samples.iter().copied() {
            tracker.ingest(sample)?;
        }
        tracker.finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdCheck {
    Minimum,
    Maximum,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThresholdEvidence {
    pub check: ThresholdCheck,
    pub sample_index: usize,
    pub timestamp_s: f32,
    pub measured_value: f32,
    pub required_value: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThresholdEvaluation {
    pub passed: bool,
    pub failed_check: Option<ThresholdCheck>,
    pub min_value: f32,
    pub max_value: f32,
    pub sample_count: usize,
    pub evidence: ThresholdEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ThresholdPoint {
    sample_index: usize,
    timestamp_s: f32,
    value: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThresholdTracker {
    limits: ThresholdLimits,
    min: Option<ThresholdPoint>,
    max: Option<ThresholdPoint>,
    previous_timestamp_s: Option<f32>,
    sample_count: usize,
}

impl ThresholdTracker {
    pub fn new(limits: ThresholdLimits) -> Result<Self, SignalError> {
        if !valid_limit_pair(limits) {
            return Err(SignalError::InvalidThresholds);
        }

        Ok(Self {
            limits,
            min: None,
            max: None,
            previous_timestamp_s: None,
            sample_count: 0,
        })
    }

    pub fn ingest(&mut self, sample: Sample) -> Result<(), SignalError> {
        if self
            .previous_timestamp_s
            .is_some_and(|previous| sample.timestamp_s <= previous)
        {
            return Err(SignalError::NonMonotonicTimestamp);
        }

        let point = ThresholdPoint {
            sample_index: self.sample_count,
            timestamp_s: sample.timestamp_s,
            value: sample.value,
        };

        let new_minimum = match self.min {
            Some(min) => point.value < min.value,
            None => true,
        };
        if new_minimum {
            self.min = Some(point);
        }
        let new_maximum = match self.max {
            Some(max) => point.value > max.value,
            None => true,
        };
        if new_maximum {
            self.max = Some(point);
        }

        self.previous_timestamp_s = Some(sample.timestamp_s);
        self.sample_count += 1;
        Ok(())
    }

    pub fn finish(self) -> Result<ThresholdEvaluation, SignalError> {
        let min = self.min.ok_or(SignalError::EmptyInput)?;
        let max = self.max.ok_or(SignalError::EmptyInput)?;

        let min_failed = self.limits.min.is_some_and(|limit| min.value < limit);
        let max_failed = self.limits.max.is_some_and(|limit| max.value > limit);
        let (failed_check, evidence) = if max_failed {
            (
                Some(ThresholdCheck::Maximum),
                ThresholdEvidence {
                    check: ThresholdCheck::Maximum,
                    sample_index: max.sample_index,
                    timestamp_s: max.timestamp_s,
                    measured_value: max.value,
                    required_value: self.limits.max.unwrap_or(max.value),
                },
            )
        } else if min_failed {
            (
                Some(ThresholdCheck::Minimum),
                ThresholdEvidence {
                    check: ThresholdCheck::Minimum,
                    sample_index: min.sample_index,
                    timestamp_s: min.timestamp_s,
                    measured_value: min.value,
                    required_value: self.limits.min.unwrap_or(min.value),
                },
            )
        } else if let Some(limit) = self.limits.max {
            (
                None,
                ThresholdEvidence {
                    check: ThresholdCheck::Maximum,
                    sample_index: max.sample_index,
                    timestamp_s: max.timestamp_s,
                    measured_value: max.value,
                    required_value: limit,
                },
            )
        } else {
            (
                None,
                ThresholdEvidence {
                    check: ThresholdCheck::Minimum,
                    sample_index: min.sample_index,
                    timestamp_s: min.timestamp_s,
                    measured_value: min.value,
                    required_value: self.limits.min.unwrap_or(min.value),
                },
            )
        };

        Ok(ThresholdEvaluation {
            passed: failed_check.is_none(),
            failed_check,
            min_value: min.value,
            max_value: max.value,
            sample_count: self.sample_count,
            evidence,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalState {
    High,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransientEventKind {
    TransientEvent,
    SpuriousTransition,
    ContactBounce,
    Dropout,
    NoiseInducedTransition,
    ThresholdCrossingEvent,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransientEventConfig {
    pub kind: TransientEventKind,
    pub expected_state: SignalState,
    pub threshold: f32,
    pub max_duration_s: f32,
}

impl TransientEventConfig {
    pub const fn new(
        kind: TransientEventKind,
        expected_state: SignalState,
        threshold: f32,
        max_duration_s: f32,
    ) -> Self {
        Self {
            kind,
            expected_state,
            threshold,
            max_duration_s,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransientEventEvaluation {
    pub passed: bool,
    pub kind: TransientEventKind,
    pub measured_duration_s: f32,
    pub required_duration_s: f32,
    pub sample_index: usize,
    pub timestamp_s: f32,
    pub observed_state: SignalState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ActiveEvent {
    observed_state: SignalState,
    start_index: usize,
    start_time_s: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransientEventDetector {
    config: TransientEventConfig,
    active: Option<ActiveEvent>,
    longest: Option<TransientEventEvaluation>,
    previous_timestamp_s: Option<f32>,
    sample_count: usize,
}

impl TransientEventDetector {
    pub fn new(config: TransientEventConfig) -> Result<Self, SignalError> {
        if !config.threshold.is_finite() {
            return Err(SignalError::InvalidThresholds);
        }
        if !config.max_duration_s.is_finite() || config.max_duration_s < 0.0 {
            return Err(SignalError::InvalidDuration);
        }

        Ok(Self {
            config,
            active: None,
            longest: None,
            previous_timestamp_s: None,
            sample_count: 0,
        })
    }

    pub fn ingest(&mut self, sample: Sample) -> Result<(), SignalError> {
        if self
            .previous_timestamp_s
            .is_some_and(|previous| sample.timestamp_s <= previous)
        {
            return Err(SignalError::NonMonotonicTimestamp);
        }

        let observed_state = state_for(sample.value, self.config.threshold);
        if observed_state == self.config.expected_state {
            self.close_active(sample.timestamp_s);
        } else if self.active.is_none() {
            self.active = Some(ActiveEvent {
                observed_state,
                start_index: self.sample_count,
                start_time_s: sample.timestamp_s,
            });
        }

        self.previous_timestamp_s = Some(sample.timestamp_s);
        self.sample_count += 1;
        Ok(())
    }

    pub fn finish(mut self) -> TransientEventEvaluation {
        if let Some(end_time_s) = self.previous_timestamp_s {
            self.close_active(end_time_s);
        }

        self.longest.unwrap_or(TransientEventEvaluation {
            passed: true,
            kind: self.config.kind,
            measured_duration_s: 0.0,
            required_duration_s: self.config.max_duration_s,
            sample_index: 0,
            timestamp_s: 0.0,
            observed_state: self.config.expected_state,
        })
    }

    fn close_active(&mut self, end_time_s: f32) {
        let Some(active) = self.active.take() else {
            return;
        };

        let measured_duration_s = end_time_s - active.start_time_s;
        let candidate = TransientEventEvaluation {
            passed: measured_duration_s <= self.config.max_duration_s,
            kind: self.config.kind,
            measured_duration_s,
            required_duration_s: self.config.max_duration_s,
            sample_index: active.start_index,
            timestamp_s: active.start_time_s,
            observed_state: active.observed_state,
        };

        let should_replace = match self.longest {
            Some(longest) => candidate.measured_duration_s > longest.measured_duration_s,
            None => true,
        };

        if should_replace {
            self.longest = Some(candidate);
        }
    }
}

pub fn evaluate_transient_event(
    samples: &[Sample],
    config: TransientEventConfig,
) -> Result<TransientEventEvaluation, SignalError> {
    if samples.is_empty() {
        return Err(SignalError::EmptyInput);
    }

    let mut detector = TransientEventDetector::new(config)?;
    for sample in samples.iter().copied() {
        detector.ingest(sample)?;
    }
    Ok(detector.finish())
}

fn state_for(value: f32, threshold: f32) -> SignalState {
    if value >= threshold {
        SignalState::High
    } else {
        SignalState::Low
    }
}

fn valid_limit_pair(limits: ThresholdLimits) -> bool {
    let min_valid = match limits.min {
        Some(min) => min.is_finite(),
        None => true,
    };
    let max_valid = match limits.max {
        Some(max) => max.is_finite(),
        None => true,
    };
    let ordered = match (limits.min, limits.max) {
        (Some(min), Some(max)) => min <= max,
        _ => true,
    };

    min_valid && max_valid && ordered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_sample_buffer_accepts_streamed_samples_without_heap() {
        let mut buffer = FixedSampleBuffer::<3>::new();

        buffer.push(Sample::new(0.0, 0.0)).expect("sample 0");
        buffer.push(Sample::new(0.1, 1.0)).expect("sample 1");
        buffer.push(Sample::new(0.2, 2.0)).expect("sample 2");

        assert_eq!(buffer.len(), 3);
        assert!(buffer.is_full());
        assert_eq!(buffer.as_slice()[2].value, 2.0);
        assert_eq!(
            buffer.push(Sample::new(0.3, 3.0)),
            Err(SignalError::BufferFull)
        );
    }

    #[test]
    fn fixed_sample_buffer_rejects_non_monotonic_timestamps() {
        let mut buffer = FixedSampleBuffer::<3>::new();

        buffer.push(Sample::new(0.1, 0.0)).expect("sample 0");

        assert_eq!(
            buffer.push(Sample::new(0.1, 1.0)),
            Err(SignalError::NonMonotonicTimestamp)
        );
    }

    #[test]
    fn threshold_limits_report_pass_and_fail_evidence() {
        let samples = [
            Sample::new(0.0, 0.0),
            Sample::new(0.1, 3.3),
            Sample::new(0.2, 5.2),
        ];
        let limits = ThresholdLimits::new(Some(0.0), Some(5.0));

        let evaluation = limits
            .evaluate(&samples)
            .expect("thresholds should evaluate");

        assert!(!evaluation.passed);
        assert_eq!(evaluation.failed_check, Some(ThresholdCheck::Maximum));
        assert_eq!(evaluation.sample_count, 3);
        assert_eq!(evaluation.max_value, 5.2);
        assert_eq!(evaluation.evidence.check, ThresholdCheck::Maximum);
        assert_eq!(evaluation.evidence.sample_index, 2);
        assert_eq!(evaluation.evidence.timestamp_s, 0.2);
        assert_eq!(evaluation.evidence.required_value, 5.0);
    }

    #[test]
    fn threshold_tracker_evaluates_streamed_samples() {
        let mut tracker =
            ThresholdTracker::new(ThresholdLimits::new(Some(-1.0), Some(5.0))).expect("tracker");

        tracker.ingest(Sample::new(0.0, 0.0)).expect("sample 0");
        tracker.ingest(Sample::new(0.1, 3.3)).expect("sample 1");
        tracker.ingest(Sample::new(0.2, 4.9)).expect("sample 2");

        let evaluation = tracker.finish().expect("threshold evaluation");

        assert!(evaluation.passed);
        assert_eq!(evaluation.failed_check, None);
        assert_eq!(evaluation.sample_count, 3);
        assert_eq!(evaluation.min_value, 0.0);
        assert_eq!(evaluation.max_value, 4.9);
    }

    #[test]
    fn threshold_tracker_rejects_invalid_limits() {
        assert_eq!(
            ThresholdTracker::new(ThresholdLimits::new(Some(5.0), Some(1.0))),
            Err(SignalError::InvalidThresholds)
        );
    }

    #[test]
    fn transient_event_detector_rejects_invalid_duration() {
        let config = TransientEventConfig::new(
            TransientEventKind::TransientEvent,
            SignalState::High,
            2.5,
            -0.1,
        );

        assert_eq!(
            TransientEventDetector::new(config),
            Err(SignalError::InvalidDuration)
        );
    }

    #[test]
    fn transient_event_detector_passes_when_dropout_is_within_limit() {
        let samples = [
            Sample::new(0.0, 5.0),
            Sample::new(0.1, 0.0),
            Sample::new(0.2, 5.0),
        ];
        let config =
            TransientEventConfig::new(TransientEventKind::Dropout, SignalState::High, 2.5, 0.1);

        let evaluation = evaluate_transient_event(&samples, config).expect("event should evaluate");

        assert!(evaluation.passed);
        assert_eq!(evaluation.measured_duration_s, 0.1);
        assert_eq!(evaluation.sample_index, 1);
    }

    #[test]
    fn transient_event_detector_fails_when_dropout_exceeds_limit() {
        let samples = [
            Sample::new(0.0, 5.0),
            Sample::new(0.1, 0.0),
            Sample::new(0.3, 5.0),
        ];
        let config =
            TransientEventConfig::new(TransientEventKind::Dropout, SignalState::High, 2.5, 0.1);

        let evaluation = evaluate_transient_event(&samples, config).expect("event should evaluate");

        assert!(!evaluation.passed);
        assert_eq!(evaluation.measured_duration_s, 0.20000002);
        assert_eq!(evaluation.required_duration_s, 0.1);
        assert_eq!(evaluation.sample_index, 1);
        assert_eq!(evaluation.timestamp_s, 0.1);
        assert_eq!(evaluation.observed_state, SignalState::Low);
    }

    #[test]
    fn streaming_detector_rejects_non_monotonic_timestamps() {
        let config = TransientEventConfig::new(
            TransientEventKind::SpuriousTransition,
            SignalState::Low,
            2.5,
            0.1,
        );
        let mut detector = TransientEventDetector::new(config).expect("detector");

        detector.ingest(Sample::new(0.1, 0.0)).expect("sample 0");

        assert_eq!(
            detector.ingest(Sample::new(0.1, 5.0)),
            Err(SignalError::NonMonotonicTimestamp)
        );
    }
}
