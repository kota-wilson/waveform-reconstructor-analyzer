use crate::error::{Result, WaveformError};
use crate::model::{
    Channel, TransformCategory, TransformExecutionMetadata, TransformParameterMetadata,
    TransformPhaseEffect, TransformStepMetadata, Waveform,
};

const TAU: f64 = std::f64::consts::PI * 2.0;
const MAX_ADC_BITS: u8 = 24;

pub trait Filter {
    fn name(&self) -> &'static str;
    fn apply(&self, waveform: &Waveform) -> Result<Waveform>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterStep {
    Offset(OffsetTransform),
    Gain(GainTransform),
    Invert(InvertTransform),
    Clamp(ClampTransform),
    Deadband(DeadbandTransform),
    DcRemove(DcRemoveTransform),
    BaselineSubtract(BaselineSubtractTransform),
    MovingAverage(MovingAverageFilter),
    MovingMedian(MovingMedianFilter),
    LowPass(LowPassFilter),
    AdcQuantize(AdcQuantizer),
}

impl Filter for FilterStep {
    fn name(&self) -> &'static str {
        match self {
            Self::Offset(filter) => filter.name(),
            Self::Gain(filter) => filter.name(),
            Self::Invert(filter) => filter.name(),
            Self::Clamp(filter) => filter.name(),
            Self::Deadband(filter) => filter.name(),
            Self::DcRemove(filter) => filter.name(),
            Self::BaselineSubtract(filter) => filter.name(),
            Self::MovingAverage(filter) => filter.name(),
            Self::MovingMedian(filter) => filter.name(),
            Self::LowPass(filter) => filter.name(),
            Self::AdcQuantize(filter) => filter.name(),
        }
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        match self {
            Self::Offset(filter) => filter.apply(waveform),
            Self::Gain(filter) => filter.apply(waveform),
            Self::Invert(filter) => filter.apply(waveform),
            Self::Clamp(filter) => filter.apply(waveform),
            Self::Deadband(filter) => filter.apply(waveform),
            Self::DcRemove(filter) => filter.apply(waveform),
            Self::BaselineSubtract(filter) => filter.apply(waveform),
            Self::MovingAverage(filter) => filter.apply(waveform),
            Self::MovingMedian(filter) => filter.apply(waveform),
            Self::LowPass(filter) => filter.apply(waveform),
            Self::AdcQuantize(filter) => filter.apply(waveform),
        }
    }
}

pub fn apply_filter_chain(waveform: &Waveform, filters: &[FilterStep]) -> Result<Waveform> {
    let mut derived = waveform.clone();
    for filter in filters {
        derived = filter.apply(&derived)?;
    }
    Ok(derived)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OffsetTransform {
    pub offset_v: f64,
}

impl Filter for OffsetTransform {
    fn name(&self) -> &'static str {
        "offset"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_finite_parameter("offset_v", self.offset_v)?;
        let channels = map_samples(waveform, |sample| sample + self.offset_v)?;
        let history_label = format!("offset(offset_v={})", self.offset_v);
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "offset",
            TransformCategory::Pointwise,
            vec![TransformParameterMetadata::float(
                "offset_v",
                self.offset_v,
                "V",
            )],
            false,
            false,
            TransformPhaseEffect::None,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GainTransform {
    pub gain: f64,
}

impl Filter for GainTransform {
    fn name(&self) -> &'static str {
        "gain"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_finite_parameter("gain", self.gain)?;
        let channels = map_samples(waveform, |sample| sample * self.gain)?;
        let history_label = format!("gain(gain={})", self.gain);
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "gain",
            TransformCategory::Pointwise,
            vec![TransformParameterMetadata::float(
                "gain", self.gain, "ratio",
            )],
            false,
            false,
            TransformPhaseEffect::None,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InvertTransform;

impl Filter for InvertTransform {
    fn name(&self) -> &'static str {
        "invert"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let channels = map_samples(waveform, |sample| -sample)?;
        let transform_step = TransformStepMetadata::implemented_desktop(
            "invert()",
            "invert",
            TransformCategory::Pointwise,
            Vec::new(),
            false,
            false,
            TransformPhaseEffect::None,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClampTransform {
    pub min_v: f64,
    pub max_v: f64,
}

impl Filter for ClampTransform {
    fn name(&self) -> &'static str {
        "clamp"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        self.validate()?;
        let channels = map_samples(waveform, |sample| sample.clamp(self.min_v, self.max_v))?;
        let history_label = format!("clamp(min_v={},max_v={})", self.min_v, self.max_v);
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "clamp",
            TransformCategory::Pointwise,
            vec![
                TransformParameterMetadata::float("min_v", self.min_v, "V"),
                TransformParameterMetadata::float("max_v", self.max_v, "V"),
            ],
            false,
            false,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

impl ClampTransform {
    fn validate(&self) -> Result<()> {
        validate_finite_parameter("min_v", self.min_v)?;
        validate_finite_parameter("max_v", self.max_v)?;
        if self.max_v < self.min_v {
            return Err(WaveformError::InvalidParameter {
                name: "max_v".to_string(),
                reason: "must be greater than or equal to min_v".to_string(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeadbandTransform {
    pub threshold_v: f64,
}

impl Filter for DeadbandTransform {
    fn name(&self) -> &'static str {
        "deadband"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        self.validate()?;
        let channels = map_samples(waveform, |sample| {
            if sample.abs() <= self.threshold_v {
                0.0
            } else {
                sample
            }
        })?;
        let history_label = format!("deadband(threshold_v={})", self.threshold_v);
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "deadband",
            TransformCategory::Pointwise,
            vec![TransformParameterMetadata::float(
                "threshold_v",
                self.threshold_v,
                "V",
            )],
            false,
            false,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

impl DeadbandTransform {
    fn validate(&self) -> Result<()> {
        validate_finite_parameter("threshold_v", self.threshold_v)?;
        if self.threshold_v < 0.0 {
            return Err(WaveformError::InvalidParameter {
                name: "threshold_v".to_string(),
                reason: "must be greater than or equal to zero".to_string(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DcRemoveTransform;

impl Filter for DcRemoveTransform {
    fn name(&self) -> &'static str {
        "dc_remove"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_finite_samples("dc_remove", &channel.samples)?;
                let mean = channel.samples.iter().sum::<f64>() / channel.samples.len() as f64;
                let samples = channel.samples.iter().map(|sample| sample - mean).collect();
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            "dc_remove()",
            "dc_remove",
            TransformCategory::Baseline,
            Vec::new(),
            TransformExecutionMetadata {
                sample_rate_required: false,
                stateful: true,
                causal: false,
                phase_effect: TransformPhaseEffect::None,
                streaming_supported: false,
                offline_only: true,
            },
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BaselineSubtractTransform {
    pub baseline_v: f64,
}

impl Filter for BaselineSubtractTransform {
    fn name(&self) -> &'static str {
        "baseline_subtract"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_finite_parameter("baseline_v", self.baseline_v)?;
        let channels = map_samples(waveform, |sample| sample - self.baseline_v)?;
        let history_label = format!("baseline_subtract(baseline_v={})", self.baseline_v);
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "baseline_subtract",
            TransformCategory::Baseline,
            vec![TransformParameterMetadata::float(
                "baseline_v",
                self.baseline_v,
                "V",
            )],
            false,
            false,
            TransformPhaseEffect::None,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MovingAverageFilter {
    pub window_samples: usize,
}

impl Filter for MovingAverageFilter {
    fn name(&self) -> &'static str {
        "moving_average"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        if self.window_samples == 0 {
            return Err(WaveformError::InvalidParameter {
                name: "window_samples".to_string(),
                reason: "must be greater than zero".to_string(),
            });
        }

        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                let samples = trailing_moving_average(&channel.samples, self.window_samples);
                Channel::new(channel.name.clone(), channel.unit.clone(), samples)
            })
            .collect();

        let history_label = format!("moving_average(window_samples={})", self.window_samples);
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "moving_average",
            TransformCategory::Windowed,
            vec![TransformParameterMetadata::integer(
                "window_samples",
                self.window_samples as u64,
                "samples",
            )],
            false,
            true,
            TransformPhaseEffect::Delay,
        );

        Ok(Waveform::new_with_time_unit(
            waveform.time.clone(),
            waveform.time_unit.clone(),
            channels,
        )?
        .as_derived_from_with_transform_step(waveform, transform_step))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MovingMedianFilter {
    pub window_samples: usize,
}

impl Filter for MovingMedianFilter {
    fn name(&self) -> &'static str {
        "moving_median"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        if self.window_samples == 0 {
            return Err(WaveformError::InvalidParameter {
                name: "window_samples".to_string(),
                reason: "must be greater than zero".to_string(),
            });
        }

        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                let samples = trailing_moving_median(&channel.samples, self.window_samples)?;
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let history_label = format!("moving_median(window_samples={})", self.window_samples);
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "moving_median",
            TransformCategory::Windowed,
            vec![TransformParameterMetadata::integer(
                "window_samples",
                self.window_samples as u64,
                "samples",
            )],
            false,
            true,
            TransformPhaseEffect::Nonlinear,
        );

        Ok(Waveform::new_with_time_unit(
            waveform.time.clone(),
            waveform.time_unit.clone(),
            channels,
        )?
        .as_derived_from_with_transform_step(waveform, transform_step))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LowPassFilter {
    pub cutoff_hz: f64,
}

impl Filter for LowPassFilter {
    fn name(&self) -> &'static str {
        "low_pass"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        if self.cutoff_hz <= 0.0 {
            return Err(WaveformError::InvalidParameter {
                name: "cutoff_hz".to_string(),
                reason: "must be greater than zero".to_string(),
            });
        }
        validate_time_axis(&waveform.time)?;

        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                let samples =
                    first_order_low_pass(&waveform.time, &channel.samples, self.cutoff_hz);
                Channel::new(channel.name.clone(), channel.unit.clone(), samples)
            })
            .collect();

        let history_label = format!("low_pass(cutoff_hz={})", self.cutoff_hz);
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "low_pass",
            TransformCategory::FrequencyFilter,
            vec![TransformParameterMetadata::float(
                "cutoff_hz",
                self.cutoff_hz,
                "Hz",
            )],
            true,
            true,
            TransformPhaseEffect::Delay,
        );

        Ok(Waveform::new_with_time_unit(
            waveform.time.clone(),
            waveform.time_unit.clone(),
            channels,
        )?
        .as_derived_from_with_transform_step(waveform, transform_step))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AdcQuantizer {
    pub bits: u8,
    pub min_v: f64,
    pub max_v: f64,
}

impl Filter for AdcQuantizer {
    fn name(&self) -> &'static str {
        "adc_quantize"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        self.validate()?;

        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                let samples = channel
                    .samples
                    .iter()
                    .copied()
                    .map(|sample| self.quantize_sample(sample))
                    .collect::<Result<Vec<_>>>()?;
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let history_label = format!(
            "adc_quantize(bits={},min_v={},max_v={})",
            self.bits, self.min_v, self.max_v
        );
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "adc_quantize",
            TransformCategory::Quantization,
            vec![
                TransformParameterMetadata::integer("bits", self.bits as u64, "bits"),
                TransformParameterMetadata::float("min_v", self.min_v, "V"),
                TransformParameterMetadata::float("max_v", self.max_v, "V"),
            ],
            false,
            false,
            TransformPhaseEffect::None,
        );

        Ok(Waveform::new_with_time_unit(
            waveform.time.clone(),
            waveform.time_unit.clone(),
            channels,
        )?
        .as_derived_from_with_transform_step(waveform, transform_step))
    }
}

impl AdcQuantizer {
    fn validate(&self) -> Result<()> {
        if self.bits == 0 || self.bits > MAX_ADC_BITS {
            return Err(WaveformError::InvalidParameter {
                name: "bits".to_string(),
                reason: format!("must be between 1 and {MAX_ADC_BITS}"),
            });
        }
        if !self.min_v.is_finite() {
            return Err(WaveformError::InvalidParameter {
                name: "min_v".to_string(),
                reason: "must be finite".to_string(),
            });
        }
        if !self.max_v.is_finite() {
            return Err(WaveformError::InvalidParameter {
                name: "max_v".to_string(),
                reason: "must be finite".to_string(),
            });
        }
        if self.max_v <= self.min_v {
            return Err(WaveformError::InvalidParameter {
                name: "max_v".to_string(),
                reason: "must be greater than min_v".to_string(),
            });
        }
        Ok(())
    }

    fn quantize_sample(&self, sample: f64) -> Result<f64> {
        if !sample.is_finite() {
            return Err(WaveformError::InvalidWaveform {
                reason: "ADC quantization requires finite samples".to_string(),
            });
        }

        let max_code = (1_u64 << self.bits) - 1;
        let normalized = ((sample - self.min_v) / (self.max_v - self.min_v)).clamp(0.0, 1.0);
        let code = (normalized * max_code as f64).round();
        let quantized = self.min_v + (code / max_code as f64) * (self.max_v - self.min_v);
        Ok(quantized)
    }
}

fn map_samples(waveform: &Waveform, transform: impl Fn(f64) -> f64 + Copy) -> Result<Vec<Channel>> {
    waveform
        .channels
        .iter()
        .map(|channel| {
            validate_finite_samples("pointwise transform", &channel.samples)?;
            let samples: Vec<f64> = channel.samples.iter().copied().map(transform).collect();
            validate_finite_samples("pointwise transform output", &samples)?;
            Ok(Channel::new(
                channel.name.clone(),
                channel.unit.clone(),
                samples,
            ))
        })
        .collect()
}

fn derived_waveform(
    source: &Waveform,
    channels: Vec<Channel>,
    transform_step: TransformStepMetadata,
) -> Result<Waveform> {
    Ok(
        Waveform::new_with_time_unit(source.time.clone(), source.time_unit.clone(), channels)?
            .as_derived_from_with_transform_step(source, transform_step),
    )
}

fn validate_finite_parameter(name: &str, value: f64) -> Result<()> {
    if !value.is_finite() {
        return Err(WaveformError::InvalidParameter {
            name: name.to_string(),
            reason: "must be finite".to_string(),
        });
    }
    Ok(())
}

fn validate_finite_samples(transform_name: &str, samples: &[f64]) -> Result<()> {
    if samples.iter().any(|sample| !sample.is_finite()) {
        return Err(WaveformError::InvalidWaveform {
            reason: format!("{transform_name} requires finite samples"),
        });
    }
    Ok(())
}

fn trailing_moving_average(samples: &[f64], window_samples: usize) -> Vec<f64> {
    let mut filtered = Vec::with_capacity(samples.len());
    for index in 0..samples.len() {
        let start = (index + 1).saturating_sub(window_samples);
        let window = &samples[start..=index];
        filtered.push(window.iter().sum::<f64>() / window.len() as f64);
    }
    filtered
}

fn trailing_moving_median(samples: &[f64], window_samples: usize) -> Result<Vec<f64>> {
    validate_finite_samples("moving_median", samples)?;
    let mut filtered = Vec::with_capacity(samples.len());
    for index in 0..samples.len() {
        let start = (index + 1).saturating_sub(window_samples);
        let mut window = samples[start..=index].to_vec();
        window.sort_by(f64::total_cmp);
        let middle = window.len() / 2;
        let median = if window.len() % 2 == 0 {
            (window[middle - 1] + window[middle]) / 2.0
        } else {
            window[middle]
        };
        filtered.push(median);
    }
    Ok(filtered)
}

fn validate_time_axis(time: &[f64]) -> Result<()> {
    for pair in time.windows(2) {
        if pair[1] <= pair[0] {
            return Err(WaveformError::InvalidWaveform {
                reason: "time samples must be strictly increasing for low-pass filtering"
                    .to_string(),
            });
        }
    }
    Ok(())
}

fn first_order_low_pass(time: &[f64], samples: &[f64], cutoff_hz: f64) -> Vec<f64> {
    if samples.is_empty() {
        return Vec::new();
    }

    let rc = 1.0 / (TAU * cutoff_hz);
    let mut filtered = Vec::with_capacity(samples.len());
    filtered.push(samples[0]);

    for index in 1..samples.len() {
        let dt = time[index] - time[index - 1];
        let alpha = dt / (rc + dt);
        let previous = filtered[index - 1];
        filtered.push(previous + alpha * (samples[index] - previous));
    }

    filtered
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        TransformCapabilityStatus, TransformEvidenceLevel, TransformInputChannelKind,
        TransformOutputChannelKind, TransformParameterValue, TransformRuntimeProfile, Unit,
    };

    fn waveform(samples: Vec<f64>) -> Waveform {
        Waveform::new(
            vec![0.0, 1.0, 2.0, 3.0],
            vec![Channel::new("input_v", Unit::volts(), samples)],
        )
        .expect("test waveform should be valid")
    }

    fn assert_common_transform_metadata(step: &TransformStepMetadata) {
        assert_eq!(
            step.input_channels.kind,
            TransformInputChannelKind::AllChannels
        );
        assert_eq!(
            step.output_channels.kind,
            TransformOutputChannelKind::DerivedChannels
        );
        assert!(step.output_channels.preserves_names);
        assert!(step.causal);
        assert!(step.streaming_supported);
        assert!(!step.offline_only);
        assert_eq!(
            step.runtime_profiles,
            vec![TransformRuntimeProfile::Desktop]
        );
        assert_eq!(
            step.capability_status,
            TransformCapabilityStatus::Implemented
        );
        assert_eq!(
            step.evidence_level,
            TransformEvidenceLevel::GoldenReportTested
        );
    }

    #[test]
    fn pointwise_transforms_apply_without_mutating_input() {
        let input = waveform(vec![-2.0, -0.5, 1.0, 4.0]);

        let offset = OffsetTransform { offset_v: 1.0 }
            .apply(&input)
            .expect("offset should apply");
        assert_eq!(input.channels[0].samples, vec![-2.0, -0.5, 1.0, 4.0]);
        assert_eq!(offset.channels[0].samples, vec![-1.0, 0.5, 2.0, 5.0]);
        assert_eq!(
            offset.metadata.transform_history,
            vec!["offset(offset_v=1)"]
        );
        assert_common_transform_metadata(&offset.metadata.transform_steps[0]);
        assert_eq!(offset.metadata.transform_steps[0].name, "offset");
        assert_eq!(
            offset.metadata.transform_steps[0].category,
            TransformCategory::Pointwise
        );
        assert_eq!(
            offset.metadata.transform_steps[0].parameters[0].value,
            TransformParameterValue::Float(1.0)
        );
        assert_eq!(
            offset.metadata.transform_steps[0].parameters[0]
                .unit
                .as_deref(),
            Some("V")
        );

        let gained = GainTransform { gain: 2.0 }
            .apply(&input)
            .expect("gain should apply");
        assert_eq!(gained.channels[0].samples, vec![-4.0, -1.0, 2.0, 8.0]);
        assert_eq!(gained.metadata.transform_history, vec!["gain(gain=2)"]);

        let inverted = InvertTransform.apply(&input).expect("invert should apply");
        assert_eq!(inverted.channels[0].samples, vec![2.0, 0.5, -1.0, -4.0]);
        assert_eq!(inverted.metadata.transform_history, vec!["invert()"]);

        let clamped = ClampTransform {
            min_v: -1.0,
            max_v: 3.0,
        }
        .apply(&input)
        .expect("clamp should apply");
        assert_eq!(clamped.channels[0].samples, vec![-1.0, -0.5, 1.0, 3.0]);
        assert_eq!(
            clamped.metadata.transform_history,
            vec!["clamp(min_v=-1,max_v=3)"]
        );
        assert_eq!(
            clamped.metadata.transform_steps[0].phase_effect,
            TransformPhaseEffect::Nonlinear
        );
    }

    #[test]
    fn deadband_and_baseline_transforms_preserve_raw_samples() {
        let input = waveform(vec![0.0, 1.0, 2.0, 5.0]);

        let deadbanded = DeadbandTransform { threshold_v: 1.0 }
            .apply(&input)
            .expect("deadband should apply");
        assert_eq!(input.channels[0].samples, vec![0.0, 1.0, 2.0, 5.0]);
        assert_eq!(deadbanded.channels[0].samples, vec![0.0, 0.0, 2.0, 5.0]);
        assert_eq!(
            deadbanded.metadata.transform_history,
            vec!["deadband(threshold_v=1)"]
        );
        assert_eq!(
            deadbanded.metadata.transform_steps[0].category,
            TransformCategory::Pointwise
        );
        assert_eq!(
            deadbanded.metadata.transform_steps[0].phase_effect,
            TransformPhaseEffect::Nonlinear
        );

        let dc_removed = DcRemoveTransform
            .apply(&input)
            .expect("dc removal should apply");
        assert_eq!(dc_removed.channels[0].samples, vec![-2.0, -1.0, 0.0, 3.0]);
        let dc_step = &dc_removed.metadata.transform_steps[0];
        assert_eq!(dc_step.name, "dc_remove");
        assert_eq!(dc_step.category, TransformCategory::Baseline);
        assert!(!dc_step.causal);
        assert!(!dc_step.streaming_supported);
        assert!(dc_step.offline_only);

        let baseline_subtracted = BaselineSubtractTransform { baseline_v: 1.0 }
            .apply(&input)
            .expect("baseline subtraction should apply");
        assert_eq!(
            baseline_subtracted.channels[0].samples,
            vec![-1.0, 0.0, 1.0, 4.0]
        );
        assert_eq!(
            baseline_subtracted.metadata.transform_history,
            vec!["baseline_subtract(baseline_v=1)"]
        );
        assert_eq!(
            baseline_subtracted.metadata.transform_steps[0].category,
            TransformCategory::Baseline
        );
    }

    #[test]
    fn moving_median_uses_trailing_window_edges() {
        let input = waveform(vec![0.0, 100.0, 2.0, 3.0]);
        let filtered = MovingMedianFilter { window_samples: 3 }
            .apply(&input)
            .expect("moving median should apply");

        assert_eq!(input.channels[0].samples, vec![0.0, 100.0, 2.0, 3.0]);
        assert_eq!(filtered.channels[0].samples, vec![0.0, 50.0, 2.0, 3.0]);
        assert_eq!(
            filtered.metadata.transform_history,
            vec!["moving_median(window_samples=3)"]
        );
        let step = &filtered.metadata.transform_steps[0];
        assert_common_transform_metadata(step);
        assert_eq!(step.name, "moving_median");
        assert_eq!(step.category, TransformCategory::Windowed);
        assert_eq!(
            step.parameters[0].value,
            TransformParameterValue::Integer(3)
        );
        assert_eq!(step.phase_effect, TransformPhaseEffect::Nonlinear);
    }

    #[test]
    fn m11_transforms_reject_invalid_parameters() {
        let input = waveform(vec![0.0, 1.0, 2.0, 3.0]);

        assert!(matches!(
            OffsetTransform { offset_v: f64::NAN }.apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            GainTransform {
                gain: f64::INFINITY
            }
            .apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            GainTransform { gain: 2.0 }.apply(&waveform(vec![f64::MAX, 0.0, 0.0, 0.0])),
            Err(WaveformError::InvalidWaveform { .. })
        ));
        assert!(matches!(
            ClampTransform {
                min_v: 2.0,
                max_v: 1.0,
            }
            .apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            DeadbandTransform { threshold_v: -0.1 }.apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            BaselineSubtractTransform {
                baseline_v: f64::NAN,
            }
            .apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            MovingMedianFilter { window_samples: 0 }.apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
    }

    #[test]
    fn moving_average_filters_each_channel_without_mutating_input() {
        let input = waveform(vec![0.0, 2.0, 4.0, 6.0]);
        let filtered = MovingAverageFilter { window_samples: 2 }
            .apply(&input)
            .expect("filter should apply");

        assert_eq!(input.channels[0].samples, vec![0.0, 2.0, 4.0, 6.0]);
        assert_eq!(filtered.channels[0].samples, vec![0.0, 1.0, 3.0, 5.0]);
        assert_eq!(
            filtered.metadata.lineage,
            crate::model::WaveformLineage::Derived
        );
        assert_eq!(
            filtered.metadata.transform_history,
            vec!["moving_average(window_samples=2)"]
        );
        let step = &filtered.metadata.transform_steps[0];
        assert_common_transform_metadata(step);
        assert_eq!(step.sequence_index, 0);
        assert_eq!(step.history_label, "moving_average(window_samples=2)");
        assert_eq!(step.name, "moving_average");
        assert_eq!(step.category, TransformCategory::Windowed);
        assert_eq!(
            step.parameters[0].value,
            TransformParameterValue::Integer(2)
        );
        assert_eq!(step.parameters[0].unit.as_deref(), Some("samples"));
        assert!(!step.sample_rate_required);
        assert!(step.stateful);
        assert_eq!(step.phase_effect, TransformPhaseEffect::Delay);
    }

    #[test]
    fn moving_average_rejects_zero_window() {
        let result =
            MovingAverageFilter { window_samples: 0 }.apply(&waveform(vec![0.0, 1.0, 2.0, 3.0]));

        assert!(matches!(
            result,
            Err(WaveformError::InvalidParameter { .. })
        ));
    }

    #[test]
    fn low_pass_smooths_step_input() {
        let input = waveform(vec![0.0, 10.0, 10.0, 10.0]);
        let filtered = LowPassFilter { cutoff_hz: 0.1 }
            .apply(&input)
            .expect("filter should apply");

        assert_eq!(filtered.channels[0].samples[0], 0.0);
        assert!(filtered.channels[0].samples[1] > 0.0);
        assert!(filtered.channels[0].samples[1] < 10.0);
        assert!(filtered.channels[0].samples[3] < 10.0);
        let step = &filtered.metadata.transform_steps[0];
        assert_common_transform_metadata(step);
        assert_eq!(step.name, "low_pass");
        assert_eq!(step.category, TransformCategory::FrequencyFilter);
        assert_eq!(
            step.parameters[0].value,
            TransformParameterValue::Float(0.1)
        );
        assert_eq!(step.parameters[0].unit.as_deref(), Some("Hz"));
        assert!(step.sample_rate_required);
        assert!(step.stateful);
        assert_eq!(step.phase_effect, TransformPhaseEffect::Delay);
    }

    #[test]
    fn adc_quantizer_snaps_samples_to_code_levels_without_mutating_input() {
        let input = waveform(vec![-0.5, 0.49, 1.51, 3.5]);
        let quantized = AdcQuantizer {
            bits: 2,
            min_v: 0.0,
            max_v: 3.0,
        }
        .apply(&input)
        .expect("quantizer should apply");

        assert_eq!(input.channels[0].samples, vec![-0.5, 0.49, 1.51, 3.5]);
        assert_eq!(quantized.channels[0].samples, vec![0.0, 0.0, 2.0, 3.0]);
        assert_eq!(
            quantized.metadata.transform_history,
            vec!["adc_quantize(bits=2,min_v=0,max_v=3)"]
        );
        let step = &quantized.metadata.transform_steps[0];
        assert_common_transform_metadata(step);
        assert_eq!(step.name, "adc_quantize");
        assert_eq!(step.category, TransformCategory::Quantization);
        assert_eq!(
            step.parameters[0].value,
            TransformParameterValue::Integer(2)
        );
        assert_eq!(step.parameters[0].unit.as_deref(), Some("bits"));
        assert_eq!(
            step.parameters[1].value,
            TransformParameterValue::Float(0.0)
        );
        assert_eq!(step.parameters[1].unit.as_deref(), Some("V"));
        assert_eq!(
            step.parameters[2].value,
            TransformParameterValue::Float(3.0)
        );
        assert_eq!(step.parameters[2].unit.as_deref(), Some("V"));
        assert!(!step.sample_rate_required);
        assert!(!step.stateful);
        assert_eq!(step.phase_effect, TransformPhaseEffect::None);
    }

    #[test]
    fn adc_quantizer_rejects_invalid_parameters() {
        for filter in [
            AdcQuantizer {
                bits: 0,
                min_v: 0.0,
                max_v: 3.0,
            },
            AdcQuantizer {
                bits: MAX_ADC_BITS + 1,
                min_v: 0.0,
                max_v: 3.0,
            },
            AdcQuantizer {
                bits: 2,
                min_v: 3.0,
                max_v: 3.0,
            },
        ] {
            let result = filter.apply(&waveform(vec![0.0, 1.0, 2.0, 3.0]));
            assert!(matches!(
                result,
                Err(WaveformError::InvalidParameter { .. })
            ));
        }
    }

    #[test]
    fn filter_chain_applies_steps_in_order() {
        let input = waveform(vec![0.0, 2.0, 4.0, 6.0]);
        let filters = vec![
            FilterStep::MovingAverage(MovingAverageFilter { window_samples: 2 }),
            FilterStep::AdcQuantize(AdcQuantizer {
                bits: 2,
                min_v: 0.0,
                max_v: 3.0,
            }),
        ];

        let derived = apply_filter_chain(&input, &filters).expect("filter chain should apply");

        assert_eq!(input.channels[0].samples, vec![0.0, 2.0, 4.0, 6.0]);
        assert_eq!(derived.channels[0].samples, vec![0.0, 1.0, 3.0, 3.0]);
        assert_eq!(
            derived.metadata.transform_history,
            vec![
                "moving_average(window_samples=2)",
                "adc_quantize(bits=2,min_v=0,max_v=3)"
            ]
        );
        assert_eq!(derived.metadata.transform_steps[0].sequence_index, 0);
        assert_eq!(derived.metadata.transform_steps[0].name, "moving_average");
        assert_eq!(derived.metadata.transform_steps[1].sequence_index, 1);
        assert_eq!(derived.metadata.transform_steps[1].name, "adc_quantize");
    }
}
