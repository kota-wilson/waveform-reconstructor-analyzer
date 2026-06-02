use serde::Deserialize;

pub use crate::criteria::{CriterionMeasurementKind, CriterionOperator};

use crate::criteria::{
    Criterion, EdgeDirection, MeasurementRequirement, MeasurementSpec, ResponseLatencySpec,
    RunSelectionConfig, SignalState, TransientEventKind, TransientEventWindow,
};
use crate::csv::CsvParseOptions;
use crate::error::{Result, WaveformError};
use crate::event::{
    BounceDetectionTransform, DebounceTransform, DwellTimeValidation, EdgeDirectionFilter,
    EdgeExtractionTransform, EventTransformStep, EventValidationStep, ExtraPulseValidation,
    GlitchRemovalTransform, MissingPulseValidation, SchmittTriggerTransform, TimeoutValidation,
};
use crate::feature::{
    AreaUnderCurveFeatureTransform, AutocorrelationFeatureTransform, BandPowerFeatureTransform,
    CorrelationFeatureTransform, CovarianceFeatureTransform, CrestFactorFeatureTransform,
    CrossCorrelationFeatureTransform, EnergyFeatureTransform, FeatureTransformStep,
    HarmonicFeatureTransform, HarmonicMetricFeatureTransform, HistogramFeatureTransform,
    IfftFeatureTransform, ImpulseEstimateFeatureTransform, KurtosisFeatureTransform,
    MaximumFeatureTransform, MeanFeatureTransform, MedianFeatureTransform, MinimumFeatureTransform,
    ModeFeatureTransform, PairedSpectrumFeatureTransform, PeakToPeakFeatureTransform,
    PercentileFeatureTransform, PowerFeatureTransform, QuantileFeatureTransform,
    RmsFeatureTransform, SkewnessFeatureTransform, SpectralRolloffFeatureTransform,
    SpectrumFeatureTransform, StandardDeviationFeatureTransform, TimeFrequencyFeatureTransform,
    VarianceFeatureTransform, WelchPsdFeatureTransform, WindowFunctionFeatureTransform, WindowSpec,
};
use crate::filter::{
    AbsoluteValueTransform, AdcCodeDefectKind, AdcCodeDefectTransform, AdcQuantizer,
    BandPassFilter, BandStopFilter, BaselineSubtractTransform, BesselLowPassFilter,
    BiquadCoefficients, BoxcarSmoothingFilter, ButterworthHighPassFilter, ButterworthLowPassFilter,
    CenteredMovingMedianFilter, ChannelArithmeticKind, ChannelArithmeticTransform,
    ChannelDelayTransform, Chebyshev1LowPassFilter, Chebyshev2LowPassFilter, ClampTransform,
    ClockDriftCorrectionTransform, CombFilter, CompandingKind, CompandingTransform,
    ControlTransform, ControlTransformKind, CoordinateRotationTransform, CropTransform,
    CrossCorrelationDelayTransform, CumulativeIntegralTransform, DcRemoveTransform,
    DeadbandTransform, DecimateTransform, DedupeTimestampsTransform, DitherTransform,
    DownsampleTransform, DriftFaultKind, DriftFaultTransform, EnvelopeTransform, ExpTransform,
    ExponentialMovingAverageFilter, FilterStep, FirFilter, FirstDerivativeTransform,
    FirstOrderHoldTransform, FixedDelayTransform, FractionalDelayTransform,
    FullWaveRectifyTransform, GainOffsetErrorKind, GainOffsetErrorTransform, GainTransform,
    GapFillTransform, GaussianSmoothingFilter, HalfWaveRectifyTransform, HampelFilter,
    HighPassBaselineFilter, HighPassFilter, IirBiquadFilter, IntegralTransform,
    InterpolateTransform, InvertTransform, JitterCorrectionTransform, LeakyIntegratorTransform,
    LinearDetrendTransform, LogTransform, LowPassFilter, MatrixTransform, MovingAverageFilter,
    MovingMedianFilter, MovingRmsTransform, NanInterpolateTransform, NanRemoveTransform,
    NoiseInjectionTransform, NoiseKind, NormalizeMode, NormalizeTransform, NotchFilter,
    OffsetTransform, OutlierDetectionTransform, PeakHoldTransform, PeriodicInterferenceKind,
    PeriodicInterferenceTransform, PiecewiseLinearTransform, PiecewisePoint,
    PolynomialDetrendTransform, PolynomialTransform, QuantileClipTransform,
    RationalResampleTransform, ResampleFixedTransform, ResampleTransform, RollingMaxTransform,
    RollingMeanBaselineTransform, RollingMeanTransform, RollingMedianBaselineTransform,
    RollingMinTransform, RollingStdDevTransform, RollingVarianceTransform, SampleAndHoldTransform,
    SampleClockJitterTransform, SampleFaultKind, SampleFaultTransform, SavitzkyGolayFilter,
    SecondDerivativeTransform, SensorConversionKind, SensorConversionParameters,
    SensorConversionTransform, SigmoidTransform, SimulationQuantizerKind,
    SimulationQuantizerTransform, SlopeDetectionTransform, SoftLimitTransform,
    SpikeRemoveTransform, SquareRootTransform, SquareTransform, TanhTransform,
    TimestampSortTransform, UpsampleTransform, VectorMagnitudeKind, VectorMagnitudeTransform,
    VibrationTransform, VibrationTransformKind, WeightedMovingAverageFilter, ZScoreTransform,
    ZeroOrderHoldTransform, ZeroPhaseFirFilter, ZeroPhaseIirBiquadFilter,
};
use crate::model::{MetadataContext, TolerancePolicy, Unit};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AnalysisConfig {
    pub input: InputConfig,
    #[serde(default)]
    pub metadata: MetadataContext,
    #[serde(default)]
    pub tolerances: TolerancePolicy,
    #[serde(default)]
    pub filters: Vec<FilterConfig>,
    #[serde(default)]
    pub feature_transforms: Vec<FeatureTransformConfig>,
    #[serde(default)]
    pub event_transforms: Vec<EventTransformConfig>,
    #[serde(default)]
    pub event_validations: Vec<EventValidationConfig>,
    #[serde(default)]
    pub criteria: Vec<CriterionConfig>,
}

impl AnalysisConfig {
    pub fn csv_options(&self) -> CsvParseOptions {
        let mut options =
            CsvParseOptions::new(self.input.time_column.clone(), self.input.channels.clone());
        options.time_unit = Unit::new(self.input.time_unit.clone());
        options.signal_unit = Unit::new(self.input.signal_unit.clone());
        options
    }

    pub fn criteria(&self) -> Result<Vec<Criterion>> {
        self.criteria
            .iter()
            .map(CriterionConfig::to_criterion)
            .collect()
    }

    pub fn filters(&self) -> Result<Vec<FilterStep>> {
        self.filters
            .iter()
            .map(FilterConfig::to_filter_step)
            .collect()
    }

    pub fn feature_transforms(&self) -> Result<Vec<FeatureTransformStep>> {
        self.feature_transforms
            .iter()
            .map(FeatureTransformConfig::to_feature_transform_step)
            .collect()
    }

    pub fn event_transforms(&self) -> Result<Vec<EventTransformStep>> {
        self.event_transforms
            .iter()
            .map(EventTransformConfig::to_event_transform_step)
            .collect()
    }

    pub fn event_validations(&self) -> Result<Vec<EventValidationStep>> {
        self.event_validations
            .iter()
            .map(EventValidationConfig::to_event_validation_step)
            .collect()
    }

    pub fn validate(&self) -> Result<()> {
        self.tolerances.validate()?;
        for criterion in &self.criteria {
            criterion.validate_schema()?;
        }
        self.feature_transforms()?;
        self.event_transforms()?;
        self.event_validations()?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InputConfig {
    pub time_column: String,
    pub channels: Vec<String>,
    #[serde(default = "default_time_unit")]
    pub time_unit: String,
    #[serde(default = "default_signal_unit")]
    pub signal_unit: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct FilterConfig {
    #[serde(rename = "type")]
    pub kind: String,
    pub window_samples: Option<usize>,
    pub cutoff_hz: Option<f64>,
    pub offset_v: Option<f64>,
    pub gain: Option<f64>,
    pub threshold_v: Option<f64>,
    pub baseline_v: Option<f64>,
    pub bits: Option<u8>,
    pub min_v: Option<f64>,
    pub max_v: Option<f64>,
    pub start_time_s: Option<f64>,
    pub end_time_s: Option<f64>,
    pub delay_s: Option<f64>,
    pub sample_interval_s: Option<f64>,
    pub channel: Option<String>,
    pub mode: Option<String>,
    pub base: Option<f64>,
    pub limit_v: Option<f64>,
    pub input_min_v: Option<f64>,
    pub input_max_v: Option<f64>,
    pub output_min: Option<f64>,
    pub output_max: Option<f64>,
    pub points: Option<Vec<PiecewisePointConfig>>,
    pub coefficients: Option<Vec<f64>>,
    pub weights: Option<Vec<f64>>,
    pub alpha: Option<f64>,
    pub sigma_samples: Option<f64>,
    pub polynomial_order: Option<usize>,
    pub outlier_sigma: Option<f64>,
    pub center_hz: Option<f64>,
    pub q: Option<f64>,
    pub delay_samples: Option<usize>,
    pub feedback_gain: Option<f64>,
    pub ripple_db: Option<f64>,
    pub stopband_attenuation_db: Option<f64>,
    pub factor: Option<usize>,
    pub upsample_factor: Option<usize>,
    pub downsample_factor: Option<usize>,
    pub reference_channel: Option<String>,
    pub target_channel: Option<String>,
    pub max_lag_samples: Option<usize>,
    pub time_constant_s: Option<f64>,
    pub threshold_per_s: Option<f64>,
    pub threshold_sigma: Option<f64>,
    pub lower_quantile: Option<f64>,
    pub upper_quantile: Option<f64>,
    pub amplitude_v: Option<f64>,
    pub stddev_v: Option<f64>,
    pub probability: Option<f64>,
    pub seed: Option<u64>,
    pub frequency_hz: Option<f64>,
    pub phase_rad: Option<f64>,
    pub drift_rate_v_per_s: Option<f64>,
    pub interval_samples: Option<usize>,
    pub fault_value_v: Option<f64>,
    pub start_index: Option<usize>,
    pub duration_samples: Option<usize>,
    pub lsb_v: Option<f64>,
    pub jitter_s: Option<f64>,
    pub missing_code: Option<u64>,
    pub gain_error: Option<f64>,
    pub offset_error_v: Option<f64>,
    pub mu: Option<f64>,
    pub channels: Option<Vec<String>>,
    pub left_channel: Option<String>,
    pub right_channel: Option<String>,
    pub x_channel: Option<String>,
    pub y_channel: Option<String>,
    pub output_channel: Option<String>,
    pub output_channels: Option<Vec<String>>,
    pub output_x_channel: Option<String>,
    pub output_y_channel: Option<String>,
    pub output_unit: Option<String>,
    pub matrix: Option<Vec<Vec<f64>>>,
    pub angle_rad: Option<f64>,
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
    pub setpoint: Option<f64>,
    pub kp: Option<f64>,
    pub ki: Option<f64>,
    pub kd: Option<f64>,
    pub rate_limit_per_s: Option<f64>,
    pub feedforward_gain: Option<f64>,
    pub feedforward_offset: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct PiecewisePointConfig {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct EventTransformConfig {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub channel: String,
    pub on_threshold_v: Option<f64>,
    pub off_threshold_v: Option<f64>,
    pub initial_state: Option<String>,
    pub min_duration_s: Option<f64>,
    pub max_duration_s: Option<f64>,
    pub window_s: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct EventValidationConfig {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub channel: String,
    pub direction: Option<String>,
    pub expected_count: Option<usize>,
    pub max_count: Option<usize>,
    pub state: Option<String>,
    pub min_duration_s: Option<f64>,
    pub start_time_s: Option<f64>,
    pub max_time_s: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FeatureTransformConfig {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub channel: String,
    pub other_channel: Option<String>,
    pub percentile: Option<f64>,
    pub quantile: Option<f64>,
    pub bins: Option<usize>,
    pub min_v: Option<f64>,
    pub max_v: Option<f64>,
    pub lag_samples: Option<usize>,
    pub window: Option<String>,
    pub window_samples: Option<usize>,
    pub overlap_samples: Option<usize>,
    pub window_beta: Option<f64>,
    pub window_alpha: Option<f64>,
    pub window_sigma: Option<f64>,
    pub fundamental_hz: Option<f64>,
    pub harmonic_count: Option<usize>,
    pub rolloff_percent: Option<f64>,
    pub band_low_hz: Option<f64>,
    pub band_high_hz: Option<f64>,
}

impl FilterConfig {
    fn to_filter_step(&self) -> Result<FilterStep> {
        match self.kind.as_str() {
            "nan_interpolate" => Ok(FilterStep::NanInterpolate(NanInterpolateTransform)),
            "nan_remove" => Ok(FilterStep::NanRemove(NanRemoveTransform)),
            "timestamp_sort" => Ok(FilterStep::TimestampSort(TimestampSortTransform)),
            "dedupe_timestamps" => Ok(FilterStep::DedupeTimestamps(DedupeTimestampsTransform)),
            "crop" => Ok(FilterStep::Crop(CropTransform {
                start_time_s: required_finite_filter_float("start_time_s", self.start_time_s)?,
                end_time_s: required_finite_filter_float("end_time_s", self.end_time_s)?,
            })),
            "fixed_delay" => Ok(FilterStep::FixedDelay(FixedDelayTransform {
                delay_s: required_finite_filter_float("delay_s", self.delay_s)?,
            })),
            "gap_fill" => Ok(FilterStep::GapFill(GapFillTransform {
                sample_interval_s: required_positive_filter_float(
                    "sample_interval_s",
                    self.sample_interval_s,
                )?,
            })),
            "resample_fixed" => Ok(FilterStep::ResampleFixed(ResampleFixedTransform {
                sample_interval_s: required_positive_filter_float(
                    "sample_interval_s",
                    self.sample_interval_s,
                )?,
            })),
            "channel_delay" => Ok(FilterStep::ChannelDelay(ChannelDelayTransform {
                channel: required_filter_string("channel", self.channel.as_deref())?,
                delay_s: required_finite_filter_float("delay_s", self.delay_s)?,
            })),
            "resample" => Ok(FilterStep::Resample(ResampleTransform {
                sample_interval_s: required_positive_filter_float(
                    "sample_interval_s",
                    self.sample_interval_s,
                )?,
            })),
            "downsample" => Ok(FilterStep::Downsample(DownsampleTransform {
                factor: required_resampling_factor("factor", self.factor)?,
            })),
            "decimate" => Ok(FilterStep::Decimate(DecimateTransform {
                factor: required_resampling_factor("factor", self.factor)?,
                cutoff_hz: required_positive_filter_float("cutoff_hz", self.cutoff_hz)?,
            })),
            "upsample" => Ok(FilterStep::Upsample(UpsampleTransform {
                factor: required_resampling_factor("factor", self.factor)?,
            })),
            "interpolate" => Ok(FilterStep::Interpolate(InterpolateTransform {
                sample_interval_s: required_positive_filter_float(
                    "sample_interval_s",
                    self.sample_interval_s,
                )?,
            })),
            "rational_resample" => Ok(FilterStep::RationalResample(RationalResampleTransform {
                upsample_factor: required_positive_filter_usize(
                    "upsample_factor",
                    self.upsample_factor,
                )?,
                downsample_factor: required_positive_filter_usize(
                    "downsample_factor",
                    self.downsample_factor,
                )?,
            })),
            "sample_and_hold" => Ok(FilterStep::SampleAndHold(SampleAndHoldTransform {
                sample_interval_s: required_positive_filter_float(
                    "sample_interval_s",
                    self.sample_interval_s,
                )?,
            })),
            "zero_order_hold" => Ok(FilterStep::ZeroOrderHold(ZeroOrderHoldTransform {
                sample_interval_s: required_positive_filter_float(
                    "sample_interval_s",
                    self.sample_interval_s,
                )?,
            })),
            "first_order_hold" => Ok(FilterStep::FirstOrderHold(FirstOrderHoldTransform {
                sample_interval_s: required_positive_filter_float(
                    "sample_interval_s",
                    self.sample_interval_s,
                )?,
            })),
            "fractional_delay" => Ok(FilterStep::FractionalDelay(FractionalDelayTransform {
                delay_s: required_finite_filter_float("delay_s", self.delay_s)?,
            })),
            "cross_correlation_delay" => Ok(FilterStep::CrossCorrelationDelay(
                CrossCorrelationDelayTransform {
                    reference_channel: required_filter_string(
                        "reference_channel",
                        self.reference_channel.as_deref(),
                    )?,
                    target_channel: required_filter_string(
                        "target_channel",
                        self.target_channel.as_deref(),
                    )?,
                    max_lag_samples: required_positive_filter_usize(
                        "max_lag_samples",
                        self.max_lag_samples,
                    )?,
                },
            )),
            "jitter_correction" => Ok(FilterStep::JitterCorrection(JitterCorrectionTransform {
                sample_interval_s: required_positive_filter_float(
                    "sample_interval_s",
                    self.sample_interval_s,
                )?,
            })),
            "clock_drift_correction" => Ok(FilterStep::ClockDriftCorrection(
                ClockDriftCorrectionTransform {
                    sample_interval_s: required_positive_filter_float(
                        "sample_interval_s",
                        self.sample_interval_s,
                    )?,
                },
            )),
            "half_wave_rectify" => Ok(FilterStep::HalfWaveRectify(HalfWaveRectifyTransform)),
            "full_wave_rectify" => Ok(FilterStep::FullWaveRectify(FullWaveRectifyTransform)),
            "envelope" => Ok(FilterStep::Envelope(EnvelopeTransform {
                alpha: required_unit_interval_filter_float("alpha", self.alpha)?,
            })),
            "moving_rms" => Ok(FilterStep::MovingRms(MovingRmsTransform {
                window_samples: required_window_samples(self.window_samples)?,
            })),
            "peak_hold" => Ok(FilterStep::PeakHold(PeakHoldTransform)),
            "first_derivative" => Ok(FilterStep::FirstDerivative(FirstDerivativeTransform)),
            "second_derivative" => Ok(FilterStep::SecondDerivative(SecondDerivativeTransform)),
            "integral" => Ok(FilterStep::Integral(IntegralTransform)),
            "cumulative_integral" => {
                Ok(FilterStep::CumulativeIntegral(CumulativeIntegralTransform))
            }
            "leaky_integrator" => Ok(FilterStep::LeakyIntegrator(LeakyIntegratorTransform {
                time_constant_s: required_positive_filter_float(
                    "time_constant_s",
                    self.time_constant_s,
                )?,
            })),
            "slope_detection" => Ok(FilterStep::SlopeDetection(SlopeDetectionTransform {
                threshold_per_s: required_positive_filter_float(
                    "threshold_per_s",
                    self.threshold_per_s,
                )?,
            })),
            "rolling_mean" => Ok(FilterStep::RollingMean(RollingMeanTransform {
                window_samples: required_window_samples(self.window_samples)?,
            })),
            "rolling_variance" => Ok(FilterStep::RollingVariance(RollingVarianceTransform {
                window_samples: required_window_samples(self.window_samples)?,
            })),
            "rolling_stddev" => Ok(FilterStep::RollingStdDev(RollingStdDevTransform {
                window_samples: required_window_samples(self.window_samples)?,
            })),
            "rolling_min" => Ok(FilterStep::RollingMin(RollingMinTransform {
                window_samples: required_window_samples(self.window_samples)?,
            })),
            "rolling_max" => Ok(FilterStep::RollingMax(RollingMaxTransform {
                window_samples: required_window_samples(self.window_samples)?,
            })),
            "z_score" => Ok(FilterStep::ZScore(ZScoreTransform)),
            "outlier_detection" => Ok(FilterStep::OutlierDetection(OutlierDetectionTransform {
                threshold_sigma: required_positive_filter_float(
                    "threshold_sigma",
                    self.threshold_sigma,
                )?,
            })),
            "quantile_clip" => Ok(FilterStep::QuantileClip(QuantileClipTransform {
                lower_quantile: required_unit_interval_filter_float(
                    "lower_quantile",
                    self.lower_quantile,
                )?,
                upper_quantile: required_unit_interval_filter_float(
                    "upper_quantile",
                    self.upper_quantile,
                )?,
            })),
            "white_noise" => self.noise_filter(NoiseKind::White),
            "gaussian_noise" => self.noise_filter(NoiseKind::Gaussian),
            "uniform_noise" => self.noise_filter(NoiseKind::Uniform),
            "pink_noise" => self.noise_filter(NoiseKind::Pink),
            "brown_noise" => self.noise_filter(NoiseKind::Brown),
            "impulse_noise" => self.noise_filter(NoiseKind::Impulse),
            "salt_pepper_noise" => self.noise_filter(NoiseKind::SaltPepper),
            "quantization_noise" => self.noise_filter(NoiseKind::Quantization),
            "periodic_interference" => self.periodic_filter(PeriodicInterferenceKind::Periodic),
            "hum_interference" => self.periodic_filter(PeriodicInterferenceKind::Hum),
            "ground_bounce" => self.drift_filter(DriftFaultKind::GroundBounce),
            "thermal_drift" => self.drift_filter(DriftFaultKind::Thermal),
            "random_walk_drift" => self.drift_filter(DriftFaultKind::RandomWalk),
            "dropout_fault" => self.sample_fault_filter(SampleFaultKind::Dropout),
            "missing_samples" => self.sample_fault_filter(SampleFaultKind::MissingSamples),
            "saturation_fault" => self.sample_fault_filter(SampleFaultKind::Saturation),
            "stuck_at_fault" => self.sample_fault_filter(SampleFaultKind::StuckAt),
            "flatline_fault" => self.sample_fault_filter(SampleFaultKind::Flatline),
            "intermittent_fault" => self.sample_fault_filter(SampleFaultKind::Intermittent),
            "rounding_quantizer" => self.quantizer_filter(SimulationQuantizerKind::Rounding),
            "floor_quantizer" => self.quantizer_filter(SimulationQuantizerKind::Floor),
            "ceil_quantizer" => self.quantizer_filter(SimulationQuantizerKind::Ceil),
            "midrise_quantizer" => self.quantizer_filter(SimulationQuantizerKind::MidRise),
            "midtread_quantizer" => self.quantizer_filter(SimulationQuantizerKind::MidTread),
            "saturating_quantizer" => self.quantizer_filter(SimulationQuantizerKind::Saturating),
            "dither" => Ok(FilterStep::Dither(DitherTransform {
                lsb_v: required_positive_filter_float("lsb_v", self.lsb_v)?,
                seed: self.seed.ok_or_else(|| missing_filter_field("seed"))?,
            })),
            "companding" => Ok(FilterStep::Companding(CompandingTransform {
                kind: self.companding_kind()?,
                max_v: required_positive_filter_float("max_v", self.max_v)?,
                mu: required_positive_filter_float("mu", self.mu)?,
            })),
            "sample_clock_jitter" => {
                Ok(FilterStep::SampleClockJitter(SampleClockJitterTransform {
                    jitter_s: required_positive_filter_float("jitter_s", self.jitter_s)?,
                    seed: self.seed.ok_or_else(|| missing_filter_field("seed"))?,
                }))
            }
            "adc_missing_code" => self.adc_code_defect_filter(AdcCodeDefectKind::MissingCode),
            "inl_error" => self.adc_code_defect_filter(AdcCodeDefectKind::Inl),
            "dnl_error" => self.adc_code_defect_filter(AdcCodeDefectKind::Dnl),
            "adc_gain_error" => Ok(FilterStep::GainOffsetError(GainOffsetErrorTransform {
                kind: GainOffsetErrorKind::Gain,
                gain_error: required_finite_filter_float("gain_error", self.gain_error)?,
                offset_error_v: 0.0,
            })),
            "adc_offset_error" => Ok(FilterStep::GainOffsetError(GainOffsetErrorTransform {
                kind: GainOffsetErrorKind::Offset,
                gain_error: 0.0,
                offset_error_v: required_finite_filter_float(
                    "offset_error_v",
                    self.offset_error_v,
                )?,
            })),
            "channel_add" => self.channel_arithmetic_filter(ChannelArithmeticKind::Add),
            "channel_subtract" => self.channel_arithmetic_filter(ChannelArithmeticKind::Subtract),
            "differential_channel" => {
                self.channel_arithmetic_filter(ChannelArithmeticKind::Differential)
            }
            "common_mode" => self.channel_arithmetic_filter(ChannelArithmeticKind::CommonMode),
            "vector_magnitude" => {
                self.vector_magnitude_filter(VectorMagnitudeKind::VectorMagnitude)
            }
            "euclidean_norm" => self.vector_magnitude_filter(VectorMagnitudeKind::EuclideanNorm),
            "matrix_transform" => self.matrix_filter(),
            "coordinate_rotation" => self.coordinate_rotation_filter(),
            "linear_sensor_conversion" => {
                self.sensor_conversion_filter(SensorConversionKind::Linear)
            }
            "pressure_transducer" => self.sensor_conversion_filter(SensorConversionKind::Pressure),
            "current_shunt" => self.sensor_conversion_filter(SensorConversionKind::CurrentShunt),
            "bridge_strain" => self.sensor_conversion_filter(SensorConversionKind::BridgeStrain),
            "load_cell_force" => self.sensor_conversion_filter(SensorConversionKind::LoadCell),
            "rtd_temperature" => self.sensor_conversion_filter(SensorConversionKind::Rtd),
            "thermistor_temperature" => {
                self.sensor_conversion_filter(SensorConversionKind::Thermistor)
            }
            "tachometer_rpm" => self.sensor_conversion_filter(SensorConversionKind::TachometerRpm),
            "encoder_position" => {
                self.sensor_conversion_filter(SensorConversionKind::EncoderPosition)
            }
            "accelerometer_units" => {
                self.sensor_conversion_filter(SensorConversionKind::Accelerometer)
            }
            "gyroscope_rate" => self.sensor_conversion_filter(SensorConversionKind::Gyroscope),
            "hall_current" => self.sensor_conversion_filter(SensorConversionKind::HallCurrent),
            "lvdt_position" => self.sensor_conversion_filter(SensorConversionKind::LvdtPosition),
            "microphone_spl" => self.sensor_conversion_filter(SensorConversionKind::MicrophoneSpl),
            "photodiode_power" => {
                self.sensor_conversion_filter(SensorConversionKind::PhotodiodePower)
            }
            "velocity_from_acceleration" => {
                self.vibration_filter(VibrationTransformKind::VelocityFromAcceleration)
            }
            "displacement_from_velocity" => {
                self.vibration_filter(VibrationTransformKind::DisplacementFromVelocity)
            }
            "vibration_severity" => {
                self.vibration_filter(VibrationTransformKind::VibrationSeverity)
            }
            "control_error" => self.control_filter(ControlTransformKind::ErrorSignal),
            "proportional_control" => {
                self.control_filter(ControlTransformKind::ProportionalControl)
            }
            "pid_control" => self.control_filter(ControlTransformKind::PidControl),
            "rate_limiter" => self.control_filter(ControlTransformKind::RateLimiter),
            "slew_rate_limit" => self.control_filter(ControlTransformKind::SlewRateLimit),
            "control_saturation" => self.control_filter(ControlTransformKind::ControlSaturation),
            "control_deadzone" => self.control_filter(ControlTransformKind::ControlDeadzone),
            "feedforward_control" => self.control_filter(ControlTransformKind::FeedforwardControl),
            "absolute_value" => Ok(FilterStep::AbsoluteValue(AbsoluteValueTransform)),
            "square" => Ok(FilterStep::Square(SquareTransform)),
            "square_root" => Ok(FilterStep::SquareRoot(SquareRootTransform)),
            "log" => Ok(FilterStep::Log(LogTransform {
                base: required_positive_filter_float("base", self.base)?,
            })),
            "exp" => Ok(FilterStep::Exp(ExpTransform {
                base: required_positive_filter_float("base", self.base)?,
            })),
            "normalize" => Ok(FilterStep::Normalize(NormalizeTransform {
                mode: self.normalize_mode()?,
            })),
            "tanh" => Ok(FilterStep::Tanh(TanhTransform)),
            "sigmoid" => Ok(FilterStep::Sigmoid(SigmoidTransform)),
            "soft_limit" => Ok(FilterStep::SoftLimit(SoftLimitTransform {
                limit_v: required_positive_filter_float("limit_v", self.limit_v)?,
            })),
            "piecewise_linear" => Ok(FilterStep::PiecewiseLinear(PiecewiseLinearTransform {
                points: required_piecewise_points(self.points.as_deref())?,
            })),
            "polynomial" => Ok(FilterStep::Polynomial(PolynomialTransform {
                coefficients: required_coefficients(self.coefficients.as_deref())?,
            })),
            "weighted_moving_average" => Ok(FilterStep::WeightedMovingAverage(
                WeightedMovingAverageFilter {
                    weights: required_weights(self.weights.as_deref())?,
                },
            )),
            "exponential_moving_average" => Ok(FilterStep::ExponentialMovingAverage(
                ExponentialMovingAverageFilter {
                    alpha: required_alpha(self.alpha)?,
                },
            )),
            "boxcar_smoothing" => Ok(FilterStep::BoxcarSmoothing(BoxcarSmoothingFilter {
                window_samples: required_window_samples(self.window_samples)?,
            })),
            "gaussian_smoothing" => Ok(FilterStep::GaussianSmoothing(GaussianSmoothingFilter {
                window_samples: required_window_samples(self.window_samples)?,
                sigma_samples: required_positive_filter_float("sigma_samples", self.sigma_samples)?,
            })),
            "savitzky_golay" => Ok(FilterStep::SavitzkyGolay(SavitzkyGolayFilter {
                window_samples: required_window_samples(self.window_samples)?,
                polynomial_order: required_polynomial_order(self.polynomial_order)?,
            })),
            "centered_moving_median" => Ok(FilterStep::CenteredMovingMedian(
                CenteredMovingMedianFilter {
                    window_samples: required_window_samples(self.window_samples)?,
                },
            )),
            "rolling_mean_baseline" => Ok(FilterStep::RollingMeanBaseline(
                RollingMeanBaselineTransform {
                    window_samples: required_window_samples(self.window_samples)?,
                },
            )),
            "rolling_median_baseline" => Ok(FilterStep::RollingMedianBaseline(
                RollingMedianBaselineTransform {
                    window_samples: required_window_samples(self.window_samples)?,
                },
            )),
            "linear_detrend" => Ok(FilterStep::LinearDetrend(LinearDetrendTransform)),
            "polynomial_detrend" => Ok(FilterStep::PolynomialDetrend(PolynomialDetrendTransform {
                polynomial_order: required_polynomial_order(self.polynomial_order)?,
            })),
            "hampel_filter" => Ok(FilterStep::HampelFilter(HampelFilter {
                window_samples: required_window_samples(self.window_samples)?,
                outlier_sigma: required_positive_filter_float("outlier_sigma", self.outlier_sigma)?,
            })),
            "spike_remove" => Ok(FilterStep::SpikeRemove(SpikeRemoveTransform {
                window_samples: required_window_samples(self.window_samples)?,
                threshold_v: required_positive_filter_float("threshold_v", self.threshold_v)?,
            })),
            "fir_filter" => Ok(FilterStep::FirFilter(FirFilter {
                coefficients: required_coefficients(self.coefficients.as_deref())?,
            })),
            "zero_phase_fir_filter" => Ok(FilterStep::ZeroPhaseFirFilter(ZeroPhaseFirFilter {
                coefficients: required_coefficients(self.coefficients.as_deref())?,
            })),
            "iir_biquad" => Ok(FilterStep::IirBiquad(IirBiquadFilter {
                coefficients: required_biquad_coefficients(self.coefficients.as_deref())?,
            })),
            "zero_phase_iir_biquad" => {
                Ok(FilterStep::ZeroPhaseIirBiquad(ZeroPhaseIirBiquadFilter {
                    coefficients: required_biquad_coefficients(self.coefficients.as_deref())?,
                }))
            }
            "high_pass" => Ok(FilterStep::HighPass(HighPassFilter {
                cutoff_hz: required_positive_filter_float("cutoff_hz", self.cutoff_hz)?,
            })),
            "band_pass" => Ok(FilterStep::BandPass(BandPassFilter {
                center_hz: required_positive_filter_float("center_hz", self.center_hz)?,
                q: required_positive_filter_float("q", self.q)?,
            })),
            "band_stop" => Ok(FilterStep::BandStop(BandStopFilter {
                center_hz: required_positive_filter_float("center_hz", self.center_hz)?,
                q: required_positive_filter_float("q", self.q)?,
            })),
            "notch" => Ok(FilterStep::Notch(NotchFilter {
                center_hz: required_positive_filter_float("center_hz", self.center_hz)?,
                q: required_positive_filter_float("q", self.q)?,
            })),
            "comb_filter" => Ok(FilterStep::CombFilter(CombFilter {
                delay_samples: required_delay_samples(self.delay_samples)?,
                feedback_gain: required_finite_filter_float("feedback_gain", self.feedback_gain)?,
            })),
            "butterworth_low_pass" => {
                Ok(FilterStep::ButterworthLowPass(ButterworthLowPassFilter {
                    cutoff_hz: required_positive_filter_float("cutoff_hz", self.cutoff_hz)?,
                }))
            }
            "butterworth_high_pass" => {
                Ok(FilterStep::ButterworthHighPass(ButterworthHighPassFilter {
                    cutoff_hz: required_positive_filter_float("cutoff_hz", self.cutoff_hz)?,
                }))
            }
            "chebyshev1_low_pass" => Ok(FilterStep::Chebyshev1LowPass(Chebyshev1LowPassFilter {
                cutoff_hz: required_positive_filter_float("cutoff_hz", self.cutoff_hz)?,
                ripple_db: required_positive_filter_float("ripple_db", self.ripple_db)?,
            })),
            "chebyshev2_low_pass" => Ok(FilterStep::Chebyshev2LowPass(Chebyshev2LowPassFilter {
                cutoff_hz: required_positive_filter_float("cutoff_hz", self.cutoff_hz)?,
                stopband_attenuation_db: required_positive_filter_float(
                    "stopband_attenuation_db",
                    self.stopband_attenuation_db,
                )?,
            })),
            "bessel_low_pass" => Ok(FilterStep::BesselLowPass(BesselLowPassFilter {
                cutoff_hz: required_positive_filter_float("cutoff_hz", self.cutoff_hz)?,
            })),
            "offset" => Ok(FilterStep::Offset(OffsetTransform {
                offset_v: self
                    .offset_v
                    .ok_or_else(|| missing_filter_field("offset_v"))?,
            })),
            "gain" => Ok(FilterStep::Gain(GainTransform {
                gain: self.gain.ok_or_else(|| missing_filter_field("gain"))?,
            })),
            "invert" => Ok(FilterStep::Invert(InvertTransform)),
            "clamp" => Ok(FilterStep::Clamp(ClampTransform {
                min_v: self.min_v.ok_or_else(|| missing_filter_field("min_v"))?,
                max_v: self.max_v.ok_or_else(|| missing_filter_field("max_v"))?,
            })),
            "deadband" => Ok(FilterStep::Deadband(DeadbandTransform {
                threshold_v: self
                    .threshold_v
                    .ok_or_else(|| missing_filter_field("threshold_v"))?,
            })),
            "dc_remove" => Ok(FilterStep::DcRemove(DcRemoveTransform)),
            "baseline_subtract" => Ok(FilterStep::BaselineSubtract(BaselineSubtractTransform {
                baseline_v: self
                    .baseline_v
                    .ok_or_else(|| missing_filter_field("baseline_v"))?,
            })),
            "high_pass_baseline" => Ok(FilterStep::HighPassBaseline(HighPassBaselineFilter {
                cutoff_hz: required_positive_filter_float("cutoff_hz", self.cutoff_hz)?,
            })),
            "moving_average" => Ok(FilterStep::MovingAverage(MovingAverageFilter {
                window_samples: self
                    .window_samples
                    .ok_or_else(|| missing_filter_field("window_samples"))?,
            })),
            "moving_median" => Ok(FilterStep::MovingMedian(MovingMedianFilter {
                window_samples: self
                    .window_samples
                    .ok_or_else(|| missing_filter_field("window_samples"))?,
            })),
            "low_pass" => Ok(FilterStep::LowPass(LowPassFilter {
                cutoff_hz: self
                    .cutoff_hz
                    .ok_or_else(|| missing_filter_field("cutoff_hz"))?,
            })),
            "adc_quantize" => Ok(FilterStep::AdcQuantize(AdcQuantizer {
                bits: self.bits.ok_or_else(|| missing_filter_field("bits"))?,
                min_v: self.min_v.ok_or_else(|| missing_filter_field("min_v"))?,
                max_v: self.max_v.ok_or_else(|| missing_filter_field("max_v"))?,
            })),
            _ => Err(WaveformError::InvalidParameter {
                name: "filters.type".to_string(),
                reason: format!("unsupported filter type `{}`", self.kind),
            }),
        }
    }

    fn noise_filter(&self, kind: NoiseKind) -> Result<FilterStep> {
        let amplitude_v = match kind {
            NoiseKind::Gaussian => required_positive_filter_float("stddev_v", self.stddev_v)?,
            NoiseKind::Quantization => required_positive_filter_float("lsb_v", self.lsb_v)?,
            NoiseKind::Uniform | NoiseKind::SaltPepper => 0.0,
            _ => required_positive_filter_float("amplitude_v", self.amplitude_v)?,
        };
        Ok(FilterStep::NoiseInjection(NoiseInjectionTransform {
            kind,
            amplitude_v,
            min_v: match kind {
                NoiseKind::Uniform | NoiseKind::SaltPepper => {
                    required_finite_filter_float("min_v", self.min_v)?
                }
                _ => 0.0,
            },
            max_v: match kind {
                NoiseKind::Uniform | NoiseKind::SaltPepper => {
                    required_finite_filter_float("max_v", self.max_v)?
                }
                _ => 0.0,
            },
            probability: match kind {
                NoiseKind::Impulse | NoiseKind::SaltPepper => {
                    required_unit_interval_filter_float("probability", self.probability)?
                }
                _ => 0.0,
            },
            seed: self.seed.ok_or_else(|| missing_filter_field("seed"))?,
        }))
    }

    fn periodic_filter(&self, kind: PeriodicInterferenceKind) -> Result<FilterStep> {
        Ok(FilterStep::PeriodicInterference(
            PeriodicInterferenceTransform {
                kind,
                amplitude_v: required_positive_filter_float("amplitude_v", self.amplitude_v)?,
                frequency_hz: required_positive_filter_float("frequency_hz", self.frequency_hz)?,
                phase_rad: self.phase_rad.unwrap_or(0.0),
            },
        ))
    }

    fn drift_filter(&self, kind: DriftFaultKind) -> Result<FilterStep> {
        Ok(FilterStep::DriftFault(DriftFaultTransform {
            kind,
            amplitude_v: match kind {
                DriftFaultKind::GroundBounce | DriftFaultKind::RandomWalk => {
                    required_positive_filter_float("amplitude_v", self.amplitude_v)?
                }
                DriftFaultKind::Thermal => 0.0,
            },
            drift_rate_v_per_s: match kind {
                DriftFaultKind::Thermal => {
                    required_finite_filter_float("drift_rate_v_per_s", self.drift_rate_v_per_s)?
                }
                _ => 0.0,
            },
            interval_samples: match kind {
                DriftFaultKind::GroundBounce => {
                    required_positive_filter_usize("interval_samples", self.interval_samples)?
                }
                _ => 1,
            },
            seed: match kind {
                DriftFaultKind::RandomWalk => {
                    self.seed.ok_or_else(|| missing_filter_field("seed"))?
                }
                _ => 0,
            },
        }))
    }

    fn sample_fault_filter(&self, kind: SampleFaultKind) -> Result<FilterStep> {
        Ok(FilterStep::SampleFault(SampleFaultTransform {
            kind,
            probability: match kind {
                SampleFaultKind::Dropout
                | SampleFaultKind::MissingSamples
                | SampleFaultKind::Intermittent => {
                    required_unit_interval_filter_float("probability", self.probability)?
                }
                _ => 0.0,
            },
            fault_value_v: match kind {
                SampleFaultKind::Dropout
                | SampleFaultKind::MissingSamples
                | SampleFaultKind::Intermittent
                | SampleFaultKind::StuckAt => {
                    required_finite_filter_float("fault_value_v", self.fault_value_v)?
                }
                SampleFaultKind::Flatline => self.fault_value_v.unwrap_or(f64::NAN),
                SampleFaultKind::Saturation => 0.0,
            },
            min_v: match kind {
                SampleFaultKind::Saturation => required_finite_filter_float("min_v", self.min_v)?,
                _ => 0.0,
            },
            max_v: match kind {
                SampleFaultKind::Saturation => required_finite_filter_float("max_v", self.max_v)?,
                _ => 0.0,
            },
            start_index: match kind {
                SampleFaultKind::StuckAt | SampleFaultKind::Flatline => self
                    .start_index
                    .ok_or_else(|| missing_filter_field("start_index"))?,
                _ => 0,
            },
            duration_samples: match kind {
                SampleFaultKind::StuckAt => {
                    required_positive_filter_usize("duration_samples", self.duration_samples)?
                }
                _ => 1,
            },
            seed: match kind {
                SampleFaultKind::Dropout
                | SampleFaultKind::MissingSamples
                | SampleFaultKind::Intermittent => {
                    self.seed.ok_or_else(|| missing_filter_field("seed"))?
                }
                _ => 0,
            },
        }))
    }

    fn quantizer_filter(&self, kind: SimulationQuantizerKind) -> Result<FilterStep> {
        Ok(FilterStep::SimulationQuantizer(
            SimulationQuantizerTransform {
                kind,
                lsb_v: match kind {
                    SimulationQuantizerKind::Saturating => 1.0,
                    _ => required_positive_filter_float("lsb_v", self.lsb_v)?,
                },
                min_v: match kind {
                    SimulationQuantizerKind::Saturating => {
                        required_finite_filter_float("min_v", self.min_v)?
                    }
                    _ => 0.0,
                },
                max_v: match kind {
                    SimulationQuantizerKind::Saturating => {
                        required_finite_filter_float("max_v", self.max_v)?
                    }
                    _ => 0.0,
                },
            },
        ))
    }

    fn companding_kind(&self) -> Result<CompandingKind> {
        let mode = required_filter_string("mode", self.mode.as_deref())?;
        match mode.as_str() {
            "mu_law" => Ok(CompandingKind::MuLaw),
            "a_law" => Ok(CompandingKind::ALaw),
            other => Err(WaveformError::InvalidParameter {
                name: "filters.mode".to_string(),
                reason: format!("expected `mu_law` or `a_law`, got `{other}`"),
            }),
        }
    }

    fn adc_code_defect_filter(&self, kind: AdcCodeDefectKind) -> Result<FilterStep> {
        Ok(FilterStep::AdcCodeDefect(AdcCodeDefectTransform {
            kind,
            bits: self.bits.ok_or_else(|| missing_filter_field("bits"))?,
            min_v: required_finite_filter_float("min_v", self.min_v)?,
            max_v: required_finite_filter_float("max_v", self.max_v)?,
            missing_code: match kind {
                AdcCodeDefectKind::MissingCode => self
                    .missing_code
                    .ok_or_else(|| missing_filter_field("missing_code"))?,
                _ => 0,
            },
            coefficients: match kind {
                AdcCodeDefectKind::Inl | AdcCodeDefectKind::Dnl => {
                    required_coefficients(self.coefficients.as_deref())?
                }
                AdcCodeDefectKind::MissingCode => Vec::new(),
            },
        }))
    }

    fn channel_arithmetic_filter(&self, kind: ChannelArithmeticKind) -> Result<FilterStep> {
        Ok(FilterStep::ChannelArithmetic(ChannelArithmeticTransform {
            kind,
            left_channel: required_filter_string("left_channel", self.left_channel.as_deref())?,
            right_channel: required_filter_string("right_channel", self.right_channel.as_deref())?,
            output_channel: required_filter_string(
                "output_channel",
                self.output_channel.as_deref(),
            )?,
            output_unit: self.output_unit.clone(),
        }))
    }

    fn vector_magnitude_filter(&self, kind: VectorMagnitudeKind) -> Result<FilterStep> {
        Ok(FilterStep::VectorMagnitude(VectorMagnitudeTransform {
            kind,
            channels: required_filter_strings("channels", self.channels.as_deref(), 2)?,
            output_channel: required_filter_string(
                "output_channel",
                self.output_channel.as_deref(),
            )?,
            output_unit: self.output_unit.clone(),
        }))
    }

    fn matrix_filter(&self) -> Result<FilterStep> {
        Ok(FilterStep::MatrixTransform(MatrixTransform {
            input_channels: required_filter_strings("channels", self.channels.as_deref(), 1)?,
            matrix: required_filter_matrix(self.matrix.as_deref())?,
            output_channels: required_filter_strings(
                "output_channels",
                self.output_channels.as_deref(),
                1,
            )?,
            output_unit: self.output_unit.clone(),
        }))
    }

    fn coordinate_rotation_filter(&self) -> Result<FilterStep> {
        Ok(FilterStep::CoordinateRotation(
            CoordinateRotationTransform {
                x_channel: required_filter_string("x_channel", self.x_channel.as_deref())?,
                y_channel: required_filter_string("y_channel", self.y_channel.as_deref())?,
                angle_rad: required_finite_filter_float("angle_rad", self.angle_rad)?,
                output_x_channel: required_filter_string(
                    "output_x_channel",
                    self.output_x_channel.as_deref(),
                )?,
                output_y_channel: required_filter_string(
                    "output_y_channel",
                    self.output_y_channel.as_deref(),
                )?,
                output_unit: self.output_unit.clone(),
            },
        ))
    }

    fn sensor_conversion_filter(&self, kind: SensorConversionKind) -> Result<FilterStep> {
        self.validate_sensor_fields(kind)?;
        Ok(FilterStep::sensor_conversion(SensorConversionTransform {
            kind,
            channel: required_filter_string("channel", self.channel.as_deref())?,
            output_channel: required_filter_string(
                "output_channel",
                self.output_channel.as_deref(),
            )?,
            output_unit: required_filter_string("output_unit", self.output_unit.as_deref())?,
            parameters: SensorConversionParameters {
                input_min_v: self.input_min_v,
                input_max_v: self.input_max_v,
                output_min: self.output_min,
                output_max: self.output_max,
                shunt_ohms: self.shunt_ohms,
                excitation_v: self.excitation_v,
                gauge_factor: self.gauge_factor,
                sensitivity_mv_v: self.sensitivity_mv_v,
                full_scale: self.full_scale,
                r0_ohm: self.r0_ohm,
                alpha_per_c: self.alpha_per_c,
                beta_k: self.beta_k,
                t0_c: self.t0_c,
                pulses_per_rev: self.pulses_per_rev,
                counts_per_rev: self.counts_per_rev,
                scale_per_rev: self.scale_per_rev,
                sensitivity_v_per_unit: self.sensitivity_v_per_unit,
                bias_v: self.bias_v,
                reference: self.reference,
                responsivity_a_per_w: self.responsivity_a_per_w,
            },
        }))
    }

    fn validate_sensor_fields(&self, kind: SensorConversionKind) -> Result<()> {
        match kind {
            SensorConversionKind::Linear | SensorConversionKind::Pressure => {
                required_finite_filter_float("input_min_v", self.input_min_v)?;
                required_finite_filter_float("input_max_v", self.input_max_v)?;
                required_finite_filter_float("output_min", self.output_min)?;
                required_finite_filter_float("output_max", self.output_max)?;
            }
            SensorConversionKind::CurrentShunt => {
                required_positive_filter_float("shunt_ohms", self.shunt_ohms)?;
            }
            SensorConversionKind::BridgeStrain => {
                required_positive_filter_float("excitation_v", self.excitation_v)?;
                required_positive_filter_float("gauge_factor", self.gauge_factor)?;
            }
            SensorConversionKind::LoadCell => {
                required_positive_filter_float("excitation_v", self.excitation_v)?;
                required_positive_filter_float("sensitivity_mv_v", self.sensitivity_mv_v)?;
                required_finite_filter_float("full_scale", self.full_scale)?;
            }
            SensorConversionKind::Rtd => {
                required_positive_filter_float("r0_ohm", self.r0_ohm)?;
                required_positive_filter_float("alpha_per_c", self.alpha_per_c)?;
            }
            SensorConversionKind::Thermistor => {
                required_positive_filter_float("r0_ohm", self.r0_ohm)?;
                required_positive_filter_float("beta_k", self.beta_k)?;
                required_finite_filter_float("t0_c", self.t0_c)?;
            }
            SensorConversionKind::TachometerRpm => {
                required_positive_filter_float("pulses_per_rev", self.pulses_per_rev)?;
            }
            SensorConversionKind::EncoderPosition => {
                required_positive_filter_float("counts_per_rev", self.counts_per_rev)?;
                required_finite_filter_float("scale_per_rev", self.scale_per_rev)?;
            }
            SensorConversionKind::Accelerometer
            | SensorConversionKind::Gyroscope
            | SensorConversionKind::HallCurrent
            | SensorConversionKind::LvdtPosition => {
                required_positive_filter_float(
                    "sensitivity_v_per_unit",
                    self.sensitivity_v_per_unit,
                )?;
                if let Some(bias_v) = self.bias_v {
                    required_finite_filter_float("bias_v", Some(bias_v))?;
                }
            }
            SensorConversionKind::MicrophoneSpl => {
                required_positive_filter_float("reference", self.reference)?;
            }
            SensorConversionKind::PhotodiodePower => {
                required_positive_filter_float("responsivity_a_per_w", self.responsivity_a_per_w)?;
            }
        }
        Ok(())
    }

    fn vibration_filter(&self, kind: VibrationTransformKind) -> Result<FilterStep> {
        Ok(FilterStep::VibrationTransform(VibrationTransform {
            kind,
            channel: required_filter_string("channel", self.channel.as_deref())?,
            output_channel: required_filter_string(
                "output_channel",
                self.output_channel.as_deref(),
            )?,
            output_unit: required_filter_string("output_unit", self.output_unit.as_deref())?,
            window_samples: match kind {
                VibrationTransformKind::VibrationSeverity => {
                    required_window_samples(self.window_samples)?
                }
                _ => self.window_samples.unwrap_or(1),
            },
        }))
    }

    fn control_filter(&self, kind: ControlTransformKind) -> Result<FilterStep> {
        self.validate_control_fields(kind)?;
        Ok(FilterStep::ControlTransform(ControlTransform {
            kind,
            channel: required_filter_string("channel", self.channel.as_deref())?,
            output_channel: required_filter_string(
                "output_channel",
                self.output_channel.as_deref(),
            )?,
            output_unit: self.output_unit.clone(),
            setpoint: self.setpoint.unwrap_or(0.0),
            kp: self.kp.unwrap_or(0.0),
            ki: self.ki.unwrap_or(0.0),
            kd: self.kd.unwrap_or(0.0),
            rate_limit_per_s: self.rate_limit_per_s.unwrap_or(0.0),
            min_v: self.min_v.unwrap_or(0.0),
            max_v: self.max_v.unwrap_or(0.0),
            threshold_v: self.threshold_v.unwrap_or(0.0),
            feedforward_gain: self.feedforward_gain.unwrap_or(0.0),
            feedforward_offset: self.feedforward_offset.unwrap_or(0.0),
        }))
    }

    fn validate_control_fields(&self, kind: ControlTransformKind) -> Result<()> {
        match kind {
            ControlTransformKind::ErrorSignal => {
                required_finite_filter_float("setpoint", self.setpoint)?;
            }
            ControlTransformKind::ProportionalControl => {
                required_finite_filter_float("setpoint", self.setpoint)?;
                required_finite_filter_float("kp", self.kp)?;
            }
            ControlTransformKind::PidControl => {
                required_finite_filter_float("setpoint", self.setpoint)?;
                required_finite_filter_float("kp", self.kp)?;
                required_finite_filter_float("ki", self.ki)?;
                required_finite_filter_float("kd", self.kd)?;
            }
            ControlTransformKind::RateLimiter | ControlTransformKind::SlewRateLimit => {
                required_positive_filter_float("rate_limit_per_s", self.rate_limit_per_s)?;
            }
            ControlTransformKind::ControlSaturation => {
                required_finite_filter_float("min_v", self.min_v)?;
                required_finite_filter_float("max_v", self.max_v)?;
            }
            ControlTransformKind::ControlDeadzone => {
                required_positive_filter_float("threshold_v", self.threshold_v)?;
            }
            ControlTransformKind::FeedforwardControl => {
                required_finite_filter_float("feedforward_gain", self.feedforward_gain)?;
                required_finite_filter_float("feedforward_offset", self.feedforward_offset)?;
            }
        }
        Ok(())
    }

    fn normalize_mode(&self) -> Result<NormalizeMode> {
        let mode = required_filter_string("mode", self.mode.as_deref())?;
        if mode == "range" {
            let input_min_v = required_finite_filter_float("input_min_v", self.input_min_v)?;
            let input_max_v = required_finite_filter_float("input_max_v", self.input_max_v)?;
            let output_min = required_finite_filter_float("output_min", self.output_min)?;
            let output_max = required_finite_filter_float("output_max", self.output_max)?;
            if input_max_v <= input_min_v {
                return Err(WaveformError::InvalidParameter {
                    name: "filters.input_max_v".to_string(),
                    reason: "must be greater than input_min_v".to_string(),
                });
            }
            if output_max <= output_min {
                return Err(WaveformError::InvalidParameter {
                    name: "filters.output_max".to_string(),
                    reason: "must be greater than output_min".to_string(),
                });
            }
            return Ok(NormalizeMode::Range {
                input_min_v,
                input_max_v,
                output_min,
                output_max,
            });
        }
        NormalizeMode::from_config(&mode).ok_or_else(|| WaveformError::InvalidParameter {
            name: "filters.mode".to_string(),
            reason: format!(
                "expected `zero_to_one`, `minus_one_to_one`, `z_score`, or `range`, got `{mode}`"
            ),
        })
    }
}

impl FeatureTransformConfig {
    fn to_feature_transform_step(&self) -> Result<FeatureTransformStep> {
        self.validate_common()?;
        match self.kind.as_str() {
            "rms" => Ok(FeatureTransformStep::Rms(RmsFeatureTransform {
                id: self.id.clone(),
                channel: self.channel.clone(),
            })),
            "peak_to_peak" => Ok(FeatureTransformStep::PeakToPeak(
                PeakToPeakFeatureTransform {
                    id: self.id.clone(),
                    channel: self.channel.clone(),
                },
            )),
            "crest_factor" => Ok(FeatureTransformStep::CrestFactor(
                CrestFactorFeatureTransform {
                    id: self.id.clone(),
                    channel: self.channel.clone(),
                },
            )),
            "energy" => Ok(FeatureTransformStep::Energy(EnergyFeatureTransform {
                id: self.id.clone(),
                channel: self.channel.clone(),
            })),
            "power" => Ok(FeatureTransformStep::Power(PowerFeatureTransform {
                id: self.id.clone(),
                channel: self.channel.clone(),
            })),
            "area_under_curve" => Ok(FeatureTransformStep::AreaUnderCurve(
                AreaUnderCurveFeatureTransform {
                    id: self.id.clone(),
                    channel: self.channel.clone(),
                },
            )),
            "impulse_estimate" => Ok(FeatureTransformStep::ImpulseEstimate(
                ImpulseEstimateFeatureTransform {
                    id: self.id.clone(),
                    channel: self.channel.clone(),
                },
            )),
            "mean" => Ok(FeatureTransformStep::Mean(MeanFeatureTransform {
                id: self.id.clone(),
                channel: self.channel.clone(),
            })),
            "median" => Ok(FeatureTransformStep::Median(MedianFeatureTransform {
                id: self.id.clone(),
                channel: self.channel.clone(),
            })),
            "mode" => Ok(FeatureTransformStep::Mode(ModeFeatureTransform {
                id: self.id.clone(),
                channel: self.channel.clone(),
            })),
            "min" => Ok(FeatureTransformStep::Minimum(MinimumFeatureTransform {
                id: self.id.clone(),
                channel: self.channel.clone(),
            })),
            "max" => Ok(FeatureTransformStep::Maximum(MaximumFeatureTransform {
                id: self.id.clone(),
                channel: self.channel.clone(),
            })),
            "variance" => Ok(FeatureTransformStep::Variance(VarianceFeatureTransform {
                id: self.id.clone(),
                channel: self.channel.clone(),
            })),
            "standard_deviation" => Ok(FeatureTransformStep::StandardDeviation(
                StandardDeviationFeatureTransform {
                    id: self.id.clone(),
                    channel: self.channel.clone(),
                },
            )),
            "skewness" => Ok(FeatureTransformStep::Skewness(SkewnessFeatureTransform {
                id: self.id.clone(),
                channel: self.channel.clone(),
            })),
            "kurtosis" => Ok(FeatureTransformStep::Kurtosis(KurtosisFeatureTransform {
                id: self.id.clone(),
                channel: self.channel.clone(),
            })),
            "percentile" => Ok(FeatureTransformStep::Percentile(
                PercentileFeatureTransform {
                    id: self.id.clone(),
                    channel: self.channel.clone(),
                    percentile: self.required_finite("percentile")?,
                },
            )),
            "quantile" => Ok(FeatureTransformStep::Quantile(QuantileFeatureTransform {
                id: self.id.clone(),
                channel: self.channel.clone(),
                quantile: self.required_finite("quantile")?,
            })),
            "histogram" => Ok(FeatureTransformStep::Histogram(HistogramFeatureTransform {
                id: self.id.clone(),
                channel: self.channel.clone(),
                bins: self.required_positive_usize("bins")?,
                min_v: self.min_v,
                max_v: self.max_v,
            })),
            "covariance" => Ok(FeatureTransformStep::Covariance(
                CovarianceFeatureTransform {
                    id: self.id.clone(),
                    channel: self.channel.clone(),
                    other_channel: self.required_string("other_channel")?,
                },
            )),
            "correlation" => Ok(FeatureTransformStep::Correlation(
                CorrelationFeatureTransform {
                    id: self.id.clone(),
                    channel: self.channel.clone(),
                    other_channel: self.required_string("other_channel")?,
                },
            )),
            "autocorrelation" => Ok(FeatureTransformStep::Autocorrelation(
                AutocorrelationFeatureTransform {
                    id: self.id.clone(),
                    channel: self.channel.clone(),
                    lag_samples: self.required_usize("lag_samples")?,
                },
            )),
            "cross_correlation" => Ok(FeatureTransformStep::CrossCorrelation(
                CrossCorrelationFeatureTransform {
                    id: self.id.clone(),
                    channel: self.channel.clone(),
                    other_channel: self.required_string("other_channel")?,
                    lag_samples: self.required_usize("lag_samples")?,
                },
            )),
            "window_function" => Ok(FeatureTransformStep::WindowFunction(
                WindowFunctionFeatureTransform {
                    id: self.id.clone(),
                    channel: self.channel.clone(),
                    window: self.window_spec()?,
                    window_samples: self.window_samples,
                },
            )),
            "dft" => Ok(FeatureTransformStep::Dft(SpectrumFeatureTransform {
                id: self.id.clone(),
                transform_name: "dft",
                channel: self.channel.clone(),
                window: self.window_spec()?,
            })),
            "fft" => Ok(FeatureTransformStep::Fft(SpectrumFeatureTransform {
                id: self.id.clone(),
                transform_name: "fft",
                channel: self.channel.clone(),
                window: self.window_spec()?,
            })),
            "ifft" => Ok(FeatureTransformStep::Ifft(IfftFeatureTransform {
                id: self.id.clone(),
                channel: self.channel.clone(),
                other_channel: self.other_channel.clone(),
            })),
            "power_spectrum" => Ok(FeatureTransformStep::PowerSpectrum(
                SpectrumFeatureTransform {
                    id: self.id.clone(),
                    transform_name: "power_spectrum",
                    channel: self.channel.clone(),
                    window: self.window_spec()?,
                },
            )),
            "psd" => Ok(FeatureTransformStep::Psd(SpectrumFeatureTransform {
                id: self.id.clone(),
                transform_name: "psd",
                channel: self.channel.clone(),
                window: self.window_spec()?,
            })),
            "welch_psd" => Ok(FeatureTransformStep::WelchPsd(WelchPsdFeatureTransform {
                id: self.id.clone(),
                channel: self.channel.clone(),
                window: self.window_spec()?,
                window_samples: self.required_positive_usize("window_samples")?,
                overlap_samples: self.overlap_samples.unwrap_or(0),
            })),
            "cross_spectrum" => Ok(FeatureTransformStep::CrossSpectrum(
                PairedSpectrumFeatureTransform {
                    id: self.id.clone(),
                    transform_name: "cross_spectrum",
                    channel: self.channel.clone(),
                    other_channel: self.required_string("other_channel")?,
                    window: self.window_spec()?,
                    window_samples: self.window_samples,
                    overlap_samples: self.overlap_samples,
                },
            )),
            "coherence" => Ok(FeatureTransformStep::Coherence(
                PairedSpectrumFeatureTransform {
                    id: self.id.clone(),
                    transform_name: "coherence",
                    channel: self.channel.clone(),
                    other_channel: self.required_string("other_channel")?,
                    window: self.window_spec()?,
                    window_samples: self.window_samples,
                    overlap_samples: self.overlap_samples,
                },
            )),
            "transfer_function" => Ok(FeatureTransformStep::TransferFunction(
                PairedSpectrumFeatureTransform {
                    id: self.id.clone(),
                    transform_name: "transfer_function",
                    channel: self.channel.clone(),
                    other_channel: self.required_string("other_channel")?,
                    window: self.window_spec()?,
                    window_samples: self.window_samples,
                    overlap_samples: self.overlap_samples,
                },
            )),
            "harmonic_analysis" => Ok(FeatureTransformStep::HarmonicAnalysis(
                HarmonicFeatureTransform {
                    id: self.id.clone(),
                    channel: self.channel.clone(),
                    window: self.window_spec()?,
                    fundamental_hz: self.optional_positive_finite("fundamental_hz")?,
                    harmonic_count: self.harmonic_count.unwrap_or(5),
                },
            )),
            "thd" => Ok(FeatureTransformStep::Thd(HarmonicMetricFeatureTransform {
                id: self.id.clone(),
                transform_name: "thd",
                channel: self.channel.clone(),
                window: self.window_spec()?,
                fundamental_hz: self.optional_positive_finite("fundamental_hz")?,
                harmonic_count: self.harmonic_count.unwrap_or(5),
            })),
            "snr" => Ok(FeatureTransformStep::Snr(HarmonicMetricFeatureTransform {
                id: self.id.clone(),
                transform_name: "snr",
                channel: self.channel.clone(),
                window: self.window_spec()?,
                fundamental_hz: self.optional_positive_finite("fundamental_hz")?,
                harmonic_count: self.harmonic_count.unwrap_or(5),
            })),
            "sinad" => Ok(FeatureTransformStep::Sinad(
                HarmonicMetricFeatureTransform {
                    id: self.id.clone(),
                    transform_name: "sinad",
                    channel: self.channel.clone(),
                    window: self.window_spec()?,
                    fundamental_hz: self.optional_positive_finite("fundamental_hz")?,
                    harmonic_count: self.harmonic_count.unwrap_or(5),
                },
            )),
            "enob" => Ok(FeatureTransformStep::Enob(HarmonicMetricFeatureTransform {
                id: self.id.clone(),
                transform_name: "enob",
                channel: self.channel.clone(),
                window: self.window_spec()?,
                fundamental_hz: self.optional_positive_finite("fundamental_hz")?,
                harmonic_count: self.harmonic_count.unwrap_or(5),
            })),
            "stft" => Ok(FeatureTransformStep::Stft(TimeFrequencyFeatureTransform {
                id: self.id.clone(),
                transform_name: "stft",
                channel: self.channel.clone(),
                window: self.window_spec()?,
                window_samples: self.required_positive_usize("window_samples")?,
                overlap_samples: self.overlap_samples.unwrap_or(0),
            })),
            "spectrogram" => Ok(FeatureTransformStep::Spectrogram(
                TimeFrequencyFeatureTransform {
                    id: self.id.clone(),
                    transform_name: "spectrogram",
                    channel: self.channel.clone(),
                    window: self.window_spec()?,
                    window_samples: self.required_positive_usize("window_samples")?,
                    overlap_samples: self.overlap_samples.unwrap_or(0),
                },
            )),
            "spectral_centroid" => Ok(FeatureTransformStep::SpectralCentroid(
                SpectrumFeatureTransform {
                    id: self.id.clone(),
                    transform_name: "spectral_centroid",
                    channel: self.channel.clone(),
                    window: self.window_spec()?,
                },
            )),
            "spectral_bandwidth" => Ok(FeatureTransformStep::SpectralBandwidth(
                SpectrumFeatureTransform {
                    id: self.id.clone(),
                    transform_name: "spectral_bandwidth",
                    channel: self.channel.clone(),
                    window: self.window_spec()?,
                },
            )),
            "spectral_rolloff" => Ok(FeatureTransformStep::SpectralRolloff(
                SpectralRolloffFeatureTransform {
                    id: self.id.clone(),
                    channel: self.channel.clone(),
                    window: self.window_spec()?,
                    rolloff_percent: self.rolloff_percent.unwrap_or(85.0),
                },
            )),
            "band_power" => Ok(FeatureTransformStep::BandPower(BandPowerFeatureTransform {
                id: self.id.clone(),
                channel: self.channel.clone(),
                window: self.window_spec()?,
                band_low_hz: self.required_finite("band_low_hz")?,
                band_high_hz: self.required_finite("band_high_hz")?,
            })),
            _ => Err(WaveformError::InvalidParameter {
                name: "feature_transforms.type".to_string(),
                reason: format!("unsupported feature transform type `{}`", self.kind),
            }),
        }
    }

    fn validate_common(&self) -> Result<()> {
        if self.id.trim().is_empty() {
            return Err(WaveformError::InvalidParameter {
                name: "feature_transforms.id".to_string(),
                reason: "must not be empty".to_string(),
            });
        }
        if self.channel.trim().is_empty() {
            return Err(WaveformError::InvalidParameter {
                name: "feature_transforms.channel".to_string(),
                reason: "must not be empty".to_string(),
            });
        }
        Ok(())
    }

    fn required_string(&self, field: &str) -> Result<String> {
        let value = match field {
            "other_channel" => self.other_channel.as_deref(),
            _ => None,
        }
        .ok_or_else(|| missing_feature_transform_field(field))?;
        if value.trim().is_empty() {
            return Err(WaveformError::InvalidParameter {
                name: format!("feature_transforms.{field}"),
                reason: "must not be empty".to_string(),
            });
        }
        Ok(value.to_string())
    }

    fn required_finite(&self, field: &str) -> Result<f64> {
        let value = match field {
            "percentile" => self.percentile,
            "quantile" => self.quantile,
            "band_low_hz" => self.band_low_hz,
            "band_high_hz" => self.band_high_hz,
            _ => None,
        }
        .ok_or_else(|| missing_feature_transform_field(field))?;
        if !value.is_finite() {
            return Err(WaveformError::InvalidParameter {
                name: format!("feature_transforms.{field}"),
                reason: "must be finite".to_string(),
            });
        }
        Ok(value)
    }

    fn required_usize(&self, field: &str) -> Result<usize> {
        match field {
            "lag_samples" => self.lag_samples,
            _ => None,
        }
        .ok_or_else(|| missing_feature_transform_field(field))
    }

    fn required_positive_usize(&self, field: &str) -> Result<usize> {
        let value = match field {
            "bins" => self.bins,
            "window_samples" => self.window_samples,
            _ => None,
        }
        .ok_or_else(|| missing_feature_transform_field(field))?;
        if value == 0 {
            return Err(WaveformError::InvalidParameter {
                name: format!("feature_transforms.{field}"),
                reason: "must be greater than zero".to_string(),
            });
        }
        Ok(value)
    }

    fn optional_positive_finite(&self, field: &str) -> Result<Option<f64>> {
        let value = match field {
            "fundamental_hz" => self.fundamental_hz,
            _ => None,
        };
        match value {
            Some(value) => {
                if !value.is_finite() || value <= 0.0 {
                    return Err(WaveformError::InvalidParameter {
                        name: format!("feature_transforms.{field}"),
                        reason: "must be finite and greater than zero".to_string(),
                    });
                }
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    fn window_spec(&self) -> Result<WindowSpec> {
        WindowSpec::from_config(
            self.window.as_deref(),
            self.window_beta,
            self.window_alpha,
            self.window_sigma,
        )
    }
}

impl EventTransformConfig {
    fn to_event_transform_step(&self) -> Result<EventTransformStep> {
        self.validate_common("event_transforms")?;
        match self.kind.as_str() {
            "schmitt_trigger" => {
                let on_threshold_v = self.required_finite("on_threshold_v")?;
                let off_threshold_v = self.required_finite("off_threshold_v")?;
                if off_threshold_v >= on_threshold_v {
                    return Err(WaveformError::InvalidParameter {
                        name: "event_transforms.off_threshold_v".to_string(),
                        reason: "must be lower than on_threshold_v".to_string(),
                    });
                }
                Ok(EventTransformStep::SchmittTrigger(
                    SchmittTriggerTransform {
                        id: self.id.clone(),
                        channel: self.channel.clone(),
                        on_threshold_v,
                        off_threshold_v,
                        initial_state: self.required_state("initial_state")?,
                    },
                ))
            }
            "debounce" => Ok(EventTransformStep::Debounce(DebounceTransform {
                id: self.id.clone(),
                channel: self.channel.clone(),
                min_duration_s: self.required_non_negative("min_duration_s")?,
            })),
            "glitch_removal" => Ok(EventTransformStep::GlitchRemoval(GlitchRemovalTransform {
                id: self.id.clone(),
                channel: self.channel.clone(),
                max_duration_s: self.required_non_negative("max_duration_s")?,
            })),
            "edge_extraction" => Ok(EventTransformStep::EdgeExtraction(
                EdgeExtractionTransform {
                    id: self.id.clone(),
                    channel: self.channel.clone(),
                },
            )),
            "bounce_detection" => Ok(EventTransformStep::BounceDetection(
                BounceDetectionTransform {
                    id: self.id.clone(),
                    channel: self.channel.clone(),
                    window_s: self.required_non_negative("window_s")?,
                },
            )),
            _ => Err(WaveformError::InvalidParameter {
                name: "event_transforms.type".to_string(),
                reason: format!("unsupported event transform type `{}`", self.kind),
            }),
        }
    }

    fn validate_common(&self, scope: &str) -> Result<()> {
        if self.id.trim().is_empty() {
            return Err(WaveformError::InvalidParameter {
                name: format!("{scope}.id"),
                reason: "must not be empty".to_string(),
            });
        }
        if self.channel.trim().is_empty() {
            return Err(WaveformError::InvalidParameter {
                name: format!("{scope}.channel"),
                reason: "must not be empty".to_string(),
            });
        }
        Ok(())
    }

    fn required_finite(&self, field: &str) -> Result<f64> {
        let value = match field {
            "on_threshold_v" => self.on_threshold_v,
            "off_threshold_v" => self.off_threshold_v,
            _ => None,
        }
        .ok_or_else(|| missing_event_transform_field(field))?;
        if value.is_finite() {
            Ok(value)
        } else {
            Err(WaveformError::InvalidParameter {
                name: format!("event_transforms.{field}"),
                reason: "must be finite".to_string(),
            })
        }
    }

    fn required_non_negative(&self, field: &str) -> Result<f64> {
        let value = match field {
            "min_duration_s" => self.min_duration_s,
            "max_duration_s" => self.max_duration_s,
            "window_s" => self.window_s,
            _ => None,
        }
        .ok_or_else(|| missing_event_transform_field(field))?;
        validate_finite_non_negative(&format!("event_transforms.{field}"), value)?;
        Ok(value)
    }

    fn required_state(&self, field: &str) -> Result<SignalState> {
        let value = match field {
            "initial_state" => self.initial_state.as_deref(),
            _ => None,
        }
        .ok_or_else(|| missing_event_transform_field(field))?;
        SignalState::from_config(value).ok_or_else(|| WaveformError::InvalidParameter {
            name: format!("event_transforms.{field}"),
            reason: format!("expected `high` or `low`, got `{value}`"),
        })
    }
}

impl EventValidationConfig {
    fn to_event_validation_step(&self) -> Result<EventValidationStep> {
        self.validate_common("event_validations")?;
        match self.kind.as_str() {
            "missing_pulse" => Ok(EventValidationStep::MissingPulse(MissingPulseValidation {
                id: self.id.clone(),
                channel: self.channel.clone(),
                direction: self.required_direction()?,
                expected_count: self.expected_count.unwrap_or(1),
            })),
            "extra_pulse" => Ok(EventValidationStep::ExtraPulse(ExtraPulseValidation {
                id: self.id.clone(),
                channel: self.channel.clone(),
                direction: self.required_direction()?,
                max_count: self
                    .max_count
                    .ok_or_else(|| missing_event_validation_field("max_count"))?,
            })),
            "dwell_time" => Ok(EventValidationStep::DwellTime(DwellTimeValidation {
                id: self.id.clone(),
                channel: self.channel.clone(),
                state: self.required_state("state")?,
                min_duration_s: self.required_non_negative("min_duration_s")?,
            })),
            "timeout" => Ok(EventValidationStep::Timeout(TimeoutValidation {
                id: self.id.clone(),
                channel: self.channel.clone(),
                direction: self.required_direction()?,
                start_time_s: self.start_time_s.unwrap_or(0.0),
                max_time_s: self.required_non_negative("max_time_s")?,
            })),
            _ => Err(WaveformError::InvalidParameter {
                name: "event_validations.type".to_string(),
                reason: format!("unsupported event validation type `{}`", self.kind),
            }),
        }
    }

    fn validate_common(&self, scope: &str) -> Result<()> {
        if self.id.trim().is_empty() {
            return Err(WaveformError::InvalidParameter {
                name: format!("{scope}.id"),
                reason: "must not be empty".to_string(),
            });
        }
        if self.channel.trim().is_empty() {
            return Err(WaveformError::InvalidParameter {
                name: format!("{scope}.channel"),
                reason: "must not be empty".to_string(),
            });
        }
        Ok(())
    }

    fn required_direction(&self) -> Result<EdgeDirectionFilter> {
        let value = self
            .direction
            .as_deref()
            .ok_or_else(|| missing_event_validation_field("direction"))?;
        EdgeDirectionFilter::from_config(value).ok_or_else(|| WaveformError::InvalidParameter {
            name: "event_validations.direction".to_string(),
            reason: format!("expected `rising`, `falling`, or `any`, got `{value}`"),
        })
    }

    fn required_state(&self, field: &str) -> Result<SignalState> {
        let value = match field {
            "state" => self.state.as_deref(),
            _ => None,
        }
        .ok_or_else(|| missing_event_validation_field(field))?;
        SignalState::from_config(value).ok_or_else(|| WaveformError::InvalidParameter {
            name: format!("event_validations.{field}"),
            reason: format!("expected `high` or `low`, got `{value}`"),
        })
    }

    fn required_non_negative(&self, field: &str) -> Result<f64> {
        let value = match field {
            "min_duration_s" => self.min_duration_s,
            "max_time_s" => self.max_time_s,
            _ => None,
        }
        .ok_or_else(|| missing_event_validation_field(field))?;
        validate_finite_non_negative(&format!("event_validations.{field}"), value)?;
        if let Some(start_time_s) = self.start_time_s {
            validate_finite_non_negative("event_validations.start_time_s", start_time_s)?;
        }
        Ok(value)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CriterionConfig {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub channel: String,
    pub measurement: Option<CriterionMeasurementConfig>,
    pub requirement: Option<CriterionRequirementConfig>,
    pub threshold_v: Option<f64>,
    pub expected_count: Option<usize>,
    pub state: Option<String>,
    pub expected_state: Option<String>,
    pub event_kind: Option<String>,
    pub min_width_s: Option<f64>,
    pub max_width_s: Option<f64>,
    pub max_duration_s: Option<f64>,
    pub min_duration_s: Option<f64>,
    pub low_threshold_v: Option<f64>,
    pub high_threshold_v: Option<f64>,
    pub direction: Option<String>,
    pub source_channel: Option<String>,
    pub source_threshold_v: Option<f64>,
    pub target_threshold_v: Option<f64>,
    pub source_state: Option<String>,
    pub expected_target_state: Option<String>,
    pub max_latency_s: Option<f64>,
    pub start_time_s: Option<f64>,
    pub end_time_s: Option<f64>,
    pub arm_after_first_expected_state: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CriterionMeasurementConfig {
    #[serde(rename = "type")]
    pub kind: String,
    pub threshold: Option<UnitValueConfig>,
    pub low_threshold: Option<UnitValueConfig>,
    pub high_threshold: Option<UnitValueConfig>,
    pub state: Option<String>,
    pub expected_state: Option<String>,
    pub event_kind: Option<String>,
    pub direction: Option<String>,
    pub selection: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CriterionRequirementConfig {
    pub operator: Option<String>,
    pub value: Option<f64>,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UnitValueConfig {
    pub value: f64,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CriterionConfigShape {
    Legacy,
    Dsl,
}

impl CriterionConfig {
    pub fn shape(&self) -> Result<CriterionConfigShape> {
        self.validate_schema()
    }

    fn validate_schema(&self) -> Result<CriterionConfigShape> {
        let has_legacy_type = self.kind.is_some();
        let has_dsl_measurement = self.measurement.is_some();
        let has_dsl_requirement = self.requirement.is_some();
        let has_dsl = has_dsl_measurement || has_dsl_requirement;
        let has_legacy_fields = self.has_legacy_specific_fields();

        if has_legacy_type && has_dsl {
            return Err(WaveformError::InvalidParameter {
                name: format!("criteria.{}", self.id),
                reason:
                    "legacy criteria using `type` cannot include DSL `measurement` or `requirement` sections"
                        .to_string(),
            });
        }

        if has_dsl {
            if has_legacy_fields {
                return Err(WaveformError::InvalidParameter {
                    name: format!("criteria.{}", self.id),
                    reason:
                        "DSL criteria cannot include legacy fields such as `threshold_v`, `state`, `direction`, or duration limits"
                            .to_string(),
                });
            }

            if !(has_dsl_measurement && has_dsl_requirement) {
                return Err(WaveformError::InvalidParameter {
                    name: format!("criteria.{}", self.id),
                    reason: "DSL criteria require both `measurement` and `requirement` sections"
                        .to_string(),
                });
            }

            let measurement = self
                .measurement
                .as_ref()
                .expect("DSL measurement section validated");
            let requirement = self
                .requirement
                .as_ref()
                .expect("DSL requirement section validated");
            let measurement_kind = measurement.validate(&self.id)?;
            requirement.validate(&self.id, measurement_kind.requirement_unit())?;
            let operator = CriterionOperator::from_config(
                requirement
                    .operator
                    .as_deref()
                    .expect("DSL operator validated"),
            )
            .expect("DSL operator validated");
            measurement.to_measurement_spec(&self.id, measurement_kind, operator)?;

            return Ok(CriterionConfigShape::Dsl);
        }

        if has_legacy_type {
            let kind = self.kind.as_deref().expect("legacy type checked");
            self.validate_legacy_kind(kind)?;
            return Ok(CriterionConfigShape::Legacy);
        }

        if has_legacy_fields {
            return Err(WaveformError::InvalidParameter {
                name: format!("criteria.{}", self.id),
                reason: "legacy criteria require a `type` field".to_string(),
            });
        }

        Err(WaveformError::InvalidParameter {
            name: format!("criteria.{}", self.id),
            reason:
                "criterion must include either legacy `type` or DSL `measurement` and `requirement` sections"
                    .to_string(),
        })
    }

    fn to_criterion(&self) -> Result<Criterion> {
        let kind = match self.validate_schema()? {
            CriterionConfigShape::Legacy => self.kind.as_deref().expect("legacy type validated"),
            CriterionConfigShape::Dsl => return self.to_measurement_criterion(),
        };

        match kind {
            "minimum_voltage" => Ok(Criterion::minimum_voltage(
                self.id.clone(),
                self.channel.clone(),
                self.required_f64("threshold_v")?,
            )),
            "maximum_voltage" => Ok(Criterion::maximum_voltage(
                self.id.clone(),
                self.channel.clone(),
                self.required_f64("threshold_v")?,
            )),
            "state_transitions" => Ok(Criterion::state_transitions(
                self.id.clone(),
                self.channel.clone(),
                self.required_f64("threshold_v")?,
                self.expected_count
                    .ok_or_else(|| missing_field("expected_count"))?,
            )),
            "pulse_width" => Ok(Criterion::pulse_width(
                self.id.clone(),
                self.channel.clone(),
                self.required_state("state")?,
                self.required_f64("threshold_v")?,
                self.min_width_s,
                self.max_width_s,
            )),
            "transient_duration" => Ok(Criterion::transient_duration(
                self.id.clone(),
                self.channel.clone(),
                self.required_state("expected_state")?,
                self.required_f64("threshold_v")?,
                self.required_f64("max_duration_s")?,
            )),
            "transient_event" => Ok(Criterion::transient_event_window(
                self.id.clone(),
                self.channel.clone(),
                self.transient_event_kind()?,
                self.required_state("expected_state")?,
                self.required_f64("threshold_v")?,
                self.required_f64("max_duration_s")?,
                TransientEventWindow {
                    start_time_s: self.start_time_s,
                    end_time_s: self.end_time_s,
                    arm_after_first_expected_state: self
                        .arm_after_first_expected_state
                        .unwrap_or(false),
                },
            )),
            "response_latency" | "state_transition_response" => Ok(Criterion::response_latency(
                self.id.clone(),
                ResponseLatencySpec {
                    source_channel: self.required_string("source_channel")?,
                    target_channel: self.channel.clone(),
                    source_threshold_v: self.required_f64("source_threshold_v")?,
                    target_threshold_v: self.required_f64("target_threshold_v")?,
                    source_state: self.required_state("source_state")?,
                    expected_target_state: self.required_state("expected_target_state")?,
                    max_latency_s: self.required_f64("max_latency_s")?,
                },
            )),
            "stable_state_duration" => Ok(Criterion::stable_state_duration(
                self.id.clone(),
                self.channel.clone(),
                self.required_state("state")?,
                self.required_f64("threshold_v")?,
                self.required_f64("min_duration_s")?,
            )),
            "rise_fall_time" => Ok(Criterion::rise_fall_time(
                self.id.clone(),
                self.channel.clone(),
                self.required_direction("direction")?,
                self.required_f64("low_threshold_v")?,
                self.required_f64("high_threshold_v")?,
                self.required_f64("max_duration_s")?,
            )),
            _ => Err(WaveformError::InvalidParameter {
                name: "criteria.type".to_string(),
                reason: format!("unsupported criterion type `{kind}`"),
            }),
        }
    }

    fn validate_legacy_kind(&self, kind: &str) -> Result<()> {
        match kind {
            "minimum_voltage" | "maximum_voltage" => {
                self.validate_required_finite("threshold_v")?;
            }
            "state_transitions" => {
                self.validate_required_finite("threshold_v")?;
                self.expected_count
                    .ok_or_else(|| missing_field("expected_count"))?;
            }
            "pulse_width" => {
                self.required_state("state")?;
                self.validate_required_finite("threshold_v")?;
                self.validate_optional_non_negative("min_width_s")?;
                self.validate_optional_non_negative("max_width_s")?;
                if self.min_width_s.is_none() && self.max_width_s.is_none() {
                    return Err(WaveformError::InvalidParameter {
                        name: "criteria.pulse_width".to_string(),
                        reason: "min_width_s or max_width_s is required".to_string(),
                    });
                }
            }
            "transient_duration" => {
                self.required_state("expected_state")?;
                self.validate_required_finite("threshold_v")?;
                self.validate_required_non_negative("max_duration_s")?;
            }
            "transient_event" => {
                self.transient_event_kind()?;
                self.required_state("expected_state")?;
                self.validate_required_finite("threshold_v")?;
                self.validate_required_non_negative("max_duration_s")?;
                self.validate_time_window()?;
            }
            "stable_state_duration" => {
                self.required_state("state")?;
                self.validate_required_finite("threshold_v")?;
                self.validate_required_non_negative("min_duration_s")?;
            }
            "rise_fall_time" => {
                self.required_direction("direction")?;
                let low_threshold_v = self.validate_required_finite("low_threshold_v")?;
                let high_threshold_v = self.validate_required_finite("high_threshold_v")?;
                if low_threshold_v >= high_threshold_v {
                    return Err(WaveformError::InvalidParameter {
                        name: "criteria.low_threshold_v".to_string(),
                        reason: "must be lower than high_threshold_v".to_string(),
                    });
                }
                self.validate_required_non_negative("max_duration_s")?;
            }
            "response_latency" | "state_transition_response" => {
                self.required_string("source_channel")?;
                self.required_state("source_state")?;
                self.required_state("expected_target_state")?;
                self.validate_required_finite("source_threshold_v")?;
                self.validate_required_finite("target_threshold_v")?;
                self.validate_required_non_negative("max_latency_s")?;
            }
            _ => {
                return Err(WaveformError::InvalidParameter {
                    name: "criteria.type".to_string(),
                    reason: format!("unsupported criterion type `{kind}`"),
                });
            }
        }

        Ok(())
    }

    fn validate_required_finite(&self, field: &str) -> Result<f64> {
        let value = self.required_f64(field)?;
        if value.is_finite() {
            Ok(value)
        } else {
            Err(WaveformError::InvalidParameter {
                name: format!("criteria.{field}"),
                reason: "must be finite".to_string(),
            })
        }
    }

    fn validate_required_non_negative(&self, field: &str) -> Result<f64> {
        let value = self.validate_required_finite(field)?;
        if value >= 0.0 {
            Ok(value)
        } else {
            Err(WaveformError::InvalidParameter {
                name: format!("criteria.{field}"),
                reason: "must be non-negative".to_string(),
            })
        }
    }

    fn validate_optional_non_negative(&self, field: &str) -> Result<()> {
        let value = match field {
            "min_width_s" => self.min_width_s,
            "max_width_s" => self.max_width_s,
            _ => None,
        };
        let Some(value) = value else {
            return Ok(());
        };

        if value.is_finite() && value >= 0.0 {
            Ok(())
        } else {
            Err(WaveformError::InvalidParameter {
                name: format!("criteria.{field}"),
                reason: "must be a finite non-negative value".to_string(),
            })
        }
    }

    fn validate_time_window(&self) -> Result<()> {
        if let Some(start_time_s) = self.start_time_s {
            if !start_time_s.is_finite() || start_time_s < 0.0 {
                return Err(WaveformError::InvalidParameter {
                    name: "criteria.start_time_s".to_string(),
                    reason: "must be a finite non-negative value".to_string(),
                });
            }
        }
        if let Some(end_time_s) = self.end_time_s {
            if !end_time_s.is_finite() || end_time_s < 0.0 {
                return Err(WaveformError::InvalidParameter {
                    name: "criteria.end_time_s".to_string(),
                    reason: "must be a finite non-negative value".to_string(),
                });
            }
        }
        if let (Some(start_time_s), Some(end_time_s)) = (self.start_time_s, self.end_time_s) {
            if end_time_s < start_time_s {
                return Err(WaveformError::InvalidParameter {
                    name: "criteria.end_time_s".to_string(),
                    reason: "must be greater than or equal to start_time_s".to_string(),
                });
            }
        }

        Ok(())
    }

    fn to_measurement_criterion(&self) -> Result<Criterion> {
        let measurement = self
            .measurement
            .as_ref()
            .expect("DSL measurement section validated");
        let requirement = self
            .requirement
            .as_ref()
            .expect("DSL requirement section validated");
        let measurement_kind = CriterionMeasurementKind::from_config(&measurement.kind)
            .expect("DSL measurement kind validated");
        let operator = CriterionOperator::from_config(
            requirement
                .operator
                .as_deref()
                .expect("DSL operator validated"),
        )
        .expect("DSL operator validated");

        let measurement = measurement.to_measurement_spec(&self.id, measurement_kind, operator)?;
        let requirement = MeasurementRequirement {
            operator,
            value: requirement.value.expect("DSL requirement value validated"),
        };

        Ok(Criterion::measurement(
            self.id.clone(),
            self.channel.clone(),
            measurement,
            requirement,
        ))
    }

    fn has_legacy_specific_fields(&self) -> bool {
        self.threshold_v.is_some()
            || self.expected_count.is_some()
            || self.state.is_some()
            || self.expected_state.is_some()
            || self.event_kind.is_some()
            || self.min_width_s.is_some()
            || self.max_width_s.is_some()
            || self.max_duration_s.is_some()
            || self.min_duration_s.is_some()
            || self.low_threshold_v.is_some()
            || self.high_threshold_v.is_some()
            || self.direction.is_some()
            || self.source_channel.is_some()
            || self.source_threshold_v.is_some()
            || self.target_threshold_v.is_some()
            || self.source_state.is_some()
            || self.expected_target_state.is_some()
            || self.max_latency_s.is_some()
            || self.start_time_s.is_some()
            || self.end_time_s.is_some()
            || self.arm_after_first_expected_state.is_some()
    }

    fn required_f64(&self, field: &str) -> Result<f64> {
        match field {
            "threshold_v" => self.threshold_v,
            "max_duration_s" => self.max_duration_s,
            "min_duration_s" => self.min_duration_s,
            "low_threshold_v" => self.low_threshold_v,
            "high_threshold_v" => self.high_threshold_v,
            "source_threshold_v" => self.source_threshold_v,
            "target_threshold_v" => self.target_threshold_v,
            "max_latency_s" => self.max_latency_s,
            _ => None,
        }
        .ok_or_else(|| missing_field(field))
    }

    fn required_string(&self, field: &str) -> Result<String> {
        match field {
            "source_channel" => self.source_channel.clone(),
            _ => None,
        }
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| missing_field(field))
    }

    fn required_state(&self, field: &str) -> Result<SignalState> {
        let value = match field {
            "state" => self.state.as_deref(),
            "expected_state" => self.expected_state.as_deref(),
            "source_state" => self.source_state.as_deref(),
            "expected_target_state" => self.expected_target_state.as_deref(),
            _ => None,
        }
        .ok_or_else(|| missing_field(field))?;

        SignalState::from_config(value).ok_or_else(|| WaveformError::InvalidParameter {
            name: format!("criteria.{field}"),
            reason: format!("expected `high` or `low`, got `{value}`"),
        })
    }

    fn required_direction(&self, field: &str) -> Result<EdgeDirection> {
        let value = self
            .direction
            .as_deref()
            .ok_or_else(|| missing_field(field))?;

        EdgeDirection::from_config(value).ok_or_else(|| WaveformError::InvalidParameter {
            name: format!("criteria.{field}"),
            reason: format!("expected `rise` or `fall`, got `{value}`"),
        })
    }

    fn transient_event_kind(&self) -> Result<TransientEventKind> {
        let value = self.event_kind.as_deref().unwrap_or("transient_event");

        TransientEventKind::from_config(value).ok_or_else(|| WaveformError::InvalidParameter {
            name: "criteria.event_kind".to_string(),
            reason: format!(
                "expected transient_event, spurious_transition, contact_bounce, dropout, noise_induced_transition, or threshold_crossing_event; got `{value}`"
            ),
        })
    }
}

impl CriterionMeasurementConfig {
    fn validate(&self, criterion_id: &str) -> Result<CriterionMeasurementKind> {
        let measurement_kind =
            CriterionMeasurementKind::from_config(&self.kind).ok_or_else(|| {
                WaveformError::InvalidParameter {
                    name: format!("criteria.{criterion_id}.measurement.type"),
                    reason: format!(
                        "unsupported measurement type `{}`; expected minimum_sample, maximum_sample, state_transition_count, pulse_width, stable_state_duration, transient_event_duration, rise_time, or fall_time",
                        self.kind
                    ),
                }
            })?;

        validate_optional_unit_value(
            &format!("criteria.{criterion_id}.measurement.threshold"),
            self.threshold.as_ref(),
            "V",
        )?;
        validate_optional_unit_value(
            &format!("criteria.{criterion_id}.measurement.low_threshold"),
            self.low_threshold.as_ref(),
            "V",
        )?;
        validate_optional_unit_value(
            &format!("criteria.{criterion_id}.measurement.high_threshold"),
            self.high_threshold.as_ref(),
            "V",
        )?;

        Ok(measurement_kind)
    }

    fn to_measurement_spec(
        &self,
        criterion_id: &str,
        measurement_kind: CriterionMeasurementKind,
        operator: CriterionOperator,
    ) -> Result<MeasurementSpec> {
        match measurement_kind {
            CriterionMeasurementKind::MinimumSample => Ok(MeasurementSpec::MinimumSample),
            CriterionMeasurementKind::MaximumSample => Ok(MeasurementSpec::MaximumSample),
            CriterionMeasurementKind::StateTransitionCount => {
                Ok(MeasurementSpec::StateTransitionCount {
                    threshold_v: self.required_threshold_value(criterion_id, "threshold")?,
                })
            }
            CriterionMeasurementKind::PulseWidth => Ok(MeasurementSpec::PulseWidth {
                state: self.required_state(criterion_id, "state")?,
                threshold_v: self.required_threshold_value(criterion_id, "threshold")?,
                selection: self.pulse_width_selection(criterion_id, operator)?,
            }),
            CriterionMeasurementKind::StableStateDuration => {
                self.validate_optional_selection(criterion_id, "longest")?;
                Ok(MeasurementSpec::StableStateDuration {
                    state: self.required_state(criterion_id, "state")?,
                    threshold_v: self.required_threshold_value(criterion_id, "threshold")?,
                })
            }
            CriterionMeasurementKind::TransientEventDuration => {
                self.validate_optional_selection(criterion_id, "longest")?;
                Ok(MeasurementSpec::TransientEventDuration {
                    event_kind: self.transient_event_kind(criterion_id)?,
                    expected_state: self.required_state(criterion_id, "expected_state")?,
                    threshold_v: self.required_threshold_value(criterion_id, "threshold")?,
                })
            }
            CriterionMeasurementKind::RiseTime => {
                let (low_threshold_v, high_threshold_v) =
                    self.validated_edge_thresholds(criterion_id)?;
                Ok(MeasurementSpec::RiseTime {
                    low_threshold_v,
                    high_threshold_v,
                })
            }
            CriterionMeasurementKind::FallTime => {
                let (low_threshold_v, high_threshold_v) =
                    self.validated_edge_thresholds(criterion_id)?;
                Ok(MeasurementSpec::FallTime {
                    low_threshold_v,
                    high_threshold_v,
                })
            }
        }
    }

    fn required_threshold_value(&self, criterion_id: &str, field: &str) -> Result<f64> {
        let value = match field {
            "threshold" => self.threshold.as_ref(),
            "low_threshold" => self.low_threshold.as_ref(),
            "high_threshold" => self.high_threshold.as_ref(),
            _ => None,
        }
        .ok_or_else(|| WaveformError::InvalidParameter {
            name: format!("criteria.{criterion_id}.measurement.{field}"),
            reason: "field is required for this measurement type".to_string(),
        })?;

        Ok(value.value)
    }

    fn validated_edge_thresholds(&self, criterion_id: &str) -> Result<(f64, f64)> {
        let low_threshold_v = self.required_threshold_value(criterion_id, "low_threshold")?;
        let high_threshold_v = self.required_threshold_value(criterion_id, "high_threshold")?;
        if low_threshold_v >= high_threshold_v {
            return Err(WaveformError::InvalidParameter {
                name: format!("criteria.{criterion_id}.measurement.low_threshold"),
                reason: "must be lower than high_threshold".to_string(),
            });
        }

        Ok((low_threshold_v, high_threshold_v))
    }

    fn required_state(&self, criterion_id: &str, field: &str) -> Result<SignalState> {
        let value = match field {
            "state" => self.state.as_deref(),
            "expected_state" => self.expected_state.as_deref(),
            _ => None,
        }
        .ok_or_else(|| WaveformError::InvalidParameter {
            name: format!("criteria.{criterion_id}.measurement.{field}"),
            reason: "field is required for this measurement type".to_string(),
        })?;

        SignalState::from_config(value).ok_or_else(|| WaveformError::InvalidParameter {
            name: format!("criteria.{criterion_id}.measurement.{field}"),
            reason: format!("expected `high` or `low`, got `{value}`"),
        })
    }

    fn pulse_width_selection(
        &self,
        criterion_id: &str,
        operator: CriterionOperator,
    ) -> Result<RunSelectionConfig> {
        if let Some(selection) = self.selection.as_deref() {
            return RunSelectionConfig::from_config(selection).ok_or_else(|| {
                WaveformError::InvalidParameter {
                    name: format!("criteria.{criterion_id}.measurement.selection"),
                    reason: format!("expected `shortest` or `longest`, got `{selection}`"),
                }
            });
        }

        match operator {
            CriterionOperator::GreaterThan | CriterionOperator::GreaterThanOrEqual => {
                Ok(RunSelectionConfig::Shortest)
            }
            CriterionOperator::LessThan | CriterionOperator::LessThanOrEqual => {
                Ok(RunSelectionConfig::Longest)
            }
            CriterionOperator::EqualTo => Err(WaveformError::InvalidParameter {
                name: format!("criteria.{criterion_id}.measurement.selection"),
                reason:
                    "field is required for equal_to pulse_width criteria; use `shortest` or `longest`"
                        .to_string(),
            }),
        }
    }

    fn validate_optional_selection(&self, criterion_id: &str, expected: &str) -> Result<()> {
        let Some(selection) = self.selection.as_deref() else {
            return Ok(());
        };

        if selection == expected {
            Ok(())
        } else {
            Err(WaveformError::InvalidParameter {
                name: format!("criteria.{criterion_id}.measurement.selection"),
                reason: format!("expected `{expected}`, got `{selection}`"),
            })
        }
    }

    fn transient_event_kind(&self, criterion_id: &str) -> Result<TransientEventKind> {
        let value = self.event_kind.as_deref().unwrap_or("transient_event");

        TransientEventKind::from_config(value).ok_or_else(|| WaveformError::InvalidParameter {
            name: format!("criteria.{criterion_id}.measurement.event_kind"),
            reason: format!(
                "expected transient_event, spurious_transition, contact_bounce, dropout, noise_induced_transition, or threshold_crossing_event; got `{value}`"
            ),
        })
    }
}

impl CriterionRequirementConfig {
    fn validate(&self, criterion_id: &str, expected_unit: &str) -> Result<()> {
        let operator = self
            .operator
            .as_deref()
            .ok_or_else(|| WaveformError::InvalidParameter {
                name: format!("criteria.{criterion_id}.requirement.operator"),
                reason: "field is required for DSL requirements".to_string(),
            })?;

        CriterionOperator::from_config(operator).ok_or_else(|| WaveformError::InvalidParameter {
            name: format!("criteria.{criterion_id}.requirement.operator"),
            reason: format!(
                "unsupported operator `{operator}`; expected less_than, less_than_or_equal, greater_than, greater_than_or_equal, or equal_to"
            ),
        })?;

        if self.value.is_none() {
            return Err(WaveformError::InvalidParameter {
                name: format!("criteria.{criterion_id}.requirement.value"),
                reason: "field is required for DSL requirements".to_string(),
            });
        }

        let unit = required_unit(
            &format!("criteria.{criterion_id}.requirement.unit"),
            self.unit.as_deref(),
        )?;
        validate_supported_unit(&format!("criteria.{criterion_id}.requirement.unit"), unit)?;
        validate_expected_unit(
            &format!("criteria.{criterion_id}.requirement.unit"),
            unit,
            expected_unit,
        )
    }
}

fn validate_optional_unit_value(
    field: &str,
    value: Option<&UnitValueConfig>,
    expected_unit: &str,
) -> Result<()> {
    let Some(value) = value else {
        return Ok(());
    };
    let unit = required_unit(&format!("{field}.unit"), value.unit.as_deref())?;
    validate_supported_unit(&format!("{field}.unit"), unit)?;
    validate_expected_unit(&format!("{field}.unit"), unit, expected_unit)
}

fn required_unit<'a>(name: &str, unit: Option<&'a str>) -> Result<&'a str> {
    unit.ok_or_else(|| WaveformError::InvalidParameter {
        name: name.to_string(),
        reason: "explicit unit field is required".to_string(),
    })
}

fn validate_supported_unit(name: &str, unit: &str) -> Result<()> {
    match unit {
        "V" | "s" | "count" => Ok(()),
        _ => Err(WaveformError::InvalidParameter {
            name: name.to_string(),
            reason: format!("unsupported unit `{unit}`; expected `V`, `s`, or `count`"),
        }),
    }
}

fn validate_expected_unit(name: &str, unit: &str, expected_unit: &str) -> Result<()> {
    if unit == expected_unit {
        Ok(())
    } else {
        Err(WaveformError::InvalidParameter {
            name: name.to_string(),
            reason: format!("expected unit `{expected_unit}`, got `{unit}`"),
        })
    }
}

fn missing_field(field: &str) -> WaveformError {
    WaveformError::InvalidParameter {
        name: format!("criteria.{field}"),
        reason: "field is required for this criterion type".to_string(),
    }
}

fn missing_filter_field(field: &str) -> WaveformError {
    WaveformError::InvalidParameter {
        name: format!("filters.{field}"),
        reason: "field is required for this filter type".to_string(),
    }
}

fn missing_feature_transform_field(field: &str) -> WaveformError {
    WaveformError::InvalidParameter {
        name: format!("feature_transforms.{field}"),
        reason: "field is required for this feature transform type".to_string(),
    }
}

fn required_finite_filter_float(field: &str, value: Option<f64>) -> Result<f64> {
    let value = value.ok_or_else(|| missing_filter_field(field))?;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(WaveformError::InvalidParameter {
            name: format!("filters.{field}"),
            reason: "must be finite".to_string(),
        })
    }
}

fn required_positive_filter_float(field: &str, value: Option<f64>) -> Result<f64> {
    let value = required_finite_filter_float(field, value)?;
    if value <= 0.0 {
        return Err(WaveformError::InvalidParameter {
            name: format!("filters.{field}"),
            reason: "must be greater than zero".to_string(),
        });
    }
    Ok(value)
}

fn required_unit_interval_filter_float(field: &str, value: Option<f64>) -> Result<f64> {
    let value = required_finite_filter_float(field, value)?;
    if !(0.0..=1.0).contains(&value) {
        return Err(WaveformError::InvalidParameter {
            name: format!("filters.{field}"),
            reason: "must be between zero and one".to_string(),
        });
    }
    Ok(value)
}

fn required_alpha(value: Option<f64>) -> Result<f64> {
    let alpha = required_finite_filter_float("alpha", value)?;
    if !(0.0..=1.0).contains(&alpha) || alpha == 0.0 {
        return Err(WaveformError::InvalidParameter {
            name: "filters.alpha".to_string(),
            reason: "must be greater than zero and less than or equal to one".to_string(),
        });
    }
    Ok(alpha)
}

fn required_window_samples(value: Option<usize>) -> Result<usize> {
    let window_samples = value.ok_or_else(|| missing_filter_field("window_samples"))?;
    if window_samples == 0 {
        return Err(WaveformError::InvalidParameter {
            name: "filters.window_samples".to_string(),
            reason: "must be greater than zero".to_string(),
        });
    }
    Ok(window_samples)
}

fn required_polynomial_order(value: Option<usize>) -> Result<usize> {
    let polynomial_order = value.ok_or_else(|| missing_filter_field("polynomial_order"))?;
    if polynomial_order > 5 {
        return Err(WaveformError::InvalidParameter {
            name: "filters.polynomial_order".to_string(),
            reason: "must be no greater than 5 for dependency-free fitting".to_string(),
        });
    }
    Ok(polynomial_order)
}

fn required_filter_string(field: &str, value: Option<&str>) -> Result<String> {
    let value = value.ok_or_else(|| missing_filter_field(field))?;
    if value.trim().is_empty() {
        return Err(WaveformError::InvalidParameter {
            name: format!("filters.{field}"),
            reason: "must not be empty".to_string(),
        });
    }
    Ok(value.to_string())
}

fn required_filter_strings(
    field: &str,
    value: Option<&[String]>,
    minimum_len: usize,
) -> Result<Vec<String>> {
    let value = value.ok_or_else(|| missing_filter_field(field))?;
    if value.len() < minimum_len {
        return Err(WaveformError::InvalidParameter {
            name: format!("filters.{field}"),
            reason: format!("must include at least {minimum_len} value(s)"),
        });
    }
    for item in value {
        if item.trim().is_empty() {
            return Err(WaveformError::InvalidParameter {
                name: format!("filters.{field}"),
                reason: "values must not be empty".to_string(),
            });
        }
    }
    Ok(value.to_vec())
}

fn required_filter_matrix(value: Option<&[Vec<f64>]>) -> Result<Vec<Vec<f64>>> {
    let matrix = value.ok_or_else(|| missing_filter_field("matrix"))?;
    if matrix.is_empty() {
        return Err(WaveformError::InvalidParameter {
            name: "filters.matrix".to_string(),
            reason: "must include at least one row".to_string(),
        });
    }
    for row in matrix {
        if row.is_empty() {
            return Err(WaveformError::InvalidParameter {
                name: "filters.matrix".to_string(),
                reason: "rows must include at least one coefficient".to_string(),
            });
        }
        if row.iter().any(|coefficient| !coefficient.is_finite()) {
            return Err(WaveformError::InvalidParameter {
                name: "filters.matrix".to_string(),
                reason: "coefficients must be finite".to_string(),
            });
        }
    }
    Ok(matrix.to_vec())
}

fn required_piecewise_points(
    value: Option<&[PiecewisePointConfig]>,
) -> Result<Vec<PiecewisePoint>> {
    let points = value.ok_or_else(|| missing_filter_field("points"))?;
    if points.len() < 2 {
        return Err(WaveformError::InvalidParameter {
            name: "filters.points".to_string(),
            reason: "must include at least two points".to_string(),
        });
    }
    for point in points {
        if !(point.x.is_finite() && point.y.is_finite()) {
            return Err(WaveformError::InvalidParameter {
                name: "filters.points".to_string(),
                reason: "x and y values must be finite".to_string(),
            });
        }
    }
    for pair in points.windows(2) {
        if pair[1].x <= pair[0].x {
            return Err(WaveformError::InvalidParameter {
                name: "filters.points".to_string(),
                reason: "x values must be strictly increasing".to_string(),
            });
        }
    }
    Ok(points
        .iter()
        .map(|point| PiecewisePoint {
            x: point.x,
            y: point.y,
        })
        .collect())
}

fn required_weights(value: Option<&[f64]>) -> Result<Vec<f64>> {
    let weights = value.ok_or_else(|| missing_filter_field("weights"))?;
    if weights.is_empty() {
        return Err(WaveformError::InvalidParameter {
            name: "filters.weights".to_string(),
            reason: "must include at least one weight".to_string(),
        });
    }
    for weight in weights {
        if !weight.is_finite() {
            return Err(WaveformError::InvalidParameter {
                name: "filters.weights".to_string(),
                reason: "all weights must be finite".to_string(),
            });
        }
        if *weight <= 0.0 {
            return Err(WaveformError::InvalidParameter {
                name: "filters.weights".to_string(),
                reason: "all weights must be greater than zero".to_string(),
            });
        }
    }
    Ok(weights.to_vec())
}

fn required_coefficients(value: Option<&[f64]>) -> Result<Vec<f64>> {
    let coefficients = value.ok_or_else(|| missing_filter_field("coefficients"))?;
    if coefficients.is_empty() {
        return Err(WaveformError::InvalidParameter {
            name: "filters.coefficients".to_string(),
            reason: "must include at least one coefficient".to_string(),
        });
    }
    if coefficients
        .iter()
        .any(|coefficient| !coefficient.is_finite())
    {
        return Err(WaveformError::InvalidParameter {
            name: "filters.coefficients".to_string(),
            reason: "all coefficients must be finite".to_string(),
        });
    }
    Ok(coefficients.to_vec())
}

fn required_biquad_coefficients(value: Option<&[f64]>) -> Result<BiquadCoefficients> {
    let coefficients = required_coefficients(value)?;
    BiquadCoefficients::from_slice(&coefficients).map_err(|error| match error {
        WaveformError::InvalidParameter { reason, .. } => WaveformError::InvalidParameter {
            name: "filters.coefficients".to_string(),
            reason,
        },
        other => other,
    })
}

fn required_delay_samples(value: Option<usize>) -> Result<usize> {
    let delay_samples = value.ok_or_else(|| missing_filter_field("delay_samples"))?;
    if delay_samples == 0 {
        return Err(WaveformError::InvalidParameter {
            name: "filters.delay_samples".to_string(),
            reason: "must be greater than zero".to_string(),
        });
    }
    Ok(delay_samples)
}

fn required_positive_filter_usize(field: &str, value: Option<usize>) -> Result<usize> {
    let value = value.ok_or_else(|| missing_filter_field(field))?;
    if value == 0 {
        return Err(WaveformError::InvalidParameter {
            name: format!("filters.{field}"),
            reason: "must be greater than zero".to_string(),
        });
    }
    Ok(value)
}

fn required_resampling_factor(field: &str, value: Option<usize>) -> Result<usize> {
    let value = required_positive_filter_usize(field, value)?;
    if value <= 1 {
        return Err(WaveformError::InvalidParameter {
            name: format!("filters.{field}"),
            reason: "must be greater than one".to_string(),
        });
    }
    Ok(value)
}

fn missing_event_transform_field(field: &str) -> WaveformError {
    WaveformError::InvalidParameter {
        name: format!("event_transforms.{field}"),
        reason: "field is required for this event transform type".to_string(),
    }
}

fn missing_event_validation_field(field: &str) -> WaveformError {
    WaveformError::InvalidParameter {
        name: format!("event_validations.{field}"),
        reason: "field is required for this event validation type".to_string(),
    }
}

fn validate_finite_non_negative(name: &str, value: f64) -> Result<()> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(WaveformError::InvalidParameter {
            name: name.to_string(),
            reason: "must be a finite non-negative value".to_string(),
        })
    }
}

fn default_time_unit() -> String {
    "s".to_string()
}

fn default_signal_unit() -> String {
    "V".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_config_to_csv_options_and_criteria() {
        let config = AnalysisConfig {
            input: InputConfig {
                time_column: "time".to_string(),
                channels: vec!["input_v".to_string()],
                time_unit: "s".to_string(),
                signal_unit: "V".to_string(),
            },
            metadata: MetadataContext::default(),
            tolerances: TolerancePolicy::default(),
            filters: vec![FilterConfig {
                kind: "moving_average".to_string(),
                window_samples: Some(2),
                cutoff_hz: None,
                offset_v: None,
                gain: None,
                threshold_v: None,
                baseline_v: None,
                bits: None,
                min_v: None,
                max_v: None,
                start_time_s: None,
                end_time_s: None,
                delay_s: None,
                sample_interval_s: None,
                channel: None,
                mode: None,
                base: None,
                limit_v: None,
                input_min_v: None,
                input_max_v: None,
                output_min: None,
                output_max: None,
                points: None,
                coefficients: None,
                weights: None,
                alpha: None,
                sigma_samples: None,
                polynomial_order: None,
                outlier_sigma: None,
                center_hz: None,
                q: None,
                delay_samples: None,
                feedback_gain: None,
                ripple_db: None,
                stopband_attenuation_db: None,
                factor: None,
                upsample_factor: None,
                downsample_factor: None,
                reference_channel: None,
                target_channel: None,
                max_lag_samples: None,
                time_constant_s: None,
                threshold_per_s: None,
                threshold_sigma: None,
                lower_quantile: None,
                upper_quantile: None,
                ..FilterConfig::default()
            }],
            feature_transforms: Vec::new(),
            event_transforms: Vec::new(),
            event_validations: Vec::new(),
            criteria: vec![CriterionConfig {
                id: "max".to_string(),
                kind: Some("maximum_voltage".to_string()),
                channel: "input_v".to_string(),
                measurement: None,
                requirement: None,
                threshold_v: Some(5.5),
                expected_count: None,
                state: None,
                expected_state: None,
                event_kind: None,
                min_width_s: None,
                max_width_s: None,
                max_duration_s: None,
                min_duration_s: None,
                low_threshold_v: None,
                high_threshold_v: None,
                direction: None,
                source_channel: None,
                source_threshold_v: None,
                target_threshold_v: None,
                source_state: None,
                expected_target_state: None,
                max_latency_s: None,
                start_time_s: None,
                end_time_s: None,
                arm_after_first_expected_state: None,
            }],
        };

        let options = config.csv_options();
        let filters = config.filters().expect("filters should convert");
        let criteria = config.criteria().expect("criteria should convert");

        assert_eq!(options.time_column, "time");
        assert_eq!(options.channel_columns, vec!["input_v"]);
        assert_eq!(options.time_unit, Unit::seconds());
        assert_eq!(options.signal_unit, Unit::volts());
        assert_eq!(config.tolerances, TolerancePolicy::default());
        assert_eq!(
            filters[0],
            FilterStep::MovingAverage(MovingAverageFilter { window_samples: 2 })
        );
        assert_eq!(criteria[0].id, "max");
    }

    #[test]
    fn converts_adc_quantizer_config_to_filter_step() {
        let config = FilterConfig {
            kind: "adc_quantize".to_string(),
            window_samples: None,
            cutoff_hz: None,
            offset_v: None,
            gain: None,
            threshold_v: None,
            baseline_v: None,
            bits: Some(12),
            min_v: Some(0.0),
            max_v: Some(5.0),
            start_time_s: None,
            end_time_s: None,
            delay_s: None,
            sample_interval_s: None,
            channel: None,
            mode: None,
            base: None,
            limit_v: None,
            input_min_v: None,
            input_max_v: None,
            output_min: None,
            output_max: None,
            points: None,
            coefficients: None,
            weights: None,
            alpha: None,
            sigma_samples: None,
            polynomial_order: None,
            outlier_sigma: None,
            center_hz: None,
            q: None,
            delay_samples: None,
            feedback_gain: None,
            ripple_db: None,
            stopband_attenuation_db: None,
            factor: None,
            upsample_factor: None,
            downsample_factor: None,
            reference_channel: None,
            target_channel: None,
            max_lag_samples: None,
            time_constant_s: None,
            threshold_per_s: None,
            threshold_sigma: None,
            lower_quantile: None,
            upper_quantile: None,
            ..FilterConfig::default()
        };

        let filter = config.to_filter_step().expect("filter should convert");

        assert_eq!(
            filter,
            FilterStep::AdcQuantize(AdcQuantizer {
                bits: 12,
                min_v: 0.0,
                max_v: 5.0,
            })
        );
    }

    #[test]
    fn legacy_filter_config_covers_current_transform_types() {
        let config: AnalysisConfig = toml::from_str(
            r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "moving_average"
window_samples = 2

[[filters]]
type = "low_pass"
cutoff_hz = 25.0

[[filters]]
type = "adc_quantize"
bits = 12
min_v = 0.0
max_v = 5.0

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
        )
        .expect("legacy filter config should deserialize");

        let filters = config.filters().expect("filters should convert");

        assert_eq!(
            filters,
            vec![
                FilterStep::MovingAverage(MovingAverageFilter { window_samples: 2 }),
                FilterStep::LowPass(LowPassFilter { cutoff_hz: 25.0 }),
                FilterStep::AdcQuantize(AdcQuantizer {
                    bits: 12,
                    min_v: 0.0,
                    max_v: 5.0,
                }),
            ]
        );
    }

    #[test]
    fn filter_config_covers_m11_transform_types() {
        let config: AnalysisConfig = toml::from_str(
            r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "offset"
offset_v = 0.5

[[filters]]
type = "gain"
gain = 2.0

[[filters]]
type = "invert"

[[filters]]
type = "clamp"
min_v = 0.0
max_v = 5.0

[[filters]]
type = "deadband"
threshold_v = 0.1

[[filters]]
type = "dc_remove"

[[filters]]
type = "baseline_subtract"
baseline_v = 1.25

[[filters]]
type = "moving_median"
window_samples = 3

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
        )
        .expect("M11 filter config should deserialize");

        let filters = config.filters().expect("filters should convert");

        assert_eq!(
            filters,
            vec![
                FilterStep::Offset(OffsetTransform { offset_v: 0.5 }),
                FilterStep::Gain(GainTransform { gain: 2.0 }),
                FilterStep::Invert(InvertTransform),
                FilterStep::Clamp(ClampTransform {
                    min_v: 0.0,
                    max_v: 5.0,
                }),
                FilterStep::Deadband(DeadbandTransform { threshold_v: 0.1 }),
                FilterStep::DcRemove(DcRemoveTransform),
                FilterStep::BaselineSubtract(BaselineSubtractTransform { baseline_v: 1.25 }),
                FilterStep::MovingMedian(MovingMedianFilter { window_samples: 3 }),
            ]
        );
    }

    #[test]
    fn filter_config_covers_m14_high_pass_baseline() {
        let config: AnalysisConfig = toml::from_str(
            r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "high_pass_baseline"
cutoff_hz = 0.5

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
        )
        .expect("M14 filter config should deserialize");

        let filters = config.filters().expect("filters should convert");

        assert_eq!(
            filters,
            vec![FilterStep::HighPassBaseline(HighPassBaselineFilter {
                cutoff_hz: 0.5
            })]
        );
    }

    #[test]
    fn filter_config_covers_m26_data_cleaning_and_timing_types() {
        let config: AnalysisConfig = toml::from_str(
            r#"
[input]
time_column = "time"
channels = ["input_v", "reference_v"]

[[filters]]
type = "timestamp_sort"

[[filters]]
type = "dedupe_timestamps"

[[filters]]
type = "nan_interpolate"

[[filters]]
type = "nan_remove"

[[filters]]
type = "crop"
start_time_s = 0.0
end_time_s = 0.4

[[filters]]
type = "fixed_delay"
delay_s = 0.01

[[filters]]
type = "gap_fill"
sample_interval_s = 0.1

[[filters]]
type = "resample_fixed"
sample_interval_s = 0.2

[[filters]]
type = "channel_delay"
channel = "reference_v"
delay_s = -0.01

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
        )
        .expect("M26 filter config should deserialize");

        let filters = config.filters().expect("filters should convert");

        assert_eq!(
            filters,
            vec![
                FilterStep::TimestampSort(TimestampSortTransform),
                FilterStep::DedupeTimestamps(DedupeTimestampsTransform),
                FilterStep::NanInterpolate(NanInterpolateTransform),
                FilterStep::NanRemove(NanRemoveTransform),
                FilterStep::Crop(CropTransform {
                    start_time_s: 0.0,
                    end_time_s: 0.4,
                }),
                FilterStep::FixedDelay(FixedDelayTransform { delay_s: 0.01 }),
                FilterStep::GapFill(GapFillTransform {
                    sample_interval_s: 0.1,
                }),
                FilterStep::ResampleFixed(ResampleFixedTransform {
                    sample_interval_s: 0.2,
                }),
                FilterStep::ChannelDelay(ChannelDelayTransform {
                    channel: "reference_v".to_string(),
                    delay_s: -0.01,
                }),
            ]
        );
    }

    #[test]
    fn filter_config_covers_m27_pointwise_normalization_and_nonlinear_types() {
        let config: AnalysisConfig = toml::from_str(
            r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "absolute_value"

[[filters]]
type = "square"

[[filters]]
type = "square_root"

[[filters]]
type = "log"
base = 10.0

[[filters]]
type = "exp"
base = 10.0

[[filters]]
type = "normalize"
mode = "zero_to_one"

[[filters]]
type = "normalize"
mode = "range"
input_min_v = 0.0
input_max_v = 10.0
output_min = -1.0
output_max = 1.0

[[filters]]
type = "tanh"

[[filters]]
type = "sigmoid"

[[filters]]
type = "soft_limit"
limit_v = 2.0

[[filters]]
type = "piecewise_linear"
points = [{ x = 0.0, y = 0.0 }, { x = 1.0, y = 2.0 }]

[[filters]]
type = "polynomial"
coefficients = [0.0, 1.0, 0.5]

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
        )
        .expect("M27 filter config should deserialize");

        let filters = config.filters().expect("filters should convert");

        assert_eq!(
            filters,
            vec![
                FilterStep::AbsoluteValue(AbsoluteValueTransform),
                FilterStep::Square(SquareTransform),
                FilterStep::SquareRoot(SquareRootTransform),
                FilterStep::Log(LogTransform { base: 10.0 }),
                FilterStep::Exp(ExpTransform { base: 10.0 }),
                FilterStep::Normalize(NormalizeTransform {
                    mode: NormalizeMode::ZeroToOne,
                }),
                FilterStep::Normalize(NormalizeTransform {
                    mode: NormalizeMode::Range {
                        input_min_v: 0.0,
                        input_max_v: 10.0,
                        output_min: -1.0,
                        output_max: 1.0,
                    },
                }),
                FilterStep::Tanh(TanhTransform),
                FilterStep::Sigmoid(SigmoidTransform),
                FilterStep::SoftLimit(SoftLimitTransform { limit_v: 2.0 }),
                FilterStep::PiecewiseLinear(PiecewiseLinearTransform {
                    points: vec![
                        PiecewisePoint { x: 0.0, y: 0.0 },
                        PiecewisePoint { x: 1.0, y: 2.0 },
                    ],
                }),
                FilterStep::Polynomial(PolynomialTransform {
                    coefficients: vec![0.0, 1.0, 0.5],
                }),
            ]
        );
    }

    #[test]
    fn filter_config_covers_m28_smoothing_baseline_types() {
        let config: AnalysisConfig = toml::from_str(
            r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "weighted_moving_average"
weights = [1.0, 2.0, 3.0]

[[filters]]
type = "exponential_moving_average"
alpha = 0.25

[[filters]]
type = "boxcar_smoothing"
window_samples = 3

[[filters]]
type = "gaussian_smoothing"
window_samples = 5
sigma_samples = 1.25

[[filters]]
type = "savitzky_golay"
window_samples = 5
polynomial_order = 2

[[filters]]
type = "centered_moving_median"
window_samples = 3

[[filters]]
type = "rolling_mean_baseline"
window_samples = 4

[[filters]]
type = "rolling_median_baseline"
window_samples = 5

[[filters]]
type = "linear_detrend"

[[filters]]
type = "polynomial_detrend"
polynomial_order = 2

[[filters]]
type = "hampel_filter"
window_samples = 3
outlier_sigma = 3.0

[[filters]]
type = "spike_remove"
window_samples = 3
threshold_v = 0.5

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
        )
        .expect("M28 filter config should deserialize");

        let filters = config.filters().expect("filters should convert");

        assert_eq!(
            filters,
            vec![
                FilterStep::WeightedMovingAverage(WeightedMovingAverageFilter {
                    weights: vec![1.0, 2.0, 3.0],
                }),
                FilterStep::ExponentialMovingAverage(ExponentialMovingAverageFilter {
                    alpha: 0.25,
                }),
                FilterStep::BoxcarSmoothing(BoxcarSmoothingFilter { window_samples: 3 }),
                FilterStep::GaussianSmoothing(GaussianSmoothingFilter {
                    window_samples: 5,
                    sigma_samples: 1.25,
                }),
                FilterStep::SavitzkyGolay(SavitzkyGolayFilter {
                    window_samples: 5,
                    polynomial_order: 2,
                }),
                FilterStep::CenteredMovingMedian(CenteredMovingMedianFilter { window_samples: 3 }),
                FilterStep::RollingMeanBaseline(RollingMeanBaselineTransform { window_samples: 4 }),
                FilterStep::RollingMedianBaseline(RollingMedianBaselineTransform {
                    window_samples: 5,
                }),
                FilterStep::LinearDetrend(LinearDetrendTransform),
                FilterStep::PolynomialDetrend(PolynomialDetrendTransform {
                    polynomial_order: 2,
                }),
                FilterStep::HampelFilter(HampelFilter {
                    window_samples: 3,
                    outlier_sigma: 3.0,
                }),
                FilterStep::SpikeRemove(SpikeRemoveTransform {
                    window_samples: 3,
                    threshold_v: 0.5,
                }),
            ]
        );
    }

    #[test]
    fn filter_config_covers_m29_standard_frequency_types() {
        let config: AnalysisConfig = toml::from_str(
            r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "fir_filter"
coefficients = [0.25, 0.5, 0.25]

[[filters]]
type = "zero_phase_fir_filter"
coefficients = [0.25, 0.5, 0.25]

[[filters]]
type = "iir_biquad"
coefficients = [1.0, 0.0, 0.0, 0.0, 0.0]

[[filters]]
type = "zero_phase_iir_biquad"
coefficients = [1.0, 0.0, 0.0, 0.0, 0.0]

[[filters]]
type = "high_pass"
cutoff_hz = 5.0

[[filters]]
type = "band_pass"
center_hz = 50.0
q = 2.0

[[filters]]
type = "band_stop"
center_hz = 60.0
q = 5.0

[[filters]]
type = "notch"
center_hz = 60.0
q = 30.0

[[filters]]
type = "comb_filter"
delay_samples = 2
feedback_gain = -0.5

[[filters]]
type = "butterworth_low_pass"
cutoff_hz = 100.0

[[filters]]
type = "butterworth_high_pass"
cutoff_hz = 5.0

[[filters]]
type = "chebyshev1_low_pass"
cutoff_hz = 100.0
ripple_db = 1.0

[[filters]]
type = "chebyshev2_low_pass"
cutoff_hz = 100.0
stopband_attenuation_db = 40.0

[[filters]]
type = "bessel_low_pass"
cutoff_hz = 100.0

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
        )
        .expect("M29 filter config should deserialize");

        let filters = config.filters().expect("filters should convert");
        let identity = BiquadCoefficients {
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
        };

        assert_eq!(
            filters,
            vec![
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
                    center_hz: 50.0,
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
                FilterStep::ButterworthLowPass(ButterworthLowPassFilter { cutoff_hz: 100.0 }),
                FilterStep::ButterworthHighPass(ButterworthHighPassFilter { cutoff_hz: 5.0 }),
                FilterStep::Chebyshev1LowPass(Chebyshev1LowPassFilter {
                    cutoff_hz: 100.0,
                    ripple_db: 1.0,
                }),
                FilterStep::Chebyshev2LowPass(Chebyshev2LowPassFilter {
                    cutoff_hz: 100.0,
                    stopband_attenuation_db: 40.0,
                }),
                FilterStep::BesselLowPass(BesselLowPassFilter { cutoff_hz: 100.0 }),
            ]
        );
    }

    #[test]
    fn filter_config_covers_m30_resampling_timing_types() {
        let config: AnalysisConfig = toml::from_str(
            r#"
[input]
time_column = "time"
channels = ["reference_v", "target_v"]

[[filters]]
type = "resample"
sample_interval_s = 0.001

[[filters]]
type = "downsample"
factor = 2

[[filters]]
type = "decimate"
factor = 2
cutoff_hz = 100.0

[[filters]]
type = "upsample"
factor = 2

[[filters]]
type = "interpolate"
sample_interval_s = 0.001

[[filters]]
type = "rational_resample"
upsample_factor = 3
downsample_factor = 2

[[filters]]
type = "sample_and_hold"
sample_interval_s = 0.001

[[filters]]
type = "zero_order_hold"
sample_interval_s = 0.001

[[filters]]
type = "first_order_hold"
sample_interval_s = 0.001

[[filters]]
type = "fractional_delay"
delay_s = 0.0005

[[filters]]
type = "cross_correlation_delay"
reference_channel = "reference_v"
target_channel = "target_v"
max_lag_samples = 2

[[filters]]
type = "jitter_correction"
sample_interval_s = 0.001

[[filters]]
type = "clock_drift_correction"
sample_interval_s = 0.001

[[criteria]]
id = "target_max"
type = "maximum_voltage"
channel = "target_v"
threshold_v = 5.5
"#,
        )
        .expect("M30 filter config should deserialize");

        let filters = config.filters().expect("filters should convert");

        assert_eq!(
            filters,
            vec![
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
            ]
        );
    }

    #[test]
    fn config_covers_m31_envelope_energy_calculus_types() {
        let config: AnalysisConfig = toml::from_str(
            r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "half_wave_rectify"

[[filters]]
type = "full_wave_rectify"

[[filters]]
type = "envelope"
alpha = 0.5

[[filters]]
type = "moving_rms"
window_samples = 3

[[filters]]
type = "peak_hold"

[[filters]]
type = "first_derivative"

[[filters]]
type = "second_derivative"

[[filters]]
type = "integral"

[[filters]]
type = "cumulative_integral"

[[filters]]
type = "leaky_integrator"
time_constant_s = 2.0

[[filters]]
type = "slope_detection"
threshold_per_s = 1.5

[[feature_transforms]]
id = "rms"
type = "rms"
channel = "input_v"

[[feature_transforms]]
id = "peak_to_peak"
type = "peak_to_peak"
channel = "input_v"

[[feature_transforms]]
id = "crest_factor"
type = "crest_factor"
channel = "input_v"

[[feature_transforms]]
id = "energy"
type = "energy"
channel = "input_v"

[[feature_transforms]]
id = "power"
type = "power"
channel = "input_v"

[[feature_transforms]]
id = "area"
type = "area_under_curve"
channel = "input_v"

[[feature_transforms]]
id = "impulse"
type = "impulse_estimate"
channel = "input_v"
"#,
        )
        .expect("M31 config should deserialize");

        assert_eq!(
            config.filters().expect("filters should convert"),
            vec![
                FilterStep::HalfWaveRectify(HalfWaveRectifyTransform),
                FilterStep::FullWaveRectify(FullWaveRectifyTransform),
                FilterStep::Envelope(EnvelopeTransform { alpha: 0.5 }),
                FilterStep::MovingRms(MovingRmsTransform { window_samples: 3 }),
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
            ]
        );
        assert_eq!(
            config
                .feature_transforms()
                .expect("feature transforms should convert"),
            vec![
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
            ]
        );
    }

    #[test]
    fn config_covers_m32_statistics_correlation_types() {
        let config: AnalysisConfig = toml::from_str(
            r#"
[input]
time_column = "time"
channels = ["input_v", "other_v"]

[[filters]]
type = "rolling_mean"
window_samples = 3

[[filters]]
type = "rolling_variance"
window_samples = 3

[[filters]]
type = "rolling_stddev"
window_samples = 3

[[filters]]
type = "rolling_min"
window_samples = 3

[[filters]]
type = "rolling_max"
window_samples = 3

[[filters]]
type = "z_score"

[[filters]]
type = "outlier_detection"
threshold_sigma = 2.5

[[filters]]
type = "quantile_clip"
lower_quantile = 0.1
upper_quantile = 0.9

[[feature_transforms]]
id = "mean"
type = "mean"
channel = "input_v"

[[feature_transforms]]
id = "median"
type = "median"
channel = "input_v"

[[feature_transforms]]
id = "mode"
type = "mode"
channel = "input_v"

[[feature_transforms]]
id = "min"
type = "min"
channel = "input_v"

[[feature_transforms]]
id = "max"
type = "max"
channel = "input_v"

[[feature_transforms]]
id = "variance"
type = "variance"
channel = "input_v"

[[feature_transforms]]
id = "standard_deviation"
type = "standard_deviation"
channel = "input_v"

[[feature_transforms]]
id = "skewness"
type = "skewness"
channel = "input_v"

[[feature_transforms]]
id = "kurtosis"
type = "kurtosis"
channel = "input_v"

[[feature_transforms]]
id = "percentile"
type = "percentile"
channel = "input_v"
percentile = 75.0

[[feature_transforms]]
id = "quantile"
type = "quantile"
channel = "input_v"
quantile = 0.25

[[feature_transforms]]
id = "histogram"
type = "histogram"
channel = "input_v"
bins = 3
min_v = 0.0
max_v = 6.0

[[feature_transforms]]
id = "covariance"
type = "covariance"
channel = "input_v"
other_channel = "other_v"

[[feature_transforms]]
id = "correlation"
type = "correlation"
channel = "input_v"
other_channel = "other_v"

[[feature_transforms]]
id = "autocorrelation"
type = "autocorrelation"
channel = "input_v"
lag_samples = 1

[[feature_transforms]]
id = "cross_correlation"
type = "cross_correlation"
channel = "input_v"
other_channel = "other_v"
lag_samples = 1
"#,
        )
        .expect("M32 config should deserialize");

        assert_eq!(
            config.filters().expect("filters should convert"),
            vec![
                FilterStep::RollingMean(RollingMeanTransform { window_samples: 3 }),
                FilterStep::RollingVariance(RollingVarianceTransform { window_samples: 3 }),
                FilterStep::RollingStdDev(RollingStdDevTransform { window_samples: 3 }),
                FilterStep::RollingMin(RollingMinTransform { window_samples: 3 }),
                FilterStep::RollingMax(RollingMaxTransform { window_samples: 3 }),
                FilterStep::ZScore(ZScoreTransform),
                FilterStep::OutlierDetection(OutlierDetectionTransform {
                    threshold_sigma: 2.5,
                }),
                FilterStep::QuantileClip(QuantileClipTransform {
                    lower_quantile: 0.1,
                    upper_quantile: 0.9,
                }),
            ]
        );
        assert_eq!(
            config
                .feature_transforms()
                .expect("feature transforms should convert"),
            vec![
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
                    id: "standard_deviation".to_string(),
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
            ]
        );
    }

    #[test]
    fn config_covers_m33_spectrum_time_frequency_types() {
        let config: AnalysisConfig = toml::from_str(
            r#"
[input]
time_column = "time"
channels = ["input_v", "other_v", "constant_spectrum_imag"]

[[feature_transforms]]
id = "window"
type = "window_function"
channel = "input_v"
window = "hann"
window_samples = 4

[[feature_transforms]]
id = "dft"
type = "dft"
channel = "input_v"

[[feature_transforms]]
id = "fft"
type = "fft"
channel = "input_v"

[[feature_transforms]]
id = "ifft"
type = "ifft"
channel = "input_v"
other_channel = "constant_spectrum_imag"

[[feature_transforms]]
id = "power"
type = "power_spectrum"
channel = "input_v"

[[feature_transforms]]
id = "psd"
type = "psd"
channel = "input_v"

[[feature_transforms]]
id = "welch"
type = "welch_psd"
channel = "input_v"
window_samples = 8
overlap_samples = 0

[[feature_transforms]]
id = "cross"
type = "cross_spectrum"
channel = "input_v"
other_channel = "other_v"

[[feature_transforms]]
id = "coherence"
type = "coherence"
channel = "input_v"
other_channel = "other_v"

[[feature_transforms]]
id = "transfer"
type = "transfer_function"
channel = "input_v"
other_channel = "other_v"

[[feature_transforms]]
id = "harmonics"
type = "harmonic_analysis"
channel = "input_v"
fundamental_hz = 1.0
harmonic_count = 3

[[feature_transforms]]
id = "thd"
type = "thd"
channel = "input_v"
fundamental_hz = 1.0
harmonic_count = 3

[[feature_transforms]]
id = "snr"
type = "snr"
channel = "input_v"
fundamental_hz = 1.0
harmonic_count = 3

[[feature_transforms]]
id = "sinad"
type = "sinad"
channel = "input_v"
fundamental_hz = 1.0
harmonic_count = 3

[[feature_transforms]]
id = "enob"
type = "enob"
channel = "input_v"
fundamental_hz = 1.0
harmonic_count = 3

[[feature_transforms]]
id = "stft"
type = "stft"
channel = "input_v"
window_samples = 8
overlap_samples = 0

[[feature_transforms]]
id = "spectrogram"
type = "spectrogram"
channel = "input_v"
window_samples = 8
overlap_samples = 0

[[feature_transforms]]
id = "centroid"
type = "spectral_centroid"
channel = "input_v"

[[feature_transforms]]
id = "bandwidth"
type = "spectral_bandwidth"
channel = "input_v"

[[feature_transforms]]
id = "rolloff"
type = "spectral_rolloff"
channel = "input_v"
rolloff_percent = 85.0

[[feature_transforms]]
id = "band_power"
type = "band_power"
channel = "input_v"
band_low_hz = 0.5
band_high_hz = 1.5
"#,
        )
        .expect("M33 config should deserialize");

        assert_eq!(
            config
                .feature_transforms()
                .expect("feature transforms should convert"),
            vec![
                FeatureTransformStep::WindowFunction(WindowFunctionFeatureTransform {
                    id: "window".to_string(),
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
                FeatureTransformStep::Ifft(IfftFeatureTransform {
                    id: "ifft".to_string(),
                    channel: "input_v".to_string(),
                    other_channel: Some("constant_spectrum_imag".to_string()),
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
                    id: "harmonics".to_string(),
                    channel: "input_v".to_string(),
                    window: WindowSpec::default(),
                    fundamental_hz: Some(1.0),
                    harmonic_count: 3,
                }),
                FeatureTransformStep::Thd(HarmonicMetricFeatureTransform {
                    id: "thd".to_string(),
                    transform_name: "thd",
                    channel: "input_v".to_string(),
                    window: WindowSpec::default(),
                    fundamental_hz: Some(1.0),
                    harmonic_count: 3,
                }),
                FeatureTransformStep::Snr(HarmonicMetricFeatureTransform {
                    id: "snr".to_string(),
                    transform_name: "snr",
                    channel: "input_v".to_string(),
                    window: WindowSpec::default(),
                    fundamental_hz: Some(1.0),
                    harmonic_count: 3,
                }),
                FeatureTransformStep::Sinad(HarmonicMetricFeatureTransform {
                    id: "sinad".to_string(),
                    transform_name: "sinad",
                    channel: "input_v".to_string(),
                    window: WindowSpec::default(),
                    fundamental_hz: Some(1.0),
                    harmonic_count: 3,
                }),
                FeatureTransformStep::Enob(HarmonicMetricFeatureTransform {
                    id: "enob".to_string(),
                    transform_name: "enob",
                    channel: "input_v".to_string(),
                    window: WindowSpec::default(),
                    fundamental_hz: Some(1.0),
                    harmonic_count: 3,
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
            ]
        );
    }

    #[test]
    fn config_covers_m34_fault_injection_adc_dac_types() {
        let config: AnalysisConfig = toml::from_str(
            r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "white_noise"
amplitude_v = 0.1
seed = 1

[[filters]]
type = "gaussian_noise"
stddev_v = 0.1
seed = 2

[[filters]]
type = "uniform_noise"
min_v = -0.1
max_v = 0.1
seed = 3

[[filters]]
type = "pink_noise"
amplitude_v = 0.1
seed = 4

[[filters]]
type = "brown_noise"
amplitude_v = 0.1
seed = 5

[[filters]]
type = "impulse_noise"
amplitude_v = 1.0
probability = 0.25
seed = 6

[[filters]]
type = "salt_pepper_noise"
min_v = 0.0
max_v = 5.0
probability = 0.25
seed = 7

[[filters]]
type = "quantization_noise"
lsb_v = 0.05
seed = 8

[[filters]]
type = "periodic_interference"
amplitude_v = 0.1
frequency_hz = 10.0
phase_rad = 0.0

[[filters]]
type = "hum_interference"
amplitude_v = 0.1
frequency_hz = 60.0

[[filters]]
type = "ground_bounce"
amplitude_v = 0.1
interval_samples = 2

[[filters]]
type = "thermal_drift"
drift_rate_v_per_s = 0.01

[[filters]]
type = "random_walk_drift"
amplitude_v = 0.01
seed = 9

[[filters]]
type = "dropout_fault"
fault_value_v = 0.0
probability = 0.2
seed = 10

[[filters]]
type = "missing_samples"
fault_value_v = -999.0
probability = 0.2
seed = 11

[[filters]]
type = "saturation_fault"
min_v = 0.0
max_v = 5.0

[[filters]]
type = "stuck_at_fault"
fault_value_v = 2.5
start_index = 1
duration_samples = 2

[[filters]]
type = "flatline_fault"
start_index = 1

[[filters]]
type = "intermittent_fault"
fault_value_v = 0.0
probability = 0.2
seed = 12

[[filters]]
type = "rounding_quantizer"
lsb_v = 0.1

[[filters]]
type = "floor_quantizer"
lsb_v = 0.1

[[filters]]
type = "ceil_quantizer"
lsb_v = 0.1

[[filters]]
type = "midrise_quantizer"
lsb_v = 0.1

[[filters]]
type = "midtread_quantizer"
lsb_v = 0.1

[[filters]]
type = "saturating_quantizer"
min_v = 0.0
max_v = 5.0

[[filters]]
type = "dither"
lsb_v = 0.1
seed = 13

[[filters]]
type = "companding"
mode = "mu_law"
max_v = 5.0
mu = 255.0

[[filters]]
type = "sample_clock_jitter"
jitter_s = 0.0001
seed = 14

[[filters]]
type = "adc_missing_code"
bits = 4
min_v = 0.0
max_v = 5.0
missing_code = 3

[[filters]]
type = "inl_error"
bits = 4
min_v = 0.0
max_v = 5.0
coefficients = [0.0, 0.01]

[[filters]]
type = "dnl_error"
bits = 4
min_v = 0.0
max_v = 5.0
coefficients = [0.0, -0.01]

[[filters]]
type = "adc_gain_error"
gain_error = 0.01

[[filters]]
type = "adc_offset_error"
offset_error_v = 0.02
"#,
        )
        .expect("M34 filter config should deserialize");

        let filters = config.filters().expect("filters should convert");

        assert_eq!(filters.len(), 33);
        assert!(matches!(
            filters.first(),
            Some(FilterStep::NoiseInjection(NoiseInjectionTransform {
                kind: NoiseKind::White,
                seed: 1,
                ..
            }))
        ));
        assert!(matches!(
            filters.get(10),
            Some(FilterStep::DriftFault(DriftFaultTransform {
                kind: DriftFaultKind::GroundBounce,
                interval_samples: 2,
                ..
            }))
        ));
        assert!(matches!(
            filters.get(24),
            Some(FilterStep::SimulationQuantizer(
                SimulationQuantizerTransform {
                    kind: SimulationQuantizerKind::Saturating,
                    ..
                }
            ))
        ));
        assert!(matches!(
            filters.last(),
            Some(FilterStep::GainOffsetError(GainOffsetErrorTransform {
                kind: GainOffsetErrorKind::Offset,
                offset_error_v: 0.02,
                ..
            }))
        ));
    }

    #[test]
    fn config_covers_m35_multi_channel_sensor_domain_types() {
        let config: AnalysisConfig = toml::from_str(
            r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "channel_add"
left_channel = "a_v"
right_channel = "b_v"
output_channel = "sum_v"

[[filters]]
type = "channel_subtract"
left_channel = "a_v"
right_channel = "b_v"
output_channel = "sub_v"

[[filters]]
type = "differential_channel"
left_channel = "a_v"
right_channel = "b_v"
output_channel = "diff_v"

[[filters]]
type = "common_mode"
left_channel = "a_v"
right_channel = "b_v"
output_channel = "common_v"

[[filters]]
type = "vector_magnitude"
channels = ["x_v", "y_v", "z_v"]
output_channel = "vector_mag"

[[filters]]
type = "euclidean_norm"
channels = ["x_v", "y_v"]
output_channel = "norm_v"

[[filters]]
type = "matrix_transform"
channels = ["x_v", "y_v"]
matrix = [[1.0, 0.0], [0.0, 1.0]]
output_channels = ["mix_x", "mix_y"]

[[filters]]
type = "coordinate_rotation"
x_channel = "x_v"
y_channel = "y_v"
angle_rad = 1.57079632679
output_x_channel = "rot_x"
output_y_channel = "rot_y"

[[filters]]
type = "linear_sensor_conversion"
channel = "input_v"
output_channel = "linear_units"
output_unit = "unit"
input_min_v = 0.0
input_max_v = 5.0
output_min = 0.0
output_max = 100.0

[[filters]]
type = "pressure_transducer"
channel = "pressure_v"
output_channel = "pressure_kpa"
output_unit = "kPa"
input_min_v = 0.0
input_max_v = 5.0
output_min = 0.0
output_max = 100.0

[[filters]]
type = "current_shunt"
channel = "shunt_v"
output_channel = "current_a"
output_unit = "A"
shunt_ohms = 0.1

[[filters]]
type = "bridge_strain"
channel = "bridge_v"
output_channel = "strain"
output_unit = "strain"
excitation_v = 5.0
gauge_factor = 2.0

[[filters]]
type = "load_cell_force"
channel = "bridge_v"
output_channel = "force_n"
output_unit = "N"
excitation_v = 5.0
sensitivity_mv_v = 2.0
full_scale = 100.0

[[filters]]
type = "rtd_temperature"
channel = "rtd_ohm"
output_channel = "rtd_c"
output_unit = "C"
r0_ohm = 100.0
alpha_per_c = 0.00385

[[filters]]
type = "thermistor_temperature"
channel = "thermistor_ohm"
output_channel = "thermistor_c"
output_unit = "C"
r0_ohm = 10000.0
beta_k = 3950.0
t0_c = 25.0

[[filters]]
type = "tachometer_rpm"
channel = "frequency_hz"
output_channel = "rpm"
output_unit = "rpm"
pulses_per_rev = 2.0

[[filters]]
type = "encoder_position"
channel = "encoder_counts"
output_channel = "angle_rad"
output_unit = "rad"
counts_per_rev = 1024.0
scale_per_rev = 6.28318530718

[[filters]]
type = "accelerometer_units"
channel = "accel_v"
output_channel = "accel_g"
output_unit = "g"
sensitivity_v_per_unit = 0.1
bias_v = 2.5

[[filters]]
type = "gyroscope_rate"
channel = "gyro_v"
output_channel = "gyro_deg_s"
output_unit = "deg/s"
sensitivity_v_per_unit = 0.02
bias_v = 2.5

[[filters]]
type = "hall_current"
channel = "hall_v"
output_channel = "hall_a"
output_unit = "A"
sensitivity_v_per_unit = 0.04
bias_v = 2.5

[[filters]]
type = "lvdt_position"
channel = "lvdt_v"
output_channel = "position_mm"
output_unit = "mm"
sensitivity_v_per_unit = 0.5
bias_v = 0.0

[[filters]]
type = "microphone_spl"
channel = "pressure_pa"
output_channel = "spl_db"
output_unit = "dB"
reference = 0.00002

[[filters]]
type = "photodiode_power"
channel = "photodiode_a"
output_channel = "optical_w"
output_unit = "W"
responsivity_a_per_w = 0.4

[[filters]]
type = "velocity_from_acceleration"
channel = "accel_m_s2"
output_channel = "velocity_m_s"
output_unit = "m/s"

[[filters]]
type = "displacement_from_velocity"
channel = "velocity_m_s"
output_channel = "displacement_m"
output_unit = "m"

[[filters]]
type = "vibration_severity"
channel = "accel_m_s2"
output_channel = "severity"
output_unit = "m/s^2"
window_samples = 4

[[filters]]
type = "control_error"
channel = "measured_v"
output_channel = "error_v"
setpoint = 5.0

[[filters]]
type = "proportional_control"
channel = "measured_v"
output_channel = "p_out"
setpoint = 5.0
kp = 2.0

[[filters]]
type = "pid_control"
channel = "measured_v"
output_channel = "pid_out"
setpoint = 5.0
kp = 2.0
ki = 0.5
kd = 0.1

[[filters]]
type = "rate_limiter"
channel = "command_v"
output_channel = "rate_limited"
rate_limit_per_s = 1.0

[[filters]]
type = "slew_rate_limit"
channel = "command_v"
output_channel = "slew_limited"
rate_limit_per_s = 1.0

[[filters]]
type = "control_saturation"
channel = "command_v"
output_channel = "saturated_v"
min_v = 0.0
max_v = 5.0

[[filters]]
type = "control_deadzone"
channel = "command_v"
output_channel = "deadzone_v"
threshold_v = 0.1

[[filters]]
type = "feedforward_control"
channel = "command_v"
output_channel = "feedforward_v"
feedforward_gain = 2.0
feedforward_offset = 0.5
"#,
        )
        .expect("M35 filter config should deserialize");

        let filters = config.filters().expect("filters should convert");
        assert_eq!(filters.len(), 34);
        assert!(matches!(
            filters.first(),
            Some(FilterStep::ChannelArithmetic(ChannelArithmeticTransform {
                kind: ChannelArithmeticKind::Add,
                ..
            }))
        ));
        assert!(matches!(
            filters.get(7),
            Some(FilterStep::CoordinateRotation(
                CoordinateRotationTransform { .. }
            ))
        ));
        assert!(matches!(
            filters.get(13),
            Some(FilterStep::SensorConversion(filter))
                if filter.kind == SensorConversionKind::Rtd
        ));
        assert!(matches!(
            filters.get(25),
            Some(FilterStep::VibrationTransform(VibrationTransform {
                kind: VibrationTransformKind::VibrationSeverity,
                ..
            }))
        ));
        assert!(matches!(
            filters.last(),
            Some(FilterStep::ControlTransform(ControlTransform {
                kind: ControlTransformKind::FeedforwardControl,
                ..
            }))
        ));
    }

    #[test]
    fn rejects_invalid_tolerance_config() {
        let config = AnalysisConfig {
            input: InputConfig {
                time_column: "time".to_string(),
                channels: vec!["input_v".to_string()],
                time_unit: "s".to_string(),
                signal_unit: "V".to_string(),
            },
            metadata: MetadataContext::default(),
            tolerances: TolerancePolicy {
                voltage_v: 0.0,
                time_s: -0.001,
            },
            filters: Vec::new(),
            feature_transforms: Vec::new(),
            event_transforms: Vec::new(),
            event_validations: Vec::new(),
            criteria: Vec::new(),
        };

        assert!(matches!(
            config.validate(),
            Err(WaveformError::InvalidParameter { .. })
        ));
    }

    #[test]
    fn deserializes_legacy_and_dsl_criteria_side_by_side() {
        let toml = r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "legacy_max"
type = "maximum_voltage"
channel = "switch_v"
threshold_v = 5.0

[[criteria]]
id = "dsl_rise"
channel = "switch_v"

[criteria.measurement]
type = "rise_time"
low_threshold = { value = 0.5, unit = "V" }
high_threshold = { value = 4.5, unit = "V" }

[criteria.requirement]
operator = "less_than_or_equal"
value = 0.005
unit = "s"
"#;

        let config = toml::from_str::<AnalysisConfig>(toml).expect("config should deserialize");

        assert_eq!(config.criteria.len(), 2);
        assert_eq!(config.criteria[0].shape(), Ok(CriterionConfigShape::Legacy));
        assert_eq!(config.criteria[1].shape(), Ok(CriterionConfigShape::Dsl));
        assert_eq!(
            config.criteria[1].measurement.as_ref().map(|measurement| {
                (
                    measurement.kind.as_str(),
                    measurement
                        .low_threshold
                        .as_ref()
                        .map(|threshold| (threshold.value, threshold.unit.as_deref())),
                    measurement
                        .high_threshold
                        .as_ref()
                        .map(|threshold| (threshold.value, threshold.unit.as_deref())),
                )
            }),
            Some(("rise_time", Some((0.5, Some("V"))), Some((4.5, Some("V")))))
        );
        assert!(config.validate().is_ok());
    }

    #[test]
    fn validates_supported_dsl_operators_and_requirement_units() {
        for (operator, measurement_section, unit) in [
            ("less_than", "type = \"maximum_sample\"", "V"),
            (
                "less_than_or_equal",
                "type = \"rise_time\"\nlow_threshold = { value = 0.5, unit = \"V\" }\nhigh_threshold = { value = 4.5, unit = \"V\" }",
                "s",
            ),
            (
                "greater_than",
                "type = \"stable_state_duration\"\nthreshold = { value = 2.5, unit = \"V\" }\nstate = \"high\"",
                "s",
            ),
            (
                "greater_than_or_equal",
                "type = \"state_transition_count\"\nthreshold = { value = 2.5, unit = \"V\" }",
                "count",
            ),
            (
                "equal_to",
                "type = \"fall_time\"\nlow_threshold = { value = 0.5, unit = \"V\" }\nhigh_threshold = { value = 4.5, unit = \"V\" }",
                "s",
            ),
        ] {
            let toml = format!(
                r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "dsl_check"
channel = "switch_v"

[criteria.measurement]
{measurement_section}

[criteria.requirement]
operator = "{operator}"
value = 1.0
unit = "{unit}"
"#
            );

            let config =
                toml::from_str::<AnalysisConfig>(&toml).expect("config should deserialize");

            assert_eq!(config.validate(), Ok(()), "{operator} should validate");
        }
    }

    #[test]
    fn rejects_unknown_dsl_operator() {
        let config = dsl_config_with_requirement("approximately", 0.005, Some("s"));

        assert_eq!(
            config.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.dsl_rise.requirement.operator".to_string(),
                reason:
                    "unsupported operator `approximately`; expected less_than, less_than_or_equal, greater_than, greater_than_or_equal, or equal_to"
                        .to_string(),
            })
        );
    }

    #[test]
    fn rejects_missing_dsl_requirement_unit() {
        let config = dsl_config_with_requirement("less_than_or_equal", 0.005, None);

        assert_eq!(
            config.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.dsl_rise.requirement.unit".to_string(),
                reason: "explicit unit field is required".to_string(),
            })
        );
    }

    #[test]
    fn rejects_unsupported_dsl_requirement_unit() {
        let config = dsl_config_with_requirement("less_than_or_equal", 5.0, Some("ms"));

        assert_eq!(
            config.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.dsl_rise.requirement.unit".to_string(),
                reason: "unsupported unit `ms`; expected `V`, `s`, or `count`".to_string(),
            })
        );
    }

    #[test]
    fn rejects_mismatched_dsl_requirement_unit() {
        let config = dsl_config_with_requirement("less_than_or_equal", 0.005, Some("V"));

        assert_eq!(
            config.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.dsl_rise.requirement.unit".to_string(),
                reason: "expected unit `s`, got `V`".to_string(),
            })
        );
    }

    #[test]
    fn rejects_missing_dsl_threshold_unit() {
        let toml = r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "dsl_rise"
channel = "switch_v"

[criteria.measurement]
type = "rise_time"
low_threshold = { value = 0.5 }
high_threshold = { value = 4.5, unit = "V" }

[criteria.requirement]
operator = "less_than_or_equal"
value = 0.005
unit = "s"
"#;
        let config = toml::from_str::<AnalysisConfig>(toml).expect("config should deserialize");

        assert_eq!(
            config.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.dsl_rise.measurement.low_threshold.unit".to_string(),
                reason: "explicit unit field is required".to_string(),
            })
        );
    }

    #[test]
    fn rejects_mismatched_dsl_threshold_unit() {
        let toml = r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "dsl_rise"
channel = "switch_v"

[criteria.measurement]
type = "rise_time"
low_threshold = { value = 0.5, unit = "s" }
high_threshold = { value = 4.5, unit = "V" }

[criteria.requirement]
operator = "less_than_or_equal"
value = 0.005
unit = "s"
"#;
        let config = toml::from_str::<AnalysisConfig>(toml).expect("config should deserialize");

        assert_eq!(
            config.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.dsl_rise.measurement.low_threshold.unit".to_string(),
                reason: "expected unit `V`, got `s`".to_string(),
            })
        );
    }

    #[test]
    fn rejects_unit_shorthand_strings() {
        let toml = r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "dsl_rise"
channel = "switch_v"

[criteria.measurement]
type = "rise_time"
low_threshold = { value = 0.5, unit = "V" }
high_threshold = { value = 4.5, unit = "V" }

[criteria.requirement]
operator = "less_than_or_equal"
value = "5ms"
unit = "s"
"#;

        assert!(toml::from_str::<AnalysisConfig>(toml).is_err());
    }

    #[test]
    fn rejects_ambiguous_mixed_legacy_and_dsl_criterion() {
        let toml = r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "mixed"
type = "maximum_voltage"
channel = "switch_v"
threshold_v = 5.0

[criteria.measurement]
type = "maximum_sample"

[criteria.requirement]
operator = "less_than_or_equal"
value = 5.0
unit = "V"
"#;

        let config = toml::from_str::<AnalysisConfig>(toml).expect("config should deserialize");

        assert_eq!(
            config.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.mixed".to_string(),
                reason:
                    "legacy criteria using `type` cannot include DSL `measurement` or `requirement` sections"
                        .to_string(),
            })
        );
    }

    #[test]
    fn rejects_missing_dsl_measurement_or_requirement_sections() {
        let missing_measurement = toml::from_str::<AnalysisConfig>(
            r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "missing_measurement"
channel = "switch_v"

[criteria.requirement]
operator = "less_than_or_equal"
value = 0.005
unit = "s"
"#,
        )
        .expect("config should deserialize");
        let missing_requirement = toml::from_str::<AnalysisConfig>(
            r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "missing_requirement"
channel = "switch_v"

[criteria.measurement]
type = "rise_time"
low_threshold = { value = 0.5, unit = "V" }
high_threshold = { value = 4.5, unit = "V" }
"#,
        )
        .expect("config should deserialize");

        assert_eq!(
            missing_measurement.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.missing_measurement".to_string(),
                reason: "DSL criteria require both `measurement` and `requirement` sections"
                    .to_string(),
            })
        );
        assert_eq!(
            missing_requirement.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.missing_requirement".to_string(),
                reason: "DSL criteria require both `measurement` and `requirement` sections"
                    .to_string(),
            })
        );
    }

    #[test]
    fn rejects_missing_dsl_requirement_value() {
        let toml = r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "missing_value"
channel = "switch_v"

[criteria.measurement]
type = "maximum_sample"

[criteria.requirement]
operator = "less_than_or_equal"
unit = "V"
"#;
        let config = toml::from_str::<AnalysisConfig>(toml).expect("config should deserialize");

        assert_eq!(
            config.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.missing_value.requirement.value".to_string(),
                reason: "field is required for DSL requirements".to_string(),
            })
        );
    }

    #[test]
    fn rejects_missing_dsl_measurement_threshold() {
        let toml = r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "missing_threshold"
channel = "switch_v"

[criteria.measurement]
type = "state_transition_count"

[criteria.requirement]
operator = "equal_to"
value = 2
unit = "count"
"#;
        let config = toml::from_str::<AnalysisConfig>(toml).expect("config should deserialize");

        assert_eq!(
            config.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.missing_threshold.measurement.threshold".to_string(),
                reason: "field is required for this measurement type".to_string(),
            })
        );
    }

    #[test]
    fn rejects_incompatible_dsl_measurement_parameters() {
        let invalid_state = toml::from_str::<AnalysisConfig>(
            r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "bad_state"
channel = "switch_v"

[criteria.measurement]
type = "pulse_width"
threshold = { value = 2.5, unit = "V" }
state = "on"

[criteria.requirement]
operator = "greater_than_or_equal"
value = 0.001
unit = "s"
"#,
        )
        .expect("config should deserialize");
        let missing_selection = toml::from_str::<AnalysisConfig>(
            r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "missing_selection"
channel = "switch_v"

[criteria.measurement]
type = "pulse_width"
threshold = { value = 2.5, unit = "V" }
state = "high"

[criteria.requirement]
operator = "equal_to"
value = 0.001
unit = "s"
"#,
        )
        .expect("config should deserialize");
        let inverted_edge = toml::from_str::<AnalysisConfig>(
            r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "bad_edge"
channel = "switch_v"

[criteria.measurement]
type = "rise_time"
low_threshold = { value = 4.5, unit = "V" }
high_threshold = { value = 0.5, unit = "V" }

[criteria.requirement]
operator = "less_than_or_equal"
value = 0.005
unit = "s"
"#,
        )
        .expect("config should deserialize");

        assert_eq!(
            invalid_state.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.bad_state.measurement.state".to_string(),
                reason: "expected `high` or `low`, got `on`".to_string(),
            })
        );
        assert_eq!(
            missing_selection.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.missing_selection.measurement.selection".to_string(),
                reason:
                    "field is required for equal_to pulse_width criteria; use `shortest` or `longest`"
                        .to_string(),
            })
        );
        assert_eq!(
            inverted_edge.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.bad_edge.measurement.low_threshold".to_string(),
                reason: "must be lower than high_threshold".to_string(),
            })
        );
    }

    #[test]
    fn converts_dsl_criteria_to_measurement_runtime_criteria() {
        let toml = r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "dsl_rise"
channel = "switch_v"

[criteria.measurement]
type = "rise_time"
low_threshold = { value = 0.5, unit = "V" }
high_threshold = { value = 4.5, unit = "V" }

[criteria.requirement]
operator = "less_than_or_equal"
value = 0.005
unit = "s"
"#;

        let config = toml::from_str::<AnalysisConfig>(toml).expect("config should deserialize");

        let criteria = config.criteria().expect("DSL criteria should convert");

        assert_eq!(criteria.len(), 1);
        assert!(matches!(
            &criteria[0].check,
            crate::criteria::CriterionCheck::Measurement {
                measurement: MeasurementSpec::RiseTime {
                    low_threshold_v: 0.5,
                    high_threshold_v: 4.5,
                },
                requirement: MeasurementRequirement {
                    operator: CriterionOperator::LessThanOrEqual,
                    value: 0.005,
                },
                ..
            }
        ));
    }

    #[test]
    fn converts_response_latency_and_windowed_transient_criteria() {
        let toml = r#"
[input]
time_column = "time_s"
channels = ["command_v", "feedback_v"]

[[criteria]]
id = "response_latency"
type = "response_latency"
channel = "feedback_v"
source_channel = "command_v"
source_threshold_v = 2.5
target_threshold_v = 2.5
source_state = "high"
expected_target_state = "high"
max_latency_s = 0.050

[[criteria]]
id = "windowed_transient"
type = "transient_event"
channel = "feedback_v"
event_kind = "transient_event"
expected_state = "high"
threshold_v = 2.5
max_duration_s = 0.001
start_time_s = 1.020
arm_after_first_expected_state = true
"#;

        let config = toml::from_str::<AnalysisConfig>(toml).expect("config should deserialize");
        let criteria = config.criteria().expect("criteria should convert");

        assert_eq!(criteria.len(), 2);
        assert!(matches!(
            &criteria[0].check,
            crate::criteria::CriterionCheck::ResponseLatency {
                source_channel,
                target_channel,
                max_latency_s,
                ..
            } if source_channel == "command_v"
                && target_channel == "feedback_v"
                && (*max_latency_s - 0.050).abs() < f64::EPSILON
        ));
        assert!(matches!(
            &criteria[1].check,
            crate::criteria::CriterionCheck::TransientEvent {
                start_time_s: Some(1.020),
                end_time_s: None,
                arm_after_first_expected_state: true,
                ..
            }
        ));
    }

    #[test]
    fn rejects_invalid_response_latency_and_transient_window_config() {
        let missing_source = toml::from_str::<AnalysisConfig>(
            r#"
[input]
time_column = "time_s"
channels = ["command_v", "feedback_v"]

[[criteria]]
id = "missing_source"
type = "response_latency"
channel = "feedback_v"
source_threshold_v = 2.5
target_threshold_v = 2.5
source_state = "high"
expected_target_state = "high"
max_latency_s = 0.050
"#,
        )
        .expect("config should deserialize");
        let inverted_window = toml::from_str::<AnalysisConfig>(
            r#"
[input]
time_column = "time_s"
channels = ["feedback_v"]

[[criteria]]
id = "bad_window"
type = "transient_event"
channel = "feedback_v"
expected_state = "high"
threshold_v = 2.5
max_duration_s = 0.001
start_time_s = 1.020
end_time_s = 1.000
"#,
        )
        .expect("config should deserialize");

        assert_eq!(
            missing_source.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.source_channel".to_string(),
                reason: "field is required for this criterion type".to_string(),
            })
        );
        assert_eq!(
            inverted_window.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.end_time_s".to_string(),
                reason: "must be greater than or equal to start_time_s".to_string(),
            })
        );
    }

    #[test]
    fn rejects_incomplete_adc_quantizer_config() {
        let config = FilterConfig {
            kind: "adc_quantize".to_string(),
            window_samples: None,
            cutoff_hz: None,
            offset_v: None,
            gain: None,
            threshold_v: None,
            baseline_v: None,
            bits: Some(12),
            min_v: Some(0.0),
            max_v: None,
            start_time_s: None,
            end_time_s: None,
            delay_s: None,
            sample_interval_s: None,
            channel: None,
            mode: None,
            base: None,
            limit_v: None,
            input_min_v: None,
            input_max_v: None,
            output_min: None,
            output_max: None,
            points: None,
            coefficients: None,
            weights: None,
            alpha: None,
            sigma_samples: None,
            polynomial_order: None,
            outlier_sigma: None,
            center_hz: None,
            q: None,
            delay_samples: None,
            feedback_gain: None,
            ripple_db: None,
            stopband_attenuation_db: None,
            factor: None,
            upsample_factor: None,
            downsample_factor: None,
            reference_channel: None,
            target_channel: None,
            max_lag_samples: None,
            time_constant_s: None,
            threshold_per_s: None,
            threshold_sigma: None,
            lower_quantile: None,
            upper_quantile: None,
            ..FilterConfig::default()
        };

        let result = config.to_filter_step();

        assert_eq!(
            result,
            Err(WaveformError::InvalidParameter {
                name: "filters.max_v".to_string(),
                reason: "field is required for this filter type".to_string(),
            })
        );
    }

    #[test]
    fn rejects_incomplete_m11_filter_config() {
        for (toml, expected_name) in [
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "offset"

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.offset_v",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "moving_median"

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.window_samples",
            ),
        ] {
            let config: AnalysisConfig = toml::from_str(toml).expect("config should deserialize");
            let result = config.filters();
            assert!(matches!(
                result,
                Err(WaveformError::InvalidParameter { name, .. }) if name == expected_name
            ));
        }
    }

    #[test]
    fn rejects_incomplete_m14_filter_config() {
        let config: AnalysisConfig = toml::from_str(
            r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "high_pass_baseline"

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
        )
        .expect("config should deserialize");

        assert_eq!(
            config.filters(),
            Err(WaveformError::InvalidParameter {
                name: "filters.cutoff_hz".to_string(),
                reason: "field is required for this filter type".to_string(),
            })
        );
    }

    #[test]
    fn rejects_incomplete_m26_filter_config() {
        for (toml, expected_name) in [
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "crop"
start_time_s = 0.0

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.end_time_s",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "gap_fill"

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.sample_interval_s",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "channel_delay"
delay_s = 0.1

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.channel",
            ),
        ] {
            let config: AnalysisConfig = toml::from_str(toml).expect("config should deserialize");
            let result = config.filters();
            assert!(matches!(
                result,
                Err(WaveformError::InvalidParameter { name, .. }) if name == expected_name
            ));
        }
    }

    #[test]
    fn rejects_incomplete_m27_filter_config() {
        for (toml, expected_name) in [
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "log"

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.base",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "normalize"

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.mode",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "soft_limit"

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.limit_v",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "piecewise_linear"

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.points",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "polynomial"

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.coefficients",
            ),
        ] {
            let config: AnalysisConfig = toml::from_str(toml).expect("config should deserialize");
            let result = config.filters();
            assert!(matches!(
                result,
                Err(WaveformError::InvalidParameter { name, .. }) if name == expected_name
            ));
        }
    }

    #[test]
    fn rejects_incomplete_m28_filter_config() {
        for (toml, expected_name) in [
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "weighted_moving_average"

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.weights",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "exponential_moving_average"

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.alpha",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "gaussian_smoothing"
window_samples = 3

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.sigma_samples",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "polynomial_detrend"

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.polynomial_order",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "hampel_filter"
window_samples = 3

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.outlier_sigma",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "spike_remove"
window_samples = 3

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.threshold_v",
            ),
        ] {
            let config: AnalysisConfig = toml::from_str(toml).expect("config should deserialize");
            let result = config.filters();
            assert!(matches!(
                result,
                Err(WaveformError::InvalidParameter { name, .. }) if name == expected_name
            ));
        }
    }

    #[test]
    fn rejects_incomplete_m29_filter_config() {
        for (toml, expected_name) in [
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "fir_filter"

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.coefficients",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "band_pass"
q = 2.0

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.center_hz",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "band_stop"
center_hz = 60.0

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.q",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "comb_filter"
feedback_gain = 0.5

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.delay_samples",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "chebyshev1_low_pass"
cutoff_hz = 100.0

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.ripple_db",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "chebyshev2_low_pass"
cutoff_hz = 100.0

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.stopband_attenuation_db",
            ),
        ] {
            let config: AnalysisConfig = toml::from_str(toml).expect("config should deserialize");
            let result = config.filters();
            assert!(matches!(
                result,
                Err(WaveformError::InvalidParameter { name, .. }) if name == expected_name
            ));
        }
    }

    #[test]
    fn rejects_invalid_m28_filter_config() {
        for (toml, expected_name) in [
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "weighted_moving_average"
weights = [1.0, 0.0]

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.weights",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "exponential_moving_average"
alpha = 1.5

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.alpha",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "boxcar_smoothing"
window_samples = 0

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.window_samples",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "gaussian_smoothing"
window_samples = 3
sigma_samples = -1.0

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.sigma_samples",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "polynomial_detrend"
polynomial_order = 6

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.polynomial_order",
            ),
        ] {
            let config: AnalysisConfig = toml::from_str(toml).expect("config should deserialize");
            let result = config.filters();
            assert!(matches!(
                result,
                Err(WaveformError::InvalidParameter { name, .. }) if name == expected_name
            ));
        }
    }

    #[test]
    fn rejects_invalid_m29_filter_config() {
        for (toml, expected_name) in [
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "iir_biquad"
coefficients = [1.0, 0.0, 0.0, 0.0]

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.coefficients",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "iir_biquad"
coefficients = [1.0, 0.0, 0.0, 0.0, 1.2]

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.coefficients",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "high_pass"
cutoff_hz = -1.0

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.cutoff_hz",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "comb_filter"
delay_samples = 0
feedback_gain = 0.5

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.delay_samples",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "notch"
center_hz = 60.0
q = 0.0

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.q",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "chebyshev1_low_pass"
cutoff_hz = 100.0
ripple_db = 0.0

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.ripple_db",
            ),
        ] {
            let config: AnalysisConfig = toml::from_str(toml).expect("config should deserialize");
            let result = config.filters();
            assert!(matches!(
                result,
                Err(WaveformError::InvalidParameter { name, .. }) if name == expected_name
            ));
        }
    }

    #[test]
    fn rejects_incomplete_m30_filter_config() {
        for (toml, expected_name) in [
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "downsample"

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.factor",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "rational_resample"
upsample_factor = 2

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.downsample_factor",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["reference_v", "target_v"]

[[filters]]
type = "cross_correlation_delay"
target_channel = "target_v"
max_lag_samples = 2

[[criteria]]
id = "target_max"
type = "maximum_voltage"
channel = "target_v"
threshold_v = 5.5
"#,
                "filters.reference_channel",
            ),
        ] {
            let config: AnalysisConfig = toml::from_str(toml).expect("config should deserialize");
            let result = config.filters();
            assert!(matches!(
                result,
                Err(WaveformError::InvalidParameter { name, .. }) if name == expected_name
            ));
        }
    }

    #[test]
    fn rejects_invalid_m30_filter_config() {
        for (toml, expected_name) in [
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "downsample"
factor = 1

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.factor",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "sample_and_hold"
sample_interval_s = 0.0

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.sample_interval_s",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "rational_resample"
upsample_factor = 0
downsample_factor = 2

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.upsample_factor",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["reference_v", "target_v"]

[[filters]]
type = "cross_correlation_delay"
reference_channel = ""
target_channel = "target_v"
max_lag_samples = 2

[[criteria]]
id = "target_max"
type = "maximum_voltage"
channel = "target_v"
threshold_v = 5.5
"#,
                "filters.reference_channel",
            ),
        ] {
            let config: AnalysisConfig = toml::from_str(toml).expect("config should deserialize");
            let result = config.filters();
            assert!(matches!(
                result,
                Err(WaveformError::InvalidParameter { name, .. }) if name == expected_name
            ));
        }
    }

    #[test]
    fn rejects_invalid_m31_filter_config() {
        for (toml, expected_name) in [
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "envelope"
alpha = 1.5
"#,
                "filters.alpha",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "leaky_integrator"
time_constant_s = 0.0
"#,
                "filters.time_constant_s",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "slope_detection"
threshold_per_s = 0.0
"#,
                "filters.threshold_per_s",
            ),
        ] {
            let config: AnalysisConfig = toml::from_str(toml).expect("config should deserialize");
            let result = config.filters();
            assert!(matches!(
                result,
                Err(WaveformError::InvalidParameter { name, .. }) if name == expected_name
            ));
        }
    }

    #[test]
    fn rejects_invalid_m31_feature_transform_config() {
        for (toml, expected_name) in [
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[feature_transforms]]
id = ""
type = "rms"
channel = "input_v"
"#,
                "feature_transforms.id",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[feature_transforms]]
id = "bad"
type = "unknown_feature"
channel = "input_v"
"#,
                "feature_transforms.type",
            ),
        ] {
            let config: AnalysisConfig = toml::from_str(toml).expect("config should deserialize");
            let result = config.feature_transforms();
            assert!(matches!(
                result,
                Err(WaveformError::InvalidParameter { name, .. }) if name == expected_name
            ));
        }
    }

    #[test]
    fn rejects_invalid_m32_filter_config() {
        for (toml, expected_name) in [
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "rolling_mean"
window_samples = 0
"#,
                "filters.window_samples",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "outlier_detection"
threshold_sigma = 0.0
"#,
                "filters.threshold_sigma",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "quantile_clip"
lower_quantile = -0.1
upper_quantile = 0.9
"#,
                "filters.lower_quantile",
            ),
        ] {
            let config: AnalysisConfig = toml::from_str(toml).expect("config should deserialize");
            let result = config.filters();
            assert!(matches!(
                result,
                Err(WaveformError::InvalidParameter { name, .. }) if name == expected_name
            ));
        }
    }

    #[test]
    fn rejects_invalid_m32_feature_transform_config() {
        for (toml, expected_name) in [
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[feature_transforms]]
id = "bad"
type = "percentile"
channel = "input_v"
"#,
                "feature_transforms.percentile",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[feature_transforms]]
id = "bad"
type = "histogram"
channel = "input_v"
bins = 0
"#,
                "feature_transforms.bins",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[feature_transforms]]
id = "bad"
type = "correlation"
channel = "input_v"
"#,
                "feature_transforms.other_channel",
            ),
        ] {
            let config: AnalysisConfig = toml::from_str(toml).expect("config should deserialize");
            let result = config.feature_transforms();
            assert!(matches!(
                result,
                Err(WaveformError::InvalidParameter { name, .. }) if name == expected_name
            ));
        }
    }

    #[test]
    fn rejects_invalid_m33_feature_transform_config() {
        for (toml, expected_name) in [
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[feature_transforms]]
id = "bad"
type = "fft"
channel = "input_v"
window = "unknown"
"#,
                "feature_transforms.window",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[feature_transforms]]
id = "bad"
type = "welch_psd"
channel = "input_v"
"#,
                "feature_transforms.window_samples",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[feature_transforms]]
id = "bad"
type = "thd"
channel = "input_v"
fundamental_hz = 0.0
"#,
                "feature_transforms.fundamental_hz",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[feature_transforms]]
id = "bad"
type = "band_power"
channel = "input_v"
band_low_hz = 1.0
"#,
                "feature_transforms.band_high_hz",
            ),
        ] {
            let config: AnalysisConfig = toml::from_str(toml).expect("config should deserialize");
            let result = config.feature_transforms();
            assert!(matches!(
                result,
                Err(WaveformError::InvalidParameter { name, .. }) if name == expected_name
            ));
        }
    }

    #[test]
    fn rejects_invalid_m34_filter_config() {
        for (toml, expected_name) in [
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "white_noise"
amplitude_v = 0.1
"#,
                "filters.seed",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "impulse_noise"
amplitude_v = 1.0
seed = 1
"#,
                "filters.probability",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "companding"
mode = "linear"
max_v = 5.0
mu = 255.0
"#,
                "filters.mode",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "sample_clock_jitter"
seed = 1
"#,
                "filters.jitter_s",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "adc_missing_code"
bits = 4
min_v = 0.0
max_v = 5.0
"#,
                "filters.missing_code",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "inl_error"
bits = 4
min_v = 0.0
max_v = 5.0
coefficients = []
"#,
                "filters.coefficients",
            ),
        ] {
            let config: AnalysisConfig = toml::from_str(toml).expect("config should deserialize");
            let result = config.filters();
            assert!(matches!(
                result,
                Err(WaveformError::InvalidParameter { name, .. }) if name == expected_name
            ));
        }
    }

    #[test]
    fn rejects_invalid_m35_filter_config() {
        for (toml, expected_name) in [
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "channel_add"
left_channel = "a_v"
right_channel = "b_v"
"#,
                "filters.output_channel",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "vector_magnitude"
channels = ["x_v"]
output_channel = "mag"
"#,
                "filters.channels",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "matrix_transform"
channels = ["x_v"]
matrix = []
output_channels = ["mix"]
"#,
                "filters.matrix",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "rtd_temperature"
channel = "rtd_ohm"
output_channel = "rtd_c"
output_unit = "C"
r0_ohm = 100.0
"#,
                "filters.alpha_per_c",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "vibration_severity"
channel = "accel"
output_channel = "severity"
output_unit = "m/s^2"
window_samples = 0
"#,
                "filters.window_samples",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "pid_control"
channel = "measured"
output_channel = "pid"
setpoint = 1.0
kp = 1.0
ki = 0.1
"#,
                "filters.kd",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "rate_limiter"
channel = "command"
output_channel = "limited"
rate_limit_per_s = 0.0
"#,
                "filters.rate_limit_per_s",
            ),
        ] {
            let config: AnalysisConfig = toml::from_str(toml).expect("config should deserialize");
            let result = config.filters();
            assert!(matches!(
                result,
                Err(WaveformError::InvalidParameter { name, .. }) if name == expected_name
            ));
        }
    }

    #[test]
    fn rejects_invalid_m27_filter_config() {
        for (toml, expected_name) in [
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "normalize"
mode = "unit_circle"

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.mode",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "normalize"
mode = "range"
input_min_v = 1.0
input_max_v = 0.0
output_min = 0.0
output_max = 1.0

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.input_max_v",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "soft_limit"
limit_v = 0.0

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.limit_v",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "piecewise_linear"
points = [{ x = 0.0, y = 0.0 }]

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.points",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "piecewise_linear"
points = [{ x = 1.0, y = 0.0 }, { x = 0.0, y = 1.0 }]

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.points",
            ),
            (
                r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "polynomial"
coefficients = []

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
                "filters.coefficients",
            ),
        ] {
            let config: AnalysisConfig = toml::from_str(toml).expect("config should deserialize");
            let result = config.filters();
            assert!(matches!(
                result,
                Err(WaveformError::InvalidParameter { name, .. }) if name == expected_name
            ));
        }
    }

    #[test]
    fn rejects_invalid_m14_filter_config() {
        for cutoff_hz in [0.0, -1.0, f64::NAN] {
            let config = FilterConfig {
                kind: "high_pass_baseline".to_string(),
                window_samples: None,
                cutoff_hz: Some(cutoff_hz),
                offset_v: None,
                gain: None,
                threshold_v: None,
                baseline_v: None,
                bits: None,
                min_v: None,
                max_v: None,
                start_time_s: None,
                end_time_s: None,
                delay_s: None,
                sample_interval_s: None,
                channel: None,
                mode: None,
                base: None,
                limit_v: None,
                input_min_v: None,
                input_max_v: None,
                output_min: None,
                output_max: None,
                points: None,
                coefficients: None,
                weights: None,
                alpha: None,
                sigma_samples: None,
                polynomial_order: None,
                outlier_sigma: None,
                center_hz: None,
                q: None,
                delay_samples: None,
                feedback_gain: None,
                ripple_db: None,
                stopband_attenuation_db: None,
                factor: None,
                upsample_factor: None,
                downsample_factor: None,
                reference_channel: None,
                target_channel: None,
                max_lag_samples: None,
                time_constant_s: None,
                threshold_per_s: None,
                threshold_sigma: None,
                lower_quantile: None,
                upper_quantile: None,
                ..FilterConfig::default()
            };

            assert!(matches!(
                config.to_filter_step(),
                Err(WaveformError::InvalidParameter { .. })
            ));
        }
    }

    #[test]
    fn converts_m12_event_transform_and_validation_config() {
        let config: AnalysisConfig = toml::from_str(
            r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[event_transforms]]
id = "switch_state"
type = "schmitt_trigger"
channel = "switch_v"
on_threshold_v = 3.0
off_threshold_v = 2.0
initial_state = "low"

[[event_transforms]]
id = "debounce"
type = "debounce"
channel = "switch_v"
min_duration_s = 0.002

[[event_transforms]]
id = "glitch"
type = "glitch_removal"
channel = "switch_v"
max_duration_s = 0.002

[[event_transforms]]
id = "edges"
type = "edge_extraction"
channel = "switch_v"

[[event_transforms]]
id = "bounce"
type = "bounce_detection"
channel = "switch_v"
window_s = 0.004

[[event_validations]]
id = "must_rise"
type = "missing_pulse"
channel = "switch_v"
direction = "rising"
expected_count = 1

[[event_validations]]
id = "no_extra_rise"
type = "extra_pulse"
channel = "switch_v"
direction = "rising"
max_count = 1

[[event_validations]]
id = "high_dwell"
type = "dwell_time"
channel = "switch_v"
state = "high"
min_duration_s = 0.001

[[event_validations]]
id = "rise_timeout"
type = "timeout"
channel = "switch_v"
direction = "rising"
start_time_s = 0.0
max_time_s = 0.002

[[criteria]]
id = "switch_max"
type = "maximum_voltage"
channel = "switch_v"
threshold_v = 5.5
"#,
        )
        .expect("M12 event config should deserialize");

        assert_eq!(
            config.event_transforms().expect("event transforms").len(),
            5
        );
        assert_eq!(
            config.event_validations().expect("event validations").len(),
            4
        );
        config.validate().expect("M12 config should validate");
    }

    #[test]
    fn rejects_invalid_m12_event_config() {
        let config: AnalysisConfig = toml::from_str(
            r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[event_transforms]]
id = "bad_state"
type = "schmitt_trigger"
channel = "switch_v"
on_threshold_v = 2.0
off_threshold_v = 2.0
initial_state = "low"

[[criteria]]
id = "switch_max"
type = "maximum_voltage"
channel = "switch_v"
threshold_v = 5.5
"#,
        )
        .expect("config should deserialize");

        assert!(matches!(
            config.validate(),
            Err(WaveformError::InvalidParameter { name, .. }) if name == "event_transforms.off_threshold_v"
        ));
    }

    #[test]
    fn rejects_invalid_m12_event_validation_config() {
        let config: AnalysisConfig = toml::from_str(
            r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[event_validations]]
id = "bad_direction"
type = "missing_pulse"
channel = "switch_v"
direction = "rise"
"#,
        )
        .expect("config should deserialize");

        assert!(matches!(
            config.validate(),
            Err(WaveformError::InvalidParameter { name, .. }) if name == "event_validations.direction"
        ));

        let config: AnalysisConfig = toml::from_str(
            r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[event_validations]]
id = "missing_max_count"
type = "extra_pulse"
channel = "switch_v"
direction = "rising"
"#,
        )
        .expect("config should deserialize");

        assert!(matches!(
            config.validate(),
            Err(WaveformError::InvalidParameter { name, .. }) if name == "event_validations.max_count"
        ));
    }

    fn dsl_config_with_requirement(
        operator: &str,
        value: f64,
        unit: Option<&str>,
    ) -> AnalysisConfig {
        let unit_line = unit
            .map(|unit| format!("unit = \"{unit}\""))
            .unwrap_or_default();
        let toml = format!(
            r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "dsl_rise"
channel = "switch_v"

[criteria.measurement]
type = "rise_time"
low_threshold = {{ value = 0.5, unit = "V" }}
high_threshold = {{ value = 4.5, unit = "V" }}

[criteria.requirement]
operator = "{operator}"
value = {value}
{unit_line}
"#
        );

        toml::from_str::<AnalysisConfig>(&toml).expect("config should deserialize")
    }
}
