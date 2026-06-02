# Structured Transform Metadata Design

Date: 2026-06-02

Status: M10-002 / issue #133 design artifact, extended by M11 issues #140 through #146 for pointwise, baseline, and moving-median transforms, by M14 for high-pass baseline correction, by M25 for source-of-truth catalog checks, by M26 for data-cleaning/timing transforms, by M27 for pointwise normalization/nonlinear conditioning, by M28 for smoothing/baseline conditioning, by M29 for standard frequency filters, by M30 for resampling/timing alignment, by M31 for envelope, energy, and calculus waveform/feature transforms, by M32 for statistics/correlation transforms, and by M33 for spectrum/time-frequency feature records.

## Purpose

FerrisOxide records derived waveform transforms as ordered human-readable strings in `transform_history`. That stays for compatibility, and transformed waveform reports now also include structured `transform_steps` metadata that can be inspected without parsing strings.

This design defines the additive `transform_steps` shape for waveform metadata. Existing `transform_history` remains present, and raw untransformed reports skip `transform_steps` so existing no-transform golden reports remain stable.

M25 adds `crates/ferrisoxide-core/src/transform_catalog.rs` as the source-of-truth catalog for implemented and planned transform metadata. New transforms should update the catalog and include metadata tests that compare emitted `TransformStepMetadata` to the catalog entry.

M26 proves that rule with `timestamp_sort`, `dedupe_timestamps`, `nan_interpolate`, `nan_remove`, `crop`, `fixed_delay`, `gap_fill`, `resample_fixed`, and `channel_delay`: each implemented transform emits structured metadata and has catalog-matching tests.

M27 extends the same rule to `absolute_value`, `square`, `square_root`, `log`, `exp`, `normalize`, `tanh`, `sigmoid`, `soft_limit`, `piecewise_linear`, and `polynomial`. M28 extends it to `weighted_moving_average`, `exponential_moving_average`, `boxcar_smoothing`, `gaussian_smoothing`, `savitzky_golay`, `centered_moving_median`, `rolling_mean_baseline`, `rolling_median_baseline`, `linear_detrend`, `polynomial_detrend`, `hampel_filter`, and `spike_remove`. M29 extends it to standard frequency filters. M30 extends it to `resample`, `downsample`, `decimate`, `upsample`, `interpolate`, `rational_resample`, `sample_and_hold`, `zero_order_hold`, `first_order_hold`, `fractional_delay`, `cross_correlation_delay`, `jitter_correction`, and `clock_drift_correction`. M31 extends it to envelope/calculus waveform filters and scalar `feature_records`. M32 extends it to rolling/statistical waveform filters, scalar statistical feature records, histogram bin records, and covariance/correlation feature records with method context. M33 extends it to frequency and time-frequency feature records with frequency-bin, window, complex, harmonic, band, and segment context.

## Compatibility Decision

`transform_history` remains the stable compatibility field.

Structured metadata is a sibling field under `waveform_metadata`:

```json
"waveform_metadata": {
  "transform_history": [
    "moving_average(window_samples=2)"
  ],
  "transform_steps": [
    {
      "sequence_index": 0,
      "history_label": "moving_average(window_samples=2)",
      "name": "moving_average",
      "category": "WindowedTransform",
      "input_channels": {
        "kind": "all_channels"
      },
      "output_channels": {
        "kind": "derived_channels",
        "preserves_names": true
      },
      "parameters": [
        {
          "name": "window_samples",
          "value": 2,
          "unit": "samples"
        }
      ],
      "sample_rate_required": false,
      "stateful": true,
      "causal": true,
      "phase_effect": "delay",
      "streaming_supported": true,
      "offline_only": false,
      "runtime_profiles": [
        "desktop"
      ],
      "capability_status": "implemented",
      "evidence_level": "golden_report_tested"
    }
  ]
}
```

`transform_steps` is serialized only when non-empty unless a schema migration explicitly requires empty arrays. Reports for raw untransformed waveforms remain byte-for-byte stable where no other schema changes are made.

## Field Shape

| Field | Type | Required | Meaning |
|---|---|---|---|
| `sequence_index` | integer | Yes | Zero-based transform order in the evaluated derived waveform. |
| `history_label` | string | Yes | Human-readable label also appended to `transform_history`. |
| `name` | string | Yes | Stable snake_case transform identifier. |
| `category` | string | Yes | Category from `docs/transform-capability-model.md`. |
| `input_channels` | object | Yes | Channel requirement or actual channel set consumed. |
| `output_channels` | object | Yes | Derived channel, event, feature, or validation output behavior. |
| `parameters` | array | Yes | Ordered parameter records with values and units. |
| `sample_rate_required` | boolean | Yes | Whether valid timing/sample-rate metadata is required. |
| `stateful` | boolean | Yes | Whether previous samples or internal state affect current output. |
| `causal` | boolean | Yes | Whether output at sample N depends only on samples up to N. |
| `phase_effect` | string | Yes | `none`, `delay`, `nonlinear`, `zero_phase`, or `not_applicable`. |
| `streaming_supported` | boolean | Yes | Whether bounded streaming execution is possible. |
| `offline_only` | boolean | Yes | Whether full-record access or forward/backward passes are required. |
| `runtime_profiles` | array | Yes | Runtime profiles from the capability model. |
| `capability_status` | string | Yes | Support status from the capability model. |
| `evidence_level` | string | Yes | Strongest evidence level from the capability model. |

## Parameter Records

Parameter records should avoid string parsing and preserve units when relevant.

| Field | Type | Required | Meaning |
|---|---|---|---|
| `name` | string | Yes | Stable parameter name. |
| `value` | number, integer, boolean, string, or array | Yes | Parameter value after validation/defaulting. |
| `unit` | string or null | Yes | Engineering unit or domain unit such as `Hz`, `V`, `samples`, or `s`; use null only for unitless values. |

## Channel Records

The current Rust representation supports the current all-channel derived waveform transforms and keeps the serialized shape open for future multi-channel transforms.

Suggested `input_channels.kind` values:

- `all_channels`
- `single_channel`
- `channel_pair`
- `channel_set`
- `event_stream`
- `feature_records`

Suggested `output_channels.kind` values:

- `derived_channels`
- `derived_channel`
- `event_records`
- `feature_records`
- `validation_records`

M12 implements `event_records` for event evidence and `validation_records` for event validation evidence. Existing waveform transforms continue to use `derived_channels`.

## Initial Transform Metadata Expectations

M10-003 owns final mappings for current transforms. This document records the direction those mappings should follow.

| Transform | `name` | `category` | `sample_rate_required` | `stateful` | `causal` | `phase_effect` |
|---|---|---|---|---|---|---|
| Moving average | `moving_average` | `WindowedTransform` | false | true | true | `delay` |
| Low-pass | `low_pass` | `FrequencyFilterTransform` | true | true | true | `delay` |
| ADC quantization | `adc_quantize` | `QuantizationTransform` | false | false | true | `none` |
| Offset | `offset` | `PointwiseTransform` | false | false | true | `none` |
| Gain | `gain` | `PointwiseTransform` | false | false | true | `none` |
| Invert | `invert` | `PointwiseTransform` | false | false | true | `none` |
| Clamp | `clamp` | `PointwiseTransform` | false | false | true | `nonlinear` |
| Deadband | `deadband` | `PointwiseTransform` | false | false | true | `nonlinear` |
| DC removal | `dc_remove` | `BaselineTransform` | false | true | false | `none` |
| Baseline subtraction | `baseline_subtract` | `BaselineTransform` | false | false | true | `none` |
| High-pass baseline correction | `high_pass_baseline` | `StatefulTransform` | true | true | true | `delay` |
| Moving median | `moving_median` | `WindowedTransform` | false | true | true | `nonlinear` |
| M27 pointwise/nonlinear transforms | `absolute_value`, `square`, `square_root`, `log`, `exp`, `tanh`, `sigmoid`, `soft_limit`, `piecewise_linear`, `polynomial` | `PointwiseTransform` | false | false | true | `nonlinear` |
| M27 normalization | `normalize` | `PointwiseTransform` | false | true | false | `none` |
| M28 causal smoothing | `weighted_moving_average`, `exponential_moving_average` | `WindowedTransform` / `StatefulTransform` | false | true | true | `delay` |
| M28 centered/offline smoothing | `boxcar_smoothing`, `gaussian_smoothing`, `savitzky_golay`, `centered_moving_median`, `hampel_filter`, `spike_remove` | `WindowedTransform` | false | true | false | `none` / `nonlinear` |
| M28 baseline/detrend | `rolling_mean_baseline`, `rolling_median_baseline`, `linear_detrend`, `polynomial_detrend` | `BaselineTransform` | false/true | true | true/false | `delay` / `nonlinear` / `none` |
| M29 coefficient frequency filters | `fir_filter`, `iir_biquad`, `comb_filter` | `FrequencyFilterTransform` | false | true | true | `delay` |
| M29 designed frequency filters | `high_pass`, `band_pass`, `band_stop`, `notch`, `butterworth_low_pass`, `butterworth_high_pass`, `chebyshev1_low_pass`, `chebyshev2_low_pass`, `bessel_low_pass` | `FrequencyFilterTransform` | true | true | true | `delay` |
| M29 zero-phase frequency filters | `zero_phase_fir_filter`, `zero_phase_iir_biquad` | `FrequencyFilterTransform` | false | true | false | `none` |
| M30 grid conversion transforms | `resample`, `upsample`, `interpolate`, `rational_resample`, `first_order_hold`, `jitter_correction`, `clock_drift_correction` | `ResamplingTransform` | true | false | false | `none` |
| M30 causal/downsample transform | `downsample` | `ResamplingTransform` | true | false | true | `none` |
| M30 delay and hold transforms | `decimate`, `sample_and_hold`, `zero_order_hold`, `fractional_delay`, `cross_correlation_delay` | `ResamplingTransform` | true | true/false | true/false | `delay` |
| M31 pointwise/stateful envelope transforms | `half_wave_rectify`, `full_wave_rectify`, `envelope`, `moving_rms`, `peak_hold` | `PointwiseTransform`, `StatefulTransform`, or `WindowedTransform` | false | true/false | true | `delay` or `nonlinear` |
| M31 calculus waveform transforms | `first_derivative`, `second_derivative`, `integral`, `cumulative_integral`, `leaky_integrator`, `slope_detection` | `FeatureTransform` or `StatefulTransform` | true | true | true | `delay` or `nonlinear` |
| M31 scalar feature records | `rms`, `peak_to_peak`, `crest_factor`, `energy`, `power`, `area_under_curve`, `impulse_estimate` | `FeatureTransform` | true/false | false | false | `none` |
| M32 rolling statistics filters | `rolling_mean`, `rolling_variance`, `rolling_stddev`, `rolling_min`, `rolling_max` | `WindowedTransform` | false | true | true | `delay` |
| M32 distribution filters | `z_score`, `outlier_detection`, `quantile_clip` | `FeatureTransform` | false | false | false | `nonlinear` |
| M32 scalar and correlation feature records | `mean`, `median`, `mode`, `min`, `max`, `variance`, `standard_deviation`, `skewness`, `kurtosis`, `percentile`, `quantile`, `histogram`, `covariance`, `correlation`, `autocorrelation`, `cross_correlation` | `FeatureTransform` | false | false | false | `none` |
| M33 frequency feature records | `dft`, `fft`, `ifft`, `power_spectrum`, `psd`, `welch_psd`, `cross_spectrum`, `coherence`, `transfer_function` | `FrequencyFilterTransform` | true/false | true/false | false | `none` |
| M33 time-frequency feature records | `stft`, `spectrogram` | `TimeFrequencyTransform` | true | true | false | `none` |
| M33 spectral scalar feature records | `window_function`, `harmonic_analysis`, `thd`, `snr`, `sinad`, `enob`, `spectral_centroid`, `spectral_bandwidth`, `spectral_rolloff`, `band_power` | `FeatureTransform` | true/false | false | false | `none` |
| M34 fault-injection simulation filters | `white_noise`, `gaussian_noise`, `uniform_noise`, `pink_noise`, `brown_noise`, `impulse_noise`, `salt_pepper_noise`, `quantization_noise`, `periodic_interference`, `hum_interference`, `ground_bounce`, `thermal_drift`, `random_walk_drift`, `dropout_fault`, `missing_samples`, `saturation_fault`, `stuck_at_fault`, `flatline_fault`, `intermittent_fault`, `sample_clock_jitter` | `FaultInjectionTransform` | true/false | true/false | false | `nonlinear` |
| M34 ADC/DAC simulation filters | `rounding_quantizer`, `floor_quantizer`, `ceil_quantizer`, `midrise_quantizer`, `midtread_quantizer`, `saturating_quantizer`, `dither`, `companding`, `adc_missing_code`, `inl_error`, `dnl_error`, `adc_gain_error`, `adc_offset_error` | `QuantizationTransform` | false | true/false | false | `nonlinear` |

## Report Schema Direction

`docs/report-schema.md` describes `transform_steps` as an additive emitted field. The report compatibility rule is:

- Keep `transform_history` unchanged.
- Emit `transform_steps` only when metadata mappings and records exist.
- Do not remove or rename existing waveform metadata fields.
- Do not require downstream consumers to parse `transform_history`.
- Update exact golden reports when the structured field is intentionally emitted or changed.

## Golden Report Expectations

M10-006 adds exact tests for these expectations:

- Raw waveform reports with no transforms should stay byte-for-byte stable if `transform_steps` is skipped when empty.
- Transformed waveform reports should add structured metadata in transform order.
- Each `transform_steps[].history_label` should match the corresponding `transform_history[]` entry.
- Tests should assert field values for implemented transform metadata, including moving average, low-pass, ADC quantization, pointwise transforms, baseline transforms, high-pass baseline correction, moving median, M26-M33 transform families, and feature-record metadata.
- Existing result and measurement evidence fields should remain unchanged unless a separate schema migration is approved.

## Rust Model

The Rust model keeps structured transform metadata in `WaveformMetadata`:

```rust
pub struct WaveformMetadata {
    pub transform_history: Vec<String>,
    pub transform_steps: Vec<TransformStepMetadata>,
    // existing fields remain unchanged
}

pub struct TransformStepMetadata {
    pub sequence_index: usize,
    pub history_label: String,
    pub name: String,
    pub category: TransformCategory,
    pub input_channels: TransformInputChannels,
    pub output_channels: TransformOutputChannels,
    pub parameters: Vec<TransformParameterMetadata>,
    pub sample_rate_required: bool,
    pub stateful: bool,
    pub causal: bool,
    pub phase_effect: TransformPhaseEffect,
    pub streaming_supported: bool,
    pub offline_only: bool,
    pub runtime_profiles: Vec<TransformRuntimeProfile>,
    pub capability_status: TransformCapabilityStatus,
    pub evidence_level: TransformEvidenceLevel,
}
```

Implementation should prefer typed enums internally and explicit `serde` names in reports.

## Hand-Off Note

Role: Software Architect
Goal: Complete M10-002 / issue #133 by defining the structured transform metadata design before implementation.
Files changed: `docs/structured-transform-metadata.md`
Checks run: Documentation and schema compatibility review.
Status: Complete through PR #138; issue #133 and milestone #10 are closed, with local updates through M33.
Known gaps: Embedded/no_std transform exposure remains future gated work. M13 adds runtime-profile validation code for current metadata through PR #164; M14 and M26-M33 desktop-only transforms remain package/runtime gated.
Next recommended step: Use this design with the M13 validator as the compatibility baseline for future transform additions and package/runtime exposure.
