#![no_std]

//! ARM64 QEMU proof slice for `wra-embedded`.
//!
//! This library is intentionally host-checkable and does not open files,
//! allocate memory, or depend on QEMU at test time. The README documents the
//! target/QEMU assumptions for turning this into a freestanding demo image.

use wra_embedded::{run_threshold_stream, LastResultSink, NoopRuntime, SliceSampleSource};
use wra_signal::{Sample, ThresholdCheck, ThresholdLimits};

pub const DEMO_SAMPLES: [Sample; 4] = [
    Sample::new(0.0, 0.0),
    Sample::new(0.001, 3.3),
    Sample::new(0.002, 5.2),
    Sample::new(0.003, 3.1),
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DemoOutcome {
    pub passed: bool,
    pub sample_count: usize,
    pub failed_check: Option<ThresholdCheck>,
    pub min_value: f32,
    pub max_value: f32,
    pub runtime_samples: usize,
}

impl DemoOutcome {
    const fn error() -> Self {
        Self {
            passed: false,
            sample_count: 0,
            failed_check: None,
            min_value: 0.0,
            max_value: 0.0,
            runtime_samples: 0,
        }
    }
}

pub fn run_demo() -> DemoOutcome {
    let mut source = SliceSampleSource::new(&DEMO_SAMPLES);
    let mut sink = LastResultSink::default();
    let mut runtime = NoopRuntime::default();

    let result = run_threshold_stream(
        &mut source,
        &mut sink,
        &mut runtime,
        ThresholdLimits::new(Some(0.0), Some(5.0)),
    );

    match result {
        Ok(evaluation) => DemoOutcome {
            passed: evaluation.passed,
            sample_count: evaluation.sample_count,
            failed_check: evaluation.failed_check,
            min_value: evaluation.min_value,
            max_value: evaluation.max_value,
            runtime_samples: runtime.samples,
        },
        Err(_) => DemoOutcome::error(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qemu_demo_exercises_no_std_threshold_path() {
        let outcome = run_demo();

        assert!(!outcome.passed);
        assert_eq!(outcome.sample_count, 4);
        assert_eq!(outcome.failed_check, Some(ThresholdCheck::Maximum));
        assert_eq!(outcome.max_value, 5.2);
        assert_eq!(outcome.runtime_samples, 4);
    }
}
