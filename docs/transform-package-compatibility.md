# Transform Package Compatibility

Date: 2026-06-02

Status: M21-M24 runtime-path compatibility contract updated through M36 closure. M25 adds catalog enforcement; M26-M35 add desktop-only transform families and feature records with package-export rejection where portable semantics are not approved; M36 confirms the compatibility map, validation corpus, and release messaging. This document connects implemented desktop transforms to rule-package export behavior and the current shared runtime-compatible semantics. It does not claim embedded runtime package loading, target hardware execution, binary package serialization, signing, hardware qualification, or certification evidence.

## Compatibility Rule

FerrisOxide supports more desktop transforms than portable rule packages can safely represent today. Rule-package export must reject unsupported transforms with clear errors rather than silently dropping, reordering, or approximating them.

M25 stores package compatibility in `crates/ferrisoxide-core/src/transform_catalog.rs`. The CLI export path checks the catalog before converting filters into rule-package schema definitions.

The current rule-package filter subset is:

- `moving_average`
- `low_pass`
- `adc_quantize`
- `offset`
- `gain`
- `invert`

`offset`, `gain`, and `invert` are supported as linear software transforms only. They are not calibration evidence, engineering-unit conversion proof, sensor accuracy evidence, hardware ADC evidence, or certification evidence.

All other implemented transforms remain desktop-only for package export until a later milestone proves portable semantics, runtime-profile support, validation evidence, and compatibility behavior.

## Transform Matrix

| Transform | Desktop Analyze | Structured Metadata | Rule-Package Export | Reason |
|---|---|---|---|---|
| `moving_average` | Supported | Supported | Supported legacy subset | Simple finite-window FIR-style smoothing already represented in rule schema. |
| `low_pass` | Supported | Supported | Supported legacy subset | First-order cutoff represented in rule schema. |
| `adc_quantize` | Supported | Supported | Supported legacy subset | Ideal quantization represented in rule schema; not hardware ADC validation. |
| `offset` | Supported | Supported | Supported M21 subset | Unit-bearing voltage offset represented in rule schema; software transform only, not calibration evidence. |
| `gain` | Supported | Supported | Supported M21 subset | Finite ratio represented in rule schema; software transform only, not sensor-span calibration evidence. |
| `invert` | Supported | Supported | Supported M21 subset | Parameterless polarity inversion represented in rule schema. |
| `clamp` | Supported | Supported | Rejected | May hide excursions; package semantics need explicit evidence. |
| `deadband` | Supported | Supported | Rejected | Nonlinear behavior can hide small changes; package semantics need explicit evidence. |
| `dc_remove` | Supported | Supported | Rejected | Baseline removal can hide failures; package semantics need explicit evidence. |
| `baseline_subtract` | Supported | Supported | Rejected | Baseline assumptions are desktop-only for now. |
| `high_pass_baseline` | Supported | Supported | Rejected | Timing-dependent, stateful, and phase-delaying; remains desktop-only until runtime/package semantics are approved. |
| `moving_median` | Supported | Supported | Rejected | Nonlinear window behavior and edge handling are not yet represented in rule schema. |
| `timestamp_sort` | Supported | Supported | Rejected | Offline timestamp repair changes sample order and needs package/runtime semantics before export. |
| `dedupe_timestamps` | Supported | Supported | Rejected | Duplicate timestamp policy changes sample count and needs explicit package/runtime semantics. |
| `nan_interpolate` | Supported | Supported | Rejected | Offline invalid-sample repair is not represented in rule packages. |
| `nan_remove` | Supported | Supported | Rejected | Row-removal behavior changes sample count and channel evidence. |
| `crop` | Supported | Supported | Rejected | Offline segment selection changes the analyzed sample window. |
| `fixed_delay` | Supported | Supported | Rejected | Timestamp shifting needs runtime/package timing semantics before export. |
| `gap_fill` | Supported | Supported | Rejected | Fixed-grid interpolation changes sample count and timing evidence. |
| `resample_fixed` | Supported | Supported | Rejected | Fixed-rate resampling needs package/runtime timing semantics before export. |
| `channel_delay` | Supported | Supported | Rejected | Offline channel alignment is not represented in rule packages. |
| `absolute_value` | Supported | Supported | Rejected | Nonlinear magnitude behavior is not represented in rule packages. |
| `square` | Supported | Supported | Rejected | Nonlinear squaring can change margin interpretation and needs portable semantics. |
| `square_root` | Supported | Supported | Rejected | Domain rejection and nonlinear behavior need explicit package/runtime semantics. |
| `log` | Supported | Supported | Rejected | Domain rejection and base handling are not represented in rule packages. |
| `exp` | Supported | Supported | Rejected | Finite-output checks and base handling are not represented in rule packages. |
| `normalize` | Supported | Supported | Rejected | Offline whole-record normalization is not portable in the current rule schema. |
| `tanh` | Supported | Supported | Rejected | Nonlinear compression is not represented in rule packages. |
| `sigmoid` | Supported | Supported | Rejected | Nonlinear logistic mapping is not represented in rule packages. |
| `soft_limit` | Supported | Supported | Rejected | Smooth limiting may hide excursions and needs explicit portable semantics. |
| `piecewise_linear` | Supported | Supported | Rejected | Lookup/piecewise behavior needs package schema, bounds, and runtime evidence. |
| `polynomial` | Supported | Supported | Rejected | Polynomial correction needs schema, coefficient bounds, and runtime evidence. |
| `weighted_moving_average` | Supported | Supported | Rejected | Weight-vector semantics and edge behavior are not represented in rule packages. |
| `exponential_moving_average` | Supported | Supported | Rejected | Stateful smoothing semantics need explicit runtime/package evidence. |
| `boxcar_smoothing` | Supported | Supported | Rejected | Offline centered smoothing is not represented in rule packages. |
| `gaussian_smoothing` | Supported | Supported | Rejected | Kernel and edge-renormalization semantics need package/runtime evidence. |
| `savitzky_golay` | Supported | Supported | Rejected | Local polynomial fitting is offline and not represented in rule packages. |
| `centered_moving_median` | Supported | Supported | Rejected | Offline centered median edge handling is not represented in rule packages. |
| `rolling_mean_baseline` | Supported | Supported | Rejected | Baseline removal can hide failures; portable semantics need explicit evidence. |
| `rolling_median_baseline` | Supported | Supported | Rejected | Nonlinear baseline removal can hide failures and needs package evidence. |
| `linear_detrend` | Supported | Supported | Rejected | Offline least-squares detrending requires whole-record timing evidence. |
| `polynomial_detrend` | Supported | Supported | Rejected | Offline polynomial fitting requires schema, bounds, and timing evidence. |
| `hampel_filter` | Supported | Supported | Rejected | Robust outlier replacement is offline and nonlinear. |
| `spike_remove` | Supported | Supported | Rejected | Median-window replacement and threshold semantics need package evidence. |
| `fir_filter` | Supported | Supported | Rejected | Coefficient vectors, edge behavior, and package/runtime semantics are not represented in rule packages. |
| `zero_phase_fir_filter` | Supported | Supported | Rejected | Offline forward/backward filtering is not portable in the current rule schema. |
| `iir_biquad` | Supported | Supported | Rejected | Biquad coefficient and stability semantics need package/runtime evidence. |
| `zero_phase_iir_biquad` | Supported | Supported | Rejected | Offline forward/backward IIR filtering is not portable in the current rule schema. |
| `high_pass` | Supported | Supported | Rejected | Timing-dependent frequency filtering needs package/runtime sample-rate semantics. |
| `band_pass` | Supported | Supported | Rejected | Designed biquad behavior, Q, and sample-rate assumptions are not represented in rule packages. |
| `band_stop` | Supported | Supported | Rejected | Designed biquad behavior, Q, and sample-rate assumptions are not represented in rule packages. |
| `notch` | Supported | Supported | Rejected | Narrow-band rejection can hide content and needs explicit portable semantics. |
| `comb_filter` | Supported | Supported | Rejected | Delay/gain comb behavior needs package/runtime evidence. |
| `butterworth_low_pass` | Supported | Supported | Rejected | Named filter-family design is not represented in the current rule schema. |
| `butterworth_high_pass` | Supported | Supported | Rejected | Named filter-family design is not represented in the current rule schema. |
| `chebyshev1_low_pass` | Supported | Supported | Rejected | Named filter-family design and ripple semantics need package/runtime evidence. |
| `chebyshev2_low_pass` | Supported | Supported | Rejected | Named filter-family design and stopband semantics need package/runtime evidence. |
| `bessel_low_pass` | Supported | Supported | Rejected | Named filter-family design is not represented in the current rule schema. |
| `resample` | Supported | Supported | Rejected | Fixed-grid resampling changes timing/sample count and needs package/runtime semantics. |
| `downsample` | Supported | Supported | Rejected | Integer sample dropping can alias content unless package evidence defines preconditions. |
| `decimate` | Supported | Supported | Rejected | Anti-alias prefiltering, factor, cutoff, and timing assumptions are not represented in rule packages. |
| `upsample` | Supported | Supported | Rejected | Interpolated sample insertion changes timing/sample count and needs package/runtime evidence. |
| `interpolate` | Supported | Supported | Rejected | Offline interpolation onto a new grid is not represented in rule packages. |
| `rational_resample` | Supported | Supported | Rejected | Rational grid conversion and derived interval semantics need package/runtime evidence. |
| `sample_and_hold` | Supported | Supported | Rejected | Hold reconstruction changes timing evidence and is not represented in rule packages. |
| `zero_order_hold` | Supported | Supported | Rejected | Previous-sample hold reconstruction needs explicit package/runtime semantics. |
| `first_order_hold` | Supported | Supported | Rejected | Linear hold reconstruction changes derived samples and needs package evidence. |
| `fractional_delay` | Supported | Supported | Rejected | Sub-sample interpolation and endpoint hold semantics are not represented in rule packages. |
| `cross_correlation_delay` | Supported | Supported | Rejected | Offline alignment estimates lag/confidence and mutates derived channel timing; package semantics are not approved. |
| `jitter_correction` | Supported | Supported | Rejected | Offline jitter repair changes timestamp grid and is not package/runtime supported. |
| `clock_drift_correction` | Supported | Supported | Rejected | Offline drift correction is not DAQ clock calibration and needs package/runtime evidence before export. |
| `half_wave_rectify` | Supported | Supported | Rejected | Nonlinear rectification changes evidence interpretation and needs package/runtime semantics. |
| `full_wave_rectify` | Supported | Supported | Rejected | Magnitude-only conversion can hide polarity and needs package/runtime evidence. |
| `envelope` | Supported | Supported | Rejected | Stateful envelope smoothing needs package/runtime semantics and edge behavior evidence. |
| `moving_rms` | Supported | Supported | Rejected | Windowed RMS edge behavior and units need package/runtime evidence. |
| `peak_hold` | Supported | Supported | Rejected | Stateful peak retention needs package/runtime reset and evidence semantics. |
| `first_derivative` | Supported | Supported | Rejected | Derivative units and timing assumptions are not represented in rule packages. |
| `second_derivative` | Supported | Supported | Rejected | Second-derivative timing assumptions are not represented in rule packages. |
| `integral` | Supported | Supported | Rejected | Accumulation units and time-axis semantics need package/runtime evidence. |
| `cumulative_integral` | Supported | Supported | Rejected | Accumulation semantics need package/runtime evidence. |
| `leaky_integrator` | Supported | Supported | Rejected | Stateful decay behavior needs package/runtime reset and timing semantics. |
| `slope_detection` | Supported | Supported | Rejected | Slope threshold units and nonlinear output semantics are not represented in rule packages. |
| `rolling_mean` | Supported | Supported | Rejected | Rolling-statistics edge behavior and reset semantics are not represented in rule packages. |
| `rolling_variance` | Supported | Supported | Rejected | Rolling population variance needs package/runtime semantics before export. |
| `rolling_stddev` | Supported | Supported | Rejected | Rolling standard deviation needs package/runtime semantics before export. |
| `rolling_min` | Supported | Supported | Rejected | Rolling minimum edge behavior is not represented in rule packages. |
| `rolling_max` | Supported | Supported | Rejected | Rolling maximum edge behavior is not represented in rule packages. |
| `z_score` | Supported | Supported | Rejected | Whole-record population statistics are offline and not represented in rule packages. |
| `outlier_detection` | Supported | Supported | Rejected | Distribution-derived 1/0 flags need package/runtime semantics before export. |
| `quantile_clip` | Supported | Supported | Rejected | Whole-record quantile clipping is offline and may hide excursions. |
| `[[feature_transforms]]` | Supported | Supported as `feature_records` | Rejected | Feature records are analysis evidence, not rule-package filter definitions. |
| M34 fault/ADC simulation filters | Supported | Supported | Rejected | Simulation-only evidence is desktop-derived and not portable package, hardware, calibration, or certification evidence. |
| M35 multi-channel/sensor/domain filters | Supported | Supported | Rejected | Derived-channel, software formula, and control-conditioning semantics need package/runtime approval before export. |

## Export Guardrail

The CLI export path calls `schema_filters()` before writing artifacts. Unsupported transforms return:

```text
rule package export does not yet support transform `<name>`
```

M14 covers the `high_pass_baseline` rejection path. M18 documented the full matrix. M21 expands support only for `offset`, `gain`, and `invert`. M26 extends the rejection matrix to all data-cleaning and timing-conditioning transforms. M27 extends the rejection matrix to nonlinear pointwise and normalization transforms. M28 extends the rejection matrix to smoothing, detrending, baseline, Hampel, and spike-cleanup transforms. M29 extends the rejection matrix to desktop-only frequency filters. M30 extends the rejection matrix to desktop-only resampling and timing-alignment transforms. M31 extends the rejection matrix to envelope/calculus waveform filters and explicitly rejects `feature_transforms` export. M32 extends the rejection matrix to statistics waveform filters while continuing to reject `feature_transforms` export. M34 and M35 extend the rejection matrix to fault/ADC simulation and multi-channel/sensor/domain filters. M36 confirms the guardrail remains catalog-driven. Rejected transforms continue to fail with the same error shape.

## Shared Runtime-Compatible Semantics

M22 adds `apply_borrowed_transform_chain()` and `BorrowedTransformStep` in `crates/ferrisoxide-rule-engine`. The helper:

- operates on caller-provided `&[f64]` input and `&mut [f64]` output buffers,
- supports `Offset`, `Gain`, and `Invert` in declared order,
- rejects empty input, output-capacity mismatch, non-finite inputs, non-finite parameters, and non-finite outputs,
- avoids CSV parsing, TOML/JSON parsing, filesystem access, report rendering, plotting, package loading, DAQ, HAL, RTOS SDK, and target-hardware dependencies.

CLI parity coverage compares desktop `FilterStep` output with the borrowed runtime-compatible helper for the same linear pointwise chain.

## Compatibility Corpus

M23 adds parse-tested positive fixtures:

- `examples/rule-package/linear-pointwise-rules.toml`
- `examples/rule-package/linear-pointwise-rules.json`

M23 also adds negative fixtures that prove unsupported nonlinear package transforms still fail as unknown filters:

- `examples/rule-package/unsupported-clamp-rules.toml`
- `examples/rule-package/unsupported-clamp-rules.json`

## Future Support Requirements

Before any rejected transform can become exportable, the approving milestone must define:

- rule-schema representation,
- runtime-profile compatibility,
- timing and sample-rate evidence requirements,
- portable validation fixtures,
- exact export artifacts,
- report/schema compatibility impact,
- embedded/no_std constraints if runtime support is claimed,
- non-goals around DAQ, hardware, calibration, and certification evidence.

## Non-Goals

M21-M24 do not add:

- binary package serialization,
- runtime package loaders,
- target hardware execution,
- HAL/RTOS SDK integration,
- cryptographic signing,
- hardware qualification or certification evidence.

M24 records a runtime-loader design gate in `docs/runtime-loader-design-gate.md`; implementation remains separately gated.

## Hand-Off Note

Role: Software Architect / Embedded RTOS Engineer
Goal: Define package export semantics for implemented transforms and the first runtime-compatible transform subset.
Files changed: `docs/transform-package-compatibility.md`, `docs/rule-package-format.md`, runtime helper, package fixtures, docs, tests, validation and traceability updates.
Checks run: See `docs/validation-log.md`.
Status: M21-M24 compatibility contract complete locally and updated through M36 closure.
Known gaps: No runtime package loader, binary package format, HAL/RTOS adapter, target execution, M31-M35 portable transform semantics, multi-artifact segmentation package semantics, or calibration/timing evidence added.
Next recommended step: Keep loader implementation and any additional portable transform export behind fresh approval gates.
