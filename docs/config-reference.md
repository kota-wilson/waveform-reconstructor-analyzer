# FerrisOxide Config Reference

Date: 2026-06-02

Status: M15 reference updated through local M36 comprehensive-suite closure. This document describes local file-based CLI and TOML behavior only. It does not approve live DAQ, GUI, hosted service, HAL/RTOS, target hardware, binary package, signing, hardware qualification, calibration certification, or certification behavior.

## CLI Commands

| Command | Purpose | Required Inputs | Output |
|---|---|---|---|
| `analyze` | Analyze one CSV waveform against criteria. | `--input`; either `--config` or `--time-column`, `--channels`, and at least one criterion flag. | Text or JSON report printed to stdout. |
| `plot` | Render an SVG waveform plot. | `--input`, `--output`; either `--config` or `--time-column` and `--channels`. | SVG file. |
| `export-rule-package` | Export reviewable rule-package artifacts from one CSV/config pair. | `--input`, `--config`, `--output-dir`, `--package-name`, `--package-version`. | `rules.toml`, `rules.json`, `validation-report.json`, `manifest.json`, `checksum.txt`. |
| `simulate` | Run fixture-driven desktop controller simulation. | `--input`, `--control-config`, `--verification-config`, `--channel-map`. | JSON by default, or written through `--output-json`. |
| `batch` | Run repeated local analyses from a batch manifest. | `--manifest`; either `--output-dir` or manifest `output_dir`. | Per-run reports plus `batch-summary.json`. |

Common output formats are `text` and `json` where supported. The CLI intentionally uses explicit flags instead of implicit discovery so command evidence can be copied into validation logs.

## Analysis Config Shape

Analysis configs are TOML files parsed into `AnalysisConfig`.

```toml
[input]
time_column = "time"
channels = ["input_v"]
time_unit = "s"
signal_unit = "V"

[tolerances]
voltage_v = 0.0
time_s = 0.0

[[filters]]
type = "moving_average"
window_samples = 3

[[criteria]]
id = "input_max_voltage"
type = "maximum_voltage"
channel = "input_v"
threshold = 5.5
```

Required sections:

| Section | Required | Notes |
|---|---|---|
| `[input]` | Yes | Declares CSV time column and one or more signal channels. |
| `[metadata]` | No | Adds optional test-run, acquisition, environment, and operator context to reports. |
| `[tolerances]` | No | Defaults to zero voltage and time tolerance. |
| `[[filters]]` | No | Ordered derived transforms applied before criteria. |
| `[[feature_transforms]]` | No | Scalar feature-record calculations evaluated after filters and before criteria report rendering. |
| `[[event_transforms]]` | No | Event extraction transforms. |
| `[[event_validations]]` | No | Event-level validation checks. |
| `[[criteria]]` | Yes for `analyze`, `plot --config`, `export-rule-package`, and `batch` analysis runs | Supports legacy and measurement-backed DSL shapes. |

## Input Fields

| Field | Required | Default | Meaning |
|---|---|---|---|
| `time_column` | Yes | none | CSV column containing timestamps. |
| `channels` | Yes | none | CSV signal columns analyzed by filters, criteria, and reports. |
| `time_unit` | No | `s` | Time-axis unit. Duration criteria and timing-dependent transforms expect seconds. |
| `signal_unit` | No | `V` | Unit assigned to configured signal channels. |

`channels` must contain at least one non-empty channel. Duplicate and decreasing timestamps are rejected by duration-sensitive analysis and timing-dependent transforms.

## Metadata Fields

| Field | Required | Meaning |
|---|---|---|
| `test_run_id` | No | Local run or fixture identifier. |
| `acquisition_notes` | No | Human notes about the fixture or data source. |
| `environment` | No | Local environment or test context. |
| `operator` | No | Human or automation descriptor. |

Metadata is report context only. It is not hardware qualification, calibration evidence, or certification metadata.

## Tolerance Fields

| Field | Required | Default | Meaning |
|---|---|---|---|
| `voltage_v` | No | `0.0` | Voltage tolerance applied to voltage comparisons. |
| `time_s` | No | `0.0` | Time tolerance applied to duration/timing comparisons. |

Tolerance values must be finite and non-negative.

## Filter And Transform Fields

Filters are ordered. Each entry requires `type` and the fields listed below.

| `type` | Required Fields | Package Export | Runtime Profile |
|---|---|---|---|
| `offset` | `offset_v` | Supported for rule-package export as a unit-bearing software transform as of M21. | Desktop metadata plus shared borrowed-slice runtime semantics for the linear subset; not calibration evidence. |
| `gain` | `gain` | Supported for rule-package export as a finite software ratio as of M21. | Desktop metadata plus shared borrowed-slice runtime semantics for the linear subset; not span-calibration evidence. |
| `invert` | none | Supported for rule-package export as a parameterless polarity transform as of M21. | Desktop metadata plus shared borrowed-slice runtime semantics for the linear subset. |
| `clamp` | `min_v`, `max_v` | Rejected. | Desktop metadata only. |
| `deadband` | `threshold_v` | Rejected. | Desktop metadata only. |
| `dc_remove` | none | Rejected. | Desktop metadata only. |
| `baseline_subtract` | `baseline_v` | Rejected. | Desktop metadata only. |
| `high_pass_baseline` | `cutoff_hz` | Rejected. | Desktop metadata only; timing-dependent and stateful. |
| `moving_average` | `window_samples` | Supported by legacy rule-package filter subset. | Desktop metadata; broader runtime loader support remains guarded. |
| `moving_median` | `window_samples` | Rejected. | Desktop metadata only. |
| `low_pass` | `cutoff_hz` | Supported by legacy rule-package filter subset. | Desktop metadata; broader runtime loader support remains guarded. |
| `adc_quantize` | `bits`, `min_v`, `max_v` | Supported by legacy rule-package filter subset. | Desktop metadata; package evidence is not hardware ADC validation. |
| `absolute_value` | none | Rejected. | Desktop metadata only; magnitude-only software transform. |
| `square` | none | Rejected. | Desktop metadata only; finite-output pointwise squaring. |
| `square_root` | none | Rejected. | Desktop metadata only; rejects negative samples. |
| `log` | `base` | Rejected. | Desktop metadata only; rejects non-positive samples and invalid bases. |
| `exp` | `base` | Rejected. | Desktop metadata only; rejects invalid bases and non-finite outputs. |
| `normalize` | `mode`; range mode also requires `input_min_v`, `input_max_v`, `output_min`, `output_max` | Rejected. | Desktop metadata only; offline channel-wise normalization. |
| `tanh` | none | Rejected. | Desktop metadata only; pointwise hyperbolic tangent mapping. |
| `sigmoid` | none | Rejected. | Desktop metadata only; pointwise logistic sigmoid mapping. |
| `soft_limit` | `limit_v` | Rejected. | Desktop metadata only; smooth tanh-based limiting. |
| `piecewise_linear` | `points` | Rejected. | Desktop metadata only; `points` must contain at least two `{ x, y }` values with strictly increasing `x`. |
| `polynomial` | `coefficients` | Rejected. | Desktop metadata only; coefficients are ordered as `c0 + c1*x + c2*x^2...`. |
| `weighted_moving_average` | `weights` | Rejected. | Desktop metadata only; trailing causal weighted smoothing with newest sample aligned to the final weight. |
| `exponential_moving_average` | `alpha` | Rejected. | Desktop metadata only; causal single-pole smoothing seeded from the first sample. |
| `boxcar_smoothing` | `window_samples` | Rejected. | Desktop metadata only; centered offline uniform smoothing with shrinking edge windows. |
| `gaussian_smoothing` | `window_samples`, `sigma_samples` | Rejected. | Desktop metadata only; centered offline Gaussian smoothing with edge renormalization. |
| `savitzky_golay` | `window_samples`, `polynomial_order` | Rejected. | Desktop metadata only; odd-window local polynomial smoothing, order capped at 5. |
| `centered_moving_median` | `window_samples` | Rejected. | Desktop metadata only; centered offline median smoothing. |
| `rolling_mean_baseline` | `window_samples` | Rejected. | Desktop metadata only; causal trailing mean baseline subtraction. |
| `rolling_median_baseline` | `window_samples` | Rejected. | Desktop metadata only; causal trailing median baseline subtraction. |
| `linear_detrend` | none | Rejected. | Desktop metadata only; offline least-squares linear drift removal over a strictly increasing time axis. |
| `polynomial_detrend` | `polynomial_order` | Rejected. | Desktop metadata only; offline polynomial drift removal, order capped at 5. |
| `hampel_filter` | `window_samples`, `outlier_sigma` | Rejected. | Desktop metadata only; offline median/MAD outlier replacement. |
| `spike_remove` | `window_samples`, `threshold_v` | Rejected. | Desktop metadata only; offline median-window spike replacement using an absolute voltage threshold. |
| `fir_filter` | `coefficients` | Rejected. | Desktop metadata only; causal FIR convolution over configured coefficients. |
| `zero_phase_fir_filter` | `coefficients` | Rejected. | Desktop metadata only; offline forward/backward FIR filtering with `phase_effect = none`. |
| `iir_biquad` | `coefficients` as `[b0, b1, b2, a1, a2]` | Rejected. | Desktop metadata only; causal biquad with pole-stability validation. |
| `zero_phase_iir_biquad` | `coefficients` as `[b0, b1, b2, a1, a2]` | Rejected. | Desktop metadata only; offline forward/backward biquad with pole-stability validation. |
| `high_pass` | `cutoff_hz` | Rejected. | Desktop metadata only; uniform sample interval and below-Nyquist cutoff required. |
| `band_pass` | `center_hz`, `q` | Rejected. | Desktop metadata only; second-order RBJ biquad with uniform sample interval and below-Nyquist center frequency. |
| `band_stop` | `center_hz`, `q` | Rejected. | Desktop metadata only; second-order RBJ biquad with uniform sample interval and below-Nyquist center frequency. |
| `notch` | `center_hz`, `q` | Rejected. | Desktop metadata only; second-order notch biquad with uniform sample interval and below-Nyquist center frequency. |
| `comb_filter` | `delay_samples`, `feedback_gain` | Rejected. | Desktop metadata only; feed-forward comb with positive delay and gain magnitude no greater than one. |
| `butterworth_low_pass` | `cutoff_hz` | Rejected. | Desktop metadata only; second-order Butterworth biquad with below-Nyquist cutoff. |
| `butterworth_high_pass` | `cutoff_hz` | Rejected. | Desktop metadata only; second-order Butterworth biquad with below-Nyquist cutoff. |
| `chebyshev1_low_pass` | `cutoff_hz`, `ripple_db` | Rejected. | Desktop metadata only; dependency-free second-order Type I prototype. |
| `chebyshev2_low_pass` | `cutoff_hz`, `stopband_attenuation_db` | Rejected. | Desktop metadata only; dependency-free second-order Type II prototype. |
| `bessel_low_pass` | `cutoff_hz` | Rejected. | Desktop metadata only; dependency-free second-order Bessel prototype. |
| `timestamp_sort` | none | Rejected. | Desktop metadata only; offline timestamp repair. |
| `dedupe_timestamps` | none | Rejected. | Desktop metadata only; keeps the first sample for each duplicate timestamp. |
| `nan_interpolate` | none | Rejected. | Desktop metadata only; offline time-based linear interpolation over NaN samples. |
| `nan_remove` | none | Rejected. | Desktop metadata only; drops rows containing NaN values. |
| `crop` | `start_time_s`, `end_time_s` | Rejected. | Desktop metadata only; inclusive time-window selection. |
| `fixed_delay` | `delay_s` | Rejected. | Desktop metadata only; shifts the waveform time axis. |
| `gap_fill` | `sample_interval_s` | Rejected. | Desktop metadata only; fixed-grid linear interpolation for gaps. |
| `resample_fixed` | `sample_interval_s` | Rejected. | Desktop metadata only; fixed-rate linear interpolation. |
| `channel_delay` | `channel`, `delay_s` | Rejected. | Desktop metadata only; offline single-channel delay alignment with endpoint hold. |
| `resample` | `sample_interval_s` | Rejected. | Desktop metadata only; offline fixed-grid linear resampling. |
| `downsample` | `factor` | Rejected. | Desktop metadata only; integer-factor sample dropping. |
| `decimate` | `factor`, `cutoff_hz` | Rejected. | Desktop metadata only; first-order anti-alias prefiltering followed by integer downsampling. |
| `upsample` | `factor` | Rejected. | Desktop metadata only; integer-factor linear interpolation. |
| `interpolate` | `sample_interval_s` | Rejected. | Desktop metadata only; offline first-order interpolation onto a fixed grid. |
| `rational_resample` | `upsample_factor`, `downsample_factor` | Rejected. | Desktop metadata only; dependency-free rational grid conversion by linear interpolation, not an efficient polyphase implementation. |
| `sample_and_hold` | `sample_interval_s` | Rejected. | Desktop metadata only; previous-sample hold onto a configured grid. |
| `zero_order_hold` | `sample_interval_s` | Rejected. | Desktop metadata only; previous-sample zero-order hold reconstruction. |
| `first_order_hold` | `sample_interval_s` | Rejected. | Desktop metadata only; linear first-order hold reconstruction. |
| `fractional_delay` | `delay_s` | Rejected. | Desktop metadata only; sub-sample delay or advance by interpolation with endpoint hold. |
| `cross_correlation_delay` | `reference_channel`, `target_channel`, `max_lag_samples` | Rejected. | Desktop metadata only; estimates integer lag, records delay/confidence, and aligns the target channel. |
| `jitter_correction` | `sample_interval_s` | Rejected. | Desktop metadata only; uneven-to-fixed-grid repair by linear interpolation. |
| `clock_drift_correction` | `sample_interval_s` | Rejected. | Desktop metadata only; nominal-grid correction that records end-of-run drift. |
| `half_wave_rectify` | none | Rejected. | Desktop metadata only; clamps negative samples to zero. |
| `full_wave_rectify` | none | Rejected. | Desktop metadata only; maps samples to absolute magnitude. |
| `envelope` | `alpha` | Rejected. | Desktop metadata only; causal absolute-value envelope smoother where `alpha` is between zero and one. |
| `moving_rms` | `window_samples` | Rejected. | Desktop metadata only; trailing causal RMS waveform with shrinking startup window. |
| `peak_hold` | none | Rejected. | Desktop metadata only; running absolute peak magnitude. |
| `first_derivative` | none | Rejected. | Desktop metadata only; finite-difference rate-of-change waveform over a strictly increasing time axis. |
| `second_derivative` | none | Rejected. | Desktop metadata only; second finite-difference rate/curvature waveform over a strictly increasing time axis. |
| `integral` | none | Rejected. | Desktop metadata only; cumulative trapezoidal integral waveform over a strictly increasing time axis. |
| `cumulative_integral` | none | Rejected. | Desktop metadata only; explicit cumulative trapezoidal integral waveform. |
| `leaky_integrator` | `time_constant_s` | Rejected. | Desktop metadata only; causal decaying accumulator over a strictly increasing time axis. |
| `slope_detection` | `threshold_per_s` | Rejected. | Desktop metadata only; maps rising/falling slopes to `1`, `-1`, or `0`. |
| `rolling_mean` | `window_samples` | Rejected. | Desktop metadata only; trailing-window local mean with shrinking startup windows. |
| `rolling_variance` | `window_samples` | Rejected. | Desktop metadata only; trailing-window population variance. |
| `rolling_stddev` | `window_samples` | Rejected. | Desktop metadata only; trailing-window population standard deviation. |
| `rolling_min` | `window_samples` | Rejected. | Desktop metadata only; trailing-window local minimum. |
| `rolling_max` | `window_samples` | Rejected. | Desktop metadata only; trailing-window local maximum. |
| `z_score` | none | Rejected. | Desktop metadata only; whole-channel population z-score waveform; rejects constant samples. |
| `outlier_detection` | `threshold_sigma` | Rejected. | Desktop metadata only; outputs `1` when absolute z-score exceeds the threshold, otherwise `0`. |
| `quantile_clip` | `lower_quantile`, `upper_quantile` | Rejected. | Desktop metadata only; clips samples to interpolated quantile bounds. |
| `white_noise` | `amplitude_v`, `seed` | Rejected. | Desktop metadata only; seeded deterministic simulation, not observed noise evidence. |
| `gaussian_noise` | `stddev_v`, `seed` | Rejected. | Desktop metadata only; seeded deterministic simulation using normal-distributed samples. |
| `uniform_noise` | `min_v`, `max_v`, `seed` | Rejected. | Desktop metadata only; seeded deterministic uniform-noise simulation. |
| `pink_noise` | `amplitude_v`, `seed` | Rejected. | Desktop metadata only; seeded first-order colored-noise approximation. |
| `brown_noise` | `amplitude_v`, `seed` | Rejected. | Desktop metadata only; seeded bounded random-walk noise approximation. |
| `impulse_noise` | `amplitude_v`, `probability`, `seed` | Rejected. | Desktop metadata only; seeded impulse-spike simulation. |
| `salt_pepper_noise` | `min_v`, `max_v`, `probability`, `seed` | Rejected. | Desktop metadata only; seeded random extreme-sample simulation. |
| `quantization_noise` | `lsb_v`, `seed` | Rejected. | Desktop metadata only; seeded quantization-noise simulation. |
| `periodic_interference` | `amplitude_v`, `frequency_hz`; optional `phase_rad` | Rejected. | Desktop metadata only; time-axis sine interference simulation. |
| `hum_interference` | `amplitude_v`, `frequency_hz`; optional `phase_rad` | Rejected. | Desktop metadata only; time-axis mains-hum-style interference simulation. |
| `ground_bounce` | `amplitude_v`, `interval_samples` | Rejected. | Desktop metadata only; alternating reference-shift simulation. |
| `thermal_drift` | `drift_rate_v_per_s` | Rejected. | Desktop metadata only; linear time-axis drift simulation. |
| `random_walk_drift` | `amplitude_v`, `seed` | Rejected. | Desktop metadata only; seeded baseline-wander simulation. |
| `dropout_fault` | `fault_value_v`, `probability`, `seed` | Rejected. | Desktop metadata only; seeded replacement-value fault simulation. |
| `missing_samples` | `fault_value_v`, `probability`, `seed` | Rejected. | Desktop metadata only; seeded missing-sample value-substitution simulation. |
| `saturation_fault` | `min_v`, `max_v` | Rejected. | Desktop metadata only; sensor/ADC saturation simulation. |
| `stuck_at_fault` | `fault_value_v`, `start_index`, `duration_samples` | Rejected. | Desktop metadata only; fixed-value stuck-at simulation over a sample window. |
| `flatline_fault` | `start_index`; optional `fault_value_v` | Rejected. | Desktop metadata only; flatline simulation from a sample index onward. |
| `intermittent_fault` | `fault_value_v`, `probability`, `seed` | Rejected. | Desktop metadata only; seeded intermittent replacement-value simulation. |
| `rounding_quantizer` | `lsb_v` | Rejected. | Desktop metadata only; round-to-nearest voltage-step quantizer simulation. |
| `floor_quantizer` | `lsb_v` | Rejected. | Desktop metadata only; floor-directed voltage-step quantizer simulation. |
| `ceil_quantizer` | `lsb_v` | Rejected. | Desktop metadata only; ceiling-directed voltage-step quantizer simulation. |
| `midrise_quantizer` | `lsb_v` | Rejected. | Desktop metadata only; mid-rise voltage-step quantizer simulation. |
| `midtread_quantizer` | `lsb_v` | Rejected. | Desktop metadata only; mid-tread voltage-step quantizer simulation. |
| `saturating_quantizer` | `min_v`, `max_v` | Rejected. | Desktop metadata only; endpoint-saturating quantizer simulation. |
| `dither` | `lsb_v`, `seed` | Rejected. | Desktop metadata only; seeded dither-before-quantize simulation. |
| `companding` | `mode`, `max_v`, `mu` | Rejected. | Desktop metadata only; `mode` is `mu_law` or `a_law`. |
| `sample_clock_jitter` | `jitter_s`, `seed` | Rejected. | Desktop metadata only; seeded time-axis jitter simulation with monotonic-time guardrails. |
| `adc_missing_code` | `bits`, `min_v`, `max_v`, `missing_code` | Rejected. | Desktop metadata only; ADC transfer simulation, not hardware ADC evidence. |
| `inl_error` | `bits`, `min_v`, `max_v`, `coefficients` | Rejected. | Desktop metadata only; ADC integral-nonlinearity transfer simulation. |
| `dnl_error` | `bits`, `min_v`, `max_v`, `coefficients` | Rejected. | Desktop metadata only; ADC differential-nonlinearity transfer simulation. |
| `adc_gain_error` | `gain_error` | Rejected. | Desktop metadata only; ADC transfer gain-error simulation. |
| `adc_offset_error` | `offset_error_v` | Rejected. | Desktop metadata only; ADC transfer offset-error simulation. |
| `channel_add` | `left_channel`, `right_channel`, `output_channel`; optional `output_unit` | Rejected. | Desktop metadata only; appends same-unit channel sum. |
| `channel_subtract` | `left_channel`, `right_channel`, `output_channel`; optional `output_unit` | Rejected. | Desktop metadata only; appends same-unit channel difference. |
| `differential_channel` | `left_channel`, `right_channel`, `output_channel`; optional `output_unit` | Rejected. | Desktop metadata only; appends differential measurement channel. |
| `common_mode` | `left_channel`, `right_channel`, `output_channel`; optional `output_unit` | Rejected. | Desktop metadata only; appends paired-channel average. |
| `vector_magnitude`, `euclidean_norm` | `channels`, `output_channel`; optional `output_unit` | Rejected. | Desktop metadata only; appends same-unit Euclidean magnitude. |
| `matrix_transform` | `channels`, `matrix`, `output_channels`; optional `output_unit` | Rejected. | Desktop metadata only; appends finite linear channel mixes. |
| `coordinate_rotation` | `x_channel`, `y_channel`, `angle_rad`, `output_x_channel`, `output_y_channel`; optional `output_unit` | Rejected. | Desktop metadata only; appends rotated two-axis coordinates. |
| `linear_sensor_conversion`, `pressure_transducer` | `channel`, `output_channel`, `output_unit`, `input_min_v`, `input_max_v`, `output_min`, `output_max` | Rejected. | Desktop metadata only; software span conversion, not calibration evidence. |
| `current_shunt` | `channel`, `output_channel`, `output_unit`, `shunt_ohms` | Rejected. | Desktop metadata only; Ohm's-law current conversion. |
| `bridge_strain` | `channel`, `output_channel`, `output_unit`, `excitation_v`, `gauge_factor` | Rejected. | Desktop metadata only; bridge strain formula with configured assumptions. |
| `load_cell_force` | `channel`, `output_channel`, `output_unit`, `excitation_v`, `sensitivity_mv_v`, `full_scale` | Rejected. | Desktop metadata only; load-cell force formula with configured assumptions. |
| `rtd_temperature` | `channel`, `output_channel`, `output_unit`, `r0_ohm`, `alpha_per_c` | Rejected. | Desktop metadata only; first-order RTD resistance-to-temperature formula. |
| `thermistor_temperature` | `channel`, `output_channel`, `output_unit`, `r0_ohm`, `beta_k`, `t0_c` | Rejected. | Desktop metadata only; thermistor Beta-equation conversion. |
| `tachometer_rpm` | `channel`, `output_channel`, `output_unit`, `pulses_per_rev` | Rejected. | Desktop metadata only; pulse-frequency to RPM conversion. |
| `encoder_position` | `channel`, `output_channel`, `output_unit`, `counts_per_rev`, `scale_per_rev` | Rejected. | Desktop metadata only; encoder counts to configured position units. |
| `accelerometer_units`, `gyroscope_rate`, `hall_current`, `lvdt_position` | `channel`, `output_channel`, `output_unit`, `sensitivity_v_per_unit`; optional `bias_v` | Rejected. | Desktop metadata only; biased voltage to configured engineering units. |
| `microphone_spl` | `channel`, `output_channel`, `output_unit`, `reference` | Rejected. | Desktop metadata only; pressure-like samples to dB SPL. |
| `photodiode_power` | `channel`, `output_channel`, `output_unit`, `responsivity_a_per_w` | Rejected. | Desktop metadata only; photodiode current to optical power. |
| `velocity_from_acceleration`, `displacement_from_velocity` | `channel`, `output_channel`, `output_unit` | Rejected. | Desktop metadata only; time-axis trapezoid integration for vibration-style workflows. |
| `vibration_severity` | `channel`, `output_channel`, `output_unit`, `window_samples` | Rejected. | Desktop metadata only; trailing RMS severity channel. |
| `control_error` | `channel`, `output_channel`, `setpoint`; optional `output_unit` | Rejected. | Desktop metadata only; setpoint-minus-measured signal. |
| `proportional_control` | `channel`, `output_channel`, `setpoint`, `kp`; optional `output_unit` | Rejected. | Desktop metadata only; proportional controller output. |
| `pid_control` | `channel`, `output_channel`, `setpoint`, `kp`, `ki`, `kd`; optional `output_unit` | Rejected. | Desktop metadata only; dependency-free PID output over the time axis. |
| `rate_limiter`, `slew_rate_limit` | `channel`, `output_channel`, `rate_limit_per_s`; optional `output_unit` | Rejected. | Desktop metadata only; maximum change per second. |
| `control_saturation` | `channel`, `output_channel`, `min_v`, `max_v`; optional `output_unit` | Rejected. | Desktop metadata only; bounded control/signal output. |
| `control_deadzone` | `channel`, `output_channel`, `threshold_v`; optional `output_unit` | Rejected. | Desktop metadata only; zeroes small control/signal values. |
| `feedforward_control` | `channel`, `output_channel`, `feedforward_gain`, `feedforward_offset`; optional `output_unit` | Rejected. | Desktop metadata only; affine feedforward output. |

All transforms preserve source waveform data by producing derived waveform artifacts. Transform metadata is included in JSON reports for derived waveforms.

Example M26 cleaning chain:

```toml
[[filters]]
type = "timestamp_sort"

[[filters]]
type = "dedupe_timestamps"

[[filters]]
type = "nan_interpolate"

[[filters]]
type = "gap_fill"
sample_interval_s = 0.1
```

Example M27 pointwise and nonlinear chain:

```toml
[[filters]]
type = "absolute_value"

[[filters]]
type = "square_root"

[[filters]]
type = "log"
base = 10.0

[[filters]]
type = "normalize"
mode = "zero_to_one"

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
coefficients = [0.0, 1.0]
```

Example M28 smoothing, detrending, baseline, and spike-cleanup chain:

```toml
[[filters]]
type = "weighted_moving_average"
weights = [1.0, 2.0, 3.0]

[[filters]]
type = "exponential_moving_average"
alpha = 0.35

[[filters]]
type = "boxcar_smoothing"
window_samples = 3

[[filters]]
type = "gaussian_smoothing"
window_samples = 5
sigma_samples = 1.0

[[filters]]
type = "savitzky_golay"
window_samples = 5
polynomial_order = 2

[[filters]]
type = "centered_moving_median"
window_samples = 3

[[filters]]
type = "rolling_mean_baseline"
window_samples = 3

[[filters]]
type = "rolling_median_baseline"
window_samples = 3

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
```

Example M30 resampling and timing-alignment chain:

```toml
[[filters]]
type = "resample"
sample_interval_s = 0.001

[[filters]]
type = "decimate"
factor = 2
cutoff_hz = 100.0

[[filters]]
type = "rational_resample"
upsample_factor = 3
downsample_factor = 2

[[filters]]
type = "fractional_delay"
delay_s = 0.0005

[[filters]]
type = "cross_correlation_delay"
reference_channel = "reference_v"
target_channel = "target_v"
max_lag_samples = 10

[[filters]]
type = "clock_drift_correction"
sample_interval_s = 0.001
```

Example M31 envelope, energy, and calculus chain:

```toml
[[filters]]
type = "half_wave_rectify"

[[filters]]
type = "envelope"
alpha = 0.5

[[filters]]
type = "moving_rms"
window_samples = 3

[[filters]]
type = "first_derivative"

[[filters]]
type = "integral"

[[filters]]
type = "leaky_integrator"
time_constant_s = 2.0

[[filters]]
type = "slope_detection"
threshold_per_s = 0.02
```

Example M32 statistics filter chain:

```toml
[[filters]]
type = "z_score"

[[filters]]
type = "rolling_mean"
window_samples = 3

[[filters]]
type = "rolling_stddev"
window_samples = 3

[[filters]]
type = "quantile_clip"
lower_quantile = 0.05
upper_quantile = 0.95

[[filters]]
type = "outlier_detection"
threshold_sigma = 2.5
```

Example M34 deterministic fault-injection and ADC/DAC simulation chain:

```toml
[[filters]]
type = "white_noise"
amplitude_v = 0.01
seed = 1

[[filters]]
type = "hum_interference"
amplitude_v = 0.03
frequency_hz = 60.0

[[filters]]
type = "random_walk_drift"
amplitude_v = 0.01
seed = 9

[[filters]]
type = "dropout_fault"
fault_value_v = 0.0
probability = 0.1
seed = 10

[[filters]]
type = "saturation_fault"
min_v = 0.0
max_v = 5.0

[[filters]]
type = "rounding_quantizer"
lsb_v = 0.1

[[filters]]
type = "dither"
lsb_v = 0.1
seed = 13

[[filters]]
type = "sample_clock_jitter"
jitter_s = 0.00001
seed = 14

[[filters]]
type = "adc_missing_code"
bits = 4
min_v = 0.0
max_v = 5.0
missing_code = 3

[[filters]]
type = "adc_gain_error"
gain_error = 0.01
```

M34 simulation conventions:

- Seeded transforms are deterministic for a given channel order and seed.
- Each M34 transform records `evidence_scope = simulation_only` in transform parameter metadata.
- Fault injection, drift, noise, ADC/DAC transfer errors, jitter, and quantizer variants are desktop/offline simulation evidence only.
- M34 does not claim hardware ADC accuracy, calibration evidence, hardware qualification, certification evidence, runtime package support, or live DAQ behavior.

Example M35 multi-channel, sensor, vibration, and control chain:

```toml
[[filters]]
type = "differential_channel"
left_channel = "sense_hi_v"
right_channel = "sense_lo_v"
output_channel = "sense_diff_v"

[[filters]]
type = "vector_magnitude"
channels = ["accel_x_v", "accel_y_v", "accel_z_v"]
output_channel = "accel_mag_v"

[[filters]]
type = "pressure_transducer"
channel = "pressure_v"
output_channel = "pressure_kpa"
output_unit = "kPa"
input_min_v = 0.5
input_max_v = 4.5
output_min = 0.0
output_max = 1000.0

[[filters]]
type = "velocity_from_acceleration"
channel = "accel_m_s2"
output_channel = "velocity_m_s"
output_unit = "m/s"

[[filters]]
type = "pid_control"
channel = "measured_v"
output_channel = "pid_out"
setpoint = 5.0
kp = 2.0
ki = 0.5
kd = 0.1
```

M35 domain-conditioning conventions:

- M35 appends named derived channels so raw/source channels remain available for later steps and report lineage.
- Multi-channel transforms validate channel existence and same-unit alignment where same-unit math is required.
- Sensor formulas record `calibration_scope = software_formula_only`; they are engineering-unit conversions, not calibration certificates or hardware accuracy evidence.
- Advanced acoustic weighting, order tracking, thermocouple polynomial/cold-junction workflows, and hardware-calibrated sensor packs remain dependency/design gated.

## Feature Transforms

Feature transforms are scalar calculations evaluated after ordered filters. They emit `feature_records` in text/JSON reports and do not directly change `overall_outcome`.

| `type` | Required Fields | Unit Behavior | Runtime/Profile Notes |
|---|---|---|---|
| `rms` | `id`, `channel` | source channel unit | Streaming-supported scalar record. |
| `peak_to_peak` | `id`, `channel` | source channel unit | Streaming-supported scalar record. |
| `crest_factor` | `id`, `channel` | `ratio`; rejects zero RMS | Streaming-supported scalar record. |
| `energy` | `id`, `channel` | `<unit>^2*s` | Offline feature record; requires strictly increasing time. |
| `power` | `id`, `channel` | `<unit>^2` | Offline feature record; requires strictly increasing time and non-zero duration. |
| `area_under_curve` | `id`, `channel` | `<unit>*s` | Offline feature record; requires strictly increasing time. |
| `impulse_estimate` | `id`, `channel` | `<unit>*s` | Offline feature record; requires strictly increasing time. |
| `mean` | `id`, `channel` | source channel unit | Offline feature record. |
| `median` | `id`, `channel` | source channel unit | Offline feature record using linear-interpolated median. |
| `mode` | `id`, `channel` | source channel unit | Offline exact-value mode; ties choose the lowest sorted value. |
| `min` | `id`, `channel` | source channel unit | Offline feature record. |
| `max` | `id`, `channel` | source channel unit | Offline feature record. |
| `variance` | `id`, `channel` | `<unit>^2` | Offline population variance feature record. |
| `standard_deviation` | `id`, `channel` | source channel unit | Offline population standard-deviation feature record. |
| `skewness` | `id`, `channel` | `ratio` | Offline feature record; rejects constant samples. |
| `kurtosis` | `id`, `channel` | `ratio` | Offline feature record; rejects constant samples. |
| `percentile` | `id`, `channel`, `percentile` | source channel unit | Offline linear-interpolated percentile; method context records percentile. |
| `quantile` | `id`, `channel`, `quantile` | source channel unit | Offline linear-interpolated quantile; method context records quantile. |
| `histogram` | `id`, `channel`, `bins`; optional `min_v`, `max_v` | `count` | Emits one record per bin with IDs `{id}_bin_{index}` and bin method context. |
| `covariance` | `id`, `channel`, `other_channel` | `<unit>*<other_unit>` | Offline population covariance over aligned samples. |
| `correlation` | `id`, `channel`, `other_channel` | `ratio` | Offline Pearson correlation over aligned samples; rejects constant channels. |
| `autocorrelation` | `id`, `channel`, `lag_samples` | `ratio` | Offline lagged Pearson correlation using `channel[t]` vs `channel[t + lag]`. |
| `cross_correlation` | `id`, `channel`, `other_channel`, `lag_samples` | `ratio` | Offline lagged Pearson correlation using `channel[t]` vs `other_channel[t + lag]`. |
| `window_function` | `id`, `channel`; optional `window`, `window_samples`, `window_beta`, `window_alpha`, `window_sigma` | `ratio` | Emits one feature record per window coefficient. Supported windows: `rectangular`, `hann`, `hamming`, `blackman`, `blackman_harris`, `flat_top`, `kaiser`, `tukey`, `bartlett`, `gaussian`. |
| `dft` | `id`, `channel`; optional `window` | source channel unit | One-sided DFT peak-amplitude records with frequency-bin method context. |
| `fft` | `id`, `channel`; optional `window` | source channel unit | Radix-2 FFT with DFT fallback; emits one-sided peak-amplitude records. |
| `ifft` | `id`, `channel`; optional `other_channel` for imaginary bins | source channel unit | Inverse transform from real and optional imaginary frequency-bin channels; emits reconstructed sample records. |
| `power_spectrum` | `id`, `channel`; optional `window` | `<unit>^2` | One-sided power spectrum records. |
| `psd` | `id`, `channel`; optional `window` | `<unit>^2/Hz` | One-sided periodogram PSD records. |
| `welch_psd` | `id`, `channel`, `window_samples`; optional `window`, `overlap_samples` | `<unit>^2/Hz` | Averaged one-sided PSD over complete windowed segments. |
| `cross_spectrum` | `id`, `channel`, `other_channel`; optional `window`, `window_samples`, `overlap_samples` | `<unit>*<other_unit>/Hz` | Averaged cross power spectral density records. |
| `coherence` | `id`, `channel`, `other_channel`; optional `window`, `window_samples`, `overlap_samples` | `ratio` | Magnitude-squared coherence records. |
| `transfer_function` | `id`, `channel`, `other_channel`; optional `window`, `window_samples`, `overlap_samples` | `<other_unit>/<unit>` | Output-over-input transfer magnitude records with complex context. |
| `harmonic_analysis` | `id`, `channel`; optional `window`, `fundamental_hz`, `harmonic_count` | source channel unit | Harmonic amplitude records using configured or dominant-bin fundamental. |
| `thd` | `id`, `channel`; optional `window`, `fundamental_hz`, `harmonic_count` | `ratio` | Total harmonic distortion from harmonic power. |
| `snr` | `id`, `channel`; optional `window`, `fundamental_hz`, `harmonic_count` | `dB` | Fundamental power over residual non-harmonic power. |
| `sinad` | `id`, `channel`; optional `window`, `fundamental_hz`, `harmonic_count` | `dB` | Fundamental power over residual noise and distortion. |
| `enob` | `id`, `channel`; optional `window`, `fundamental_hz`, `harmonic_count` | `bits` | ENOB derived from SINAD. |
| `stft` | `id`, `channel`, `window_samples`; optional `window`, `overlap_samples` | source channel unit | STFT amplitude records per complete segment and frequency bin. |
| `spectrogram` | `id`, `channel`, `window_samples`; optional `window`, `overlap_samples` | `<unit>^2/Hz` | Spectrogram PSD records per complete segment and frequency bin. |
| `spectral_centroid` | `id`, `channel`; optional `window` | `Hz` | Power-weighted spectral centroid. |
| `spectral_bandwidth` | `id`, `channel`; optional `window` | `Hz` | Power-weighted spectral bandwidth around centroid. |
| `spectral_rolloff` | `id`, `channel`; optional `window`, `rolloff_percent` | `Hz` | Frequency where cumulative positive-frequency power reaches the configured percentage; default is 85%. |
| `band_power` | `id`, `channel`, `band_low_hz`, `band_high_hz`; optional `window` | `<unit>^2` | One-sided power summed inside a frequency band. |

Example M31 feature records:

```toml
[[feature_transforms]]
id = "m31_rms"
type = "rms"
channel = "input_v"

[[feature_transforms]]
id = "m31_energy"
type = "energy"
channel = "input_v"

[[feature_transforms]]
id = "m31_impulse"
type = "impulse_estimate"
channel = "input_v"
```

Example M32 feature records:

```toml
[[feature_transforms]]
id = "m32_mean"
type = "mean"
channel = "input_v"

[[feature_transforms]]
id = "m32_p95"
type = "percentile"
channel = "input_v"
percentile = 95.0

[[feature_transforms]]
id = "m32_histogram"
type = "histogram"
channel = "input_v"
bins = 10

[[feature_transforms]]
id = "m32_cross_correlation"
type = "cross_correlation"
channel = "input_v"
other_channel = "reference_v"
lag_samples = 2
```

Example M33 spectral feature records:

```toml
[[feature_transforms]]
id = "m33_fft"
type = "fft"
channel = "input_v"
window = "hann"

[[feature_transforms]]
id = "m33_welch"
type = "welch_psd"
channel = "input_v"
window = "hann"
window_samples = 256
overlap_samples = 128

[[feature_transforms]]
id = "m33_transfer"
type = "transfer_function"
channel = "input_v"
other_channel = "output_v"

[[feature_transforms]]
id = "m33_spectrogram"
type = "spectrogram"
channel = "input_v"
window_samples = 256
overlap_samples = 128

[[feature_transforms]]
id = "m33_band_power"
type = "band_power"
channel = "input_v"
band_low_hz = 10.0
band_high_hz = 100.0
```

M33 spectral conventions:

- `fft` uses radix-2 FFT for power-of-two lengths and DFT fallback for other lengths.
- `dft`, `fft`, `power_spectrum`, and `psd` emit one-sided records from DC through Nyquist for real-valued input.
- Peak-amplitude spectra divide by coherent window gain; interior one-sided bins are doubled.
- Power spectra use peak amplitude squared divided by two for interior sine bins; DC and Nyquist bins are not halved.
- PSD records use one-sided periodogram scaling with `sample_rate_hz * sum(window^2)`.
- `welch_psd`, `stft`, and `spectrogram` use complete segments only and reject `overlap_samples >= window_samples`.
- Spectral and time-frequency outputs are `feature_records`; they do not mutate waveform samples or directly decide pass/fail.

## Legacy Criteria

Legacy criteria use `type` directly on `[[criteria]]`.

| `type` | Required Fields | Optional Fields |
|---|---|---|
| `minimum_voltage` | `id`, `channel`, `threshold` | none |
| `maximum_voltage` | `id`, `channel`, `threshold` | none |
| `state_transitions` | `id`, `channel`, `threshold`, `expected_count` | none |
| `pulse_width` | `id`, `channel`, `threshold` | `min_width_s`, `max_width_s`, `state` |
| `transient_duration` | `id`, `channel`, `threshold`, `max_duration_s`, `expected_state` | `event_kind` |
| `transient_event` | `id`, `channel`, `threshold`, `max_duration_s`, `expected_state`, `event_kind` | `start_time_s`, `end_time_s`, `arm_after_first_expected_state` |
| `response_latency` | `id`, `source_channel`, `target_channel`, `source_threshold`, `target_threshold`, `source_state`, `expected_target_state`, `max_latency_s` | none |
| `stable_state_duration` | `id`, `channel`, `threshold`, `state`, `min_duration_s` | none |
| `rise_fall_time` | `id`, `channel`, `direction`, `low_threshold`, `high_threshold`, `max_time_s` | none |

## Measurement-Backed Criteria DSL

DSL criteria use nested `measurement` and `requirement` sections.

```toml
[[criteria]]
id = "input_max_voltage"
channel = "input_v"

[criteria.measurement]
type = "maximum_sample"

[criteria.requirement]
operator = "less_than_or_equal"
value = { value = 5.5, unit = "V" }
```

Supported operators:

- `less_than`
- `less_than_or_equal`
- `greater_than`
- `greater_than_or_equal`
- `equal_to`

Supported requirement units are explicit unit fields such as `V`, `s`, and `count`. Shorthand values such as `10ms` are not supported.

## Event Transforms

| `type` | Required Fields | Optional Fields | Output |
|---|---|---|---|
| `schmitt_trigger` | `id`, `channel`, `on_threshold_v`, `off_threshold_v`, `initial_state` | none | State transition event records. |
| `edge_extraction` | `id`, `channel` | `direction` | Edge event records. |
| `debounce` | `id`, `channel`, `min_duration_s` | none | Rejected-pulse event records. |
| `glitch_removal` | `id`, `channel`, `max_duration_s` | none | Rejected-pulse event records. |
| `bounce_detection` | `id`, `channel`, `window_s` | none | Bounce event records. |

Event records are evidence, not pass/fail decisions by themselves.

## Event Validations

| `type` | Required Fields | Optional Fields |
|---|---|---|
| `missing_pulse` | `id`, `channel`, `direction`, `expected_count` | none |
| `extra_pulse` | `id`, `channel`, `direction`, `max_count` | none |
| `dwell_time` | `id`, `channel`, `state`, `min_duration_s` | none |
| `timeout` | `id`, `channel`, `state`, `start_time_s`, `max_time_s` | none |

Event validation failures contribute to `overall_outcome = fail`.

## Batch Manifest Shape

Batch manifests are local TOML files consumed by `ferrisoxide-signal batch`.

```toml
default_format = "json"
summary_file = "batch-summary.json"

[[runs]]
id = "basic_config"
input = "basic-waveform.csv"
config = "basic-config.toml"
report = "basic-config.json"
```

| Field | Required | Meaning |
|---|---|---|
| `output_dir` | No when `--output-dir` is passed | Output directory for all reports and the summary. Manifest-relative paths resolve from the manifest directory; CLI `--output-dir` paths resolve from the current working directory. |
| `summary_file` | No | Summary file name under the output directory. Defaults to `batch-summary.json`. |
| `default_format` | No | `json` or `text`. Defaults to `json`. |
| `[[runs]].id` | Yes | Stable run ID. Must be unique and cannot contain path separators. |
| `[[runs]].input` | Yes | CSV path. Relative paths resolve from the manifest directory. |
| `[[runs]].config` | Yes | Analysis config path. Relative paths resolve from the manifest directory. |
| `[[runs]].report` | No | Per-run report path under the output directory. Defaults to `{id}.json` or `{id}.txt`. |
| `[[runs]].format` | No | Per-run report format, `json` or `text`. |

Batch output paths must be relative and must not contain parent directory components. The command refuses to overwrite existing report and summary files unless `--overwrite` is passed.

## Compatibility Policy

- Additive fields may be proposed when golden fixtures, report docs, and release notes are updated together.
- Removing or renaming config fields, report fields, artifact names, or behavior requires a schema compatibility approval gate.
- New dependencies require dependency approval before implementation.
- New runtime, DAQ, HAL/RTOS, hardware, binary package, signing, or certification scope requires a fresh proposal and approval gate.

## Invalid-Config Matrix

| Invalid Case | Expected Behavior | Existing Evidence |
|---|---|---|
| Empty `input.channels` | Reject before analysis. | Config loader checks. |
| Empty `criteria` | Reject before analysis. | Config loader checks. |
| Unsupported `filters.type` | Reject with `unsupported filter type`. | Config tests. |
| Missing required filter field | Reject with the missing field name. | Config tests. |
| Invalid `high_pass_baseline.cutoff_hz` | Reject zero, negative, non-finite, or missing cutoff. | M14 tests. |
| Invalid M26 timing filter field | Reject missing or non-finite `start_time_s`, `end_time_s`, `delay_s`, `sample_interval_s`, or empty `channel` where required. | M26 config tests. |
| M26 repair/resampling invalid data | Reject all-NaN interpolation channels, infinite samples, invalid timestamps, empty crop windows, and oversized fixed grids. | M26 core tests. |
| Invalid M27 pointwise/nonlinear config | Reject missing `base`, `mode`, `limit_v`, `points`, or `coefficients`; reject inverted normalization ranges, empty/unsorted piecewise points, and empty coefficient lists. | M27 config tests. |
| Invalid M27 transform domain | Reject square-root of negative samples, log of non-positive samples, invalid log/exp bases, constant normalization inputs where a range is required, and non-finite outputs. | M27 core tests. |
| Invalid M28 smoothing/baseline config | Reject missing or invalid `weights`, `alpha`, `window_samples`, `sigma_samples`, `polynomial_order`, `outlier_sigma`, and `threshold_v`. | M28 config tests. |
| Invalid M28 transform domain | Reject non-finite samples, invalid time axes for detrending, even Savitzky-Golay windows, unsupported polynomial orders, and non-positive smoothing/spike thresholds. | M28 core tests. |
| Invalid M29 frequency-filter config | Reject missing or invalid coefficients, cutoff/center frequency, Q, delay, feedback gain, ripple, or stopband attenuation values. | M29 config tests. |
| Invalid M29 transform domain | Reject unstable IIR biquads, nonuniform timing for designed filters, and above-Nyquist cutoff/center frequencies. | M29 core tests. |
| Invalid M30 resampling/timing config | Reject missing or invalid `sample_interval_s`, `factor`, `upsample_factor`, `downsample_factor`, `delay_s`, `reference_channel`, `target_channel`, or `max_lag_samples` values. | M30 config tests. |
| Invalid M30 transform domain | Reject invalid time axes, non-finite samples where interpolation is required, invalid resampling factors, decimation cutoffs above target Nyquist, missing alignment channels, and cross-correlation inputs that cannot produce a finite estimate. | M30 core tests. |
| Invalid M31 envelope/calculus config | Reject invalid `alpha`, `window_samples`, `time_constant_s`, or `threshold_per_s`; reject missing feature IDs/channels or unsupported feature types. | M31 config tests. |
| Invalid M31 transform domain | Reject invalid time axes for derivative/integral/leaky/slope calculations, non-finite samples, zero RMS crest-factor inputs, and feature references to unknown channels. | M31 core and feature tests. |
| Invalid M32 statistics/correlation config | Reject invalid `window_samples`, `threshold_sigma`, `lower_quantile`, `upper_quantile`, missing percentile/quantile/bin/other-channel/lag fields, and zero histogram bins. | M32 config tests. |
| Invalid M32 transform domain | Reject constant z-score/skewness/kurtosis/correlation inputs, invalid histogram ranges, non-finite samples, invalid percentiles/quantiles, lag values greater than or equal to sample count, and unknown feature channels. | M32 core and feature tests. |
| Invalid M34 simulation config | Reject missing stochastic seeds, missing probability controls, invalid companding modes, missing jitter values, missing ADC missing-code values, and empty INL/DNL coefficient lists. | M34 config tests. |
| Invalid M34 transform domain | Reject invalid probabilities, invalid ADC ranges or missing-code indexes, out-of-window stuck-at ranges, excessive sample-clock jitter, non-finite samples where simulation requires finite input, and invalid quantizer ranges. | M34 core tests. |
| Invalid M35 domain-pack config | Reject missing output channels, insufficient channel lists, empty matrix rows, missing sensor constants, missing vibration windows, and missing PID/rate/control constants. | M35 config tests. |
| Invalid M35 transform domain | Reject missing channels, mismatched multi-channel units, invalid matrix shape, non-positive thermistor samples, non-monotonic time axes for integration/PID/rate limiting, and non-positive control/sensor parameters. | M35 core tests. |
| Mixed legacy and DSL criterion fields | Reject ambiguous shape. | DSL invalid-config tests. |
| Unknown DSL operator | Reject with clear operator error. | DSL invalid-config tests. |
| Missing DSL measurement or requirement | Reject. | DSL invalid-config tests. |
| Unsupported unit or mismatched requirement unit | Reject. | DSL invalid-config tests. |
| Invalid event transform thresholds or states | Reject. | M12 config tests. |
| Empty batch manifest | Reject before running any analysis. | M17 CLI test. |
| Duplicate batch run ID | Reject manifest. | Batch manifest validator. |
| Batch report path with parent directory | Reject manifest. | Batch manifest validator. |

## Hand-Off Note

Role: Documentation Engineer / Software Architect
Goal: Document the implemented CLI/config surface before MVP exit.
Files changed: `docs/config-reference.md`, README links, batch example, validation and traceability updates.
Checks run: See `docs/validation-log.md`.
Status: M15 reference artifact complete locally and updated through M36.
Known gaps: This is a human-maintained reference; automated config-reference generation is not implemented. Exact elliptic/Cauer design remains dependency-gated, efficient polyphase resampling remains dependency/performance-gated, Hilbert envelope remains dependency/design-gated, optimized FFT/STFT throughput work remains separately gated, `split_by_event` remains future-gated, and advanced acoustic/calibration packs remain dependency/design gated.
Next recommended step: Keep this document updated with every future config, CLI, or report schema change.
