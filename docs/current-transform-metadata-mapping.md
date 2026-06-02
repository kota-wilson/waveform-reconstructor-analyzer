# Current Transform Metadata Mapping

Date: 2026-06-02

Status: Current mapping artifact updated through local M35 multi-channel, sensor, and domain conditioning filters and checked against the M25 transform catalog.

## Purpose

FerrisOxide currently implements waveform transform steps in `crates/ferrisoxide-core/src/filter.rs`:

- `moving_average`
- `low_pass`
- `adc_quantize`
- `offset`
- `gain`
- `invert`
- `clamp`
- `deadband`
- `dc_remove`
- `baseline_subtract`
- `high_pass_baseline`
- `moving_median`
- `timestamp_sort`
- `dedupe_timestamps`
- `nan_interpolate`
- `nan_remove`
- `crop`
- `fixed_delay`
- `gap_fill`
- `resample_fixed`
- `channel_delay`
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
- `rolling_mean`
- `rolling_variance`
- `rolling_stddev`
- `rolling_min`
- `rolling_max`
- `z_score`
- `outlier_detection`
- `quantile_clip`

M12 also implements event and validation transform metadata in `crates/ferrisoxide-core/src/event.rs` for `event_records[].transform_metadata` and `event_validations[].transform_metadata`.

M31 and M32 also implement feature transform metadata in `crates/ferrisoxide-core/src/feature.rs` for `feature_records[].transform_metadata`.

This document defines the structured metadata values these transforms emit in reports.

M25 adds `crates/ferrisoxide-core/src/transform_catalog.rs` and `docs/transform-catalog.md` as the source-of-truth catalog. This mapping document remains behavior detail for the currently implemented transforms; catalog tests compare emitted metadata to the source-of-truth entries.

## Compatibility Rule

The existing `transform_history` strings remain the compatibility field:

- `moving_average(window_samples={window_samples})`
- `low_pass(cutoff_hz={cutoff_hz})`
- `adc_quantize(bits={bits},min_v={min_v},max_v={max_v})`
- `offset(offset_v={offset_v})`
- `gain(gain={gain})`
- `invert()`
- `clamp(min_v={min_v},max_v={max_v})`
- `deadband(threshold_v={threshold_v})`
- `dc_remove()`
- `baseline_subtract(baseline_v={baseline_v})`
- `high_pass_baseline(cutoff_hz={cutoff_hz})`
- `moving_median(window_samples={window_samples})`
- `timestamp_sort(order=ascending)`
- `dedupe_timestamps(policy=keep_first)`
- `nan_interpolate()`
- `nan_remove(policy=drop_rows_with_nan)`
- `crop(start_time_s={start_time_s},end_time_s={end_time_s})`
- `fixed_delay(delay_s={delay_s})`
- `gap_fill(sample_interval_s={sample_interval_s})`
- `resample_fixed(sample_interval_s={sample_interval_s})`
- `channel_delay(channel={channel},delay_s={delay_s})`
- `absolute_value()`
- `square()`
- `square_root()`
- `log(base={base})`
- `exp(base={base})`
- `normalize(mode={mode})` or `normalize(mode=range,input_min_v={input_min_v},input_max_v={input_max_v},output_min={output_min},output_max={output_max})`
- `tanh()`
- `sigmoid()`
- `soft_limit(limit_v={limit_v})`
- `piecewise_linear(points={point_count})`
- `polynomial(coefficients={coefficient_count})`
- `weighted_moving_average(weights={weight_count})`
- `exponential_moving_average(alpha={alpha})`
- `boxcar_smoothing(window_samples={window_samples})`
- `gaussian_smoothing(window_samples={window_samples},sigma_samples={sigma_samples})`
- `savitzky_golay(window_samples={window_samples},polynomial_order={polynomial_order})`
- `centered_moving_median(window_samples={window_samples})`
- `rolling_mean_baseline(window_samples={window_samples})`
- `rolling_median_baseline(window_samples={window_samples})`
- `linear_detrend()`
- `polynomial_detrend(polynomial_order={polynomial_order})`
- `hampel_filter(window_samples={window_samples},outlier_sigma={outlier_sigma})`
- `spike_remove(window_samples={window_samples},threshold_v={threshold_v})`
- `fir_filter(coefficients={coefficient_count})`
- `zero_phase_fir_filter(coefficients={coefficient_count})`
- `iir_biquad(coefficients=5)`
- `zero_phase_iir_biquad(coefficients=5)`
- `high_pass(cutoff_hz={cutoff_hz})`
- `band_pass(center_hz={center_hz},q={q})`
- `band_stop(center_hz={center_hz},q={q})`
- `notch(center_hz={center_hz},q={q})`
- `comb_filter(delay_samples={delay_samples},feedback_gain={feedback_gain})`
- `butterworth_low_pass(cutoff_hz={cutoff_hz})`
- `butterworth_high_pass(cutoff_hz={cutoff_hz})`
- `chebyshev1_low_pass(cutoff_hz={cutoff_hz},ripple_db={ripple_db})`
- `chebyshev2_low_pass(cutoff_hz={cutoff_hz},stopband_attenuation_db={stopband_attenuation_db})`
- `bessel_low_pass(cutoff_hz={cutoff_hz})`
- `resample(sample_interval_s={sample_interval_s})`
- `downsample(factor={factor})`
- `decimate(factor={factor},cutoff_hz={cutoff_hz})`
- `upsample(factor={factor})`
- `interpolate(sample_interval_s={sample_interval_s})`
- `rational_resample(upsample_factor={upsample_factor},downsample_factor={downsample_factor})`
- `sample_and_hold(sample_interval_s={sample_interval_s})`
- `zero_order_hold(sample_interval_s={sample_interval_s})`
- `first_order_hold(sample_interval_s={sample_interval_s})`
- `fractional_delay(delay_s={delay_s})`
- `cross_correlation_delay(reference_channel={reference_channel},target_channel={target_channel},max_lag_samples={max_lag_samples})`
- `jitter_correction(sample_interval_s={sample_interval_s})`
- `clock_drift_correction(sample_interval_s={sample_interval_s})`
- `half_wave_rectify()`
- `full_wave_rectify()`
- `envelope(alpha={alpha})`
- `moving_rms(window_samples={window_samples})`
- `peak_hold()`
- `first_derivative()`
- `second_derivative()`
- `integral()`
- `cumulative_integral()`
- `leaky_integrator(time_constant_s={time_constant_s})`
- `slope_detection(threshold_per_s={threshold_per_s})`
- feature-record history labels use `{feature_name}(channel={channel})`.

Structured metadata must keep `history_label` equal to the matching `transform_history` entry for the same sequence index.

## Shared Mapping Rules

| Field | Rule |
|---|---|
| `sequence_index` | Zero-based position in the applied filter/transform chain. |
| `input_channels.kind` | `all_channels` for all current transforms. |
| `output_channels.kind` | `derived_channels` for waveform transforms, `event_records` for event transforms, `validation_records` for event validation transforms, and `feature_records` for feature transforms. |
| `output_channels.preserves_names` | `true` for waveform transforms and `false` for event/validation record outputs. |
| `runtime_profiles` | `["desktop"]` for current implementation support. |
| `capability_status` | `implemented` for all current transforms. |
| `evidence_level` | `golden_report_tested` for all current transforms because current behavior is covered by unit/fixture/golden report paths. |

No current transform is exposed as Raspberry Pi 5 no_std or Pico 2 runtime support by this mapping. Those profiles require later explicit code, target, and parity evidence.

The Schmitt state primitive in `ferrisoxide-rule-engine` is no_std-compatible, but the M12 report-oriented event pipeline remains desktop-only until a later bounded-buffer runtime exists.

## M12 Event And Validation Transforms

| Transform | `name` | `category` | `output_channels.kind` | `parameters` | `sample_rate_required` | `stateful` | `causal` | `phase_effect` | `streaming_supported` | `offline_only` |
|---|---|---|---|---|---|---|---|---|---|---|
| Schmitt trigger | `schmitt_trigger` | `StatefulTransform` | `event_records` | `on_threshold_v`, `off_threshold_v` in `V` | true | true | true | `nonlinear` | false | true |
| Debounce | `debounce` | `StatefulTransform` | `event_records` | `min_duration_s` in `s` | true | true | true | `nonlinear` | false | true |
| Glitch removal | `glitch_removal` | `StatefulTransform` | `event_records` | `max_duration_s` in `s` | true | true | true | `nonlinear` | false | true |
| Edge extraction | `edge_extraction` | `EventTransform` | `event_records` | none | true | false | true | `nonlinear` | false | true |
| Bounce detection | `bounce_detection` | `StatefulTransform` | `event_records` | `window_s` in `s` | true | true | true | `nonlinear` | false | true |
| Missing pulse validation | `missing_pulse` | `ValidationTransform` | `validation_records` | `expected_count` in `events` | true | false | true | `nonlinear` | false | true |
| Extra pulse validation | `extra_pulse` | `ValidationTransform` | `validation_records` | `max_count` in `events` | true | false | true | `nonlinear` | false | true |
| Dwell-time validation | `dwell_time` | `ValidationTransform` | `validation_records` | `min_duration_s` in `s` | true | false | true | `nonlinear` | false | true |
| Timeout validation | `timeout` | `ValidationTransform` | `validation_records` | `start_time_s`, `max_time_s` in `s` | true | false | true | `nonlinear` | false | true |

Event metadata is nested under event and validation evidence rather than `waveform_metadata.transform_steps`, because event transforms do not create derived waveform channels.

## M11 Pointwise, Baseline, And Windowed Transforms

| Transform | `name` | `category` | `parameters` | `sample_rate_required` | `stateful` | `causal` | `phase_effect` | `streaming_supported` | `offline_only` |
|---|---|---|---|---|---|---|---|---|---|
| Offset | `offset` | `PointwiseTransform` | `offset_v` in `V` | false | false | true | `none` | true | false |
| Gain | `gain` | `PointwiseTransform` | `gain` as `ratio` | false | false | true | `none` | true | false |
| Invert | `invert` | `PointwiseTransform` | none | false | false | true | `none` | true | false |
| Clamp | `clamp` | `PointwiseTransform` | `min_v`, `max_v` in `V` | false | false | true | `nonlinear` | true | false |
| Deadband | `deadband` | `PointwiseTransform` | `threshold_v` in `V` | false | false | true | `nonlinear` | true | false |
| DC removal | `dc_remove` | `BaselineTransform` | none | false | true | false | `none` | false | true |
| Baseline subtraction | `baseline_subtract` | `BaselineTransform` | `baseline_v` in `V` | false | false | true | `none` | true | false |
| High-pass baseline correction | `high_pass_baseline` | `StatefulTransform` | `cutoff_hz` in `Hz` | true | true | true | `delay` | true | false |
| Moving median | `moving_median` | `WindowedTransform` | `window_samples` in `samples` | false | true | true | `nonlinear` | true | false |

M14 implements first-order high-pass baseline correction as desktop-only support. It is not portable rule-package export support, embedded runtime support, calibrated drift removal, or hardware timing evidence.

## M27 Pointwise, Normalization, And Nonlinear Conditioning

| Transform | `name` | `category` | `parameters` | `sample_rate_required` | `stateful` | `causal` | `phase_effect` | `streaming_supported` | `offline_only` |
|---|---|---|---|---|---|---|---|---|---|
| Absolute value | `absolute_value` | `PointwiseTransform` | none | false | false | true | `nonlinear` | true | false |
| Square | `square` | `PointwiseTransform` | none | false | false | true | `nonlinear` | true | false |
| Square root | `square_root` | `PointwiseTransform` | none | false | false | true | `nonlinear` | true | false |
| Log | `log` | `PointwiseTransform` | `base` as `ratio` | false | false | true | `nonlinear` | true | false |
| Exponential | `exp` | `PointwiseTransform` | `base` as `ratio` | false | false | true | `nonlinear` | true | false |
| Normalize | `normalize` | `PointwiseTransform` | `mode`; range mode adds `input_min_v`, `input_max_v`, `output_min`, `output_max` | false | true | false | `none` | false | true |
| Tanh | `tanh` | `PointwiseTransform` | none | false | false | true | `nonlinear` | true | false |
| Sigmoid | `sigmoid` | `PointwiseTransform` | none | false | false | true | `nonlinear` | true | false |
| Soft limit | `soft_limit` | `PointwiseTransform` | `limit_v` in `V` | false | false | true | `nonlinear` | true | false |
| Piecewise linear | `piecewise_linear` | `PointwiseTransform` | `point_count` in `points` | false | false | true | `nonlinear` | true | false |
| Polynomial | `polynomial` | `PointwiseTransform` | `coefficient_count` in `coefficients` | false | false | true | `nonlinear` | true | false |

Behavior notes:

- M27 transforms produce derived waveforms and preserve raw waveform samples.
- `square_root` rejects negative samples, and `log` rejects non-positive samples.
- `log` and `exp` require finite bases greater than zero and not equal to one.
- `normalize` is offline-only because record-level min/max or statistics are needed for common modes.
- `piecewise_linear` requires at least two finite points with strictly increasing `x` values and holds endpoint values outside the configured range.
- `polynomial` evaluates coefficients in increasing order, `c0 + c1*x + c2*x^2...`.
- Rule-package export rejects all M27 transforms until separate package/runtime semantics are approved.

## M28 Smoothing, Detrending, Baseline, And Spike Cleanup

| Transform | `name` | `category` | `parameters` | `sample_rate_required` | `stateful` | `causal` | `phase_effect` | `streaming_supported` | `offline_only` |
|---|---|---|---|---|---|---|---|---|---|
| Weighted moving average | `weighted_moving_average` | `WindowedTransform` | `weight_count` in `weights` | false | true | true | `delay` | true | false |
| Exponential moving average | `exponential_moving_average` | `StatefulTransform` | `alpha` as `ratio` | false | true | true | `delay` | true | false |
| Boxcar smoothing | `boxcar_smoothing` | `WindowedTransform` | `window_samples` in `samples` | false | true | false | `none` | false | true |
| Gaussian smoothing | `gaussian_smoothing` | `WindowedTransform` | `window_samples`, `sigma_samples` in `samples` | false | true | false | `none` | false | true |
| Savitzky-Golay smoothing | `savitzky_golay` | `WindowedTransform` | `window_samples` in `samples`, `polynomial_order` in `order` | false | true | false | `none` | false | true |
| Centered moving median | `centered_moving_median` | `WindowedTransform` | `window_samples` in `samples` | false | true | false | `nonlinear` | false | true |
| Rolling mean baseline | `rolling_mean_baseline` | `BaselineTransform` | `window_samples` in `samples` | false | true | true | `delay` | true | false |
| Rolling median baseline | `rolling_median_baseline` | `BaselineTransform` | `window_samples` in `samples` | false | true | true | `nonlinear` | true | false |
| Linear detrend | `linear_detrend` | `BaselineTransform` | none | true | true | false | `none` | false | true |
| Polynomial detrend | `polynomial_detrend` | `BaselineTransform` | `polynomial_order` in `order` | true | true | false | `none` | false | true |
| Hampel filter | `hampel_filter` | `WindowedTransform` | `window_samples` in `samples`, `outlier_sigma` in `sigma` | false | true | false | `nonlinear` | false | true |
| Spike removal | `spike_remove` | `WindowedTransform` | `window_samples` in `samples`, `threshold_v` in `V` | false | true | false | `nonlinear` | false | true |

Behavior notes:

- M28 transforms produce derived waveforms and preserve raw waveform samples.
- `weighted_moving_average` and `exponential_moving_average` are causal, streaming-supported smoothers with phase delay.
- `boxcar_smoothing`, `gaussian_smoothing`, `savitzky_golay`, `centered_moving_median`, `hampel_filter`, and `spike_remove` are offline centered-window operations.
- Centered boxcar, Gaussian, and median smoothing use shrinking edge windows. Savitzky-Golay, Hampel, and spike removal use fixed centered windows when enough samples exist, shifted at edges.
- `linear_detrend` and `polynomial_detrend` require a strictly increasing time axis and are baseline-removal transforms, not calibrated drift correction.
- Rule-package export rejects all M28 transforms until separate package/runtime semantics are approved.

## M29 Standard Frequency Filters

| Transform | `name` | `category` | `parameters` | `sample_rate_required` | `stateful` | `causal` | `phase_effect` | `streaming_supported` | `offline_only` |
|---|---|---|---|---|---|---|---|---|---|
| FIR coefficient filter | `fir_filter` | `FrequencyFilterTransform` | `coefficient_count` in `coefficients` | false | true | true | `delay` | true | false |
| Zero-phase FIR filter | `zero_phase_fir_filter` | `FrequencyFilterTransform` | `coefficient_count` in `coefficients` | false | true | false | `none` | false | true |
| IIR biquad coefficient filter | `iir_biquad` | `FrequencyFilterTransform` | `coefficient_count` in `coefficients` | false | true | true | `delay` | true | false |
| Zero-phase IIR biquad | `zero_phase_iir_biquad` | `FrequencyFilterTransform` | `coefficient_count` in `coefficients` | false | true | false | `none` | false | true |
| High-pass | `high_pass` | `FrequencyFilterTransform` | `cutoff_hz` in `Hz` | true | true | true | `delay` | true | false |
| Band-pass | `band_pass` | `FrequencyFilterTransform` | `center_hz` in `Hz`, `q` as `ratio` | true | true | true | `delay` | true | false |
| Band-stop | `band_stop` | `FrequencyFilterTransform` | `center_hz` in `Hz`, `q` as `ratio` | true | true | true | `delay` | true | false |
| Notch | `notch` | `FrequencyFilterTransform` | `center_hz` in `Hz`, `q` as `ratio` | true | true | true | `delay` | true | false |
| Comb filter | `comb_filter` | `FrequencyFilterTransform` | `delay_samples` in `samples`, `feedback_gain` as `ratio` | false | true | true | `delay` | true | false |
| Butterworth low-pass | `butterworth_low_pass` | `FrequencyFilterTransform` | `cutoff_hz` in `Hz` | true | true | true | `delay` | true | false |
| Butterworth high-pass | `butterworth_high_pass` | `FrequencyFilterTransform` | `cutoff_hz` in `Hz` | true | true | true | `delay` | true | false |
| Chebyshev Type I low-pass | `chebyshev1_low_pass` | `FrequencyFilterTransform` | `cutoff_hz` in `Hz`, `ripple_db` in `dB` | true | true | true | `delay` | true | false |
| Chebyshev Type II low-pass | `chebyshev2_low_pass` | `FrequencyFilterTransform` | `cutoff_hz` in `Hz`, `stopband_attenuation_db` in `dB` | true | true | true | `delay` | true | false |
| Bessel low-pass | `bessel_low_pass` | `FrequencyFilterTransform` | `cutoff_hz` in `Hz` | true | true | true | `delay` | true | false |

Behavior notes:

- M29 filters produce derived waveforms and preserve raw waveform samples.
- Generic FIR and IIR coefficient filters validate finite coefficients; biquad filters also validate poles are inside the unit circle.
- Designed filters require a uniform sample interval and reject cutoff or center frequencies at or above Nyquist.
- Zero-phase filters are offline-only forward/backward transforms with `phase_effect = none`.
- Exact elliptic/Cauer design remains cataloged as dependency-gated until a numeric-library review is approved.
- Rule-package export rejects all M29 transforms until separate package/runtime semantics are approved.

## M30 Resampling And Timing Alignment

| Transform | `name` | `category` | `parameters` | `sample_rate_required` | `stateful` | `causal` | `phase_effect` | `streaming_supported` | `offline_only` |
|---|---|---|---|---|---|---|---|---|---|
| Fixed-grid resampling | `resample` | `ResamplingTransform` | `sample_interval_s` in `s` | true | false | false | `none` | false | true |
| Downsampling | `downsample` | `ResamplingTransform` | `factor` as `ratio` | true | false | true | `none` | true | false |
| Decimation | `decimate` | `ResamplingTransform` | `factor` as `ratio`, `cutoff_hz` in `Hz` | true | true | true | `delay` | true | false |
| Upsampling | `upsample` | `ResamplingTransform` | `factor` as `ratio` | true | false | false | `none` | false | true |
| Interpolation | `interpolate` | `ResamplingTransform` | `sample_interval_s` in `s` | true | false | false | `none` | false | true |
| Rational resampling | `rational_resample` | `ResamplingTransform` | `upsample_factor`, `downsample_factor`, and derived `sample_interval_s` | true | false | false | `none` | false | true |
| Sample-and-hold | `sample_and_hold` | `ResamplingTransform` | `sample_interval_s` in `s` | true | true | false | `delay` | false | true |
| Zero-order hold | `zero_order_hold` | `ResamplingTransform` | `sample_interval_s` in `s` | true | true | false | `delay` | false | true |
| First-order hold | `first_order_hold` | `ResamplingTransform` | `sample_interval_s` in `s` | true | false | false | `none` | false | true |
| Fractional delay | `fractional_delay` | `ResamplingTransform` | `delay_s` in `s` | true | false | false | `delay` | false | true |
| Cross-correlation delay alignment | `cross_correlation_delay` | `ResamplingTransform` | `reference_channel`, `target_channel`, `max_lag_samples`, computed `estimated_lag_samples`, `estimated_delay_s`, and `confidence` | true | true | false | `delay` | false | true |
| Jitter correction | `jitter_correction` | `ResamplingTransform` | `sample_interval_s` in `s` | true | false | false | `none` | false | true |
| Clock-drift correction | `clock_drift_correction` | `ResamplingTransform` | `sample_interval_s` and computed `end_drift_s` in `s` | true | false | false | `none` | false | true |

Behavior notes:

- M30 transforms produce derived waveforms and preserve raw waveform samples.
- Fixed-grid interpolation and hold transforms require positive finite `sample_interval_s` and a valid increasing time axis.
- `decimate` validates an integer factor greater than one, requires uniform timing, applies a first-order low-pass prefilter, and rejects cutoffs above the target Nyquist frequency.
- `rational_resample` is dependency-free linear interpolation onto the rational target grid; efficient polyphase resampling remains dependency/performance-gated.
- `fractional_delay` delays or advances all channels using linear interpolation and endpoint hold.
- `cross_correlation_delay` estimates an integer sample lag between the reference and target channels, records lag/delay/confidence metadata, and then aligns the target channel.
- `jitter_correction` and `clock_drift_correction` are offline fixed-grid repair transforms; they are not DAQ clock calibration or hardware timing evidence.
- Rule-package export rejects all M30 transforms until separate package/runtime semantics are approved.

## M31 Envelope, Energy, And Calculus

| Transform | `name` | `category` | `output_channels.kind` | `parameters` | `sample_rate_required` | `stateful` | `causal` | `phase_effect` | `streaming_supported` | `offline_only` |
|---|---|---|---|---|---|---|---|---|---|---|
| Half-wave rectification | `half_wave_rectify` | `PointwiseTransform` | `derived_channels` | none | false | false | true | `nonlinear` | true | false |
| Full-wave rectification | `full_wave_rectify` | `PointwiseTransform` | `derived_channels` | none | false | false | true | `nonlinear` | true | false |
| Envelope | `envelope` | `StatefulTransform` | `derived_channels` | `alpha` as `ratio` | false | true | true | `delay` | true | false |
| Moving RMS | `moving_rms` | `WindowedTransform` | `derived_channels` | `window_samples` in `samples` | false | true | true | `delay` | true | false |
| Peak hold | `peak_hold` | `StatefulTransform` | `derived_channels` | none | false | true | true | `nonlinear` | true | false |
| First derivative | `first_derivative` | `FeatureTransform` | `derived_channels` | none | true | true | true | `delay` | true | false |
| Second derivative | `second_derivative` | `FeatureTransform` | `derived_channels` | none | true | true | true | `delay` | true | false |
| Integral | `integral` | `FeatureTransform` | `derived_channels` | none | true | true | true | `delay` | true | false |
| Cumulative integral | `cumulative_integral` | `FeatureTransform` | `derived_channels` | none | true | true | true | `delay` | true | false |
| Leaky integrator | `leaky_integrator` | `StatefulTransform` | `derived_channels` | `time_constant_s` in `s` | true | true | true | `delay` | true | false |
| Slope detection | `slope_detection` | `FeatureTransform` | `derived_channels` | `threshold_per_s` in `unit/s` | true | true | true | `nonlinear` | true | false |
| RMS feature | `rms` | `FeatureTransform` | `feature_records` | `channel` | false | false | false | `none` | true | false |
| Peak-to-peak feature | `peak_to_peak` | `FeatureTransform` | `feature_records` | `channel` | false | false | false | `none` | true | false |
| Crest factor feature | `crest_factor` | `FeatureTransform` | `feature_records` | `channel` | false | false | false | `none` | true | false |
| Energy feature | `energy` | `FeatureTransform` | `feature_records` | `channel` | true | false | false | `none` | false | true |
| Power feature | `power` | `FeatureTransform` | `feature_records` | `channel` | true | false | false | `none` | false | true |
| Area-under-curve feature | `area_under_curve` | `FeatureTransform` | `feature_records` | `channel` | true | false | false | `none` | false | true |
| Impulse estimate feature | `impulse_estimate` | `FeatureTransform` | `feature_records` | `channel` | true | false | false | `none` | false | true |

Behavior notes:

- M31 waveform filters produce derived waveforms and preserve raw waveform samples.
- M31 scalar calculations emit `feature_records` and do not change `overall_outcome`.
- Calculus and energy-style features require a finite strictly increasing time axis.
- Feature units are recorded as the source unit, `ratio`, `<unit>^2*s`, `<unit>^2`, or `<unit>*s`.
- Hilbert envelope remains dependency/design-gated.
- Rule-package export rejects all M31 waveform filters, and `feature_transforms` are not exported to rule packages.

## M32 Statistics And Correlation

| Transform | `name` | `category` | `output_channels.kind` | `parameters` | `sample_rate_required` | `stateful` | `causal` | `phase_effect` | `streaming_supported` | `offline_only` |
|---|---|---|---|---|---|---|---|---|---|---|
| Rolling mean | `rolling_mean` | `WindowedTransform` | `derived_channels` | `window_samples` in `samples` | false | true | true | `delay` | true | false |
| Rolling variance | `rolling_variance` | `WindowedTransform` | `derived_channels` | `window_samples` in `samples` | false | true | true | `delay` | true | false |
| Rolling standard deviation | `rolling_stddev` | `WindowedTransform` | `derived_channels` | `window_samples` in `samples` | false | true | true | `delay` | true | false |
| Rolling minimum | `rolling_min` | `WindowedTransform` | `derived_channels` | `window_samples` in `samples` | false | true | true | `delay` | true | false |
| Rolling maximum | `rolling_max` | `WindowedTransform` | `derived_channels` | `window_samples` in `samples` | false | true | true | `delay` | true | false |
| Z-score waveform | `z_score` | `FeatureTransform` | `derived_channels` | none | false | false | false | `nonlinear` | false | true |
| Outlier detection | `outlier_detection` | `FeatureTransform` | `derived_channels` | `threshold_sigma` in `sigma` | false | false | false | `nonlinear` | false | true |
| Quantile clipping | `quantile_clip` | `FeatureTransform` | `derived_channels` | `lower_quantile`, `upper_quantile` as `ratio` | false | false | false | `nonlinear` | false | true |
| Mean feature | `mean` | `FeatureTransform` | `feature_records` | `channel` | false | false | false | `none` | false | true |
| Median feature | `median` | `FeatureTransform` | `feature_records` | `channel` | false | false | false | `none` | false | true |
| Mode feature | `mode` | `FeatureTransform` | `feature_records` | `channel` | false | false | false | `none` | false | true |
| Minimum feature | `min` | `FeatureTransform` | `feature_records` | `channel` | false | false | false | `none` | false | true |
| Maximum feature | `max` | `FeatureTransform` | `feature_records` | `channel` | false | false | false | `none` | false | true |
| Variance feature | `variance` | `FeatureTransform` | `feature_records` | `channel` | false | false | false | `none` | false | true |
| Standard-deviation feature | `standard_deviation` | `FeatureTransform` | `feature_records` | `channel` | false | false | false | `none` | false | true |
| Skewness feature | `skewness` | `FeatureTransform` | `feature_records` | `channel` | false | false | false | `none` | false | true |
| Kurtosis feature | `kurtosis` | `FeatureTransform` | `feature_records` | `channel` | false | false | false | `none` | false | true |
| Percentile feature | `percentile` | `FeatureTransform` | `feature_records` | `channel`, `percentile` | false | false | false | `none` | false | true |
| Quantile feature | `quantile` | `FeatureTransform` | `feature_records` | `channel`, `quantile` | false | false | false | `none` | false | true |
| Histogram feature | `histogram` | `FeatureTransform` | `feature_records` | `channel`, `bins`, optional range | false | false | false | `none` | false | true |
| Covariance feature | `covariance` | `FeatureTransform` | `feature_records` | `channel`, `other_channel` | false | false | false | `none` | false | true |
| Correlation feature | `correlation` | `FeatureTransform` | `feature_records` | `channel`, `other_channel` | false | false | false | `none` | false | true |
| Autocorrelation feature | `autocorrelation` | `FeatureTransform` | `feature_records` | `channel`, `lag_samples` | false | false | false | `none` | false | true |
| Cross-correlation feature | `cross_correlation` | `FeatureTransform` | `feature_records` | `channel`, `other_channel`, `lag_samples` | false | false | false | `none` | false | true |

Behavior notes:

- M32 rolling waveform filters use trailing windows with shrinking startup windows.
- M32 distribution waveform filters are offline-only and reject constant or invalid quantile inputs where required.
- M32 scalar calculations emit `feature_records`; histogram emits one feature record per bin using `{id}_bin_{index}` IDs.
- Method context records percentile, quantile, histogram bin bounds, comparison channel, and lag samples when applicable.
- Lagged correlation convention is `channel[t]` compared with `other_channel[t + lag_samples]`; autocorrelation applies the same convention to one channel.
- Rule-package export rejects all M32 waveform filters, and `feature_transforms` are not exported to rule packages.

## M33 Spectrum, Windows, And Time-Frequency

| Transform | `name` | `category` | `output_channels.kind` | `parameters` | `sample_rate_required` | `stateful` | `causal` | `phase_effect` | `streaming_supported` | `offline_only` |
|---|---|---|---|---|---|---|---|---|---|---|
| Window coefficients | `window_function` | `FeatureTransform` | `feature_records` | `channel`, `window`, optional `window_samples` | false | false | false | `none` | false | true |
| DFT amplitude spectrum | `dft` | `FrequencyFilterTransform` | `feature_records` | `channel`, `window` | true | false | false | `none` | false | true |
| FFT amplitude spectrum | `fft` | `FrequencyFilterTransform` | `feature_records` | `channel`, `window` | true | false | false | `none` | false | true |
| Inverse FFT reconstruction | `ifft` | `FrequencyFilterTransform` | `feature_records` | `channel`, optional `other_channel` | false | false | false | `none` | false | true |
| Power spectrum | `power_spectrum` | `FrequencyFilterTransform` | `feature_records` | `channel`, `window` | true | false | false | `none` | false | true |
| Power spectral density | `psd` | `FrequencyFilterTransform` | `feature_records` | `channel`, `window` | true | false | false | `none` | false | true |
| Welch PSD | `welch_psd` | `FrequencyFilterTransform` | `feature_records` | `channel`, `window`, `window_samples`, `overlap_samples` | true | true | false | `none` | false | true |
| Cross spectrum | `cross_spectrum` | `FrequencyFilterTransform` | `feature_records` | `channel`, `other_channel`, `window` | true | true | false | `none` | false | true |
| Coherence | `coherence` | `FrequencyFilterTransform` | `feature_records` | `channel`, `other_channel`, `window` | true | true | false | `none` | false | true |
| Transfer function estimate | `transfer_function` | `FrequencyFilterTransform` | `feature_records` | `channel`, `other_channel`, `window` | true | true | false | `none` | false | true |
| Harmonic analysis | `harmonic_analysis` | `FeatureTransform` | `feature_records` | `channel`, `window`, optional `fundamental_hz`, `harmonic_count` | true | false | false | `none` | false | true |
| THD | `thd` | `FeatureTransform` | `feature_records` | `channel`, `window`, optional `fundamental_hz`, `harmonic_count` | true | false | false | `none` | false | true |
| SNR | `snr` | `FeatureTransform` | `feature_records` | `channel`, `window`, optional `fundamental_hz`, `harmonic_count` | true | false | false | `none` | false | true |
| SINAD | `sinad` | `FeatureTransform` | `feature_records` | `channel`, `window`, optional `fundamental_hz`, `harmonic_count` | true | false | false | `none` | false | true |
| ENOB | `enob` | `FeatureTransform` | `feature_records` | `channel`, `window`, optional `fundamental_hz`, `harmonic_count` | true | false | false | `none` | false | true |
| STFT | `stft` | `TimeFrequencyTransform` | `feature_records` | `channel`, `window`, `window_samples`, `overlap_samples` | true | true | false | `none` | false | true |
| Spectrogram | `spectrogram` | `TimeFrequencyTransform` | `feature_records` | `channel`, `window`, `window_samples`, `overlap_samples` | true | true | false | `none` | false | true |
| Spectral centroid | `spectral_centroid` | `FeatureTransform` | `feature_records` | `channel`, `window` | true | false | false | `none` | false | true |
| Spectral bandwidth | `spectral_bandwidth` | `FeatureTransform` | `feature_records` | `channel`, `window` | true | false | false | `none` | false | true |
| Spectral rolloff | `spectral_rolloff` | `FeatureTransform` | `feature_records` | `channel`, `window`, optional `rolloff_percent` | true | false | false | `none` | false | true |
| Band power | `band_power` | `FeatureTransform` | `feature_records` | `channel`, `window`, `band_low_hz`, `band_high_hz` | true | false | false | `none` | false | true |

Behavior notes:

- M33 uses dependency-free radix-2 FFT for power-of-two sample counts and DFT fallback for non-power-of-two/reference inputs.
- Spectrum records use one-sided real-input bins from DC through Nyquist and include frequency, bin width, complex value, amplitude/magnitude, phase, window, and normalization context.
- `psd`, `welch_psd`, and `spectrogram` use one-sided PSD scaling with `sample_rate_hz * sum(window^2)`.
- `welch_psd`, `stft`, and `spectrogram` use complete segments only and reject invalid overlap/window combinations.
- M33 records are desktop/offline feature evidence and do not mutate waveform samples or directly decide pass/fail.
- No new numeric dependency is added; optimized FFT dependency/performance work remains future-gated.

## M34 Fault Injection And ADC/DAC Simulation

| Transform | `name` | `category` | `output_channels.kind` | `parameters` | `sample_rate_required` | `stateful` | `causal` | `phase_effect` | `streaming_supported` | `offline_only` |
|---|---|---|---|---|---|---|---|---|---|---|
| White-noise injection | `white_noise` | `FaultInjectionTransform` | `derived_channels` | `amplitude_v`, `seed`, `evidence_scope = simulation_only` | false | true | false | `nonlinear` | false | true |
| Gaussian noise | `gaussian_noise` | `FaultInjectionTransform` | `derived_channels` | `stddev_v`, `seed`, `evidence_scope = simulation_only` | false | true | false | `nonlinear` | false | true |
| Uniform noise | `uniform_noise` | `FaultInjectionTransform` | `derived_channels` | `min_v`, `max_v`, `seed`, `evidence_scope = simulation_only` | false | true | false | `nonlinear` | false | true |
| Pink noise | `pink_noise` | `FaultInjectionTransform` | `derived_channels` | `amplitude_v`, `seed`, `evidence_scope = simulation_only` | false | true | false | `nonlinear` | false | true |
| Brown noise | `brown_noise` | `FaultInjectionTransform` | `derived_channels` | `amplitude_v`, `seed`, `evidence_scope = simulation_only` | false | true | false | `nonlinear` | false | true |
| Impulse noise | `impulse_noise` | `FaultInjectionTransform` | `derived_channels` | `amplitude_v`, `probability`, `seed`, `evidence_scope = simulation_only` | false | true | false | `nonlinear` | false | true |
| Salt-and-pepper noise | `salt_pepper_noise` | `FaultInjectionTransform` | `derived_channels` | `min_v`, `max_v`, `probability`, `seed`, `evidence_scope = simulation_only` | false | true | false | `nonlinear` | false | true |
| Quantization noise | `quantization_noise` | `FaultInjectionTransform` | `derived_channels` | `lsb_v`, `seed`, `evidence_scope = simulation_only` | false | true | false | `nonlinear` | false | true |
| Periodic interference | `periodic_interference` | `FaultInjectionTransform` | `derived_channels` | `amplitude_v`, `frequency_hz`, `phase_rad`, `evidence_scope = simulation_only` | true | false | false | `nonlinear` | false | true |
| Hum interference | `hum_interference` | `FaultInjectionTransform` | `derived_channels` | `amplitude_v`, `frequency_hz`, `phase_rad`, `evidence_scope = simulation_only` | true | false | false | `nonlinear` | false | true |
| Ground-bounce simulation | `ground_bounce` | `FaultInjectionTransform` | `derived_channels` | `amplitude_v`, `interval_samples`, `evidence_scope = simulation_only` | false | false | false | `nonlinear` | false | true |
| Thermal drift simulation | `thermal_drift` | `FaultInjectionTransform` | `derived_channels` | `drift_rate_v_per_s`, `evidence_scope = simulation_only` | true | false | false | `nonlinear` | false | true |
| Random-walk drift | `random_walk_drift` | `FaultInjectionTransform` | `derived_channels` | `amplitude_v`, `seed`, `evidence_scope = simulation_only` | false | true | false | `nonlinear` | false | true |
| Dropout fault | `dropout_fault` | `FaultInjectionTransform` | `derived_channels` | `fault_value_v`, `probability`, `seed`, `evidence_scope = simulation_only` | false | true | false | `nonlinear` | false | true |
| Missing-sample simulation | `missing_samples` | `FaultInjectionTransform` | `derived_channels` | `fault_value_v`, `probability`, `seed`, `evidence_scope = simulation_only` | false | true | false | `nonlinear` | false | true |
| Saturation fault | `saturation_fault` | `FaultInjectionTransform` | `derived_channels` | `min_v`, `max_v`, `evidence_scope = simulation_only` | false | true | false | `nonlinear` | false | true |
| Stuck-at fault | `stuck_at_fault` | `FaultInjectionTransform` | `derived_channels` | `fault_value_v`, `start_index`, `duration_samples`, `evidence_scope = simulation_only` | false | true | false | `nonlinear` | false | true |
| Flatline fault | `flatline_fault` | `FaultInjectionTransform` | `derived_channels` | `start_index`, `evidence_scope = simulation_only` | false | true | false | `nonlinear` | false | true |
| Intermittent fault | `intermittent_fault` | `FaultInjectionTransform` | `derived_channels` | `fault_value_v`, `probability`, `seed`, `evidence_scope = simulation_only` | false | true | false | `nonlinear` | false | true |
| Rounding quantizer | `rounding_quantizer` | `QuantizationTransform` | `derived_channels` | `lsb_v`, `evidence_scope = simulation_only` | false | false | false | `nonlinear` | false | true |
| Floor quantizer | `floor_quantizer` | `QuantizationTransform` | `derived_channels` | `lsb_v`, `evidence_scope = simulation_only` | false | false | false | `nonlinear` | false | true |
| Ceil quantizer | `ceil_quantizer` | `QuantizationTransform` | `derived_channels` | `lsb_v`, `evidence_scope = simulation_only` | false | false | false | `nonlinear` | false | true |
| Mid-rise quantizer | `midrise_quantizer` | `QuantizationTransform` | `derived_channels` | `lsb_v`, `evidence_scope = simulation_only` | false | false | false | `nonlinear` | false | true |
| Mid-tread quantizer | `midtread_quantizer` | `QuantizationTransform` | `derived_channels` | `lsb_v`, `evidence_scope = simulation_only` | false | false | false | `nonlinear` | false | true |
| Saturating quantizer | `saturating_quantizer` | `QuantizationTransform` | `derived_channels` | `min_v`, `max_v`, `evidence_scope = simulation_only` | false | false | false | `nonlinear` | false | true |
| Dither | `dither` | `QuantizationTransform` | `derived_channels` | `lsb_v`, `seed`, `evidence_scope = simulation_only` | false | true | false | `nonlinear` | false | true |
| Companding | `companding` | `QuantizationTransform` | `derived_channels` | `mode`, `max_v`, `mu`, `evidence_scope = simulation_only` | false | false | false | `nonlinear` | false | true |
| Sample-clock jitter | `sample_clock_jitter` | `FaultInjectionTransform` | `derived_channels` | `jitter_s`, `seed`, `evidence_scope = simulation_only` | true | true | false | `nonlinear` | false | true |
| ADC missing code | `adc_missing_code` | `QuantizationTransform` | `derived_channels` | `bits`, `min_v`, `max_v`, `missing_code`, `evidence_scope = simulation_only` | false | false | false | `nonlinear` | false | true |
| INL error | `inl_error` | `QuantizationTransform` | `derived_channels` | `bits`, `min_v`, `max_v`, `coefficients`, `evidence_scope = simulation_only` | false | false | false | `nonlinear` | false | true |
| DNL error | `dnl_error` | `QuantizationTransform` | `derived_channels` | `bits`, `min_v`, `max_v`, `coefficients`, `evidence_scope = simulation_only` | false | false | false | `nonlinear` | false | true |
| ADC gain error | `adc_gain_error` | `QuantizationTransform` | `derived_channels` | `gain_error`, `evidence_scope = simulation_only` | false | false | false | `nonlinear` | false | true |
| ADC offset error | `adc_offset_error` | `QuantizationTransform` | `derived_channels` | `offset_error_v`, `evidence_scope = simulation_only` | false | false | false | `nonlinear` | false | true |

Behavior notes:

- M34 seeded transforms are deterministic for a fixed seed and channel order.
- M34 uses a dependency-free deterministic RNG/noise path; no third-party RNG or noise crate is added.
- Every M34 transform records `evidence_scope = simulation_only` in transform parameters.
- M34 output is derived waveform data and preserves source waveform lineage.
- Rule-package export rejects all M34 transforms; runtime/package exposure remains separately gated.
- M34 does not claim observed hardware behavior, ADC accuracy, calibration, hardware qualification, certification evidence, live DAQ behavior, or HAL/RTOS support.

## M26 Data Cleaning And Timing Conditioning

| Transform | `name` | `category` | `parameters` | `sample_rate_required` | `stateful` | `causal` | `phase_effect` | `streaming_supported` | `offline_only` |
|---|---|---|---|---|---|---|---|---|---|
| Timestamp sort | `timestamp_sort` | `DataCleaningTransform` | `order = ascending` | true | false | false | `none` | false | true |
| Duplicate timestamp removal | `dedupe_timestamps` | `DataCleaningTransform` | `policy = keep_first` | true | false | false | `none` | false | true |
| NaN interpolation | `nan_interpolate` | `DataCleaningTransform` | none | true | false | false | `none` | false | true |
| NaN row removal | `nan_remove` | `DataCleaningTransform` | `policy = drop_rows_with_nan` | false | false | true | `none` | true | false |
| Crop | `crop` | `DataCleaningTransform` | `start_time_s`, `end_time_s` in `s` | true | false | false | `none` | false | true |
| Fixed delay | `fixed_delay` | `ResamplingTransform` | `delay_s` in `s` | true | false | true | `delay` | true | false |
| Gap fill | `gap_fill` | `ResamplingTransform` | `sample_interval_s` in `s` | true | false | false | `delay` | false | true |
| Fixed-rate resampling | `resample_fixed` | `ResamplingTransform` | `sample_interval_s` in `s` | true | false | false | `delay` | false | true |
| Channel delay alignment | `channel_delay` | `ResamplingTransform` | `channel`, `delay_s` in `s` | true | false | false | `delay` | false | true |

Behavior notes:

- M26 transforms produce derived waveforms and preserve raw waveform samples.
- `timestamp_sort`, `dedupe_timestamps`, `crop`, and `fixed_delay` reject infinite samples but may run before `nan_interpolate` or `nan_remove`.
- `nan_interpolate`, `gap_fill`, `resample_fixed`, and `channel_delay` use linear interpolation; endpoint values are held when a requested interpolation time falls outside the source channel range.
- `split_by_event` is cataloged as planned and deferred because it creates multiple segment artifacts rather than one derived waveform.
- Rule-package export rejects all M26 transforms until separate package/runtime semantics are approved.

## Moving Average

| Field | Value |
|---|---|
| `name` | `moving_average` |
| `category` | `WindowedTransform` |
| `history_label` | `moving_average(window_samples={window_samples})` |
| `parameters` | `window_samples` as integer with unit `samples` |
| `sample_rate_required` | `false` |
| `stateful` | `true` |
| `causal` | `true` |
| `phase_effect` | `delay` |
| `streaming_supported` | `true` |
| `offline_only` | `false` |

Behavior notes:

- Current implementation uses a trailing window that includes the current sample.
- `window_samples` must be greater than zero.
- Edge behavior uses shorter windows at the beginning of the signal.
- It preserves channel names and units.

Example emitted metadata:

```json
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
```

## Low-Pass

| Field | Value |
|---|---|
| `name` | `low_pass` |
| `category` | `FrequencyFilterTransform` |
| `history_label` | `low_pass(cutoff_hz={cutoff_hz})` |
| `parameters` | `cutoff_hz` as number with unit `Hz` |
| `sample_rate_required` | `true` |
| `stateful` | `true` |
| `causal` | `true` |
| `phase_effect` | `delay` |
| `streaming_supported` | `true` |
| `offline_only` | `false` |

Behavior notes:

- Current implementation is a first-order low-pass smoothing filter.
- `cutoff_hz` must be greater than zero.
- Time samples must be strictly increasing.
- The equation uses adjacent timestamp deltas and assumes the configured time axis is compatible with seconds when `cutoff_hz` is interpreted as Hz.
- It preserves channel names and units.

Example emitted metadata:

```json
{
  "sequence_index": 0,
  "history_label": "low_pass(cutoff_hz=10)",
  "name": "low_pass",
  "category": "FrequencyFilterTransform",
  "input_channels": {
    "kind": "all_channels"
  },
  "output_channels": {
    "kind": "derived_channels",
    "preserves_names": true
  },
  "parameters": [
    {
      "name": "cutoff_hz",
      "value": 10.0,
      "unit": "Hz"
    }
  ],
  "sample_rate_required": true,
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
```

## ADC Quantization

| Field | Value |
|---|---|
| `name` | `adc_quantize` |
| `category` | `QuantizationTransform` |
| `history_label` | `adc_quantize(bits={bits},min_v={min_v},max_v={max_v})` |
| `parameters` | `bits` as integer with unit `bits`; `min_v` and `max_v` as numbers with unit `V` |
| `sample_rate_required` | `false` |
| `stateful` | `false` |
| `causal` | `true` |
| `phase_effect` | `none` |
| `streaming_supported` | `true` |
| `offline_only` | `false` |

Behavior notes:

- Current implementation simulates ideal endpoint-inclusive ADC code quantization.
- `bits` must be between 1 and 24.
- `min_v` and `max_v` must be finite, and `max_v` must be greater than `min_v`.
- Samples outside the configured voltage range are clipped before code conversion.
- Output values remain in volts for downstream criteria.
- It preserves channel names and units.

Example emitted metadata:

```json
{
  "sequence_index": 0,
  "history_label": "adc_quantize(bits=8,min_v=0,max_v=5)",
  "name": "adc_quantize",
  "category": "QuantizationTransform",
  "input_channels": {
    "kind": "all_channels"
  },
  "output_channels": {
    "kind": "derived_channels",
    "preserves_names": true
  },
  "parameters": [
    {
      "name": "bits",
      "value": 8,
      "unit": "bits"
    },
    {
      "name": "min_v",
      "value": 0.0,
      "unit": "V"
    },
    {
      "name": "max_v",
      "value": 5.0,
      "unit": "V"
    }
  ],
  "sample_rate_required": false,
  "stateful": false,
  "causal": true,
  "phase_effect": "none",
  "streaming_supported": true,
  "offline_only": false,
  "runtime_profiles": [
    "desktop"
  ],
  "capability_status": "implemented",
  "evidence_level": "golden_report_tested"
}
```

## Chained Transform Mapping

For a chain such as:

```text
moving_average(window_samples=2) -> adc_quantize(bits=8,min_v=0,max_v=5)
```

`transform_steps` should have:

- two records
- `sequence_index` values `0` and `1`
- `history_label` values that exactly match `transform_history[0]` and `transform_history[1]`
- channel/output behavior repeated for each step unless a later transform changes output kind

## M35 Multi-Channel, Sensor, And Domain Conditioning Mapping

M35 transforms append explicitly named derived channels instead of overwriting the source channel set. Each emitted step uses `output_channels.kind = derived_channels`, `runtime_profiles = ["desktop"]`, `capability_status = implemented`, `evidence_level = golden_report_tested`, and `package_support = rejected_desktop_only`. The software formulas are conditioning evidence only; they are not hardware calibration, sensor accuracy, qualification, certification, live DAQ, HAL/RTOS, or rule-package/runtime evidence.

| Transform names | Category | Sample rate required | Stateful | Causal | Phase effect | Streaming supported | Offline only | Mapping notes |
|---|---|---:|---:|---:|---|---:|---:|---|
| `channel_add`, `channel_subtract`, `differential_channel`, `common_mode` | `MultiChannelTransform` | `false` | `false` | `true` | `none` | `true` | `false` | Requires aligned same-unit `left_channel` and `right_channel`; appends configured `output_channel`. |
| `vector_magnitude`, `euclidean_norm` | `MultiChannelTransform` | `false` | `false` | `true` | `nonlinear` | `true` | `false` | Requires two or more same-unit channels; appends configured magnitude/norm channel. |
| `matrix_transform`, `coordinate_rotation` | `MultiChannelTransform` | `false` | `false` | `true` | `none` | `true` | `false` | Requires matrix/output shape or two-axis rotation outputs to match declared channel names. |
| `linear_sensor_conversion`, `pressure_transducer`, `current_shunt`, `bridge_strain`, `load_cell_force`, `tachometer_rpm`, `encoder_position`, `accelerometer_units`, `gyroscope_rate`, `hall_current`, `lvdt_position`, `photodiode_power` | `CalibrationTransform` | `false` | `false` | `true` | `none` | `true` | `false` | Applies configured software conversion formula and declares `output_unit`; does not prove sensor calibration. |
| `rtd_temperature`, `thermistor_temperature`, `microphone_spl` | `CalibrationTransform` | `false` | `false` | `true` | `nonlinear` | `true` | `false` | Applies nonlinear conversion formula with finite-input checks and declared output unit. |
| `velocity_from_acceleration`, `displacement_from_velocity` | `CalibrationTransform` | `true` | `true` | `true` | `none` | `true` | `false` | Uses waveform time deltas for trapezoid-style cumulative integration into configured output units. |
| `vibration_severity` | `CalibrationTransform` | `false` | `true` | `true` | `delay` | `true` | `false` | Emits trailing RMS severity over configured `window_samples`. |
| `control_error`, `proportional_control`, `feedforward_control` | `ControlTransform` | `false` | `false` | `true` | `none` | `true` | `false` | Emits software control-signal calculations from configured setpoint/gain/offset values. |
| `pid_control`, `rate_limiter`, `slew_rate_limit` | `ControlTransform` | `true` | `true` | `true` | `delay` | `true` | `false` | Uses time deltas for controller/increment state and validates finite rates/gains. |
| `control_saturation`, `control_deadzone` | `ControlTransform` | `false` | `false` | `true` | `nonlinear` | `true` | `false` | Applies bounded nonlinear control conditioning. |

M35 also records catalog-only dependency/design-gated entries for `phase_difference`, `gain_phase_match`, `advanced_acoustic_pack`, and `advanced_sensor_calibration_pack`. They remain `runtime_profiles = ["future_gated"]`, `evidence_level = documented_only`, and `package_support = dependency_gated` until dependency review, estimator/calibration conventions, known-answer fixtures, and performance evidence are added.

## Hand-Off Note

Role: Systems Engineer / Software Architect
Goal: Complete M10-003 / issue #134 by mapping current implemented transforms to structured metadata values, updated through local M36 comprehensive-suite closure.
Files changed: `docs/current-transform-metadata-mapping.md`
Checks run: Documentation and compatibility review.
Status: Complete through PR #138 and updated locally through M36.
Known gaps: Bounded embedded event exposure and future runtime/package transform exposure remain gated work. M13 adds runtime-profile validator code for this mapping through PR #164; M14 and M26-M35 transforms remain desktop-only unless a later runtime/package gate approves more. Hilbert envelope, optimized FFT dependency/performance work, phase-difference estimation, gain/phase matching, advanced acoustic/sensor packs, and large spectral-output UX remain gated.
Next recommended step: Use this mapping with the M13 validator before future transform package or runtime exposure.
