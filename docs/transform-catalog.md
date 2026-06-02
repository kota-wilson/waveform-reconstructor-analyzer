# Transform Catalog

Date: 2026-06-02

Status: M25 source-of-truth catalog and completeness contract, updated through local M36 comprehensive-suite closure. This document describes how FerrisOxide exposes the transform catalog from code. The authoritative catalog entries live in `crates/ferrisoxide-core/src/transform_catalog.rs` and are inspectable through the CLI.

## Purpose

FerrisOxide now has one catalog surface for implemented and planned transform capabilities. The catalog prevents the project from spreading transform status across disconnected Markdown tables, config code, report metadata, and rule-package export behavior.

The catalog answers:

- which transforms exist,
- which milestone owns them,
- whether they are implemented, planned, research-only, or dependency-gated,
- which config surface will expose them,
- which output kind they create,
- whether sample rate is required,
- whether the transform is stateful, causal, streaming-safe, or offline-only,
- what phase effect it has,
- which runtime profiles are supported,
- what evidence level exists,
- whether rule-package export supports or rejects the transform,
- which documentation file explains the behavior.

## How To Inspect

Use the CLI catalog command:

```text
ferrisoxide-signal transforms --format text
ferrisoxide-signal transforms --format json
```

The output is generated from `ferrisoxide_core::transform_catalog::transform_catalog()`. Documentation should point to this command or to the Rust catalog instead of maintaining independent hand-written transform lists.

## Catalog Contract

Every catalog entry has these fields:

| Field | Meaning |
|---|---|
| `name` | Stable snake_case transform identifier. |
| `label` | Human-readable display label. |
| `milestone` | Owning milestone or `implemented` for existing transforms completed before M25. |
| `family` | Roadmap family, such as `frequency_filters` or `fault_injection_adc_dac`. |
| `category` | Architecture category, such as `PointwiseTransform`, `WindowedTransform`, or `ValidationTransform`. |
| `config_surface` | Config/API surface where the transform is or will be exposed. |
| `output_kind` | Derived waveform channels, event records, validation records, feature records, or catalog-only artifact. |
| `parameters` | Required parameter names and units. |
| `sample_rate_required` | Whether valid timing metadata is required. |
| `stateful` | Whether previous samples or internal state affect output. |
| `causal` | Whether output at sample N depends only on samples up to N. |
| `phase_effect` | `none`, `delay`, or `nonlinear` for the current Rust model. |
| `streaming_supported` | Whether bounded streaming execution is supported for the stated runtime profiles. |
| `offline_only` | Whether full-record access is required. |
| `runtime_profiles` | Runtime profiles currently supported or future-gated. |
| `capability_status` | `implemented`, `planned`, `research`, `dependency_gated`, `hardware_gated`, or `certification_gated`. |
| `evidence_level` | Strongest current evidence level. |
| `package_support` | Rule-package compatibility decision. |
| `docs_path` | Documentation path for behavior, assumptions, or roadmap scope. |
| `notes` | Short non-goal or scope note. |

## Implemented Coverage

The catalog includes every current transform that emits `TransformStepMetadata`:

- waveform filters from `crates/ferrisoxide-core/src/filter.rs`,
- event transforms from `crates/ferrisoxide-core/src/event.rs`,
- event validation transforms from `crates/ferrisoxide-core/src/event.rs`.

Focused M25-M35 tests apply representative waveform, data-cleaning, pointwise/nonlinear, smoothing/baseline, frequency-filter, resampling/timing, envelope/calculus, statistics/correlation, spectrum/time-frequency, fault-injection/ADC-DAC simulation, multi-channel/sensor/domain, feature-record, and event pipelines, then compare emitted `TransformStepMetadata` against the matching catalog entry. M36 closes catalog/readiness evidence and tests CLI catalog output for the `comprehensive_suite_closure` entry. A transform should not be added to config or reports without adding a catalog entry and metadata test coverage.

M26 adds implemented catalog entries for:

- `timestamp_sort`
- `dedupe_timestamps`
- `nan_interpolate`
- `nan_remove`
- `crop`
- `fixed_delay`
- `gap_fill`
- `resample_fixed`
- `channel_delay`

M27 adds implemented catalog entries for:

- `absolute_value`
- `square`
- `square_root`
- `log`
- `exp`
- `normalize`
- `tanh`
- `sigmoid`
- `soft_limit`
- `piecewise_linear`
- `polynomial`

M28 adds implemented catalog entries for:

- `weighted_moving_average`
- `exponential_moving_average`
- `boxcar_smoothing`
- `gaussian_smoothing`
- `savitzky_golay`
- `centered_moving_median`
- `rolling_mean_baseline`
- `rolling_median_baseline`
- `linear_detrend`
- `polynomial_detrend`
- `hampel_filter`
- `spike_remove`

M29 adds implemented catalog entries for:

- `fir_filter`
- `zero_phase_fir_filter`
- `iir_biquad`
- `zero_phase_iir_biquad`
- `high_pass`
- `band_pass`
- `band_stop`
- `notch`
- `comb_filter`
- `butterworth_low_pass`
- `butterworth_high_pass`
- `chebyshev1_low_pass`
- `chebyshev2_low_pass`
- `bessel_low_pass`

M29 keeps `elliptic_low_pass` dependency-gated pending exact filter-design dependency review.

M30 adds implemented catalog entries for:

- `resample`
- `downsample`
- `decimate`
- `upsample`
- `interpolate`
- `rational_resample`
- `sample_and_hold`
- `zero_order_hold`
- `first_order_hold`
- `fractional_delay`
- `cross_correlation_delay`
- `jitter_correction`
- `clock_drift_correction`

M30 keeps `polyphase_resample` dependency/performance-gated pending a numeric helper and performance review.

M31 adds implemented catalog entries for:

- `half_wave_rectify`
- `full_wave_rectify`
- `envelope`
- `moving_rms`
- `peak_hold`
- `first_derivative`
- `second_derivative`
- `integral`
- `cumulative_integral`
- `leaky_integrator`
- `slope_detection`
- `rms`
- `peak_to_peak`
- `crest_factor`
- `energy`
- `power`
- `area_under_curve`
- `impulse_estimate`

M31 keeps Hilbert envelope and analytic-signal workflows dependency/design-gated pending a separate signal-processing design review.

M32 adds implemented catalog entries for:

- `rolling_mean`
- `rolling_variance`
- `rolling_stddev`
- `rolling_min`
- `rolling_max`
- `z_score`
- `outlier_detection`
- `quantile_clip`
- `mean`
- `median`
- `mode`
- `min`
- `max`
- `variance`
- `standard_deviation`
- `skewness`
- `kurtosis`
- `percentile`
- `quantile`
- `histogram`
- `covariance`
- `correlation`
- `autocorrelation`
- `cross_correlation`

M33 adds implemented catalog entries for:

- `window_function`
- `dft`
- `fft`
- `ifft`
- `power_spectrum`
- `psd`
- `welch_psd`
- `cross_spectrum`
- `coherence`
- `transfer_function`
- `harmonic_analysis`
- `thd`
- `snr`
- `sinad`
- `enob`
- `stft`
- `spectrogram`
- `spectral_centroid`
- `spectral_bandwidth`
- `spectral_rolloff`
- `band_power`

M34 adds implemented catalog entries for:

- `white_noise`
- `gaussian_noise`
- `uniform_noise`
- `pink_noise`
- `brown_noise`
- `impulse_noise`
- `salt_pepper_noise`
- `quantization_noise`
- `periodic_interference`
- `hum_interference`
- `ground_bounce`
- `thermal_drift`
- `random_walk_drift`
- `dropout_fault`
- `missing_samples`
- `saturation_fault`
- `stuck_at_fault`
- `flatline_fault`
- `intermittent_fault`
- `rounding_quantizer`
- `floor_quantizer`
- `ceil_quantizer`
- `midrise_quantizer`
- `midtread_quantizer`
- `saturating_quantizer`
- `dither`
- `companding`
- `sample_clock_jitter`
- `adc_missing_code`
- `inl_error`
- `dnl_error`
- `adc_gain_error`
- `adc_offset_error`

M34 transforms are desktop-only simulation evidence. Each emitted transform step records `evidence_scope = simulation_only`; catalog entries reject rule-package export and do not claim hardware ADC accuracy, calibration, hardware qualification, certification, live DAQ, or runtime-package support.

## M35 Multi-Channel, Sensor, And Domain Conditioning

M35 adds desktop `[[filters]]` support for multi-channel math, software engineering-unit conversion, vibration-style integration/RMS severity, and control-signal conditioning:

- multi-channel derived channels: `channel_add`, `channel_subtract`, `differential_channel`, `common_mode`, `vector_magnitude`, `euclidean_norm`, `matrix_transform`, and `coordinate_rotation`,
- sensor and engineering-unit conversions: `linear_sensor_conversion`, `pressure_transducer`, `current_shunt`, `bridge_strain`, `load_cell_force`, `rtd_temperature`, `thermistor_temperature`, `tachometer_rpm`, `encoder_position`, `accelerometer_units`, `gyroscope_rate`, `hall_current`, `lvdt_position`, `microphone_spl`, and `photodiode_power`,
- vibration and control conditioning: `velocity_from_acceleration`, `displacement_from_velocity`, `vibration_severity`, `control_error`, `proportional_control`, `pid_control`, `rate_limiter`, `slew_rate_limit`, `control_saturation`, `control_deadzone`, and `feedforward_control`.

M35 transforms are software-only conditioning evidence. They validate channel names, output-channel names, matrix shape, required sensor parameters, sample-rate needs, and unit declarations, but they do not create hardware calibration evidence, sensor accuracy evidence, qualification evidence, certification evidence, live DAQ support, HAL/RTOS support, or rule-package/runtime support.

M35 also records dependency/design-gated entries for `phase_difference`, `gain_phase_match`, `advanced_acoustic_pack`, and `advanced_sensor_calibration_pack`. Those entries are catalog-visible so the comprehensive roadmap is auditable without claiming support before estimator conventions, calibration data, dependency review, fixtures, and performance evidence exist.

## Future-Gated Coverage

The catalog also includes implemented M36 closure work, dependency-gated exact elliptic/Cauer design, dependency-gated efficient polyphase resampling, dependency/design-gated Hilbert envelope, optimized FFT dependency/performance follow-up, advanced M35 domain entries, and the future-gated `split_by_event` segmentation entry.

The implemented M36 closure entry is `comprehensive_suite_closure`. It is a catalog/readiness artifact, not a waveform filter, feature transform, runtime-loader implementation, package-export expansion, hardware evidence, or certification claim.

Planned entries use `future_gated`, `dependency_gated`, `research`, or `planned` status until implementation milestones add code, fixtures, docs, package decisions, and validation evidence.

## Rule-Package Compatibility

Rule-package export now consults the catalog before converting filters into schema filters.

Current package-supported transforms are:

- `offset`
- `gain`
- `invert`
- `moving_average`
- `low_pass`
- `adc_quantize`

Desktop-only implemented transforms remain rejected with this error shape:

```text
rule package export does not yet support transform `<name>`
```

Future milestones may update package support only after adding schema representation, runtime-profile evidence, known-answer fixtures, export tests, and compatibility documentation.

All M26 data-cleaning/timing-conditioning transforms, all M27 nonlinear/normalization transforms, all M28 smoothing/baseline transforms, all M29 frequency-filter transforms, all M30 resampling/timing transforms, all M31 waveform filters, all M32 waveform filters, all M34 fault-injection/ADC-DAC simulation filters, and all M35 multi-channel/sensor/domain conditioning filters are currently desktop-only and rejected for rule-package export. M31-M33 `[[feature_transforms]]` are feature-record evidence and are not exported to rule packages.

## Runtime And Evidence Boundaries

Current implemented transform catalog entries support the `desktop` runtime profile only unless separately documented. Planned entries use `future_gated` runtime support.

The catalog is software evidence only. It does not claim:

- live DAQ support,
- HAL or RTOS support,
- target hardware execution,
- real-time guarantees,
- hardware calibration,
- hardware qualification,
- certification evidence.

## M25 Acceptance Evidence

| Acceptance Criterion | Evidence |
|---|---|
| Every existing transform appears in the catalog. | `transform_catalog::tests::implemented_waveform_filter_metadata_matches_catalog`, `transform_catalog::tests::m26_data_cleaning_filter_metadata_matches_catalog`, `transform_catalog::tests::m27_pointwise_filter_metadata_matches_catalog`, `transform_catalog::tests::m28_smoothing_baseline_filter_metadata_matches_catalog`, M30/M31/M32/M33 metadata tests, `transform_catalog::tests::m34_fault_injection_adc_dac_metadata_matches_catalog`, `transform_catalog::tests::m35_multi_channel_sensor_domain_entries_are_cataloged`, feature-record catalog tests, and `transform_catalog::tests::implemented_event_transform_metadata_matches_catalog`. |
| Every existing transform has metadata tests. | Focused M25, M26, M27, M28, M29, M30, M31, M32, M33, M34, and M35 tests compare emitted metadata to catalog entries. |
| Docs can list supported transforms without stale manual duplication. | `ferrisoxide-signal transforms --format text/json` renders from the Rust catalog. |
| Unsupported transforms still fail clearly in rule-package export. | CLI export checks `FilterStep::rule_package_export_supported()` before schema conversion. Existing rejection tests remain in place. |
| Planned work is separated from implemented work. | Catalog entries carry `capability_status`, `evidence_level`, `milestone`, `runtime_profiles`, and `package_support`. |

## Update Rule

When a later milestone implements or changes a transform, update:

1. `crates/ferrisoxide-core/src/transform_catalog.rs`.
2. Transform implementation/config/report code.
3. Metadata tests comparing emitted metadata to catalog entries.
4. Rule-package compatibility tests if package support changes.
5. Relevant docs, requirements, traceability, risk, pipeline report, and validation log.

## Hand-Off Note

Role: Software Architect / Documentation Engineer
Goal: Implement the M25 transform registry and completeness contract.
Files changed: `crates/ferrisoxide-core/src/transform_catalog.rs`, `crates/ferrisoxide-core/src/lib.rs`, `crates/ferrisoxide-core/src/filter.rs`, `crates/ferrisoxide-cli/src/main.rs`, and this document.
Checks run: See `docs/validation-log.md`.
Status: Complete and merged through PR #175 for M25-M36.
Known gaps: Exact elliptic/Cauer design, efficient polyphase resampling, Hilbert envelope, optimized FFT dependency/performance follow-up, phase-difference estimation, gain/phase matching, advanced acoustic/domain packs, and large spectral-output UX remain gated; `split_by_event` remains future-gated as a multi-artifact segmentation feature.
Next recommended step: Choose one gated advanced follow-up or a separate release-publication plan.
