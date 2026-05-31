#![no_std]

//! Zephyr feasibility sketch for a future `wra-embedded` adapter.
//!
//! This file is not built by the workspace. It records the intended boundary
//! without adding Zephyr SDK, CMake, west, bindings, or unsafe FFI.

use core::convert::Infallible;

use wra_embedded::{EventSink, RuntimeHooks, SampleSource};
use wra_signal::{Sample, ThresholdEvaluation, TransientEventEvaluation};

pub struct ZephyrSampleSource;

impl SampleSource for ZephyrSampleSource {
    type Error = Infallible;

    fn poll_sample(&mut self) -> Result<Option<Sample>, Self::Error> {
        // Future adapter work should read one timestamped value from a Zephyr
        // driver or queue and convert it into `wra_signal::Sample`.
        Ok(None)
    }
}

pub struct ZephyrEventSink;

impl EventSink for ZephyrEventSink {
    type Error = Infallible;

    fn record_threshold(&mut self, _evaluation: &ThresholdEvaluation) -> Result<(), Self::Error> {
        // Future adapter work may forward this to a queue, telemetry packet,
        // assertion hook, or retained result buffer.
        Ok(())
    }

    fn record_transient_event(
        &mut self,
        _evaluation: &TransientEventEvaluation,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub struct ZephyrRuntimeHooks;

impl RuntimeHooks for ZephyrRuntimeHooks {
    type Error = Infallible;

    fn before_poll(&mut self) -> Result<(), Self::Error> {
        // Future adapter work may wait on a Zephyr queue/semaphore or poll a
        // driver-facing ring buffer here.
        Ok(())
    }

    fn after_sample(&mut self, _sample: Sample) -> Result<(), Self::Error> {
        // Future adapter work may yield, feed a watchdog, or update runtime
        // telemetry here. No RTOS API is called in this feasibility sketch.
        Ok(())
    }
}
