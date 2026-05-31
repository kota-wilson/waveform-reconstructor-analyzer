#![no_std]

//! Measurement primitives for engineering signal validation.
//!
//! This crate owns reusable measurement calculations over time/sample slices.
//! It deliberately avoids CSV parsing, report rendering, plotting, file I/O,
//! allocation, and third-party dependencies so criteria, reports, plots, and
//! future adapters can consume the same measured facts.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasurementError {
    EmptyInput,
    MismatchedLength,
    NonMonotonicTimeAxis,
}

pub type Result<T> = core::result::Result<T, MeasurementError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalState {
    High,
    Low,
}

impl SignalState {
    pub fn from_config(value: &str) -> Option<Self> {
        match value {
            "high" => Some(Self::High),
            "low" => Some(Self::Low),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Low => "low",
        }
    }

    pub const fn opposite(self) -> Self {
        match self {
            Self::High => Self::Low,
            Self::Low => Self::High,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeDirection {
    Rise,
    Fall,
}

impl EdgeDirection {
    pub fn from_config(value: &str) -> Option<Self> {
        match value {
            "rise" => Some(Self::Rise),
            "fall" => Some(Self::Fall),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rise => "rise",
            Self::Fall => "fall",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SampleMeasurement {
    pub sample_index: usize,
    pub timestamp: f64,
    pub value: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransitionCount {
    pub count: usize,
    pub first_index: Option<usize>,
    pub first_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateRun {
    pub state: SignalState,
    pub start_index: usize,
    pub start_time: f64,
    pub duration_s: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunSelection {
    Shortest,
    Longest,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeTimeMeasurement {
    pub start_index: usize,
    pub start_time: f64,
    pub end_index: usize,
    pub end_time: f64,
    pub duration_s: f64,
}

pub fn minimum_sample(time: &[f64], samples: &[f64]) -> Result<SampleMeasurement> {
    extremum_sample(time, samples, |candidate, current| candidate < current)
}

pub fn maximum_sample(time: &[f64], samples: &[f64]) -> Result<SampleMeasurement> {
    extremum_sample(time, samples, |candidate, current| candidate > current)
}

pub fn count_state_transitions(
    time: &[f64],
    samples: &[f64],
    threshold_v: f64,
    voltage_tolerance_v: f64,
) -> Result<TransitionCount> {
    validate_time_and_samples(time, samples)?;

    let mut count = 0_usize;
    let mut first_index = None;
    for (index, pair) in samples.windows(2).enumerate() {
        let previous = sample_state(pair[0], threshold_v, voltage_tolerance_v);
        let next = sample_state(pair[1], threshold_v, voltage_tolerance_v);
        if previous != next {
            let transition_index = index + 1;
            count += 1;
            if first_index.is_none() {
                first_index = Some(transition_index);
            }
        }
    }

    Ok(TransitionCount {
        count,
        first_index,
        first_timestamp: first_index.map(|index| time[index]),
    })
}

pub fn state_run_extremum(
    time: &[f64],
    samples: &[f64],
    state: SignalState,
    threshold_v: f64,
    voltage_tolerance_v: f64,
    selection: RunSelection,
) -> Result<Option<StateRun>> {
    validate_strict_time_and_samples(time, samples)?;

    let mut best = None;
    let mut start_index = 0_usize;
    let mut current_state = sample_state(samples[0], threshold_v, voltage_tolerance_v);

    for (index, sample) in samples.iter().copied().enumerate().skip(1) {
        let next_state = sample_state(sample, threshold_v, voltage_tolerance_v);
        if next_state != current_state {
            update_selected_run(
                &mut best,
                run_from_indices(time, current_state, start_index, index - 1),
                state,
                selection,
            );
            start_index = index;
            current_state = next_state;
        }
    }

    update_selected_run(
        &mut best,
        run_from_indices(time, current_state, start_index, samples.len() - 1),
        state,
        selection,
    );

    Ok(best)
}

pub fn measure_rise_time(
    time: &[f64],
    samples: &[f64],
    low_threshold_v: f64,
    high_threshold_v: f64,
    voltage_tolerance_v: f64,
) -> Result<Option<EdgeTimeMeasurement>> {
    validate_strict_time_and_samples(time, samples)?;

    let Some(start_index) = samples
        .iter()
        .position(|value| *value + voltage_tolerance_v >= low_threshold_v)
    else {
        return Ok(None);
    };
    let Some(end_index) =
        samples
            .iter()
            .enumerate()
            .skip(start_index)
            .find_map(|(index, value)| {
                (*value + voltage_tolerance_v >= high_threshold_v).then_some(index)
            })
    else {
        return Ok(None);
    };

    Ok(Some(edge_measurement(time, start_index, end_index)))
}

pub fn measure_fall_time(
    time: &[f64],
    samples: &[f64],
    low_threshold_v: f64,
    high_threshold_v: f64,
    voltage_tolerance_v: f64,
) -> Result<Option<EdgeTimeMeasurement>> {
    validate_strict_time_and_samples(time, samples)?;

    let Some(high_index) = samples
        .iter()
        .position(|value| *value + voltage_tolerance_v >= high_threshold_v)
    else {
        return Ok(None);
    };
    let Some(start_index) =
        samples
            .iter()
            .enumerate()
            .skip(high_index)
            .find_map(|(index, value)| {
                (*value - voltage_tolerance_v <= high_threshold_v).then_some(index)
            })
    else {
        return Ok(None);
    };
    let Some(end_index) =
        samples
            .iter()
            .enumerate()
            .skip(start_index)
            .find_map(|(index, value)| {
                (*value - voltage_tolerance_v <= low_threshold_v).then_some(index)
            })
    else {
        return Ok(None);
    };

    Ok(Some(edge_measurement(time, start_index, end_index)))
}

fn extremum_sample(
    time: &[f64],
    samples: &[f64],
    is_better: impl Fn(f64, f64) -> bool,
) -> Result<SampleMeasurement> {
    validate_time_and_samples(time, samples)?;

    let mut best_index = 0_usize;
    let mut best_value = samples[0];
    for (index, value) in samples.iter().copied().enumerate().skip(1) {
        if is_better(value, best_value) {
            best_index = index;
            best_value = value;
        }
    }

    Ok(SampleMeasurement {
        sample_index: best_index,
        timestamp: time[best_index],
        value: best_value,
    })
}

fn validate_time_and_samples(time: &[f64], samples: &[f64]) -> Result<()> {
    if time.is_empty() || samples.is_empty() {
        return Err(MeasurementError::EmptyInput);
    }
    if time.len() != samples.len() {
        return Err(MeasurementError::MismatchedLength);
    }
    Ok(())
}

fn validate_strict_time_and_samples(time: &[f64], samples: &[f64]) -> Result<()> {
    validate_time_and_samples(time, samples)?;
    if time.windows(2).any(|pair| pair[1] <= pair[0]) {
        return Err(MeasurementError::NonMonotonicTimeAxis);
    }
    Ok(())
}

fn update_selected_run(
    best: &mut Option<StateRun>,
    candidate: StateRun,
    target_state: SignalState,
    selection: RunSelection,
) {
    if candidate.state != target_state {
        return;
    }

    let replace = match best {
        Some(current) => match selection {
            RunSelection::Shortest => candidate.duration_s < current.duration_s,
            RunSelection::Longest => candidate.duration_s >= current.duration_s,
        },
        None => true,
    };

    if replace {
        *best = Some(candidate);
    }
}

fn run_from_indices(
    time: &[f64],
    state: SignalState,
    start_index: usize,
    end_index: usize,
) -> StateRun {
    let end_time = time.get(end_index + 1).copied().unwrap_or(time[end_index]);
    StateRun {
        state,
        start_index,
        start_time: time[start_index],
        duration_s: end_time - time[start_index],
    }
}

fn edge_measurement(time: &[f64], start_index: usize, end_index: usize) -> EdgeTimeMeasurement {
    EdgeTimeMeasurement {
        start_index,
        start_time: time[start_index],
        end_index,
        end_time: time[end_index],
        duration_s: time[end_index] - time[start_index],
    }
}

fn sample_state(value: f64, threshold_v: f64, voltage_tolerance_v: f64) -> SignalState {
    if value + voltage_tolerance_v >= threshold_v {
        SignalState::High
    } else {
        SignalState::Low
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measures_voltage_extrema() {
        let time = [0.0, 0.1, 0.2];
        let samples = [1.0, -0.5, 4.0];

        let minimum = minimum_sample(&time, &samples).expect("minimum should measure");
        let maximum = maximum_sample(&time, &samples).expect("maximum should measure");

        assert_eq!(minimum.sample_index, 1);
        assert_eq!(minimum.timestamp, 0.1);
        assert_eq!(minimum.value, -0.5);
        assert_eq!(maximum.sample_index, 2);
        assert_eq!(maximum.timestamp, 0.2);
        assert_eq!(maximum.value, 4.0);
    }

    #[test]
    fn counts_state_transitions() {
        let time = [0.0, 0.001, 0.002, 0.003, 0.004];
        let samples = [0.0, 0.0, 5.0, 5.0, 0.0];

        let measurement =
            count_state_transitions(&time, &samples, 2.5, 0.0).expect("count should measure");

        assert_eq!(measurement.count, 2);
        assert_eq!(measurement.first_index, Some(2));
        assert_eq!(measurement.first_timestamp, Some(0.002));
    }

    #[test]
    fn selects_shortest_and_longest_state_runs() {
        let time = [0.0, 0.001, 0.002, 0.004, 0.007, 0.008];
        let samples = [0.0, 5.0, 5.0, 0.0, 5.0, 0.0];

        let shortest = state_run_extremum(
            &time,
            &samples,
            SignalState::High,
            2.5,
            0.0,
            RunSelection::Shortest,
        )
        .expect("shortest should measure")
        .expect("high run should exist");
        let longest = state_run_extremum(
            &time,
            &samples,
            SignalState::High,
            2.5,
            0.0,
            RunSelection::Longest,
        )
        .expect("longest should measure")
        .expect("high run should exist");

        assert_eq!(shortest.start_index, 4);
        assert_eq!(shortest.duration_s, 0.001);
        assert_eq!(longest.start_index, 1);
        assert_eq!(longest.duration_s, 0.003);
    }

    #[test]
    fn measures_rise_and_fall_time() {
        let time = [0.0, 0.001, 0.002, 0.003, 0.004, 0.005];
        let samples = [0.0, 0.5, 2.5, 4.5, 5.0, 0.0];

        let rise = measure_rise_time(&time, &samples, 0.5, 4.5, 0.0)
            .expect("rise should measure")
            .expect("rise should exist");
        let fall = measure_fall_time(&time, &samples, 0.5, 4.5, 0.0)
            .expect("fall should measure")
            .expect("fall should exist");

        assert_eq!(rise.start_index, 1);
        assert_eq!(rise.end_index, 3);
        assert_eq!(rise.duration_s, 0.002);
        assert_eq!(fall.start_index, 3);
        assert_eq!(fall.end_index, 5);
        assert_eq!(fall.duration_s, 0.002);
    }

    #[test]
    fn rejects_non_monotonic_time_for_duration_measurements() {
        let time = [0.0, 0.001, 0.001];
        let samples = [0.0, 5.0, 0.0];

        let result = state_run_extremum(
            &time,
            &samples,
            SignalState::High,
            2.5,
            0.0,
            RunSelection::Longest,
        );

        assert_eq!(result, Err(MeasurementError::NonMonotonicTimeAxis));
    }
}
