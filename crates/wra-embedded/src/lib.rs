#![no_std]

//! RTOS and ARM64 adapter boundaries for embedded waveform analysis.
//!
//! This crate wraps `wra-signal` primitives with small traits for sample
//! sources, event sinks, and runtime hooks. It intentionally avoids CSV
//! parsing, file I/O, allocation, plotting, hardware HALs, and RTOS-specific
//! APIs so adapters can be added without changing the signal core.

use core::convert::Infallible;

use wra_signal::{
    Sample, SignalError, ThresholdEvaluation, ThresholdLimits, ThresholdTracker,
    TransientEventConfig, TransientEventDetector, TransientEventEvaluation,
};

pub trait SampleSource {
    type Error;

    fn poll_sample(&mut self) -> Result<Option<Sample>, Self::Error>;
}

pub trait EventSink {
    type Error;

    fn record_threshold(&mut self, evaluation: &ThresholdEvaluation) -> Result<(), Self::Error>;

    fn record_transient_event(
        &mut self,
        evaluation: &TransientEventEvaluation,
    ) -> Result<(), Self::Error>;
}

pub trait RuntimeHooks {
    type Error;

    fn before_poll(&mut self) -> Result<(), Self::Error>;

    fn after_sample(&mut self, sample: Sample) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterError<SourceError, SinkError, RuntimeError> {
    Source(SourceError),
    Sink(SinkError),
    Runtime(RuntimeError),
    Signal(SignalError),
}

pub type StreamResult<T, S, K, R> = Result<
    T,
    AdapterError<<S as SampleSource>::Error, <K as EventSink>::Error, <R as RuntimeHooks>::Error>,
>;

pub struct SliceSampleSource<'a> {
    samples: &'a [Sample],
    index: usize,
}

impl<'a> SliceSampleSource<'a> {
    pub const fn new(samples: &'a [Sample]) -> Self {
        Self { samples, index: 0 }
    }
}

impl SampleSource for SliceSampleSource<'_> {
    type Error = Infallible;

    fn poll_sample(&mut self) -> Result<Option<Sample>, Self::Error> {
        let Some(sample) = self.samples.get(self.index).copied() else {
            return Ok(None);
        };

        self.index += 1;
        Ok(Some(sample))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NoopRuntime {
    pub polls: usize,
    pub samples: usize,
}

impl RuntimeHooks for NoopRuntime {
    type Error = Infallible;

    fn before_poll(&mut self) -> Result<(), Self::Error> {
        self.polls += 1;
        Ok(())
    }

    fn after_sample(&mut self, _sample: Sample) -> Result<(), Self::Error> {
        self.samples += 1;
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct LastResultSink {
    pub threshold: Option<ThresholdEvaluation>,
    pub transient_event: Option<TransientEventEvaluation>,
}

impl EventSink for LastResultSink {
    type Error = Infallible;

    fn record_threshold(&mut self, evaluation: &ThresholdEvaluation) -> Result<(), Self::Error> {
        self.threshold = Some(*evaluation);
        Ok(())
    }

    fn record_transient_event(
        &mut self,
        evaluation: &TransientEventEvaluation,
    ) -> Result<(), Self::Error> {
        self.transient_event = Some(*evaluation);
        Ok(())
    }
}

pub fn run_threshold_stream<S, K, R>(
    source: &mut S,
    sink: &mut K,
    runtime: &mut R,
    limits: ThresholdLimits,
) -> StreamResult<ThresholdEvaluation, S, K, R>
where
    S: SampleSource,
    K: EventSink,
    R: RuntimeHooks,
{
    let mut tracker = ThresholdTracker::new(limits).map_err(AdapterError::Signal)?;

    loop {
        runtime.before_poll().map_err(AdapterError::Runtime)?;
        let sample = source.poll_sample().map_err(AdapterError::Source)?;
        let Some(sample) = sample else {
            break;
        };

        tracker.ingest(sample).map_err(AdapterError::Signal)?;
        runtime
            .after_sample(sample)
            .map_err(AdapterError::Runtime)?;
    }

    let evaluation = tracker.finish().map_err(AdapterError::Signal)?;
    sink.record_threshold(&evaluation)
        .map_err(AdapterError::Sink)?;
    Ok(evaluation)
}

pub fn run_transient_event_stream<S, K, R>(
    source: &mut S,
    sink: &mut K,
    runtime: &mut R,
    config: TransientEventConfig,
) -> StreamResult<TransientEventEvaluation, S, K, R>
where
    S: SampleSource,
    K: EventSink,
    R: RuntimeHooks,
{
    let mut detector = TransientEventDetector::new(config).map_err(AdapterError::Signal)?;
    let mut saw_sample = false;

    loop {
        runtime.before_poll().map_err(AdapterError::Runtime)?;
        let sample = source.poll_sample().map_err(AdapterError::Source)?;
        let Some(sample) = sample else {
            break;
        };

        saw_sample = true;
        detector.ingest(sample).map_err(AdapterError::Signal)?;
        runtime
            .after_sample(sample)
            .map_err(AdapterError::Runtime)?;
    }

    if !saw_sample {
        return Err(AdapterError::Signal(SignalError::EmptyInput));
    }

    let evaluation = detector.finish();
    sink.record_transient_event(&evaluation)
        .map_err(AdapterError::Sink)?;
    Ok(evaluation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wra_signal::{SignalState, ThresholdCheck, TransientEventKind};

    #[test]
    fn threshold_stream_uses_source_runtime_sink_and_signal_core() {
        let samples = [
            Sample::new(0.0, 0.0),
            Sample::new(0.1, 3.3),
            Sample::new(0.2, 5.2),
        ];
        let mut source = SliceSampleSource::new(&samples);
        let mut sink = LastResultSink::default();
        let mut runtime = NoopRuntime::default();

        let evaluation = run_threshold_stream(
            &mut source,
            &mut sink,
            &mut runtime,
            ThresholdLimits::new(Some(0.0), Some(5.0)),
        )
        .expect("threshold stream should evaluate");

        assert!(!evaluation.passed);
        assert_eq!(evaluation.failed_check, Some(ThresholdCheck::Maximum));
        assert_eq!(sink.threshold, Some(evaluation));
        assert_eq!(runtime.samples, 3);
        assert_eq!(runtime.polls, 4);
    }

    #[test]
    fn transient_event_stream_records_longest_event() {
        let samples = [
            Sample::new(0.0, 5.0),
            Sample::new(0.1, 0.0),
            Sample::new(0.3, 5.0),
        ];
        let mut source = SliceSampleSource::new(&samples);
        let mut sink = LastResultSink::default();
        let mut runtime = NoopRuntime::default();
        let config =
            TransientEventConfig::new(TransientEventKind::Dropout, SignalState::High, 2.5, 0.1);

        let evaluation = run_transient_event_stream(&mut source, &mut sink, &mut runtime, config)
            .expect("transient stream should evaluate");

        assert!(!evaluation.passed);
        assert_eq!(evaluation.measured_duration_s, 0.20000002);
        assert_eq!(sink.transient_event, Some(evaluation));
        assert_eq!(runtime.samples, 3);
        assert_eq!(runtime.polls, 4);
    }

    #[test]
    fn empty_threshold_stream_returns_signal_error_without_sink_record() {
        let mut source = SliceSampleSource::new(&[]);
        let mut sink = LastResultSink::default();
        let mut runtime = NoopRuntime::default();

        let error = run_threshold_stream(
            &mut source,
            &mut sink,
            &mut runtime,
            ThresholdLimits::new(Some(0.0), Some(5.0)),
        )
        .expect_err("empty stream should fail");

        assert!(matches!(
            error,
            AdapterError::Signal(SignalError::EmptyInput)
        ));
        assert_eq!(sink.threshold, None);
        assert_eq!(runtime.polls, 1);
    }

    #[test]
    fn non_monotonic_stream_propagates_signal_error() {
        let samples = [Sample::new(0.0, 0.0), Sample::new(0.0, 1.0)];
        let mut source = SliceSampleSource::new(&samples);
        let mut sink = LastResultSink::default();
        let mut runtime = NoopRuntime::default();

        let error = run_threshold_stream(
            &mut source,
            &mut sink,
            &mut runtime,
            ThresholdLimits::new(Some(0.0), Some(5.0)),
        )
        .expect_err("non-monotonic stream should fail");

        assert!(matches!(
            error,
            AdapterError::Signal(SignalError::NonMonotonicTimestamp)
        ));
        assert_eq!(sink.threshold, None);
    }
}
