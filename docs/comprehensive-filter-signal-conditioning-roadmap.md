# Comprehensive Filter And Signal Conditioning Roadmap

Date: 2026-06-02

Status: Execution roadmap for M25-M36. M25 is complete as the transform registry and completeness contract. M26 is complete for data-cleaning and timing-conditioning transforms. M27 is complete for pointwise, normalization, and nonlinear conditioning transforms. M28 is complete for smoothing, detrending, baseline, Hampel, and spike-cleanup transforms. M29 is complete for standard frequency filters. M30 is complete for resampling and timing alignment transforms. M31 is complete for envelope, energy, and calculus transforms. M32 is complete for statistical and correlation transforms. M33 is complete for spectrum, window, and time-frequency analysis. M34 is complete for deterministic fault injection and ADC/DAC simulation. M35 is complete for multi-channel, sensor, vibration, and control conditioning. M36 is complete as the comprehensive-suite closure milestone and merged to `main` through PR #175. This roadmap defines the completed milestone path for making FerrisOxide a comprehensive sampled-waveform conditioning and calculation library. It does not add dependencies, create GitHub issues, publish a release tag, add live DAQ, add HAL/RTOS SDKs, implement runtime loaders, execute target hardware, or create hardware qualification or certification evidence.

Related requirements: WRA-RQ-110 through WRA-RQ-121.

## Product Intent

After this milestone set, an engineer working with sampled DAQ or controller-test waveforms should be able to stay inside FerrisOxide for normal signal conditioning, simulated signal defects, engineering calculations, and evidence generation.

The intended coverage is comprehensive for practical sampled waveform preparation and analysis:

- data cleanup,
- pointwise and nonlinear transforms,
- smoothing and baseline correction,
- standard frequency filters,
- resampling and timing alignment,
- envelope, energy, calculus, and statistical calculations,
- spectrum and time-frequency analysis,
- simulated noise, faults, ADC/DAC behavior, and signal conditioning defects,
- multi-channel transforms and sensor-oriented engineering-unit conversions,
- validation fixtures, documentation, and transform catalog evidence.

This does not mean FerrisOxide will claim every academic DSP method, every proprietary instrument workflow, every sensor calibration standard, hardware validation, certified analysis, or real-time embedded implementation in one pass.

## Operating Rules

- Preserve raw waveform data; every transform creates derived channels, feature records, event records, or validation records.
- Make transform metadata mandatory: category, parameters, units, input/output channel behavior, sample-rate needs, causality, phase effect, streaming/offline support, runtime profile, and evidence level.
- Treat feature extraction and validation separately. A feature computes evidence; a validation transform decides pass/fail.
- Prefer dependency-free implementations for simple transforms.
- Require dependency review before FFT/filter-design/math packages, external numeric crates, or calibration libraries.
- Add known-answer fixtures before broad support claims.
- Keep desktop/offline support separate from no_std/runtime support.
- Keep rule-package exposure behind explicit compatibility gates.
- Keep hardware, live DAQ, HAL/RTOS, target execution, signing, and certification out of scope unless separately approved.

## Milestone Overview

| Milestone | Name | Primary Outcome | Dependency Gate |
|---|---|---|---|
| M25 | Transform Registry And Completeness Contract | Central transform catalog, capability schema, docs, config/report/package compatibility rules. | No new dependency expected |
| M26 | Data Cleaning And Timing Conditioning | Implemented locally for NaN repair/removal, timestamp sort/dedupe, crop, fixed delay, gap fill, fixed-rate resampling, and channel delay alignment; `split_by_event` remains future-gated because it creates multiple segment artifacts. | No new dependency added |
| M27 | Pointwise, Normalization, And Nonlinear Suite | Implemented locally for absolute/square/square-root/log/exp, zero-to-one/minus-one-to-one/z-score/range normalization, tanh, sigmoid, soft limit, and piecewise/polynomial transforms, with existing offset/gain/invert/clamp/deadband coverage retained. | No new dependency added |
| M28 | Smoothing, Detrending, And Baseline Suite | Implemented locally for weighted moving average, EMA, boxcar/Gaussian/Savitzky-Golay smoothing, centered moving median, rolling mean/median baseline correction, linear/polynomial detrend, Hampel filter, and spike removal. | No new dependency added |
| M29 | Standard Frequency Filter Suite | Implemented locally for FIR/IIR coefficient filters, high-pass, band-pass, band-stop, notch, feed-forward comb, Butterworth low/high-pass, Chebyshev Type I/II low-pass, Bessel low-pass, zero-phase FIR/IIR offline filtering, catalog metadata, sample-rate/Nyquist validation, and dependency-gated elliptic/Cauer planning. | No new dependency added; elliptic/Cauer remains dependency-gated |
| M30 | Resampling And Timing Alignment Suite | Implemented locally for fixed-grid resampling/interpolation, downsample, decimate with anti-alias prefiltering, upsample, dependency-free rational resampling, sample/zero/first-order hold, fractional delay, cross-correlation delay alignment, jitter correction, and clock-drift correction; efficient polyphase resampling remains dependency/performance-gated. | No new dependency added; polyphase remains dependency-gated |
| M31 | Envelope, Energy, And Calculus Suite | Implemented locally for half/full-wave rectification, envelope, moving RMS, peak hold, first/second derivative, integral/cumulative integral, leaky integrator, slope detection, and feature records for RMS, peak-to-peak, crest factor, energy, power, area, and impulse estimate; Hilbert envelope remains dependency/design-gated. | No new dependency added |
| M32 | Statistical And Correlation Suite | Implemented locally for rolling mean/variance/stddev/min/max, z-score, outlier detection, quantile clipping, mean/median/mode/min/max, variance/stddev, skewness/kurtosis, percentile/quantile, histogram bin records, covariance, Pearson correlation, autocorrelation, and cross-correlation. | No new dependency added |
| M33 | Spectrum, Windows, And Time-Frequency Suite | Implemented locally for window coefficients, DFT, radix-2 FFT with DFT fallback, IFFT, power spectrum, PSD, Welch PSD, cross-spectrum, coherence, transfer estimate, harmonic analysis, THD, SNR, SINAD, ENOB, STFT, spectrogram, centroid, bandwidth, rolloff, and band power. | Dependency review complete; no new dependency added |
| M34 | Fault Injection And ADC/DAC Simulation Suite | Implemented locally for white/Gaussian/uniform/pink/brown noise, impulse/salt-pepper/quantization noise, periodic/hum interference, ground-bounce/thermal/random-walk drift, dropout/missing/saturation/stuck-at/flatline/intermittent faults, quantizer variants, dithering, companding, sample-clock jitter, missing codes, INL/DNL, gain error, and offset error. | RNG/noise dependency review complete; no new dependency added |
| M35 | Multi-Channel, Sensor, And Domain Conditioning Packs | Implemented locally for channel arithmetic, differential/common-mode, vector/euclidean norm, matrix transform, coordinate rotation, software sensor conversions, vibration integration/severity, and control-signal transforms; advanced phase/gain matching, acoustic features, and calibration packs remain dependency/design-gated. | No new dependency added |
| M36 | Completeness, UX, And Compatibility Closure | Complete for catalog completeness review, examples, docs, validation corpus, negative-case matrix, benchmark-readiness evidence, rule-package/runtime compatibility map, release/community/retrospective closure, stale-reference checks, and PR #175 mainline merge evidence. | No new dependency added |

## M25: Transform Registry And Completeness Contract

Goal: make the transform surface discoverable and enforceable before adding many algorithms.

Required artifacts:

- `TransformCatalog` or equivalent registry of implemented and planned transforms.
- Transform metadata contract for category, parameters, units, timing, phase, runtime, evidence, and package support.
- Config/report/package compatibility rules for adding transforms.
- Generated or source-of-truth transform catalog docs.
- Coverage matrix that separates implemented, planned, gated, dependency-gated, and hardware-gated transforms.

Acceptance criteria:

- Every existing transform appears in the catalog.
- Every existing transform has metadata tests.
- Docs can list supported transforms without stale manual duplication.
- Unsupported transforms still fail clearly in rule-package export.
- No new algorithm is added before the registry is in place.

## M26: Data Cleaning And Timing Conditioning

Goal: handle the messy DAQ export problems that block analysis before filtering begins.

Scope:

- Implemented locally: NaN removal and interpolation.
- Implemented locally: gap filling by fixed-grid linear interpolation.
- Deferred to M28: spike removal and Hampel-style outlier marking.
- Implemented locally: duplicate timestamp removal.
- Implemented locally: time sorting.
- Implemented locally: segment trimming and cropping.
- Future-gated after M36: split-by-event, because it creates multiple segment artifacts rather than one derived waveform and needs a segment-artifact/report contract.
- Implemented locally: channel resynchronization through `channel_delay`.
- Implemented locally: fixed-rate timestamp normalization through `resample_fixed`.
- Implemented locally: fixed delay and channel alignment.

Acceptance criteria:

- Invalid data handling is explicit and reported through structured errors for missing fields, non-finite parameters, invalid timestamps, all-NaN interpolation channels, empty crop windows, and oversized fixed grids.
- Raw data remains preserved by producing derived waveforms with transform history and structured transform metadata.
- Every cleaning action records the operation name, parameters, category, phase effect, streaming/offline behavior, package support, runtime profile, and evidence level in the transform catalog/report metadata.
- Known-answer tests and `examples/m26-data-cleaning-waveform.csv` cover missing samples, duplicate timestamps, unordered timestamps, and gaps.
- Cleaning transforms do not silently change pass/fail evidence without lineage; the M26 CLI fixture evaluates criteria after the full cleaning chain and reports the transform steps.

## M27: Pointwise, Normalization, And Nonlinear Suite

Goal: make per-sample conditioning complete enough that engineers do not need one-off scripts for ordinary waveform math.

Scope:

- offset, gain, invert extensions already implemented,
- Implemented locally: absolute value.
- Implemented locally: square.
- Implemented locally: square root with negative-sample rejection.
- Implemented locally: log and exp with base/domain/finite-output validation.
- Implemented locally: clamp/limit through existing `clamp`.
- Implemented locally: normalize to `0..1`, `-1..1`, z-score, and configured range,
- Implemented locally: deadzone/deadband through existing `deadband`.
- Implemented locally: hard clipping through existing `clamp`; soft clipping through `soft_limit`.
- Implemented locally: saturation-style bounded compression through `soft_limit`.
- Implemented locally: sigmoid and tanh.
- Implemented locally: lookup-table style mapping through `piecewise_linear`.
- Implemented locally: piecewise linear.
- Implemented locally: polynomial correction.

Acceptance criteria:

- Domain errors are structured and deterministic.
- Unit behavior is documented for every transform.
- Known-answer tests prove formulas.
- Config examples cover common DAQ scaling and normalization workflows.
- Calibration wording stays scoped: software transform support is not sensor calibration evidence.
- Rule-package export rejects all M27 transforms until separate package/runtime semantics are approved.

## M28: Smoothing, Detrending, And Baseline Suite

Goal: provide the standard preprocessing filters engineers expect before analysis.

Status: Complete for desktop `[[filters]]` support with no new dependencies. See `docs/m28-smoothing-baseline-pipeline-report.md`.

Scope:

- weighted moving average,
- exponential moving average,
- boxcar smoothing,
- Gaussian smoothing,
- Savitzky-Golay smoothing,
- moving median refinements,
- rolling mean/median baseline correction,
- linear detrend,
- polynomial detrend,
- Hampel filter,
- robust spike removal.

Acceptance criteria:

- Edge behavior is documented and tested through M29 core tests and metadata docs.
- Phase/latency behavior is explicit in transform metadata and catalog entries.
- Causal and offline variants are separated: trailing weighted/EMA/rolling baseline transforms are causal where appropriate, while centered smoothing, detrending, Hampel, and spike cleanup are offline-only.
- Detrending/baseline transforms include overfiltering warnings in metadata, risk, package compatibility, and the pipeline report.
- Synthetic drift, shape, and spike fixtures prove expected behavior in focused tests and the M28 CLI fixture.

## M29: Standard Frequency Filter Suite

Goal: make FerrisOxide useful as the normal filter toolbox for sampled waveforms.

Status: Complete for dependency-free desktop `[[filters]]` frequency conditioning. Exact elliptic/Cauer design remains dependency-gated pending numeric-library review.

Scope:

- FIR filter representation and convolution behavior,
- IIR filter representation and stability checks,
- low-pass, high-pass, band-pass, band-stop,
- notch filters,
- comb filters if validated,
- Butterworth,
- Chebyshev Type I and II,
- elliptic/Cauer,
- Bessel,
- Gaussian where appropriate,
- zero-phase offline forward/backward filtering.

Acceptance criteria:

- Filter design parameters are validated before execution.
- Frequency response fixtures or generated known-answer tests exist.
- Sample-rate requirements are enforced.
- Phase effects are visible in transform metadata.
- Zero-phase filters are offline-only.
- Dependency decision is recorded before using external numeric/filter-design crates.

Implemented M29 surface:

- `fir_filter` and `zero_phase_fir_filter`,
- `iir_biquad` and `zero_phase_iir_biquad` with pole-stability checks,
- `high_pass`,
- `band_pass`, `band_stop`, and `notch`,
- `comb_filter`,
- `butterworth_low_pass` and `butterworth_high_pass`,
- `chebyshev1_low_pass` and `chebyshev2_low_pass`,
- `bessel_low_pass`,
- `elliptic_low_pass` cataloged as dependency-gated rather than implemented without a suitable numeric-library review.

## M30: Resampling And Timing Alignment Suite

Goal: let engineers normalize sample grids and align channels before comparison.

Status: Complete for dependency-free desktop `[[filters]]` support with no new dependencies. See `docs/m30-resampling-timing-pipeline-report.md`.

Scope:

- Implemented locally: `resample` fixed-grid linear resampling.
- Implemented locally: `downsample` by integer factor.
- Implemented locally: `decimate` with first-order anti-alias prefiltering and target-Nyquist cutoff validation.
- Implemented locally: `upsample` by integer factor using linear interpolation.
- Implemented locally: `interpolate` onto a configured fixed grid.
- Implemented locally: `rational_resample` as dependency-free rational grid conversion using linear interpolation.
- Dependency-gated: efficient `polyphase_resample`, because it needs dependency/performance review before a stronger implementation claim.
- Implemented locally: `sample_and_hold`, `zero_order_hold`, and `first_order_hold`.
- Implemented locally: existing `fixed_delay`, plus M30 `fractional_delay`.
- Implemented locally: `cross_correlation_delay` with estimated lag, delay, and confidence metadata.
- Implemented locally: `jitter_correction` and `clock_drift_correction`.
- Implemented locally: irregular-to-fixed-grid normalization through `resample`, `interpolate`, `jitter_correction`, and `clock_drift_correction`.

Acceptance criteria:

- Time-axis assumptions are explicit through required seconds-based `sample_interval_s`, factor, cutoff, channel, and delay fields.
- Resampling records new timing metadata in derived `WaveformMetadata.sample_interval` and preserves transform history/steps.
- Anti-aliasing requirements are enforced for `decimate`: factor must be greater than one and `cutoff_hz` must not exceed target Nyquist.
- Alignment evidence records shift amount and confidence through `cross_correlation_delay` parameters `estimated_lag_samples`, `estimated_delay_s`, and `confidence`.
- Fixtures cover fixed-grid resampling, interpolation/holds, factor conversion, fractional delay, cross-correlation alignment, jitter repair, clock-drift correction, invalid timing/config cases, and package-export rejection.

## M31: Envelope, Energy, And Calculus Suite

Goal: cover common engineering calculations from conditioned waveforms.

Scope:

- Implemented locally through `[[filters]]`: `half_wave_rectify`, `full_wave_rectify`, `envelope`, `moving_rms`, `peak_hold`, `first_derivative`, `second_derivative`, `integral`, `cumulative_integral`, `leaky_integrator`, and `slope_detection`.
- Implemented locally through `[[feature_transforms]]`: `rms`, `peak_to_peak`, `crest_factor`, `energy`, `power`, `area_under_curve`, and `impulse_estimate`.
- Dependency/design-gated: Hilbert envelope and analytic-signal workflows.

Acceptance criteria:

- Calculation units are documented in feature records as source units, ratio, squared-unit seconds, squared units, or unit-seconds.
- Time-axis requirements are enforced for calculus, energy, power, area, impulse, leaky integration, and slope detection.
- Known-answer fixtures cover rectification, envelope smoothing, moving RMS, derivatives, integrals, leaky integration, slope detection, RMS, peak-to-peak, crest factor, energy, power, area, and impulse.
- Feature records stay separate from validation decisions and do not affect `overall_outcome`.

## M32: Statistical And Correlation Suite

Goal: make statistical waveform summaries and similarity calculations first-class evidence.

Scope:

- Implemented locally through `[[filters]]`: `rolling_mean`, `rolling_variance`, `rolling_stddev`, `rolling_min`, `rolling_max`, `z_score`, `outlier_detection`, and `quantile_clip`.
- Implemented locally through `[[feature_transforms]]`: `mean`, `median`, `mode`, `min`, `max`, `variance`, `standard_deviation`, `skewness`, `kurtosis`, `percentile`, `quantile`, `histogram`, `covariance`, `correlation`, `autocorrelation`, and `cross_correlation`.
- Histogram emits one feature record per bin using `{id}_bin_{index}` IDs and bin method context.
- Correlation lag convention is `channel[t]` compared with `other_channel[t + lag_samples]`; autocorrelation uses the same convention on one channel.

Acceptance criteria:

- Empty, constant, NaN, and non-finite inputs are handled explicitly through structured errors for finite samples, constant skew/correlation inputs, invalid quantiles, invalid histogram ranges, and lag values greater than or equal to sample count.
- Rolling window behavior is deterministic: M32 uses trailing windows with shrinking startup windows.
- Correlation lag conventions are documented in config/report docs and method context.
- Feature records include method context for percentiles, quantiles, histogram bins, comparison channels, and lag samples.
- Validation examples show statistics as `feature_records` while pass/fail remains controlled by criteria/event validations.

## M33: Spectrum, Windows, And Time-Frequency Suite

Goal: provide the offline spectral analysis toolbox engineers expect for sampled signals.

Scope:

- rectangular, Hann, Hamming, Blackman, Blackman-Harris, flat-top, Kaiser, Tukey, Bartlett, and Gaussian windows,
- FFT and IFFT,
- DFT for small/reference cases,
- power spectrum,
- PSD,
- Welch PSD,
- cross spectrum,
- coherence,
- transfer function estimate,
- harmonic analysis,
- THD, SNR, SINAD, ENOB,
- STFT and spectrogram,
- spectral centroid, bandwidth, rolloff, and band power.

Acceptance criteria:

- Dependency review is complete before implementation. Status: Pass; M33 records a no-new-dependency decision in `docs/m33-spectrum-time-frequency-pipeline-report.md`.
- Frequency-bin, scaling, window-normalization, and unit conventions are documented. Status: Pass; see config/report/metadata docs.
- Known-answer sine, square, harmonic, and noise fixtures exist. Status: Pass; see M33 core tests and `examples/m33-spectrum-waveform.csv`.
- Spectral outputs are feature records, not hidden waveform mutations. Status: Pass; M33 uses `[[feature_transforms]]` and `feature_records`.
- Time-frequency outputs are desktop/offline only unless later bounded runtime work proves otherwise. Status: Pass; catalog metadata marks STFT/spectrogram desktop/offline.

## M34: Fault Injection And ADC/DAC Simulation Suite

Goal: let engineers simulate signal-conditioning and DAQ/controller faults without external scripts.

Scope:

- white, Gaussian, uniform, pink, and brown noise,
- impulse and salt-and-pepper noise,
- periodic interference and 50/60 Hz hum,
- ground-bounce and reference shifts,
- thermal drift and random walk drift,
- dropout and missing samples,
- saturation and clipping,
- stuck-at and flatline faults,
- intermittent faults,
- quantization noise,
- rounding/floor/ceil quantizers,
- mid-rise and mid-tread quantizers,
- dithering,
- companding,
- sample clock jitter,
- ADC missing codes,
- INL, DNL, gain error, and offset error.

Acceptance criteria:

- Fault generation is deterministic when seeded. Status: Pass; M34 seeded core tests reapply noise, drift, sample fault, dither, and jitter transforms with identical outputs.
- Randomness policy and seed recording are documented. Status: Pass; M34 config reference, metadata mapping, and transform parameters record seeds and `evidence_scope = simulation_only`.
- Fault-injection transforms are clearly separated from measured signal evidence. Status: Pass; M34 transforms emit derived waveforms while preserving source waveform lineage.
- ADC/DAC simulations avoid hardware accuracy claims. Status: Pass; M34 docs and metadata call the outputs simulation-only and reject runtime/package/hardware claims.
- Validation fixtures prove expected distributions or deterministic seeded outputs. Status: Pass; M34 core/config/catalog/CLI tests and `examples/m34-fault-adc-*` fixture cover deterministic and known-output behavior.

## M35: Multi-Channel, Sensor, And Domain Conditioning Packs

Goal: cover common multi-channel and sensor-specific workflows in one place.

Scope:

- Implemented locally: channel addition/subtraction, differential channel, and common-mode derived channels.
- Implemented locally: vector magnitude, Euclidean norm, matrix transform, and two-axis coordinate rotation.
- Already covered in M30: cross-correlation delay alignment.
- Dependency/design-gated: phase difference and gain/phase matching, because they need estimator/window/unwrap conventions and calibration semantics before support claims.
- Implemented locally as software formulas: linear sensor conversion, pressure transducer conversion, current shunt conversion, bridge strain conversion, load-cell force conversion, RTD temperature, thermistor temperature, tachometer RPM, encoder position, accelerometer units, gyroscope rate, Hall current, LVDT position, microphone SPL, and photodiode optical power.
- Dependency/design-gated: thermocouple cold-junction compensation, advanced acoustic features, and advanced sensor calibration packs.
- Implemented locally: velocity from acceleration, displacement from velocity, vibration severity, control error, proportional control, PID control, rate limiter, slew-rate limit, control saturation, control deadzone, and feedforward control.

Acceptance criteria:

- Sensor transforms are separated into domain packs with assumptions and reference formulas. Status: Pass; M35 config/catalog/docs split software sensor formulas from dependency-gated advanced calibration packs.
- Calibration transforms are not represented as hardware calibration evidence without separate evidence. Status: Pass; M35 metadata and docs describe them as software-only conditioning and declare no hardware calibration, qualification, certification, DAQ, HAL/RTOS, or package-runtime support.
- Multi-channel transforms validate channel units and alignment. Status: Pass; M35 core/config tests cover missing channels, mismatched units, duplicate outputs, matrix shape, and explicit output-channel declarations.
- Domain packs can be enabled incrementally without blocking the core transform suite. Status: Pass; 34 M35 `[[filters]]` are implemented without a new dependency while `phase_difference`, `gain_phase_match`, `advanced_acoustic_pack`, and `advanced_sensor_calibration_pack` remain dependency/design-gated catalog entries.

## M36: Completeness, UX, And Compatibility Closure

Goal: close the comprehensive suite with documentation, examples, tests, and compatibility gates that make it usable by engineers.

Required artifacts:

- Transform catalog reference.
- Config examples by workflow.
- Known-answer validation corpus index by transform family.
- Negative-case matrix.
- Benchmarks for large waveform conditioning.
- Report/schema compatibility review.
- Rule-package compatibility review.
- Runtime-profile compatibility review.
- Dependency/security review update.
- README and config-reference refresh.
- Release readiness, community messaging, and retrospective updates.

Acceptance criteria:

- Every transform in M25-M35 is either implemented with evidence or explicitly marked future-gated. Status: Pass; the catalog reports 219 entries, including M25-M35 implemented coverage, dependency-gated advanced entries, `split_by_event` as a future-gated multi-artifact segmentation entry, and `comprehensive_suite_closure` as an implemented M36 registry artifact.
- Engineers can search docs by transform name and see config, parameters, units, examples, runtime support, package support, and evidence level. Status: Pass; `docs/config-reference.md`, `docs/transform-catalog.md`, `docs/current-transform-metadata-mapping.md`, `docs/validation-corpus-index.md`, and `ferrisoxide-signal transforms --format text/json` provide the search surface.
- Full workspace validation passes. Status: Pass; `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`, `git diff --check`, trailing-whitespace scans, Markdown link scans, catalog CLI tests, package rejection tests, M35 direct fixture analysis, and benchmark helper execution are recorded in `docs/validation-log.md`.
- Unsupported runtime/package paths fail clearly. Status: Pass; package export still supports only the current approved subset and rejects desktop-only transform families through the catalog-driven export guardrail.
- No hardware, DAQ, RTOS, or certification claims are introduced without separate gates. Status: Pass; release, community, risk, and retrospective artifacts keep live DAQ, HAL/RTOS, target hardware, runtime-loader implementation, signing, hardware calibration, hardware qualification, and certification evidence out of scope.

## Recommended Issue Grouping

Each milestone should be broken into small issue groups:

| Issue Type | Purpose |
|---|---|
| Architecture | Transform contract, category, metadata, API, compatibility decision. |
| Implementation | Core algorithm, config, report integration, docs mapping. |
| Known-answer fixtures | Independent expected values and deterministic datasets. |
| Negative cases | Invalid parameters, invalid units, invalid timing, non-finite values, unsupported runtime/package requests. |
| Documentation | Equations, assumptions, examples, non-goals, config reference. |
| Package/runtime gate | Whether the transform can enter rule packages or runtime profiles. |

## Dependency Policy

M25-M35 start dependency-free where practical. M34 records a no-new-RNG/noise-dependency decision, and M35 records a no-new-domain/calibration-dependency decision for the implemented software formula subset.

M29, M30, M33, and advanced M35 work may require dependency review for exact advanced designs. M29 keeps exact elliptic/Cauer design dependency-gated, M30 keeps efficient polyphase resampling dependency/performance-gated, M31 keeps Hilbert envelope dependency/design-gated, M33 keeps optimized FFT dependency/performance follow-up gated, and M35 keeps phase-difference estimation, gain/phase matching, advanced acoustic features, and advanced sensor calibration packs gated. Before adding dependencies, the project must record:

- why local implementation is insufficient,
- license and supply-chain review,
- no_std impact,
- reproducibility impact,
- benchmark impact,
- alternatives considered,
- rollback plan.

## Stop Conditions

Stop before:

- adding new dependencies without approval,
- claiming every taxonomy item is implemented,
- exposing a transform in rule packages without compatibility tests,
- exposing a transform to no_std/runtime profiles without bounded-buffer evidence,
- adding live DAQ, vendor SDKs, HAL/RTOS SDKs, target hardware, unsafe FFI, signing, or certification evidence,
- turning random fault injection into nondeterministic validation evidence,
- allowing transforms to silently mutate raw data.

## Hand-Off Note

Role: Product Architect / Software Architect
Goal: Define the post-MVP milestone path for a comprehensive FerrisOxide filter and signal-conditioning suite.
Files changed: `docs/comprehensive-filter-signal-conditioning-roadmap.md`, `docs/post-mvp-roadmap.md`, `docs/next-milestones-roadmap.md`, `requirements.md`, `traceability-matrix.md`, `risk-register.md`, `project-state.md`, `orchestration-plan.md`, `README.md`, `CHANGELOG.md`, `docs/validation-log.md`, and root studio memory files.
Checks run: `cargo fmt --check`; `git diff --check`; trailing-whitespace scans; local Markdown link-target scans.
Status: M25, M26, M27, M28, M29, M30, M31, M32, M33, M34, M35, and M36 complete and merged to `main` through PR #175.
Known gaps: Release publication remains separately gated. Exact elliptic/Cauer design, efficient polyphase resampling, Hilbert envelope, optimized FFT dependency/performance work, phase-difference estimation, gain/phase matching, large spectral-output UX, advanced acoustic/domain packs, advanced sensor calibration packs, and `split_by_event` multi-artifact segmentation remain separately gated follow-ups.
Next recommended step: Choose one gated advanced follow-up or a separate release-publication plan.
