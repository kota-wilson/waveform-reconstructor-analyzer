use crate::error::{Result, WaveformError};
use crate::model::{
    TransformCategory, TransformExecutionMetadata, TransformOutputChannels,
    TransformParameterMetadata, TransformPhaseEffect, TransformStepMetadata, Waveform,
};
use serde::Serialize;
use std::f64::consts::PI;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FeatureRecord {
    pub id: String,
    pub transform: String,
    pub channel: String,
    pub value: f64,
    pub unit: String,
    #[serde(default, skip_serializing_if = "FeatureMethodContext::is_empty")]
    pub method_context: FeatureMethodContext,
    pub transform_metadata: TransformStepMetadata,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct FeatureMethodContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentile: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantile: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bins: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_samples: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_hz: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_frequency_hz: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_width_hz: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_samples: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlap_samples: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_start_s: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_end_s: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imaginary: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magnitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_rad: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harmonic_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fundamental_hz: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band_low_hz: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band_high_hz: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolloff_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalization: Option<String>,
}

impl FeatureMethodContext {
    fn is_empty(&self) -> bool {
        self.percentile.is_none()
            && self.quantile.is_none()
            && self.bins.is_none()
            && self.bin_index.is_none()
            && self.bin_min.is_none()
            && self.bin_max.is_none()
            && self.other_channel.is_none()
            && self.lag_samples.is_none()
            && self.frequency_hz.is_none()
            && self.bin_frequency_hz.is_none()
            && self.bin_width_hz.is_none()
            && self.window.is_none()
            && self.window_index.is_none()
            && self.window_samples.is_none()
            && self.overlap_samples.is_none()
            && self.sample_index.is_none()
            && self.segment_index.is_none()
            && self.segment_start_s.is_none()
            && self.segment_end_s.is_none()
            && self.real.is_none()
            && self.imaginary.is_none()
            && self.magnitude.is_none()
            && self.phase_rad.is_none()
            && self.harmonic_index.is_none()
            && self.fundamental_hz.is_none()
            && self.band_low_hz.is_none()
            && self.band_high_hz.is_none()
            && self.rolloff_percent.is_none()
            && self.normalization.is_none()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FeatureTransformStep {
    Rms(RmsFeatureTransform),
    PeakToPeak(PeakToPeakFeatureTransform),
    CrestFactor(CrestFactorFeatureTransform),
    Energy(EnergyFeatureTransform),
    Power(PowerFeatureTransform),
    AreaUnderCurve(AreaUnderCurveFeatureTransform),
    ImpulseEstimate(ImpulseEstimateFeatureTransform),
    Mean(MeanFeatureTransform),
    Median(MedianFeatureTransform),
    Mode(ModeFeatureTransform),
    Minimum(MinimumFeatureTransform),
    Maximum(MaximumFeatureTransform),
    Variance(VarianceFeatureTransform),
    StandardDeviation(StandardDeviationFeatureTransform),
    Skewness(SkewnessFeatureTransform),
    Kurtosis(KurtosisFeatureTransform),
    Percentile(PercentileFeatureTransform),
    Quantile(QuantileFeatureTransform),
    Histogram(HistogramFeatureTransform),
    Covariance(CovarianceFeatureTransform),
    Correlation(CorrelationFeatureTransform),
    Autocorrelation(AutocorrelationFeatureTransform),
    CrossCorrelation(CrossCorrelationFeatureTransform),
    WindowFunction(WindowFunctionFeatureTransform),
    Dft(SpectrumFeatureTransform),
    Fft(SpectrumFeatureTransform),
    Ifft(IfftFeatureTransform),
    PowerSpectrum(SpectrumFeatureTransform),
    Psd(SpectrumFeatureTransform),
    WelchPsd(WelchPsdFeatureTransform),
    CrossSpectrum(PairedSpectrumFeatureTransform),
    Coherence(PairedSpectrumFeatureTransform),
    TransferFunction(PairedSpectrumFeatureTransform),
    HarmonicAnalysis(HarmonicFeatureTransform),
    Thd(HarmonicMetricFeatureTransform),
    Snr(HarmonicMetricFeatureTransform),
    Sinad(HarmonicMetricFeatureTransform),
    Enob(HarmonicMetricFeatureTransform),
    Stft(TimeFrequencyFeatureTransform),
    Spectrogram(TimeFrequencyFeatureTransform),
    SpectralCentroid(SpectrumFeatureTransform),
    SpectralBandwidth(SpectrumFeatureTransform),
    SpectralRolloff(SpectralRolloffFeatureTransform),
    BandPower(BandPowerFeatureTransform),
}

impl FeatureTransformStep {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Rms(transform) => transform.name(),
            Self::PeakToPeak(transform) => transform.name(),
            Self::CrestFactor(transform) => transform.name(),
            Self::Energy(transform) => transform.name(),
            Self::Power(transform) => transform.name(),
            Self::AreaUnderCurve(transform) => transform.name(),
            Self::ImpulseEstimate(transform) => transform.name(),
            Self::Mean(transform) => transform.name(),
            Self::Median(transform) => transform.name(),
            Self::Mode(transform) => transform.name(),
            Self::Minimum(transform) => transform.name(),
            Self::Maximum(transform) => transform.name(),
            Self::Variance(transform) => transform.name(),
            Self::StandardDeviation(transform) => transform.name(),
            Self::Skewness(transform) => transform.name(),
            Self::Kurtosis(transform) => transform.name(),
            Self::Percentile(transform) => transform.name(),
            Self::Quantile(transform) => transform.name(),
            Self::Histogram(transform) => transform.name(),
            Self::Covariance(transform) => transform.name(),
            Self::Correlation(transform) => transform.name(),
            Self::Autocorrelation(transform) => transform.name(),
            Self::CrossCorrelation(transform) => transform.name(),
            Self::WindowFunction(transform) => transform.name(),
            Self::Dft(transform) => transform.name(),
            Self::Fft(transform) => transform.name(),
            Self::Ifft(transform) => transform.name(),
            Self::PowerSpectrum(transform) => transform.name(),
            Self::Psd(transform) => transform.name(),
            Self::WelchPsd(transform) => transform.name(),
            Self::CrossSpectrum(transform) => transform.name(),
            Self::Coherence(transform) => transform.name(),
            Self::TransferFunction(transform) => transform.name(),
            Self::HarmonicAnalysis(transform) => transform.name(),
            Self::Thd(transform) => transform.name(),
            Self::Snr(transform) => transform.name(),
            Self::Sinad(transform) => transform.name(),
            Self::Enob(transform) => transform.name(),
            Self::Stft(transform) => transform.name(),
            Self::Spectrogram(transform) => transform.name(),
            Self::SpectralCentroid(transform) => transform.name(),
            Self::SpectralBandwidth(transform) => transform.name(),
            Self::SpectralRolloff(transform) => transform.name(),
            Self::BandPower(transform) => transform.name(),
        }
    }

    pub fn evaluate(
        &self,
        waveform: &Waveform,
        sequence_index: usize,
    ) -> Result<Vec<FeatureRecord>> {
        match self {
            Self::Rms(transform) => transform.evaluate(waveform, sequence_index),
            Self::PeakToPeak(transform) => transform.evaluate(waveform, sequence_index),
            Self::CrestFactor(transform) => transform.evaluate(waveform, sequence_index),
            Self::Energy(transform) => transform.evaluate(waveform, sequence_index),
            Self::Power(transform) => transform.evaluate(waveform, sequence_index),
            Self::AreaUnderCurve(transform) => transform.evaluate(waveform, sequence_index),
            Self::ImpulseEstimate(transform) => transform.evaluate(waveform, sequence_index),
            Self::Mean(transform) => transform.evaluate(waveform, sequence_index),
            Self::Median(transform) => transform.evaluate(waveform, sequence_index),
            Self::Mode(transform) => transform.evaluate(waveform, sequence_index),
            Self::Minimum(transform) => transform.evaluate(waveform, sequence_index),
            Self::Maximum(transform) => transform.evaluate(waveform, sequence_index),
            Self::Variance(transform) => transform.evaluate(waveform, sequence_index),
            Self::StandardDeviation(transform) => transform.evaluate(waveform, sequence_index),
            Self::Skewness(transform) => transform.evaluate(waveform, sequence_index),
            Self::Kurtosis(transform) => transform.evaluate(waveform, sequence_index),
            Self::Percentile(transform) => transform.evaluate(waveform, sequence_index),
            Self::Quantile(transform) => transform.evaluate(waveform, sequence_index),
            Self::Histogram(transform) => transform.evaluate(waveform, sequence_index),
            Self::Covariance(transform) => transform.evaluate(waveform, sequence_index),
            Self::Correlation(transform) => transform.evaluate(waveform, sequence_index),
            Self::Autocorrelation(transform) => transform.evaluate(waveform, sequence_index),
            Self::CrossCorrelation(transform) => transform.evaluate(waveform, sequence_index),
            Self::WindowFunction(transform) => transform.evaluate(waveform, sequence_index),
            Self::Dft(transform) => transform.evaluate(waveform, sequence_index),
            Self::Fft(transform) => transform.evaluate(waveform, sequence_index),
            Self::Ifft(transform) => transform.evaluate(waveform, sequence_index),
            Self::PowerSpectrum(transform) => transform.evaluate(waveform, sequence_index),
            Self::Psd(transform) => transform.evaluate(waveform, sequence_index),
            Self::WelchPsd(transform) => transform.evaluate(waveform, sequence_index),
            Self::CrossSpectrum(transform) => transform.evaluate(waveform, sequence_index),
            Self::Coherence(transform) => transform.evaluate(waveform, sequence_index),
            Self::TransferFunction(transform) => transform.evaluate(waveform, sequence_index),
            Self::HarmonicAnalysis(transform) => transform.evaluate(waveform, sequence_index),
            Self::Thd(transform) => transform.evaluate(waveform, sequence_index),
            Self::Snr(transform) => transform.evaluate(waveform, sequence_index),
            Self::Sinad(transform) => transform.evaluate(waveform, sequence_index),
            Self::Enob(transform) => transform.evaluate(waveform, sequence_index),
            Self::Stft(transform) => transform.evaluate(waveform, sequence_index),
            Self::Spectrogram(transform) => transform.evaluate(waveform, sequence_index),
            Self::SpectralCentroid(transform) => transform.evaluate(waveform, sequence_index),
            Self::SpectralBandwidth(transform) => transform.evaluate(waveform, sequence_index),
            Self::SpectralRolloff(transform) => transform.evaluate(waveform, sequence_index),
            Self::BandPower(transform) => transform.evaluate(waveform, sequence_index),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RmsFeatureTransform {
    pub id: String,
    pub channel: String,
}

impl RmsFeatureTransform {
    fn name(&self) -> &'static str {
        "rms"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let channel = feature_channel(waveform, &self.channel)?;
        Ok(vec![feature_record(
            &self.id,
            self.name(),
            channel.name.clone(),
            rms_value(&channel.samples)?,
            channel.unit.name.clone(),
            sequence_index,
            &self.channel,
        )?])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PeakToPeakFeatureTransform {
    pub id: String,
    pub channel: String,
}

impl PeakToPeakFeatureTransform {
    fn name(&self) -> &'static str {
        "peak_to_peak"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let channel = feature_channel(waveform, &self.channel)?;
        Ok(vec![feature_record(
            &self.id,
            self.name(),
            channel.name.clone(),
            peak_to_peak_value(&channel.samples)?,
            channel.unit.name.clone(),
            sequence_index,
            &self.channel,
        )?])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrestFactorFeatureTransform {
    pub id: String,
    pub channel: String,
}

impl CrestFactorFeatureTransform {
    fn name(&self) -> &'static str {
        "crest_factor"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let channel = feature_channel(waveform, &self.channel)?;
        Ok(vec![feature_record(
            &self.id,
            self.name(),
            channel.name.clone(),
            crest_factor_value(&channel.samples)?,
            "ratio".to_string(),
            sequence_index,
            &self.channel,
        )?])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnergyFeatureTransform {
    pub id: String,
    pub channel: String,
}

impl EnergyFeatureTransform {
    fn name(&self) -> &'static str {
        "energy"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let channel = feature_channel(waveform, &self.channel)?;
        Ok(vec![feature_record(
            &self.id,
            self.name(),
            channel.name.clone(),
            energy_value(&waveform.time, &channel.samples)?,
            format!("{}^2*s", channel.unit.name),
            sequence_index,
            &self.channel,
        )?])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PowerFeatureTransform {
    pub id: String,
    pub channel: String,
}

impl PowerFeatureTransform {
    fn name(&self) -> &'static str {
        "power"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let channel = feature_channel(waveform, &self.channel)?;
        let duration = waveform_duration(&waveform.time)?;
        Ok(vec![feature_record(
            &self.id,
            self.name(),
            channel.name.clone(),
            energy_value(&waveform.time, &channel.samples)? / duration,
            format!("{}^2", channel.unit.name),
            sequence_index,
            &self.channel,
        )?])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AreaUnderCurveFeatureTransform {
    pub id: String,
    pub channel: String,
}

impl AreaUnderCurveFeatureTransform {
    fn name(&self) -> &'static str {
        "area_under_curve"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let channel = feature_channel(waveform, &self.channel)?;
        Ok(vec![feature_record(
            &self.id,
            self.name(),
            channel.name.clone(),
            trapezoidal_area(&waveform.time, &channel.samples)?,
            format!("{}*s", channel.unit.name),
            sequence_index,
            &self.channel,
        )?])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImpulseEstimateFeatureTransform {
    pub id: String,
    pub channel: String,
}

impl ImpulseEstimateFeatureTransform {
    fn name(&self) -> &'static str {
        "impulse_estimate"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let channel = feature_channel(waveform, &self.channel)?;
        Ok(vec![feature_record(
            &self.id,
            self.name(),
            channel.name.clone(),
            trapezoidal_area(&waveform.time, &channel.samples)?,
            format!("{}*s", channel.unit.name),
            sequence_index,
            &self.channel,
        )?])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeanFeatureTransform {
    pub id: String,
    pub channel: String,
}

impl MeanFeatureTransform {
    fn name(&self) -> &'static str {
        "mean"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let channel = feature_channel(waveform, &self.channel)?;
        m32_feature_record(
            &self.id,
            self.name(),
            &self.channel,
            channel.name.clone(),
            mean_value(&channel.samples)?,
            channel.unit.name.clone(),
            sequence_index,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MedianFeatureTransform {
    pub id: String,
    pub channel: String,
}

impl MedianFeatureTransform {
    fn name(&self) -> &'static str {
        "median"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let channel = feature_channel(waveform, &self.channel)?;
        m32_feature_record(
            &self.id,
            self.name(),
            &self.channel,
            channel.name.clone(),
            percentile_value(&channel.samples, 0.5)?,
            channel.unit.name.clone(),
            sequence_index,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModeFeatureTransform {
    pub id: String,
    pub channel: String,
}

impl ModeFeatureTransform {
    fn name(&self) -> &'static str {
        "mode"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let channel = feature_channel(waveform, &self.channel)?;
        m32_feature_record(
            &self.id,
            self.name(),
            &self.channel,
            channel.name.clone(),
            mode_value(&channel.samples)?,
            channel.unit.name.clone(),
            sequence_index,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MinimumFeatureTransform {
    pub id: String,
    pub channel: String,
}

impl MinimumFeatureTransform {
    fn name(&self) -> &'static str {
        "min"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let channel = feature_channel(waveform, &self.channel)?;
        let (min, _) = min_max_value(&channel.samples)?;
        m32_feature_record(
            &self.id,
            self.name(),
            &self.channel,
            channel.name.clone(),
            min,
            channel.unit.name.clone(),
            sequence_index,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MaximumFeatureTransform {
    pub id: String,
    pub channel: String,
}

impl MaximumFeatureTransform {
    fn name(&self) -> &'static str {
        "max"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let channel = feature_channel(waveform, &self.channel)?;
        let (_, max) = min_max_value(&channel.samples)?;
        m32_feature_record(
            &self.id,
            self.name(),
            &self.channel,
            channel.name.clone(),
            max,
            channel.unit.name.clone(),
            sequence_index,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarianceFeatureTransform {
    pub id: String,
    pub channel: String,
}

impl VarianceFeatureTransform {
    fn name(&self) -> &'static str {
        "variance"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let channel = feature_channel(waveform, &self.channel)?;
        m32_feature_record(
            &self.id,
            self.name(),
            &self.channel,
            channel.name.clone(),
            variance_value(&channel.samples)?,
            format!("{}^2", channel.unit.name),
            sequence_index,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StandardDeviationFeatureTransform {
    pub id: String,
    pub channel: String,
}

impl StandardDeviationFeatureTransform {
    fn name(&self) -> &'static str {
        "standard_deviation"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let channel = feature_channel(waveform, &self.channel)?;
        m32_feature_record(
            &self.id,
            self.name(),
            &self.channel,
            channel.name.clone(),
            variance_value(&channel.samples)?.sqrt(),
            channel.unit.name.clone(),
            sequence_index,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SkewnessFeatureTransform {
    pub id: String,
    pub channel: String,
}

impl SkewnessFeatureTransform {
    fn name(&self) -> &'static str {
        "skewness"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let channel = feature_channel(waveform, &self.channel)?;
        m32_feature_record(
            &self.id,
            self.name(),
            &self.channel,
            channel.name.clone(),
            skewness_value(&channel.samples)?,
            "ratio".to_string(),
            sequence_index,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KurtosisFeatureTransform {
    pub id: String,
    pub channel: String,
}

impl KurtosisFeatureTransform {
    fn name(&self) -> &'static str {
        "kurtosis"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let channel = feature_channel(waveform, &self.channel)?;
        m32_feature_record(
            &self.id,
            self.name(),
            &self.channel,
            channel.name.clone(),
            kurtosis_value(&channel.samples)?,
            "ratio".to_string(),
            sequence_index,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PercentileFeatureTransform {
    pub id: String,
    pub channel: String,
    pub percentile: f64,
}

impl PercentileFeatureTransform {
    fn name(&self) -> &'static str {
        "percentile"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        validate_percentile(self.percentile)?;
        let channel = feature_channel(waveform, &self.channel)?;
        let context = FeatureMethodContext {
            percentile: Some(self.percentile),
            ..FeatureMethodContext::default()
        };
        m32_feature_record_with_parameters(M32FeatureRecordFields {
            id: &self.id,
            transform_name: self.name(),
            channel_name: channel.name.clone(),
            value: percentile_value(&channel.samples, self.percentile / 100.0)?,
            unit: channel.unit.name.clone(),
            sequence_index,
            parameters: vec![
                TransformParameterMetadata::text("channel", self.channel.clone()),
                TransformParameterMetadata::float("percentile", self.percentile, "percent"),
            ],
            method_context: context,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuantileFeatureTransform {
    pub id: String,
    pub channel: String,
    pub quantile: f64,
}

impl QuantileFeatureTransform {
    fn name(&self) -> &'static str {
        "quantile"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        validate_quantile(self.quantile)?;
        let channel = feature_channel(waveform, &self.channel)?;
        let context = FeatureMethodContext {
            quantile: Some(self.quantile),
            ..FeatureMethodContext::default()
        };
        m32_feature_record_with_parameters(M32FeatureRecordFields {
            id: &self.id,
            transform_name: self.name(),
            channel_name: channel.name.clone(),
            value: percentile_value(&channel.samples, self.quantile)?,
            unit: channel.unit.name.clone(),
            sequence_index,
            parameters: vec![
                TransformParameterMetadata::text("channel", self.channel.clone()),
                TransformParameterMetadata::float("quantile", self.quantile, "ratio"),
            ],
            method_context: context,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HistogramFeatureTransform {
    pub id: String,
    pub channel: String,
    pub bins: usize,
    pub min_v: Option<f64>,
    pub max_v: Option<f64>,
}

impl HistogramFeatureTransform {
    fn name(&self) -> &'static str {
        "histogram"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        histogram_records(self, waveform, sequence_index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CovarianceFeatureTransform {
    pub id: String,
    pub channel: String,
    pub other_channel: String,
}

impl CovarianceFeatureTransform {
    fn name(&self) -> &'static str {
        "covariance"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let (channel, other) = feature_channel_pair(waveform, &self.channel, &self.other_channel)?;
        paired_feature_record(
            &self.id,
            self.name(),
            channel.name.clone(),
            &self.channel,
            &self.other_channel,
            covariance_value(&channel.samples, &other.samples)?,
            format!("{}*{}", channel.unit.name, other.unit.name),
            sequence_index,
            None,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CorrelationFeatureTransform {
    pub id: String,
    pub channel: String,
    pub other_channel: String,
}

impl CorrelationFeatureTransform {
    fn name(&self) -> &'static str {
        "correlation"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let (channel, other) = feature_channel_pair(waveform, &self.channel, &self.other_channel)?;
        paired_feature_record(
            &self.id,
            self.name(),
            channel.name.clone(),
            &self.channel,
            &self.other_channel,
            correlation_value(&channel.samples, &other.samples)?,
            "ratio".to_string(),
            sequence_index,
            None,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AutocorrelationFeatureTransform {
    pub id: String,
    pub channel: String,
    pub lag_samples: usize,
}

impl AutocorrelationFeatureTransform {
    fn name(&self) -> &'static str {
        "autocorrelation"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let channel = feature_channel(waveform, &self.channel)?;
        let value = lagged_correlation_value(&channel.samples, &channel.samples, self.lag_samples)?;
        m32_feature_record_with_parameters(M32FeatureRecordFields {
            id: &self.id,
            transform_name: self.name(),
            channel_name: channel.name.clone(),
            value,
            unit: "ratio".to_string(),
            sequence_index,
            parameters: vec![
                TransformParameterMetadata::text("channel", self.channel.clone()),
                TransformParameterMetadata::integer(
                    "lag_samples",
                    self.lag_samples as u64,
                    "samples",
                ),
            ],
            method_context: FeatureMethodContext {
                lag_samples: Some(self.lag_samples),
                ..FeatureMethodContext::default()
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrossCorrelationFeatureTransform {
    pub id: String,
    pub channel: String,
    pub other_channel: String,
    pub lag_samples: usize,
}

impl CrossCorrelationFeatureTransform {
    fn name(&self) -> &'static str {
        "cross_correlation"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        let (channel, other) = feature_channel_pair(waveform, &self.channel, &self.other_channel)?;
        paired_feature_record(
            &self.id,
            self.name(),
            channel.name.clone(),
            &self.channel,
            &self.other_channel,
            lagged_correlation_value(&channel.samples, &other.samples, self.lag_samples)?,
            "ratio".to_string(),
            sequence_index,
            Some(self.lag_samples),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WindowFunctionFeatureTransform {
    pub id: String,
    pub channel: String,
    pub window: WindowSpec,
    pub window_samples: Option<usize>,
}

impl WindowFunctionFeatureTransform {
    fn name(&self) -> &'static str {
        "window_function"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        window_function_records(self, waveform, sequence_index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpectrumFeatureTransform {
    pub id: String,
    pub transform_name: &'static str,
    pub channel: String,
    pub window: WindowSpec,
}

impl SpectrumFeatureTransform {
    fn name(&self) -> &'static str {
        self.transform_name
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        spectrum_feature_records(self, waveform, sequence_index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfftFeatureTransform {
    pub id: String,
    pub channel: String,
    pub other_channel: Option<String>,
}

impl IfftFeatureTransform {
    fn name(&self) -> &'static str {
        "ifft"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        ifft_feature_records(self, waveform, sequence_index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WelchPsdFeatureTransform {
    pub id: String,
    pub channel: String,
    pub window: WindowSpec,
    pub window_samples: usize,
    pub overlap_samples: usize,
}

impl WelchPsdFeatureTransform {
    fn name(&self) -> &'static str {
        "welch_psd"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        welch_psd_records(self, waveform, sequence_index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PairedSpectrumFeatureTransform {
    pub id: String,
    pub transform_name: &'static str,
    pub channel: String,
    pub other_channel: String,
    pub window: WindowSpec,
    pub window_samples: Option<usize>,
    pub overlap_samples: Option<usize>,
}

impl PairedSpectrumFeatureTransform {
    fn name(&self) -> &'static str {
        self.transform_name
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        paired_spectrum_records(self, waveform, sequence_index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HarmonicFeatureTransform {
    pub id: String,
    pub channel: String,
    pub window: WindowSpec,
    pub fundamental_hz: Option<f64>,
    pub harmonic_count: usize,
}

impl HarmonicFeatureTransform {
    fn name(&self) -> &'static str {
        "harmonic_analysis"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        harmonic_analysis_records(self, waveform, sequence_index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HarmonicMetricFeatureTransform {
    pub id: String,
    pub transform_name: &'static str,
    pub channel: String,
    pub window: WindowSpec,
    pub fundamental_hz: Option<f64>,
    pub harmonic_count: usize,
}

impl HarmonicMetricFeatureTransform {
    fn name(&self) -> &'static str {
        self.transform_name
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        harmonic_metric_record(self, waveform, sequence_index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimeFrequencyFeatureTransform {
    pub id: String,
    pub transform_name: &'static str,
    pub channel: String,
    pub window: WindowSpec,
    pub window_samples: usize,
    pub overlap_samples: usize,
}

impl TimeFrequencyFeatureTransform {
    fn name(&self) -> &'static str {
        self.transform_name
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        time_frequency_records(self, waveform, sequence_index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpectralRolloffFeatureTransform {
    pub id: String,
    pub channel: String,
    pub window: WindowSpec,
    pub rolloff_percent: f64,
}

impl SpectralRolloffFeatureTransform {
    fn name(&self) -> &'static str {
        "spectral_rolloff"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        spectral_rolloff_record(self, waveform, sequence_index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BandPowerFeatureTransform {
    pub id: String,
    pub channel: String,
    pub window: WindowSpec,
    pub band_low_hz: f64,
    pub band_high_hz: f64,
}

impl BandPowerFeatureTransform {
    fn name(&self) -> &'static str {
        "band_power"
    }

    fn evaluate(&self, waveform: &Waveform, sequence_index: usize) -> Result<Vec<FeatureRecord>> {
        band_power_record(self, waveform, sequence_index)
    }
}

pub fn evaluate_feature_transforms(
    waveform: &Waveform,
    transforms: &[FeatureTransformStep],
) -> Result<Vec<FeatureRecord>> {
    let mut records = Vec::new();
    for (index, transform) in transforms.iter().enumerate() {
        records.extend(transform.evaluate(waveform, index)?);
    }
    Ok(records)
}

fn feature_record(
    id: &str,
    transform_name: &'static str,
    channel: String,
    value: f64,
    unit: String,
    sequence_index: usize,
    channel_parameter: &str,
) -> Result<FeatureRecord> {
    feature_record_with_context(FeatureRecordFields {
        id,
        transform_name,
        category: TransformCategory::Feature,
        channel_name: channel,
        value,
        unit,
        sequence_index,
        parameters: vec![TransformParameterMetadata::text(
            "channel",
            channel_parameter.to_string(),
        )],
        execution: feature_execution(transform_name),
        method_context: FeatureMethodContext::default(),
    })
}

struct FeatureRecordFields<'a> {
    id: &'a str,
    transform_name: &'static str,
    category: TransformCategory,
    channel_name: String,
    value: f64,
    unit: String,
    sequence_index: usize,
    parameters: Vec<TransformParameterMetadata>,
    execution: TransformExecutionMetadata,
    method_context: FeatureMethodContext,
}

fn feature_record_with_context(fields: FeatureRecordFields<'_>) -> Result<FeatureRecord> {
    validate_finite_parameter(fields.transform_name, fields.value)?;
    let mut transform_metadata = TransformStepMetadata::implemented_desktop_with_execution(
        feature_history_label(fields.transform_name, &fields.parameters),
        fields.transform_name,
        fields.category,
        fields.parameters,
        fields.execution,
    );
    transform_metadata.sequence_index = fields.sequence_index;
    transform_metadata.output_channels = TransformOutputChannels::feature_records();

    Ok(FeatureRecord {
        id: fields.id.to_string(),
        transform: fields.transform_name.to_string(),
        channel: fields.channel_name,
        value: fields.value,
        unit: fields.unit,
        method_context: fields.method_context,
        transform_metadata,
    })
}

fn feature_history_label(
    transform_name: &'static str,
    parameters: &[TransformParameterMetadata],
) -> String {
    let rendered = parameters
        .iter()
        .map(|parameter| match &parameter.value {
            crate::model::TransformParameterValue::Float(value) => {
                format!("{}={}", parameter.name, value)
            }
            crate::model::TransformParameterValue::Integer(value) => {
                format!("{}={}", parameter.name, value)
            }
            crate::model::TransformParameterValue::Bool(value) => {
                format!("{}={}", parameter.name, value)
            }
            crate::model::TransformParameterValue::Text(value) => {
                format!("{}={}", parameter.name, value)
            }
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("{transform_name}({rendered})")
}

fn feature_execution(transform_name: &'static str) -> TransformExecutionMetadata {
    TransformExecutionMetadata {
        sample_rate_required: transform_name != "rms"
            && transform_name != "peak_to_peak"
            && transform_name != "crest_factor",
        stateful: false,
        causal: false,
        phase_effect: TransformPhaseEffect::None,
        streaming_supported: transform_name == "rms"
            || transform_name == "peak_to_peak"
            || transform_name == "crest_factor",
        offline_only: transform_name != "rms"
            && transform_name != "peak_to_peak"
            && transform_name != "crest_factor",
    }
}

fn offline_feature_execution() -> TransformExecutionMetadata {
    TransformExecutionMetadata {
        sample_rate_required: false,
        stateful: false,
        causal: false,
        phase_effect: TransformPhaseEffect::None,
        streaming_supported: false,
        offline_only: true,
    }
}

fn m32_feature_record(
    id: &str,
    transform_name: &'static str,
    channel_parameter: &str,
    channel_name: String,
    value: f64,
    unit: String,
    sequence_index: usize,
) -> Result<Vec<FeatureRecord>> {
    m32_feature_record_with_parameters(M32FeatureRecordFields {
        id,
        transform_name,
        channel_name,
        value,
        unit,
        sequence_index,
        parameters: vec![TransformParameterMetadata::text(
            "channel",
            channel_parameter.to_string(),
        )],
        method_context: FeatureMethodContext::default(),
    })
}

struct M32FeatureRecordFields<'a> {
    id: &'a str,
    transform_name: &'static str,
    channel_name: String,
    value: f64,
    unit: String,
    sequence_index: usize,
    parameters: Vec<TransformParameterMetadata>,
    method_context: FeatureMethodContext,
}

fn m32_feature_record_with_parameters(
    fields: M32FeatureRecordFields<'_>,
) -> Result<Vec<FeatureRecord>> {
    Ok(vec![feature_record_with_context(FeatureRecordFields {
        id: fields.id,
        transform_name: fields.transform_name,
        category: TransformCategory::Feature,
        channel_name: fields.channel_name,
        value: fields.value,
        unit: fields.unit,
        sequence_index: fields.sequence_index,
        parameters: fields.parameters,
        execution: offline_feature_execution(),
        method_context: fields.method_context,
    })?])
}

#[allow(clippy::too_many_arguments)]
fn paired_feature_record(
    id: &str,
    transform_name: &'static str,
    channel_name: String,
    channel_parameter: &str,
    other_channel_parameter: &str,
    value: f64,
    unit: String,
    sequence_index: usize,
    lag_samples: Option<usize>,
) -> Result<Vec<FeatureRecord>> {
    let mut parameters = vec![
        TransformParameterMetadata::text("channel", channel_parameter.to_string()),
        TransformParameterMetadata::text("other_channel", other_channel_parameter.to_string()),
    ];
    if let Some(lag_samples) = lag_samples {
        parameters.push(TransformParameterMetadata::integer(
            "lag_samples",
            lag_samples as u64,
            "samples",
        ));
    }

    m32_feature_record_with_parameters(M32FeatureRecordFields {
        id,
        transform_name,
        channel_name,
        value,
        unit,
        sequence_index,
        parameters,
        method_context: FeatureMethodContext {
            other_channel: Some(other_channel_parameter.to_string()),
            lag_samples,
            ..FeatureMethodContext::default()
        },
    })
}

fn histogram_records(
    transform: &HistogramFeatureTransform,
    waveform: &Waveform,
    sequence_index: usize,
) -> Result<Vec<FeatureRecord>> {
    validate_positive_usize("bins", transform.bins)?;
    let channel = feature_channel(waveform, &transform.channel)?;
    validate_finite_samples("histogram", &channel.samples)?;
    let (data_min, data_max) = min_max_value(&channel.samples)?;
    let min = transform.min_v.unwrap_or(data_min);
    let max = transform.max_v.unwrap_or(data_max);
    validate_finite_parameter("min_v", min)?;
    validate_finite_parameter("max_v", max)?;
    if max <= min {
        return Err(WaveformError::InvalidParameter {
            name: "feature_transforms.max_v".to_string(),
            reason: "must be greater than min_v".to_string(),
        });
    }

    let mut counts = vec![0_usize; transform.bins];
    let width = (max - min) / transform.bins as f64;
    for sample in &channel.samples {
        if *sample < min || *sample > max {
            continue;
        }
        let mut bin_index = ((*sample - min) / width).floor() as usize;
        if bin_index >= transform.bins {
            bin_index = transform.bins - 1;
        }
        counts[bin_index] += 1;
    }

    let mut records = Vec::with_capacity(transform.bins);
    for (bin_index, count) in counts.into_iter().enumerate() {
        let bin_min = min + width * bin_index as f64;
        let bin_max = bin_min + width;
        records.push(feature_record_with_context(FeatureRecordFields {
            id: &format!("{}_bin_{bin_index}", transform.id),
            transform_name: transform.name(),
            category: TransformCategory::Feature,
            channel_name: channel.name.clone(),
            value: count as f64,
            unit: "count".to_string(),
            sequence_index,
            parameters: vec![
                TransformParameterMetadata::text("channel", transform.channel.clone()),
                TransformParameterMetadata::integer("bins", transform.bins as u64, "bins"),
                TransformParameterMetadata::float("min_v", min, &channel.unit.name),
                TransformParameterMetadata::float("max_v", max, &channel.unit.name),
            ],
            execution: offline_feature_execution(),
            method_context: FeatureMethodContext {
                bins: Some(transform.bins),
                bin_index: Some(bin_index),
                bin_min: Some(bin_min),
                bin_max: Some(bin_max),
                ..FeatureMethodContext::default()
            },
        })?);
    }
    Ok(records)
}

fn feature_channel<'a>(
    waveform: &'a Waveform,
    channel_name: &str,
) -> Result<&'a crate::model::Channel> {
    waveform
        .channel(channel_name)
        .ok_or_else(|| WaveformError::InvalidParameter {
            name: "feature_transforms.channel".to_string(),
            reason: format!("unknown channel `{channel_name}`"),
        })
}

fn feature_channel_pair<'a>(
    waveform: &'a Waveform,
    channel_name: &str,
    other_channel_name: &str,
) -> Result<(&'a crate::model::Channel, &'a crate::model::Channel)> {
    let channel = feature_channel(waveform, channel_name)?;
    let other = feature_channel(waveform, other_channel_name)?;
    Ok((channel, other))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowKind {
    Rectangular,
    Hann,
    Hamming,
    Blackman,
    BlackmanHarris,
    FlatTop,
    Kaiser,
    Tukey,
    Bartlett,
    Gaussian,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowSpec {
    kind: WindowKind,
    beta: f64,
    alpha: f64,
    sigma: f64,
}

impl Default for WindowSpec {
    fn default() -> Self {
        Self {
            kind: WindowKind::Rectangular,
            beta: 8.6,
            alpha: 0.5,
            sigma: 0.4,
        }
    }
}

impl WindowSpec {
    pub fn from_config(
        window: Option<&str>,
        beta: Option<f64>,
        alpha: Option<f64>,
        sigma: Option<f64>,
    ) -> Result<Self> {
        let kind = match window.unwrap_or("rectangular") {
            "rectangular" => WindowKind::Rectangular,
            "hann" => WindowKind::Hann,
            "hamming" => WindowKind::Hamming,
            "blackman" => WindowKind::Blackman,
            "blackman_harris" => WindowKind::BlackmanHarris,
            "flat_top" => WindowKind::FlatTop,
            "kaiser" => WindowKind::Kaiser,
            "tukey" => WindowKind::Tukey,
            "bartlett" => WindowKind::Bartlett,
            "gaussian" => WindowKind::Gaussian,
            other => {
                return Err(WaveformError::InvalidParameter {
                    name: "feature_transforms.window".to_string(),
                    reason: format!("unsupported window `{other}`"),
                });
            }
        };
        let spec = Self {
            kind,
            beta: beta.unwrap_or(8.6),
            alpha: alpha.unwrap_or(0.5),
            sigma: sigma.unwrap_or(0.4),
        };
        validate_finite_parameter("window_beta", spec.beta)?;
        validate_finite_parameter("window_alpha", spec.alpha)?;
        validate_finite_parameter("window_sigma", spec.sigma)?;
        if spec.beta < 0.0 {
            return Err(WaveformError::InvalidParameter {
                name: "feature_transforms.window_beta".to_string(),
                reason: "must be non-negative".to_string(),
            });
        }
        if !(0.0..=1.0).contains(&spec.alpha) {
            return Err(WaveformError::InvalidParameter {
                name: "feature_transforms.window_alpha".to_string(),
                reason: "must be between zero and one".to_string(),
            });
        }
        if spec.sigma <= 0.0 {
            return Err(WaveformError::InvalidParameter {
                name: "feature_transforms.window_sigma".to_string(),
                reason: "must be greater than zero".to_string(),
            });
        }
        Ok(spec)
    }

    fn name(self) -> &'static str {
        match self.kind {
            WindowKind::Rectangular => "rectangular",
            WindowKind::Hann => "hann",
            WindowKind::Hamming => "hamming",
            WindowKind::Blackman => "blackman",
            WindowKind::BlackmanHarris => "blackman_harris",
            WindowKind::FlatTop => "flat_top",
            WindowKind::Kaiser => "kaiser",
            WindowKind::Tukey => "tukey",
            WindowKind::Bartlett => "bartlett",
            WindowKind::Gaussian => "gaussian",
        }
    }

    fn parameters(self) -> Vec<TransformParameterMetadata> {
        let mut parameters = vec![TransformParameterMetadata::text("window", self.name())];
        match self.kind {
            WindowKind::Kaiser => {
                parameters.push(TransformParameterMetadata::float(
                    "window_beta",
                    self.beta,
                    "ratio",
                ));
            }
            WindowKind::Tukey => {
                parameters.push(TransformParameterMetadata::float(
                    "window_alpha",
                    self.alpha,
                    "ratio",
                ));
            }
            WindowKind::Gaussian => {
                parameters.push(TransformParameterMetadata::float(
                    "window_sigma",
                    self.sigma,
                    "ratio",
                ));
            }
            _ => {}
        }
        parameters
    }

    fn coefficients(self, len: usize) -> Result<Vec<f64>> {
        validate_positive_usize("window_samples", len)?;
        if len == 1 {
            return Ok(vec![1.0]);
        }
        let denom = (len - 1) as f64;
        let midpoint = denom / 2.0;
        let coefficients = (0..len)
            .map(|index| {
                let n = index as f64;
                match self.kind {
                    WindowKind::Rectangular => 1.0,
                    WindowKind::Hann => 0.5 - 0.5 * (2.0 * PI * n / denom).cos(),
                    WindowKind::Hamming => 0.54 - 0.46 * (2.0 * PI * n / denom).cos(),
                    WindowKind::Blackman => {
                        0.42 - 0.5 * (2.0 * PI * n / denom).cos()
                            + 0.08 * (4.0 * PI * n / denom).cos()
                    }
                    WindowKind::BlackmanHarris => {
                        0.35875 - 0.48829 * (2.0 * PI * n / denom).cos()
                            + 0.14128 * (4.0 * PI * n / denom).cos()
                            - 0.01168 * (6.0 * PI * n / denom).cos()
                    }
                    WindowKind::FlatTop => {
                        0.215_578_95 - 0.416_631_58 * (2.0 * PI * n / denom).cos()
                            + 0.277_263_158 * (4.0 * PI * n / denom).cos()
                            - 0.083_578_947 * (6.0 * PI * n / denom).cos()
                            + 0.006_947_368 * (8.0 * PI * n / denom).cos()
                    }
                    WindowKind::Kaiser => {
                        let ratio = (2.0 * n / denom) - 1.0;
                        bessel_i0(self.beta * (1.0 - ratio * ratio).max(0.0).sqrt())
                            / bessel_i0(self.beta)
                    }
                    WindowKind::Tukey => tukey_coefficient(n, denom, self.alpha),
                    WindowKind::Bartlett => 1.0 - ((n - midpoint) / midpoint).abs(),
                    WindowKind::Gaussian => {
                        let x = (n - midpoint) / (self.sigma * midpoint);
                        (-0.5 * x * x).exp()
                    }
                }
            })
            .collect();
        Ok(coefficients)
    }
}

fn tukey_coefficient(n: f64, denom: f64, alpha: f64) -> f64 {
    if alpha <= f64::EPSILON {
        return 1.0;
    }
    if alpha >= 1.0 {
        return 0.5 - 0.5 * (2.0 * PI * n / denom).cos();
    }
    let edge = alpha * denom / 2.0;
    if n < edge {
        0.5 * (1.0 + (PI * (2.0 * n / (alpha * denom) - 1.0)).cos())
    } else if n <= denom * (1.0 - alpha / 2.0) {
        1.0
    } else {
        0.5 * (1.0 + (PI * (2.0 * n / (alpha * denom) - 2.0 / alpha + 1.0)).cos())
    }
}

fn bessel_i0(value: f64) -> f64 {
    let mut sum = 1.0;
    let mut term = 1.0;
    let scaled = value * value / 4.0;
    for order in 1..=24 {
        let order_f = order as f64;
        term *= scaled / (order_f * order_f);
        sum += term;
        if term.abs() < 1.0e-14 {
            break;
        }
    }
    sum
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ComplexValue {
    real: f64,
    imaginary: f64,
}

impl ComplexValue {
    const ZERO: Self = Self {
        real: 0.0,
        imaginary: 0.0,
    };

    fn new(real: f64, imaginary: f64) -> Self {
        Self { real, imaginary }
    }

    fn from_polar(magnitude: f64, phase: f64) -> Self {
        Self {
            real: magnitude * phase.cos(),
            imaginary: magnitude * phase.sin(),
        }
    }

    fn conjugate(self) -> Self {
        Self::new(self.real, -self.imaginary)
    }

    fn magnitude(self) -> f64 {
        self.real.hypot(self.imaginary)
    }

    fn phase(self) -> f64 {
        self.imaginary.atan2(self.real)
    }
}

impl std::ops::Add for ComplexValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.real + rhs.real, self.imaginary + rhs.imaginary)
    }
}

impl std::ops::Sub for ComplexValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.real - rhs.real, self.imaginary - rhs.imaginary)
    }
}

impl std::ops::Mul for ComplexValue {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.real * rhs.real - self.imaginary * rhs.imaginary,
            self.real * rhs.imaginary + self.imaginary * rhs.real,
        )
    }
}

impl std::ops::Div<f64> for ComplexValue {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.real / rhs, self.imaginary / rhs)
    }
}

#[derive(Debug, Clone, Copy)]
struct SpectrumBin {
    index: usize,
    frequency_hz: f64,
    value: ComplexValue,
    amplitude: f64,
    power: f64,
    psd: f64,
}

#[derive(Debug, Clone)]
struct SpectrumAnalysis {
    bins: Vec<SpectrumBin>,
    sample_rate_hz: f64,
    bin_width_hz: f64,
    window: WindowSpec,
    window_samples: usize,
}

fn sample_rate_hz(waveform: &Waveform, transform_name: &str) -> Result<f64> {
    let sample_rate =
        waveform
            .metadata
            .nominal_sample_rate_hz
            .ok_or_else(|| WaveformError::InvalidWaveform {
                reason: format!("{transform_name} requires a nominal sample rate"),
            })?;
    validate_positive_parameter("sample_rate_hz", sample_rate)?;
    Ok(sample_rate)
}

fn spectrum_analysis(
    samples: &[f64],
    sample_rate_hz: f64,
    window: WindowSpec,
) -> Result<SpectrumAnalysis> {
    validate_finite_samples("spectrum", samples)?;
    if samples.len() < 2 {
        return Err(WaveformError::InvalidWaveform {
            reason: "spectrum analysis requires at least two samples".to_string(),
        });
    }
    let coefficients = window.coefficients(samples.len())?;
    let windowed = samples
        .iter()
        .zip(coefficients.iter())
        .map(|(sample, coefficient)| sample * coefficient)
        .collect::<Vec<_>>();
    let coherent_gain = coefficients.iter().sum::<f64>() / coefficients.len() as f64;
    if coherent_gain.abs() <= f64::EPSILON {
        return Err(WaveformError::InvalidWaveform {
            reason: "window coherent gain must be non-zero".to_string(),
        });
    }
    let window_power = coefficients
        .iter()
        .map(|coefficient| coefficient * coefficient)
        .sum::<f64>();
    if window_power <= f64::EPSILON {
        return Err(WaveformError::InvalidWaveform {
            reason: "window power must be non-zero".to_string(),
        });
    }
    let spectrum = forward_transform_real(&windowed);
    let len = samples.len();
    let bin_width_hz = sample_rate_hz / len as f64;
    let nyquist_index = len / 2;
    let bins = (0..=nyquist_index)
        .map(|index| {
            let raw = spectrum[index];
            let interior = index != 0 && !(len % 2 == 0 && index == nyquist_index);
            let one_sided = if interior { 2.0 } else { 1.0 };
            let amplitude = one_sided * raw.magnitude() / (len as f64 * coherent_gain);
            let power = if interior {
                amplitude * amplitude / 2.0
            } else {
                amplitude * amplitude
            };
            let psd = one_sided * raw.magnitude().powi(2) / (sample_rate_hz * window_power);
            SpectrumBin {
                index,
                frequency_hz: index as f64 * bin_width_hz,
                value: raw,
                amplitude,
                power,
                psd,
            }
        })
        .collect();
    Ok(SpectrumAnalysis {
        bins,
        sample_rate_hz,
        bin_width_hz,
        window,
        window_samples: len,
    })
}

fn forward_transform_real(samples: &[f64]) -> Vec<ComplexValue> {
    let complex = samples
        .iter()
        .map(|sample| ComplexValue::new(*sample, 0.0))
        .collect::<Vec<_>>();
    forward_transform_complex(&complex)
}

fn forward_transform_complex(samples: &[ComplexValue]) -> Vec<ComplexValue> {
    if samples.len().is_power_of_two() {
        fft_radix2(samples, false)
    } else {
        dft_complex(samples, false)
    }
}

fn inverse_transform_complex(samples: &[ComplexValue]) -> Vec<ComplexValue> {
    if samples.len().is_power_of_two() {
        fft_radix2(samples, true)
    } else {
        dft_complex(samples, true)
    }
}

fn fft_radix2(samples: &[ComplexValue], inverse: bool) -> Vec<ComplexValue> {
    let len = samples.len();
    let mut output = samples.to_vec();
    let mut j = 0_usize;
    for i in 1..len {
        let mut bit = len >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            output.swap(i, j);
        }
    }

    let direction = if inverse { 1.0 } else { -1.0 };
    let mut size = 2;
    while size <= len {
        let half = size / 2;
        let angle = direction * 2.0 * PI / size as f64;
        let phase_step = ComplexValue::from_polar(1.0, angle);
        for start in (0..len).step_by(size) {
            let mut phase = ComplexValue::new(1.0, 0.0);
            for offset in 0..half {
                let even = output[start + offset];
                let odd = output[start + offset + half] * phase;
                output[start + offset] = even + odd;
                output[start + offset + half] = even - odd;
                phase = phase * phase_step;
            }
        }
        size <<= 1;
    }
    if inverse {
        for value in &mut output {
            *value = *value / len as f64;
        }
    }
    output
}

fn dft_complex(samples: &[ComplexValue], inverse: bool) -> Vec<ComplexValue> {
    let len = samples.len();
    let direction = if inverse { 1.0 } else { -1.0 };
    let scale = if inverse { len as f64 } else { 1.0 };
    (0..len)
        .map(|bin| {
            let mut sum = ComplexValue::ZERO;
            for (index, sample) in samples.iter().enumerate() {
                let angle = direction * 2.0 * PI * bin as f64 * index as f64 / len as f64;
                sum = sum + (*sample * ComplexValue::from_polar(1.0, angle));
            }
            sum / scale
        })
        .collect()
}

fn window_function_records(
    transform: &WindowFunctionFeatureTransform,
    waveform: &Waveform,
    sequence_index: usize,
) -> Result<Vec<FeatureRecord>> {
    let channel = feature_channel(waveform, &transform.channel)?;
    let window_samples = transform.window_samples.unwrap_or(channel.samples.len());
    let coefficients = transform.window.coefficients(window_samples)?;
    let mut records = Vec::with_capacity(coefficients.len());
    for (index, coefficient) in coefficients.into_iter().enumerate() {
        let mut parameters =
            feature_channel_window_parameters(&transform.channel, transform.window);
        parameters.push(TransformParameterMetadata::integer(
            "window_samples",
            window_samples as u64,
            "samples",
        ));
        records.push(feature_record_with_context(FeatureRecordFields {
            id: &format!("{}_sample_{index}", transform.id),
            transform_name: transform.name(),
            category: TransformCategory::Feature,
            channel_name: channel.name.clone(),
            value: coefficient,
            unit: "ratio".to_string(),
            sequence_index,
            parameters,
            execution: offline_spectral_execution(false, false),
            method_context: FeatureMethodContext {
                window: Some(transform.window.name().to_string()),
                window_index: Some(index),
                sample_index: Some(index),
                window_samples: Some(window_samples),
                normalization: Some("coefficient".to_string()),
                ..FeatureMethodContext::default()
            },
        })?);
    }
    Ok(records)
}

fn spectrum_feature_records(
    transform: &SpectrumFeatureTransform,
    waveform: &Waveform,
    sequence_index: usize,
) -> Result<Vec<FeatureRecord>> {
    let channel = feature_channel(waveform, &transform.channel)?;
    let sample_rate = sample_rate_hz(waveform, transform.name())?;
    let analysis = spectrum_analysis(&channel.samples, sample_rate, transform.window)?;
    match transform.name() {
        "dft" | "fft" => spectrum_bin_records(SpectrumBinRecordRequest {
            id: &transform.id,
            transform_name: transform.name(),
            channel_parameter: &transform.channel,
            channel_name: channel.name.clone(),
            unit: channel.unit.name.clone(),
            sequence_index,
            analysis: &analysis,
            value: SpectrumBinValue::Amplitude,
            normalization: "one_sided_peak_amplitude",
        }),
        "power_spectrum" => spectrum_bin_records(SpectrumBinRecordRequest {
            id: &transform.id,
            transform_name: transform.name(),
            channel_parameter: &transform.channel,
            channel_name: channel.name.clone(),
            unit: format!("{}^2", channel.unit.name),
            sequence_index,
            analysis: &analysis,
            value: SpectrumBinValue::Power,
            normalization: "one_sided_power",
        }),
        "psd" => spectrum_bin_records(SpectrumBinRecordRequest {
            id: &transform.id,
            transform_name: transform.name(),
            channel_parameter: &transform.channel,
            channel_name: channel.name.clone(),
            unit: format!("{}^2/Hz", channel.unit.name),
            sequence_index,
            analysis: &analysis,
            value: SpectrumBinValue::Psd,
            normalization: "one_sided_power_spectral_density",
        }),
        "spectral_centroid" => {
            let centroid = spectral_centroid(&analysis)?;
            scalar_spectrum_record(ScalarSpectrumRecordRequest {
                id: &transform.id,
                transform_name: transform.name(),
                channel_parameter: &transform.channel,
                channel_name: channel.name.clone(),
                value: centroid,
                unit: "Hz".to_string(),
                sequence_index,
                analysis: &analysis,
                normalization: "power_weighted_frequency",
                extra_context: FeatureMethodContext::default(),
            })
            .map(|record| vec![record])
        }
        "spectral_bandwidth" => {
            let centroid = spectral_centroid(&analysis)?;
            let bandwidth = spectral_bandwidth(&analysis, centroid)?;
            scalar_spectrum_record(ScalarSpectrumRecordRequest {
                id: &transform.id,
                transform_name: transform.name(),
                channel_parameter: &transform.channel,
                channel_name: channel.name.clone(),
                value: bandwidth,
                unit: "Hz".to_string(),
                sequence_index,
                analysis: &analysis,
                normalization: "power_weighted_standard_deviation",
                extra_context: FeatureMethodContext {
                    frequency_hz: Some(centroid),
                    ..FeatureMethodContext::default()
                },
            })
            .map(|record| vec![record])
        }
        other => Err(WaveformError::InvalidParameter {
            name: "feature_transforms.type".to_string(),
            reason: format!("unsupported spectrum feature transform `{other}`"),
        }),
    }
}

fn ifft_feature_records(
    transform: &IfftFeatureTransform,
    waveform: &Waveform,
    sequence_index: usize,
) -> Result<Vec<FeatureRecord>> {
    let real_channel = feature_channel(waveform, &transform.channel)?;
    let imaginary_channel = match &transform.other_channel {
        Some(channel) => Some(feature_channel(waveform, channel)?),
        None => None,
    };
    let complex_samples = match imaginary_channel {
        Some(imaginary) => {
            validate_same_length("ifft", &real_channel.samples, &imaginary.samples)?;
            real_channel
                .samples
                .iter()
                .zip(imaginary.samples.iter())
                .map(|(real, imaginary)| ComplexValue::new(*real, *imaginary))
                .collect::<Vec<_>>()
        }
        None => {
            validate_finite_samples("ifft", &real_channel.samples)?;
            real_channel
                .samples
                .iter()
                .map(|real| ComplexValue::new(*real, 0.0))
                .collect::<Vec<_>>()
        }
    };
    if complex_samples.len() < 2 {
        return Err(WaveformError::InvalidWaveform {
            reason: "ifft requires at least two frequency bins".to_string(),
        });
    }
    let reconstructed = inverse_transform_complex(&complex_samples);
    let mut records = Vec::with_capacity(reconstructed.len());
    for (index, value) in reconstructed.iter().enumerate() {
        let mut parameters = vec![TransformParameterMetadata::text(
            "channel",
            transform.channel.clone(),
        )];
        if let Some(other_channel) = &transform.other_channel {
            parameters.push(TransformParameterMetadata::text(
                "other_channel",
                other_channel.clone(),
            ));
        }
        records.push(feature_record_with_context(FeatureRecordFields {
            id: &format!("{}_sample_{index}", transform.id),
            transform_name: transform.name(),
            category: TransformCategory::FrequencyFilter,
            channel_name: real_channel.name.clone(),
            value: value.real,
            unit: real_channel.unit.name.clone(),
            sequence_index,
            parameters,
            execution: offline_spectral_execution(false, false),
            method_context: FeatureMethodContext {
                sample_index: Some(index),
                real: Some(value.real),
                imaginary: Some(value.imaginary),
                normalization: Some("inverse_transform_divides_by_n".to_string()),
                ..FeatureMethodContext::default()
            },
        })?);
    }
    Ok(records)
}

fn welch_psd_records(
    transform: &WelchPsdFeatureTransform,
    waveform: &Waveform,
    sequence_index: usize,
) -> Result<Vec<FeatureRecord>> {
    let channel = feature_channel(waveform, &transform.channel)?;
    let sample_rate = sample_rate_hz(waveform, transform.name())?;
    let segments = segment_ranges(
        channel.samples.len(),
        transform.window_samples,
        transform.overlap_samples,
    )?;
    let mut accum = Vec::<SpectrumBin>::new();
    for (segment_index, (start, end)) in segments.iter().copied().enumerate() {
        let analysis =
            spectrum_analysis(&channel.samples[start..end], sample_rate, transform.window)?;
        if segment_index == 0 {
            accum = analysis.bins;
        } else {
            for (target, source) in accum.iter_mut().zip(analysis.bins.iter()) {
                target.psd += source.psd;
            }
        }
    }
    for bin in &mut accum {
        bin.psd /= segments.len() as f64;
    }
    let analysis = SpectrumAnalysis {
        bins: accum,
        sample_rate_hz: sample_rate,
        bin_width_hz: sample_rate / transform.window_samples as f64,
        window: transform.window,
        window_samples: transform.window_samples,
    };
    let mut records = Vec::with_capacity(analysis.bins.len());
    for bin in &analysis.bins {
        let mut parameters = feature_channel_window_parameters(&transform.channel, analysis.window);
        parameters.push(TransformParameterMetadata::integer(
            "window_samples",
            transform.window_samples as u64,
            "samples",
        ));
        parameters.push(TransformParameterMetadata::integer(
            "overlap_samples",
            transform.overlap_samples as u64,
            "samples",
        ));
        records.push(feature_record_with_context(FeatureRecordFields {
            id: &format!("{}_bin_{}", transform.id, bin.index),
            transform_name: transform.name(),
            category: TransformCategory::FrequencyFilter,
            channel_name: channel.name.clone(),
            value: bin.psd,
            unit: format!("{}^2/Hz", channel.unit.name),
            sequence_index,
            parameters,
            execution: offline_spectral_execution(true, true),
            method_context: FeatureMethodContext {
                overlap_samples: Some(transform.overlap_samples),
                ..spectrum_bin_context(&analysis, bin, "welch_averaged_power_spectral_density")
            },
        })?);
    }
    Ok(records)
}

fn paired_spectrum_records(
    transform: &PairedSpectrumFeatureTransform,
    waveform: &Waveform,
    sequence_index: usize,
) -> Result<Vec<FeatureRecord>> {
    let (channel, other) =
        feature_channel_pair(waveform, &transform.channel, &transform.other_channel)?;
    validate_same_length(transform.name(), &channel.samples, &other.samples)?;
    let sample_rate = sample_rate_hz(waveform, transform.name())?;
    let window_samples = transform.window_samples.unwrap_or(channel.samples.len());
    let overlap_samples = transform.overlap_samples.unwrap_or(0);
    let bins = averaged_paired_spectrum(
        &channel.samples,
        &other.samples,
        sample_rate,
        transform.window,
        window_samples,
        overlap_samples,
    )?;
    let mut records = Vec::with_capacity(bins.len());
    for bin in bins {
        let (value, unit, normalization, context_complex) = match transform.name() {
            "cross_spectrum" => (
                bin.cross.magnitude(),
                format!("{}*{}/Hz", channel.unit.name, other.unit.name),
                "averaged_cross_power_spectral_density",
                bin.cross,
            ),
            "coherence" => {
                let coherence = if bin.input_psd <= f64::EPSILON || bin.output_psd <= f64::EPSILON {
                    0.0
                } else {
                    bin.cross.magnitude().powi(2) / (bin.input_psd * bin.output_psd)
                };
                (
                    coherence.clamp(0.0, 1.0),
                    "ratio".to_string(),
                    "magnitude_squared_coherence",
                    ComplexValue::new(coherence, 0.0),
                )
            }
            "transfer_function" => {
                let transfer = if bin.input_psd <= f64::EPSILON {
                    ComplexValue::ZERO
                } else {
                    bin.cross / bin.input_psd
                };
                (
                    transfer.magnitude(),
                    format!("{}/{}", other.unit.name, channel.unit.name),
                    "output_over_input_transfer_estimate",
                    transfer,
                )
            }
            other_name => {
                return Err(WaveformError::InvalidParameter {
                    name: "feature_transforms.type".to_string(),
                    reason: format!("unsupported paired spectrum transform `{other_name}`"),
                });
            }
        };
        let mut parameters =
            feature_channel_window_parameters(&transform.channel, transform.window);
        parameters.push(TransformParameterMetadata::text(
            "other_channel",
            transform.other_channel.clone(),
        ));
        parameters.push(TransformParameterMetadata::integer(
            "window_samples",
            window_samples as u64,
            "samples",
        ));
        parameters.push(TransformParameterMetadata::integer(
            "overlap_samples",
            overlap_samples as u64,
            "samples",
        ));
        records.push(feature_record_with_context(FeatureRecordFields {
            id: &format!("{}_bin_{}", transform.id, bin.index),
            transform_name: transform.name(),
            category: TransformCategory::FrequencyFilter,
            channel_name: channel.name.clone(),
            value,
            unit,
            sequence_index,
            parameters,
            execution: offline_spectral_execution(true, true),
            method_context: FeatureMethodContext {
                other_channel: Some(transform.other_channel.clone()),
                bin_index: Some(bin.index),
                frequency_hz: Some(bin.frequency_hz),
                bin_frequency_hz: Some(bin.frequency_hz),
                bin_width_hz: Some(bin.bin_width_hz),
                window: Some(transform.window.name().to_string()),
                window_samples: Some(window_samples),
                overlap_samples: Some(overlap_samples),
                real: Some(context_complex.real),
                imaginary: Some(context_complex.imaginary),
                magnitude: Some(context_complex.magnitude()),
                phase_rad: Some(context_complex.phase()),
                normalization: Some(normalization.to_string()),
                ..FeatureMethodContext::default()
            },
        })?);
    }
    Ok(records)
}

fn harmonic_analysis_records(
    transform: &HarmonicFeatureTransform,
    waveform: &Waveform,
    sequence_index: usize,
) -> Result<Vec<FeatureRecord>> {
    let channel = feature_channel(waveform, &transform.channel)?;
    let sample_rate = sample_rate_hz(waveform, transform.name())?;
    let analysis = spectrum_analysis(&channel.samples, sample_rate, transform.window)?;
    let harmonic_bins = harmonic_bins(
        &analysis,
        transform.fundamental_hz,
        transform.harmonic_count,
    )?;
    let fundamental_hz = harmonic_bins[0].0;
    let mut records = Vec::with_capacity(harmonic_bins.len());
    for (harmonic_index, (_, bin)) in harmonic_bins.iter().enumerate() {
        let harmonic_number = harmonic_index + 1;
        let mut parameters =
            feature_channel_window_parameters(&transform.channel, transform.window);
        parameters.push(TransformParameterMetadata::integer(
            "harmonic_count",
            transform.harmonic_count as u64,
            "harmonics",
        ));
        if let Some(fundamental_hz) = transform.fundamental_hz {
            parameters.push(TransformParameterMetadata::float(
                "fundamental_hz",
                fundamental_hz,
                "Hz",
            ));
        }
        records.push(feature_record_with_context(FeatureRecordFields {
            id: &format!("{}_harmonic_{harmonic_number}", transform.id),
            transform_name: transform.name(),
            category: TransformCategory::Feature,
            channel_name: channel.name.clone(),
            value: bin.amplitude,
            unit: channel.unit.name.clone(),
            sequence_index,
            parameters,
            execution: offline_spectral_execution(true, false),
            method_context: FeatureMethodContext {
                harmonic_index: Some(harmonic_number),
                fundamental_hz: Some(fundamental_hz),
                frequency_hz: Some(bin.frequency_hz),
                bin_index: Some(bin.index),
                bin_frequency_hz: Some(bin.frequency_hz),
                bin_width_hz: Some(analysis.bin_width_hz),
                window: Some(transform.window.name().to_string()),
                window_samples: Some(analysis.window_samples),
                magnitude: Some(bin.amplitude),
                phase_rad: Some(bin.value.phase()),
                normalization: Some("one_sided_peak_amplitude".to_string()),
                ..FeatureMethodContext::default()
            },
        })?);
    }
    Ok(records)
}

fn harmonic_metric_record(
    transform: &HarmonicMetricFeatureTransform,
    waveform: &Waveform,
    sequence_index: usize,
) -> Result<Vec<FeatureRecord>> {
    let channel = feature_channel(waveform, &transform.channel)?;
    let sample_rate = sample_rate_hz(waveform, transform.name())?;
    let analysis = spectrum_analysis(&channel.samples, sample_rate, transform.window)?;
    let harmonic_bins = harmonic_bins(
        &analysis,
        transform.fundamental_hz,
        transform.harmonic_count,
    )?;
    let fundamental_power = harmonic_bins[0].1.power;
    if fundamental_power <= f64::EPSILON {
        return Err(WaveformError::InvalidWaveform {
            reason: format!("{} requires non-zero fundamental power", transform.name()),
        });
    }
    let harmonic_power = harmonic_bins
        .iter()
        .skip(1)
        .map(|(_, bin)| bin.power)
        .sum::<f64>();
    let total_power = analysis
        .bins
        .iter()
        .filter(|bin| bin.index != 0)
        .map(|bin| bin.power)
        .sum::<f64>();
    let residual_power = (total_power - fundamental_power).max(0.0);
    let value = match transform.name() {
        "thd" => (harmonic_power / fundamental_power).sqrt(),
        "snr" => ratio_db(
            fundamental_power,
            (residual_power - harmonic_power).max(f64::EPSILON),
        ),
        "sinad" => ratio_db(fundamental_power, residual_power.max(f64::EPSILON)),
        "enob" => (ratio_db(fundamental_power, residual_power.max(f64::EPSILON)) - 1.76) / 6.02,
        other => {
            return Err(WaveformError::InvalidParameter {
                name: "feature_transforms.type".to_string(),
                reason: format!("unsupported harmonic metric `{other}`"),
            });
        }
    };
    let unit = match transform.name() {
        "thd" => "ratio",
        "enob" => "bits",
        _ => "dB",
    };
    let fundamental_hz = harmonic_bins[0].0;
    let record = scalar_spectrum_record(ScalarSpectrumRecordRequest {
        id: &transform.id,
        transform_name: transform.name(),
        channel_parameter: &transform.channel,
        channel_name: channel.name.clone(),
        value,
        unit: unit.to_string(),
        sequence_index,
        analysis: &analysis,
        normalization: transform.name(),
        extra_context: FeatureMethodContext {
            fundamental_hz: Some(fundamental_hz),
            harmonic_index: Some(transform.harmonic_count),
            ..FeatureMethodContext::default()
        },
    })?;
    Ok(vec![record])
}

fn time_frequency_records(
    transform: &TimeFrequencyFeatureTransform,
    waveform: &Waveform,
    sequence_index: usize,
) -> Result<Vec<FeatureRecord>> {
    let channel = feature_channel(waveform, &transform.channel)?;
    let sample_rate = sample_rate_hz(waveform, transform.name())?;
    let segments = segment_ranges(
        channel.samples.len(),
        transform.window_samples,
        transform.overlap_samples,
    )?;
    let mut records = Vec::new();
    for (segment_index, (start, end)) in segments.into_iter().enumerate() {
        let analysis =
            spectrum_analysis(&channel.samples[start..end], sample_rate, transform.window)?;
        let start_s = waveform.time[start];
        let end_s = waveform.time[end - 1];
        for bin in &analysis.bins {
            let value = if transform.name() == "spectrogram" {
                bin.psd
            } else {
                bin.amplitude
            };
            let unit = if transform.name() == "spectrogram" {
                format!("{}^2/Hz", channel.unit.name)
            } else {
                channel.unit.name.clone()
            };
            let mut parameters =
                feature_channel_window_parameters(&transform.channel, transform.window);
            parameters.push(TransformParameterMetadata::integer(
                "window_samples",
                transform.window_samples as u64,
                "samples",
            ));
            parameters.push(TransformParameterMetadata::integer(
                "overlap_samples",
                transform.overlap_samples as u64,
                "samples",
            ));
            records.push(feature_record_with_context(FeatureRecordFields {
                id: &format!("{}_segment_{segment_index}_bin_{}", transform.id, bin.index),
                transform_name: transform.name(),
                category: TransformCategory::TimeFrequency,
                channel_name: channel.name.clone(),
                value,
                unit,
                sequence_index,
                parameters,
                execution: offline_spectral_execution(true, true),
                method_context: FeatureMethodContext {
                    segment_index: Some(segment_index),
                    segment_start_s: Some(start_s),
                    segment_end_s: Some(end_s),
                    bin_index: Some(bin.index),
                    frequency_hz: Some(bin.frequency_hz),
                    bin_frequency_hz: Some(bin.frequency_hz),
                    bin_width_hz: Some(analysis.bin_width_hz),
                    window: Some(transform.window.name().to_string()),
                    window_samples: Some(transform.window_samples),
                    overlap_samples: Some(transform.overlap_samples),
                    real: Some(bin.value.real),
                    imaginary: Some(bin.value.imaginary),
                    magnitude: Some(bin.amplitude),
                    phase_rad: Some(bin.value.phase()),
                    normalization: Some(if transform.name() == "spectrogram" {
                        "one_sided_power_spectral_density".to_string()
                    } else {
                        "one_sided_peak_amplitude".to_string()
                    }),
                    ..FeatureMethodContext::default()
                },
            })?);
        }
    }
    Ok(records)
}

fn spectral_rolloff_record(
    transform: &SpectralRolloffFeatureTransform,
    waveform: &Waveform,
    sequence_index: usize,
) -> Result<Vec<FeatureRecord>> {
    validate_percentile(transform.rolloff_percent)?;
    let channel = feature_channel(waveform, &transform.channel)?;
    let sample_rate = sample_rate_hz(waveform, transform.name())?;
    let analysis = spectrum_analysis(&channel.samples, sample_rate, transform.window)?;
    let total_power = positive_frequency_power(&analysis)?;
    let target = total_power * (transform.rolloff_percent / 100.0);
    let mut cumulative = 0.0;
    let mut rolloff_hz = 0.0;
    for bin in analysis.bins.iter().filter(|bin| bin.index != 0) {
        cumulative += bin.power;
        if cumulative >= target {
            rolloff_hz = bin.frequency_hz;
            break;
        }
    }
    let record = scalar_spectrum_record(ScalarSpectrumRecordRequest {
        id: &transform.id,
        transform_name: transform.name(),
        channel_parameter: &transform.channel,
        channel_name: channel.name.clone(),
        value: rolloff_hz,
        unit: "Hz".to_string(),
        sequence_index,
        analysis: &analysis,
        normalization: "cumulative_power_rolloff",
        extra_context: FeatureMethodContext {
            rolloff_percent: Some(transform.rolloff_percent),
            ..FeatureMethodContext::default()
        },
    })?;
    Ok(vec![record])
}

fn band_power_record(
    transform: &BandPowerFeatureTransform,
    waveform: &Waveform,
    sequence_index: usize,
) -> Result<Vec<FeatureRecord>> {
    validate_band(transform.band_low_hz, transform.band_high_hz)?;
    let channel = feature_channel(waveform, &transform.channel)?;
    let sample_rate = sample_rate_hz(waveform, transform.name())?;
    let analysis = spectrum_analysis(&channel.samples, sample_rate, transform.window)?;
    let nyquist = sample_rate / 2.0;
    if transform.band_high_hz > nyquist {
        return Err(WaveformError::InvalidParameter {
            name: "feature_transforms.band_high_hz".to_string(),
            reason: "must not exceed Nyquist frequency".to_string(),
        });
    }
    let power = analysis
        .bins
        .iter()
        .filter(|bin| {
            bin.frequency_hz >= transform.band_low_hz && bin.frequency_hz <= transform.band_high_hz
        })
        .map(|bin| bin.power)
        .sum::<f64>();
    let record = scalar_spectrum_record(ScalarSpectrumRecordRequest {
        id: &transform.id,
        transform_name: transform.name(),
        channel_parameter: &transform.channel,
        channel_name: channel.name.clone(),
        value: power,
        unit: format!("{}^2", channel.unit.name),
        sequence_index,
        analysis: &analysis,
        normalization: "sum_one_sided_power_in_band",
        extra_context: FeatureMethodContext {
            band_low_hz: Some(transform.band_low_hz),
            band_high_hz: Some(transform.band_high_hz),
            ..FeatureMethodContext::default()
        },
    })?;
    Ok(vec![record])
}

#[derive(Clone, Copy)]
enum SpectrumBinValue {
    Amplitude,
    Power,
    Psd,
}

impl SpectrumBinValue {
    fn read(self, bin: &SpectrumBin) -> f64 {
        match self {
            Self::Amplitude => bin.amplitude,
            Self::Power => bin.power,
            Self::Psd => bin.psd,
        }
    }
}

struct SpectrumBinRecordRequest<'a> {
    id: &'a str,
    transform_name: &'static str,
    channel_parameter: &'a str,
    channel_name: String,
    unit: String,
    sequence_index: usize,
    analysis: &'a SpectrumAnalysis,
    value: SpectrumBinValue,
    normalization: &'a str,
}

fn spectrum_bin_records(request: SpectrumBinRecordRequest<'_>) -> Result<Vec<FeatureRecord>> {
    let mut records = Vec::with_capacity(request.analysis.bins.len());
    for bin in &request.analysis.bins {
        records.push(feature_record_with_context(FeatureRecordFields {
            id: &format!("{}_bin_{}", request.id, bin.index),
            transform_name: request.transform_name,
            category: TransformCategory::FrequencyFilter,
            channel_name: request.channel_name.clone(),
            value: request.value.read(bin),
            unit: request.unit.clone(),
            sequence_index: request.sequence_index,
            parameters: feature_channel_window_parameters(
                request.channel_parameter,
                request.analysis.window,
            ),
            execution: offline_spectral_execution(true, false),
            method_context: spectrum_bin_context(request.analysis, bin, request.normalization),
        })?);
    }
    Ok(records)
}

struct ScalarSpectrumRecordRequest<'a> {
    id: &'a str,
    transform_name: &'static str,
    channel_parameter: &'a str,
    channel_name: String,
    value: f64,
    unit: String,
    sequence_index: usize,
    analysis: &'a SpectrumAnalysis,
    normalization: &'a str,
    extra_context: FeatureMethodContext,
}

fn scalar_spectrum_record(request: ScalarSpectrumRecordRequest<'_>) -> Result<FeatureRecord> {
    let mut context = FeatureMethodContext {
        bin_width_hz: Some(request.analysis.bin_width_hz),
        window: Some(request.analysis.window.name().to_string()),
        window_samples: Some(request.analysis.window_samples),
        normalization: Some(request.normalization.to_string()),
        ..FeatureMethodContext::default()
    };
    merge_feature_context(&mut context, request.extra_context);
    feature_record_with_context(FeatureRecordFields {
        id: request.id,
        transform_name: request.transform_name,
        category: TransformCategory::Feature,
        channel_name: request.channel_name,
        value: request.value,
        unit: request.unit,
        sequence_index: request.sequence_index,
        parameters: feature_channel_window_parameters(
            request.channel_parameter,
            request.analysis.window,
        ),
        execution: offline_spectral_execution(true, false),
        method_context: context,
    })
}

fn feature_channel_window_parameters(
    channel_parameter: &str,
    window: WindowSpec,
) -> Vec<TransformParameterMetadata> {
    let mut parameters = vec![TransformParameterMetadata::text(
        "channel",
        channel_parameter.to_string(),
    )];
    parameters.extend(window.parameters());
    parameters
}

fn spectrum_bin_context(
    analysis: &SpectrumAnalysis,
    bin: &SpectrumBin,
    normalization: &str,
) -> FeatureMethodContext {
    FeatureMethodContext {
        bin_index: Some(bin.index),
        frequency_hz: Some(bin.frequency_hz),
        bin_frequency_hz: Some(bin.frequency_hz),
        bin_width_hz: Some(analysis.bin_width_hz),
        window: Some(analysis.window.name().to_string()),
        window_samples: Some(analysis.window_samples),
        real: Some(bin.value.real),
        imaginary: Some(bin.value.imaginary),
        magnitude: Some(bin.amplitude),
        phase_rad: Some(bin.value.phase()),
        normalization: Some(normalization.to_string()),
        ..FeatureMethodContext::default()
    }
}

fn offline_spectral_execution(
    sample_rate_required: bool,
    stateful: bool,
) -> TransformExecutionMetadata {
    TransformExecutionMetadata {
        sample_rate_required,
        stateful,
        causal: false,
        phase_effect: TransformPhaseEffect::None,
        streaming_supported: false,
        offline_only: true,
    }
}

fn spectral_centroid(analysis: &SpectrumAnalysis) -> Result<f64> {
    let total_power = positive_frequency_power(analysis)?;
    Ok(analysis
        .bins
        .iter()
        .filter(|bin| bin.index != 0)
        .map(|bin| bin.frequency_hz * bin.power)
        .sum::<f64>()
        / total_power)
}

fn spectral_bandwidth(analysis: &SpectrumAnalysis, centroid_hz: f64) -> Result<f64> {
    let total_power = positive_frequency_power(analysis)?;
    Ok((analysis
        .bins
        .iter()
        .filter(|bin| bin.index != 0)
        .map(|bin| (bin.frequency_hz - centroid_hz).powi(2) * bin.power)
        .sum::<f64>()
        / total_power)
        .sqrt())
}

fn positive_frequency_power(analysis: &SpectrumAnalysis) -> Result<f64> {
    let total_power = analysis
        .bins
        .iter()
        .filter(|bin| bin.index != 0)
        .map(|bin| bin.power)
        .sum::<f64>();
    if total_power <= f64::EPSILON {
        return Err(WaveformError::InvalidWaveform {
            reason: "spectral feature requires non-zero positive-frequency power".to_string(),
        });
    }
    Ok(total_power)
}

#[derive(Debug, Clone, Copy)]
struct PairedSpectrumBin {
    index: usize,
    frequency_hz: f64,
    bin_width_hz: f64,
    input_psd: f64,
    output_psd: f64,
    cross: ComplexValue,
}

fn averaged_paired_spectrum(
    input: &[f64],
    output: &[f64],
    sample_rate_hz: f64,
    window: WindowSpec,
    window_samples: usize,
    overlap_samples: usize,
) -> Result<Vec<PairedSpectrumBin>> {
    let segments = segment_ranges(input.len(), window_samples, overlap_samples)?;
    let coefficients = window.coefficients(window_samples)?;
    let window_power = coefficients
        .iter()
        .map(|coefficient| coefficient * coefficient)
        .sum::<f64>();
    if window_power <= f64::EPSILON {
        return Err(WaveformError::InvalidWaveform {
            reason: "window power must be non-zero".to_string(),
        });
    }
    let bin_width_hz = sample_rate_hz / window_samples as f64;
    let nyquist_index = window_samples / 2;
    let mut bins = (0..=nyquist_index)
        .map(|index| PairedSpectrumBin {
            index,
            frequency_hz: index as f64 * bin_width_hz,
            bin_width_hz,
            input_psd: 0.0,
            output_psd: 0.0,
            cross: ComplexValue::ZERO,
        })
        .collect::<Vec<_>>();
    for (start, end) in segments.iter().copied() {
        let input_segment = input[start..end]
            .iter()
            .zip(coefficients.iter())
            .map(|(sample, coefficient)| sample * coefficient)
            .collect::<Vec<_>>();
        let output_segment = output[start..end]
            .iter()
            .zip(coefficients.iter())
            .map(|(sample, coefficient)| sample * coefficient)
            .collect::<Vec<_>>();
        let input_fft = forward_transform_real(&input_segment);
        let output_fft = forward_transform_real(&output_segment);
        for bin in &mut bins {
            let interior =
                bin.index != 0 && !(window_samples % 2 == 0 && bin.index == nyquist_index);
            let one_sided = if interior { 2.0 } else { 1.0 };
            let scale = one_sided / (sample_rate_hz * window_power);
            let input_value = input_fft[bin.index];
            let output_value = output_fft[bin.index];
            bin.input_psd += input_value.magnitude().powi(2) * scale;
            bin.output_psd += output_value.magnitude().powi(2) * scale;
            bin.cross = bin.cross
                + ((output_value * input_value.conjugate()) * ComplexValue::new(scale, 0.0));
        }
    }
    for bin in &mut bins {
        bin.input_psd /= segments.len() as f64;
        bin.output_psd /= segments.len() as f64;
        bin.cross = bin.cross / segments.len() as f64;
    }
    Ok(bins)
}

fn segment_ranges(
    sample_count: usize,
    window_samples: usize,
    overlap_samples: usize,
) -> Result<Vec<(usize, usize)>> {
    validate_positive_usize("window_samples", window_samples)?;
    if overlap_samples >= window_samples {
        return Err(WaveformError::InvalidParameter {
            name: "feature_transforms.overlap_samples".to_string(),
            reason: "must be less than window_samples".to_string(),
        });
    }
    if window_samples > sample_count {
        return Err(WaveformError::InvalidParameter {
            name: "feature_transforms.window_samples".to_string(),
            reason: "must not exceed sample count".to_string(),
        });
    }
    let step = window_samples - overlap_samples;
    let mut ranges = Vec::new();
    let mut start = 0;
    while start + window_samples <= sample_count {
        ranges.push((start, start + window_samples));
        start += step;
    }
    if ranges.is_empty() {
        return Err(WaveformError::InvalidWaveform {
            reason: "windowing produced no complete segments".to_string(),
        });
    }
    Ok(ranges)
}

fn harmonic_bins(
    analysis: &SpectrumAnalysis,
    fundamental_hz: Option<f64>,
    harmonic_count: usize,
) -> Result<Vec<(f64, SpectrumBin)>> {
    validate_positive_usize("harmonic_count", harmonic_count)?;
    let fundamental = match fundamental_hz {
        Some(value) => {
            validate_positive_parameter("fundamental_hz", value)?;
            value
        }
        None => analysis
            .bins
            .iter()
            .filter(|bin| bin.index != 0)
            .max_by(|left, right| left.amplitude.total_cmp(&right.amplitude))
            .map(|bin| bin.frequency_hz)
            .ok_or_else(|| WaveformError::InvalidWaveform {
                reason: "harmonic analysis requires at least one positive frequency bin"
                    .to_string(),
            })?,
    };
    let nyquist = analysis.sample_rate_hz / 2.0;
    if fundamental > nyquist {
        return Err(WaveformError::InvalidParameter {
            name: "feature_transforms.fundamental_hz".to_string(),
            reason: "must not exceed Nyquist frequency".to_string(),
        });
    }
    let mut bins = Vec::new();
    for harmonic in 1..=harmonic_count {
        let target_hz = fundamental * harmonic as f64;
        if target_hz > nyquist + analysis.bin_width_hz / 2.0 {
            break;
        }
        let bin = nearest_frequency_bin(analysis, target_hz)?;
        bins.push((fundamental, bin));
    }
    if bins.is_empty() {
        return Err(WaveformError::InvalidWaveform {
            reason: "harmonic analysis found no bins".to_string(),
        });
    }
    Ok(bins)
}

fn nearest_frequency_bin(analysis: &SpectrumAnalysis, frequency_hz: f64) -> Result<SpectrumBin> {
    analysis
        .bins
        .iter()
        .min_by(|left, right| {
            (left.frequency_hz - frequency_hz)
                .abs()
                .total_cmp(&(right.frequency_hz - frequency_hz).abs())
        })
        .copied()
        .ok_or_else(|| WaveformError::InvalidWaveform {
            reason: "spectrum analysis produced no bins".to_string(),
        })
}

fn ratio_db(numerator: f64, denominator: f64) -> f64 {
    10.0 * (numerator / denominator).log10()
}

fn validate_band(low_hz: f64, high_hz: f64) -> Result<()> {
    validate_finite_parameter("band_low_hz", low_hz)?;
    validate_finite_parameter("band_high_hz", high_hz)?;
    if low_hz < 0.0 {
        return Err(WaveformError::InvalidParameter {
            name: "feature_transforms.band_low_hz".to_string(),
            reason: "must be non-negative".to_string(),
        });
    }
    if high_hz <= low_hz {
        return Err(WaveformError::InvalidParameter {
            name: "feature_transforms.band_high_hz".to_string(),
            reason: "must be greater than band_low_hz".to_string(),
        });
    }
    Ok(())
}

fn merge_feature_context(target: &mut FeatureMethodContext, source: FeatureMethodContext) {
    if source.percentile.is_some() {
        target.percentile = source.percentile;
    }
    if source.quantile.is_some() {
        target.quantile = source.quantile;
    }
    if source.bins.is_some() {
        target.bins = source.bins;
    }
    if source.bin_index.is_some() {
        target.bin_index = source.bin_index;
    }
    if source.bin_min.is_some() {
        target.bin_min = source.bin_min;
    }
    if source.bin_max.is_some() {
        target.bin_max = source.bin_max;
    }
    if source.other_channel.is_some() {
        target.other_channel = source.other_channel;
    }
    if source.lag_samples.is_some() {
        target.lag_samples = source.lag_samples;
    }
    if source.frequency_hz.is_some() {
        target.frequency_hz = source.frequency_hz;
    }
    if source.bin_frequency_hz.is_some() {
        target.bin_frequency_hz = source.bin_frequency_hz;
    }
    if source.bin_width_hz.is_some() {
        target.bin_width_hz = source.bin_width_hz;
    }
    if source.window.is_some() {
        target.window = source.window;
    }
    if source.window_index.is_some() {
        target.window_index = source.window_index;
    }
    if source.window_samples.is_some() {
        target.window_samples = source.window_samples;
    }
    if source.overlap_samples.is_some() {
        target.overlap_samples = source.overlap_samples;
    }
    if source.sample_index.is_some() {
        target.sample_index = source.sample_index;
    }
    if source.segment_index.is_some() {
        target.segment_index = source.segment_index;
    }
    if source.segment_start_s.is_some() {
        target.segment_start_s = source.segment_start_s;
    }
    if source.segment_end_s.is_some() {
        target.segment_end_s = source.segment_end_s;
    }
    if source.real.is_some() {
        target.real = source.real;
    }
    if source.imaginary.is_some() {
        target.imaginary = source.imaginary;
    }
    if source.magnitude.is_some() {
        target.magnitude = source.magnitude;
    }
    if source.phase_rad.is_some() {
        target.phase_rad = source.phase_rad;
    }
    if source.harmonic_index.is_some() {
        target.harmonic_index = source.harmonic_index;
    }
    if source.fundamental_hz.is_some() {
        target.fundamental_hz = source.fundamental_hz;
    }
    if source.band_low_hz.is_some() {
        target.band_low_hz = source.band_low_hz;
    }
    if source.band_high_hz.is_some() {
        target.band_high_hz = source.band_high_hz;
    }
    if source.rolloff_percent.is_some() {
        target.rolloff_percent = source.rolloff_percent;
    }
    if source.normalization.is_some() {
        target.normalization = source.normalization;
    }
}

fn rms_value(samples: &[f64]) -> Result<f64> {
    validate_finite_samples("rms", samples)?;
    let mean_square =
        samples.iter().map(|sample| sample * sample).sum::<f64>() / samples.len() as f64;
    Ok(mean_square.sqrt())
}

fn mean_value(samples: &[f64]) -> Result<f64> {
    validate_finite_samples("mean", samples)?;
    Ok(samples.iter().sum::<f64>() / samples.len() as f64)
}

fn min_max_value(samples: &[f64]) -> Result<(f64, f64)> {
    validate_finite_samples("min_max", samples)?;
    let mut min = samples[0];
    let mut max = samples[0];
    for sample in &samples[1..] {
        min = min.min(*sample);
        max = max.max(*sample);
    }
    Ok((min, max))
}

fn mode_value(samples: &[f64]) -> Result<f64> {
    validate_finite_samples("mode", samples)?;
    let mut sorted = samples.to_vec();
    sorted.sort_by(f64::total_cmp);
    let mut best_value = sorted[0];
    let mut best_count = 1_usize;
    let mut current_value = sorted[0];
    let mut current_count = 1_usize;
    for sample in &sorted[1..] {
        if sample.total_cmp(&current_value).is_eq() {
            current_count += 1;
        } else {
            if current_count > best_count {
                best_value = current_value;
                best_count = current_count;
            }
            current_value = *sample;
            current_count = 1;
        }
    }
    if current_count > best_count {
        best_value = current_value;
    }
    Ok(best_value)
}

fn variance_value(samples: &[f64]) -> Result<f64> {
    let mean = mean_value(samples)?;
    Ok(samples
        .iter()
        .map(|sample| {
            let delta = sample - mean;
            delta * delta
        })
        .sum::<f64>()
        / samples.len() as f64)
}

fn skewness_value(samples: &[f64]) -> Result<f64> {
    let mean = mean_value(samples)?;
    let std_dev = variance_value(samples)?.sqrt();
    validate_non_constant("skewness", std_dev)?;
    Ok(samples
        .iter()
        .map(|sample| ((sample - mean) / std_dev).powi(3))
        .sum::<f64>()
        / samples.len() as f64)
}

fn kurtosis_value(samples: &[f64]) -> Result<f64> {
    let mean = mean_value(samples)?;
    let std_dev = variance_value(samples)?.sqrt();
    validate_non_constant("kurtosis", std_dev)?;
    Ok(samples
        .iter()
        .map(|sample| ((sample - mean) / std_dev).powi(4))
        .sum::<f64>()
        / samples.len() as f64)
}

fn percentile_value(samples: &[f64], quantile: f64) -> Result<f64> {
    validate_quantile(quantile)?;
    validate_finite_samples("percentile", samples)?;
    let mut sorted = samples.to_vec();
    sorted.sort_by(f64::total_cmp);
    if sorted.len() == 1 {
        return Ok(sorted[0]);
    }
    let rank = quantile * (sorted.len() - 1) as f64;
    let lower_index = rank.floor() as usize;
    let upper_index = rank.ceil() as usize;
    if lower_index == upper_index {
        return Ok(sorted[lower_index]);
    }
    let fraction = rank - lower_index as f64;
    Ok(sorted[lower_index] * (1.0 - fraction) + sorted[upper_index] * fraction)
}

fn covariance_value(left: &[f64], right: &[f64]) -> Result<f64> {
    validate_same_length("covariance", left, right)?;
    let left_mean = mean_value(left)?;
    let right_mean = mean_value(right)?;
    Ok(left
        .iter()
        .zip(right.iter())
        .map(|(left, right)| (left - left_mean) * (right - right_mean))
        .sum::<f64>()
        / left.len() as f64)
}

fn correlation_value(left: &[f64], right: &[f64]) -> Result<f64> {
    let covariance = covariance_value(left, right)?;
    let left_std = variance_value(left)?.sqrt();
    let right_std = variance_value(right)?.sqrt();
    validate_non_constant("correlation", left_std)?;
    validate_non_constant("correlation", right_std)?;
    Ok(covariance / (left_std * right_std))
}

fn lagged_correlation_value(left: &[f64], right: &[f64], lag_samples: usize) -> Result<f64> {
    validate_same_length("lagged_correlation", left, right)?;
    if lag_samples >= left.len() {
        return Err(WaveformError::InvalidParameter {
            name: "feature_transforms.lag_samples".to_string(),
            reason: "must be less than the sample count".to_string(),
        });
    }
    let pair_count = left.len() - lag_samples;
    correlation_value(&left[..pair_count], &right[lag_samples..])
}

fn validate_same_length(transform_name: &str, left: &[f64], right: &[f64]) -> Result<()> {
    validate_finite_samples(transform_name, left)?;
    validate_finite_samples(transform_name, right)?;
    if left.len() != right.len() {
        return Err(WaveformError::MismatchedSampleCount {
            expected: left.len(),
            actual: right.len(),
        });
    }
    Ok(())
}

fn peak_to_peak_value(samples: &[f64]) -> Result<f64> {
    validate_finite_samples("peak_to_peak", samples)?;
    let mut min = samples[0];
    let mut max = samples[0];
    for sample in &samples[1..] {
        min = min.min(*sample);
        max = max.max(*sample);
    }
    Ok(max - min)
}

fn crest_factor_value(samples: &[f64]) -> Result<f64> {
    let rms = rms_value(samples)?;
    if rms <= f64::EPSILON {
        return Err(WaveformError::InvalidWaveform {
            reason: "crest_factor requires non-zero RMS".to_string(),
        });
    }
    let peak = samples
        .iter()
        .fold(0.0_f64, |max, sample| max.max(sample.abs()));
    Ok(peak / rms)
}

fn energy_value(time: &[f64], samples: &[f64]) -> Result<f64> {
    validate_time_axis(time, "energy")?;
    validate_finite_samples("energy", samples)?;
    if samples.len() != time.len() {
        return Err(WaveformError::MismatchedSampleCount {
            expected: time.len(),
            actual: samples.len(),
        });
    }

    let mut energy = 0.0;
    for index in 1..samples.len() {
        let dt = time[index] - time[index - 1];
        energy += 0.5 * (samples[index - 1].powi(2) + samples[index].powi(2)) * dt;
    }
    Ok(energy)
}

fn trapezoidal_area(time: &[f64], samples: &[f64]) -> Result<f64> {
    validate_time_axis(time, "area_under_curve")?;
    validate_finite_samples("area_under_curve", samples)?;
    if samples.len() != time.len() {
        return Err(WaveformError::MismatchedSampleCount {
            expected: time.len(),
            actual: samples.len(),
        });
    }

    let mut area = 0.0;
    for index in 1..samples.len() {
        let dt = time[index] - time[index - 1];
        area += 0.5 * (samples[index - 1] + samples[index]) * dt;
    }
    Ok(area)
}

fn waveform_duration(time: &[f64]) -> Result<f64> {
    validate_time_axis(time, "power")?;
    let duration = time[time.len() - 1] - time[0];
    validate_positive_parameter("duration_s", duration)?;
    Ok(duration)
}

fn validate_finite_samples(transform_name: &str, samples: &[f64]) -> Result<()> {
    if samples.is_empty() || samples.iter().any(|sample| !sample.is_finite()) {
        return Err(WaveformError::InvalidWaveform {
            reason: format!("{transform_name} requires finite samples"),
        });
    }
    Ok(())
}

fn validate_time_axis(time: &[f64], transform_name: &str) -> Result<()> {
    if time.iter().any(|sample_time| !sample_time.is_finite()) {
        return Err(WaveformError::InvalidWaveform {
            reason: format!("time samples must be finite for {transform_name}"),
        });
    }
    for pair in time.windows(2) {
        if pair[1] <= pair[0] {
            return Err(WaveformError::InvalidWaveform {
                reason: format!("time samples must be strictly increasing for {transform_name}"),
            });
        }
    }
    Ok(())
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

fn validate_percentile(percentile: f64) -> Result<()> {
    validate_finite_parameter("percentile", percentile)?;
    if !(0.0..=100.0).contains(&percentile) {
        return Err(WaveformError::InvalidParameter {
            name: "percentile".to_string(),
            reason: "must be between zero and 100".to_string(),
        });
    }
    Ok(())
}

fn validate_quantile(quantile: f64) -> Result<()> {
    validate_finite_parameter("quantile", quantile)?;
    if !(0.0..=1.0).contains(&quantile) {
        return Err(WaveformError::InvalidParameter {
            name: "quantile".to_string(),
            reason: "must be between zero and one".to_string(),
        });
    }
    Ok(())
}

fn validate_positive_usize(name: &str, value: usize) -> Result<()> {
    if value == 0 {
        return Err(WaveformError::InvalidParameter {
            name: name.to_string(),
            reason: "must be greater than zero".to_string(),
        });
    }
    Ok(())
}

fn validate_non_constant(transform_name: &str, std_dev: f64) -> Result<()> {
    if std_dev <= f64::EPSILON {
        return Err(WaveformError::InvalidWaveform {
            reason: format!("{transform_name} requires non-constant samples"),
        });
    }
    Ok(())
}

fn validate_positive_parameter(name: &str, value: f64) -> Result<()> {
    validate_finite_parameter(name, value)?;
    if value <= 0.0 {
        return Err(WaveformError::InvalidParameter {
            name: name.to_string(),
            reason: "must be greater than zero".to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Channel, TransformOutputChannelKind, Unit};
    use crate::transform_catalog::transform_catalog_entry;

    fn waveform() -> Waveform {
        Waveform::new(
            vec![0.0, 1.0, 2.0],
            vec![Channel::new("input_v", Unit::volts(), vec![3.0, 4.0, 0.0])],
        )
        .expect("waveform should be valid")
    }

    #[test]
    fn evaluates_m31_scalar_features_with_known_answer_values() {
        let features = vec![
            FeatureTransformStep::Rms(RmsFeatureTransform {
                id: "rms".to_string(),
                channel: "input_v".to_string(),
            }),
            FeatureTransformStep::PeakToPeak(PeakToPeakFeatureTransform {
                id: "peak_to_peak".to_string(),
                channel: "input_v".to_string(),
            }),
            FeatureTransformStep::CrestFactor(CrestFactorFeatureTransform {
                id: "crest_factor".to_string(),
                channel: "input_v".to_string(),
            }),
            FeatureTransformStep::Energy(EnergyFeatureTransform {
                id: "energy".to_string(),
                channel: "input_v".to_string(),
            }),
            FeatureTransformStep::Power(PowerFeatureTransform {
                id: "power".to_string(),
                channel: "input_v".to_string(),
            }),
            FeatureTransformStep::AreaUnderCurve(AreaUnderCurveFeatureTransform {
                id: "area".to_string(),
                channel: "input_v".to_string(),
            }),
            FeatureTransformStep::ImpulseEstimate(ImpulseEstimateFeatureTransform {
                id: "impulse".to_string(),
                channel: "input_v".to_string(),
            }),
        ];

        let records =
            evaluate_feature_transforms(&waveform(), &features).expect("features should evaluate");

        assert_close(records[0].value, (25.0_f64 / 3.0).sqrt());
        assert_close(records[1].value, 4.0);
        assert_close(records[2].value, 4.0 / (25.0_f64 / 3.0).sqrt());
        assert_close(records[3].value, 20.5);
        assert_close(records[4].value, 10.25);
        assert_close(records[5].value, 5.5);
        assert_close(records[6].value, 5.5);
        assert_eq!(records[0].unit, "V");
        assert_eq!(records[2].unit, "ratio");
        assert_eq!(records[3].unit, "V^2*s");
        assert_eq!(records[5].unit, "V*s");
        assert_eq!(
            records[0].transform_metadata.output_channels.kind,
            TransformOutputChannelKind::FeatureRecords
        );
        for record in records {
            let entry = transform_catalog_entry(&record.transform)
                .expect("feature record should be cataloged");
            assert!(
                entry.matches_step_metadata(&record.transform_metadata),
                "metadata for `{}` does not match catalog entry",
                record.transform
            );
        }
    }

    #[test]
    fn evaluates_m32_statistics_correlation_features_with_known_answer_values() {
        let waveform = Waveform::new(
            vec![0.0, 1.0, 2.0, 3.0],
            vec![
                Channel::new("input_v", Unit::volts(), vec![1.0, 2.0, 2.0, 5.0]),
                Channel::new("other_v", Unit::volts(), vec![2.0, 4.0, 4.0, 10.0]),
            ],
        )
        .expect("waveform should be valid");
        let features = vec![
            FeatureTransformStep::Mean(MeanFeatureTransform {
                id: "mean".to_string(),
                channel: "input_v".to_string(),
            }),
            FeatureTransformStep::Median(MedianFeatureTransform {
                id: "median".to_string(),
                channel: "input_v".to_string(),
            }),
            FeatureTransformStep::Mode(ModeFeatureTransform {
                id: "mode".to_string(),
                channel: "input_v".to_string(),
            }),
            FeatureTransformStep::Minimum(MinimumFeatureTransform {
                id: "min".to_string(),
                channel: "input_v".to_string(),
            }),
            FeatureTransformStep::Maximum(MaximumFeatureTransform {
                id: "max".to_string(),
                channel: "input_v".to_string(),
            }),
            FeatureTransformStep::Variance(VarianceFeatureTransform {
                id: "variance".to_string(),
                channel: "input_v".to_string(),
            }),
            FeatureTransformStep::StandardDeviation(StandardDeviationFeatureTransform {
                id: "stddev".to_string(),
                channel: "input_v".to_string(),
            }),
            FeatureTransformStep::Skewness(SkewnessFeatureTransform {
                id: "skewness".to_string(),
                channel: "input_v".to_string(),
            }),
            FeatureTransformStep::Kurtosis(KurtosisFeatureTransform {
                id: "kurtosis".to_string(),
                channel: "input_v".to_string(),
            }),
            FeatureTransformStep::Percentile(PercentileFeatureTransform {
                id: "percentile".to_string(),
                channel: "input_v".to_string(),
                percentile: 75.0,
            }),
            FeatureTransformStep::Quantile(QuantileFeatureTransform {
                id: "quantile".to_string(),
                channel: "input_v".to_string(),
                quantile: 0.25,
            }),
            FeatureTransformStep::Histogram(HistogramFeatureTransform {
                id: "histogram".to_string(),
                channel: "input_v".to_string(),
                bins: 3,
                min_v: Some(0.0),
                max_v: Some(6.0),
            }),
            FeatureTransformStep::Covariance(CovarianceFeatureTransform {
                id: "covariance".to_string(),
                channel: "input_v".to_string(),
                other_channel: "other_v".to_string(),
            }),
            FeatureTransformStep::Correlation(CorrelationFeatureTransform {
                id: "correlation".to_string(),
                channel: "input_v".to_string(),
                other_channel: "other_v".to_string(),
            }),
            FeatureTransformStep::Autocorrelation(AutocorrelationFeatureTransform {
                id: "autocorrelation".to_string(),
                channel: "input_v".to_string(),
                lag_samples: 1,
            }),
            FeatureTransformStep::CrossCorrelation(CrossCorrelationFeatureTransform {
                id: "cross_correlation".to_string(),
                channel: "input_v".to_string(),
                other_channel: "other_v".to_string(),
                lag_samples: 1,
            }),
        ];

        let records =
            evaluate_feature_transforms(&waveform, &features).expect("features should evaluate");

        assert_eq!(records.len(), 18);
        assert_record_value(&records, "mean", 2.5);
        assert_record_value(&records, "median", 2.0);
        assert_record_value(&records, "mode", 2.0);
        assert_record_value(&records, "min", 1.0);
        assert_record_value(&records, "max", 5.0);
        assert_record_value(&records, "variance", 2.25);
        assert_record_value(&records, "stddev", 1.5);
        assert_record_value(&records, "skewness", 8.0 / 9.0);
        assert_record_value(&records, "kurtosis", 59.0 / 27.0);
        assert_record_value(&records, "percentile", 2.75);
        assert_record_value(&records, "quantile", 1.75);
        assert_record_value(&records, "histogram_bin_0", 1.0);
        assert_record_value(&records, "histogram_bin_1", 2.0);
        assert_record_value(&records, "histogram_bin_2", 1.0);
        assert_record_value(&records, "covariance", 4.5);
        assert_record_value(&records, "correlation", 1.0);
        assert_record_value(&records, "autocorrelation", 0.5);
        assert_record_value(&records, "cross_correlation", 0.5);

        let percentile = record(&records, "percentile");
        assert_eq!(percentile.method_context.percentile, Some(75.0));
        let histogram_bin = record(&records, "histogram_bin_1");
        assert_eq!(histogram_bin.method_context.bins, Some(3));
        assert_eq!(histogram_bin.method_context.bin_index, Some(1));
        assert_eq!(histogram_bin.method_context.bin_min, Some(2.0));
        assert_eq!(histogram_bin.method_context.bin_max, Some(4.0));
        let cross_correlation = record(&records, "cross_correlation");
        assert_eq!(
            cross_correlation.method_context.other_channel.as_deref(),
            Some("other_v")
        );
        assert_eq!(cross_correlation.method_context.lag_samples, Some(1));

        for record in records {
            let entry = transform_catalog_entry(&record.transform)
                .expect("feature record should be cataloged");
            assert!(
                entry.matches_step_metadata(&record.transform_metadata),
                "metadata for `{}` does not match catalog entry: {:?} vs {:?}",
                record.transform,
                record.transform_metadata,
                entry
            );
        }
    }

    #[test]
    fn evaluates_m33_spectrum_window_and_spectral_features_with_known_answers() {
        let waveform = m33_waveform();
        let features = vec![
            FeatureTransformStep::WindowFunction(WindowFunctionFeatureTransform {
                id: "hann".to_string(),
                channel: "input_v".to_string(),
                window: WindowSpec::from_config(Some("hann"), None, None, None).unwrap(),
                window_samples: Some(4),
            }),
            FeatureTransformStep::Dft(SpectrumFeatureTransform {
                id: "dft".to_string(),
                transform_name: "dft",
                channel: "input_v".to_string(),
                window: WindowSpec::default(),
            }),
            FeatureTransformStep::Fft(SpectrumFeatureTransform {
                id: "fft".to_string(),
                transform_name: "fft",
                channel: "input_v".to_string(),
                window: WindowSpec::default(),
            }),
            FeatureTransformStep::PowerSpectrum(SpectrumFeatureTransform {
                id: "power".to_string(),
                transform_name: "power_spectrum",
                channel: "input_v".to_string(),
                window: WindowSpec::default(),
            }),
            FeatureTransformStep::Psd(SpectrumFeatureTransform {
                id: "psd".to_string(),
                transform_name: "psd",
                channel: "input_v".to_string(),
                window: WindowSpec::default(),
            }),
            FeatureTransformStep::SpectralCentroid(SpectrumFeatureTransform {
                id: "centroid".to_string(),
                transform_name: "spectral_centroid",
                channel: "input_v".to_string(),
                window: WindowSpec::default(),
            }),
            FeatureTransformStep::SpectralBandwidth(SpectrumFeatureTransform {
                id: "bandwidth".to_string(),
                transform_name: "spectral_bandwidth",
                channel: "input_v".to_string(),
                window: WindowSpec::default(),
            }),
            FeatureTransformStep::SpectralRolloff(SpectralRolloffFeatureTransform {
                id: "rolloff".to_string(),
                channel: "input_v".to_string(),
                window: WindowSpec::default(),
                rolloff_percent: 85.0,
            }),
            FeatureTransformStep::BandPower(BandPowerFeatureTransform {
                id: "band_power".to_string(),
                channel: "input_v".to_string(),
                window: WindowSpec::default(),
                band_low_hz: 0.5,
                band_high_hz: 1.5,
            }),
        ];

        let records =
            evaluate_feature_transforms(&waveform, &features).expect("features should evaluate");

        assert_eq!(record(&records, "hann_sample_0").value, 0.0);
        assert_close(record(&records, "hann_sample_1").value, 0.75);
        assert_close(record(&records, "hann_sample_2").value, 0.75);
        assert_eq!(record(&records, "hann_sample_3").value, 0.0);
        assert_close(record(&records, "dft_bin_1").value, 1.0);
        assert_close(record(&records, "fft_bin_1").value, 1.0);
        assert_close(record(&records, "power_bin_1").value, 0.5);
        assert_close(record(&records, "psd_bin_1").value, 0.5);
        assert_close(record(&records, "centroid").value, 1.0);
        assert_close(record(&records, "bandwidth").value, 0.0);
        assert_close(record(&records, "rolloff").value, 1.0);
        assert_close(record(&records, "band_power").value, 0.5);

        let fft = record(&records, "fft_bin_1");
        assert_eq!(fft.method_context.bin_index, Some(1));
        assert_eq!(fft.method_context.frequency_hz, Some(1.0));
        assert_eq!(fft.method_context.bin_width_hz, Some(1.0));
        assert_eq!(fft.method_context.window.as_deref(), Some("rectangular"));

        assert_catalog_matches(&records);
    }

    #[test]
    fn evaluates_m33_paired_harmonic_and_time_frequency_features() {
        let waveform = m33_waveform();
        let features = vec![
            FeatureTransformStep::WelchPsd(WelchPsdFeatureTransform {
                id: "welch".to_string(),
                channel: "input_v".to_string(),
                window: WindowSpec::default(),
                window_samples: 8,
                overlap_samples: 0,
            }),
            FeatureTransformStep::CrossSpectrum(PairedSpectrumFeatureTransform {
                id: "cross".to_string(),
                transform_name: "cross_spectrum",
                channel: "input_v".to_string(),
                other_channel: "other_v".to_string(),
                window: WindowSpec::default(),
                window_samples: None,
                overlap_samples: None,
            }),
            FeatureTransformStep::Coherence(PairedSpectrumFeatureTransform {
                id: "coherence".to_string(),
                transform_name: "coherence",
                channel: "input_v".to_string(),
                other_channel: "other_v".to_string(),
                window: WindowSpec::default(),
                window_samples: None,
                overlap_samples: None,
            }),
            FeatureTransformStep::TransferFunction(PairedSpectrumFeatureTransform {
                id: "transfer".to_string(),
                transform_name: "transfer_function",
                channel: "input_v".to_string(),
                other_channel: "other_v".to_string(),
                window: WindowSpec::default(),
                window_samples: None,
                overlap_samples: None,
            }),
            FeatureTransformStep::HarmonicAnalysis(HarmonicFeatureTransform {
                id: "harmonic".to_string(),
                channel: "harmonic_v".to_string(),
                window: WindowSpec::default(),
                fundamental_hz: Some(1.0),
                harmonic_count: 3,
            }),
            FeatureTransformStep::Thd(HarmonicMetricFeatureTransform {
                id: "thd".to_string(),
                transform_name: "thd",
                channel: "harmonic_v".to_string(),
                window: WindowSpec::default(),
                fundamental_hz: Some(1.0),
                harmonic_count: 3,
            }),
            FeatureTransformStep::Snr(HarmonicMetricFeatureTransform {
                id: "snr".to_string(),
                transform_name: "snr",
                channel: "noisy_v".to_string(),
                window: WindowSpec::default(),
                fundamental_hz: Some(1.0),
                harmonic_count: 1,
            }),
            FeatureTransformStep::Sinad(HarmonicMetricFeatureTransform {
                id: "sinad".to_string(),
                transform_name: "sinad",
                channel: "noisy_v".to_string(),
                window: WindowSpec::default(),
                fundamental_hz: Some(1.0),
                harmonic_count: 1,
            }),
            FeatureTransformStep::Enob(HarmonicMetricFeatureTransform {
                id: "enob".to_string(),
                transform_name: "enob",
                channel: "noisy_v".to_string(),
                window: WindowSpec::default(),
                fundamental_hz: Some(1.0),
                harmonic_count: 1,
            }),
            FeatureTransformStep::Stft(TimeFrequencyFeatureTransform {
                id: "stft".to_string(),
                transform_name: "stft",
                channel: "input_v".to_string(),
                window: WindowSpec::default(),
                window_samples: 8,
                overlap_samples: 0,
            }),
            FeatureTransformStep::Spectrogram(TimeFrequencyFeatureTransform {
                id: "spectrogram".to_string(),
                transform_name: "spectrogram",
                channel: "input_v".to_string(),
                window: WindowSpec::default(),
                window_samples: 8,
                overlap_samples: 0,
            }),
            FeatureTransformStep::Ifft(IfftFeatureTransform {
                id: "ifft".to_string(),
                channel: "constant_spectrum_real".to_string(),
                other_channel: Some("constant_spectrum_imag".to_string()),
            }),
        ];

        let records =
            evaluate_feature_transforms(&waveform, &features).expect("features should evaluate");

        assert_close(record(&records, "welch_bin_1").value, 0.5);
        assert_close(record(&records, "cross_bin_1").value, 1.0);
        assert_close(record(&records, "coherence_bin_1").value, 1.0);
        assert_close(record(&records, "transfer_bin_1").value, 2.0);
        assert_close(record(&records, "harmonic_harmonic_1").value, 1.0);
        assert_close(record(&records, "harmonic_harmonic_2").value, 0.25);
        assert_close(record(&records, "thd").value, 0.25);
        assert_close(record(&records, "snr").value, 20.0);
        assert_close(record(&records, "sinad").value, 20.0);
        assert_close(record(&records, "enob").value, (20.0 - 1.76) / 6.02);
        assert_close(record(&records, "stft_segment_0_bin_1").value, 1.0);
        assert_close(record(&records, "spectrogram_segment_0_bin_1").value, 0.5);
        assert_close(record(&records, "ifft_sample_0").value, 1.0);
        assert_close(record(&records, "ifft_sample_1").value, 1.0);

        let spectrogram = record(&records, "spectrogram_segment_0_bin_1");
        assert_eq!(spectrogram.method_context.segment_index, Some(0));
        assert_eq!(spectrogram.method_context.segment_start_s, Some(0.0));
        assert_eq!(spectrogram.method_context.segment_end_s, Some(0.875));
        assert_eq!(spectrogram.method_context.window_samples, Some(8));

        assert_catalog_matches(&records);
    }

    #[test]
    fn rejects_invalid_m33_spectrum_feature_inputs() {
        let waveform = m33_waveform();
        assert!(matches!(
            WindowSpec::from_config(Some("not_a_window"), None, None, None),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            evaluate_feature_transforms(
                &waveform,
                &[FeatureTransformStep::WelchPsd(WelchPsdFeatureTransform {
                    id: "bad".to_string(),
                    channel: "input_v".to_string(),
                    window: WindowSpec::default(),
                    window_samples: 4,
                    overlap_samples: 4,
                })],
            ),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            evaluate_feature_transforms(
                &waveform,
                &[FeatureTransformStep::BandPower(BandPowerFeatureTransform {
                    id: "bad".to_string(),
                    channel: "input_v".to_string(),
                    window: WindowSpec::default(),
                    band_low_hz: 1.0,
                    band_high_hz: 5.0,
                })],
            ),
            Err(WaveformError::InvalidParameter { .. })
        ));
        let no_rate = Waveform::new(
            vec![0.0],
            vec![Channel::new("input_v", Unit::volts(), vec![1.0])],
        )
        .expect("single-sample waveform is valid");
        assert!(matches!(
            evaluate_feature_transforms(
                &no_rate,
                &[FeatureTransformStep::Fft(SpectrumFeatureTransform {
                    id: "bad".to_string(),
                    transform_name: "fft",
                    channel: "input_v".to_string(),
                    window: WindowSpec::default(),
                })],
            ),
            Err(WaveformError::InvalidWaveform { .. })
        ));
    }

    #[test]
    fn rejects_unknown_feature_channel() {
        let transform = FeatureTransformStep::Rms(RmsFeatureTransform {
            id: "rms".to_string(),
            channel: "missing".to_string(),
        });

        assert!(matches!(
            evaluate_feature_transforms(&waveform(), &[transform]),
            Err(WaveformError::InvalidParameter { .. })
        ));
    }

    #[test]
    fn rejects_zero_rms_crest_factor() {
        let waveform = Waveform::new(
            vec![0.0, 1.0],
            vec![Channel::new("input_v", Unit::volts(), vec![0.0, 0.0])],
        )
        .expect("waveform should be valid");
        let transform = FeatureTransformStep::CrestFactor(CrestFactorFeatureTransform {
            id: "crest".to_string(),
            channel: "input_v".to_string(),
        });

        assert!(matches!(
            evaluate_feature_transforms(&waveform, &[transform]),
            Err(WaveformError::InvalidWaveform { .. })
        ));
    }

    #[test]
    fn rejects_invalid_m32_statistics_feature_inputs() {
        let waveform = waveform();
        assert!(matches!(
            evaluate_feature_transforms(
                &waveform,
                &[FeatureTransformStep::Percentile(
                    PercentileFeatureTransform {
                        id: "bad".to_string(),
                        channel: "input_v".to_string(),
                        percentile: 101.0,
                    }
                )],
            ),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            evaluate_feature_transforms(
                &waveform,
                &[FeatureTransformStep::Histogram(HistogramFeatureTransform {
                    id: "bad".to_string(),
                    channel: "input_v".to_string(),
                    bins: 2,
                    min_v: Some(5.0),
                    max_v: Some(1.0),
                })],
            ),
            Err(WaveformError::InvalidParameter { .. })
        ));

        let constant = Waveform::new(
            vec![0.0, 1.0, 2.0],
            vec![
                Channel::new("input_v", Unit::volts(), vec![1.0, 1.0, 1.0]),
                Channel::new("other_v", Unit::volts(), vec![2.0, 3.0, 4.0]),
            ],
        )
        .expect("waveform should be valid");
        assert!(matches!(
            evaluate_feature_transforms(
                &constant,
                &[FeatureTransformStep::Correlation(
                    CorrelationFeatureTransform {
                        id: "bad".to_string(),
                        channel: "input_v".to_string(),
                        other_channel: "other_v".to_string(),
                    }
                )],
            ),
            Err(WaveformError::InvalidWaveform { .. })
        ));
        assert!(matches!(
            evaluate_feature_transforms(
                &waveform,
                &[FeatureTransformStep::Autocorrelation(
                    AutocorrelationFeatureTransform {
                        id: "bad".to_string(),
                        channel: "input_v".to_string(),
                        lag_samples: 3,
                    },
                )],
            ),
            Err(WaveformError::InvalidParameter { .. })
        ));
    }

    fn assert_record_value(records: &[FeatureRecord], id: &str, expected: f64) {
        assert_close(record(records, id).value, expected);
    }

    fn m33_waveform() -> Waveform {
        let time = (0..8).map(|index| index as f64 / 8.0).collect::<Vec<_>>();
        let sine = (0..8)
            .map(|index| (2.0 * PI * index as f64 / 8.0).sin())
            .collect::<Vec<_>>();
        let harmonic = (0..8)
            .map(|index| {
                (2.0 * PI * index as f64 / 8.0).sin() + 0.25 * (4.0 * PI * index as f64 / 8.0).sin()
            })
            .collect::<Vec<_>>();
        let noisy = (0..8)
            .map(|index| {
                (2.0 * PI * index as f64 / 8.0).sin() + 0.1 * (6.0 * PI * index as f64 / 8.0).cos()
            })
            .collect::<Vec<_>>();
        let square = vec![1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0];
        Waveform::new(
            time,
            vec![
                Channel::new("input_v", Unit::volts(), sine.clone()),
                Channel::new(
                    "other_v",
                    Unit::volts(),
                    sine.iter().map(|sample| sample * 2.0).collect(),
                ),
                Channel::new("harmonic_v", Unit::volts(), harmonic),
                Channel::new("noisy_v", Unit::volts(), noisy),
                Channel::new("square_v", Unit::volts(), square),
                Channel::new(
                    "constant_spectrum_real",
                    Unit::volts(),
                    vec![8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                ),
                Channel::new(
                    "constant_spectrum_imag",
                    Unit::volts(),
                    vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                ),
            ],
        )
        .expect("m33 waveform should be valid")
    }

    fn assert_catalog_matches(records: &[FeatureRecord]) {
        for record in records {
            let entry = transform_catalog_entry(&record.transform)
                .expect("feature record should be cataloged");
            assert!(
                entry.matches_step_metadata(&record.transform_metadata),
                "metadata for `{}` does not match catalog entry: {:?} vs {:?}",
                record.transform,
                record.transform_metadata,
                entry
            );
        }
    }

    fn record<'a>(records: &'a [FeatureRecord], id: &str) -> &'a FeatureRecord {
        records
            .iter()
            .find(|record| record.id == id)
            .unwrap_or_else(|| panic!("missing feature record `{id}`"))
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1.0e-9,
            "expected {expected}, got {actual}"
        );
    }
}
