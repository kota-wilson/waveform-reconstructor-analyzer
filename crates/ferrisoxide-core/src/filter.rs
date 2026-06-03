use crate::error::{Result, WaveformError};
use crate::model::{
    Channel, TransformCategory, TransformExecutionMetadata, TransformOutputChannels,
    TransformParameterMetadata, TransformPhaseEffect, TransformStepMetadata, Unit, Waveform,
};
use crate::transform_catalog::{transform_catalog_entry, TransformCatalogEntry};

const TAU: f64 = std::f64::consts::PI * 2.0;
const MAX_ADC_BITS: u8 = 24;
const MAX_RESAMPLED_SAMPLES: usize = 1_000_000;

pub trait Filter {
    fn name(&self) -> &'static str;
    fn apply(&self, waveform: &Waveform) -> Result<Waveform>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum FilterStep {
    NanInterpolate(NanInterpolateTransform),
    NanRemove(NanRemoveTransform),
    TimestampSort(TimestampSortTransform),
    DedupeTimestamps(DedupeTimestampsTransform),
    Crop(CropTransform),
    FixedDelay(FixedDelayTransform),
    GapFill(GapFillTransform),
    ResampleFixed(ResampleFixedTransform),
    ChannelDelay(ChannelDelayTransform),
    Resample(ResampleTransform),
    Downsample(DownsampleTransform),
    Decimate(DecimateTransform),
    Upsample(UpsampleTransform),
    Interpolate(InterpolateTransform),
    RationalResample(RationalResampleTransform),
    SampleAndHold(SampleAndHoldTransform),
    ZeroOrderHold(ZeroOrderHoldTransform),
    FirstOrderHold(FirstOrderHoldTransform),
    FractionalDelay(FractionalDelayTransform),
    CrossCorrelationDelay(CrossCorrelationDelayTransform),
    JitterCorrection(JitterCorrectionTransform),
    ClockDriftCorrection(ClockDriftCorrectionTransform),
    HalfWaveRectify(HalfWaveRectifyTransform),
    FullWaveRectify(FullWaveRectifyTransform),
    Envelope(EnvelopeTransform),
    MovingRms(MovingRmsTransform),
    PeakHold(PeakHoldTransform),
    FirstDerivative(FirstDerivativeTransform),
    SecondDerivative(SecondDerivativeTransform),
    Integral(IntegralTransform),
    CumulativeIntegral(CumulativeIntegralTransform),
    LeakyIntegrator(LeakyIntegratorTransform),
    SlopeDetection(SlopeDetectionTransform),
    RollingMean(RollingMeanTransform),
    RollingVariance(RollingVarianceTransform),
    RollingStdDev(RollingStdDevTransform),
    RollingMin(RollingMinTransform),
    RollingMax(RollingMaxTransform),
    ZScore(ZScoreTransform),
    OutlierDetection(OutlierDetectionTransform),
    QuantileClip(QuantileClipTransform),
    NoiseInjection(NoiseInjectionTransform),
    PeriodicInterference(PeriodicInterferenceTransform),
    DriftFault(DriftFaultTransform),
    SampleFault(SampleFaultTransform),
    SimulationQuantizer(SimulationQuantizerTransform),
    Dither(DitherTransform),
    Companding(CompandingTransform),
    SampleClockJitter(SampleClockJitterTransform),
    AdcCodeDefect(AdcCodeDefectTransform),
    GainOffsetError(GainOffsetErrorTransform),
    ChannelArithmetic(ChannelArithmeticTransform),
    VectorMagnitude(VectorMagnitudeTransform),
    MatrixTransform(MatrixTransform),
    CoordinateRotation(CoordinateRotationTransform),
    SensorConversion(Box<SensorConversionTransform>),
    VibrationTransform(VibrationTransform),
    ControlTransform(ControlTransform),
    AbsoluteValue(AbsoluteValueTransform),
    Square(SquareTransform),
    SquareRoot(SquareRootTransform),
    Log(LogTransform),
    Exp(ExpTransform),
    Normalize(NormalizeTransform),
    Tanh(TanhTransform),
    Sigmoid(SigmoidTransform),
    SoftLimit(SoftLimitTransform),
    PiecewiseLinear(PiecewiseLinearTransform),
    Polynomial(PolynomialTransform),
    WeightedMovingAverage(WeightedMovingAverageFilter),
    ExponentialMovingAverage(ExponentialMovingAverageFilter),
    BoxcarSmoothing(BoxcarSmoothingFilter),
    GaussianSmoothing(GaussianSmoothingFilter),
    SavitzkyGolay(SavitzkyGolayFilter),
    CenteredMovingMedian(CenteredMovingMedianFilter),
    RollingMeanBaseline(RollingMeanBaselineTransform),
    RollingMedianBaseline(RollingMedianBaselineTransform),
    LinearDetrend(LinearDetrendTransform),
    PolynomialDetrend(PolynomialDetrendTransform),
    HampelFilter(HampelFilter),
    SpikeRemove(SpikeRemoveTransform),
    FirFilter(FirFilter),
    ZeroPhaseFirFilter(ZeroPhaseFirFilter),
    IirBiquad(IirBiquadFilter),
    ZeroPhaseIirBiquad(ZeroPhaseIirBiquadFilter),
    HighPass(HighPassFilter),
    BandPass(BandPassFilter),
    BandStop(BandStopFilter),
    Notch(NotchFilter),
    CombFilter(CombFilter),
    ButterworthLowPass(ButterworthLowPassFilter),
    ButterworthHighPass(ButterworthHighPassFilter),
    Chebyshev1LowPass(Chebyshev1LowPassFilter),
    Chebyshev2LowPass(Chebyshev2LowPassFilter),
    BesselLowPass(BesselLowPassFilter),
    Offset(OffsetTransform),
    Gain(GainTransform),
    Invert(InvertTransform),
    Clamp(ClampTransform),
    Deadband(DeadbandTransform),
    DcRemove(DcRemoveTransform),
    BaselineSubtract(BaselineSubtractTransform),
    HighPassBaseline(HighPassBaselineFilter),
    MovingAverage(MovingAverageFilter),
    MovingMedian(MovingMedianFilter),
    LowPass(LowPassFilter),
    AdcQuantize(AdcQuantizer),
    ChannelScoped(ChannelScopedFilter),
}

impl FilterStep {
    pub fn sensor_conversion(filter: SensorConversionTransform) -> Self {
        Self::SensorConversion(Box::new(filter))
    }

    pub fn channel_scoped(channel: String, inner: FilterStep) -> Self {
        Self::ChannelScoped(ChannelScopedFilter {
            channel,
            inner: Box::new(inner),
        })
    }

    pub fn supports_channel_scoping(&self) -> bool {
        matches!(
            self,
            Self::HalfWaveRectify(_)
                | Self::FullWaveRectify(_)
                | Self::Envelope(_)
                | Self::MovingRms(_)
                | Self::PeakHold(_)
                | Self::FirstDerivative(_)
                | Self::SecondDerivative(_)
                | Self::Integral(_)
                | Self::CumulativeIntegral(_)
                | Self::LeakyIntegrator(_)
                | Self::SlopeDetection(_)
                | Self::RollingMean(_)
                | Self::RollingVariance(_)
                | Self::RollingStdDev(_)
                | Self::RollingMin(_)
                | Self::RollingMax(_)
                | Self::ZScore(_)
                | Self::OutlierDetection(_)
                | Self::QuantileClip(_)
                | Self::AbsoluteValue(_)
                | Self::Square(_)
                | Self::SquareRoot(_)
                | Self::Log(_)
                | Self::Exp(_)
                | Self::Normalize(_)
                | Self::Tanh(_)
                | Self::Sigmoid(_)
                | Self::SoftLimit(_)
                | Self::WeightedMovingAverage(_)
                | Self::ExponentialMovingAverage(_)
                | Self::BoxcarSmoothing(_)
                | Self::GaussianSmoothing(_)
                | Self::SavitzkyGolay(_)
                | Self::CenteredMovingMedian(_)
                | Self::RollingMeanBaseline(_)
                | Self::RollingMedianBaseline(_)
                | Self::LinearDetrend(_)
                | Self::PolynomialDetrend(_)
                | Self::HampelFilter(_)
                | Self::SpikeRemove(_)
                | Self::FirFilter(_)
                | Self::ZeroPhaseFirFilter(_)
                | Self::IirBiquad(_)
                | Self::ZeroPhaseIirBiquad(_)
                | Self::HighPass(_)
                | Self::BandPass(_)
                | Self::BandStop(_)
                | Self::Notch(_)
                | Self::CombFilter(_)
                | Self::ButterworthLowPass(_)
                | Self::ButterworthHighPass(_)
                | Self::Chebyshev1LowPass(_)
                | Self::Chebyshev2LowPass(_)
                | Self::BesselLowPass(_)
                | Self::Offset(_)
                | Self::Gain(_)
                | Self::Invert(_)
                | Self::Clamp(_)
                | Self::Deadband(_)
                | Self::DcRemove(_)
                | Self::BaselineSubtract(_)
                | Self::HighPassBaseline(_)
                | Self::MovingAverage(_)
                | Self::MovingMedian(_)
                | Self::LowPass(_)
                | Self::AdcQuantize(_)
        )
    }
}

impl Filter for FilterStep {
    fn name(&self) -> &'static str {
        match self {
            Self::NanInterpolate(filter) => filter.name(),
            Self::NanRemove(filter) => filter.name(),
            Self::TimestampSort(filter) => filter.name(),
            Self::DedupeTimestamps(filter) => filter.name(),
            Self::Crop(filter) => filter.name(),
            Self::FixedDelay(filter) => filter.name(),
            Self::GapFill(filter) => filter.name(),
            Self::ResampleFixed(filter) => filter.name(),
            Self::ChannelDelay(filter) => filter.name(),
            Self::Resample(filter) => filter.name(),
            Self::Downsample(filter) => filter.name(),
            Self::Decimate(filter) => filter.name(),
            Self::Upsample(filter) => filter.name(),
            Self::Interpolate(filter) => filter.name(),
            Self::RationalResample(filter) => filter.name(),
            Self::SampleAndHold(filter) => filter.name(),
            Self::ZeroOrderHold(filter) => filter.name(),
            Self::FirstOrderHold(filter) => filter.name(),
            Self::FractionalDelay(filter) => filter.name(),
            Self::CrossCorrelationDelay(filter) => filter.name(),
            Self::JitterCorrection(filter) => filter.name(),
            Self::ClockDriftCorrection(filter) => filter.name(),
            Self::HalfWaveRectify(filter) => filter.name(),
            Self::FullWaveRectify(filter) => filter.name(),
            Self::Envelope(filter) => filter.name(),
            Self::MovingRms(filter) => filter.name(),
            Self::PeakHold(filter) => filter.name(),
            Self::FirstDerivative(filter) => filter.name(),
            Self::SecondDerivative(filter) => filter.name(),
            Self::Integral(filter) => filter.name(),
            Self::CumulativeIntegral(filter) => filter.name(),
            Self::LeakyIntegrator(filter) => filter.name(),
            Self::SlopeDetection(filter) => filter.name(),
            Self::RollingMean(filter) => filter.name(),
            Self::RollingVariance(filter) => filter.name(),
            Self::RollingStdDev(filter) => filter.name(),
            Self::RollingMin(filter) => filter.name(),
            Self::RollingMax(filter) => filter.name(),
            Self::ZScore(filter) => filter.name(),
            Self::OutlierDetection(filter) => filter.name(),
            Self::QuantileClip(filter) => filter.name(),
            Self::NoiseInjection(filter) => filter.name(),
            Self::PeriodicInterference(filter) => filter.name(),
            Self::DriftFault(filter) => filter.name(),
            Self::SampleFault(filter) => filter.name(),
            Self::SimulationQuantizer(filter) => filter.name(),
            Self::Dither(filter) => filter.name(),
            Self::Companding(filter) => filter.name(),
            Self::SampleClockJitter(filter) => filter.name(),
            Self::AdcCodeDefect(filter) => filter.name(),
            Self::GainOffsetError(filter) => filter.name(),
            Self::ChannelArithmetic(filter) => filter.name(),
            Self::VectorMagnitude(filter) => filter.name(),
            Self::MatrixTransform(filter) => filter.name(),
            Self::CoordinateRotation(filter) => filter.name(),
            Self::SensorConversion(filter) => filter.name(),
            Self::VibrationTransform(filter) => filter.name(),
            Self::ControlTransform(filter) => filter.name(),
            Self::AbsoluteValue(filter) => filter.name(),
            Self::Square(filter) => filter.name(),
            Self::SquareRoot(filter) => filter.name(),
            Self::Log(filter) => filter.name(),
            Self::Exp(filter) => filter.name(),
            Self::Normalize(filter) => filter.name(),
            Self::Tanh(filter) => filter.name(),
            Self::Sigmoid(filter) => filter.name(),
            Self::SoftLimit(filter) => filter.name(),
            Self::PiecewiseLinear(filter) => filter.name(),
            Self::Polynomial(filter) => filter.name(),
            Self::WeightedMovingAverage(filter) => filter.name(),
            Self::ExponentialMovingAverage(filter) => filter.name(),
            Self::BoxcarSmoothing(filter) => filter.name(),
            Self::GaussianSmoothing(filter) => filter.name(),
            Self::SavitzkyGolay(filter) => filter.name(),
            Self::CenteredMovingMedian(filter) => filter.name(),
            Self::RollingMeanBaseline(filter) => filter.name(),
            Self::RollingMedianBaseline(filter) => filter.name(),
            Self::LinearDetrend(filter) => filter.name(),
            Self::PolynomialDetrend(filter) => filter.name(),
            Self::HampelFilter(filter) => filter.name(),
            Self::SpikeRemove(filter) => filter.name(),
            Self::FirFilter(filter) => filter.name(),
            Self::ZeroPhaseFirFilter(filter) => filter.name(),
            Self::IirBiquad(filter) => filter.name(),
            Self::ZeroPhaseIirBiquad(filter) => filter.name(),
            Self::HighPass(filter) => filter.name(),
            Self::BandPass(filter) => filter.name(),
            Self::BandStop(filter) => filter.name(),
            Self::Notch(filter) => filter.name(),
            Self::CombFilter(filter) => filter.name(),
            Self::ButterworthLowPass(filter) => filter.name(),
            Self::ButterworthHighPass(filter) => filter.name(),
            Self::Chebyshev1LowPass(filter) => filter.name(),
            Self::Chebyshev2LowPass(filter) => filter.name(),
            Self::BesselLowPass(filter) => filter.name(),
            Self::Offset(filter) => filter.name(),
            Self::Gain(filter) => filter.name(),
            Self::Invert(filter) => filter.name(),
            Self::Clamp(filter) => filter.name(),
            Self::Deadband(filter) => filter.name(),
            Self::DcRemove(filter) => filter.name(),
            Self::BaselineSubtract(filter) => filter.name(),
            Self::HighPassBaseline(filter) => filter.name(),
            Self::MovingAverage(filter) => filter.name(),
            Self::MovingMedian(filter) => filter.name(),
            Self::LowPass(filter) => filter.name(),
            Self::AdcQuantize(filter) => filter.name(),
            Self::ChannelScoped(filter) => filter.name(),
        }
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        match self {
            Self::NanInterpolate(filter) => filter.apply(waveform),
            Self::NanRemove(filter) => filter.apply(waveform),
            Self::TimestampSort(filter) => filter.apply(waveform),
            Self::DedupeTimestamps(filter) => filter.apply(waveform),
            Self::Crop(filter) => filter.apply(waveform),
            Self::FixedDelay(filter) => filter.apply(waveform),
            Self::GapFill(filter) => filter.apply(waveform),
            Self::ResampleFixed(filter) => filter.apply(waveform),
            Self::ChannelDelay(filter) => filter.apply(waveform),
            Self::Resample(filter) => filter.apply(waveform),
            Self::Downsample(filter) => filter.apply(waveform),
            Self::Decimate(filter) => filter.apply(waveform),
            Self::Upsample(filter) => filter.apply(waveform),
            Self::Interpolate(filter) => filter.apply(waveform),
            Self::RationalResample(filter) => filter.apply(waveform),
            Self::SampleAndHold(filter) => filter.apply(waveform),
            Self::ZeroOrderHold(filter) => filter.apply(waveform),
            Self::FirstOrderHold(filter) => filter.apply(waveform),
            Self::FractionalDelay(filter) => filter.apply(waveform),
            Self::CrossCorrelationDelay(filter) => filter.apply(waveform),
            Self::JitterCorrection(filter) => filter.apply(waveform),
            Self::ClockDriftCorrection(filter) => filter.apply(waveform),
            Self::HalfWaveRectify(filter) => filter.apply(waveform),
            Self::FullWaveRectify(filter) => filter.apply(waveform),
            Self::Envelope(filter) => filter.apply(waveform),
            Self::MovingRms(filter) => filter.apply(waveform),
            Self::PeakHold(filter) => filter.apply(waveform),
            Self::FirstDerivative(filter) => filter.apply(waveform),
            Self::SecondDerivative(filter) => filter.apply(waveform),
            Self::Integral(filter) => filter.apply(waveform),
            Self::CumulativeIntegral(filter) => filter.apply(waveform),
            Self::LeakyIntegrator(filter) => filter.apply(waveform),
            Self::SlopeDetection(filter) => filter.apply(waveform),
            Self::RollingMean(filter) => filter.apply(waveform),
            Self::RollingVariance(filter) => filter.apply(waveform),
            Self::RollingStdDev(filter) => filter.apply(waveform),
            Self::RollingMin(filter) => filter.apply(waveform),
            Self::RollingMax(filter) => filter.apply(waveform),
            Self::ZScore(filter) => filter.apply(waveform),
            Self::OutlierDetection(filter) => filter.apply(waveform),
            Self::QuantileClip(filter) => filter.apply(waveform),
            Self::NoiseInjection(filter) => filter.apply(waveform),
            Self::PeriodicInterference(filter) => filter.apply(waveform),
            Self::DriftFault(filter) => filter.apply(waveform),
            Self::SampleFault(filter) => filter.apply(waveform),
            Self::SimulationQuantizer(filter) => filter.apply(waveform),
            Self::Dither(filter) => filter.apply(waveform),
            Self::Companding(filter) => filter.apply(waveform),
            Self::SampleClockJitter(filter) => filter.apply(waveform),
            Self::AdcCodeDefect(filter) => filter.apply(waveform),
            Self::GainOffsetError(filter) => filter.apply(waveform),
            Self::ChannelArithmetic(filter) => filter.apply(waveform),
            Self::VectorMagnitude(filter) => filter.apply(waveform),
            Self::MatrixTransform(filter) => filter.apply(waveform),
            Self::CoordinateRotation(filter) => filter.apply(waveform),
            Self::SensorConversion(filter) => filter.apply(waveform),
            Self::VibrationTransform(filter) => filter.apply(waveform),
            Self::ControlTransform(filter) => filter.apply(waveform),
            Self::AbsoluteValue(filter) => filter.apply(waveform),
            Self::Square(filter) => filter.apply(waveform),
            Self::SquareRoot(filter) => filter.apply(waveform),
            Self::Log(filter) => filter.apply(waveform),
            Self::Exp(filter) => filter.apply(waveform),
            Self::Normalize(filter) => filter.apply(waveform),
            Self::Tanh(filter) => filter.apply(waveform),
            Self::Sigmoid(filter) => filter.apply(waveform),
            Self::SoftLimit(filter) => filter.apply(waveform),
            Self::PiecewiseLinear(filter) => filter.apply(waveform),
            Self::Polynomial(filter) => filter.apply(waveform),
            Self::WeightedMovingAverage(filter) => filter.apply(waveform),
            Self::ExponentialMovingAverage(filter) => filter.apply(waveform),
            Self::BoxcarSmoothing(filter) => filter.apply(waveform),
            Self::GaussianSmoothing(filter) => filter.apply(waveform),
            Self::SavitzkyGolay(filter) => filter.apply(waveform),
            Self::CenteredMovingMedian(filter) => filter.apply(waveform),
            Self::RollingMeanBaseline(filter) => filter.apply(waveform),
            Self::RollingMedianBaseline(filter) => filter.apply(waveform),
            Self::LinearDetrend(filter) => filter.apply(waveform),
            Self::PolynomialDetrend(filter) => filter.apply(waveform),
            Self::HampelFilter(filter) => filter.apply(waveform),
            Self::SpikeRemove(filter) => filter.apply(waveform),
            Self::FirFilter(filter) => filter.apply(waveform),
            Self::ZeroPhaseFirFilter(filter) => filter.apply(waveform),
            Self::IirBiquad(filter) => filter.apply(waveform),
            Self::ZeroPhaseIirBiquad(filter) => filter.apply(waveform),
            Self::HighPass(filter) => filter.apply(waveform),
            Self::BandPass(filter) => filter.apply(waveform),
            Self::BandStop(filter) => filter.apply(waveform),
            Self::Notch(filter) => filter.apply(waveform),
            Self::CombFilter(filter) => filter.apply(waveform),
            Self::ButterworthLowPass(filter) => filter.apply(waveform),
            Self::ButterworthHighPass(filter) => filter.apply(waveform),
            Self::Chebyshev1LowPass(filter) => filter.apply(waveform),
            Self::Chebyshev2LowPass(filter) => filter.apply(waveform),
            Self::BesselLowPass(filter) => filter.apply(waveform),
            Self::Offset(filter) => filter.apply(waveform),
            Self::Gain(filter) => filter.apply(waveform),
            Self::Invert(filter) => filter.apply(waveform),
            Self::Clamp(filter) => filter.apply(waveform),
            Self::Deadband(filter) => filter.apply(waveform),
            Self::DcRemove(filter) => filter.apply(waveform),
            Self::BaselineSubtract(filter) => filter.apply(waveform),
            Self::HighPassBaseline(filter) => filter.apply(waveform),
            Self::MovingAverage(filter) => filter.apply(waveform),
            Self::MovingMedian(filter) => filter.apply(waveform),
            Self::LowPass(filter) => filter.apply(waveform),
            Self::AdcQuantize(filter) => filter.apply(waveform),
            Self::ChannelScoped(filter) => filter.apply(waveform),
        }
    }
}

impl FilterStep {
    pub fn catalog_entry(&self) -> Option<&'static TransformCatalogEntry> {
        transform_catalog_entry(self.name())
    }

    pub fn rule_package_export_supported(&self) -> bool {
        if matches!(self, Self::ChannelScoped(_)) {
            return false;
        }
        self.catalog_entry()
            .is_some_and(|entry| entry.supports_rule_package())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelScopedFilter {
    pub channel: String,
    pub inner: Box<FilterStep>,
}

impl Filter for ChannelScopedFilter {
    fn name(&self) -> &'static str {
        self.inner.name()
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_required_channel_name("channel", &self.channel)?;
        let channel_index = waveform
            .channels
            .iter()
            .position(|channel| channel.name == self.channel)
            .ok_or_else(|| WaveformError::InvalidParameter {
                name: "channel".to_string(),
                reason: format!("channel `{}` was not found", self.channel),
            })?;
        let target_channel = waveform.channels[channel_index].clone();
        let scoped_waveform = Waveform::new_with_time_unit(
            waveform.time.clone(),
            waveform.time_unit.clone(),
            vec![target_channel],
        )?;
        let filtered = self.inner.apply(&scoped_waveform)?;
        if filtered.time != waveform.time {
            return Err(WaveformError::InvalidParameter {
                name: "filters.channel".to_string(),
                reason: format!(
                    "filter type `{}` changes the time axis and cannot be channel-scoped",
                    self.inner.name()
                ),
            });
        }
        if filtered.channels.len() != 1 || filtered.channels[0].samples.len() != waveform.time.len()
        {
            return Err(WaveformError::InvalidParameter {
                name: "filters.channel".to_string(),
                reason: format!(
                    "filter type `{}` changes channel shape and cannot be channel-scoped",
                    self.inner.name()
                ),
            });
        }

        let mut channels = waveform.channels.clone();
        channels[channel_index] = filtered.channels[0].clone();
        let mut transform_step = filtered
            .metadata
            .transform_steps
            .last()
            .cloned()
            .unwrap_or_else(|| {
                TransformStepMetadata::implemented_desktop_with_execution(
                    format!("{}(channel={})", self.inner.name(), self.channel),
                    self.inner.name(),
                    TransformCategory::Pointwise,
                    Vec::new(),
                    offline_execution(false, false, TransformPhaseEffect::None),
                )
            });
        transform_step.history_label = format!(
            "{} scoped to {}",
            transform_step.history_label, self.channel
        );
        transform_step
            .parameters
            .push(TransformParameterMetadata::text(
                "channel",
                self.channel.clone(),
            ));
        derived_waveform(waveform, channels, transform_step)
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
pub struct NanInterpolateTransform;

impl Filter for NanInterpolateTransform {
    fn name(&self) -> &'static str {
        "nan_interpolate"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_time_axis(&waveform.time, "NaN interpolation")?;
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_no_infinite_samples("nan_interpolate", &channel.samples)?;
                let samples = interpolate_nan_samples(&waveform.time, &channel.samples)?;
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            "nan_interpolate()",
            "nan_interpolate",
            TransformCategory::DataCleaning,
            Vec::new(),
            offline_execution(true, false, TransformPhaseEffect::None),
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NanRemoveTransform;

impl Filter for NanRemoveTransform {
    fn name(&self) -> &'static str {
        "nan_remove"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_finite_time_axis(&waveform.time, "NaN removal")?;
        validate_channels_allowing_nan(waveform, "nan_remove")?;

        let keep_indices = waveform
            .time
            .iter()
            .enumerate()
            .filter_map(|(index, _)| {
                let has_nan = waveform
                    .channels
                    .iter()
                    .any(|channel| channel.samples[index].is_nan());
                (!has_nan).then_some(index)
            })
            .collect::<Vec<_>>();

        if keep_indices.is_empty() {
            return Err(WaveformError::InvalidWaveform {
                reason: "nan_remove would remove every sample row".to_string(),
            });
        }

        let time = select_time(&waveform.time, &keep_indices);
        let channels = select_channels(&waveform.channels, &keep_indices);
        let transform_step = TransformStepMetadata::implemented_desktop(
            "nan_remove(policy=drop_rows_with_nan)",
            "nan_remove",
            TransformCategory::DataCleaning,
            vec![TransformParameterMetadata::text(
                "policy",
                "drop_rows_with_nan",
            )],
            false,
            false,
            TransformPhaseEffect::None,
        );
        derived_waveform_with_time(waveform, time, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimestampSortTransform;

impl Filter for TimestampSortTransform {
    fn name(&self) -> &'static str {
        "timestamp_sort"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_finite_time_axis(&waveform.time, "timestamp sorting")?;
        validate_channels_allowing_nan(waveform, "timestamp_sort")?;

        let indices = sorted_time_indices(&waveform.time);
        let time = select_time(&waveform.time, &indices);
        let channels = select_channels(&waveform.channels, &indices);
        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            "timestamp_sort(order=ascending)",
            "timestamp_sort",
            TransformCategory::DataCleaning,
            vec![TransformParameterMetadata::text("order", "ascending")],
            offline_execution(true, false, TransformPhaseEffect::None),
        );
        derived_waveform_with_time(waveform, time, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DedupeTimestampsTransform;

impl Filter for DedupeTimestampsTransform {
    fn name(&self) -> &'static str {
        "dedupe_timestamps"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_finite_time_axis(&waveform.time, "timestamp dedupe")?;
        validate_channels_allowing_nan(waveform, "dedupe_timestamps")?;

        let mut seen = Vec::new();
        let keep_indices = waveform
            .time
            .iter()
            .enumerate()
            .filter_map(|(index, sample_time)| {
                if seen.iter().any(|seen_time| sample_time == seen_time) {
                    None
                } else {
                    seen.push(*sample_time);
                    Some(index)
                }
            })
            .collect::<Vec<_>>();

        let time = select_time(&waveform.time, &keep_indices);
        let channels = select_channels(&waveform.channels, &keep_indices);
        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            "dedupe_timestamps(policy=keep_first)",
            "dedupe_timestamps",
            TransformCategory::DataCleaning,
            vec![TransformParameterMetadata::text("policy", "keep_first")],
            offline_execution(true, false, TransformPhaseEffect::None),
        );
        derived_waveform_with_time(waveform, time, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CropTransform {
    pub start_time_s: f64,
    pub end_time_s: f64,
}

impl Filter for CropTransform {
    fn name(&self) -> &'static str {
        "crop"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        self.validate()?;
        validate_finite_time_axis(&waveform.time, "cropping")?;
        validate_channels_allowing_nan(waveform, "crop")?;

        let keep_indices = waveform
            .time
            .iter()
            .enumerate()
            .filter_map(|(index, sample_time)| {
                ((*sample_time >= self.start_time_s) && (*sample_time <= self.end_time_s))
                    .then_some(index)
            })
            .collect::<Vec<_>>();

        if keep_indices.is_empty() {
            return Err(WaveformError::InvalidWaveform {
                reason: "crop selected no samples".to_string(),
            });
        }

        let time = select_time(&waveform.time, &keep_indices);
        let channels = select_channels(&waveform.channels, &keep_indices);
        let history_label = format!(
            "crop(start_time_s={},end_time_s={})",
            self.start_time_s, self.end_time_s
        );
        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            history_label,
            "crop",
            TransformCategory::DataCleaning,
            vec![
                TransformParameterMetadata::float("start_time_s", self.start_time_s, "s"),
                TransformParameterMetadata::float("end_time_s", self.end_time_s, "s"),
            ],
            offline_execution(true, false, TransformPhaseEffect::None),
        );
        derived_waveform_with_time(waveform, time, channels, transform_step)
    }
}

impl CropTransform {
    fn validate(&self) -> Result<()> {
        validate_finite_parameter("start_time_s", self.start_time_s)?;
        validate_finite_parameter("end_time_s", self.end_time_s)?;
        if self.end_time_s < self.start_time_s {
            return Err(WaveformError::InvalidParameter {
                name: "end_time_s".to_string(),
                reason: "must be greater than or equal to start_time_s".to_string(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FixedDelayTransform {
    pub delay_s: f64,
}

impl Filter for FixedDelayTransform {
    fn name(&self) -> &'static str {
        "fixed_delay"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_finite_parameter("delay_s", self.delay_s)?;
        validate_finite_time_axis(&waveform.time, "fixed delay")?;
        validate_channels_allowing_nan(waveform, "fixed_delay")?;

        let time = waveform
            .time
            .iter()
            .map(|sample_time| {
                let shifted = sample_time + self.delay_s;
                if shifted.is_finite() {
                    Ok(shifted)
                } else {
                    Err(WaveformError::InvalidWaveform {
                        reason: "fixed_delay produced a non-finite timestamp".to_string(),
                    })
                }
            })
            .collect::<Result<Vec<_>>>()?;

        let history_label = format!("fixed_delay(delay_s={})", self.delay_s);
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "fixed_delay",
            TransformCategory::Resampling,
            vec![TransformParameterMetadata::float(
                "delay_s",
                self.delay_s,
                "s",
            )],
            true,
            false,
            TransformPhaseEffect::Delay,
        );
        derived_waveform_with_time(waveform, time, waveform.channels.clone(), transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GapFillTransform {
    pub sample_interval_s: f64,
}

impl Filter for GapFillTransform {
    fn name(&self) -> &'static str {
        "gap_fill"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let history_label = format!("gap_fill(sample_interval_s={})", self.sample_interval_s);
        resample_waveform(
            waveform,
            self.sample_interval_s,
            history_label,
            "gap_fill",
            TransformCategory::Resampling,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResampleFixedTransform {
    pub sample_interval_s: f64,
}

impl Filter for ResampleFixedTransform {
    fn name(&self) -> &'static str {
        "resample_fixed"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let history_label = format!(
            "resample_fixed(sample_interval_s={})",
            self.sample_interval_s
        );
        resample_waveform(
            waveform,
            self.sample_interval_s,
            history_label,
            "resample_fixed",
            TransformCategory::Resampling,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelDelayTransform {
    pub channel: String,
    pub delay_s: f64,
}

impl Filter for ChannelDelayTransform {
    fn name(&self) -> &'static str {
        "channel_delay"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_required_channel_name("channel", &self.channel)?;
        validate_finite_parameter("delay_s", self.delay_s)?;
        validate_time_axis(&waveform.time, "channel delay alignment")?;
        validate_channels_allowing_nan(waveform, "channel_delay")?;

        let source_channel =
            waveform
                .channel(&self.channel)
                .ok_or_else(|| WaveformError::InvalidParameter {
                    name: "channel".to_string(),
                    reason: format!("channel `{}` was not found", self.channel),
                })?;
        validate_finite_samples("channel_delay", &source_channel.samples)?;

        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                let samples = if channel.name == self.channel {
                    waveform
                        .time
                        .iter()
                        .map(|sample_time| {
                            interpolate_series(
                                &waveform.time,
                                &source_channel.samples,
                                sample_time - self.delay_s,
                            )
                        })
                        .collect()
                } else {
                    channel.samples.clone()
                };
                Channel::new(channel.name.clone(), channel.unit.clone(), samples)
            })
            .collect();

        let history_label = format!(
            "channel_delay(channel={},delay_s={})",
            self.channel, self.delay_s
        );
        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            history_label,
            "channel_delay",
            TransformCategory::Resampling,
            vec![
                TransformParameterMetadata::text("channel", self.channel.clone()),
                TransformParameterMetadata::float("delay_s", self.delay_s, "s"),
            ],
            offline_execution(true, false, TransformPhaseEffect::Delay),
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResampleTransform {
    pub sample_interval_s: f64,
}

impl Filter for ResampleTransform {
    fn name(&self) -> &'static str {
        "resample"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let history_label = format!("resample(sample_interval_s={})", self.sample_interval_s);
        resample_waveform_with_interpolation(
            waveform,
            self.sample_interval_s,
            history_label,
            "resample",
            vec![TransformParameterMetadata::float(
                "sample_interval_s",
                self.sample_interval_s,
                "s",
            )],
            offline_execution(true, false, TransformPhaseEffect::None),
            TimeInterpolationKind::Linear,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DownsampleTransform {
    pub factor: usize,
}

impl Filter for DownsampleTransform {
    fn name(&self) -> &'static str {
        "downsample"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_resampling_factor("factor", self.factor)?;
        validate_time_axis(&waveform.time, "downsample")?;
        validate_channels_allowing_nan(waveform, "downsample")?;

        let indices = decimated_indices(waveform.time.len(), self.factor);
        let time = select_time(&waveform.time, &indices);
        let channels = select_channels(&waveform.channels, &indices);
        let history_label = format!("downsample(factor={})", self.factor);
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "downsample",
            TransformCategory::Resampling,
            vec![TransformParameterMetadata::integer(
                "factor",
                self.factor as u64,
                "ratio",
            )],
            true,
            false,
            TransformPhaseEffect::None,
        );
        derived_waveform_with_time(waveform, time, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DecimateTransform {
    pub factor: usize,
    pub cutoff_hz: f64,
}

impl Filter for DecimateTransform {
    fn name(&self) -> &'static str {
        "decimate"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_resampling_factor("factor", self.factor)?;
        let sample_rate_hz = uniform_sample_rate(&waveform.time, "decimate")?;
        validate_positive_frequency("cutoff_hz", self.cutoff_hz)?;
        let target_nyquist_hz = sample_rate_hz / (2.0 * self.factor as f64);
        if self.cutoff_hz > target_nyquist_hz {
            return Err(WaveformError::InvalidParameter {
                name: "cutoff_hz".to_string(),
                reason: format!(
                    "must be less than or equal to target Nyquist frequency {target_nyquist_hz} Hz"
                ),
            });
        }

        let indices = decimated_indices(waveform.time.len(), self.factor);
        let time = select_time(&waveform.time, &indices);
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_finite_samples("decimate", &channel.samples)?;
                let filtered =
                    first_order_low_pass(&waveform.time, &channel.samples, self.cutoff_hz);
                let samples = indices
                    .iter()
                    .map(|index| filtered[*index])
                    .collect::<Vec<_>>();
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let history_label = format!(
            "decimate(factor={},cutoff_hz={})",
            self.factor, self.cutoff_hz
        );
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "decimate",
            TransformCategory::Resampling,
            vec![
                TransformParameterMetadata::integer("factor", self.factor as u64, "ratio"),
                TransformParameterMetadata::float("cutoff_hz", self.cutoff_hz, "Hz"),
            ],
            true,
            true,
            TransformPhaseEffect::Delay,
        );
        derived_waveform_with_time(waveform, time, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UpsampleTransform {
    pub factor: usize,
}

impl Filter for UpsampleTransform {
    fn name(&self) -> &'static str {
        "upsample"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_resampling_factor("factor", self.factor)?;
        let interval = uniform_sample_interval(&waveform.time, "upsample")?;
        let sample_count = resampled_count_for_uniform_factor(waveform.time.len(), self.factor)?;
        let time = time_grid_by_count(
            waveform.time[0],
            interval / self.factor as f64,
            sample_count,
        )?;
        let channels = interpolate_channels_to_grid(
            waveform,
            &time,
            "upsample",
            TimeInterpolationKind::Linear,
        )?;

        let history_label = format!("upsample(factor={})", self.factor);
        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            history_label,
            "upsample",
            TransformCategory::Resampling,
            vec![TransformParameterMetadata::integer(
                "factor",
                self.factor as u64,
                "ratio",
            )],
            offline_execution(true, false, TransformPhaseEffect::None),
        );
        derived_waveform_with_time(waveform, time, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InterpolateTransform {
    pub sample_interval_s: f64,
}

impl Filter for InterpolateTransform {
    fn name(&self) -> &'static str {
        "interpolate"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let history_label = format!("interpolate(sample_interval_s={})", self.sample_interval_s);
        resample_waveform_with_interpolation(
            waveform,
            self.sample_interval_s,
            history_label,
            "interpolate",
            vec![TransformParameterMetadata::float(
                "sample_interval_s",
                self.sample_interval_s,
                "s",
            )],
            offline_execution(true, false, TransformPhaseEffect::None),
            TimeInterpolationKind::Linear,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RationalResampleTransform {
    pub upsample_factor: usize,
    pub downsample_factor: usize,
}

impl Filter for RationalResampleTransform {
    fn name(&self) -> &'static str {
        "rational_resample"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_positive_usize("upsample_factor", self.upsample_factor)?;
        validate_positive_usize("downsample_factor", self.downsample_factor)?;
        let source_interval_s = uniform_sample_interval(&waveform.time, "rational_resample")?;
        let sample_interval_s =
            source_interval_s * self.downsample_factor as f64 / self.upsample_factor as f64;
        validate_sample_interval(sample_interval_s)?;
        let time = fixed_time_grid(&waveform.time, sample_interval_s)?;
        let channels = interpolate_channels_to_grid(
            waveform,
            &time,
            "rational_resample",
            TimeInterpolationKind::Linear,
        )?;

        let history_label = format!(
            "rational_resample(upsample_factor={},downsample_factor={})",
            self.upsample_factor, self.downsample_factor
        );
        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            history_label,
            "rational_resample",
            TransformCategory::Resampling,
            vec![
                TransformParameterMetadata::integer(
                    "upsample_factor",
                    self.upsample_factor as u64,
                    "ratio",
                ),
                TransformParameterMetadata::integer(
                    "downsample_factor",
                    self.downsample_factor as u64,
                    "ratio",
                ),
                TransformParameterMetadata::float("sample_interval_s", sample_interval_s, "s"),
            ],
            offline_execution(true, false, TransformPhaseEffect::None),
        );
        derived_waveform_with_time(waveform, time, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SampleAndHoldTransform {
    pub sample_interval_s: f64,
}

impl Filter for SampleAndHoldTransform {
    fn name(&self) -> &'static str {
        "sample_and_hold"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let history_label = format!(
            "sample_and_hold(sample_interval_s={})",
            self.sample_interval_s
        );
        resample_waveform_with_interpolation(
            waveform,
            self.sample_interval_s,
            history_label,
            "sample_and_hold",
            vec![TransformParameterMetadata::float(
                "sample_interval_s",
                self.sample_interval_s,
                "s",
            )],
            offline_execution(true, true, TransformPhaseEffect::Delay),
            TimeInterpolationKind::Previous,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZeroOrderHoldTransform {
    pub sample_interval_s: f64,
}

impl Filter for ZeroOrderHoldTransform {
    fn name(&self) -> &'static str {
        "zero_order_hold"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let history_label = format!(
            "zero_order_hold(sample_interval_s={})",
            self.sample_interval_s
        );
        resample_waveform_with_interpolation(
            waveform,
            self.sample_interval_s,
            history_label,
            "zero_order_hold",
            vec![TransformParameterMetadata::float(
                "sample_interval_s",
                self.sample_interval_s,
                "s",
            )],
            offline_execution(true, true, TransformPhaseEffect::Delay),
            TimeInterpolationKind::Previous,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FirstOrderHoldTransform {
    pub sample_interval_s: f64,
}

impl Filter for FirstOrderHoldTransform {
    fn name(&self) -> &'static str {
        "first_order_hold"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let history_label = format!(
            "first_order_hold(sample_interval_s={})",
            self.sample_interval_s
        );
        resample_waveform_with_interpolation(
            waveform,
            self.sample_interval_s,
            history_label,
            "first_order_hold",
            vec![TransformParameterMetadata::float(
                "sample_interval_s",
                self.sample_interval_s,
                "s",
            )],
            offline_execution(true, false, TransformPhaseEffect::None),
            TimeInterpolationKind::Linear,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FractionalDelayTransform {
    pub delay_s: f64,
}

impl Filter for FractionalDelayTransform {
    fn name(&self) -> &'static str {
        "fractional_delay"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_finite_parameter("delay_s", self.delay_s)?;
        validate_time_axis(&waveform.time, "fractional_delay")?;
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_finite_samples("fractional_delay", &channel.samples)?;
                let samples =
                    fractional_delay_samples(&waveform.time, &channel.samples, self.delay_s);
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let history_label = format!("fractional_delay(delay_s={})", self.delay_s);
        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            history_label,
            "fractional_delay",
            TransformCategory::Resampling,
            vec![TransformParameterMetadata::float(
                "delay_s",
                self.delay_s,
                "s",
            )],
            offline_execution(true, false, TransformPhaseEffect::Delay),
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrossCorrelationDelayTransform {
    pub reference_channel: String,
    pub target_channel: String,
    pub max_lag_samples: usize,
}

impl Filter for CrossCorrelationDelayTransform {
    fn name(&self) -> &'static str {
        "cross_correlation_delay"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_required_channel_name("reference_channel", &self.reference_channel)?;
        validate_required_channel_name("target_channel", &self.target_channel)?;
        validate_positive_usize("max_lag_samples", self.max_lag_samples)?;
        let sample_interval_s = uniform_sample_interval(&waveform.time, "cross_correlation_delay")?;
        let reference = waveform.channel(&self.reference_channel).ok_or_else(|| {
            WaveformError::InvalidParameter {
                name: "reference_channel".to_string(),
                reason: format!("channel `{}` was not found", self.reference_channel),
            }
        })?;
        let target = waveform.channel(&self.target_channel).ok_or_else(|| {
            WaveformError::InvalidParameter {
                name: "target_channel".to_string(),
                reason: format!("channel `{}` was not found", self.target_channel),
            }
        })?;
        validate_finite_samples("cross_correlation_delay", &reference.samples)?;
        validate_finite_samples("cross_correlation_delay", &target.samples)?;

        let estimate = estimate_cross_correlation_delay(
            &reference.samples,
            &target.samples,
            self.max_lag_samples,
        )?;
        let estimated_delay_s = estimate.lag_samples as f64 * sample_interval_s;
        let correction_delay_s = -estimated_delay_s;
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                let samples = if channel.name == self.target_channel {
                    fractional_delay_samples(&waveform.time, &channel.samples, correction_delay_s)
                } else {
                    channel.samples.clone()
                };
                Channel::new(channel.name.clone(), channel.unit.clone(), samples)
            })
            .collect::<Vec<_>>();

        let history_label = format!(
            "cross_correlation_delay(reference_channel={},target_channel={},max_lag_samples={})",
            self.reference_channel, self.target_channel, self.max_lag_samples
        );
        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            history_label,
            "cross_correlation_delay",
            TransformCategory::Resampling,
            vec![
                TransformParameterMetadata::text(
                    "reference_channel",
                    self.reference_channel.clone(),
                ),
                TransformParameterMetadata::text("target_channel", self.target_channel.clone()),
                TransformParameterMetadata::integer(
                    "max_lag_samples",
                    self.max_lag_samples as u64,
                    "samples",
                ),
                TransformParameterMetadata::float(
                    "estimated_lag_samples",
                    estimate.lag_samples as f64,
                    "samples",
                ),
                TransformParameterMetadata::float("estimated_delay_s", estimated_delay_s, "s"),
                TransformParameterMetadata::float("confidence", estimate.confidence, "ratio"),
            ],
            offline_execution(true, true, TransformPhaseEffect::Delay),
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JitterCorrectionTransform {
    pub sample_interval_s: f64,
}

impl Filter for JitterCorrectionTransform {
    fn name(&self) -> &'static str {
        "jitter_correction"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let history_label = format!(
            "jitter_correction(sample_interval_s={})",
            self.sample_interval_s
        );
        resample_waveform_with_interpolation(
            waveform,
            self.sample_interval_s,
            history_label,
            "jitter_correction",
            vec![TransformParameterMetadata::float(
                "sample_interval_s",
                self.sample_interval_s,
                "s",
            )],
            offline_execution(true, false, TransformPhaseEffect::None),
            TimeInterpolationKind::Linear,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClockDriftCorrectionTransform {
    pub sample_interval_s: f64,
}

impl Filter for ClockDriftCorrectionTransform {
    fn name(&self) -> &'static str {
        "clock_drift_correction"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_sample_interval(self.sample_interval_s)?;
        validate_time_axis(&waveform.time, "clock_drift_correction")?;
        let time = time_grid_by_count(
            waveform.time[0],
            self.sample_interval_s,
            waveform.time.len(),
        )?;
        let channels = interpolate_channels_to_grid(
            waveform,
            &time,
            "clock_drift_correction",
            TimeInterpolationKind::Linear,
        )?;
        let source_end = *waveform
            .time
            .last()
            .expect("validated waveforms always include a timestamp");
        let target_end = *time
            .last()
            .expect("clock drift correction keeps at least one timestamp");
        let end_drift_s = source_end - target_end;

        let history_label = format!(
            "clock_drift_correction(sample_interval_s={})",
            self.sample_interval_s
        );
        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            history_label,
            "clock_drift_correction",
            TransformCategory::Resampling,
            vec![
                TransformParameterMetadata::float("sample_interval_s", self.sample_interval_s, "s"),
                TransformParameterMetadata::float("end_drift_s", end_drift_s, "s"),
            ],
            offline_execution(true, false, TransformPhaseEffect::None),
        );
        derived_waveform_with_time(waveform, time, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HalfWaveRectifyTransform;

impl Filter for HalfWaveRectifyTransform {
    fn name(&self) -> &'static str {
        "half_wave_rectify"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        apply_m31_transform(
            waveform,
            "half_wave_rectify()",
            "half_wave_rectify",
            TransformCategory::Pointwise,
            Vec::new(),
            TransformExecutionMetadata {
                sample_rate_required: false,
                stateful: false,
                causal: true,
                phase_effect: TransformPhaseEffect::Nonlinear,
                streaming_supported: true,
                offline_only: false,
            },
            |_, samples| Ok(samples.iter().map(|sample| sample.max(0.0)).collect()),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FullWaveRectifyTransform;

impl Filter for FullWaveRectifyTransform {
    fn name(&self) -> &'static str {
        "full_wave_rectify"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        apply_m31_transform(
            waveform,
            "full_wave_rectify()",
            "full_wave_rectify",
            TransformCategory::Pointwise,
            Vec::new(),
            TransformExecutionMetadata {
                sample_rate_required: false,
                stateful: false,
                causal: true,
                phase_effect: TransformPhaseEffect::Nonlinear,
                streaming_supported: true,
                offline_only: false,
            },
            |_, samples| Ok(samples.iter().map(|sample| sample.abs()).collect()),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnvelopeTransform {
    pub alpha: f64,
}

impl Filter for EnvelopeTransform {
    fn name(&self) -> &'static str {
        "envelope"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_unit_interval("alpha", self.alpha)?;
        apply_m31_transform(
            waveform,
            format!("envelope(alpha={})", self.alpha),
            "envelope",
            TransformCategory::Stateful,
            vec![TransformParameterMetadata::float(
                "alpha", self.alpha, "ratio",
            )],
            TransformExecutionMetadata {
                sample_rate_required: false,
                stateful: true,
                causal: true,
                phase_effect: TransformPhaseEffect::Delay,
                streaming_supported: true,
                offline_only: false,
            },
            |_, samples| Ok(envelope_samples(samples, self.alpha)),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MovingRmsTransform {
    pub window_samples: usize,
}

impl Filter for MovingRmsTransform {
    fn name(&self) -> &'static str {
        "moving_rms"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_window_samples(self.window_samples)?;
        apply_m31_transform(
            waveform,
            format!("moving_rms(window_samples={})", self.window_samples),
            "moving_rms",
            TransformCategory::Windowed,
            vec![TransformParameterMetadata::integer(
                "window_samples",
                self.window_samples as u64,
                "samples",
            )],
            TransformExecutionMetadata {
                sample_rate_required: false,
                stateful: true,
                causal: true,
                phase_effect: TransformPhaseEffect::Delay,
                streaming_supported: true,
                offline_only: false,
            },
            |_, samples| Ok(moving_rms_samples(samples, self.window_samples)),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PeakHoldTransform;

impl Filter for PeakHoldTransform {
    fn name(&self) -> &'static str {
        "peak_hold"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        apply_m31_transform(
            waveform,
            "peak_hold()",
            "peak_hold",
            TransformCategory::Stateful,
            Vec::new(),
            TransformExecutionMetadata {
                sample_rate_required: false,
                stateful: true,
                causal: true,
                phase_effect: TransformPhaseEffect::Nonlinear,
                streaming_supported: true,
                offline_only: false,
            },
            |_, samples| Ok(peak_hold_samples(samples)),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FirstDerivativeTransform;

impl Filter for FirstDerivativeTransform {
    fn name(&self) -> &'static str {
        "first_derivative"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        apply_m31_transform(
            waveform,
            "first_derivative()",
            "first_derivative",
            TransformCategory::Feature,
            Vec::new(),
            TransformExecutionMetadata {
                sample_rate_required: true,
                stateful: true,
                causal: true,
                phase_effect: TransformPhaseEffect::Delay,
                streaming_supported: true,
                offline_only: false,
            },
            derivative_samples,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SecondDerivativeTransform;

impl Filter for SecondDerivativeTransform {
    fn name(&self) -> &'static str {
        "second_derivative"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        apply_m31_transform(
            waveform,
            "second_derivative()",
            "second_derivative",
            TransformCategory::Feature,
            Vec::new(),
            TransformExecutionMetadata {
                sample_rate_required: true,
                stateful: true,
                causal: true,
                phase_effect: TransformPhaseEffect::Delay,
                streaming_supported: true,
                offline_only: false,
            },
            |time, samples| {
                let first = derivative_samples(time, samples)?;
                derivative_samples(time, &first)
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntegralTransform;

impl Filter for IntegralTransform {
    fn name(&self) -> &'static str {
        "integral"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        cumulative_integral_waveform(waveform, "integral()", "integral")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CumulativeIntegralTransform;

impl Filter for CumulativeIntegralTransform {
    fn name(&self) -> &'static str {
        "cumulative_integral"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        cumulative_integral_waveform(waveform, "cumulative_integral()", "cumulative_integral")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LeakyIntegratorTransform {
    pub time_constant_s: f64,
}

impl Filter for LeakyIntegratorTransform {
    fn name(&self) -> &'static str {
        "leaky_integrator"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_positive_finite_parameter("time_constant_s", self.time_constant_s)?;
        apply_m31_transform(
            waveform,
            format!("leaky_integrator(time_constant_s={})", self.time_constant_s),
            "leaky_integrator",
            TransformCategory::Stateful,
            vec![TransformParameterMetadata::float(
                "time_constant_s",
                self.time_constant_s,
                "s",
            )],
            TransformExecutionMetadata {
                sample_rate_required: true,
                stateful: true,
                causal: true,
                phase_effect: TransformPhaseEffect::Delay,
                streaming_supported: true,
                offline_only: false,
            },
            |time, samples| leaky_integrator_samples(time, samples, self.time_constant_s),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SlopeDetectionTransform {
    pub threshold_per_s: f64,
}

impl Filter for SlopeDetectionTransform {
    fn name(&self) -> &'static str {
        "slope_detection"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_positive_finite_parameter("threshold_per_s", self.threshold_per_s)?;
        apply_m31_transform(
            waveform,
            format!("slope_detection(threshold_per_s={})", self.threshold_per_s),
            "slope_detection",
            TransformCategory::Feature,
            vec![TransformParameterMetadata::float(
                "threshold_per_s",
                self.threshold_per_s,
                "unit/s",
            )],
            TransformExecutionMetadata {
                sample_rate_required: true,
                stateful: true,
                causal: true,
                phase_effect: TransformPhaseEffect::Nonlinear,
                streaming_supported: true,
                offline_only: false,
            },
            |time, samples| slope_detection_samples(time, samples, self.threshold_per_s),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RollingMeanTransform {
    pub window_samples: usize,
}

impl Filter for RollingMeanTransform {
    fn name(&self) -> &'static str {
        "rolling_mean"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        apply_m32_rolling_transform(
            waveform,
            self.name(),
            self.window_samples,
            rolling_mean_samples,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RollingVarianceTransform {
    pub window_samples: usize,
}

impl Filter for RollingVarianceTransform {
    fn name(&self) -> &'static str {
        "rolling_variance"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        apply_m32_rolling_transform(
            waveform,
            self.name(),
            self.window_samples,
            rolling_variance_samples,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RollingStdDevTransform {
    pub window_samples: usize,
}

impl Filter for RollingStdDevTransform {
    fn name(&self) -> &'static str {
        "rolling_stddev"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        apply_m32_rolling_transform(
            waveform,
            self.name(),
            self.window_samples,
            rolling_stddev_samples,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RollingMinTransform {
    pub window_samples: usize,
}

impl Filter for RollingMinTransform {
    fn name(&self) -> &'static str {
        "rolling_min"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        apply_m32_rolling_transform(
            waveform,
            self.name(),
            self.window_samples,
            rolling_min_samples,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RollingMaxTransform {
    pub window_samples: usize,
}

impl Filter for RollingMaxTransform {
    fn name(&self) -> &'static str {
        "rolling_max"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        apply_m32_rolling_transform(
            waveform,
            self.name(),
            self.window_samples,
            rolling_max_samples,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZScoreTransform;

impl Filter for ZScoreTransform {
    fn name(&self) -> &'static str {
        "z_score"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        apply_m32_distribution_transform(waveform, self.name(), Vec::new(), z_score_samples)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OutlierDetectionTransform {
    pub threshold_sigma: f64,
}

impl Filter for OutlierDetectionTransform {
    fn name(&self) -> &'static str {
        "outlier_detection"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_positive_finite_parameter("threshold_sigma", self.threshold_sigma)?;
        apply_m32_distribution_transform(
            waveform,
            self.name(),
            vec![TransformParameterMetadata::float(
                "threshold_sigma",
                self.threshold_sigma,
                "sigma",
            )],
            |samples| outlier_detection_samples(samples, self.threshold_sigma),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuantileClipTransform {
    pub lower_quantile: f64,
    pub upper_quantile: f64,
}

impl Filter for QuantileClipTransform {
    fn name(&self) -> &'static str {
        "quantile_clip"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_quantile_pair(self.lower_quantile, self.upper_quantile)?;
        apply_m32_distribution_transform(
            waveform,
            self.name(),
            vec![
                TransformParameterMetadata::float("lower_quantile", self.lower_quantile, "ratio"),
                TransformParameterMetadata::float("upper_quantile", self.upper_quantile, "ratio"),
            ],
            |samples| quantile_clip_samples(samples, self.lower_quantile, self.upper_quantile),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoiseKind {
    White,
    Gaussian,
    Uniform,
    Pink,
    Brown,
    Impulse,
    SaltPepper,
    Quantization,
}

impl NoiseKind {
    const fn name(self) -> &'static str {
        match self {
            Self::White => "white_noise",
            Self::Gaussian => "gaussian_noise",
            Self::Uniform => "uniform_noise",
            Self::Pink => "pink_noise",
            Self::Brown => "brown_noise",
            Self::Impulse => "impulse_noise",
            Self::SaltPepper => "salt_pepper_noise",
            Self::Quantization => "quantization_noise",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NoiseInjectionTransform {
    pub kind: NoiseKind,
    pub amplitude_v: f64,
    pub min_v: f64,
    pub max_v: f64,
    pub probability: f64,
    pub seed: u64,
}

impl Filter for NoiseInjectionTransform {
    fn name(&self) -> &'static str {
        self.kind.name()
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        self.validate()?;
        let channels = waveform
            .channels
            .iter()
            .enumerate()
            .map(|(channel_index, channel)| {
                validate_finite_samples(self.name(), &channel.samples)?;
                let mut rng = DeterministicRng::new(self.seed ^ ((channel_index as u64 + 1) << 32));
                let mut colored_state = 0.0;
                let samples = channel
                    .samples
                    .iter()
                    .copied()
                    .map(|sample| self.simulated_sample(sample, &mut rng, &mut colored_state))
                    .collect::<Result<Vec<_>>>()?;
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        let transform_step = simulation_transform_step(
            self.name(),
            self.noise_history_label(),
            TransformCategory::FaultInjection,
            self.noise_parameters(),
            false,
            true,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

impl NoiseInjectionTransform {
    fn validate(&self) -> Result<()> {
        match self.kind {
            NoiseKind::Uniform | NoiseKind::SaltPepper => {
                validate_finite_parameter("min_v", self.min_v)?;
                validate_finite_parameter("max_v", self.max_v)?;
                if self.max_v <= self.min_v {
                    return Err(WaveformError::InvalidParameter {
                        name: "max_v".to_string(),
                        reason: "must be greater than min_v".to_string(),
                    });
                }
            }
            NoiseKind::Gaussian
            | NoiseKind::White
            | NoiseKind::Pink
            | NoiseKind::Brown
            | NoiseKind::Impulse
            | NoiseKind::Quantization => {
                validate_positive_parameter(self.amplitude_name(), self.amplitude_v)?
            }
        }
        if matches!(self.kind, NoiseKind::Impulse | NoiseKind::SaltPepper) {
            validate_probability(self.probability)?;
        }
        Ok(())
    }

    fn amplitude_name(&self) -> &'static str {
        match self.kind {
            NoiseKind::Gaussian => "stddev_v",
            NoiseKind::Quantization => "lsb_v",
            _ => "amplitude_v",
        }
    }

    fn simulated_sample(
        &self,
        sample: f64,
        rng: &mut DeterministicRng,
        colored_state: &mut f64,
    ) -> Result<f64> {
        let output = match self.kind {
            NoiseKind::White => sample + rng.signed_unit() * self.amplitude_v,
            NoiseKind::Gaussian => sample + rng.gaussian() * self.amplitude_v,
            NoiseKind::Uniform => sample + self.min_v + rng.unit() * (self.max_v - self.min_v),
            NoiseKind::Pink => {
                *colored_state =
                    0.85 * *colored_state + 0.15 * rng.signed_unit() * self.amplitude_v;
                sample + *colored_state
            }
            NoiseKind::Brown => {
                *colored_state = (*colored_state + rng.signed_unit() * self.amplitude_v)
                    .clamp(-self.amplitude_v, self.amplitude_v);
                sample + *colored_state
            }
            NoiseKind::Impulse => {
                if rng.unit() < self.probability {
                    sample + rng.signed_unit() * self.amplitude_v
                } else {
                    sample
                }
            }
            NoiseKind::SaltPepper => {
                if rng.unit() < self.probability {
                    if rng.unit() < 0.5 {
                        self.min_v
                    } else {
                        self.max_v
                    }
                } else {
                    sample
                }
            }
            NoiseKind::Quantization => sample + (rng.unit() - 0.5) * self.amplitude_v,
        };
        if output.is_finite() {
            Ok(output)
        } else {
            Err(WaveformError::InvalidWaveform {
                reason: format!("{} produced a non-finite sample", self.name()),
            })
        }
    }

    fn noise_history_label(&self) -> String {
        match self.kind {
            NoiseKind::Uniform => format!(
                "{}(min_v={},max_v={},seed={})",
                self.name(),
                self.min_v,
                self.max_v,
                self.seed
            ),
            NoiseKind::SaltPepper => format!(
                "{}(min_v={},max_v={},probability={},seed={})",
                self.name(),
                self.min_v,
                self.max_v,
                self.probability,
                self.seed
            ),
            NoiseKind::Impulse => format!(
                "{}(amplitude_v={},probability={},seed={})",
                self.name(),
                self.amplitude_v,
                self.probability,
                self.seed
            ),
            _ => format!(
                "{}({}={},seed={})",
                self.name(),
                self.amplitude_name(),
                self.amplitude_v,
                self.seed
            ),
        }
    }

    fn noise_parameters(&self) -> Vec<TransformParameterMetadata> {
        let mut parameters = match self.kind {
            NoiseKind::Uniform | NoiseKind::SaltPepper => vec![
                TransformParameterMetadata::float("min_v", self.min_v, "V"),
                TransformParameterMetadata::float("max_v", self.max_v, "V"),
            ],
            _ => vec![TransformParameterMetadata::float(
                self.amplitude_name(),
                self.amplitude_v,
                "V",
            )],
        };
        if matches!(self.kind, NoiseKind::Impulse | NoiseKind::SaltPepper) {
            parameters.push(TransformParameterMetadata::float(
                "probability",
                self.probability,
                "ratio",
            ));
        }
        parameters.push(TransformParameterMetadata::integer(
            "seed", self.seed, "seed",
        ));
        parameters.push(TransformParameterMetadata::text(
            "evidence_scope",
            "simulation_only",
        ));
        parameters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeriodicInterferenceKind {
    Periodic,
    Hum,
}

impl PeriodicInterferenceKind {
    const fn name(self) -> &'static str {
        match self {
            Self::Periodic => "periodic_interference",
            Self::Hum => "hum_interference",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PeriodicInterferenceTransform {
    pub kind: PeriodicInterferenceKind,
    pub amplitude_v: f64,
    pub frequency_hz: f64,
    pub phase_rad: f64,
}

impl Filter for PeriodicInterferenceTransform {
    fn name(&self) -> &'static str {
        self.kind.name()
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_positive_parameter("amplitude_v", self.amplitude_v)?;
        validate_positive_parameter("frequency_hz", self.frequency_hz)?;
        validate_finite_parameter("phase_rad", self.phase_rad)?;
        validate_finite_time_axis(&waveform.time, self.name())?;
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_finite_samples(self.name(), &channel.samples)?;
                let samples = waveform
                    .time
                    .iter()
                    .zip(channel.samples.iter())
                    .map(|(time, sample)| {
                        sample
                            + self.amplitude_v
                                * (TAU * self.frequency_hz * time + self.phase_rad).sin()
                    })
                    .collect();
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        let parameters = vec![
            TransformParameterMetadata::float("amplitude_v", self.amplitude_v, "V"),
            TransformParameterMetadata::float("frequency_hz", self.frequency_hz, "Hz"),
            TransformParameterMetadata::float("phase_rad", self.phase_rad, "rad"),
            TransformParameterMetadata::text("evidence_scope", "simulation_only"),
        ];
        let transform_step = simulation_transform_step(
            self.name(),
            format!(
                "{}(amplitude_v={},frequency_hz={},phase_rad={})",
                self.name(),
                self.amplitude_v,
                self.frequency_hz,
                self.phase_rad
            ),
            TransformCategory::FaultInjection,
            parameters,
            true,
            false,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriftFaultKind {
    GroundBounce,
    Thermal,
    RandomWalk,
}

impl DriftFaultKind {
    const fn name(self) -> &'static str {
        match self {
            Self::GroundBounce => "ground_bounce",
            Self::Thermal => "thermal_drift",
            Self::RandomWalk => "random_walk_drift",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DriftFaultTransform {
    pub kind: DriftFaultKind,
    pub amplitude_v: f64,
    pub drift_rate_v_per_s: f64,
    pub interval_samples: usize,
    pub seed: u64,
}

impl Filter for DriftFaultTransform {
    fn name(&self) -> &'static str {
        self.kind.name()
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        self.validate()?;
        if matches!(self.kind, DriftFaultKind::Thermal) {
            validate_finite_time_axis(&waveform.time, self.name())?;
        }
        let channels = waveform
            .channels
            .iter()
            .enumerate()
            .map(|(channel_index, channel)| {
                validate_finite_samples(self.name(), &channel.samples)?;
                let mut rng = DeterministicRng::new(self.seed ^ ((channel_index as u64 + 3) << 32));
                let mut walk = 0.0;
                let samples = channel
                    .samples
                    .iter()
                    .enumerate()
                    .map(|(index, sample)| {
                        let drift = match self.kind {
                            DriftFaultKind::GroundBounce => {
                                if (index / self.interval_samples) % 2 == 0 {
                                    self.amplitude_v
                                } else {
                                    -self.amplitude_v
                                }
                            }
                            DriftFaultKind::Thermal => {
                                self.drift_rate_v_per_s * (waveform.time[index] - waveform.time[0])
                            }
                            DriftFaultKind::RandomWalk => {
                                walk += rng.signed_unit() * self.amplitude_v;
                                walk
                            }
                        };
                        sample + drift
                    })
                    .collect();
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        let transform_step = simulation_transform_step(
            self.name(),
            self.drift_history_label(),
            TransformCategory::FaultInjection,
            self.drift_parameters(),
            matches!(self.kind, DriftFaultKind::Thermal),
            matches!(self.kind, DriftFaultKind::RandomWalk),
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

impl DriftFaultTransform {
    fn validate(&self) -> Result<()> {
        match self.kind {
            DriftFaultKind::GroundBounce => {
                validate_finite_parameter("amplitude_v", self.amplitude_v)?;
                if self.interval_samples == 0 {
                    return Err(WaveformError::InvalidParameter {
                        name: "interval_samples".to_string(),
                        reason: "must be greater than zero".to_string(),
                    });
                }
            }
            DriftFaultKind::Thermal => {
                validate_finite_parameter("drift_rate_v_per_s", self.drift_rate_v_per_s)?
            }
            DriftFaultKind::RandomWalk => {
                validate_positive_parameter("amplitude_v", self.amplitude_v)?
            }
        }
        Ok(())
    }

    fn drift_history_label(&self) -> String {
        match self.kind {
            DriftFaultKind::GroundBounce => format!(
                "{}(amplitude_v={},interval_samples={})",
                self.name(),
                self.amplitude_v,
                self.interval_samples
            ),
            DriftFaultKind::Thermal => format!(
                "{}(drift_rate_v_per_s={})",
                self.name(),
                self.drift_rate_v_per_s
            ),
            DriftFaultKind::RandomWalk => format!(
                "{}(amplitude_v={},seed={})",
                self.name(),
                self.amplitude_v,
                self.seed
            ),
        }
    }

    fn drift_parameters(&self) -> Vec<TransformParameterMetadata> {
        let mut parameters = match self.kind {
            DriftFaultKind::GroundBounce => vec![
                TransformParameterMetadata::float("amplitude_v", self.amplitude_v, "V"),
                TransformParameterMetadata::integer(
                    "interval_samples",
                    self.interval_samples as u64,
                    "samples",
                ),
            ],
            DriftFaultKind::Thermal => vec![TransformParameterMetadata::float(
                "drift_rate_v_per_s",
                self.drift_rate_v_per_s,
                "V/s",
            )],
            DriftFaultKind::RandomWalk => vec![
                TransformParameterMetadata::float("amplitude_v", self.amplitude_v, "V"),
                TransformParameterMetadata::integer("seed", self.seed, "seed"),
            ],
        };
        parameters.push(TransformParameterMetadata::text(
            "evidence_scope",
            "simulation_only",
        ));
        parameters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleFaultKind {
    Dropout,
    MissingSamples,
    Saturation,
    StuckAt,
    Flatline,
    Intermittent,
}

impl SampleFaultKind {
    const fn name(self) -> &'static str {
        match self {
            Self::Dropout => "dropout_fault",
            Self::MissingSamples => "missing_samples",
            Self::Saturation => "saturation_fault",
            Self::StuckAt => "stuck_at_fault",
            Self::Flatline => "flatline_fault",
            Self::Intermittent => "intermittent_fault",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SampleFaultTransform {
    pub kind: SampleFaultKind,
    pub probability: f64,
    pub fault_value_v: f64,
    pub min_v: f64,
    pub max_v: f64,
    pub start_index: usize,
    pub duration_samples: usize,
    pub seed: u64,
}

impl Filter for SampleFaultTransform {
    fn name(&self) -> &'static str {
        self.kind.name()
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        self.validate(waveform.time.len())?;
        let channels = waveform
            .channels
            .iter()
            .enumerate()
            .map(|(channel_index, channel)| {
                validate_finite_samples(self.name(), &channel.samples)?;
                let mut rng = DeterministicRng::new(self.seed ^ ((channel_index as u64 + 7) << 32));
                let flatline_value = if self.fault_value_v.is_finite() {
                    self.fault_value_v
                } else {
                    channel.samples[self.start_index]
                };
                let samples = channel
                    .samples
                    .iter()
                    .enumerate()
                    .map(|(index, sample)| match self.kind {
                        SampleFaultKind::Dropout
                        | SampleFaultKind::MissingSamples
                        | SampleFaultKind::Intermittent => {
                            if rng.unit() < self.probability {
                                self.fault_value_v
                            } else {
                                *sample
                            }
                        }
                        SampleFaultKind::Saturation => sample.clamp(self.min_v, self.max_v),
                        SampleFaultKind::StuckAt => {
                            if self.index_in_window(index) {
                                self.fault_value_v
                            } else {
                                *sample
                            }
                        }
                        SampleFaultKind::Flatline => {
                            if index >= self.start_index {
                                flatline_value
                            } else {
                                *sample
                            }
                        }
                    })
                    .collect();
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        let transform_step = simulation_transform_step(
            self.name(),
            self.sample_fault_history_label(),
            TransformCategory::FaultInjection,
            self.sample_fault_parameters(),
            false,
            true,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

impl SampleFaultTransform {
    fn validate(&self, sample_count: usize) -> Result<()> {
        match self.kind {
            SampleFaultKind::Dropout
            | SampleFaultKind::MissingSamples
            | SampleFaultKind::Intermittent => {
                validate_probability(self.probability)?;
                validate_finite_parameter("fault_value_v", self.fault_value_v)?;
            }
            SampleFaultKind::Saturation => {
                validate_finite_parameter("min_v", self.min_v)?;
                validate_finite_parameter("max_v", self.max_v)?;
                if self.max_v <= self.min_v {
                    return Err(WaveformError::InvalidParameter {
                        name: "max_v".to_string(),
                        reason: "must be greater than min_v".to_string(),
                    });
                }
            }
            SampleFaultKind::StuckAt => {
                validate_finite_parameter("fault_value_v", self.fault_value_v)?;
                validate_index_window(self.start_index, self.duration_samples, sample_count)?;
            }
            SampleFaultKind::Flatline => validate_start_index(self.start_index, sample_count)?,
        }
        Ok(())
    }

    fn index_in_window(&self, index: usize) -> bool {
        index >= self.start_index && index < self.start_index + self.duration_samples
    }

    fn sample_fault_history_label(&self) -> String {
        match self.kind {
            SampleFaultKind::Saturation => {
                format!("{}(min_v={},max_v={})", self.name(), self.min_v, self.max_v)
            }
            SampleFaultKind::StuckAt => format!(
                "{}(fault_value_v={},start_index={},duration_samples={})",
                self.name(),
                self.fault_value_v,
                self.start_index,
                self.duration_samples
            ),
            SampleFaultKind::Flatline => {
                format!("{}(start_index={})", self.name(), self.start_index)
            }
            _ => format!(
                "{}(fault_value_v={},probability={},seed={})",
                self.name(),
                self.fault_value_v,
                self.probability,
                self.seed
            ),
        }
    }

    fn sample_fault_parameters(&self) -> Vec<TransformParameterMetadata> {
        let mut parameters = match self.kind {
            SampleFaultKind::Saturation => vec![
                TransformParameterMetadata::float("min_v", self.min_v, "V"),
                TransformParameterMetadata::float("max_v", self.max_v, "V"),
            ],
            SampleFaultKind::StuckAt => vec![
                TransformParameterMetadata::float("fault_value_v", self.fault_value_v, "V"),
                TransformParameterMetadata::integer(
                    "start_index",
                    self.start_index as u64,
                    "samples",
                ),
                TransformParameterMetadata::integer(
                    "duration_samples",
                    self.duration_samples as u64,
                    "samples",
                ),
            ],
            SampleFaultKind::Flatline => vec![TransformParameterMetadata::integer(
                "start_index",
                self.start_index as u64,
                "samples",
            )],
            _ => vec![
                TransformParameterMetadata::float("fault_value_v", self.fault_value_v, "V"),
                TransformParameterMetadata::float("probability", self.probability, "ratio"),
                TransformParameterMetadata::integer("seed", self.seed, "seed"),
            ],
        };
        parameters.push(TransformParameterMetadata::text(
            "evidence_scope",
            "simulation_only",
        ));
        parameters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationQuantizerKind {
    Rounding,
    Floor,
    Ceil,
    MidRise,
    MidTread,
    Saturating,
}

impl SimulationQuantizerKind {
    const fn name(self) -> &'static str {
        match self {
            Self::Rounding => "rounding_quantizer",
            Self::Floor => "floor_quantizer",
            Self::Ceil => "ceil_quantizer",
            Self::MidRise => "midrise_quantizer",
            Self::MidTread => "midtread_quantizer",
            Self::Saturating => "saturating_quantizer",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimulationQuantizerTransform {
    pub kind: SimulationQuantizerKind,
    pub lsb_v: f64,
    pub min_v: f64,
    pub max_v: f64,
}

impl Filter for SimulationQuantizerTransform {
    fn name(&self) -> &'static str {
        self.kind.name()
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        self.validate()?;
        let channels = map_finite_samples(waveform, self.name(), |sample| match self.kind {
            SimulationQuantizerKind::Rounding => {
                quantize_by_lsb(sample, self.lsb_v, QuantizerRounding::Round)
            }
            SimulationQuantizerKind::Floor => {
                quantize_by_lsb(sample, self.lsb_v, QuantizerRounding::Floor)
            }
            SimulationQuantizerKind::Ceil => {
                quantize_by_lsb(sample, self.lsb_v, QuantizerRounding::Ceil)
            }
            SimulationQuantizerKind::MidRise => (sample / self.lsb_v)
                .floor()
                .mul_add(self.lsb_v, self.lsb_v / 2.0),
            SimulationQuantizerKind::MidTread => {
                quantize_by_lsb(sample, self.lsb_v, QuantizerRounding::Round)
            }
            SimulationQuantizerKind::Saturating => sample.clamp(self.min_v, self.max_v),
        })?;
        let transform_step = simulation_transform_step(
            self.name(),
            self.quantizer_history_label(),
            TransformCategory::Quantization,
            self.quantizer_parameters(),
            false,
            false,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

impl SimulationQuantizerTransform {
    fn validate(&self) -> Result<()> {
        if self.kind == SimulationQuantizerKind::Saturating {
            validate_finite_parameter("min_v", self.min_v)?;
            validate_finite_parameter("max_v", self.max_v)?;
            if self.max_v <= self.min_v {
                return Err(WaveformError::InvalidParameter {
                    name: "max_v".to_string(),
                    reason: "must be greater than min_v".to_string(),
                });
            }
        } else {
            validate_positive_parameter("lsb_v", self.lsb_v)?;
        }
        Ok(())
    }

    fn quantizer_history_label(&self) -> String {
        if self.kind == SimulationQuantizerKind::Saturating {
            format!("{}(min_v={},max_v={})", self.name(), self.min_v, self.max_v)
        } else {
            format!("{}(lsb_v={})", self.name(), self.lsb_v)
        }
    }

    fn quantizer_parameters(&self) -> Vec<TransformParameterMetadata> {
        let mut parameters = if self.kind == SimulationQuantizerKind::Saturating {
            vec![
                TransformParameterMetadata::float("min_v", self.min_v, "V"),
                TransformParameterMetadata::float("max_v", self.max_v, "V"),
            ]
        } else {
            vec![TransformParameterMetadata::float("lsb_v", self.lsb_v, "V")]
        };
        parameters.push(TransformParameterMetadata::text(
            "evidence_scope",
            "simulation_only",
        ));
        parameters
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DitherTransform {
    pub lsb_v: f64,
    pub seed: u64,
}

impl Filter for DitherTransform {
    fn name(&self) -> &'static str {
        "dither"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_positive_parameter("lsb_v", self.lsb_v)?;
        let channels = waveform
            .channels
            .iter()
            .enumerate()
            .map(|(channel_index, channel)| {
                validate_finite_samples(self.name(), &channel.samples)?;
                let mut rng =
                    DeterministicRng::new(self.seed ^ ((channel_index as u64 + 11) << 32));
                let samples = channel
                    .samples
                    .iter()
                    .map(|sample| {
                        let dithered = sample + (rng.unit() - 0.5) * self.lsb_v;
                        quantize_by_lsb(dithered, self.lsb_v, QuantizerRounding::Round)
                    })
                    .collect();
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        let transform_step = simulation_transform_step(
            self.name(),
            format!("dither(lsb_v={},seed={})", self.lsb_v, self.seed),
            TransformCategory::Quantization,
            vec![
                TransformParameterMetadata::float("lsb_v", self.lsb_v, "V"),
                TransformParameterMetadata::integer("seed", self.seed, "seed"),
                TransformParameterMetadata::text("evidence_scope", "simulation_only"),
            ],
            false,
            true,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompandingKind {
    MuLaw,
    ALaw,
}

impl CompandingKind {
    const fn label(self) -> &'static str {
        match self {
            Self::MuLaw => "mu_law",
            Self::ALaw => "a_law",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CompandingTransform {
    pub kind: CompandingKind,
    pub max_v: f64,
    pub mu: f64,
}

impl Filter for CompandingTransform {
    fn name(&self) -> &'static str {
        "companding"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_positive_parameter("max_v", self.max_v)?;
        validate_positive_parameter("mu", self.mu)?;
        let channels = map_finite_samples(waveform, self.name(), |sample| {
            compand_sample(sample, self.kind, self.max_v, self.mu)
        })?;
        let transform_step = simulation_transform_step(
            self.name(),
            format!(
                "companding(mode={},max_v={},mu={})",
                self.kind.label(),
                self.max_v,
                self.mu
            ),
            TransformCategory::Quantization,
            vec![
                TransformParameterMetadata::text("mode", self.kind.label()),
                TransformParameterMetadata::float("max_v", self.max_v, "V"),
                TransformParameterMetadata::float("mu", self.mu, "ratio"),
                TransformParameterMetadata::text("evidence_scope", "simulation_only"),
            ],
            false,
            false,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SampleClockJitterTransform {
    pub jitter_s: f64,
    pub seed: u64,
}

impl Filter for SampleClockJitterTransform {
    fn name(&self) -> &'static str {
        "sample_clock_jitter"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_positive_parameter("jitter_s", self.jitter_s)?;
        validate_time_axis(&waveform.time, self.name())?;
        let min_interval = waveform
            .time
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .fold(f64::INFINITY, f64::min);
        if self.jitter_s >= min_interval * 0.49 {
            return Err(WaveformError::InvalidParameter {
                name: "jitter_s".to_string(),
                reason: "must be less than 49% of the minimum sample interval".to_string(),
            });
        }
        let mut rng = DeterministicRng::new(self.seed);
        let mut time = waveform
            .time
            .iter()
            .map(|sample_time| sample_time + rng.signed_unit() * self.jitter_s)
            .collect::<Vec<_>>();
        time[0] = waveform.time[0];
        validate_time_axis(&time, self.name())?;
        let channels = waveform.channels.clone();
        let transform_step = simulation_transform_step(
            self.name(),
            format!(
                "sample_clock_jitter(jitter_s={},seed={})",
                self.jitter_s, self.seed
            ),
            TransformCategory::FaultInjection,
            vec![
                TransformParameterMetadata::float("jitter_s", self.jitter_s, "s"),
                TransformParameterMetadata::integer("seed", self.seed, "seed"),
                TransformParameterMetadata::text("evidence_scope", "simulation_only"),
            ],
            true,
            true,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform_with_time(waveform, time, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdcCodeDefectKind {
    MissingCode,
    Inl,
    Dnl,
}

impl AdcCodeDefectKind {
    const fn name(self) -> &'static str {
        match self {
            Self::MissingCode => "adc_missing_code",
            Self::Inl => "inl_error",
            Self::Dnl => "dnl_error",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdcCodeDefectTransform {
    pub kind: AdcCodeDefectKind,
    pub bits: u8,
    pub min_v: f64,
    pub max_v: f64,
    pub missing_code: u64,
    pub coefficients: Vec<f64>,
}

impl Filter for AdcCodeDefectTransform {
    fn name(&self) -> &'static str {
        self.kind.name()
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        self.validate()?;
        let max_code = max_adc_code(self.bits);
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_finite_samples(self.name(), &channel.samples)?;
                let samples = channel
                    .samples
                    .iter()
                    .map(|sample| {
                        let code = adc_code_for_sample(*sample, self.bits, self.min_v, self.max_v);
                        let adjusted_code = match self.kind {
                            AdcCodeDefectKind::MissingCode if code == self.missing_code => {
                                if code < max_code {
                                    code + 1
                                } else {
                                    code - 1
                                }
                            }
                            _ => code,
                        };
                        let base =
                            adc_value_for_code(adjusted_code, max_code, self.min_v, self.max_v);
                        let lsb = (self.max_v - self.min_v) / max_code as f64;
                        let error = match self.kind {
                            AdcCodeDefectKind::MissingCode => 0.0,
                            AdcCodeDefectKind::Inl => {
                                let normalized = adjusted_code as f64 / max_code as f64;
                                evaluate_polynomial(&self.coefficients, normalized)
                            }
                            AdcCodeDefectKind::Dnl => {
                                self.coefficients[adjusted_code as usize % self.coefficients.len()]
                                    * lsb
                            }
                        };
                        base + error
                    })
                    .collect();
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        let transform_step = simulation_transform_step(
            self.name(),
            self.adc_defect_history_label(),
            TransformCategory::Quantization,
            self.adc_defect_parameters(),
            false,
            false,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

impl AdcCodeDefectTransform {
    fn validate(&self) -> Result<()> {
        validate_adc_range(self.bits, self.min_v, self.max_v)?;
        match self.kind {
            AdcCodeDefectKind::MissingCode => {
                if self.missing_code > max_adc_code(self.bits) {
                    return Err(WaveformError::InvalidParameter {
                        name: "missing_code".to_string(),
                        reason: "must fit in the configured ADC resolution".to_string(),
                    });
                }
            }
            AdcCodeDefectKind::Inl | AdcCodeDefectKind::Dnl => {
                if self.coefficients.is_empty()
                    || self
                        .coefficients
                        .iter()
                        .any(|coefficient| !coefficient.is_finite())
                {
                    return Err(WaveformError::InvalidParameter {
                        name: "coefficients".to_string(),
                        reason: "must contain finite coefficients".to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    fn adc_defect_history_label(&self) -> String {
        match self.kind {
            AdcCodeDefectKind::MissingCode => format!(
                "{}(bits={},min_v={},max_v={},missing_code={})",
                self.name(),
                self.bits,
                self.min_v,
                self.max_v,
                self.missing_code
            ),
            _ => format!(
                "{}(bits={},min_v={},max_v={},coefficients={})",
                self.name(),
                self.bits,
                self.min_v,
                self.max_v,
                self.coefficients.len()
            ),
        }
    }

    fn adc_defect_parameters(&self) -> Vec<TransformParameterMetadata> {
        let mut parameters = adc_range_parameters(self.bits, self.min_v, self.max_v);
        match self.kind {
            AdcCodeDefectKind::MissingCode => parameters.push(TransformParameterMetadata::integer(
                "missing_code",
                self.missing_code,
                "code",
            )),
            _ => parameters.push(TransformParameterMetadata::integer(
                "coefficients",
                self.coefficients.len() as u64,
                "count",
            )),
        }
        parameters.push(TransformParameterMetadata::text(
            "evidence_scope",
            "simulation_only",
        ));
        parameters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GainOffsetErrorKind {
    Gain,
    Offset,
}

impl GainOffsetErrorKind {
    const fn name(self) -> &'static str {
        match self {
            Self::Gain => "adc_gain_error",
            Self::Offset => "adc_offset_error",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GainOffsetErrorTransform {
    pub kind: GainOffsetErrorKind,
    pub gain_error: f64,
    pub offset_error_v: f64,
}

impl Filter for GainOffsetErrorTransform {
    fn name(&self) -> &'static str {
        self.kind.name()
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        match self.kind {
            GainOffsetErrorKind::Gain => validate_finite_parameter("gain_error", self.gain_error)?,
            GainOffsetErrorKind::Offset => {
                validate_finite_parameter("offset_error_v", self.offset_error_v)?
            }
        }
        let channels = map_finite_samples(waveform, self.name(), |sample| match self.kind {
            GainOffsetErrorKind::Gain => sample * (1.0 + self.gain_error),
            GainOffsetErrorKind::Offset => sample + self.offset_error_v,
        })?;
        let (history_label, parameters) = match self.kind {
            GainOffsetErrorKind::Gain => (
                format!("adc_gain_error(gain_error={})", self.gain_error),
                vec![
                    TransformParameterMetadata::float("gain_error", self.gain_error, "ratio"),
                    TransformParameterMetadata::text("evidence_scope", "simulation_only"),
                ],
            ),
            GainOffsetErrorKind::Offset => (
                format!("adc_offset_error(offset_error_v={})", self.offset_error_v),
                vec![
                    TransformParameterMetadata::float("offset_error_v", self.offset_error_v, "V"),
                    TransformParameterMetadata::text("evidence_scope", "simulation_only"),
                ],
            ),
        };
        let transform_step = simulation_transform_step(
            self.name(),
            history_label,
            TransformCategory::Quantization,
            parameters,
            false,
            false,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelArithmeticKind {
    Add,
    Subtract,
    Differential,
    CommonMode,
}

impl ChannelArithmeticKind {
    const fn name(self) -> &'static str {
        match self {
            Self::Add => "channel_add",
            Self::Subtract => "channel_subtract",
            Self::Differential => "differential_channel",
            Self::CommonMode => "common_mode",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelArithmeticTransform {
    pub kind: ChannelArithmeticKind,
    pub left_channel: String,
    pub right_channel: String,
    pub output_channel: String,
    pub output_unit: Option<String>,
}

impl Filter for ChannelArithmeticTransform {
    fn name(&self) -> &'static str {
        self.kind.name()
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_output_channel_name(&self.output_channel)?;
        let left = required_waveform_channel(waveform, &self.left_channel, self.name())?;
        let right = required_waveform_channel(waveform, &self.right_channel, self.name())?;
        validate_same_unit(self.name(), left, right)?;
        validate_finite_samples(self.name(), &left.samples)?;
        validate_finite_samples(self.name(), &right.samples)?;

        let samples = left
            .samples
            .iter()
            .zip(&right.samples)
            .map(|(left, right)| match self.kind {
                ChannelArithmeticKind::Add => left + right,
                ChannelArithmeticKind::Subtract | ChannelArithmeticKind::Differential => {
                    left - right
                }
                ChannelArithmeticKind::CommonMode => (left + right) * 0.5,
            })
            .collect::<Vec<_>>();
        validate_finite_samples(self.name(), &samples)?;

        let output_unit = configured_or_source_unit(self.output_unit.as_deref(), &left.unit);
        let history_label = format!(
            "{}(left_channel={},right_channel={},output_channel={})",
            self.name(),
            self.left_channel,
            self.right_channel,
            self.output_channel
        );
        let transform_step = m35_transform_step(
            self.name(),
            history_label,
            TransformCategory::MultiChannel,
            channel_pair_parameters(
                &self.left_channel,
                &self.right_channel,
                &self.output_channel,
                output_unit.name.as_str(),
            ),
            m35_streaming_execution(false, false, TransformPhaseEffect::None),
        );
        append_derived_channels(
            waveform,
            vec![Channel::new(
                self.output_channel.clone(),
                output_unit,
                samples,
            )],
            transform_step,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorMagnitudeKind {
    VectorMagnitude,
    EuclideanNorm,
}

impl VectorMagnitudeKind {
    const fn name(self) -> &'static str {
        match self {
            Self::VectorMagnitude => "vector_magnitude",
            Self::EuclideanNorm => "euclidean_norm",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VectorMagnitudeTransform {
    pub kind: VectorMagnitudeKind,
    pub channels: Vec<String>,
    pub output_channel: String,
    pub output_unit: Option<String>,
}

impl Filter for VectorMagnitudeTransform {
    fn name(&self) -> &'static str {
        self.kind.name()
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_output_channel_name(&self.output_channel)?;
        let channels = required_waveform_channels(waveform, &self.channels, self.name(), 2)?;
        validate_matching_units(self.name(), &channels)?;
        let source_unit = channels[0].unit.clone();
        let mut samples = Vec::with_capacity(waveform.sample_count());
        for sample_index in 0..waveform.sample_count() {
            let sum_of_squares = channels
                .iter()
                .map(|channel| {
                    let sample = channel.samples[sample_index];
                    sample * sample
                })
                .sum::<f64>();
            samples.push(sum_of_squares.sqrt());
        }
        validate_finite_samples(self.name(), &samples)?;

        let output_unit = configured_or_source_unit(self.output_unit.as_deref(), &source_unit);
        let history_label = format!(
            "{}(channels={},output_channel={})",
            self.name(),
            self.channels.join("+"),
            self.output_channel
        );
        let transform_step = m35_transform_step(
            self.name(),
            history_label,
            TransformCategory::MultiChannel,
            vec![
                TransformParameterMetadata::integer(
                    "channels",
                    self.channels.len() as u64,
                    "count",
                ),
                TransformParameterMetadata::text("output_channel", &self.output_channel),
                TransformParameterMetadata::text("output_unit", output_unit.name.as_str()),
            ],
            m35_streaming_execution(false, false, TransformPhaseEffect::Nonlinear),
        );
        append_derived_channels(
            waveform,
            vec![Channel::new(
                self.output_channel.clone(),
                output_unit,
                samples,
            )],
            transform_step,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatrixTransform {
    pub input_channels: Vec<String>,
    pub matrix: Vec<Vec<f64>>,
    pub output_channels: Vec<String>,
    pub output_unit: Option<String>,
}

impl Filter for MatrixTransform {
    fn name(&self) -> &'static str {
        "matrix_transform"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let input_channels =
            required_waveform_channels(waveform, &self.input_channels, self.name(), 1)?;
        validate_matching_units(self.name(), &input_channels)?;
        validate_matrix_transform_shape(
            &self.matrix,
            self.input_channels.len(),
            &self.output_channels,
        )?;
        for output_channel in &self.output_channels {
            validate_output_channel_name(output_channel)?;
        }

        let source_unit = input_channels[0].unit.clone();
        let output_unit = configured_or_source_unit(self.output_unit.as_deref(), &source_unit);
        let mut derived = Vec::with_capacity(self.matrix.len());
        for (row_index, row) in self.matrix.iter().enumerate() {
            let samples = (0..waveform.sample_count())
                .map(|sample_index| {
                    row.iter()
                        .zip(&input_channels)
                        .map(|(coefficient, channel)| coefficient * channel.samples[sample_index])
                        .sum::<f64>()
                })
                .collect::<Vec<_>>();
            validate_finite_samples(self.name(), &samples)?;
            derived.push(Channel::new(
                self.output_channels[row_index].clone(),
                output_unit.clone(),
                samples,
            ));
        }

        let history_label = format!(
            "matrix_transform(input_channels={},output_channels={})",
            self.input_channels.len(),
            self.output_channels.len()
        );
        let transform_step = m35_transform_step(
            self.name(),
            history_label,
            TransformCategory::MultiChannel,
            vec![
                TransformParameterMetadata::integer(
                    "input_channels",
                    self.input_channels.len() as u64,
                    "count",
                ),
                TransformParameterMetadata::integer(
                    "output_channels",
                    self.output_channels.len() as u64,
                    "count",
                ),
                TransformParameterMetadata::integer(
                    "matrix_rows",
                    self.matrix.len() as u64,
                    "rows",
                ),
                TransformParameterMetadata::text("output_unit", output_unit.name.as_str()),
            ],
            m35_streaming_execution(false, false, TransformPhaseEffect::None),
        );
        append_derived_channels(waveform, derived, transform_step)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CoordinateRotationTransform {
    pub x_channel: String,
    pub y_channel: String,
    pub angle_rad: f64,
    pub output_x_channel: String,
    pub output_y_channel: String,
    pub output_unit: Option<String>,
}

impl Filter for CoordinateRotationTransform {
    fn name(&self) -> &'static str {
        "coordinate_rotation"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_finite_parameter("angle_rad", self.angle_rad)?;
        validate_output_channel_name(&self.output_x_channel)?;
        validate_output_channel_name(&self.output_y_channel)?;
        let x = required_waveform_channel(waveform, &self.x_channel, self.name())?;
        let y = required_waveform_channel(waveform, &self.y_channel, self.name())?;
        validate_same_unit(self.name(), x, y)?;
        validate_finite_samples(self.name(), &x.samples)?;
        validate_finite_samples(self.name(), &y.samples)?;

        let cos_angle = self.angle_rad.cos();
        let sin_angle = self.angle_rad.sin();
        let rotated_x = x
            .samples
            .iter()
            .zip(&y.samples)
            .map(|(x, y)| cos_angle * x - sin_angle * y)
            .collect::<Vec<_>>();
        let rotated_y = x
            .samples
            .iter()
            .zip(&y.samples)
            .map(|(x, y)| sin_angle * x + cos_angle * y)
            .collect::<Vec<_>>();
        validate_finite_samples(self.name(), &rotated_x)?;
        validate_finite_samples(self.name(), &rotated_y)?;

        let output_unit = configured_or_source_unit(self.output_unit.as_deref(), &x.unit);
        let history_label = format!(
            "coordinate_rotation(x_channel={},y_channel={},angle_rad={})",
            self.x_channel, self.y_channel, self.angle_rad
        );
        let transform_step = m35_transform_step(
            self.name(),
            history_label,
            TransformCategory::MultiChannel,
            vec![
                TransformParameterMetadata::text("x_channel", &self.x_channel),
                TransformParameterMetadata::text("y_channel", &self.y_channel),
                TransformParameterMetadata::float("angle_rad", self.angle_rad, "rad"),
                TransformParameterMetadata::text("output_x_channel", &self.output_x_channel),
                TransformParameterMetadata::text("output_y_channel", &self.output_y_channel),
                TransformParameterMetadata::text("output_unit", output_unit.name.as_str()),
            ],
            m35_streaming_execution(false, false, TransformPhaseEffect::None),
        );
        append_derived_channels(
            waveform,
            vec![
                Channel::new(
                    self.output_x_channel.clone(),
                    output_unit.clone(),
                    rotated_x,
                ),
                Channel::new(self.output_y_channel.clone(), output_unit, rotated_y),
            ],
            transform_step,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorConversionKind {
    Linear,
    Pressure,
    CurrentShunt,
    BridgeStrain,
    LoadCell,
    Rtd,
    Thermistor,
    TachometerRpm,
    EncoderPosition,
    Accelerometer,
    Gyroscope,
    HallCurrent,
    LvdtPosition,
    MicrophoneSpl,
    PhotodiodePower,
}

impl SensorConversionKind {
    const fn name(self) -> &'static str {
        match self {
            Self::Linear => "linear_sensor_conversion",
            Self::Pressure => "pressure_transducer",
            Self::CurrentShunt => "current_shunt",
            Self::BridgeStrain => "bridge_strain",
            Self::LoadCell => "load_cell_force",
            Self::Rtd => "rtd_temperature",
            Self::Thermistor => "thermistor_temperature",
            Self::TachometerRpm => "tachometer_rpm",
            Self::EncoderPosition => "encoder_position",
            Self::Accelerometer => "accelerometer_units",
            Self::Gyroscope => "gyroscope_rate",
            Self::HallCurrent => "hall_current",
            Self::LvdtPosition => "lvdt_position",
            Self::MicrophoneSpl => "microphone_spl",
            Self::PhotodiodePower => "photodiode_power",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SensorConversionParameters {
    pub input_min_v: Option<f64>,
    pub input_max_v: Option<f64>,
    pub output_min: Option<f64>,
    pub output_max: Option<f64>,
    pub shunt_ohms: Option<f64>,
    pub excitation_v: Option<f64>,
    pub gauge_factor: Option<f64>,
    pub sensitivity_mv_v: Option<f64>,
    pub full_scale: Option<f64>,
    pub r0_ohm: Option<f64>,
    pub alpha_per_c: Option<f64>,
    pub beta_k: Option<f64>,
    pub t0_c: Option<f64>,
    pub pulses_per_rev: Option<f64>,
    pub counts_per_rev: Option<f64>,
    pub scale_per_rev: Option<f64>,
    pub sensitivity_v_per_unit: Option<f64>,
    pub bias_v: Option<f64>,
    pub reference: Option<f64>,
    pub responsivity_a_per_w: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SensorConversionTransform {
    pub kind: SensorConversionKind,
    pub channel: String,
    pub output_channel: String,
    pub output_unit: String,
    pub parameters: SensorConversionParameters,
}

impl Filter for SensorConversionTransform {
    fn name(&self) -> &'static str {
        self.kind.name()
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_output_channel_name(&self.output_channel)?;
        validate_output_channel_name(&self.output_unit)?;
        let channel = required_waveform_channel(waveform, &self.channel, self.name())?;
        validate_finite_samples(self.name(), &channel.samples)?;

        let samples = channel
            .samples
            .iter()
            .copied()
            .map(|sample| self.convert_sample(sample))
            .collect::<Result<Vec<_>>>()?;
        validate_finite_samples(self.name(), &samples)?;

        let history_label = format!(
            "{}(channel={},output_channel={})",
            self.name(),
            self.channel,
            self.output_channel
        );
        let transform_step = m35_transform_step(
            self.name(),
            history_label,
            TransformCategory::Calibration,
            self.sensor_parameters()?,
            m35_streaming_execution(false, false, self.phase_effect()),
        );
        append_derived_channels(
            waveform,
            vec![Channel::new(
                self.output_channel.clone(),
                Unit::new(self.output_unit.clone()),
                samples,
            )],
            transform_step,
        )
    }
}

impl SensorConversionTransform {
    fn convert_sample(&self, sample: f64) -> Result<f64> {
        validate_finite_parameter("sensor_sample", sample)?;
        match self.kind {
            SensorConversionKind::Linear | SensorConversionKind::Pressure => {
                let input_min = self.required_finite("input_min_v")?;
                let input_max = self.required_finite("input_max_v")?;
                let output_min = self.required_finite("output_min")?;
                let output_max = self.required_finite("output_max")?;
                if input_max <= input_min {
                    return invalid_m35_parameter(
                        "input_max_v",
                        "must be greater than input_min_v",
                    );
                }
                Ok(output_min
                    + ((sample - input_min) / (input_max - input_min)) * (output_max - output_min))
            }
            SensorConversionKind::CurrentShunt => {
                Ok(sample / self.required_positive("shunt_ohms")?)
            }
            SensorConversionKind::BridgeStrain => {
                let excitation_v = self.required_positive("excitation_v")?;
                let gauge_factor = self.required_positive("gauge_factor")?;
                Ok((4.0 * sample / excitation_v) / gauge_factor)
            }
            SensorConversionKind::LoadCell => {
                let excitation_v = self.required_positive("excitation_v")?;
                let sensitivity_mv_v = self.required_positive("sensitivity_mv_v")?;
                let full_scale = self.required_finite("full_scale")?;
                Ok((sample / excitation_v) / (sensitivity_mv_v / 1000.0) * full_scale)
            }
            SensorConversionKind::Rtd => {
                let r0 = self.required_positive("r0_ohm")?;
                let alpha = self.required_positive("alpha_per_c")?;
                Ok((sample - r0) / (r0 * alpha))
            }
            SensorConversionKind::Thermistor => {
                let r0 = self.required_positive("r0_ohm")?;
                let beta = self.required_positive("beta_k")?;
                let t0_c = self.required_finite("t0_c")?;
                if sample <= 0.0 {
                    return invalid_m35_parameter(
                        "sensor_sample",
                        "thermistor resistance samples must be greater than zero",
                    );
                }
                let t0_k = t0_c + 273.15;
                if t0_k <= 0.0 {
                    return invalid_m35_parameter("t0_c", "must be greater than -273.15");
                }
                Ok(1.0 / (1.0 / t0_k + (sample / r0).ln() / beta) - 273.15)
            }
            SensorConversionKind::TachometerRpm => {
                let pulses_per_rev = self.required_positive("pulses_per_rev")?;
                Ok(sample * 60.0 / pulses_per_rev)
            }
            SensorConversionKind::EncoderPosition => {
                let counts_per_rev = self.required_positive("counts_per_rev")?;
                let scale_per_rev = self.required_finite("scale_per_rev")?;
                Ok(sample / counts_per_rev * scale_per_rev)
            }
            SensorConversionKind::Accelerometer
            | SensorConversionKind::Gyroscope
            | SensorConversionKind::HallCurrent
            | SensorConversionKind::LvdtPosition => {
                let sensitivity = self.required_positive("sensitivity_v_per_unit")?;
                let bias = self.parameters.bias_v.unwrap_or(0.0);
                validate_finite_parameter("bias_v", bias)?;
                Ok((sample - bias) / sensitivity)
            }
            SensorConversionKind::MicrophoneSpl => {
                let reference = self.required_positive("reference")?;
                let pressure = sample.abs();
                if pressure <= 0.0 {
                    return invalid_m35_parameter(
                        "sensor_sample",
                        "microphone SPL requires non-zero pressure samples",
                    );
                }
                Ok(20.0 * (pressure / reference).log10())
            }
            SensorConversionKind::PhotodiodePower => {
                Ok(sample / self.required_positive("responsivity_a_per_w")?)
            }
        }
    }

    fn sensor_parameters(&self) -> Result<Vec<TransformParameterMetadata>> {
        let mut parameters = vec![
            TransformParameterMetadata::text("channel", &self.channel),
            TransformParameterMetadata::text("output_channel", &self.output_channel),
            TransformParameterMetadata::text("output_unit", &self.output_unit),
            TransformParameterMetadata::text("calibration_scope", "software_formula_only"),
        ];
        match self.kind {
            SensorConversionKind::Linear | SensorConversionKind::Pressure => {
                parameters.push(TransformParameterMetadata::float(
                    "input_min_v",
                    self.required_finite("input_min_v")?,
                    "V",
                ));
                parameters.push(TransformParameterMetadata::float(
                    "input_max_v",
                    self.required_finite("input_max_v")?,
                    "V",
                ));
                parameters.push(TransformParameterMetadata::float(
                    "output_min",
                    self.required_finite("output_min")?,
                    self.output_unit.as_str(),
                ));
                parameters.push(TransformParameterMetadata::float(
                    "output_max",
                    self.required_finite("output_max")?,
                    self.output_unit.as_str(),
                ));
            }
            SensorConversionKind::CurrentShunt => {
                parameters.push(TransformParameterMetadata::float(
                    "shunt_ohms",
                    self.required_positive("shunt_ohms")?,
                    "ohm",
                ))
            }
            SensorConversionKind::BridgeStrain => {
                parameters.push(TransformParameterMetadata::float(
                    "excitation_v",
                    self.required_positive("excitation_v")?,
                    "V",
                ));
                parameters.push(TransformParameterMetadata::float(
                    "gauge_factor",
                    self.required_positive("gauge_factor")?,
                    "ratio",
                ));
            }
            SensorConversionKind::LoadCell => {
                parameters.push(TransformParameterMetadata::float(
                    "excitation_v",
                    self.required_positive("excitation_v")?,
                    "V",
                ));
                parameters.push(TransformParameterMetadata::float(
                    "sensitivity_mv_v",
                    self.required_positive("sensitivity_mv_v")?,
                    "mV/V",
                ));
                parameters.push(TransformParameterMetadata::float(
                    "full_scale",
                    self.required_finite("full_scale")?,
                    self.output_unit.as_str(),
                ));
            }
            SensorConversionKind::Rtd => {
                parameters.push(TransformParameterMetadata::float(
                    "r0_ohm",
                    self.required_positive("r0_ohm")?,
                    "ohm",
                ));
                parameters.push(TransformParameterMetadata::float(
                    "alpha_per_c",
                    self.required_positive("alpha_per_c")?,
                    "1/C",
                ));
            }
            SensorConversionKind::Thermistor => {
                parameters.push(TransformParameterMetadata::float(
                    "r0_ohm",
                    self.required_positive("r0_ohm")?,
                    "ohm",
                ));
                parameters.push(TransformParameterMetadata::float(
                    "beta_k",
                    self.required_positive("beta_k")?,
                    "K",
                ));
                parameters.push(TransformParameterMetadata::float(
                    "t0_c",
                    self.required_finite("t0_c")?,
                    "C",
                ));
            }
            SensorConversionKind::TachometerRpm => {
                parameters.push(TransformParameterMetadata::float(
                    "pulses_per_rev",
                    self.required_positive("pulses_per_rev")?,
                    "pulses/rev",
                ));
            }
            SensorConversionKind::EncoderPosition => {
                parameters.push(TransformParameterMetadata::float(
                    "counts_per_rev",
                    self.required_positive("counts_per_rev")?,
                    "counts/rev",
                ));
                parameters.push(TransformParameterMetadata::float(
                    "scale_per_rev",
                    self.required_finite("scale_per_rev")?,
                    self.output_unit.as_str(),
                ));
            }
            SensorConversionKind::Accelerometer
            | SensorConversionKind::Gyroscope
            | SensorConversionKind::HallCurrent
            | SensorConversionKind::LvdtPosition => {
                parameters.push(TransformParameterMetadata::float(
                    "sensitivity_v_per_unit",
                    self.required_positive("sensitivity_v_per_unit")?,
                    "V/unit",
                ));
                parameters.push(TransformParameterMetadata::float(
                    "bias_v",
                    self.parameters.bias_v.unwrap_or(0.0),
                    "V",
                ));
            }
            SensorConversionKind::MicrophoneSpl => {
                parameters.push(TransformParameterMetadata::float(
                    "reference",
                    self.required_positive("reference")?,
                    "reference",
                ))
            }
            SensorConversionKind::PhotodiodePower => {
                parameters.push(TransformParameterMetadata::float(
                    "responsivity_a_per_w",
                    self.required_positive("responsivity_a_per_w")?,
                    "A/W",
                ))
            }
        }
        Ok(parameters)
    }

    fn phase_effect(&self) -> TransformPhaseEffect {
        match self.kind {
            SensorConversionKind::Rtd
            | SensorConversionKind::Thermistor
            | SensorConversionKind::MicrophoneSpl => TransformPhaseEffect::Nonlinear,
            _ => TransformPhaseEffect::None,
        }
    }

    fn required_finite(&self, field: &str) -> Result<f64> {
        let value = match field {
            "input_min_v" => self.parameters.input_min_v,
            "input_max_v" => self.parameters.input_max_v,
            "output_min" => self.parameters.output_min,
            "output_max" => self.parameters.output_max,
            "full_scale" => self.parameters.full_scale,
            "t0_c" => self.parameters.t0_c,
            "scale_per_rev" => self.parameters.scale_per_rev,
            _ => None,
        }
        .ok_or_else(|| WaveformError::InvalidParameter {
            name: field.to_string(),
            reason: "field is required for this sensor conversion".to_string(),
        })?;
        validate_finite_parameter(field, value)?;
        Ok(value)
    }

    fn required_positive(&self, field: &str) -> Result<f64> {
        let value = match field {
            "shunt_ohms" => self.parameters.shunt_ohms,
            "excitation_v" => self.parameters.excitation_v,
            "gauge_factor" => self.parameters.gauge_factor,
            "sensitivity_mv_v" => self.parameters.sensitivity_mv_v,
            "r0_ohm" => self.parameters.r0_ohm,
            "alpha_per_c" => self.parameters.alpha_per_c,
            "beta_k" => self.parameters.beta_k,
            "pulses_per_rev" => self.parameters.pulses_per_rev,
            "counts_per_rev" => self.parameters.counts_per_rev,
            "sensitivity_v_per_unit" => self.parameters.sensitivity_v_per_unit,
            "reference" => self.parameters.reference,
            "responsivity_a_per_w" => self.parameters.responsivity_a_per_w,
            _ => None,
        }
        .ok_or_else(|| WaveformError::InvalidParameter {
            name: field.to_string(),
            reason: "field is required for this sensor conversion".to_string(),
        })?;
        validate_positive_parameter(field, value)?;
        Ok(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VibrationTransformKind {
    VelocityFromAcceleration,
    DisplacementFromVelocity,
    VibrationSeverity,
}

impl VibrationTransformKind {
    const fn name(self) -> &'static str {
        match self {
            Self::VelocityFromAcceleration => "velocity_from_acceleration",
            Self::DisplacementFromVelocity => "displacement_from_velocity",
            Self::VibrationSeverity => "vibration_severity",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VibrationTransform {
    pub kind: VibrationTransformKind,
    pub channel: String,
    pub output_channel: String,
    pub output_unit: String,
    pub window_samples: usize,
}

impl Filter for VibrationTransform {
    fn name(&self) -> &'static str {
        self.kind.name()
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_output_channel_name(&self.output_channel)?;
        validate_output_channel_name(&self.output_unit)?;
        let channel = required_waveform_channel(waveform, &self.channel, self.name())?;
        validate_finite_samples(self.name(), &channel.samples)?;

        let (samples, execution, parameters) = match self.kind {
            VibrationTransformKind::VelocityFromAcceleration
            | VibrationTransformKind::DisplacementFromVelocity => {
                validate_time_axis(&waveform.time, self.name())?;
                let samples = cumulative_integral_samples(&waveform.time, &channel.samples)?;
                (
                    samples,
                    m35_streaming_execution(true, true, TransformPhaseEffect::None),
                    vec![
                        TransformParameterMetadata::text("channel", &self.channel),
                        TransformParameterMetadata::text("output_channel", &self.output_channel),
                        TransformParameterMetadata::text("output_unit", &self.output_unit),
                    ],
                )
            }
            VibrationTransformKind::VibrationSeverity => {
                if self.window_samples == 0 {
                    return invalid_m35_parameter("window_samples", "must be greater than zero");
                }
                (
                    moving_rms_samples(&channel.samples, self.window_samples),
                    m35_streaming_execution(false, true, TransformPhaseEffect::Delay),
                    vec![
                        TransformParameterMetadata::text("channel", &self.channel),
                        TransformParameterMetadata::text("output_channel", &self.output_channel),
                        TransformParameterMetadata::text("output_unit", &self.output_unit),
                        TransformParameterMetadata::integer(
                            "window_samples",
                            self.window_samples as u64,
                            "samples",
                        ),
                    ],
                )
            }
        };
        validate_finite_samples(self.name(), &samples)?;

        let history_label = format!(
            "{}(channel={},output_channel={})",
            self.name(),
            self.channel,
            self.output_channel
        );
        let transform_step = m35_transform_step(
            self.name(),
            history_label,
            TransformCategory::Calibration,
            parameters,
            execution,
        );
        append_derived_channels(
            waveform,
            vec![Channel::new(
                self.output_channel.clone(),
                Unit::new(self.output_unit.clone()),
                samples,
            )],
            transform_step,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlTransformKind {
    ErrorSignal,
    ProportionalControl,
    PidControl,
    RateLimiter,
    SlewRateLimit,
    ControlSaturation,
    ControlDeadzone,
    FeedforwardControl,
}

impl ControlTransformKind {
    const fn name(self) -> &'static str {
        match self {
            Self::ErrorSignal => "control_error",
            Self::ProportionalControl => "proportional_control",
            Self::PidControl => "pid_control",
            Self::RateLimiter => "rate_limiter",
            Self::SlewRateLimit => "slew_rate_limit",
            Self::ControlSaturation => "control_saturation",
            Self::ControlDeadzone => "control_deadzone",
            Self::FeedforwardControl => "feedforward_control",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ControlTransform {
    pub kind: ControlTransformKind,
    pub channel: String,
    pub output_channel: String,
    pub output_unit: Option<String>,
    pub setpoint: f64,
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
    pub rate_limit_per_s: f64,
    pub min_v: f64,
    pub max_v: f64,
    pub threshold_v: f64,
    pub feedforward_gain: f64,
    pub feedforward_offset: f64,
}

impl Filter for ControlTransform {
    fn name(&self) -> &'static str {
        self.kind.name()
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_output_channel_name(&self.output_channel)?;
        let channel = required_waveform_channel(waveform, &self.channel, self.name())?;
        validate_finite_samples(self.name(), &channel.samples)?;

        let (samples, execution, parameters) = match self.kind {
            ControlTransformKind::ErrorSignal => {
                validate_finite_parameter("setpoint", self.setpoint)?;
                (
                    channel
                        .samples
                        .iter()
                        .map(|sample| self.setpoint - sample)
                        .collect::<Vec<_>>(),
                    m35_streaming_execution(false, false, TransformPhaseEffect::None),
                    vec![
                        TransformParameterMetadata::text("channel", &self.channel),
                        TransformParameterMetadata::text("output_channel", &self.output_channel),
                        TransformParameterMetadata::float("setpoint", self.setpoint, "unit"),
                    ],
                )
            }
            ControlTransformKind::ProportionalControl => {
                validate_finite_parameter("setpoint", self.setpoint)?;
                validate_finite_parameter("kp", self.kp)?;
                (
                    channel
                        .samples
                        .iter()
                        .map(|sample| self.kp * (self.setpoint - sample))
                        .collect::<Vec<_>>(),
                    m35_streaming_execution(false, false, TransformPhaseEffect::None),
                    vec![
                        TransformParameterMetadata::text("channel", &self.channel),
                        TransformParameterMetadata::text("output_channel", &self.output_channel),
                        TransformParameterMetadata::float("setpoint", self.setpoint, "unit"),
                        TransformParameterMetadata::float("kp", self.kp, "ratio"),
                    ],
                )
            }
            ControlTransformKind::PidControl => {
                validate_time_axis(&waveform.time, self.name())?;
                validate_finite_parameter("setpoint", self.setpoint)?;
                validate_finite_parameter("kp", self.kp)?;
                validate_finite_parameter("ki", self.ki)?;
                validate_finite_parameter("kd", self.kd)?;
                (
                    pid_samples(
                        &waveform.time,
                        &channel.samples,
                        self.setpoint,
                        self.kp,
                        self.ki,
                        self.kd,
                    ),
                    m35_streaming_execution(true, true, TransformPhaseEffect::Delay),
                    vec![
                        TransformParameterMetadata::text("channel", &self.channel),
                        TransformParameterMetadata::text("output_channel", &self.output_channel),
                        TransformParameterMetadata::float("setpoint", self.setpoint, "unit"),
                        TransformParameterMetadata::float("kp", self.kp, "ratio"),
                        TransformParameterMetadata::float("ki", self.ki, "1/s"),
                        TransformParameterMetadata::float("kd", self.kd, "s"),
                    ],
                )
            }
            ControlTransformKind::RateLimiter | ControlTransformKind::SlewRateLimit => {
                validate_time_axis(&waveform.time, self.name())?;
                validate_positive_parameter("rate_limit_per_s", self.rate_limit_per_s)?;
                (
                    rate_limited_samples(&waveform.time, &channel.samples, self.rate_limit_per_s),
                    m35_streaming_execution(true, true, TransformPhaseEffect::Delay),
                    vec![
                        TransformParameterMetadata::text("channel", &self.channel),
                        TransformParameterMetadata::text("output_channel", &self.output_channel),
                        TransformParameterMetadata::float(
                            "rate_limit_per_s",
                            self.rate_limit_per_s,
                            "unit/s",
                        ),
                    ],
                )
            }
            ControlTransformKind::ControlSaturation => {
                validate_finite_parameter("min_v", self.min_v)?;
                validate_finite_parameter("max_v", self.max_v)?;
                if self.max_v < self.min_v {
                    return invalid_m35_parameter(
                        "max_v",
                        "must be greater than or equal to min_v",
                    );
                }
                (
                    channel
                        .samples
                        .iter()
                        .map(|sample| sample.clamp(self.min_v, self.max_v))
                        .collect::<Vec<_>>(),
                    m35_streaming_execution(false, false, TransformPhaseEffect::Nonlinear),
                    vec![
                        TransformParameterMetadata::text("channel", &self.channel),
                        TransformParameterMetadata::text("output_channel", &self.output_channel),
                        TransformParameterMetadata::float("min_v", self.min_v, "V"),
                        TransformParameterMetadata::float("max_v", self.max_v, "V"),
                    ],
                )
            }
            ControlTransformKind::ControlDeadzone => {
                validate_positive_parameter("threshold_v", self.threshold_v)?;
                (
                    channel
                        .samples
                        .iter()
                        .map(|sample| {
                            if sample.abs() < self.threshold_v {
                                0.0
                            } else {
                                *sample
                            }
                        })
                        .collect::<Vec<_>>(),
                    m35_streaming_execution(false, false, TransformPhaseEffect::Nonlinear),
                    vec![
                        TransformParameterMetadata::text("channel", &self.channel),
                        TransformParameterMetadata::text("output_channel", &self.output_channel),
                        TransformParameterMetadata::float("threshold_v", self.threshold_v, "V"),
                    ],
                )
            }
            ControlTransformKind::FeedforwardControl => {
                validate_finite_parameter("feedforward_gain", self.feedforward_gain)?;
                validate_finite_parameter("feedforward_offset", self.feedforward_offset)?;
                (
                    channel
                        .samples
                        .iter()
                        .map(|sample| self.feedforward_gain * sample + self.feedforward_offset)
                        .collect::<Vec<_>>(),
                    m35_streaming_execution(false, false, TransformPhaseEffect::None),
                    vec![
                        TransformParameterMetadata::text("channel", &self.channel),
                        TransformParameterMetadata::text("output_channel", &self.output_channel),
                        TransformParameterMetadata::float(
                            "feedforward_gain",
                            self.feedforward_gain,
                            "ratio",
                        ),
                        TransformParameterMetadata::float(
                            "feedforward_offset",
                            self.feedforward_offset,
                            "unit",
                        ),
                    ],
                )
            }
        };
        validate_finite_samples(self.name(), &samples)?;

        let output_unit = configured_or_source_unit(self.output_unit.as_deref(), &channel.unit);
        let history_label = format!(
            "{}(channel={},output_channel={})",
            self.name(),
            self.channel,
            self.output_channel
        );
        let transform_step = m35_transform_step(
            self.name(),
            history_label,
            TransformCategory::Control,
            parameters,
            execution,
        );
        append_derived_channels(
            waveform,
            vec![Channel::new(
                self.output_channel.clone(),
                output_unit,
                samples,
            )],
            transform_step,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AbsoluteValueTransform;

impl Filter for AbsoluteValueTransform {
    fn name(&self) -> &'static str {
        "absolute_value"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let channels = map_samples(waveform, f64::abs)?;
        let transform_step = TransformStepMetadata::implemented_desktop(
            "absolute_value()",
            "absolute_value",
            TransformCategory::Pointwise,
            Vec::new(),
            false,
            false,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SquareTransform;

impl Filter for SquareTransform {
    fn name(&self) -> &'static str {
        "square"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let channels = map_samples(waveform, |sample| sample * sample)?;
        let transform_step = TransformStepMetadata::implemented_desktop(
            "square()",
            "square",
            TransformCategory::Pointwise,
            Vec::new(),
            false,
            false,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SquareRootTransform;

impl Filter for SquareRootTransform {
    fn name(&self) -> &'static str {
        "square_root"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let channels = map_samples_checked(waveform, "square_root", |sample| {
            if sample < 0.0 {
                return Err(WaveformError::InvalidWaveform {
                    reason: "square_root requires non-negative samples".to_string(),
                });
            }
            Ok(sample.sqrt())
        })?;
        let transform_step = TransformStepMetadata::implemented_desktop(
            "square_root()",
            "square_root",
            TransformCategory::Pointwise,
            Vec::new(),
            false,
            false,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LogTransform {
    pub base: f64,
}

impl Filter for LogTransform {
    fn name(&self) -> &'static str {
        "log"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_log_exp_base(self.base)?;
        let channels = map_samples_checked(waveform, "log", |sample| {
            if sample <= 0.0 {
                return Err(WaveformError::InvalidWaveform {
                    reason: "log requires positive samples".to_string(),
                });
            }
            Ok(sample.log(self.base))
        })?;
        let history_label = format!("log(base={})", self.base);
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "log",
            TransformCategory::Pointwise,
            vec![TransformParameterMetadata::float(
                "base", self.base, "ratio",
            )],
            false,
            false,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExpTransform {
    pub base: f64,
}

impl Filter for ExpTransform {
    fn name(&self) -> &'static str {
        "exp"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_log_exp_base(self.base)?;
        let channels = map_samples_checked(waveform, "exp", |sample| Ok(self.base.powf(sample)))?;
        let history_label = format!("exp(base={})", self.base);
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "exp",
            TransformCategory::Pointwise,
            vec![TransformParameterMetadata::float(
                "base", self.base, "ratio",
            )],
            false,
            false,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NormalizeMode {
    ZeroToOne,
    MinusOneToOne,
    ZScore,
    Range {
        input_min_v: f64,
        input_max_v: f64,
        output_min: f64,
        output_max: f64,
    },
}

impl NormalizeMode {
    pub fn from_config(value: &str) -> Option<Self> {
        match value {
            "zero_to_one" => Some(Self::ZeroToOne),
            "minus_one_to_one" => Some(Self::MinusOneToOne),
            "z_score" => Some(Self::ZScore),
            _ => None,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ZeroToOne => "zero_to_one",
            Self::MinusOneToOne => "minus_one_to_one",
            Self::ZScore => "z_score",
            Self::Range { .. } => "range",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NormalizeTransform {
    pub mode: NormalizeMode,
}

impl Filter for NormalizeTransform {
    fn name(&self) -> &'static str {
        "normalize"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_finite_samples("normalize", &channel.samples)?;
                let samples = normalize_samples(&channel.samples, self.mode)?;
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let history_label = match self.mode {
            NormalizeMode::Range {
                input_min_v,
                input_max_v,
                output_min,
                output_max,
            } => format!(
                "normalize(mode=range,input_min_v={input_min_v},input_max_v={input_max_v},output_min={output_min},output_max={output_max})"
            ),
            mode => format!("normalize(mode={})", mode.as_str()),
        };
        let mut parameters = vec![TransformParameterMetadata::text("mode", self.mode.as_str())];
        if let NormalizeMode::Range {
            input_min_v,
            input_max_v,
            output_min,
            output_max,
        } = self.mode
        {
            parameters.extend([
                TransformParameterMetadata::float("input_min_v", input_min_v, "V"),
                TransformParameterMetadata::float("input_max_v", input_max_v, "V"),
                TransformParameterMetadata::float("output_min", output_min, "ratio"),
                TransformParameterMetadata::float("output_max", output_max, "ratio"),
            ]);
        }
        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            history_label,
            "normalize",
            TransformCategory::Pointwise,
            parameters,
            offline_execution(false, true, TransformPhaseEffect::None),
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TanhTransform;

impl Filter for TanhTransform {
    fn name(&self) -> &'static str {
        "tanh"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let channels = map_samples(waveform, f64::tanh)?;
        let transform_step = TransformStepMetadata::implemented_desktop(
            "tanh()",
            "tanh",
            TransformCategory::Pointwise,
            Vec::new(),
            false,
            false,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SigmoidTransform;

impl Filter for SigmoidTransform {
    fn name(&self) -> &'static str {
        "sigmoid"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let channels = map_samples(waveform, |sample| 1.0 / (1.0 + (-sample).exp()))?;
        let transform_step = TransformStepMetadata::implemented_desktop(
            "sigmoid()",
            "sigmoid",
            TransformCategory::Pointwise,
            Vec::new(),
            false,
            false,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SoftLimitTransform {
    pub limit_v: f64,
}

impl Filter for SoftLimitTransform {
    fn name(&self) -> &'static str {
        "soft_limit"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_positive_parameter("limit_v", self.limit_v)?;
        let channels = map_samples(waveform, |sample| {
            self.limit_v * (sample / self.limit_v).tanh()
        })?;
        let history_label = format!("soft_limit(limit_v={})", self.limit_v);
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "soft_limit",
            TransformCategory::Pointwise,
            vec![TransformParameterMetadata::float(
                "limit_v",
                self.limit_v,
                "V",
            )],
            false,
            false,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PiecewisePoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PiecewiseLinearTransform {
    pub points: Vec<PiecewisePoint>,
}

impl Filter for PiecewiseLinearTransform {
    fn name(&self) -> &'static str {
        "piecewise_linear"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_piecewise_points(&self.points)?;
        let channels = map_samples(waveform, |sample| {
            interpolate_piecewise(&self.points, sample)
        })?;
        let history_label = format!("piecewise_linear(points={})", self.points.len());
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "piecewise_linear",
            TransformCategory::Pointwise,
            vec![TransformParameterMetadata::integer(
                "point_count",
                self.points.len() as u64,
                "points",
            )],
            false,
            false,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PolynomialTransform {
    pub coefficients: Vec<f64>,
}

impl Filter for PolynomialTransform {
    fn name(&self) -> &'static str {
        "polynomial"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_polynomial_coefficients(&self.coefficients)?;
        let channels = map_samples(waveform, |sample| {
            evaluate_polynomial(&self.coefficients, sample)
        })?;
        let history_label = format!("polynomial(coefficients={})", self.coefficients.len());
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "polynomial",
            TransformCategory::Pointwise,
            vec![TransformParameterMetadata::integer(
                "coefficient_count",
                self.coefficients.len() as u64,
                "coefficients",
            )],
            false,
            false,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WeightedMovingAverageFilter {
    pub weights: Vec<f64>,
}

impl Filter for WeightedMovingAverageFilter {
    fn name(&self) -> &'static str {
        "weighted_moving_average"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_weights(&self.weights)?;
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_finite_samples("weighted_moving_average", &channel.samples)?;
                let samples = trailing_weighted_moving_average(&channel.samples, &self.weights);
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let history_label = format!("weighted_moving_average(weights={})", self.weights.len());
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "weighted_moving_average",
            TransformCategory::Windowed,
            vec![TransformParameterMetadata::integer(
                "weight_count",
                self.weights.len() as u64,
                "weights",
            )],
            false,
            true,
            TransformPhaseEffect::Delay,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExponentialMovingAverageFilter {
    pub alpha: f64,
}

impl Filter for ExponentialMovingAverageFilter {
    fn name(&self) -> &'static str {
        "exponential_moving_average"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_alpha(self.alpha)?;
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_finite_samples("exponential_moving_average", &channel.samples)?;
                let samples = exponential_moving_average(&channel.samples, self.alpha);
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let history_label = format!("exponential_moving_average(alpha={})", self.alpha);
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "exponential_moving_average",
            TransformCategory::Stateful,
            vec![TransformParameterMetadata::float(
                "alpha", self.alpha, "ratio",
            )],
            false,
            true,
            TransformPhaseEffect::Delay,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoxcarSmoothingFilter {
    pub window_samples: usize,
}

impl Filter for BoxcarSmoothingFilter {
    fn name(&self) -> &'static str {
        "boxcar_smoothing"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_window_samples(self.window_samples)?;
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_finite_samples("boxcar_smoothing", &channel.samples)?;
                let samples = centered_boxcar_smoothing(&channel.samples, self.window_samples);
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let history_label = format!("boxcar_smoothing(window_samples={})", self.window_samples);
        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            history_label,
            "boxcar_smoothing",
            TransformCategory::Windowed,
            vec![TransformParameterMetadata::integer(
                "window_samples",
                self.window_samples as u64,
                "samples",
            )],
            offline_execution(false, true, TransformPhaseEffect::None),
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GaussianSmoothingFilter {
    pub window_samples: usize,
    pub sigma_samples: f64,
}

impl Filter for GaussianSmoothingFilter {
    fn name(&self) -> &'static str {
        "gaussian_smoothing"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_window_samples(self.window_samples)?;
        validate_positive_parameter("sigma_samples", self.sigma_samples)?;
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_finite_samples("gaussian_smoothing", &channel.samples)?;
                let samples = centered_gaussian_smoothing(
                    &channel.samples,
                    self.window_samples,
                    self.sigma_samples,
                );
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let history_label = format!(
            "gaussian_smoothing(window_samples={},sigma_samples={})",
            self.window_samples, self.sigma_samples
        );
        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            history_label,
            "gaussian_smoothing",
            TransformCategory::Windowed,
            vec![
                TransformParameterMetadata::integer(
                    "window_samples",
                    self.window_samples as u64,
                    "samples",
                ),
                TransformParameterMetadata::float("sigma_samples", self.sigma_samples, "samples"),
            ],
            offline_execution(false, true, TransformPhaseEffect::None),
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SavitzkyGolayFilter {
    pub window_samples: usize,
    pub polynomial_order: usize,
}

impl Filter for SavitzkyGolayFilter {
    fn name(&self) -> &'static str {
        "savitzky_golay"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_savitzky_golay_parameters(self.window_samples, self.polynomial_order)?;
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_finite_samples("savitzky_golay", &channel.samples)?;
                let samples = savitzky_golay_smoothing(
                    &channel.samples,
                    self.window_samples,
                    self.polynomial_order,
                )?;
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let history_label = format!(
            "savitzky_golay(window_samples={},polynomial_order={})",
            self.window_samples, self.polynomial_order
        );
        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            history_label,
            "savitzky_golay",
            TransformCategory::Windowed,
            vec![
                TransformParameterMetadata::integer(
                    "window_samples",
                    self.window_samples as u64,
                    "samples",
                ),
                TransformParameterMetadata::integer(
                    "polynomial_order",
                    self.polynomial_order as u64,
                    "order",
                ),
            ],
            offline_execution(false, true, TransformPhaseEffect::None),
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CenteredMovingMedianFilter {
    pub window_samples: usize,
}

impl Filter for CenteredMovingMedianFilter {
    fn name(&self) -> &'static str {
        "centered_moving_median"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_window_samples(self.window_samples)?;
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_finite_samples("centered_moving_median", &channel.samples)?;
                let samples = centered_moving_median(&channel.samples, self.window_samples);
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let history_label = format!(
            "centered_moving_median(window_samples={})",
            self.window_samples
        );
        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            history_label,
            "centered_moving_median",
            TransformCategory::Windowed,
            vec![TransformParameterMetadata::integer(
                "window_samples",
                self.window_samples as u64,
                "samples",
            )],
            offline_execution(false, true, TransformPhaseEffect::Nonlinear),
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RollingMeanBaselineTransform {
    pub window_samples: usize,
}

impl Filter for RollingMeanBaselineTransform {
    fn name(&self) -> &'static str {
        "rolling_mean_baseline"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_window_samples(self.window_samples)?;
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_finite_samples("rolling_mean_baseline", &channel.samples)?;
                let baseline = trailing_moving_average(&channel.samples, self.window_samples);
                let samples = channel
                    .samples
                    .iter()
                    .zip(baseline)
                    .map(|(sample, baseline)| sample - baseline)
                    .collect();
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let history_label = format!(
            "rolling_mean_baseline(window_samples={})",
            self.window_samples
        );
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "rolling_mean_baseline",
            TransformCategory::Baseline,
            vec![TransformParameterMetadata::integer(
                "window_samples",
                self.window_samples as u64,
                "samples",
            )],
            false,
            true,
            TransformPhaseEffect::Delay,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RollingMedianBaselineTransform {
    pub window_samples: usize,
}

impl Filter for RollingMedianBaselineTransform {
    fn name(&self) -> &'static str {
        "rolling_median_baseline"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_window_samples(self.window_samples)?;
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_finite_samples("rolling_median_baseline", &channel.samples)?;
                let baseline = trailing_moving_median(&channel.samples, self.window_samples)?;
                let samples = channel
                    .samples
                    .iter()
                    .zip(baseline)
                    .map(|(sample, baseline)| sample - baseline)
                    .collect();
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let history_label = format!(
            "rolling_median_baseline(window_samples={})",
            self.window_samples
        );
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "rolling_median_baseline",
            TransformCategory::Baseline,
            vec![TransformParameterMetadata::integer(
                "window_samples",
                self.window_samples as u64,
                "samples",
            )],
            false,
            true,
            TransformPhaseEffect::Nonlinear,
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearDetrendTransform;

impl Filter for LinearDetrendTransform {
    fn name(&self) -> &'static str {
        "linear_detrend"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_time_axis(&waveform.time, "linear_detrend")?;
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_finite_samples("linear_detrend", &channel.samples)?;
                let samples = detrend_samples(&waveform.time, &channel.samples, 1)?;
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            "linear_detrend()",
            "linear_detrend",
            TransformCategory::Baseline,
            Vec::new(),
            offline_execution(true, true, TransformPhaseEffect::None),
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PolynomialDetrendTransform {
    pub polynomial_order: usize,
}

impl Filter for PolynomialDetrendTransform {
    fn name(&self) -> &'static str {
        "polynomial_detrend"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_polynomial_fit_order(self.polynomial_order)?;
        validate_time_axis(&waveform.time, "polynomial_detrend")?;
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_finite_samples("polynomial_detrend", &channel.samples)?;
                let samples =
                    detrend_samples(&waveform.time, &channel.samples, self.polynomial_order)?;
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let history_label = format!(
            "polynomial_detrend(polynomial_order={})",
            self.polynomial_order
        );
        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            history_label,
            "polynomial_detrend",
            TransformCategory::Baseline,
            vec![TransformParameterMetadata::integer(
                "polynomial_order",
                self.polynomial_order as u64,
                "order",
            )],
            offline_execution(true, true, TransformPhaseEffect::None),
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HampelFilter {
    pub window_samples: usize,
    pub outlier_sigma: f64,
}

impl Filter for HampelFilter {
    fn name(&self) -> &'static str {
        "hampel_filter"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_window_samples(self.window_samples)?;
        validate_positive_parameter("outlier_sigma", self.outlier_sigma)?;
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_finite_samples("hampel_filter", &channel.samples)?;
                let samples =
                    hampel_filter(&channel.samples, self.window_samples, self.outlier_sigma);
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let history_label = format!(
            "hampel_filter(window_samples={},outlier_sigma={})",
            self.window_samples, self.outlier_sigma
        );
        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            history_label,
            "hampel_filter",
            TransformCategory::Windowed,
            vec![
                TransformParameterMetadata::integer(
                    "window_samples",
                    self.window_samples as u64,
                    "samples",
                ),
                TransformParameterMetadata::float("outlier_sigma", self.outlier_sigma, "sigma"),
            ],
            offline_execution(false, true, TransformPhaseEffect::Nonlinear),
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpikeRemoveTransform {
    pub window_samples: usize,
    pub threshold_v: f64,
}

impl Filter for SpikeRemoveTransform {
    fn name(&self) -> &'static str {
        "spike_remove"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_window_samples(self.window_samples)?;
        validate_positive_parameter("threshold_v", self.threshold_v)?;
        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_finite_samples("spike_remove", &channel.samples)?;
                let samples = spike_remove(&channel.samples, self.window_samples, self.threshold_v);
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let history_label = format!(
            "spike_remove(window_samples={},threshold_v={})",
            self.window_samples, self.threshold_v
        );
        let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
            history_label,
            "spike_remove",
            TransformCategory::Windowed,
            vec![
                TransformParameterMetadata::integer(
                    "window_samples",
                    self.window_samples as u64,
                    "samples",
                ),
                TransformParameterMetadata::float("threshold_v", self.threshold_v, "V"),
            ],
            offline_execution(false, true, TransformPhaseEffect::Nonlinear),
        );
        derived_waveform(waveform, channels, transform_step)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FirFilter {
    pub coefficients: Vec<f64>,
}

impl Filter for FirFilter {
    fn name(&self) -> &'static str {
        "fir_filter"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_filter_coefficients("coefficients", &self.coefficients)?;
        apply_frequency_filter(
            waveform,
            "fir_filter",
            format!("fir_filter(coefficients={})", self.coefficients.len()),
            vec![TransformParameterMetadata::integer(
                "coefficient_count",
                self.coefficients.len() as u64,
                "coefficients",
            )],
            TransformExecutionMetadata {
                sample_rate_required: false,
                stateful: true,
                causal: true,
                phase_effect: TransformPhaseEffect::Delay,
                streaming_supported: true,
                offline_only: false,
            },
            |samples| Ok(fir_convolution(samples, &self.coefficients)),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ZeroPhaseFirFilter {
    pub coefficients: Vec<f64>,
}

impl Filter for ZeroPhaseFirFilter {
    fn name(&self) -> &'static str {
        "zero_phase_fir_filter"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_filter_coefficients("coefficients", &self.coefficients)?;
        apply_frequency_filter(
            waveform,
            "zero_phase_fir_filter",
            format!(
                "zero_phase_fir_filter(coefficients={})",
                self.coefficients.len()
            ),
            vec![TransformParameterMetadata::integer(
                "coefficient_count",
                self.coefficients.len() as u64,
                "coefficients",
            )],
            offline_execution(false, true, TransformPhaseEffect::None),
            |samples| Ok(zero_phase_fir(samples, &self.coefficients)),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BiquadCoefficients {
    pub b0: f64,
    pub b1: f64,
    pub b2: f64,
    pub a1: f64,
    pub a2: f64,
}

impl BiquadCoefficients {
    pub fn from_slice(coefficients: &[f64]) -> Result<Self> {
        if coefficients.len() != 5 {
            return Err(WaveformError::InvalidParameter {
                name: "coefficients".to_string(),
                reason: "biquad filters require coefficients [b0, b1, b2, a1, a2]".to_string(),
            });
        }
        let coefficients = Self {
            b0: coefficients[0],
            b1: coefficients[1],
            b2: coefficients[2],
            a1: coefficients[3],
            a2: coefficients[4],
        };
        coefficients.validate("coefficients")?;
        Ok(coefficients)
    }

    fn validate(self, name: &str) -> Result<()> {
        for value in [self.b0, self.b1, self.b2, self.a1, self.a2] {
            validate_finite_parameter(name, value)?;
        }
        validate_biquad_stability(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IirBiquadFilter {
    pub coefficients: BiquadCoefficients,
}

impl Filter for IirBiquadFilter {
    fn name(&self) -> &'static str {
        "iir_biquad"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        self.coefficients.validate("coefficients")?;
        apply_biquad_filter(
            waveform,
            "iir_biquad",
            "iir_biquad(coefficients=5)".to_string(),
            self.coefficients,
            vec![TransformParameterMetadata::integer(
                "coefficient_count",
                5,
                "coefficients",
            )],
            TransformExecutionMetadata {
                sample_rate_required: false,
                stateful: true,
                causal: true,
                phase_effect: TransformPhaseEffect::Delay,
                streaming_supported: true,
                offline_only: false,
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZeroPhaseIirBiquadFilter {
    pub coefficients: BiquadCoefficients,
}

impl Filter for ZeroPhaseIirBiquadFilter {
    fn name(&self) -> &'static str {
        "zero_phase_iir_biquad"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        self.coefficients.validate("coefficients")?;
        apply_frequency_filter(
            waveform,
            "zero_phase_iir_biquad",
            "zero_phase_iir_biquad(coefficients=5)".to_string(),
            vec![TransformParameterMetadata::integer(
                "coefficient_count",
                5,
                "coefficients",
            )],
            offline_execution(false, true, TransformPhaseEffect::None),
            |samples| Ok(zero_phase_biquad(samples, self.coefficients)),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HighPassFilter {
    pub cutoff_hz: f64,
}

impl Filter for HighPassFilter {
    fn name(&self) -> &'static str {
        "high_pass"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_designed_filter_frequency(
            &waveform.time,
            "high_pass",
            "cutoff_hz",
            self.cutoff_hz,
        )?;
        apply_frequency_filter(
            waveform,
            "high_pass",
            format!("high_pass(cutoff_hz={})", self.cutoff_hz),
            vec![TransformParameterMetadata::float(
                "cutoff_hz",
                self.cutoff_hz,
                "Hz",
            )],
            TransformExecutionMetadata {
                sample_rate_required: true,
                stateful: true,
                causal: true,
                phase_effect: TransformPhaseEffect::Delay,
                streaming_supported: true,
                offline_only: false,
            },
            |samples| {
                Ok(first_order_high_pass(
                    &waveform.time,
                    samples,
                    self.cutoff_hz,
                ))
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BandPassFilter {
    pub center_hz: f64,
    pub q: f64,
}

impl Filter for BandPassFilter {
    fn name(&self) -> &'static str {
        "band_pass"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let coefficients = rbj_biquad_for_waveform(
            waveform,
            "band_pass",
            BiquadDesignKind::BandPass,
            self.center_hz,
            self.q,
        )?;
        apply_biquad_filter(
            waveform,
            "band_pass",
            format!("band_pass(center_hz={},q={})", self.center_hz, self.q),
            coefficients,
            center_q_parameters(self.center_hz, self.q),
            designed_filter_execution(TransformPhaseEffect::Delay),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BandStopFilter {
    pub center_hz: f64,
    pub q: f64,
}

impl Filter for BandStopFilter {
    fn name(&self) -> &'static str {
        "band_stop"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let coefficients = rbj_biquad_for_waveform(
            waveform,
            "band_stop",
            BiquadDesignKind::BandStop,
            self.center_hz,
            self.q,
        )?;
        apply_biquad_filter(
            waveform,
            "band_stop",
            format!("band_stop(center_hz={},q={})", self.center_hz, self.q),
            coefficients,
            center_q_parameters(self.center_hz, self.q),
            designed_filter_execution(TransformPhaseEffect::Delay),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NotchFilter {
    pub center_hz: f64,
    pub q: f64,
}

impl Filter for NotchFilter {
    fn name(&self) -> &'static str {
        "notch"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let coefficients = rbj_biquad_for_waveform(
            waveform,
            "notch",
            BiquadDesignKind::Notch,
            self.center_hz,
            self.q,
        )?;
        apply_biquad_filter(
            waveform,
            "notch",
            format!("notch(center_hz={},q={})", self.center_hz, self.q),
            coefficients,
            center_q_parameters(self.center_hz, self.q),
            designed_filter_execution(TransformPhaseEffect::Delay),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CombFilter {
    pub delay_samples: usize,
    pub feedback_gain: f64,
}

impl Filter for CombFilter {
    fn name(&self) -> &'static str {
        "comb_filter"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_comb_filter_parameters(self.delay_samples, self.feedback_gain)?;
        apply_frequency_filter(
            waveform,
            "comb_filter",
            format!(
                "comb_filter(delay_samples={},feedback_gain={})",
                self.delay_samples, self.feedback_gain
            ),
            vec![
                TransformParameterMetadata::integer(
                    "delay_samples",
                    self.delay_samples as u64,
                    "samples",
                ),
                TransformParameterMetadata::float("feedback_gain", self.feedback_gain, "ratio"),
            ],
            TransformExecutionMetadata {
                sample_rate_required: false,
                stateful: true,
                causal: true,
                phase_effect: TransformPhaseEffect::Delay,
                streaming_supported: true,
                offline_only: false,
            },
            |samples| {
                Ok(feedforward_comb(
                    samples,
                    self.delay_samples,
                    self.feedback_gain,
                ))
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ButterworthLowPassFilter {
    pub cutoff_hz: f64,
}

impl Filter for ButterworthLowPassFilter {
    fn name(&self) -> &'static str {
        "butterworth_low_pass"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let sample_rate_hz = validate_designed_filter_frequency(
            &waveform.time,
            "butterworth_low_pass",
            "cutoff_hz",
            self.cutoff_hz,
        )?;
        let coefficients = rbj_biquad(
            BiquadDesignKind::LowPass,
            sample_rate_hz,
            self.cutoff_hz,
            std::f64::consts::FRAC_1_SQRT_2,
        )?;
        apply_biquad_filter(
            waveform,
            "butterworth_low_pass",
            format!("butterworth_low_pass(cutoff_hz={})", self.cutoff_hz),
            coefficients,
            cutoff_parameters(self.cutoff_hz),
            designed_filter_execution(TransformPhaseEffect::Delay),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ButterworthHighPassFilter {
    pub cutoff_hz: f64,
}

impl Filter for ButterworthHighPassFilter {
    fn name(&self) -> &'static str {
        "butterworth_high_pass"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let sample_rate_hz = validate_designed_filter_frequency(
            &waveform.time,
            "butterworth_high_pass",
            "cutoff_hz",
            self.cutoff_hz,
        )?;
        let coefficients = rbj_biquad(
            BiquadDesignKind::HighPass,
            sample_rate_hz,
            self.cutoff_hz,
            std::f64::consts::FRAC_1_SQRT_2,
        )?;
        apply_biquad_filter(
            waveform,
            "butterworth_high_pass",
            format!("butterworth_high_pass(cutoff_hz={})", self.cutoff_hz),
            coefficients,
            cutoff_parameters(self.cutoff_hz),
            designed_filter_execution(TransformPhaseEffect::Delay),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Chebyshev1LowPassFilter {
    pub cutoff_hz: f64,
    pub ripple_db: f64,
}

impl Filter for Chebyshev1LowPassFilter {
    fn name(&self) -> &'static str {
        "chebyshev1_low_pass"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let sample_rate_hz = validate_designed_filter_frequency(
            &waveform.time,
            "chebyshev1_low_pass",
            "cutoff_hz",
            self.cutoff_hz,
        )?;
        validate_positive_parameter("ripple_db", self.ripple_db)?;
        let coefficients =
            chebyshev1_low_pass_biquad(sample_rate_hz, self.cutoff_hz, self.ripple_db)?;
        apply_biquad_filter(
            waveform,
            "chebyshev1_low_pass",
            format!(
                "chebyshev1_low_pass(cutoff_hz={},ripple_db={})",
                self.cutoff_hz, self.ripple_db
            ),
            coefficients,
            vec![
                TransformParameterMetadata::float("cutoff_hz", self.cutoff_hz, "Hz"),
                TransformParameterMetadata::float("ripple_db", self.ripple_db, "dB"),
            ],
            designed_filter_execution(TransformPhaseEffect::Delay),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Chebyshev2LowPassFilter {
    pub cutoff_hz: f64,
    pub stopband_attenuation_db: f64,
}

impl Filter for Chebyshev2LowPassFilter {
    fn name(&self) -> &'static str {
        "chebyshev2_low_pass"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let sample_rate_hz = validate_designed_filter_frequency(
            &waveform.time,
            "chebyshev2_low_pass",
            "cutoff_hz",
            self.cutoff_hz,
        )?;
        validate_positive_parameter("stopband_attenuation_db", self.stopband_attenuation_db)?;
        let coefficients = chebyshev2_low_pass_biquad(
            sample_rate_hz,
            self.cutoff_hz,
            self.stopband_attenuation_db,
        )?;
        apply_biquad_filter(
            waveform,
            "chebyshev2_low_pass",
            format!(
                "chebyshev2_low_pass(cutoff_hz={},stopband_attenuation_db={})",
                self.cutoff_hz, self.stopband_attenuation_db
            ),
            coefficients,
            vec![
                TransformParameterMetadata::float("cutoff_hz", self.cutoff_hz, "Hz"),
                TransformParameterMetadata::float(
                    "stopband_attenuation_db",
                    self.stopband_attenuation_db,
                    "dB",
                ),
            ],
            designed_filter_execution(TransformPhaseEffect::Delay),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BesselLowPassFilter {
    pub cutoff_hz: f64,
}

impl Filter for BesselLowPassFilter {
    fn name(&self) -> &'static str {
        "bessel_low_pass"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        let sample_rate_hz = validate_designed_filter_frequency(
            &waveform.time,
            "bessel_low_pass",
            "cutoff_hz",
            self.cutoff_hz,
        )?;
        let coefficients = bessel_low_pass_biquad(sample_rate_hz, self.cutoff_hz)?;
        apply_biquad_filter(
            waveform,
            "bessel_low_pass",
            format!("bessel_low_pass(cutoff_hz={})", self.cutoff_hz),
            coefficients,
            cutoff_parameters(self.cutoff_hz),
            designed_filter_execution(TransformPhaseEffect::Delay),
        )
    }
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
pub struct HighPassBaselineFilter {
    pub cutoff_hz: f64,
}

impl Filter for HighPassBaselineFilter {
    fn name(&self) -> &'static str {
        "high_pass_baseline"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        validate_positive_frequency("cutoff_hz", self.cutoff_hz)?;
        validate_time_axis(&waveform.time, "high-pass baseline correction")?;

        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                validate_finite_samples("high_pass_baseline", &channel.samples)?;
                let samples =
                    first_order_high_pass(&waveform.time, &channel.samples, self.cutoff_hz);
                Ok(Channel::new(
                    channel.name.clone(),
                    channel.unit.clone(),
                    samples,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let history_label = format!("high_pass_baseline(cutoff_hz={})", self.cutoff_hz);
        let transform_step = TransformStepMetadata::implemented_desktop(
            history_label,
            "high_pass_baseline",
            TransformCategory::Stateful,
            vec![TransformParameterMetadata::float(
                "cutoff_hz",
                self.cutoff_hz,
                "Hz",
            )],
            true,
            true,
            TransformPhaseEffect::Delay,
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
        validate_time_axis(&waveform.time, "low-pass filtering")?;

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

fn map_samples_checked(
    waveform: &Waveform,
    transform_name: &str,
    transform: impl Fn(f64) -> Result<f64> + Copy,
) -> Result<Vec<Channel>> {
    waveform
        .channels
        .iter()
        .map(|channel| {
            validate_finite_samples(transform_name, &channel.samples)?;
            let samples = channel
                .samples
                .iter()
                .copied()
                .map(transform)
                .collect::<Result<Vec<_>>>()?;
            validate_finite_samples(&format!("{transform_name} output"), &samples)?;
            Ok(Channel::new(
                channel.name.clone(),
                channel.unit.clone(),
                samples,
            ))
        })
        .collect()
}

fn apply_m31_transform(
    waveform: &Waveform,
    history_label: impl Into<String>,
    transform_name: &'static str,
    category: TransformCategory,
    parameters: Vec<TransformParameterMetadata>,
    execution: TransformExecutionMetadata,
    transform: impl Fn(&[f64], &[f64]) -> Result<Vec<f64>> + Copy,
) -> Result<Waveform> {
    if execution.sample_rate_required {
        validate_time_axis(&waveform.time, transform_name)?;
    } else {
        validate_finite_time_axis(&waveform.time, transform_name)?;
    }

    let channels = waveform
        .channels
        .iter()
        .map(|channel| {
            validate_finite_samples(transform_name, &channel.samples)?;
            let samples = transform(&waveform.time, &channel.samples)?;
            validate_finite_samples(&format!("{transform_name} output"), &samples)?;
            Ok(Channel::new(
                channel.name.clone(),
                channel.unit.clone(),
                samples,
            ))
        })
        .collect::<Result<Vec<_>>>()?;

    let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
        history_label,
        transform_name,
        category,
        parameters,
        execution,
    );
    derived_waveform(waveform, channels, transform_step)
}

fn validate_unit_interval(name: &str, value: f64) -> Result<()> {
    validate_finite_parameter(name, value)?;
    if !(0.0..=1.0).contains(&value) {
        return Err(WaveformError::InvalidParameter {
            name: name.to_string(),
            reason: "must be between zero and one".to_string(),
        });
    }
    Ok(())
}

fn validate_positive_finite_parameter(name: &str, value: f64) -> Result<()> {
    validate_positive_parameter(name, value)
}

fn envelope_samples(samples: &[f64], alpha: f64) -> Vec<f64> {
    if samples.is_empty() {
        return Vec::new();
    }

    let mut envelope = Vec::with_capacity(samples.len());
    envelope.push(samples[0].abs());
    for sample in &samples[1..] {
        let previous = *envelope
            .last()
            .expect("envelope always has one seeded output sample");
        envelope.push(alpha * sample.abs() + (1.0 - alpha) * previous);
    }
    envelope
}

fn moving_rms_samples(samples: &[f64], window_samples: usize) -> Vec<f64> {
    let mut output = Vec::with_capacity(samples.len());
    for index in 0..samples.len() {
        let start = (index + 1).saturating_sub(window_samples);
        let window = &samples[start..=index];
        let mean_square =
            window.iter().map(|sample| sample * sample).sum::<f64>() / window.len() as f64;
        output.push(mean_square.sqrt());
    }
    output
}

fn peak_hold_samples(samples: &[f64]) -> Vec<f64> {
    let mut output = Vec::with_capacity(samples.len());
    let mut peak = 0.0_f64;
    for sample in samples {
        peak = peak.max(sample.abs());
        output.push(peak);
    }
    output
}

fn derivative_samples(time: &[f64], samples: &[f64]) -> Result<Vec<f64>> {
    validate_time_axis(time, "derivative")?;
    if samples.len() != time.len() {
        return Err(WaveformError::MismatchedSampleCount {
            expected: time.len(),
            actual: samples.len(),
        });
    }

    let mut output = Vec::with_capacity(samples.len());
    output.push(0.0);
    for index in 1..samples.len() {
        let dt = time[index] - time[index - 1];
        output.push((samples[index] - samples[index - 1]) / dt);
    }
    Ok(output)
}

fn cumulative_integral_samples(time: &[f64], samples: &[f64]) -> Result<Vec<f64>> {
    validate_time_axis(time, "cumulative_integral")?;
    if samples.len() != time.len() {
        return Err(WaveformError::MismatchedSampleCount {
            expected: time.len(),
            actual: samples.len(),
        });
    }

    let mut output = Vec::with_capacity(samples.len());
    let mut accumulator = 0.0;
    output.push(accumulator);
    for index in 1..samples.len() {
        let dt = time[index] - time[index - 1];
        accumulator += 0.5 * (samples[index - 1] + samples[index]) * dt;
        output.push(accumulator);
    }
    Ok(output)
}

fn cumulative_integral_waveform(
    waveform: &Waveform,
    history_label: impl Into<String>,
    transform_name: &'static str,
) -> Result<Waveform> {
    apply_m31_transform(
        waveform,
        history_label,
        transform_name,
        TransformCategory::Feature,
        Vec::new(),
        TransformExecutionMetadata {
            sample_rate_required: true,
            stateful: true,
            causal: true,
            phase_effect: TransformPhaseEffect::Delay,
            streaming_supported: true,
            offline_only: false,
        },
        cumulative_integral_samples,
    )
}

fn leaky_integrator_samples(
    time: &[f64],
    samples: &[f64],
    time_constant_s: f64,
) -> Result<Vec<f64>> {
    validate_time_axis(time, "leaky_integrator")?;
    if samples.len() != time.len() {
        return Err(WaveformError::MismatchedSampleCount {
            expected: time.len(),
            actual: samples.len(),
        });
    }

    let mut output = Vec::with_capacity(samples.len());
    let mut state = 0.0;
    output.push(state);
    for index in 1..samples.len() {
        let dt = time[index] - time[index - 1];
        let decay = (-dt / time_constant_s).exp();
        state = state * decay + samples[index] * dt;
        output.push(state);
    }
    Ok(output)
}

fn slope_detection_samples(
    time: &[f64],
    samples: &[f64],
    threshold_per_s: f64,
) -> Result<Vec<f64>> {
    derivative_samples(time, samples).map(|derivative| {
        derivative
            .into_iter()
            .map(|slope| {
                if slope > threshold_per_s {
                    1.0
                } else if slope < -threshold_per_s {
                    -1.0
                } else {
                    0.0
                }
            })
            .collect()
    })
}

fn apply_m32_rolling_transform(
    waveform: &Waveform,
    transform_name: &'static str,
    window_samples: usize,
    transform: impl Fn(&[f64], usize) -> Vec<f64> + Copy,
) -> Result<Waveform> {
    validate_window_samples(window_samples)?;
    apply_m31_transform(
        waveform,
        format!("{transform_name}(window_samples={window_samples})"),
        transform_name,
        TransformCategory::Windowed,
        vec![TransformParameterMetadata::integer(
            "window_samples",
            window_samples as u64,
            "samples",
        )],
        TransformExecutionMetadata {
            sample_rate_required: false,
            stateful: true,
            causal: true,
            phase_effect: TransformPhaseEffect::Delay,
            streaming_supported: true,
            offline_only: false,
        },
        |_, samples| Ok(transform(samples, window_samples)),
    )
}

fn apply_m32_distribution_transform(
    waveform: &Waveform,
    transform_name: &'static str,
    parameters: Vec<TransformParameterMetadata>,
    transform: impl Fn(&[f64]) -> Result<Vec<f64>> + Copy,
) -> Result<Waveform> {
    apply_m31_transform(
        waveform,
        distribution_history_label(transform_name, &parameters),
        transform_name,
        TransformCategory::Feature,
        parameters,
        TransformExecutionMetadata {
            sample_rate_required: false,
            stateful: false,
            causal: false,
            phase_effect: TransformPhaseEffect::Nonlinear,
            streaming_supported: false,
            offline_only: true,
        },
        |_, samples| transform(samples),
    )
}

fn distribution_history_label(
    transform_name: &'static str,
    parameters: &[TransformParameterMetadata],
) -> String {
    if parameters.is_empty() {
        return format!("{transform_name}()");
    }
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

fn rolling_mean_samples(samples: &[f64], window_samples: usize) -> Vec<f64> {
    rolling_statistic(samples, window_samples, |window| {
        window.iter().sum::<f64>() / window.len() as f64
    })
}

fn rolling_variance_samples(samples: &[f64], window_samples: usize) -> Vec<f64> {
    rolling_statistic(samples, window_samples, population_variance)
}

fn rolling_stddev_samples(samples: &[f64], window_samples: usize) -> Vec<f64> {
    rolling_statistic(samples, window_samples, |window| {
        population_variance(window).sqrt()
    })
}

fn rolling_min_samples(samples: &[f64], window_samples: usize) -> Vec<f64> {
    rolling_statistic(samples, window_samples, |window| min_max(window).0)
}

fn rolling_max_samples(samples: &[f64], window_samples: usize) -> Vec<f64> {
    rolling_statistic(samples, window_samples, |window| min_max(window).1)
}

fn rolling_statistic(
    samples: &[f64],
    window_samples: usize,
    statistic: impl Fn(&[f64]) -> f64,
) -> Vec<f64> {
    let mut output = Vec::with_capacity(samples.len());
    for index in 0..samples.len() {
        let start = (index + 1).saturating_sub(window_samples);
        output.push(statistic(&samples[start..=index]));
    }
    output
}

fn z_score_samples(samples: &[f64]) -> Result<Vec<f64>> {
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    let std_dev = population_variance(samples).sqrt();
    if std_dev <= f64::EPSILON {
        return Err(WaveformError::InvalidWaveform {
            reason: "z_score requires non-constant samples".to_string(),
        });
    }
    Ok(samples
        .iter()
        .map(|sample| (sample - mean) / std_dev)
        .collect())
}

fn outlier_detection_samples(samples: &[f64], threshold_sigma: f64) -> Result<Vec<f64>> {
    z_score_samples(samples).map(|z_scores| {
        z_scores
            .into_iter()
            .map(|z_score| {
                if z_score.abs() > threshold_sigma {
                    1.0
                } else {
                    0.0
                }
            })
            .collect()
    })
}

fn quantile_clip_samples(
    samples: &[f64],
    lower_quantile: f64,
    upper_quantile: f64,
) -> Result<Vec<f64>> {
    let lower = quantile_value(samples, lower_quantile)?;
    let upper = quantile_value(samples, upper_quantile)?;
    Ok(samples
        .iter()
        .map(|sample| sample.clamp(lower, upper))
        .collect())
}

fn population_variance(samples: &[f64]) -> f64 {
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    samples
        .iter()
        .map(|sample| {
            let delta = sample - mean;
            delta * delta
        })
        .sum::<f64>()
        / samples.len() as f64
}

fn quantile_value(samples: &[f64], quantile: f64) -> Result<f64> {
    validate_quantile(quantile, "quantile")?;
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

fn validate_quantile_pair(lower_quantile: f64, upper_quantile: f64) -> Result<()> {
    validate_quantile(lower_quantile, "lower_quantile")?;
    validate_quantile(upper_quantile, "upper_quantile")?;
    if upper_quantile <= lower_quantile {
        return Err(WaveformError::InvalidParameter {
            name: "upper_quantile".to_string(),
            reason: "must be greater than lower_quantile".to_string(),
        });
    }
    Ok(())
}

fn validate_quantile(quantile: f64, name: &str) -> Result<()> {
    validate_finite_parameter(name, quantile)?;
    if !(0.0..=1.0).contains(&quantile) {
        return Err(WaveformError::InvalidParameter {
            name: name.to_string(),
            reason: "must be between zero and one".to_string(),
        });
    }
    Ok(())
}

fn apply_frequency_filter(
    waveform: &Waveform,
    transform_name: &'static str,
    history_label: String,
    parameters: Vec<TransformParameterMetadata>,
    execution: TransformExecutionMetadata,
    mut filter: impl FnMut(&[f64]) -> Result<Vec<f64>>,
) -> Result<Waveform> {
    validate_time_axis(&waveform.time, transform_name)?;
    let channels = waveform
        .channels
        .iter()
        .map(|channel| {
            validate_finite_samples(transform_name, &channel.samples)?;
            let samples = filter(&channel.samples)?;
            validate_finite_samples(&format!("{transform_name} output"), &samples)?;
            Ok(Channel::new(
                channel.name.clone(),
                channel.unit.clone(),
                samples,
            ))
        })
        .collect::<Result<Vec<_>>>()?;

    let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
        history_label,
        transform_name,
        TransformCategory::FrequencyFilter,
        parameters,
        execution,
    );
    derived_waveform(waveform, channels, transform_step)
}

fn apply_biquad_filter(
    waveform: &Waveform,
    transform_name: &'static str,
    history_label: String,
    coefficients: BiquadCoefficients,
    parameters: Vec<TransformParameterMetadata>,
    execution: TransformExecutionMetadata,
) -> Result<Waveform> {
    coefficients.validate("coefficients")?;
    apply_frequency_filter(
        waveform,
        transform_name,
        history_label,
        parameters,
        execution,
        |samples| Ok(apply_biquad(samples, coefficients)),
    )
}

fn designed_filter_execution(phase_effect: TransformPhaseEffect) -> TransformExecutionMetadata {
    TransformExecutionMetadata {
        sample_rate_required: true,
        stateful: true,
        causal: true,
        phase_effect,
        streaming_supported: true,
        offline_only: false,
    }
}

fn cutoff_parameters(cutoff_hz: f64) -> Vec<TransformParameterMetadata> {
    vec![TransformParameterMetadata::float(
        "cutoff_hz",
        cutoff_hz,
        "Hz",
    )]
}

fn center_q_parameters(center_hz: f64, q: f64) -> Vec<TransformParameterMetadata> {
    vec![
        TransformParameterMetadata::float("center_hz", center_hz, "Hz"),
        TransformParameterMetadata::float("q", q, "ratio"),
    ]
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

fn derived_waveform_with_time(
    source: &Waveform,
    time: Vec<f64>,
    channels: Vec<Channel>,
    transform_step: TransformStepMetadata,
) -> Result<Waveform> {
    Ok(
        Waveform::new_with_time_unit(time, source.time_unit.clone(), channels)?
            .as_derived_from_with_transform_step(source, transform_step),
    )
}

fn offline_execution(
    sample_rate_required: bool,
    stateful: bool,
    phase_effect: TransformPhaseEffect,
) -> TransformExecutionMetadata {
    TransformExecutionMetadata {
        sample_rate_required,
        stateful,
        causal: false,
        phase_effect,
        streaming_supported: false,
        offline_only: true,
    }
}

fn simulation_transform_step(
    transform_name: &'static str,
    history_label: String,
    category: TransformCategory,
    parameters: Vec<TransformParameterMetadata>,
    sample_rate_required: bool,
    stateful: bool,
    phase_effect: TransformPhaseEffect,
) -> TransformStepMetadata {
    TransformStepMetadata::implemented_desktop_with_execution(
        history_label,
        transform_name,
        category,
        parameters,
        offline_execution(sample_rate_required, stateful, phase_effect),
    )
}

fn m35_streaming_execution(
    sample_rate_required: bool,
    stateful: bool,
    phase_effect: TransformPhaseEffect,
) -> TransformExecutionMetadata {
    TransformExecutionMetadata {
        sample_rate_required,
        stateful,
        causal: true,
        phase_effect,
        streaming_supported: true,
        offline_only: false,
    }
}

fn m35_transform_step(
    transform_name: &'static str,
    history_label: String,
    category: TransformCategory,
    parameters: Vec<TransformParameterMetadata>,
    execution: TransformExecutionMetadata,
) -> TransformStepMetadata {
    let mut step = TransformStepMetadata::implemented_desktop_with_execution(
        history_label,
        transform_name,
        category,
        parameters,
        execution,
    );
    step.output_channels = TransformOutputChannels::derived_channels_with_new_names();
    step
}

fn append_derived_channels(
    source: &Waveform,
    new_channels: Vec<Channel>,
    transform_step: TransformStepMetadata,
) -> Result<Waveform> {
    let mut channels = source.channels.clone();
    channels.extend(new_channels);
    derived_waveform(source, channels, transform_step)
}

fn required_waveform_channel<'a>(
    waveform: &'a Waveform,
    channel_name: &str,
    transform_name: &str,
) -> Result<&'a Channel> {
    waveform
        .channel(channel_name)
        .ok_or_else(|| WaveformError::InvalidParameter {
            name: "channel".to_string(),
            reason: format!("{transform_name} requires channel `{channel_name}`"),
        })
}

fn required_waveform_channels<'a>(
    waveform: &'a Waveform,
    channel_names: &[String],
    transform_name: &str,
    minimum_count: usize,
) -> Result<Vec<&'a Channel>> {
    if channel_names.len() < minimum_count {
        return Err(WaveformError::InvalidParameter {
            name: "channels".to_string(),
            reason: format!("must include at least {minimum_count} channel name(s)"),
        });
    }
    let mut channels = Vec::with_capacity(channel_names.len());
    for channel_name in channel_names {
        if channel_name.trim().is_empty() {
            return Err(WaveformError::InvalidParameter {
                name: "channels".to_string(),
                reason: "channel names must not be empty".to_string(),
            });
        }
        let channel = required_waveform_channel(waveform, channel_name, transform_name)?;
        validate_finite_samples(transform_name, &channel.samples)?;
        channels.push(channel);
    }
    Ok(channels)
}

fn validate_output_channel_name(name: &str) -> Result<()> {
    if name.trim().is_empty() {
        return Err(WaveformError::InvalidParameter {
            name: "output_channel".to_string(),
            reason: "must not be empty".to_string(),
        });
    }
    Ok(())
}

fn validate_same_unit(transform_name: &str, left: &Channel, right: &Channel) -> Result<()> {
    if left.unit != right.unit {
        return Err(WaveformError::InvalidParameter {
            name: "channels".to_string(),
            reason: format!("{transform_name} requires matching channel units"),
        });
    }
    Ok(())
}

fn validate_matching_units(transform_name: &str, channels: &[&Channel]) -> Result<()> {
    if let Some(first) = channels.first() {
        for channel in channels.iter().skip(1) {
            validate_same_unit(transform_name, first, channel)?;
        }
    }
    Ok(())
}

fn configured_or_source_unit(output_unit: Option<&str>, source_unit: &Unit) -> Unit {
    output_unit
        .filter(|unit| !unit.trim().is_empty())
        .map(Unit::new)
        .unwrap_or_else(|| source_unit.clone())
}

fn channel_pair_parameters(
    left_channel: &str,
    right_channel: &str,
    output_channel: &str,
    output_unit: &str,
) -> Vec<TransformParameterMetadata> {
    vec![
        TransformParameterMetadata::text("left_channel", left_channel),
        TransformParameterMetadata::text("right_channel", right_channel),
        TransformParameterMetadata::text("output_channel", output_channel),
        TransformParameterMetadata::text("output_unit", output_unit),
    ]
}

fn validate_matrix_transform_shape(
    matrix: &[Vec<f64>],
    input_channel_count: usize,
    output_channels: &[String],
) -> Result<()> {
    if matrix.is_empty() {
        return Err(WaveformError::InvalidParameter {
            name: "matrix".to_string(),
            reason: "must include at least one row".to_string(),
        });
    }
    if matrix.len() != output_channels.len() {
        return Err(WaveformError::InvalidParameter {
            name: "output_channels".to_string(),
            reason: "must include exactly one output channel name per matrix row".to_string(),
        });
    }
    for (row_index, row) in matrix.iter().enumerate() {
        if row.len() != input_channel_count {
            return Err(WaveformError::InvalidParameter {
                name: "matrix".to_string(),
                reason: format!(
                    "row {row_index} must contain exactly {input_channel_count} coefficient(s)"
                ),
            });
        }
        if row.iter().any(|coefficient| !coefficient.is_finite()) {
            return Err(WaveformError::InvalidParameter {
                name: "matrix".to_string(),
                reason: "all coefficients must be finite".to_string(),
            });
        }
    }
    Ok(())
}

fn invalid_m35_parameter<T>(name: &str, reason: &str) -> Result<T> {
    Err(WaveformError::InvalidParameter {
        name: name.to_string(),
        reason: reason.to_string(),
    })
}

fn pid_samples(
    time: &[f64],
    samples: &[f64],
    setpoint: f64,
    kp: f64,
    ki: f64,
    kd: f64,
) -> Vec<f64> {
    let mut output = Vec::with_capacity(samples.len());
    let mut integral = 0.0;
    let mut previous_error = setpoint - samples[0];
    output.push(kp * previous_error);
    for index in 1..samples.len() {
        let dt = time[index] - time[index - 1];
        let error = setpoint - samples[index];
        integral += 0.5 * (previous_error + error) * dt;
        let derivative = (error - previous_error) / dt;
        output.push(kp * error + ki * integral + kd * derivative);
        previous_error = error;
    }
    output
}

fn rate_limited_samples(time: &[f64], samples: &[f64], rate_limit_per_s: f64) -> Vec<f64> {
    let mut output = Vec::with_capacity(samples.len());
    output.push(samples[0]);
    for index in 1..samples.len() {
        let dt = time[index] - time[index - 1];
        let max_delta = rate_limit_per_s * dt;
        let previous = output[index - 1];
        let delta = (samples[index] - previous).clamp(-max_delta, max_delta);
        output.push(previous + delta);
    }
    output
}

#[derive(Debug, Clone, Copy)]
struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        Self {
            state: seed ^ 0x9e37_79b9_7f4a_7c15,
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut value = self.state;
        value ^= value >> 12;
        value ^= value << 25;
        value ^= value >> 27;
        self.state = value;
        value.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    fn unit(&mut self) -> f64 {
        let value = self.next_u64() >> 11;
        (value as f64) / ((1_u64 << 53) as f64)
    }

    fn signed_unit(&mut self) -> f64 {
        self.unit() * 2.0 - 1.0
    }

    fn gaussian(&mut self) -> f64 {
        let u1 = self.unit().max(f64::MIN_POSITIVE);
        let u2 = self.unit();
        (-2.0 * u1.ln()).sqrt() * (TAU * u2).cos()
    }
}

fn validate_probability(probability: f64) -> Result<()> {
    validate_finite_parameter("probability", probability)?;
    if !(0.0..=1.0).contains(&probability) {
        return Err(WaveformError::InvalidParameter {
            name: "probability".to_string(),
            reason: "must be between zero and one".to_string(),
        });
    }
    Ok(())
}

fn validate_start_index(start_index: usize, sample_count: usize) -> Result<()> {
    if start_index >= sample_count {
        return Err(WaveformError::InvalidParameter {
            name: "start_index".to_string(),
            reason: "must be less than the waveform sample count".to_string(),
        });
    }
    Ok(())
}

fn validate_index_window(
    start_index: usize,
    duration_samples: usize,
    sample_count: usize,
) -> Result<()> {
    validate_start_index(start_index, sample_count)?;
    if duration_samples == 0 {
        return Err(WaveformError::InvalidParameter {
            name: "duration_samples".to_string(),
            reason: "must be greater than zero".to_string(),
        });
    }
    if start_index + duration_samples > sample_count {
        return Err(WaveformError::InvalidParameter {
            name: "duration_samples".to_string(),
            reason: "must fit within the waveform sample count".to_string(),
        });
    }
    Ok(())
}

fn map_finite_samples(
    waveform: &Waveform,
    transform_name: &str,
    transform: impl Fn(f64) -> f64 + Copy,
) -> Result<Vec<Channel>> {
    waveform
        .channels
        .iter()
        .map(|channel| {
            validate_finite_samples(transform_name, &channel.samples)?;
            let samples = channel
                .samples
                .iter()
                .copied()
                .map(transform)
                .collect::<Vec<_>>();
            validate_finite_samples(transform_name, &samples)?;
            Ok(Channel::new(
                channel.name.clone(),
                channel.unit.clone(),
                samples,
            ))
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum QuantizerRounding {
    Round,
    Floor,
    Ceil,
}

fn quantize_by_lsb(sample: f64, lsb_v: f64, rounding: QuantizerRounding) -> f64 {
    let scaled = sample / lsb_v;
    let code = match rounding {
        QuantizerRounding::Round => scaled.round(),
        QuantizerRounding::Floor => scaled.floor(),
        QuantizerRounding::Ceil => scaled.ceil(),
    };
    code * lsb_v
}

fn compand_sample(sample: f64, kind: CompandingKind, max_v: f64, mu: f64) -> f64 {
    let normalized = (sample / max_v).clamp(-1.0, 1.0);
    let sign = normalized.signum();
    let magnitude = normalized.abs();
    let compressed = match kind {
        CompandingKind::MuLaw => ((1.0 + mu * magnitude).ln() / (1.0 + mu).ln()) * sign,
        CompandingKind::ALaw => {
            if magnitude < 1.0 / mu {
                sign * (mu * magnitude) / (1.0 + mu.ln())
            } else {
                sign * (1.0 + (mu * magnitude).ln()) / (1.0 + mu.ln())
            }
        }
    };
    compressed * max_v
}

fn validate_adc_range(bits: u8, min_v: f64, max_v: f64) -> Result<()> {
    if bits == 0 || bits > MAX_ADC_BITS {
        return Err(WaveformError::InvalidParameter {
            name: "bits".to_string(),
            reason: format!("must be between 1 and {MAX_ADC_BITS}"),
        });
    }
    validate_finite_parameter("min_v", min_v)?;
    validate_finite_parameter("max_v", max_v)?;
    if max_v <= min_v {
        return Err(WaveformError::InvalidParameter {
            name: "max_v".to_string(),
            reason: "must be greater than min_v".to_string(),
        });
    }
    Ok(())
}

fn max_adc_code(bits: u8) -> u64 {
    (1_u64 << bits) - 1
}

fn adc_code_for_sample(sample: f64, bits: u8, min_v: f64, max_v: f64) -> u64 {
    let max_code = max_adc_code(bits);
    let normalized = ((sample - min_v) / (max_v - min_v)).clamp(0.0, 1.0);
    (normalized * max_code as f64).round() as u64
}

fn adc_value_for_code(code: u64, max_code: u64, min_v: f64, max_v: f64) -> f64 {
    min_v + (code as f64 / max_code as f64) * (max_v - min_v)
}

fn adc_range_parameters(bits: u8, min_v: f64, max_v: f64) -> Vec<TransformParameterMetadata> {
    vec![
        TransformParameterMetadata::integer("bits", bits as u64, "bits"),
        TransformParameterMetadata::float("min_v", min_v, "V"),
        TransformParameterMetadata::float("max_v", max_v, "V"),
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimeInterpolationKind {
    Linear,
    Previous,
}

fn resample_waveform(
    waveform: &Waveform,
    sample_interval_s: f64,
    history_label: String,
    transform_name: &'static str,
    category: TransformCategory,
) -> Result<Waveform> {
    resample_waveform_with_interpolation(
        waveform,
        sample_interval_s,
        history_label,
        transform_name,
        vec![TransformParameterMetadata::float(
            "sample_interval_s",
            sample_interval_s,
            "s",
        )],
        offline_execution(true, false, TransformPhaseEffect::Delay),
        TimeInterpolationKind::Linear,
    )
    .map(|mut waveform| {
        if let Some(step) = waveform.metadata.transform_steps.last_mut() {
            step.category = category;
        }
        waveform
    })
}

fn resample_waveform_with_interpolation(
    waveform: &Waveform,
    sample_interval_s: f64,
    history_label: String,
    transform_name: &'static str,
    parameters: Vec<TransformParameterMetadata>,
    execution: TransformExecutionMetadata,
    interpolation: TimeInterpolationKind,
) -> Result<Waveform> {
    validate_sample_interval(sample_interval_s)?;
    validate_time_axis(&waveform.time, transform_name)?;

    let time = fixed_time_grid(&waveform.time, sample_interval_s)?;
    let channels = interpolate_channels_to_grid(waveform, &time, transform_name, interpolation)?;

    let transform_step = TransformStepMetadata::implemented_desktop_with_execution(
        history_label,
        transform_name,
        TransformCategory::Resampling,
        parameters,
        execution,
    );
    derived_waveform_with_time(waveform, time, channels, transform_step)
}

fn interpolate_channels_to_grid(
    waveform: &Waveform,
    target_time: &[f64],
    transform_name: &str,
    interpolation: TimeInterpolationKind,
) -> Result<Vec<Channel>> {
    waveform
        .channels
        .iter()
        .map(|channel| {
            validate_finite_samples(transform_name, &channel.samples)?;
            let samples = target_time
                .iter()
                .map(|sample_time| match interpolation {
                    TimeInterpolationKind::Linear => {
                        interpolate_series(&waveform.time, &channel.samples, *sample_time)
                    }
                    TimeInterpolationKind::Previous => {
                        previous_hold_series(&waveform.time, &channel.samples, *sample_time)
                    }
                })
                .collect();
            Ok(Channel::new(
                channel.name.clone(),
                channel.unit.clone(),
                samples,
            ))
        })
        .collect()
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

fn validate_log_exp_base(base: f64) -> Result<()> {
    validate_positive_parameter("base", base)?;
    if (base - 1.0).abs() < f64::EPSILON {
        return Err(WaveformError::InvalidParameter {
            name: "base".to_string(),
            reason: "must not equal one".to_string(),
        });
    }
    Ok(())
}

fn validate_sample_interval(sample_interval_s: f64) -> Result<()> {
    validate_finite_parameter("sample_interval_s", sample_interval_s)?;
    if sample_interval_s <= 0.0 {
        return Err(WaveformError::InvalidParameter {
            name: "sample_interval_s".to_string(),
            reason: "must be greater than zero".to_string(),
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

fn validate_resampling_factor(name: &str, value: usize) -> Result<()> {
    if value <= 1 {
        return Err(WaveformError::InvalidParameter {
            name: name.to_string(),
            reason: "must be greater than one".to_string(),
        });
    }
    Ok(())
}

fn validate_required_channel_name(name: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(WaveformError::InvalidParameter {
            name: name.to_string(),
            reason: "must not be empty".to_string(),
        });
    }
    Ok(())
}

fn validate_positive_frequency(name: &str, value: f64) -> Result<()> {
    validate_finite_parameter(name, value)?;
    if value <= 0.0 {
        return Err(WaveformError::InvalidParameter {
            name: name.to_string(),
            reason: "must be greater than zero".to_string(),
        });
    }
    Ok(())
}

fn validate_filter_coefficients(name: &str, coefficients: &[f64]) -> Result<()> {
    if coefficients.is_empty() {
        return Err(WaveformError::InvalidParameter {
            name: name.to_string(),
            reason: "must include at least one coefficient".to_string(),
        });
    }
    for coefficient in coefficients {
        validate_finite_parameter(name, *coefficient)?;
    }
    Ok(())
}

fn validate_q(q: f64) -> Result<()> {
    validate_positive_parameter("q", q)?;
    if q > 1_000.0 {
        return Err(WaveformError::InvalidParameter {
            name: "q".to_string(),
            reason: "must be no greater than 1000 for dependency-free biquad design".to_string(),
        });
    }
    Ok(())
}

fn validate_comb_filter_parameters(delay_samples: usize, feedback_gain: f64) -> Result<()> {
    if delay_samples == 0 {
        return Err(WaveformError::InvalidParameter {
            name: "delay_samples".to_string(),
            reason: "must be greater than zero".to_string(),
        });
    }
    validate_finite_parameter("feedback_gain", feedback_gain)?;
    if feedback_gain.abs() > 1.0 {
        return Err(WaveformError::InvalidParameter {
            name: "feedback_gain".to_string(),
            reason: "feed-forward comb gain magnitude must be less than or equal to one"
                .to_string(),
        });
    }
    Ok(())
}

fn uniform_sample_interval(time: &[f64], transform_name: &str) -> Result<f64> {
    validate_time_axis(time, transform_name)?;
    if time.len() < 2 {
        return Err(WaveformError::InvalidWaveform {
            reason: format!("{transform_name} requires at least two timestamps"),
        });
    }
    let interval = time[1] - time[0];
    validate_positive_parameter("sample_interval_s", interval)?;
    let tolerance = (interval.abs() * 1.0e-9).max(1.0e-12);
    for pair in time.windows(2) {
        let candidate = pair[1] - pair[0];
        if (candidate - interval).abs() > tolerance {
            return Err(WaveformError::InvalidWaveform {
                reason: format!("{transform_name} requires a uniform sample interval"),
            });
        }
    }
    Ok(interval)
}

fn uniform_sample_rate(time: &[f64], transform_name: &str) -> Result<f64> {
    Ok(1.0 / uniform_sample_interval(time, transform_name)?)
}

fn validate_designed_filter_frequency(
    time: &[f64],
    transform_name: &str,
    parameter_name: &str,
    frequency_hz: f64,
) -> Result<f64> {
    validate_positive_frequency(parameter_name, frequency_hz)?;
    let sample_rate_hz = uniform_sample_rate(time, transform_name)?;
    let nyquist_hz = sample_rate_hz / 2.0;
    if frequency_hz >= nyquist_hz {
        return Err(WaveformError::InvalidParameter {
            name: parameter_name.to_string(),
            reason: format!("must be below Nyquist frequency {nyquist_hz} Hz"),
        });
    }
    Ok(sample_rate_hz)
}

fn fir_convolution(samples: &[f64], coefficients: &[f64]) -> Vec<f64> {
    let mut filtered = Vec::with_capacity(samples.len());
    for index in 0..samples.len() {
        let mut accumulator = 0.0;
        for (tap, coefficient) in coefficients.iter().enumerate() {
            if index >= tap {
                accumulator += coefficient * samples[index - tap];
            }
        }
        filtered.push(accumulator);
    }
    filtered
}

fn zero_phase_fir(samples: &[f64], coefficients: &[f64]) -> Vec<f64> {
    let forward = fir_convolution(samples, coefficients);
    let mut reversed = forward;
    reversed.reverse();
    let mut backward = fir_convolution(&reversed, coefficients);
    backward.reverse();
    backward
}

fn apply_biquad(samples: &[f64], coefficients: BiquadCoefficients) -> Vec<f64> {
    let mut filtered = Vec::with_capacity(samples.len());
    let mut x1 = 0.0;
    let mut x2 = 0.0;
    let mut y1 = 0.0;
    let mut y2 = 0.0;

    for sample in samples {
        let output = coefficients.b0 * sample + coefficients.b1 * x1 + coefficients.b2 * x2
            - coefficients.a1 * y1
            - coefficients.a2 * y2;
        filtered.push(output);
        x2 = x1;
        x1 = *sample;
        y2 = y1;
        y1 = output;
    }
    filtered
}

fn zero_phase_biquad(samples: &[f64], coefficients: BiquadCoefficients) -> Vec<f64> {
    let forward = apply_biquad(samples, coefficients);
    let mut reversed = forward;
    reversed.reverse();
    let mut backward = apply_biquad(&reversed, coefficients);
    backward.reverse();
    backward
}

fn feedforward_comb(samples: &[f64], delay_samples: usize, feedback_gain: f64) -> Vec<f64> {
    let mut filtered = Vec::with_capacity(samples.len());
    for (index, sample) in samples.iter().enumerate() {
        let delayed = if index >= delay_samples {
            samples[index - delay_samples]
        } else {
            0.0
        };
        filtered.push(sample + feedback_gain * delayed);
    }
    filtered
}

fn validate_biquad_stability(coefficients: BiquadCoefficients) -> Result<()> {
    let discriminant = coefficients.a1 * coefficients.a1 - 4.0 * coefficients.a2;
    let stable = if discriminant >= 0.0 {
        let root = discriminant.sqrt();
        let pole_one = (-coefficients.a1 + root) / 2.0;
        let pole_two = (-coefficients.a1 - root) / 2.0;
        pole_one.abs() < 1.0 && pole_two.abs() < 1.0
    } else {
        coefficients.a2 > 0.0 && coefficients.a2.sqrt() < 1.0
    };
    if stable {
        Ok(())
    } else {
        Err(WaveformError::InvalidParameter {
            name: "coefficients".to_string(),
            reason: "biquad poles must be inside the unit circle".to_string(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BiquadDesignKind {
    LowPass,
    HighPass,
    BandPass,
    BandStop,
    Notch,
}

fn rbj_biquad_for_waveform(
    waveform: &Waveform,
    transform_name: &str,
    kind: BiquadDesignKind,
    center_hz: f64,
    q: f64,
) -> Result<BiquadCoefficients> {
    let sample_rate_hz =
        validate_designed_filter_frequency(&waveform.time, transform_name, "center_hz", center_hz)?;
    rbj_biquad(kind, sample_rate_hz, center_hz, q)
}

fn rbj_biquad(
    kind: BiquadDesignKind,
    sample_rate_hz: f64,
    frequency_hz: f64,
    q: f64,
) -> Result<BiquadCoefficients> {
    validate_positive_parameter("sample_rate_hz", sample_rate_hz)?;
    validate_positive_frequency("frequency_hz", frequency_hz)?;
    validate_q(q)?;
    if frequency_hz >= sample_rate_hz / 2.0 {
        return Err(WaveformError::InvalidParameter {
            name: "frequency_hz".to_string(),
            reason: "must be below Nyquist frequency".to_string(),
        });
    }

    let omega = TAU * frequency_hz / sample_rate_hz;
    let sin_omega = omega.sin();
    let cos_omega = omega.cos();
    let alpha = sin_omega / (2.0 * q);
    let (b0, b1, b2, a0, a1, a2) = match kind {
        BiquadDesignKind::LowPass => (
            (1.0 - cos_omega) / 2.0,
            1.0 - cos_omega,
            (1.0 - cos_omega) / 2.0,
            1.0 + alpha,
            -2.0 * cos_omega,
            1.0 - alpha,
        ),
        BiquadDesignKind::HighPass => (
            (1.0 + cos_omega) / 2.0,
            -(1.0 + cos_omega),
            (1.0 + cos_omega) / 2.0,
            1.0 + alpha,
            -2.0 * cos_omega,
            1.0 - alpha,
        ),
        BiquadDesignKind::BandPass => (
            alpha,
            0.0,
            -alpha,
            1.0 + alpha,
            -2.0 * cos_omega,
            1.0 - alpha,
        ),
        BiquadDesignKind::BandStop | BiquadDesignKind::Notch => (
            1.0,
            -2.0 * cos_omega,
            1.0,
            1.0 + alpha,
            -2.0 * cos_omega,
            1.0 - alpha,
        ),
    };

    normalized_biquad(b0, b1, b2, a0, a1, a2)
}

fn normalized_biquad(
    b0: f64,
    b1: f64,
    b2: f64,
    a0: f64,
    a1: f64,
    a2: f64,
) -> Result<BiquadCoefficients> {
    validate_positive_parameter("a0", a0.abs())?;
    let coefficients = BiquadCoefficients {
        b0: b0 / a0,
        b1: b1 / a0,
        b2: b2 / a0,
        a1: a1 / a0,
        a2: a2 / a0,
    };
    coefficients.validate("coefficients")?;
    Ok(coefficients)
}

fn low_pass_from_analog_pole_pair(
    sample_rate_hz: f64,
    cutoff_hz: f64,
    pole_real: f64,
    pole_imag: f64,
) -> Result<BiquadCoefficients> {
    validate_positive_parameter("sample_rate_hz", sample_rate_hz)?;
    validate_positive_frequency("cutoff_hz", cutoff_hz)?;
    let warped = prewarped_analog_frequency(sample_rate_hz, cutoff_hz)?;
    let (pole_z_real, pole_z_imag) =
        bilinear_complex(warped * pole_real, warped * pole_imag, sample_rate_hz)?;
    let a1 = -2.0 * pole_z_real;
    let a2 = pole_z_real * pole_z_real + pole_z_imag * pole_z_imag;
    let gain = (1.0 + a1 + a2) / 4.0;
    let coefficients = BiquadCoefficients {
        b0: gain,
        b1: 2.0 * gain,
        b2: gain,
        a1,
        a2,
    };
    coefficients.validate("coefficients")?;
    Ok(coefficients)
}

fn chebyshev1_low_pass_biquad(
    sample_rate_hz: f64,
    cutoff_hz: f64,
    ripple_db: f64,
) -> Result<BiquadCoefficients> {
    validate_positive_parameter("ripple_db", ripple_db)?;
    let epsilon = (10.0_f64.powf(ripple_db / 10.0) - 1.0).sqrt();
    validate_positive_parameter("ripple_epsilon", epsilon)?;
    let mu = (1.0 / epsilon).asinh() / 2.0;
    let theta = std::f64::consts::FRAC_PI_4;
    low_pass_from_analog_pole_pair(
        sample_rate_hz,
        cutoff_hz,
        -mu.sinh() * theta.sin(),
        mu.cosh() * theta.cos(),
    )
}

fn chebyshev2_low_pass_biquad(
    sample_rate_hz: f64,
    cutoff_hz: f64,
    stopband_attenuation_db: f64,
) -> Result<BiquadCoefficients> {
    validate_positive_parameter("stopband_attenuation_db", stopband_attenuation_db)?;
    let epsilon = 1.0 / (10.0_f64.powf(stopband_attenuation_db / 10.0) - 1.0).sqrt();
    validate_positive_parameter("stopband_epsilon", epsilon)?;
    let mu = (1.0 / epsilon).asinh() / 2.0;
    let theta = std::f64::consts::FRAC_PI_4;
    let sigma = -mu.sinh() * theta.sin();
    let omega = mu.cosh() * theta.cos();
    let denominator = sigma * sigma + omega * omega;
    validate_positive_parameter("chebyshev2_pole_denominator", denominator)?;
    let warped = prewarped_analog_frequency(sample_rate_hz, cutoff_hz)?;
    let pole_real = warped * sigma / denominator;
    let pole_imag = -warped * omega / denominator;
    let analog_zero_imag = warped / theta.cos();
    let (zero_z_real, zero_z_imag) = bilinear_complex(0.0, analog_zero_imag, sample_rate_hz)?;
    let (pole_z_real, pole_z_imag) = bilinear_complex(pole_real, pole_imag, sample_rate_hz)?;
    biquad_from_digital_zero_pole_pairs(zero_z_real, zero_z_imag, pole_z_real, pole_z_imag)
}

fn bessel_low_pass_biquad(sample_rate_hz: f64, cutoff_hz: f64) -> Result<BiquadCoefficients> {
    low_pass_from_analog_pole_pair(sample_rate_hz, cutoff_hz, -1.5, 3.0_f64.sqrt() / 2.0)
}

fn biquad_from_digital_zero_pole_pairs(
    zero_real: f64,
    zero_imag: f64,
    pole_real: f64,
    pole_imag: f64,
) -> Result<BiquadCoefficients> {
    let base_b0 = 1.0;
    let base_b1 = -2.0 * zero_real;
    let base_b2 = zero_real * zero_real + zero_imag * zero_imag;
    let a1 = -2.0 * pole_real;
    let a2 = pole_real * pole_real + pole_imag * pole_imag;
    let numerator_dc = base_b0 + base_b1 + base_b2;
    validate_positive_parameter("numerator_dc_gain", numerator_dc.abs())?;
    let gain = (1.0 + a1 + a2) / numerator_dc;
    let coefficients = BiquadCoefficients {
        b0: gain * base_b0,
        b1: gain * base_b1,
        b2: gain * base_b2,
        a1,
        a2,
    };
    coefficients.validate("coefficients")?;
    Ok(coefficients)
}

fn prewarped_analog_frequency(sample_rate_hz: f64, cutoff_hz: f64) -> Result<f64> {
    validate_positive_parameter("sample_rate_hz", sample_rate_hz)?;
    validate_positive_frequency("cutoff_hz", cutoff_hz)?;
    if cutoff_hz >= sample_rate_hz / 2.0 {
        return Err(WaveformError::InvalidParameter {
            name: "cutoff_hz".to_string(),
            reason: "must be below Nyquist frequency".to_string(),
        });
    }
    let warped = 2.0 * sample_rate_hz * (std::f64::consts::PI * cutoff_hz / sample_rate_hz).tan();
    validate_positive_parameter("prewarped_frequency", warped)?;
    Ok(warped)
}

fn bilinear_complex(real: f64, imag: f64, sample_rate_hz: f64) -> Result<(f64, f64)> {
    validate_finite_parameter("analog_real", real)?;
    validate_finite_parameter("analog_imag", imag)?;
    validate_positive_parameter("sample_rate_hz", sample_rate_hz)?;
    let two_fs = 2.0 * sample_rate_hz;
    let numerator_real = two_fs + real;
    let numerator_imag = imag;
    let denominator_real = two_fs - real;
    let denominator_imag = -imag;
    let denominator_norm =
        denominator_real * denominator_real + denominator_imag * denominator_imag;
    validate_positive_parameter("bilinear_denominator", denominator_norm)?;
    Ok((
        (numerator_real * denominator_real + numerator_imag * denominator_imag) / denominator_norm,
        (numerator_imag * denominator_real - numerator_real * denominator_imag) / denominator_norm,
    ))
}

fn validate_no_infinite_samples(transform_name: &str, samples: &[f64]) -> Result<()> {
    if samples.iter().any(|sample| sample.is_infinite()) {
        return Err(WaveformError::InvalidWaveform {
            reason: format!("{transform_name} rejects infinite samples"),
        });
    }
    Ok(())
}

fn validate_channels_allowing_nan(waveform: &Waveform, transform_name: &str) -> Result<()> {
    for channel in &waveform.channels {
        validate_no_infinite_samples(transform_name, &channel.samples)?;
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

fn validate_finite_time_axis(time: &[f64], transform_name: &str) -> Result<()> {
    if time.iter().any(|sample_time| !sample_time.is_finite()) {
        return Err(WaveformError::InvalidWaveform {
            reason: format!("time samples must be finite for {transform_name}"),
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

fn validate_window_samples(window_samples: usize) -> Result<()> {
    if window_samples == 0 {
        return Err(WaveformError::InvalidParameter {
            name: "window_samples".to_string(),
            reason: "must be greater than zero".to_string(),
        });
    }
    Ok(())
}

fn validate_weights(weights: &[f64]) -> Result<()> {
    if weights.is_empty() {
        return Err(WaveformError::InvalidParameter {
            name: "weights".to_string(),
            reason: "must include at least one weight".to_string(),
        });
    }
    for weight in weights {
        validate_finite_parameter("weights", *weight)?;
        if *weight <= 0.0 {
            return Err(WaveformError::InvalidParameter {
                name: "weights".to_string(),
                reason: "all weights must be greater than zero".to_string(),
            });
        }
    }
    Ok(())
}

fn validate_alpha(alpha: f64) -> Result<()> {
    validate_finite_parameter("alpha", alpha)?;
    if !(0.0..=1.0).contains(&alpha) || alpha == 0.0 {
        return Err(WaveformError::InvalidParameter {
            name: "alpha".to_string(),
            reason: "must be greater than zero and less than or equal to one".to_string(),
        });
    }
    Ok(())
}

fn validate_polynomial_fit_order(order: usize) -> Result<()> {
    if order > 5 {
        return Err(WaveformError::InvalidParameter {
            name: "polynomial_order".to_string(),
            reason: "must be no greater than 5 for dependency-free fitting".to_string(),
        });
    }
    Ok(())
}

fn validate_savitzky_golay_parameters(
    window_samples: usize,
    polynomial_order: usize,
) -> Result<()> {
    validate_window_samples(window_samples)?;
    validate_polynomial_fit_order(polynomial_order)?;
    if window_samples <= polynomial_order {
        return Err(WaveformError::InvalidParameter {
            name: "window_samples".to_string(),
            reason: "must be greater than polynomial_order".to_string(),
        });
    }
    if window_samples % 2 == 0 {
        return Err(WaveformError::InvalidParameter {
            name: "window_samples".to_string(),
            reason: "must be odd for centered Savitzky-Golay smoothing".to_string(),
        });
    }
    Ok(())
}

fn trailing_weighted_moving_average(samples: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut filtered = Vec::with_capacity(samples.len());
    for index in 0..samples.len() {
        let window_len = weights.len().min(index + 1);
        let sample_start = index + 1 - window_len;
        let weight_start = weights.len() - window_len;
        let window = &samples[sample_start..=index];
        let active_weights = &weights[weight_start..];
        let weight_sum = active_weights.iter().sum::<f64>();
        let weighted_sum = window
            .iter()
            .zip(active_weights)
            .map(|(sample, weight)| sample * weight)
            .sum::<f64>();
        filtered.push(weighted_sum / weight_sum);
    }
    filtered
}

fn exponential_moving_average(samples: &[f64], alpha: f64) -> Vec<f64> {
    if samples.is_empty() {
        return Vec::new();
    }
    let mut filtered = Vec::with_capacity(samples.len());
    filtered.push(samples[0]);
    for sample in &samples[1..] {
        let previous = *filtered
            .last()
            .expect("EMA always has one seeded output sample");
        filtered.push(alpha * sample + (1.0 - alpha) * previous);
    }
    filtered
}

fn centered_shrinking_window_bounds(
    len: usize,
    index: usize,
    window_samples: usize,
) -> (usize, usize) {
    let radius = window_samples / 2;
    let start = index.saturating_sub(radius);
    let end = (index + radius + 1).min(len);
    (start, end)
}

fn centered_fixed_window_bounds(len: usize, index: usize, window_samples: usize) -> (usize, usize) {
    let window_len = window_samples.min(len);
    let radius = window_len / 2;
    let mut start = index.saturating_sub(radius);
    if start + window_len > len {
        start = len - window_len;
    }
    (start, start + window_len)
}

fn centered_boxcar_smoothing(samples: &[f64], window_samples: usize) -> Vec<f64> {
    let mut filtered = Vec::with_capacity(samples.len());
    for index in 0..samples.len() {
        let (start, end) = centered_shrinking_window_bounds(samples.len(), index, window_samples);
        let window = &samples[start..end];
        filtered.push(window.iter().sum::<f64>() / window.len() as f64);
    }
    filtered
}

fn centered_gaussian_smoothing(
    samples: &[f64],
    window_samples: usize,
    sigma_samples: f64,
) -> Vec<f64> {
    let mut filtered = Vec::with_capacity(samples.len());
    for index in 0..samples.len() {
        let (start, end) = centered_shrinking_window_bounds(samples.len(), index, window_samples);
        let mut weighted_sum = 0.0;
        let mut weight_sum = 0.0;
        for (sample_index, sample) in samples.iter().enumerate().take(end).skip(start) {
            let distance = sample_index as f64 - index as f64;
            let weight = (-0.5 * (distance / sigma_samples).powi(2)).exp();
            weighted_sum += sample * weight;
            weight_sum += weight;
        }
        filtered.push(weighted_sum / weight_sum);
    }
    filtered
}

fn centered_moving_median(samples: &[f64], window_samples: usize) -> Vec<f64> {
    let mut filtered = Vec::with_capacity(samples.len());
    for index in 0..samples.len() {
        let (start, end) = centered_shrinking_window_bounds(samples.len(), index, window_samples);
        filtered.push(median(&samples[start..end]));
    }
    filtered
}

fn savitzky_golay_smoothing(
    samples: &[f64],
    window_samples: usize,
    polynomial_order: usize,
) -> Result<Vec<f64>> {
    if samples.len() <= polynomial_order {
        return Err(WaveformError::InvalidWaveform {
            reason: "savitzky_golay requires more samples than polynomial_order".to_string(),
        });
    }

    let mut filtered = Vec::with_capacity(samples.len());
    for index in 0..samples.len() {
        let (start, end) = centered_fixed_window_bounds(samples.len(), index, window_samples);
        let x = (start..end)
            .map(|sample_index| sample_index as f64 - index as f64)
            .collect::<Vec<_>>();
        let y = samples[start..end].to_vec();
        let coefficients =
            least_squares_polynomial_coefficients(&x, &y, polynomial_order, "savitzky_golay")?;
        filtered.push(coefficients[0]);
    }
    Ok(filtered)
}

fn detrend_samples(time: &[f64], samples: &[f64], polynomial_order: usize) -> Result<Vec<f64>> {
    if samples.len() <= polynomial_order {
        return Err(WaveformError::InvalidWaveform {
            reason: "detrending requires more samples than polynomial_order".to_string(),
        });
    }
    let x = normalized_time_axis(time)?;
    let coefficients =
        least_squares_polynomial_coefficients(&x, samples, polynomial_order, "detrending")?;
    Ok(x.iter()
        .zip(samples)
        .map(|(x_value, sample)| sample - evaluate_polynomial(&coefficients, *x_value))
        .collect())
}

fn normalized_time_axis(time: &[f64]) -> Result<Vec<f64>> {
    let start = time[0];
    let end = *time
        .last()
        .expect("validated waveforms always include a timestamp");
    let center = (start + end) / 2.0;
    let scale = (end - start) / 2.0;
    validate_positive_parameter("time_range_s", scale)?;
    Ok(time
        .iter()
        .map(|sample_time| (sample_time - center) / scale)
        .collect())
}

fn least_squares_polynomial_coefficients(
    x: &[f64],
    y: &[f64],
    order: usize,
    transform_name: &str,
) -> Result<Vec<f64>> {
    validate_polynomial_fit_order(order)?;
    let coefficient_count = order + 1;
    if x.len() != y.len() || x.len() < coefficient_count {
        return Err(WaveformError::InvalidWaveform {
            reason: format!("{transform_name} requires at least order + 1 samples"),
        });
    }

    let mut normal_matrix = vec![vec![0.0; coefficient_count]; coefficient_count];
    let mut normal_rhs = vec![0.0; coefficient_count];
    for (x_value, y_value) in x.iter().zip(y) {
        validate_finite_parameter("fit_x", *x_value)?;
        validate_finite_parameter("fit_y", *y_value)?;
        let mut powers = vec![1.0; coefficient_count];
        for power in 1..coefficient_count {
            powers[power] = powers[power - 1] * x_value;
        }
        for row in 0..coefficient_count {
            normal_rhs[row] += powers[row] * y_value;
            for col in 0..coefficient_count {
                normal_matrix[row][col] += powers[row] * powers[col];
            }
        }
    }

    solve_linear_system(normal_matrix, normal_rhs, transform_name)
}

fn solve_linear_system(
    mut matrix: Vec<Vec<f64>>,
    mut rhs: Vec<f64>,
    transform_name: &str,
) -> Result<Vec<f64>> {
    let size = rhs.len();
    for pivot_index in 0..size {
        let mut pivot_row = pivot_index;
        let mut pivot_abs = matrix[pivot_index][pivot_index].abs();
        for (row, values) in matrix.iter().enumerate().take(size).skip(pivot_index + 1) {
            let candidate_abs = values[pivot_index].abs();
            if candidate_abs > pivot_abs {
                pivot_abs = candidate_abs;
                pivot_row = row;
            }
        }
        if pivot_abs <= 1.0e-12 {
            return Err(WaveformError::InvalidWaveform {
                reason: format!("{transform_name} polynomial fit is singular"),
            });
        }
        if pivot_row != pivot_index {
            matrix.swap(pivot_index, pivot_row);
            rhs.swap(pivot_index, pivot_row);
        }

        let pivot = matrix[pivot_index][pivot_index];
        for value in matrix[pivot_index].iter_mut().take(size).skip(pivot_index) {
            *value /= pivot;
        }
        rhs[pivot_index] /= pivot;
        let pivot_tail = matrix[pivot_index][pivot_index..size].to_vec();

        for row in 0..size {
            if row == pivot_index {
                continue;
            }
            let factor = matrix[row][pivot_index];
            if factor == 0.0 {
                continue;
            }
            for (value, pivot_value) in matrix[row][pivot_index..size]
                .iter_mut()
                .zip(pivot_tail.iter())
            {
                *value -= factor * pivot_value;
            }
            rhs[row] -= factor * rhs[pivot_index];
        }
    }
    Ok(rhs)
}

fn median(samples: &[f64]) -> f64 {
    let mut sorted = samples.to_vec();
    sorted.sort_by(f64::total_cmp);
    let middle = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        (sorted[middle - 1] + sorted[middle]) / 2.0
    } else {
        sorted[middle]
    }
}

fn hampel_filter(samples: &[f64], window_samples: usize, outlier_sigma: f64) -> Vec<f64> {
    let mut filtered = Vec::with_capacity(samples.len());
    for index in 0..samples.len() {
        let (start, end) = centered_fixed_window_bounds(samples.len(), index, window_samples);
        let window = &samples[start..end];
        let window_median = median(window);
        let deviations = window
            .iter()
            .map(|sample| (sample - window_median).abs())
            .collect::<Vec<_>>();
        let mad = median(&deviations);
        let scaled_mad = 1.4826 * mad;
        let delta = (samples[index] - window_median).abs();
        let is_outlier = if scaled_mad <= f64::EPSILON {
            delta > f64::EPSILON
        } else {
            delta > outlier_sigma * scaled_mad
        };
        filtered.push(if is_outlier {
            window_median
        } else {
            samples[index]
        });
    }
    filtered
}

fn spike_remove(samples: &[f64], window_samples: usize, threshold_v: f64) -> Vec<f64> {
    let mut filtered = Vec::with_capacity(samples.len());
    for index in 0..samples.len() {
        let (start, end) = centered_fixed_window_bounds(samples.len(), index, window_samples);
        let window_median = median(&samples[start..end]);
        if (samples[index] - window_median).abs() > threshold_v {
            filtered.push(window_median);
        } else {
            filtered.push(samples[index]);
        }
    }
    filtered
}

fn normalize_samples(samples: &[f64], mode: NormalizeMode) -> Result<Vec<f64>> {
    match mode {
        NormalizeMode::ZeroToOne => {
            let (min, max) = min_max(samples);
            normalize_range(samples, min, max, 0.0, 1.0)
        }
        NormalizeMode::MinusOneToOne => {
            let (min, max) = min_max(samples);
            normalize_range(samples, min, max, -1.0, 1.0)
        }
        NormalizeMode::ZScore => {
            let mean = samples.iter().sum::<f64>() / samples.len() as f64;
            let variance = samples
                .iter()
                .map(|sample| {
                    let delta = sample - mean;
                    delta * delta
                })
                .sum::<f64>()
                / samples.len() as f64;
            let std_dev = variance.sqrt();
            if std_dev <= f64::EPSILON {
                return Err(WaveformError::InvalidWaveform {
                    reason: "normalize z_score requires non-constant samples".to_string(),
                });
            }
            Ok(samples
                .iter()
                .map(|sample| (sample - mean) / std_dev)
                .collect())
        }
        NormalizeMode::Range {
            input_min_v,
            input_max_v,
            output_min,
            output_max,
        } => {
            validate_normalize_range(input_min_v, input_max_v, output_min, output_max)?;
            normalize_range(samples, input_min_v, input_max_v, output_min, output_max)
        }
    }
}

fn min_max(samples: &[f64]) -> (f64, f64) {
    let mut min = samples[0];
    let mut max = samples[0];
    for sample in &samples[1..] {
        min = min.min(*sample);
        max = max.max(*sample);
    }
    (min, max)
}

fn validate_normalize_range(
    input_min_v: f64,
    input_max_v: f64,
    output_min: f64,
    output_max: f64,
) -> Result<()> {
    validate_finite_parameter("input_min_v", input_min_v)?;
    validate_finite_parameter("input_max_v", input_max_v)?;
    validate_finite_parameter("output_min", output_min)?;
    validate_finite_parameter("output_max", output_max)?;
    if input_max_v <= input_min_v {
        return Err(WaveformError::InvalidParameter {
            name: "input_max_v".to_string(),
            reason: "must be greater than input_min_v".to_string(),
        });
    }
    if output_max <= output_min {
        return Err(WaveformError::InvalidParameter {
            name: "output_max".to_string(),
            reason: "must be greater than output_min".to_string(),
        });
    }
    Ok(())
}

fn normalize_range(
    samples: &[f64],
    input_min_v: f64,
    input_max_v: f64,
    output_min: f64,
    output_max: f64,
) -> Result<Vec<f64>> {
    validate_normalize_range(input_min_v, input_max_v, output_min, output_max)?;
    Ok(samples
        .iter()
        .map(|sample| {
            let fraction = (sample - input_min_v) / (input_max_v - input_min_v);
            output_min + fraction * (output_max - output_min)
        })
        .collect())
}

fn validate_piecewise_points(points: &[PiecewisePoint]) -> Result<()> {
    if points.len() < 2 {
        return Err(WaveformError::InvalidParameter {
            name: "points".to_string(),
            reason: "must include at least two points".to_string(),
        });
    }
    for point in points {
        validate_finite_parameter("points.x", point.x)?;
        validate_finite_parameter("points.y", point.y)?;
    }
    for pair in points.windows(2) {
        if pair[1].x <= pair[0].x {
            return Err(WaveformError::InvalidParameter {
                name: "points".to_string(),
                reason: "x values must be strictly increasing".to_string(),
            });
        }
    }
    Ok(())
}

fn interpolate_piecewise(points: &[PiecewisePoint], sample: f64) -> f64 {
    if sample <= points[0].x {
        return points[0].y;
    }
    let last_index = points.len() - 1;
    if sample >= points[last_index].x {
        return points[last_index].y;
    }
    let upper = points
        .partition_point(|point| point.x < sample)
        .min(last_index);
    if points[upper].x == sample {
        return points[upper].y;
    }
    let lower = upper - 1;
    let fraction = (sample - points[lower].x) / (points[upper].x - points[lower].x);
    points[lower].y + fraction * (points[upper].y - points[lower].y)
}

fn validate_polynomial_coefficients(coefficients: &[f64]) -> Result<()> {
    if coefficients.is_empty() {
        return Err(WaveformError::InvalidParameter {
            name: "coefficients".to_string(),
            reason: "must include at least one coefficient".to_string(),
        });
    }
    for coefficient in coefficients {
        validate_finite_parameter("coefficients", *coefficient)?;
    }
    Ok(())
}

fn evaluate_polynomial(coefficients: &[f64], sample: f64) -> f64 {
    coefficients
        .iter()
        .rev()
        .fold(0.0, |accumulator, coefficient| {
            accumulator * sample + coefficient
        })
}

fn sorted_time_indices(time: &[f64]) -> Vec<usize> {
    let mut indices = (0..time.len()).collect::<Vec<_>>();
    indices.sort_by(|left, right| time[*left].total_cmp(&time[*right]));
    indices
}

fn select_time(time: &[f64], indices: &[usize]) -> Vec<f64> {
    indices.iter().map(|index| time[*index]).collect()
}

fn select_channels(channels: &[Channel], indices: &[usize]) -> Vec<Channel> {
    channels
        .iter()
        .map(|channel| {
            let samples = indices
                .iter()
                .map(|index| channel.samples[*index])
                .collect::<Vec<_>>();
            Channel::new(channel.name.clone(), channel.unit.clone(), samples)
        })
        .collect()
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

fn interpolate_nan_samples(time: &[f64], samples: &[f64]) -> Result<Vec<f64>> {
    if samples.iter().all(|sample| sample.is_nan()) {
        return Err(WaveformError::InvalidWaveform {
            reason: "nan_interpolate requires at least one finite sample per channel".to_string(),
        });
    }

    let mut repaired = Vec::with_capacity(samples.len());
    for (index, sample) in samples.iter().enumerate() {
        if sample.is_finite() {
            repaired.push(*sample);
        } else if sample.is_nan() {
            repaired.push(interpolated_nan_value(time, samples, index));
        } else {
            return Err(WaveformError::InvalidWaveform {
                reason: "nan_interpolate rejects infinite samples".to_string(),
            });
        }
    }
    Ok(repaired)
}

fn interpolated_nan_value(time: &[f64], samples: &[f64], index: usize) -> f64 {
    let previous = (0..index)
        .rev()
        .find(|candidate| samples[*candidate].is_finite());
    let next = ((index + 1)..samples.len()).find(|candidate| samples[*candidate].is_finite());

    match (previous, next) {
        (Some(previous), Some(next)) => {
            let previous_time = time[previous];
            let next_time = time[next];
            let fraction = (time[index] - previous_time) / (next_time - previous_time);
            samples[previous] + fraction * (samples[next] - samples[previous])
        }
        (Some(previous), None) => samples[previous],
        (None, Some(next)) => samples[next],
        (None, None) => unreachable!("all-NaN samples are rejected before interpolation"),
    }
}

fn fixed_time_grid(source_time: &[f64], sample_interval_s: f64) -> Result<Vec<f64>> {
    let start = source_time[0];
    let end = *source_time
        .last()
        .expect("validated waveforms always include a timestamp");
    let duration = end - start;
    if duration < 0.0 {
        return Err(WaveformError::InvalidWaveform {
            reason: "fixed-grid resampling requires ascending timestamps".to_string(),
        });
    }

    let intervals = (duration / sample_interval_s).floor();
    if !intervals.is_finite() || intervals < 0.0 {
        return Err(WaveformError::InvalidParameter {
            name: "sample_interval_s".to_string(),
            reason: "would produce an invalid fixed time grid".to_string(),
        });
    }

    let sample_count = intervals as usize + 1;
    if sample_count > MAX_RESAMPLED_SAMPLES {
        return Err(WaveformError::InvalidParameter {
            name: "sample_interval_s".to_string(),
            reason: format!("would produce more than {MAX_RESAMPLED_SAMPLES} samples"),
        });
    }

    Ok((0..sample_count)
        .map(|index| start + sample_interval_s * index as f64)
        .collect())
}

fn time_grid_by_count(
    start_time_s: f64,
    sample_interval_s: f64,
    sample_count: usize,
) -> Result<Vec<f64>> {
    validate_finite_parameter("start_time_s", start_time_s)?;
    validate_sample_interval(sample_interval_s)?;
    if sample_count == 0 {
        return Err(WaveformError::InvalidParameter {
            name: "sample_count".to_string(),
            reason: "must be greater than zero".to_string(),
        });
    }
    if sample_count > MAX_RESAMPLED_SAMPLES {
        return Err(WaveformError::InvalidParameter {
            name: "sample_count".to_string(),
            reason: format!("would produce more than {MAX_RESAMPLED_SAMPLES} samples"),
        });
    }
    (0..sample_count)
        .map(|index| {
            let sample_time = start_time_s + sample_interval_s * index as f64;
            if sample_time.is_finite() {
                Ok(sample_time)
            } else {
                Err(WaveformError::InvalidParameter {
                    name: "sample_interval_s".to_string(),
                    reason: "would produce a non-finite timestamp".to_string(),
                })
            }
        })
        .collect()
}

fn resampled_count_for_uniform_factor(source_len: usize, factor: usize) -> Result<usize> {
    if source_len == 0 {
        return Err(WaveformError::EmptyInput);
    }
    let sample_count = (source_len - 1)
        .checked_mul(factor)
        .and_then(|count| count.checked_add(1))
        .ok_or_else(|| WaveformError::InvalidParameter {
            name: "factor".to_string(),
            reason: "would overflow output sample count".to_string(),
        })?;
    if sample_count > MAX_RESAMPLED_SAMPLES {
        return Err(WaveformError::InvalidParameter {
            name: "factor".to_string(),
            reason: format!("would produce more than {MAX_RESAMPLED_SAMPLES} samples"),
        });
    }
    Ok(sample_count)
}

fn decimated_indices(len: usize, factor: usize) -> Vec<usize> {
    (0..len).step_by(factor).collect()
}

fn interpolate_series(time: &[f64], samples: &[f64], target_time: f64) -> f64 {
    if target_time <= time[0] {
        return samples[0];
    }
    let last_index = time.len() - 1;
    if target_time >= time[last_index] {
        return samples[last_index];
    }

    let upper = time
        .partition_point(|sample_time| *sample_time < target_time)
        .min(last_index);
    if time[upper] == target_time {
        return samples[upper];
    }

    let lower = upper - 1;
    let fraction = (target_time - time[lower]) / (time[upper] - time[lower]);
    samples[lower] + fraction * (samples[upper] - samples[lower])
}

fn previous_hold_series(time: &[f64], samples: &[f64], target_time: f64) -> f64 {
    if target_time <= time[0] {
        return samples[0];
    }
    let last_index = time.len() - 1;
    if target_time >= time[last_index] {
        return samples[last_index];
    }
    let upper = time
        .partition_point(|sample_time| *sample_time <= target_time)
        .min(last_index);
    samples[upper.saturating_sub(1)]
}

fn fractional_delay_samples(time: &[f64], samples: &[f64], delay_s: f64) -> Vec<f64> {
    time.iter()
        .map(|sample_time| interpolate_series(time, samples, sample_time - delay_s))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct CrossCorrelationDelayEstimate {
    lag_samples: isize,
    confidence: f64,
}

fn estimate_cross_correlation_delay(
    reference: &[f64],
    target: &[f64],
    max_lag_samples: usize,
) -> Result<CrossCorrelationDelayEstimate> {
    if reference.len() != target.len() {
        return Err(WaveformError::InvalidWaveform {
            reason: "cross_correlation_delay requires channels with matching sample counts"
                .to_string(),
        });
    }
    if reference.len() < 2 {
        return Err(WaveformError::InvalidWaveform {
            reason: "cross_correlation_delay requires at least two samples".to_string(),
        });
    }
    if max_lag_samples >= reference.len() {
        return Err(WaveformError::InvalidParameter {
            name: "max_lag_samples".to_string(),
            reason: "must be less than the waveform sample count".to_string(),
        });
    }

    let max_lag = max_lag_samples as isize;
    let mut best: Option<CrossCorrelationDelayEstimate> = None;
    for lag in -max_lag..=max_lag {
        let Some(correlation) = normalized_cross_correlation_at_lag(reference, target, lag) else {
            continue;
        };
        let candidate = CrossCorrelationDelayEstimate {
            lag_samples: lag,
            confidence: correlation.abs(),
        };
        let replace = match best {
            Some(current) => {
                candidate.confidence > current.confidence + 1.0e-12
                    || ((candidate.confidence - current.confidence).abs() <= 1.0e-12
                        && candidate.lag_samples.abs() < current.lag_samples.abs())
            }
            None => true,
        };
        if replace {
            best = Some(candidate);
        }
    }

    best.ok_or_else(|| WaveformError::InvalidWaveform {
        reason: "cross_correlation_delay could not compute a finite delay estimate".to_string(),
    })
}

fn normalized_cross_correlation_at_lag(
    reference: &[f64],
    target: &[f64],
    lag_samples: isize,
) -> Option<f64> {
    let mut pairs = Vec::new();
    for (reference_index, reference_sample) in reference.iter().enumerate() {
        let target_index = reference_index as isize + lag_samples;
        if target_index < 0 || target_index >= target.len() as isize {
            continue;
        }
        pairs.push((*reference_sample, target[target_index as usize]));
    }
    if pairs.len() < 2 {
        return None;
    }

    let reference_mean = pairs
        .iter()
        .map(|(reference_sample, _)| reference_sample)
        .sum::<f64>()
        / pairs.len() as f64;
    let target_mean = pairs
        .iter()
        .map(|(_, target_sample)| target_sample)
        .sum::<f64>()
        / pairs.len() as f64;
    let mut numerator = 0.0;
    let mut reference_energy = 0.0;
    let mut target_energy = 0.0;
    for (reference_sample, target_sample) in pairs {
        let reference_delta = reference_sample - reference_mean;
        let target_delta = target_sample - target_mean;
        numerator += reference_delta * target_delta;
        reference_energy += reference_delta * reference_delta;
        target_energy += target_delta * target_delta;
    }
    let denominator = (reference_energy * target_energy).sqrt();
    if denominator <= f64::EPSILON {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn first_order_high_pass(time: &[f64], samples: &[f64], cutoff_hz: f64) -> Vec<f64> {
    if samples.is_empty() {
        return Vec::new();
    }

    let rc = 1.0 / (TAU * cutoff_hz);
    let mut filtered = Vec::with_capacity(samples.len());
    filtered.push(0.0);

    for index in 1..samples.len() {
        let dt = time[index] - time[index - 1];
        let alpha = rc / (rc + dt);
        let previous = filtered[index - 1];
        filtered.push(alpha * (previous + samples[index] - samples[index - 1]));
    }

    filtered
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

    fn waveform_with_time(time: Vec<f64>, samples: Vec<f64>) -> Waveform {
        Waveform::new(time, vec![Channel::new("input_v", Unit::volts(), samples)])
            .expect("test waveform should be valid")
    }

    fn waveform_with_channels(time: Vec<f64>, channels: Vec<Channel>) -> Waveform {
        Waveform::new(time, channels).expect("test waveform should be valid")
    }

    fn assert_close_vec(actual: &[f64], expected: &[f64]) {
        assert_eq!(actual.len(), expected.len());
        for (actual, expected) in actual.iter().zip(expected) {
            assert!(
                (actual - expected).abs() < 1.0e-9,
                "expected {expected}, got {actual}"
            );
        }
    }

    fn assert_close_value(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "expected {expected} +/- {tolerance}, got {actual}"
        );
    }

    fn sampled_waveform(
        sample_count: usize,
        sample_rate_hz: f64,
        signal: impl Fn(usize, f64) -> f64,
    ) -> Waveform {
        let sample_interval_s = 1.0 / sample_rate_hz;
        let time = (0..sample_count)
            .map(|index| index as f64 * sample_interval_s)
            .collect::<Vec<_>>();
        let samples = time
            .iter()
            .enumerate()
            .map(|(index, timestamp)| signal(index, *timestamp))
            .collect::<Vec<_>>();
        waveform_with_time(time, samples)
    }

    fn biquad_response_magnitude(
        coefficients: BiquadCoefficients,
        sample_rate_hz: f64,
        frequency_hz: f64,
    ) -> f64 {
        let omega = TAU * frequency_hz / sample_rate_hz;
        let numerator_real =
            coefficients.b0 + coefficients.b1 * omega.cos() + coefficients.b2 * (2.0 * omega).cos();
        let numerator_imag = -coefficients.b1 * omega.sin() - coefficients.b2 * (2.0 * omega).sin();
        let denominator_real =
            1.0 + coefficients.a1 * omega.cos() + coefficients.a2 * (2.0 * omega).cos();
        let denominator_imag =
            -coefficients.a1 * omega.sin() - coefficients.a2 * (2.0 * omega).sin();
        let numerator = (numerator_real * numerator_real + numerator_imag * numerator_imag).sqrt();
        let denominator =
            (denominator_real * denominator_real + denominator_imag * denominator_imag).sqrt();
        numerator / denominator
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
    fn m26_nan_interpolate_repairs_samples_without_mutating_input() {
        let input = waveform(vec![f64::NAN, 1.0, f64::NAN, 4.0]);

        let repaired = NanInterpolateTransform
            .apply(&input)
            .expect("NaN interpolation should apply");

        assert!(input.channels[0].samples[0].is_nan());
        assert_close_vec(&repaired.channels[0].samples, &[1.0, 1.0, 2.5, 4.0]);
        assert_eq!(
            repaired.metadata.transform_history,
            vec!["nan_interpolate()"]
        );
        let step = &repaired.metadata.transform_steps[0];
        assert_eq!(step.name, "nan_interpolate");
        assert_eq!(step.category, TransformCategory::DataCleaning);
        assert!(step.sample_rate_required);
        assert!(!step.causal);
        assert!(!step.streaming_supported);
        assert!(step.offline_only);
        assert_eq!(step.phase_effect, TransformPhaseEffect::None);
    }

    #[test]
    fn m26_nan_remove_drops_rows_with_any_nan() {
        let input = waveform_with_channels(
            vec![0.0, 1.0, 2.0, 3.0],
            vec![
                Channel::new("input_v", Unit::volts(), vec![0.0, f64::NAN, 2.0, 3.0]),
                Channel::new("reference_v", Unit::volts(), vec![0.0, 1.0, f64::NAN, 3.0]),
            ],
        );

        let cleaned = NanRemoveTransform
            .apply(&input)
            .expect("NaN removal should apply");

        assert_eq!(cleaned.time, vec![0.0, 3.0]);
        assert_eq!(cleaned.channels[0].samples, vec![0.0, 3.0]);
        assert_eq!(cleaned.channels[1].samples, vec![0.0, 3.0]);
        assert_eq!(
            cleaned.metadata.transform_history,
            vec!["nan_remove(policy=drop_rows_with_nan)"]
        );
        let step = &cleaned.metadata.transform_steps[0];
        assert_eq!(step.name, "nan_remove");
        assert_eq!(step.category, TransformCategory::DataCleaning);
        assert!(!step.sample_rate_required);
        assert!(step.causal);
        assert!(step.streaming_supported);
        assert!(!step.offline_only);
    }

    #[test]
    fn m26_timestamp_sort_and_dedupe_repair_time_axis() {
        let input = waveform_with_time(vec![0.2, 0.0, 0.2, 0.1], vec![2.0, 0.0, 20.0, 1.0]);

        let sorted = TimestampSortTransform
            .apply(&input)
            .expect("timestamp sort should apply");
        assert_eq!(sorted.time, vec![0.0, 0.1, 0.2, 0.2]);
        assert_eq!(sorted.channels[0].samples, vec![0.0, 1.0, 2.0, 20.0]);
        assert_eq!(
            sorted.metadata.transform_history,
            vec!["timestamp_sort(order=ascending)"]
        );

        let deduped = DedupeTimestampsTransform
            .apply(&sorted)
            .expect("timestamp dedupe should apply");
        assert_eq!(deduped.time, vec![0.0, 0.1, 0.2]);
        assert_eq!(deduped.channels[0].samples, vec![0.0, 1.0, 2.0]);
        assert_eq!(
            deduped.metadata.transform_history,
            vec![
                "timestamp_sort(order=ascending)",
                "dedupe_timestamps(policy=keep_first)"
            ]
        );
        assert_eq!(deduped.metadata.transform_steps[0].sequence_index, 0);
        assert_eq!(deduped.metadata.transform_steps[1].sequence_index, 1);
        assert!(deduped.metadata.transform_steps[1].offline_only);
    }

    #[test]
    fn m26_crop_and_fixed_delay_update_time_axis() {
        let input = waveform(vec![0.0, 1.0, 2.0, 3.0]);

        let cropped = CropTransform {
            start_time_s: 1.0,
            end_time_s: 2.0,
        }
        .apply(&input)
        .expect("crop should apply");
        assert_eq!(cropped.time, vec![1.0, 2.0]);
        assert_eq!(cropped.channels[0].samples, vec![1.0, 2.0]);
        assert_eq!(
            cropped.metadata.transform_history,
            vec!["crop(start_time_s=1,end_time_s=2)"]
        );

        let delayed = FixedDelayTransform { delay_s: 0.5 }
            .apply(&cropped)
            .expect("fixed delay should apply");
        assert_eq!(delayed.time, vec![1.5, 2.5]);
        assert_eq!(delayed.channels[0].samples, vec![1.0, 2.0]);
        assert_eq!(
            delayed.metadata.transform_history,
            vec![
                "crop(start_time_s=1,end_time_s=2)",
                "fixed_delay(delay_s=0.5)"
            ]
        );
        assert_eq!(
            delayed.metadata.transform_steps[1].category,
            TransformCategory::Resampling
        );
        assert_eq!(
            delayed.metadata.transform_steps[1].phase_effect,
            TransformPhaseEffect::Delay
        );
    }

    #[test]
    fn m26_gap_fill_and_resample_fixed_build_uniform_grid() {
        let input = waveform_with_time(vec![0.0, 0.2, 0.4], vec![0.0, 2.0, 4.0]);

        let gap_filled = GapFillTransform {
            sample_interval_s: 0.1,
        }
        .apply(&input)
        .expect("gap fill should apply");
        assert_close_vec(&gap_filled.time, &[0.0, 0.1, 0.2, 0.3, 0.4]);
        assert_close_vec(&gap_filled.channels[0].samples, &[0.0, 1.0, 2.0, 3.0, 4.0]);
        let gap_step = &gap_filled.metadata.transform_steps[0];
        assert_eq!(gap_step.name, "gap_fill");
        assert_eq!(gap_step.category, TransformCategory::Resampling);
        assert!(gap_step.sample_rate_required);
        assert!(!gap_step.causal);
        assert!(!gap_step.streaming_supported);
        assert!(gap_step.offline_only);

        let resampled = ResampleFixedTransform {
            sample_interval_s: 0.2,
        }
        .apply(&gap_filled)
        .expect("fixed resampling should apply");
        assert_close_vec(&resampled.time, &[0.0, 0.2, 0.4]);
        assert_close_vec(&resampled.channels[0].samples, &[0.0, 2.0, 4.0]);
        assert_eq!(resampled.metadata.transform_steps[1].name, "resample_fixed");
    }

    #[test]
    fn m26_channel_delay_aligns_selected_channel() {
        let input = waveform_with_channels(
            vec![0.0, 1.0, 2.0, 3.0],
            vec![
                Channel::new("input_v", Unit::volts(), vec![0.0, 10.0, 20.0, 30.0]),
                Channel::new("reference_v", Unit::volts(), vec![5.0, 5.0, 5.0, 5.0]),
            ],
        );

        let aligned = ChannelDelayTransform {
            channel: "input_v".to_string(),
            delay_s: 1.0,
        }
        .apply(&input)
        .expect("channel delay should apply");

        assert_eq!(aligned.time, input.time);
        assert_eq!(aligned.channels[0].samples, vec![0.0, 0.0, 10.0, 20.0]);
        assert_eq!(aligned.channels[1].samples, vec![5.0, 5.0, 5.0, 5.0]);
        assert_eq!(
            aligned.metadata.transform_history,
            vec!["channel_delay(channel=input_v,delay_s=1)"]
        );
        let step = &aligned.metadata.transform_steps[0];
        assert_eq!(step.name, "channel_delay");
        assert_eq!(step.category, TransformCategory::Resampling);
        assert!(!step.causal);
        assert!(step.offline_only);
    }

    #[test]
    fn m26_transforms_reject_invalid_inputs() {
        assert!(matches!(
            NanInterpolateTransform.apply(&waveform(vec![f64::NAN, f64::NAN, f64::NAN, f64::NAN])),
            Err(WaveformError::InvalidWaveform { .. })
        ));
        assert!(matches!(
            GapFillTransform {
                sample_interval_s: 0.0,
            }
            .apply(&waveform(vec![0.0, 1.0, 2.0, 3.0])),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            ChannelDelayTransform {
                channel: "missing_v".to_string(),
                delay_s: 1.0,
            }
            .apply(&waveform(vec![0.0, 1.0, 2.0, 3.0])),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            CropTransform {
                start_time_s: 3.0,
                end_time_s: 1.0,
            }
            .apply(&waveform(vec![0.0, 1.0, 2.0, 3.0])),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            CropTransform {
                start_time_s: 10.0,
                end_time_s: 11.0,
            }
            .apply(&waveform(vec![0.0, 1.0, 2.0, 3.0])),
            Err(WaveformError::InvalidWaveform { .. })
        ));
    }

    #[test]
    fn m27_pointwise_transforms_apply_without_mutating_input() {
        let input = waveform(vec![-2.0, -0.5, 1.0, 4.0]);

        let absolute = AbsoluteValueTransform
            .apply(&input)
            .expect("absolute value should apply");
        assert_eq!(input.channels[0].samples, vec![-2.0, -0.5, 1.0, 4.0]);
        assert_close_vec(&absolute.channels[0].samples, &[2.0, 0.5, 1.0, 4.0]);
        assert_eq!(
            absolute.metadata.transform_history,
            vec!["absolute_value()"]
        );
        assert_eq!(absolute.metadata.transform_steps[0].name, "absolute_value");
        assert_eq!(
            absolute.metadata.transform_steps[0].phase_effect,
            TransformPhaseEffect::Nonlinear
        );

        let squared = SquareTransform.apply(&input).expect("square should apply");
        assert_close_vec(&squared.channels[0].samples, &[4.0, 0.25, 1.0, 16.0]);
        assert_eq!(squared.metadata.transform_history, vec!["square()"]);

        let rooted = SquareRootTransform
            .apply(&waveform(vec![0.0, 1.0, 4.0, 9.0]))
            .expect("square root should apply");
        assert_close_vec(&rooted.channels[0].samples, &[0.0, 1.0, 2.0, 3.0]);

        let logged = LogTransform { base: 10.0 }
            .apply(&waveform(vec![1.0, 10.0, 100.0, 1000.0]))
            .expect("log should apply");
        assert_close_vec(&logged.channels[0].samples, &[0.0, 1.0, 2.0, 3.0]);
        assert_eq!(logged.metadata.transform_history, vec!["log(base=10)"]);

        let expanded = ExpTransform { base: 10.0 }
            .apply(&waveform(vec![0.0, 1.0, 2.0, 3.0]))
            .expect("exp should apply");
        assert_close_vec(&expanded.channels[0].samples, &[1.0, 10.0, 100.0, 1000.0]);
        assert_eq!(expanded.metadata.transform_history, vec!["exp(base=10)"]);

        let tanh = TanhTransform
            .apply(&waveform(vec![-2.0, -0.5, 0.5, 2.0]))
            .expect("tanh should apply");
        assert_close_vec(
            &tanh.channels[0].samples,
            &[
                (-2.0_f64).tanh(),
                (-0.5_f64).tanh(),
                0.5_f64.tanh(),
                2.0_f64.tanh(),
            ],
        );
        assert_eq!(tanh.metadata.transform_history, vec!["tanh()"]);

        let sigmoid = SigmoidTransform
            .apply(&waveform(vec![-2.0, -0.5, 0.5, 2.0]))
            .expect("sigmoid should apply");
        assert_close_vec(
            &sigmoid.channels[0].samples,
            &[
                1.0 / (1.0 + 2.0_f64.exp()),
                1.0 / (1.0 + 0.5_f64.exp()),
                1.0 / (1.0 + (-0.5_f64).exp()),
                1.0 / (1.0 + (-2.0_f64).exp()),
            ],
        );
        assert_eq!(sigmoid.metadata.transform_history, vec!["sigmoid()"]);

        let soft_limited = SoftLimitTransform { limit_v: 2.0 }
            .apply(&input)
            .expect("soft limit should apply");
        assert_close_vec(
            &soft_limited.channels[0].samples,
            &[
                2.0 * (-1.0_f64).tanh(),
                2.0 * (-0.25_f64).tanh(),
                2.0 * 0.5_f64.tanh(),
                2.0 * 2.0_f64.tanh(),
            ],
        );
        assert_eq!(
            soft_limited.metadata.transform_history,
            vec!["soft_limit(limit_v=2)"]
        );
    }

    #[test]
    fn m27_normalize_modes_apply_expected_values() {
        let input = waveform(vec![1.0, 2.0, 3.0, 4.0]);

        let zero_to_one = NormalizeTransform {
            mode: NormalizeMode::ZeroToOne,
        }
        .apply(&input)
        .expect("zero-to-one normalize should apply");
        assert_close_vec(
            &zero_to_one.channels[0].samples,
            &[0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0],
        );
        let step = &zero_to_one.metadata.transform_steps[0];
        assert_eq!(step.name, "normalize");
        assert!(step.stateful);
        assert!(!step.causal);
        assert!(!step.streaming_supported);
        assert!(step.offline_only);
        assert_eq!(step.phase_effect, TransformPhaseEffect::None);

        let minus_one_to_one = NormalizeTransform {
            mode: NormalizeMode::MinusOneToOne,
        }
        .apply(&input)
        .expect("minus-one-to-one normalize should apply");
        assert_close_vec(
            &minus_one_to_one.channels[0].samples,
            &[-1.0, -1.0 / 3.0, 1.0 / 3.0, 1.0],
        );

        let z_score = NormalizeTransform {
            mode: NormalizeMode::ZScore,
        }
        .apply(&input)
        .expect("z-score normalize should apply");
        let std_dev = 1.25_f64.sqrt();
        assert_close_vec(
            &z_score.channels[0].samples,
            &[
                (1.0 - 2.5) / std_dev,
                (2.0 - 2.5) / std_dev,
                (3.0 - 2.5) / std_dev,
                (4.0 - 2.5) / std_dev,
            ],
        );

        let range = NormalizeTransform {
            mode: NormalizeMode::Range {
                input_min_v: 0.0,
                input_max_v: 10.0,
                output_min: -1.0,
                output_max: 1.0,
            },
        }
        .apply(&waveform(vec![0.0, 5.0, 10.0, 15.0]))
        .expect("configured range normalize should apply");
        assert_close_vec(&range.channels[0].samples, &[-1.0, 0.0, 1.0, 2.0]);
        assert_eq!(range.metadata.transform_steps[0].parameters.len(), 5);
    }

    #[test]
    fn m27_piecewise_and_polynomial_apply_expected_values() {
        let input = waveform(vec![-1.0, 0.0, 0.5, 1.0]);

        let piecewise = PiecewiseLinearTransform {
            points: vec![
                PiecewisePoint { x: 0.0, y: 0.0 },
                PiecewisePoint { x: 1.0, y: 10.0 },
            ],
        }
        .apply(&input)
        .expect("piecewise linear should apply");
        assert_close_vec(&piecewise.channels[0].samples, &[0.0, 0.0, 5.0, 10.0]);
        assert_eq!(
            piecewise.metadata.transform_history,
            vec!["piecewise_linear(points=2)"]
        );

        let polynomial = PolynomialTransform {
            coefficients: vec![1.0, 2.0, 3.0],
        }
        .apply(&input)
        .expect("polynomial should apply");
        assert_close_vec(&polynomial.channels[0].samples, &[2.0, 1.0, 2.75, 6.0]);
        assert_eq!(
            polynomial.metadata.transform_history,
            vec!["polynomial(coefficients=3)"]
        );
    }

    #[test]
    fn m27_transforms_reject_invalid_inputs() {
        assert!(matches!(
            SquareRootTransform.apply(&waveform(vec![-1.0, 0.0, 1.0, 2.0])),
            Err(WaveformError::InvalidWaveform { .. })
        ));
        assert!(matches!(
            LogTransform { base: 10.0 }.apply(&waveform(vec![0.0, 1.0, 2.0, 3.0])),
            Err(WaveformError::InvalidWaveform { .. })
        ));
        assert!(matches!(
            LogTransform { base: 1.0 }.apply(&waveform(vec![1.0, 2.0, 3.0, 4.0])),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            ExpTransform { base: 1.0 }.apply(&waveform(vec![1.0, 2.0, 3.0, 4.0])),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            NormalizeTransform {
                mode: NormalizeMode::ZeroToOne,
            }
            .apply(&waveform(vec![1.0, 1.0, 1.0, 1.0])),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            NormalizeTransform {
                mode: NormalizeMode::ZScore,
            }
            .apply(&waveform(vec![1.0, 1.0, 1.0, 1.0])),
            Err(WaveformError::InvalidWaveform { .. })
        ));
        assert!(matches!(
            SoftLimitTransform { limit_v: 0.0 }.apply(&waveform(vec![0.0, 1.0, 2.0, 3.0])),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            PiecewiseLinearTransform {
                points: vec![PiecewisePoint { x: 0.0, y: 0.0 }],
            }
            .apply(&waveform(vec![0.0, 1.0, 2.0, 3.0])),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            PiecewiseLinearTransform {
                points: vec![
                    PiecewisePoint { x: 1.0, y: 0.0 },
                    PiecewisePoint { x: 0.0, y: 1.0 }
                ],
            }
            .apply(&waveform(vec![0.0, 1.0, 2.0, 3.0])),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            PolynomialTransform {
                coefficients: Vec::new(),
            }
            .apply(&waveform(vec![0.0, 1.0, 2.0, 3.0])),
            Err(WaveformError::InvalidParameter { .. })
        ));
    }

    #[test]
    fn m28_causal_smoothing_filters_use_trailing_edges() {
        let input = waveform(vec![0.0, 6.0, 12.0, 6.0]);

        let weighted = WeightedMovingAverageFilter {
            weights: vec![1.0, 2.0],
        }
        .apply(&input)
        .expect("weighted moving average should apply");
        assert_close_vec(&weighted.channels[0].samples, &[0.0, 4.0, 10.0, 8.0]);
        assert_eq!(
            weighted.metadata.transform_history,
            vec!["weighted_moving_average(weights=2)"]
        );
        let weighted_step = &weighted.metadata.transform_steps[0];
        assert_eq!(weighted_step.name, "weighted_moving_average");
        assert!(weighted_step.causal);
        assert!(weighted_step.streaming_supported);
        assert_eq!(weighted_step.phase_effect, TransformPhaseEffect::Delay);

        let ema = ExponentialMovingAverageFilter { alpha: 0.5 }
            .apply(&input)
            .expect("EMA should apply");
        assert_close_vec(&ema.channels[0].samples, &[0.0, 3.0, 7.5, 6.75]);
        assert_eq!(
            ema.metadata.transform_history,
            vec!["exponential_moving_average(alpha=0.5)"]
        );
        assert_eq!(
            ema.metadata.transform_steps[0].category,
            TransformCategory::Stateful
        );
    }

    #[test]
    fn m28_centered_smoothing_filters_document_offline_edges() {
        let input = waveform(vec![1.0, 2.0, 3.0, 4.0]);

        let boxcar = BoxcarSmoothingFilter { window_samples: 3 }
            .apply(&input)
            .expect("boxcar smoothing should apply");
        assert_close_vec(&boxcar.channels[0].samples, &[1.5, 2.0, 3.0, 3.5]);
        let boxcar_step = &boxcar.metadata.transform_steps[0];
        assert_eq!(boxcar_step.name, "boxcar_smoothing");
        assert!(!boxcar_step.causal);
        assert!(!boxcar_step.streaming_supported);
        assert!(boxcar_step.offline_only);
        assert_eq!(boxcar_step.phase_effect, TransformPhaseEffect::None);

        let gaussian = GaussianSmoothingFilter {
            window_samples: 3,
            sigma_samples: 1.0,
        }
        .apply(&input)
        .expect("gaussian smoothing should apply");
        let edge_weight = (-0.5_f64).exp();
        assert_close_vec(
            &gaussian.channels[0].samples,
            &[
                (1.0 + 2.0 * edge_weight) / (1.0 + edge_weight),
                2.0,
                3.0,
                (4.0 + 3.0 * edge_weight) / (1.0 + edge_weight),
            ],
        );

        let median = CenteredMovingMedianFilter { window_samples: 3 }
            .apply(&waveform(vec![1.0, 100.0, 2.0, 3.0]))
            .expect("centered median should apply");
        assert_close_vec(&median.channels[0].samples, &[50.5, 2.0, 3.0, 2.5]);
        assert_eq!(
            median.metadata.transform_steps[0].phase_effect,
            TransformPhaseEffect::Nonlinear
        );
    }

    #[test]
    fn m28_savitzky_golay_reproduces_quadratic_shape() {
        let input = waveform_with_time(
            vec![0.0, 1.0, 2.0, 3.0, 4.0],
            vec![1.0, 4.0, 9.0, 16.0, 25.0],
        );

        let smoothed = SavitzkyGolayFilter {
            window_samples: 5,
            polynomial_order: 2,
        }
        .apply(&input)
        .expect("Savitzky-Golay smoothing should apply");

        assert_close_vec(&smoothed.channels[0].samples, &[1.0, 4.0, 9.0, 16.0, 25.0]);
        assert_eq!(
            smoothed.metadata.transform_history,
            vec!["savitzky_golay(window_samples=5,polynomial_order=2)"]
        );
        assert!(smoothed.metadata.transform_steps[0].offline_only);
    }

    #[test]
    fn m28_rolling_baseline_and_detrend_remove_drift() {
        let input = waveform_with_time(vec![0.0, 1.0, 2.0, 3.0], vec![1.0, 1.0, 10.0, 1.0]);

        let rolling_mean = RollingMeanBaselineTransform { window_samples: 2 }
            .apply(&input)
            .expect("rolling mean baseline should apply");
        assert_close_vec(&rolling_mean.channels[0].samples, &[0.0, 0.0, 4.5, -4.5]);

        let rolling_median = RollingMedianBaselineTransform { window_samples: 3 }
            .apply(&input)
            .expect("rolling median baseline should apply");
        assert_close_vec(&rolling_median.channels[0].samples, &[0.0, 0.0, 9.0, 0.0]);
        assert_eq!(
            rolling_median.metadata.transform_steps[0].phase_effect,
            TransformPhaseEffect::Nonlinear
        );

        let linear = LinearDetrendTransform
            .apply(&waveform_with_time(
                vec![0.0, 1.0, 2.0, 3.0],
                vec![2.0, 4.0, 6.0, 8.0],
            ))
            .expect("linear detrend should apply");
        assert_close_vec(&linear.channels[0].samples, &[0.0, 0.0, 0.0, 0.0]);
        assert!(linear.metadata.transform_steps[0].sample_rate_required);

        let polynomial = PolynomialDetrendTransform {
            polynomial_order: 2,
        }
        .apply(&waveform_with_time(
            vec![0.0, 1.0, 2.0, 3.0],
            vec![1.0, 6.0, 17.0, 34.0],
        ))
        .expect("polynomial detrend should apply");
        assert_close_vec(&polynomial.channels[0].samples, &[0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn m28_hampel_and_spike_removal_replace_outliers() {
        let input = waveform(vec![1.0, 1.0, 10.0, 1.0]);

        let hampel = HampelFilter {
            window_samples: 3,
            outlier_sigma: 3.0,
        }
        .apply(&input)
        .expect("Hampel filter should apply");
        assert_close_vec(&hampel.channels[0].samples, &[1.0, 1.0, 1.0, 1.0]);
        assert_eq!(
            hampel.metadata.transform_history,
            vec!["hampel_filter(window_samples=3,outlier_sigma=3)"]
        );

        let spike_removed = SpikeRemoveTransform {
            window_samples: 3,
            threshold_v: 2.0,
        }
        .apply(&input)
        .expect("spike removal should apply");
        assert_close_vec(&spike_removed.channels[0].samples, &[1.0, 1.0, 1.0, 1.0]);
        assert!(spike_removed.metadata.transform_steps[0].offline_only);
    }

    #[test]
    fn m28_transforms_reject_invalid_inputs() {
        assert!(matches!(
            WeightedMovingAverageFilter {
                weights: vec![1.0, 0.0],
            }
            .apply(&waveform(vec![0.0, 1.0, 2.0, 3.0])),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            ExponentialMovingAverageFilter { alpha: 0.0 }
                .apply(&waveform(vec![0.0, 1.0, 2.0, 3.0])),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            GaussianSmoothingFilter {
                window_samples: 3,
                sigma_samples: 0.0,
            }
            .apply(&waveform(vec![0.0, 1.0, 2.0, 3.0])),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            SavitzkyGolayFilter {
                window_samples: 4,
                polynomial_order: 2,
            }
            .apply(&waveform(vec![0.0, 1.0, 2.0, 3.0])),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            PolynomialDetrendTransform {
                polynomial_order: 6,
            }
            .apply(&waveform(vec![0.0, 1.0, 2.0, 3.0])),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            LinearDetrendTransform.apply(&waveform_with_time(
                vec![0.0, 0.0, 1.0, 2.0],
                vec![0.0, 1.0, 2.0, 3.0],
            )),
            Err(WaveformError::InvalidWaveform { .. })
        ));
        assert!(matches!(
            HampelFilter {
                window_samples: 3,
                outlier_sigma: 0.0,
            }
            .apply(&waveform(vec![0.0, 1.0, 2.0, 3.0])),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            SpikeRemoveTransform {
                window_samples: 3,
                threshold_v: 0.0,
            }
            .apply(&waveform(vec![0.0, 1.0, 2.0, 3.0])),
            Err(WaveformError::InvalidParameter { .. })
        ));
    }

    #[test]
    fn m29_fir_iir_and_comb_filters_apply_known_answer_outputs() {
        let input = waveform_with_time(vec![0.0, 0.001, 0.002, 0.003], vec![0.0, 2.0, 4.0, 6.0]);

        let fir = FirFilter {
            coefficients: vec![0.5, 0.5],
        }
        .apply(&input)
        .expect("FIR should apply");
        assert_close_vec(&fir.channels[0].samples, &[0.0, 1.0, 3.0, 5.0]);
        assert_eq!(fir.metadata.transform_steps[0].name, "fir_filter");
        assert_eq!(
            fir.metadata.transform_steps[0].phase_effect,
            TransformPhaseEffect::Delay
        );

        let zero_phase_fir = ZeroPhaseFirFilter {
            coefficients: vec![0.5, 0.5],
        }
        .apply(&waveform_with_time(
            vec![0.0, 0.001, 0.002, 0.003],
            vec![0.0, 0.0, 10.0, 0.0],
        ))
        .expect("zero-phase FIR should apply");
        assert_close_vec(&zero_phase_fir.channels[0].samples, &[0.0, 2.5, 5.0, 2.5]);
        assert!(zero_phase_fir.metadata.transform_steps[0].offline_only);
        assert!(!zero_phase_fir.metadata.transform_steps[0].causal);
        assert_eq!(
            zero_phase_fir.metadata.transform_steps[0].phase_effect,
            TransformPhaseEffect::None
        );

        let identity = BiquadCoefficients {
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
        };
        let iir = IirBiquadFilter {
            coefficients: identity,
        }
        .apply(&input)
        .expect("identity biquad should apply");
        assert_close_vec(&iir.channels[0].samples, &[0.0, 2.0, 4.0, 6.0]);

        let zero_phase_iir = ZeroPhaseIirBiquadFilter {
            coefficients: identity,
        }
        .apply(&input)
        .expect("zero-phase identity biquad should apply");
        assert_close_vec(&zero_phase_iir.channels[0].samples, &[0.0, 2.0, 4.0, 6.0]);
        assert!(zero_phase_iir.metadata.transform_steps[0].offline_only);

        let comb = CombFilter {
            delay_samples: 2,
            feedback_gain: -1.0,
        }
        .apply(&waveform_with_time(
            vec![0.0, 0.001, 0.002, 0.003],
            vec![1.0, 1.0, 1.0, 1.0],
        ))
        .expect("comb filter should apply");
        assert_close_vec(&comb.channels[0].samples, &[1.0, 1.0, 0.0, 0.0]);
    }

    #[test]
    fn m29_designed_filters_have_expected_generated_frequency_response() {
        let sample_rate_hz = 1000.0;

        let band_pass = rbj_biquad(BiquadDesignKind::BandPass, sample_rate_hz, 100.0, 5.0).unwrap();
        assert!(biquad_response_magnitude(band_pass, sample_rate_hz, 100.0) > 0.9);
        assert!(biquad_response_magnitude(band_pass, sample_rate_hz, 20.0) < 0.25);

        let notch = rbj_biquad(BiquadDesignKind::Notch, sample_rate_hz, 60.0, 30.0).unwrap();
        assert!(biquad_response_magnitude(notch, sample_rate_hz, 60.0) < 1.0e-9);
        assert!(biquad_response_magnitude(notch, sample_rate_hz, 10.0) > 0.95);

        let butter_low = rbj_biquad(
            BiquadDesignKind::LowPass,
            sample_rate_hz,
            100.0,
            std::f64::consts::FRAC_1_SQRT_2,
        )
        .unwrap();
        assert_close_value(
            biquad_response_magnitude(butter_low, sample_rate_hz, 100.0),
            std::f64::consts::FRAC_1_SQRT_2,
            1.0e-9,
        );
        assert!(biquad_response_magnitude(butter_low, sample_rate_hz, 300.0) < 0.15);

        let butter_high = rbj_biquad(
            BiquadDesignKind::HighPass,
            sample_rate_hz,
            100.0,
            std::f64::consts::FRAC_1_SQRT_2,
        )
        .unwrap();
        assert_close_value(
            biquad_response_magnitude(butter_high, sample_rate_hz, 100.0),
            std::f64::consts::FRAC_1_SQRT_2,
            1.0e-9,
        );
        assert!(biquad_response_magnitude(butter_high, sample_rate_hz, 20.0) < 0.05);

        let cheb1 = chebyshev1_low_pass_biquad(sample_rate_hz, 100.0, 1.0).unwrap();
        assert_close_value(
            biquad_response_magnitude(cheb1, sample_rate_hz, 0.0),
            1.0,
            1.0e-9,
        );
        assert!(biquad_response_magnitude(cheb1, sample_rate_hz, 300.0) < 0.2);

        let cheb2 = chebyshev2_low_pass_biquad(sample_rate_hz, 100.0, 40.0).unwrap();
        assert_close_value(
            biquad_response_magnitude(cheb2, sample_rate_hz, 0.0),
            1.0,
            1.0e-9,
        );
        assert!(biquad_response_magnitude(cheb2, sample_rate_hz, 300.0) < 0.2);

        let bessel = bessel_low_pass_biquad(sample_rate_hz, 100.0).unwrap();
        assert_close_value(
            biquad_response_magnitude(bessel, sample_rate_hz, 0.0),
            1.0,
            1.0e-9,
        );
        assert!(biquad_response_magnitude(bessel, sample_rate_hz, 300.0) < 0.3);
    }

    #[test]
    fn m29_standard_frequency_filter_metadata_matches_catalog() {
        let waveform = sampled_waveform(64, 1000.0, |index, timestamp| {
            (TAU * 20.0 * timestamp).sin() + index as f64 * 1.0e-4
        });
        let identity = BiquadCoefficients {
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
        };
        let filters = [
            FilterStep::FirFilter(FirFilter {
                coefficients: vec![0.25, 0.5, 0.25],
            }),
            FilterStep::ZeroPhaseFirFilter(ZeroPhaseFirFilter {
                coefficients: vec![0.25, 0.5, 0.25],
            }),
            FilterStep::IirBiquad(IirBiquadFilter {
                coefficients: identity,
            }),
            FilterStep::ZeroPhaseIirBiquad(ZeroPhaseIirBiquadFilter {
                coefficients: identity,
            }),
            FilterStep::HighPass(HighPassFilter { cutoff_hz: 5.0 }),
            FilterStep::BandPass(BandPassFilter {
                center_hz: 20.0,
                q: 2.0,
            }),
            FilterStep::BandStop(BandStopFilter {
                center_hz: 60.0,
                q: 5.0,
            }),
            FilterStep::Notch(NotchFilter {
                center_hz: 60.0,
                q: 30.0,
            }),
            FilterStep::CombFilter(CombFilter {
                delay_samples: 2,
                feedback_gain: -0.5,
            }),
            FilterStep::ButterworthLowPass(ButterworthLowPassFilter { cutoff_hz: 120.0 }),
            FilterStep::ButterworthHighPass(ButterworthHighPassFilter { cutoff_hz: 5.0 }),
            FilterStep::Chebyshev1LowPass(Chebyshev1LowPassFilter {
                cutoff_hz: 150.0,
                ripple_db: 1.0,
            }),
            FilterStep::Chebyshev2LowPass(Chebyshev2LowPassFilter {
                cutoff_hz: 150.0,
                stopband_attenuation_db: 40.0,
            }),
            FilterStep::BesselLowPass(BesselLowPassFilter { cutoff_hz: 150.0 }),
        ];

        let derived =
            apply_filter_chain(&waveform, &filters).expect("M29 filter chain should evaluate");
        let names = derived
            .metadata
            .transform_steps
            .iter()
            .map(|step| step.name.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            names,
            vec![
                "fir_filter",
                "zero_phase_fir_filter",
                "iir_biquad",
                "zero_phase_iir_biquad",
                "high_pass",
                "band_pass",
                "band_stop",
                "notch",
                "comb_filter",
                "butterworth_low_pass",
                "butterworth_high_pass",
                "chebyshev1_low_pass",
                "chebyshev2_low_pass",
                "bessel_low_pass",
            ]
        );
        for step in &derived.metadata.transform_steps {
            let entry =
                transform_catalog_entry(&step.name).expect("implemented step should be cataloged");
            assert!(
                entry.matches_step_metadata(step),
                "metadata for `{}` does not match catalog entry: {step:?} vs {entry:?}",
                step.name
            );
        }
    }

    #[test]
    fn m29_frequency_filters_reject_invalid_inputs() {
        let waveform = sampled_waveform(8, 1000.0, |index, _| index as f64);
        assert!(matches!(
            IirBiquadFilter {
                coefficients: BiquadCoefficients {
                    b0: 1.0,
                    b1: 0.0,
                    b2: 0.0,
                    a1: 0.0,
                    a2: 1.2,
                },
            }
            .apply(&waveform),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            BandPassFilter {
                center_hz: 500.0,
                q: 2.0,
            }
            .apply(&waveform),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            BandPassFilter {
                center_hz: 100.0,
                q: 0.0,
            }
            .apply(&waveform),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            CombFilter {
                delay_samples: 0,
                feedback_gain: 0.5,
            }
            .apply(&waveform),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            Chebyshev1LowPassFilter {
                cutoff_hz: 100.0,
                ripple_db: 0.0,
            }
            .apply(&waveform),
            Err(WaveformError::InvalidParameter { .. })
        ));

        let nonuniform =
            waveform_with_time(vec![0.0, 0.001, 0.003, 0.004], vec![0.0, 1.0, 2.0, 3.0]);
        assert!(matches!(
            NotchFilter {
                center_hz: 60.0,
                q: 30.0,
            }
            .apply(&nonuniform),
            Err(WaveformError::InvalidWaveform { .. })
        ));
    }

    #[test]
    fn m30_resampling_timing_transforms_apply_known_answer_outputs() {
        let ramp = waveform_with_time(vec![0.0, 1.0, 2.0, 3.0, 4.0], vec![0.0, 1.0, 2.0, 3.0, 4.0]);

        let downsampled = DownsampleTransform { factor: 2 }
            .apply(&ramp)
            .expect("downsample should apply");
        assert_close_vec(&downsampled.time, &[0.0, 2.0, 4.0]);
        assert_close_vec(&downsampled.channels[0].samples, &[0.0, 2.0, 4.0]);

        let constant =
            waveform_with_time(vec![0.0, 1.0, 2.0, 3.0, 4.0], vec![1.0, 1.0, 1.0, 1.0, 1.0]);
        let decimated = DecimateTransform {
            factor: 2,
            cutoff_hz: 0.2,
        }
        .apply(&constant)
        .expect("decimation should apply");
        assert_close_vec(&decimated.time, &[0.0, 2.0, 4.0]);
        assert_close_vec(&decimated.channels[0].samples, &[1.0, 1.0, 1.0]);

        let upsampled = UpsampleTransform { factor: 2 }
            .apply(&ramp)
            .expect("upsample should apply");
        assert_close_vec(
            &upsampled.time,
            &[0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0],
        );
        assert_close_vec(&upsampled.channels[0].samples, &upsampled.time);

        let interpolated = InterpolateTransform {
            sample_interval_s: 0.5,
        }
        .apply(&ramp)
        .expect("interpolation should apply");
        assert_close_vec(&interpolated.channels[0].samples, &interpolated.time);

        let rational = RationalResampleTransform {
            upsample_factor: 3,
            downsample_factor: 2,
        }
        .apply(&ramp)
        .expect("rational resampling should apply");
        assert_eq!(rational.time.len(), 7);
        assert_close_value(*rational.time.last().unwrap(), 4.0, 1.0e-9);
        assert_close_vec(&rational.channels[0].samples, &rational.time);

        for filter in [
            FilterStep::SampleAndHold(SampleAndHoldTransform {
                sample_interval_s: 0.5,
            }),
            FilterStep::ZeroOrderHold(ZeroOrderHoldTransform {
                sample_interval_s: 0.5,
            }),
        ] {
            let held = filter.apply(&ramp).expect("hold transform should apply");
            assert_close_vec(
                &held.channels[0].samples,
                &[0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 3.0, 3.0, 4.0],
            );
        }

        let first_order = FirstOrderHoldTransform {
            sample_interval_s: 0.5,
        }
        .apply(&ramp)
        .expect("first-order hold should apply");
        assert_close_vec(&first_order.channels[0].samples, &first_order.time);

        let delayed = FractionalDelayTransform { delay_s: 0.5 }
            .apply(&ramp)
            .expect("fractional delay should apply");
        assert_close_vec(&delayed.channels[0].samples, &[0.0, 0.5, 1.5, 2.5, 3.5]);

        let jittered = waveform_with_time(vec![0.0, 1.1, 2.0, 3.05], vec![0.0, 1.1, 2.0, 3.05]);
        let jitter_corrected = JitterCorrectionTransform {
            sample_interval_s: 1.0,
        }
        .apply(&jittered)
        .expect("jitter correction should apply");
        assert_close_vec(&jitter_corrected.time, &[0.0, 1.0, 2.0, 3.0]);
        assert_close_vec(&jitter_corrected.channels[0].samples, &[0.0, 1.0, 2.0, 3.0]);

        let drifted = waveform_with_time(vec![0.0, 1.05, 2.1, 3.15], vec![0.0, 1.05, 2.1, 3.15]);
        let drift_corrected = ClockDriftCorrectionTransform {
            sample_interval_s: 1.0,
        }
        .apply(&drifted)
        .expect("clock drift correction should apply");
        assert_close_vec(&drift_corrected.time, &[0.0, 1.0, 2.0, 3.0]);
        assert_close_vec(&drift_corrected.channels[0].samples, &[0.0, 1.0, 2.0, 3.0]);

        let alignment_input = waveform_with_channels(
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
            vec![
                Channel::new(
                    "reference_v",
                    Unit::volts(),
                    vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
                ),
                Channel::new(
                    "target_v",
                    Unit::volts(),
                    vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
                ),
            ],
        );
        let aligned = CrossCorrelationDelayTransform {
            reference_channel: "reference_v".to_string(),
            target_channel: "target_v".to_string(),
            max_lag_samples: 2,
        }
        .apply(&alignment_input)
        .expect("cross-correlation alignment should apply");
        assert_close_vec(
            &aligned.channel("target_v").unwrap().samples,
            &[0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        );
        let estimated_lag = aligned.metadata.transform_steps[0]
            .parameters
            .iter()
            .find(|parameter| parameter.name == "estimated_lag_samples")
            .expect("estimated lag should be recorded");
        assert_eq!(estimated_lag.value, TransformParameterValue::Float(1.0));
    }

    #[test]
    fn m30_resampling_timing_metadata_matches_catalog() {
        let time = (0..32).map(|index| index as f64 / 1000.0).collect();
        let waveform = Waveform::new(
            time,
            vec![
                Channel::new(
                    "reference_v",
                    Unit::volts(),
                    (0..32)
                        .map(|index| if index == 10 { 1.0 } else { 0.0 })
                        .collect(),
                ),
                Channel::new(
                    "target_v",
                    Unit::volts(),
                    (0..32)
                        .map(|index| if index == 11 { 1.0 } else { 0.0 })
                        .collect(),
                ),
            ],
        )
        .expect("waveform should be valid");
        let filters = vec![
            FilterStep::Resample(ResampleTransform {
                sample_interval_s: 0.001,
            }),
            FilterStep::Downsample(DownsampleTransform { factor: 2 }),
            FilterStep::Decimate(DecimateTransform {
                factor: 2,
                cutoff_hz: 100.0,
            }),
            FilterStep::Upsample(UpsampleTransform { factor: 2 }),
            FilterStep::Interpolate(InterpolateTransform {
                sample_interval_s: 0.001,
            }),
            FilterStep::RationalResample(RationalResampleTransform {
                upsample_factor: 3,
                downsample_factor: 2,
            }),
            FilterStep::SampleAndHold(SampleAndHoldTransform {
                sample_interval_s: 0.001,
            }),
            FilterStep::ZeroOrderHold(ZeroOrderHoldTransform {
                sample_interval_s: 0.001,
            }),
            FilterStep::FirstOrderHold(FirstOrderHoldTransform {
                sample_interval_s: 0.001,
            }),
            FilterStep::FractionalDelay(FractionalDelayTransform { delay_s: 0.0005 }),
            FilterStep::CrossCorrelationDelay(CrossCorrelationDelayTransform {
                reference_channel: "reference_v".to_string(),
                target_channel: "target_v".to_string(),
                max_lag_samples: 2,
            }),
            FilterStep::JitterCorrection(JitterCorrectionTransform {
                sample_interval_s: 0.001,
            }),
            FilterStep::ClockDriftCorrection(ClockDriftCorrectionTransform {
                sample_interval_s: 0.001,
            }),
        ];

        let names = filters.iter().map(Filter::name).collect::<Vec<_>>();
        assert_eq!(
            names,
            vec![
                "resample",
                "downsample",
                "decimate",
                "upsample",
                "interpolate",
                "rational_resample",
                "sample_and_hold",
                "zero_order_hold",
                "first_order_hold",
                "fractional_delay",
                "cross_correlation_delay",
                "jitter_correction",
                "clock_drift_correction",
            ]
        );

        for filter in filters {
            let derived = filter
                .apply(&waveform)
                .expect("M30 transform should evaluate");
            let step = &derived.metadata.transform_steps[0];
            let entry =
                transform_catalog_entry(&step.name).expect("implemented step should be cataloged");
            assert!(
                entry.matches_step_metadata(step),
                "metadata for `{}` does not match catalog entry: {step:?} vs {entry:?}",
                step.name
            );
        }
    }

    #[test]
    fn m30_resampling_timing_transforms_reject_invalid_inputs() {
        let waveform = sampled_waveform(8, 1000.0, |index, _| index as f64);
        assert!(matches!(
            DownsampleTransform { factor: 1 }.apply(&waveform),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            DecimateTransform {
                factor: 2,
                cutoff_hz: 400.0,
            }
            .apply(&waveform),
            Err(WaveformError::InvalidParameter { .. })
        ));
        let nonuniform =
            waveform_with_time(vec![0.0, 0.001, 0.003, 0.004], vec![0.0, 1.0, 2.0, 3.0]);
        assert!(matches!(
            UpsampleTransform { factor: 2 }.apply(&nonuniform),
            Err(WaveformError::InvalidWaveform { .. })
        ));
        assert!(matches!(
            SampleAndHoldTransform {
                sample_interval_s: 0.0,
            }
            .apply(&waveform),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            ClockDriftCorrectionTransform {
                sample_interval_s: 0.0,
            }
            .apply(&waveform),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            CrossCorrelationDelayTransform {
                reference_channel: "input_v".to_string(),
                target_channel: "missing_v".to_string(),
                max_lag_samples: 2,
            }
            .apply(&waveform),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            CrossCorrelationDelayTransform {
                reference_channel: "input_v".to_string(),
                target_channel: "input_v".to_string(),
                max_lag_samples: 8,
            }
            .apply(&waveform),
            Err(WaveformError::InvalidParameter { .. })
        ));
    }

    #[test]
    fn m31_envelope_energy_calculus_filters_apply_known_answer_outputs() {
        let signed = waveform(vec![-2.0, -1.0, 0.0, 3.0]);
        let half_wave = HalfWaveRectifyTransform
            .apply(&signed)
            .expect("half-wave rectification should apply");
        assert_close_vec(&half_wave.channels[0].samples, &[0.0, 0.0, 0.0, 3.0]);

        let full_wave = FullWaveRectifyTransform
            .apply(&signed)
            .expect("full-wave rectification should apply");
        assert_close_vec(&full_wave.channels[0].samples, &[2.0, 1.0, 0.0, 3.0]);

        let envelope = EnvelopeTransform { alpha: 0.5 }
            .apply(&signed)
            .expect("envelope should apply");
        assert_close_vec(&envelope.channels[0].samples, &[2.0, 1.5, 0.75, 1.875]);

        let rms_input = waveform(vec![3.0, 4.0, 0.0, 0.0]);
        let moving_rms = MovingRmsTransform { window_samples: 2 }
            .apply(&rms_input)
            .expect("moving RMS should apply");
        assert_close_vec(
            &moving_rms.channels[0].samples,
            &[3.0, 12.5_f64.sqrt(), 8.0_f64.sqrt(), 0.0],
        );

        let peak_hold = PeakHoldTransform
            .apply(&waveform(vec![-1.0, 2.0, -3.0, 1.0]))
            .expect("peak hold should apply");
        assert_close_vec(&peak_hold.channels[0].samples, &[1.0, 2.0, 3.0, 3.0]);

        let ramp = waveform(vec![0.0, 1.0, 3.0, 6.0]);
        let first_derivative = FirstDerivativeTransform
            .apply(&ramp)
            .expect("first derivative should apply");
        assert_close_vec(&first_derivative.channels[0].samples, &[0.0, 1.0, 2.0, 3.0]);

        let second_derivative = SecondDerivativeTransform
            .apply(&ramp)
            .expect("second derivative should apply");
        assert_close_vec(
            &second_derivative.channels[0].samples,
            &[0.0, 1.0, 1.0, 1.0],
        );

        for filter in [
            FilterStep::Integral(IntegralTransform),
            FilterStep::CumulativeIntegral(CumulativeIntegralTransform),
        ] {
            let integrated = filter.apply(&ramp).expect("integral should apply");
            assert_close_vec(&integrated.channels[0].samples, &[0.0, 0.5, 2.5, 7.0]);
        }

        let leaky = LeakyIntegratorTransform {
            time_constant_s: 2.0,
        }
        .apply(&ramp)
        .expect("leaky integrator should apply");
        assert_close_value(leaky.channels[0].samples[0], 0.0, 1.0e-12);
        assert_close_value(leaky.channels[0].samples[1], 1.0, 1.0e-12);
        let expected_third = 1.0 * (-0.5_f64).exp() + 3.0;
        let expected_fourth = expected_third * (-0.5_f64).exp() + 6.0;
        assert_close_value(leaky.channels[0].samples[2], expected_third, 1.0e-12);
        assert_close_value(leaky.channels[0].samples[3], expected_fourth, 1.0e-12);

        let slopes = SlopeDetectionTransform {
            threshold_per_s: 1.5,
        }
        .apply(&ramp)
        .expect("slope detection should apply");
        assert_close_vec(&slopes.channels[0].samples, &[0.0, 0.0, 1.0, 1.0]);
    }

    #[test]
    fn m31_envelope_energy_calculus_metadata_matches_catalog() {
        let waveform = waveform(vec![0.0, 1.0, 3.0, 6.0]);
        let filters = vec![
            FilterStep::HalfWaveRectify(HalfWaveRectifyTransform),
            FilterStep::FullWaveRectify(FullWaveRectifyTransform),
            FilterStep::Envelope(EnvelopeTransform { alpha: 0.5 }),
            FilterStep::MovingRms(MovingRmsTransform { window_samples: 2 }),
            FilterStep::PeakHold(PeakHoldTransform),
            FilterStep::FirstDerivative(FirstDerivativeTransform),
            FilterStep::SecondDerivative(SecondDerivativeTransform),
            FilterStep::Integral(IntegralTransform),
            FilterStep::CumulativeIntegral(CumulativeIntegralTransform),
            FilterStep::LeakyIntegrator(LeakyIntegratorTransform {
                time_constant_s: 2.0,
            }),
            FilterStep::SlopeDetection(SlopeDetectionTransform {
                threshold_per_s: 1.5,
            }),
        ];

        let names = filters.iter().map(Filter::name).collect::<Vec<_>>();
        assert_eq!(
            names,
            vec![
                "half_wave_rectify",
                "full_wave_rectify",
                "envelope",
                "moving_rms",
                "peak_hold",
                "first_derivative",
                "second_derivative",
                "integral",
                "cumulative_integral",
                "leaky_integrator",
                "slope_detection",
            ]
        );

        for filter in filters {
            let derived = filter
                .apply(&waveform)
                .expect("M31 transform should evaluate");
            let step = &derived.metadata.transform_steps[0];
            let entry =
                transform_catalog_entry(&step.name).expect("implemented step should be cataloged");
            assert!(
                entry.matches_step_metadata(step),
                "metadata for `{}` does not match catalog entry: {step:?} vs {entry:?}",
                step.name
            );
        }
    }

    #[test]
    fn m31_envelope_energy_calculus_filters_reject_invalid_inputs() {
        let waveform = waveform(vec![0.0, 1.0, 2.0, 3.0]);
        assert!(matches!(
            EnvelopeTransform { alpha: -0.1 }.apply(&waveform),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            MovingRmsTransform { window_samples: 0 }.apply(&waveform),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            LeakyIntegratorTransform {
                time_constant_s: 0.0,
            }
            .apply(&waveform),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            SlopeDetectionTransform {
                threshold_per_s: 0.0,
            }
            .apply(&waveform),
            Err(WaveformError::InvalidParameter { .. })
        ));

        let nonmonotonic = waveform_with_time(vec![0.0, 1.0, 1.0, 2.0], vec![0.0, 1.0, 2.0, 3.0]);
        assert!(matches!(
            FirstDerivativeTransform.apply(&nonmonotonic),
            Err(WaveformError::InvalidWaveform { .. })
        ));
    }

    #[test]
    fn m32_statistics_filters_apply_known_answer_outputs() {
        let input = waveform(vec![1.0, 2.0, 4.0, 7.0]);

        let rolling_mean = RollingMeanTransform { window_samples: 3 }
            .apply(&input)
            .expect("rolling mean should apply");
        assert_close_vec(
            &rolling_mean.channels[0].samples,
            &[1.0, 1.5, 7.0 / 3.0, 13.0 / 3.0],
        );

        let rolling_variance = RollingVarianceTransform { window_samples: 3 }
            .apply(&input)
            .expect("rolling variance should apply");
        assert_close_vec(
            &rolling_variance.channels[0].samples,
            &[0.0, 0.25, 14.0 / 9.0, 38.0 / 9.0],
        );

        let rolling_stddev = RollingStdDevTransform { window_samples: 3 }
            .apply(&input)
            .expect("rolling stddev should apply");
        assert_close_vec(
            &rolling_stddev.channels[0].samples,
            &[0.0, 0.5, (14.0_f64 / 9.0).sqrt(), (38.0_f64 / 9.0).sqrt()],
        );

        let rolling_min = RollingMinTransform { window_samples: 3 }
            .apply(&input)
            .expect("rolling min should apply");
        assert_close_vec(&rolling_min.channels[0].samples, &[1.0, 1.0, 1.0, 2.0]);

        let rolling_max = RollingMaxTransform { window_samples: 3 }
            .apply(&input)
            .expect("rolling max should apply");
        assert_close_vec(&rolling_max.channels[0].samples, &[1.0, 2.0, 4.0, 7.0]);

        let z_score = ZScoreTransform.apply(&input).expect("z-score should apply");
        let std_dev = 5.25_f64.sqrt();
        assert_close_vec(
            &z_score.channels[0].samples,
            &[
                (1.0 - 3.5) / std_dev,
                (2.0 - 3.5) / std_dev,
                (4.0 - 3.5) / std_dev,
                (7.0 - 3.5) / std_dev,
            ],
        );

        let outliers = OutlierDetectionTransform {
            threshold_sigma: 1.0,
        }
        .apply(&input)
        .expect("outlier detection should apply");
        assert_close_vec(&outliers.channels[0].samples, &[1.0, 0.0, 0.0, 1.0]);

        let clipped = QuantileClipTransform {
            lower_quantile: 0.25,
            upper_quantile: 0.75,
        }
        .apply(&input)
        .expect("quantile clipping should apply");
        assert_close_vec(&clipped.channels[0].samples, &[1.75, 2.0, 4.0, 4.75]);
    }

    #[test]
    fn m32_statistics_filter_metadata_matches_catalog() {
        let waveform = waveform(vec![1.0, 2.0, 4.0, 7.0]);
        let filters = vec![
            FilterStep::RollingMean(RollingMeanTransform { window_samples: 3 }),
            FilterStep::RollingVariance(RollingVarianceTransform { window_samples: 3 }),
            FilterStep::RollingStdDev(RollingStdDevTransform { window_samples: 3 }),
            FilterStep::RollingMin(RollingMinTransform { window_samples: 3 }),
            FilterStep::RollingMax(RollingMaxTransform { window_samples: 3 }),
            FilterStep::ZScore(ZScoreTransform),
            FilterStep::OutlierDetection(OutlierDetectionTransform {
                threshold_sigma: 1.0,
            }),
            FilterStep::QuantileClip(QuantileClipTransform {
                lower_quantile: 0.25,
                upper_quantile: 0.75,
            }),
        ];

        let names = filters.iter().map(Filter::name).collect::<Vec<_>>();
        assert_eq!(
            names,
            vec![
                "rolling_mean",
                "rolling_variance",
                "rolling_stddev",
                "rolling_min",
                "rolling_max",
                "z_score",
                "outlier_detection",
                "quantile_clip",
            ]
        );

        for filter in filters {
            let derived = filter
                .apply(&waveform)
                .expect("M32 transform should evaluate");
            let step = &derived.metadata.transform_steps[0];
            let entry =
                transform_catalog_entry(&step.name).expect("implemented step should be cataloged");
            assert!(
                entry.matches_step_metadata(step),
                "metadata for `{}` does not match catalog entry: {step:?} vs {entry:?}",
                step.name
            );
        }
    }

    #[test]
    fn m32_statistics_filters_reject_invalid_inputs() {
        let input = waveform(vec![1.0, 2.0, 4.0, 7.0]);
        assert!(matches!(
            RollingMeanTransform { window_samples: 0 }.apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            OutlierDetectionTransform {
                threshold_sigma: 0.0,
            }
            .apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            QuantileClipTransform {
                lower_quantile: 0.75,
                upper_quantile: 0.25,
            }
            .apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            ZScoreTransform.apply(&waveform(vec![1.0, 1.0, 1.0, 1.0])),
            Err(WaveformError::InvalidWaveform { .. })
        ));
    }

    #[test]
    fn m34_fault_injection_and_adc_simulation_filters_apply_known_outputs() {
        let input = waveform_with_time(vec![0.0, 0.25, 0.5, 0.75], vec![0.0, 1.0, 2.0, 3.0]);

        let periodic = PeriodicInterferenceTransform {
            kind: PeriodicInterferenceKind::Periodic,
            amplitude_v: 1.0,
            frequency_hz: 1.0,
            phase_rad: 0.0,
        }
        .apply(&input)
        .expect("periodic interference should apply");
        assert_close_vec(&periodic.channels[0].samples, &[0.0, 2.0, 2.0, 2.0]);
        assert_eq!(
            periodic.metadata.transform_steps[0].category,
            TransformCategory::FaultInjection
        );

        let thermal = DriftFaultTransform {
            kind: DriftFaultKind::Thermal,
            amplitude_v: 0.0,
            drift_rate_v_per_s: 2.0,
            interval_samples: 1,
            seed: 0,
        }
        .apply(&input)
        .expect("thermal drift should apply");
        assert_close_vec(&thermal.channels[0].samples, &[0.0, 1.5, 3.0, 4.5]);

        let saturated = SampleFaultTransform {
            kind: SampleFaultKind::Saturation,
            probability: 0.0,
            fault_value_v: 0.0,
            min_v: 0.5,
            max_v: 2.5,
            start_index: 0,
            duration_samples: 1,
            seed: 0,
        }
        .apply(&input)
        .expect("saturation fault should apply");
        assert_close_vec(&saturated.channels[0].samples, &[0.5, 1.0, 2.0, 2.5]);

        let stuck = SampleFaultTransform {
            kind: SampleFaultKind::StuckAt,
            probability: 0.0,
            fault_value_v: 9.0,
            min_v: 0.0,
            max_v: 0.0,
            start_index: 1,
            duration_samples: 2,
            seed: 0,
        }
        .apply(&input)
        .expect("stuck-at fault should apply");
        assert_close_vec(&stuck.channels[0].samples, &[0.0, 9.0, 9.0, 3.0]);

        let quantizer_input = waveform(vec![0.2, 0.7, 1.2, 1.7]);
        let midrise = SimulationQuantizerTransform {
            kind: SimulationQuantizerKind::MidRise,
            lsb_v: 1.0,
            min_v: 0.0,
            max_v: 0.0,
        }
        .apply(&quantizer_input)
        .expect("mid-rise quantizer should apply");
        assert_close_vec(&midrise.channels[0].samples, &[0.5, 0.5, 1.5, 1.5]);

        let missing_code = AdcCodeDefectTransform {
            kind: AdcCodeDefectKind::MissingCode,
            bits: 2,
            min_v: 0.0,
            max_v: 3.0,
            missing_code: 1,
            coefficients: Vec::new(),
        }
        .apply(&input)
        .expect("missing-code simulation should apply");
        assert_close_vec(&missing_code.channels[0].samples, &[0.0, 2.0, 2.0, 3.0]);

        let gain_error = GainOffsetErrorTransform {
            kind: GainOffsetErrorKind::Gain,
            gain_error: 0.1,
            offset_error_v: 0.0,
        }
        .apply(&input)
        .expect("gain-error simulation should apply");
        assert_close_vec(&gain_error.channels[0].samples, &[0.0, 1.1, 2.2, 3.3]);
    }

    #[test]
    fn m34_seeded_random_faults_are_deterministic_and_record_metadata() {
        let input = waveform_with_time(vec![0.0, 0.1, 0.2, 0.3], vec![1.0, 1.0, 1.0, 1.0]);
        let filters = vec![
            FilterStep::NoiseInjection(NoiseInjectionTransform {
                kind: NoiseKind::White,
                amplitude_v: 0.1,
                min_v: 0.0,
                max_v: 0.0,
                probability: 0.0,
                seed: 42,
            }),
            FilterStep::NoiseInjection(NoiseInjectionTransform {
                kind: NoiseKind::Gaussian,
                amplitude_v: 0.1,
                min_v: 0.0,
                max_v: 0.0,
                probability: 0.0,
                seed: 43,
            }),
            FilterStep::NoiseInjection(NoiseInjectionTransform {
                kind: NoiseKind::SaltPepper,
                amplitude_v: 0.0,
                min_v: -1.0,
                max_v: 1.0,
                probability: 1.0,
                seed: 44,
            }),
            FilterStep::DriftFault(DriftFaultTransform {
                kind: DriftFaultKind::RandomWalk,
                amplitude_v: 0.05,
                drift_rate_v_per_s: 0.0,
                interval_samples: 1,
                seed: 45,
            }),
            FilterStep::SampleFault(SampleFaultTransform {
                kind: SampleFaultKind::Dropout,
                probability: 1.0,
                fault_value_v: 0.0,
                min_v: 0.0,
                max_v: 0.0,
                start_index: 0,
                duration_samples: 1,
                seed: 46,
            }),
            FilterStep::Dither(DitherTransform {
                lsb_v: 0.25,
                seed: 47,
            }),
        ];

        for filter in filters {
            let first = filter
                .apply(&input)
                .expect("M34 seeded filter should apply");
            let second = filter
                .apply(&input)
                .expect("M34 seeded filter should repeat");
            assert_eq!(first.channels[0].samples, second.channels[0].samples);
            let step = &first.metadata.transform_steps[0];
            assert!(step
                .parameters
                .iter()
                .any(|parameter| parameter.name == "evidence_scope"));
            assert!(matches!(
                step.category,
                TransformCategory::FaultInjection | TransformCategory::Quantization
            ));
        }

        let jitter = SampleClockJitterTransform {
            jitter_s: 0.001,
            seed: 48,
        };
        let first = jitter.apply(&input).expect("jitter should apply");
        let second = jitter.apply(&input).expect("jitter should repeat");
        assert_eq!(first.time, second.time);
        assert_ne!(first.time, input.time);
        assert_eq!(
            first.metadata.transform_steps[0].category,
            TransformCategory::FaultInjection
        );
    }

    #[test]
    fn m34_fault_and_adc_simulation_filters_reject_invalid_inputs() {
        let input = waveform_with_time(vec![0.0, 0.1, 0.2, 0.3], vec![0.0, 1.0, 2.0, 3.0]);
        assert!(matches!(
            NoiseInjectionTransform {
                kind: NoiseKind::White,
                amplitude_v: 0.0,
                min_v: 0.0,
                max_v: 0.0,
                probability: 0.0,
                seed: 1,
            }
            .apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            NoiseInjectionTransform {
                kind: NoiseKind::Impulse,
                amplitude_v: 1.0,
                min_v: 0.0,
                max_v: 0.0,
                probability: 1.5,
                seed: 1,
            }
            .apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            SampleFaultTransform {
                kind: SampleFaultKind::StuckAt,
                probability: 0.0,
                fault_value_v: 1.0,
                min_v: 0.0,
                max_v: 0.0,
                start_index: 3,
                duration_samples: 2,
                seed: 0,
            }
            .apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            SampleClockJitterTransform {
                jitter_s: 0.1,
                seed: 1,
            }
            .apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            AdcCodeDefectTransform {
                kind: AdcCodeDefectKind::MissingCode,
                bits: 2,
                min_v: 0.0,
                max_v: 3.0,
                missing_code: 9,
                coefficients: Vec::new(),
            }
            .apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            AdcCodeDefectTransform {
                kind: AdcCodeDefectKind::Inl,
                bits: 2,
                min_v: 0.0,
                max_v: 3.0,
                missing_code: 0,
                coefficients: Vec::new(),
            }
            .apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
    }

    #[test]
    fn m35_multi_channel_transforms_apply_known_outputs() {
        let input = waveform_with_channels(
            vec![0.0, 1.0, 2.0, 3.0],
            vec![
                Channel::new("a_v", Unit::volts(), vec![1.0, 2.0, 3.0, 4.0]),
                Channel::new("b_v", Unit::volts(), vec![0.5, 1.0, 1.5, 2.0]),
                Channel::new("x_v", Unit::volts(), vec![3.0, 0.0, 4.0, 0.0]),
                Channel::new("y_v", Unit::volts(), vec![4.0, 5.0, 0.0, 0.0]),
            ],
        );

        let differential = ChannelArithmeticTransform {
            kind: ChannelArithmeticKind::Differential,
            left_channel: "a_v".to_string(),
            right_channel: "b_v".to_string(),
            output_channel: "diff_v".to_string(),
            output_unit: None,
        }
        .apply(&input)
        .expect("differential channel should apply");
        assert_close_vec(
            &differential.channel("diff_v").unwrap().samples,
            &[0.5, 1.0, 1.5, 2.0],
        );
        assert_eq!(
            differential.metadata.transform_steps[0].category,
            TransformCategory::MultiChannel
        );
        assert!(
            !differential.metadata.transform_steps[0]
                .output_channels
                .preserves_names
        );

        let common_mode = ChannelArithmeticTransform {
            kind: ChannelArithmeticKind::CommonMode,
            left_channel: "a_v".to_string(),
            right_channel: "b_v".to_string(),
            output_channel: "common_v".to_string(),
            output_unit: None,
        }
        .apply(&input)
        .expect("common mode should apply");
        assert_close_vec(
            &common_mode.channel("common_v").unwrap().samples,
            &[0.75, 1.5, 2.25, 3.0],
        );

        let magnitude = VectorMagnitudeTransform {
            kind: VectorMagnitudeKind::VectorMagnitude,
            channels: vec!["x_v".to_string(), "y_v".to_string()],
            output_channel: "mag_v".to_string(),
            output_unit: None,
        }
        .apply(&input)
        .expect("vector magnitude should apply");
        assert_close_vec(
            &magnitude.channel("mag_v").unwrap().samples,
            &[5.0, 5.0, 4.0, 0.0],
        );

        let matrix = MatrixTransform {
            input_channels: vec!["a_v".to_string(), "b_v".to_string()],
            matrix: vec![vec![1.0, 1.0], vec![1.0, -1.0]],
            output_channels: vec!["sum_v".to_string(), "sub_v".to_string()],
            output_unit: None,
        }
        .apply(&input)
        .expect("matrix transform should apply");
        assert_close_vec(
            &matrix.channel("sum_v").unwrap().samples,
            &[1.5, 3.0, 4.5, 6.0],
        );
        assert_close_vec(
            &matrix.channel("sub_v").unwrap().samples,
            &[0.5, 1.0, 1.5, 2.0],
        );

        let rotated = CoordinateRotationTransform {
            x_channel: "x_v".to_string(),
            y_channel: "y_v".to_string(),
            angle_rad: std::f64::consts::FRAC_PI_2,
            output_x_channel: "rot_x".to_string(),
            output_y_channel: "rot_y".to_string(),
            output_unit: None,
        }
        .apply(&input)
        .expect("coordinate rotation should apply");
        assert_close_vec(
            &rotated.channel("rot_x").unwrap().samples,
            &[-4.0, -5.0, 0.0, 0.0],
        );
        assert_close_vec(
            &rotated.channel("rot_y").unwrap().samples,
            &[3.0, 0.0, 4.0, 0.0],
        );
    }

    #[test]
    fn m35_sensor_vibration_and_control_transforms_apply_known_outputs() {
        let input = waveform_with_channels(
            vec![0.0, 1.0, 2.0, 3.0],
            vec![
                Channel::new("pressure_v", Unit::volts(), vec![0.0, 2.5, 5.0, 7.5]),
                Channel::new("shunt_v", Unit::volts(), vec![0.1, 0.2, 0.3, 0.4]),
                Channel::new("bridge_v", Unit::volts(), vec![0.0, 0.005, 0.01, 0.015]),
                Channel::new(
                    "rtd_ohm",
                    Unit::new("ohm"),
                    vec![100.0, 100.385, 100.77, 101.155],
                ),
                Channel::new(
                    "thermistor_ohm",
                    Unit::new("ohm"),
                    vec![10000.0, 10000.0, 10000.0, 10000.0],
                ),
                Channel::new("accel_v", Unit::volts(), vec![2.5, 2.6, 2.7, 2.8]),
                Channel::new("frequency_hz", Unit::new("Hz"), vec![1.0, 2.0, 3.0, 4.0]),
                Channel::new("accel_m_s2", Unit::new("m/s^2"), vec![0.0, 2.0, 2.0, 2.0]),
                Channel::new("measured_v", Unit::volts(), vec![0.0, 0.0, 0.0, 0.0]),
                Channel::new("command_v", Unit::volts(), vec![0.0, 10.0, 0.0, 10.0]),
            ],
        );

        let pressure = SensorConversionTransform {
            kind: SensorConversionKind::Pressure,
            channel: "pressure_v".to_string(),
            output_channel: "pressure_kpa".to_string(),
            output_unit: "kPa".to_string(),
            parameters: SensorConversionParameters {
                input_min_v: Some(0.0),
                input_max_v: Some(5.0),
                output_min: Some(0.0),
                output_max: Some(100.0),
                ..SensorConversionParameters::default()
            },
        }
        .apply(&input)
        .expect("pressure conversion should apply");
        assert_close_vec(
            &pressure.channel("pressure_kpa").unwrap().samples,
            &[0.0, 50.0, 100.0, 150.0],
        );
        assert_eq!(
            pressure.metadata.transform_steps[0].category,
            TransformCategory::Calibration
        );

        let current = SensorConversionTransform {
            kind: SensorConversionKind::CurrentShunt,
            channel: "shunt_v".to_string(),
            output_channel: "current_a".to_string(),
            output_unit: "A".to_string(),
            parameters: SensorConversionParameters {
                shunt_ohms: Some(0.1),
                ..SensorConversionParameters::default()
            },
        }
        .apply(&input)
        .expect("current shunt should apply");
        assert_close_vec(
            &current.channel("current_a").unwrap().samples,
            &[1.0, 2.0, 3.0, 4.0],
        );

        let load_cell = SensorConversionTransform {
            kind: SensorConversionKind::LoadCell,
            channel: "bridge_v".to_string(),
            output_channel: "force_n".to_string(),
            output_unit: "N".to_string(),
            parameters: SensorConversionParameters {
                excitation_v: Some(5.0),
                sensitivity_mv_v: Some(2.0),
                full_scale: Some(100.0),
                ..SensorConversionParameters::default()
            },
        }
        .apply(&input)
        .expect("load cell should apply");
        assert_close_vec(
            &load_cell.channel("force_n").unwrap().samples,
            &[0.0, 50.0, 100.0, 150.0],
        );

        let rtd = SensorConversionTransform {
            kind: SensorConversionKind::Rtd,
            channel: "rtd_ohm".to_string(),
            output_channel: "rtd_c".to_string(),
            output_unit: "C".to_string(),
            parameters: SensorConversionParameters {
                r0_ohm: Some(100.0),
                alpha_per_c: Some(0.00385),
                ..SensorConversionParameters::default()
            },
        }
        .apply(&input)
        .expect("RTD should apply");
        assert_close_vec(
            &rtd.channel("rtd_c").unwrap().samples,
            &[0.0, 1.0, 2.0, 3.0],
        );

        let thermistor = SensorConversionTransform {
            kind: SensorConversionKind::Thermistor,
            channel: "thermistor_ohm".to_string(),
            output_channel: "thermistor_c".to_string(),
            output_unit: "C".to_string(),
            parameters: SensorConversionParameters {
                r0_ohm: Some(10000.0),
                beta_k: Some(3950.0),
                t0_c: Some(25.0),
                ..SensorConversionParameters::default()
            },
        }
        .apply(&input)
        .expect("thermistor should apply");
        assert_close_vec(
            &thermistor.channel("thermistor_c").unwrap().samples,
            &[25.0, 25.0, 25.0, 25.0],
        );

        let tach = SensorConversionTransform {
            kind: SensorConversionKind::TachometerRpm,
            channel: "frequency_hz".to_string(),
            output_channel: "rpm".to_string(),
            output_unit: "rpm".to_string(),
            parameters: SensorConversionParameters {
                pulses_per_rev: Some(2.0),
                ..SensorConversionParameters::default()
            },
        }
        .apply(&input)
        .expect("tachometer should apply");
        assert_close_vec(
            &tach.channel("rpm").unwrap().samples,
            &[30.0, 60.0, 90.0, 120.0],
        );

        let accel = SensorConversionTransform {
            kind: SensorConversionKind::Accelerometer,
            channel: "accel_v".to_string(),
            output_channel: "accel_g".to_string(),
            output_unit: "g".to_string(),
            parameters: SensorConversionParameters {
                sensitivity_v_per_unit: Some(0.1),
                bias_v: Some(2.5),
                ..SensorConversionParameters::default()
            },
        }
        .apply(&input)
        .expect("accelerometer should apply");
        assert_close_vec(
            &accel.channel("accel_g").unwrap().samples,
            &[0.0, 1.0, 2.0, 3.0],
        );

        let velocity = VibrationTransform {
            kind: VibrationTransformKind::VelocityFromAcceleration,
            channel: "accel_m_s2".to_string(),
            output_channel: "velocity_m_s".to_string(),
            output_unit: "m/s".to_string(),
            window_samples: 1,
        }
        .apply(&input)
        .expect("velocity integration should apply");
        assert_close_vec(
            &velocity.channel("velocity_m_s").unwrap().samples,
            &[0.0, 1.0, 3.0, 5.0],
        );

        let severity = VibrationTransform {
            kind: VibrationTransformKind::VibrationSeverity,
            channel: "accel_m_s2".to_string(),
            output_channel: "severity".to_string(),
            output_unit: "m/s^2".to_string(),
            window_samples: 2,
        }
        .apply(&input)
        .expect("vibration severity should apply");
        assert_close_vec(
            &severity.channel("severity").unwrap().samples,
            &[0.0, 2.0_f64.sqrt(), 2.0, 2.0],
        );

        let pid = ControlTransform {
            kind: ControlTransformKind::PidControl,
            channel: "measured_v".to_string(),
            output_channel: "pid_v".to_string(),
            output_unit: None,
            setpoint: 1.0,
            kp: 1.0,
            ki: 1.0,
            kd: 0.0,
            rate_limit_per_s: 0.0,
            min_v: 0.0,
            max_v: 0.0,
            threshold_v: 0.0,
            feedforward_gain: 0.0,
            feedforward_offset: 0.0,
        }
        .apply(&input)
        .expect("PID should apply");
        assert_close_vec(
            &pid.channel("pid_v").unwrap().samples,
            &[1.0, 2.0, 3.0, 4.0],
        );
        assert_eq!(
            pid.metadata.transform_steps[0].category,
            TransformCategory::Control
        );

        let rate = ControlTransform {
            kind: ControlTransformKind::RateLimiter,
            channel: "command_v".to_string(),
            output_channel: "limited_v".to_string(),
            output_unit: None,
            setpoint: 0.0,
            kp: 0.0,
            ki: 0.0,
            kd: 0.0,
            rate_limit_per_s: 2.0,
            min_v: 0.0,
            max_v: 0.0,
            threshold_v: 0.0,
            feedforward_gain: 0.0,
            feedforward_offset: 0.0,
        }
        .apply(&input)
        .expect("rate limiter should apply");
        assert_close_vec(
            &rate.channel("limited_v").unwrap().samples,
            &[0.0, 2.0, 0.0, 2.0],
        );
    }

    #[test]
    fn m35_transforms_reject_invalid_inputs() {
        let mismatched_units = waveform_with_channels(
            vec![0.0, 1.0, 2.0, 3.0],
            vec![
                Channel::new("voltage", Unit::volts(), vec![1.0, 2.0, 3.0, 4.0]),
                Channel::new("current", Unit::new("A"), vec![1.0, 2.0, 3.0, 4.0]),
            ],
        );
        assert!(matches!(
            ChannelArithmeticTransform {
                kind: ChannelArithmeticKind::Add,
                left_channel: "voltage".to_string(),
                right_channel: "current".to_string(),
                output_channel: "sum".to_string(),
                output_unit: None,
            }
            .apply(&mismatched_units),
            Err(WaveformError::InvalidParameter { .. })
        ));

        let input = waveform(vec![0.0, 1.0, 2.0, 3.0]);
        assert!(matches!(
            VectorMagnitudeTransform {
                kind: VectorMagnitudeKind::VectorMagnitude,
                channels: vec!["input_v".to_string(), "missing".to_string()],
                output_channel: "mag".to_string(),
                output_unit: None,
            }
            .apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            MatrixTransform {
                input_channels: vec!["input_v".to_string()],
                matrix: vec![vec![1.0, 2.0]],
                output_channels: vec!["bad".to_string()],
                output_unit: None,
            }
            .apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            SensorConversionTransform {
                kind: SensorConversionKind::Thermistor,
                channel: "input_v".to_string(),
                output_channel: "temp".to_string(),
                output_unit: "C".to_string(),
                parameters: SensorConversionParameters {
                    r0_ohm: Some(10000.0),
                    beta_k: Some(3950.0),
                    t0_c: Some(25.0),
                    ..SensorConversionParameters::default()
                },
            }
            .apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            ControlTransform {
                kind: ControlTransformKind::RateLimiter,
                channel: "input_v".to_string(),
                output_channel: "limited".to_string(),
                output_unit: None,
                setpoint: 0.0,
                kp: 0.0,
                ki: 0.0,
                kd: 0.0,
                rate_limit_per_s: 0.0,
                min_v: 0.0,
                max_v: 0.0,
                threshold_v: 0.0,
                feedforward_gain: 0.0,
                feedforward_offset: 0.0,
            }
            .apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
    }

    #[test]
    fn m35_filter_metadata_matches_catalog() {
        let input = waveform_with_channels(
            vec![0.0, 1.0, 2.0, 3.0],
            vec![
                Channel::new("a_v", Unit::volts(), vec![1.0, 2.0, 3.0, 4.0]),
                Channel::new("b_v", Unit::volts(), vec![0.5, 1.0, 1.5, 2.0]),
            ],
        );
        let filters = vec![
            FilterStep::ChannelArithmetic(ChannelArithmeticTransform {
                kind: ChannelArithmeticKind::Add,
                left_channel: "a_v".to_string(),
                right_channel: "b_v".to_string(),
                output_channel: "sum_v".to_string(),
                output_unit: None,
            }),
            FilterStep::VectorMagnitude(VectorMagnitudeTransform {
                kind: VectorMagnitudeKind::EuclideanNorm,
                channels: vec!["a_v".to_string(), "b_v".to_string()],
                output_channel: "norm_v".to_string(),
                output_unit: None,
            }),
            FilterStep::sensor_conversion(SensorConversionTransform {
                kind: SensorConversionKind::CurrentShunt,
                channel: "a_v".to_string(),
                output_channel: "current_a".to_string(),
                output_unit: "A".to_string(),
                parameters: SensorConversionParameters {
                    shunt_ohms: Some(1.0),
                    ..SensorConversionParameters::default()
                },
            }),
            FilterStep::ControlTransform(ControlTransform {
                kind: ControlTransformKind::ControlDeadzone,
                channel: "a_v".to_string(),
                output_channel: "deadzone_v".to_string(),
                output_unit: None,
                setpoint: 0.0,
                kp: 0.0,
                ki: 0.0,
                kd: 0.0,
                rate_limit_per_s: 0.0,
                min_v: 0.0,
                max_v: 0.0,
                threshold_v: 0.75,
                feedforward_gain: 0.0,
                feedforward_offset: 0.0,
            }),
        ];
        let derived =
            apply_filter_chain(&input, &filters).expect("M35 metadata chain should evaluate");
        for step in &derived.metadata.transform_steps {
            let entry = transform_catalog_entry(&step.name).expect("M35 step should be cataloged");
            assert!(
                entry.matches_step_metadata(step),
                "metadata for `{}` does not match catalog entry: {step:?} vs {entry:?}",
                step.name
            );
        }
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
    fn high_pass_baseline_removes_constant_baseline_and_records_metadata() {
        let input = waveform(vec![5.0, 5.0, 5.0, 5.0]);
        let filtered = HighPassBaselineFilter { cutoff_hz: 0.5 }
            .apply(&input)
            .expect("high-pass baseline should apply");

        assert_eq!(input.channels[0].samples, vec![5.0, 5.0, 5.0, 5.0]);
        assert_eq!(filtered.channels[0].samples, vec![0.0, 0.0, 0.0, 0.0]);
        assert_eq!(
            filtered.metadata.transform_history,
            vec!["high_pass_baseline(cutoff_hz=0.5)"]
        );

        let step = &filtered.metadata.transform_steps[0];
        assert_common_transform_metadata(step);
        assert_eq!(step.name, "high_pass_baseline");
        assert_eq!(step.category, TransformCategory::Stateful);
        assert_eq!(
            step.parameters[0].value,
            TransformParameterValue::Float(0.5)
        );
        assert_eq!(step.parameters[0].unit.as_deref(), Some("Hz"));
        assert!(step.sample_rate_required);
        assert!(step.stateful);
        assert_eq!(step.phase_effect, TransformPhaseEffect::Delay);
    }

    #[test]
    fn high_pass_baseline_attenuates_slow_drift() {
        let input = waveform(vec![10.0, 10.1, 10.2, 10.3]);
        let filtered = HighPassBaselineFilter { cutoff_hz: 0.5 }
            .apply(&input)
            .expect("high-pass baseline should apply");

        assert_eq!(filtered.channels[0].samples[0], 0.0);
        assert!(filtered.channels[0].samples[1] > 0.0);
        assert!(filtered.channels[0].samples[3].abs() < 0.2);
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
        assert!(matches!(
            HighPassBaselineFilter { cutoff_hz: 0.0 }.apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            HighPassBaselineFilter {
                cutoff_hz: f64::NAN,
            }
            .apply(&input),
            Err(WaveformError::InvalidParameter { .. })
        ));
        assert!(matches!(
            HighPassBaselineFilter { cutoff_hz: 1.0 }.apply(&waveform(vec![
                f64::NAN,
                1.0,
                2.0,
                3.0
            ])),
            Err(WaveformError::InvalidWaveform { .. })
        ));
    }

    #[test]
    fn high_pass_baseline_rejects_invalid_time_axis() {
        let duplicate_time = Waveform::new(
            vec![0.0, 1.0, 1.0, 2.0],
            vec![Channel::new(
                "input_v",
                Unit::volts(),
                vec![0.0, 1.0, 2.0, 3.0],
            )],
        )
        .expect("test waveform should construct");

        assert!(matches!(
            HighPassBaselineFilter { cutoff_hz: 1.0 }.apply(&duplicate_time),
            Err(WaveformError::InvalidWaveform { .. })
        ));

        let non_finite_time = Waveform::new(
            vec![0.0, 1.0, f64::NAN, 2.0],
            vec![Channel::new(
                "input_v",
                Unit::volts(),
                vec![0.0, 1.0, 2.0, 3.0],
            )],
        )
        .expect("test waveform should construct");

        assert!(matches!(
            HighPassBaselineFilter { cutoff_hz: 1.0 }.apply(&non_finite_time),
            Err(WaveformError::InvalidWaveform { .. })
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
    fn channel_scoped_filter_updates_only_selected_channel() {
        let input = waveform_with_channels(
            vec![0.0, 1.0, 2.0, 3.0],
            vec![
                Channel::new("input_v", Unit::volts(), vec![1.0, 2.0, 3.0, 4.0]),
                Channel::new("output_v", Unit::volts(), vec![10.0, 11.0, 12.0, 13.0]),
            ],
        );
        let filtered = FilterStep::channel_scoped(
            "input_v".to_string(),
            FilterStep::Gain(GainTransform { gain: 2.0 }),
        )
        .apply(&input)
        .expect("channel-scoped filter should apply");

        assert_eq!(input.channels[0].samples, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(filtered.channels[0].samples, vec![2.0, 4.0, 6.0, 8.0]);
        assert_eq!(filtered.channels[1].samples, vec![10.0, 11.0, 12.0, 13.0]);
        let step = &filtered.metadata.transform_steps[0];
        assert_eq!(step.name, "gain");
        assert!(step.history_label.contains("scoped to input_v"));
        assert!(step.parameters.iter().any(|parameter| {
            parameter.name == "channel"
                && parameter.value == TransformParameterValue::Text("input_v".to_string())
        }));
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
