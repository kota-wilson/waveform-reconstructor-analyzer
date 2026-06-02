# Changelog

All notable changes to this project will be documented here.

The format follows Keep a Changelog principles, and the project intends to use semantic versioning once public releases begin.

## Unreleased

### Changed

- Adopted FerrisOxide Signal as the in-repository product identity, including `ferrisoxide-*` workspace packages and the `ferrisoxide-signal` CLI binary.
- Expanded the main README into a complete product guide covering workflow, examples, repo layout, commands, reports, plotting, rule packages, embedded boundaries, validation assets, and contribution expectations.

### Added

- Initial repository skeleton.
- Project charter, requirements, risk register, architecture, and traceability matrix.
- Rust workspace with core library and CLI analysis path.
- Simple CSV waveform parsing with named time and channel columns.
- Moving-average and first-order low-pass filters.
- Min/max voltage criteria evaluation and text report rendering.
- TOML config parsing for input mapping, filters, and criteria.
- JSON report rendering for automation use.
- GitHub issue templates, PR template, and CI workflow.
- Real waveform fixtures for clean/noisy square waves, analog switch bounce, dropout, slow rise/fall, and multi-channel data.
- Waveform criteria for state transitions, pulse width, transient duration, transient event detection, stable-state duration, and rise/fall time.
- Report evidence fields for failed criterion, measured/required values, sample index, timestamp, and channel.
- Golden JSON report tests and invalid config validation tests.
- `ferrisoxide-signal` `no_std` crate with fixed-size buffers, streaming threshold checks, and transient event detection for future embedded adapters.
- Simulated ADC quantization as an ordered pre-criteria waveform transform for CLI and TOML workflows.
- Waveform metadata for source, units, sample interval, sample rate, lineage, and transform history in text and JSON reports.
- v0.3.0 validation roadmap and validation dataset folder structure.
- Known-answer validation fixtures, environmental dropout/contact-bounce examples, and exact expected validation report tests.
- Configurable voltage/time tolerance policy with report evidence.
- Report evidence context fields for validation profile, evidence source, tolerance policy, and confidence notes.
- Optional validation metadata context for test-run ID, acquisition notes, environment, and operator.
- Time-axis validation for duration criteria plus sample interval and nominal sample-rate documentation.
- Filter behavior documentation with moving-average, first-order low-pass, and ideal ADC quantization equations.
- Project-local large-CSV benchmark helper and baseline benchmark documentation.
- Optional desktop SVG plotting with 2D waveform plots and 3D line plots using a configured third-axis column.
- `ferrisoxide-embedded` `no_std` adapter boundary plus ARM64 QEMU and Zephyr feasibility prototype artifacts.
- `ferrisoxide-measurements` `no_std` crate with reusable extrema, state-transition, state-run duration, and rise/fall measurement primitives used by criteria evidence.
- `ferrisoxide-control-schema` crate with production control config schema types, validation helpers, and a parse-tested example config for future controller-in-the-loop workflows.
- `ferrisoxide-verification-schema` crate with test verification config schema types, validation helpers, and a parse-tested example config for future qualification and controller-in-the-loop workflows.
- `ferrisoxide-simulator` crate with a deterministic virtual controller simulation engine over production control configs and abstract sample frames.
- `ferrisoxide-daq` crate with fixture/test-double DAQ sample-source abstractions for deterministic controller-in-the-loop input.
- `ferrisoxide-controller-io` crate with host-checkable controller input/output traits and fake I/O for portable controller boundaries.
- `simulate` CLI workflow that loads production control config, test verification config, a channel map, and fixture CSV input to produce simulation trace plus verification evidence.
- `ferrisoxide-deployment` crate with RTOS/controller deployment package manifest schema, required artifact roles, validation helpers, checksum drift-detection wording, and a heated-actuator package fixture.
- Deployment manifest mode profiles that separate `production_control`, `test_verification`, and `signal_validation` purposes and reject mixed production/test artifact combinations.
- Controller config parity test comparing desktop simulation state/evidence with embedded-compatible borrowed-rule evidence over the same configs, channel map, and waveform input.
- Qualification evidence report schema and exact JSON fixture linking config versions, channel map, simulation trace, criteria evidence, deployment metadata, checksum evidence, generated timestamp, and non-certification scope notes.
- Config reference, artifact contract, local batch analysis workflow, transform-package compatibility matrix, validation corpus index, MVP-exit readiness report, and post-MVP roadmap for the local M15-M20 MVP-exit pass.
- `batch` CLI workflow for local repeated CSV/config analysis with per-run reports, deterministic summary JSON, partial-failure handling, overwrite protection, and manifest validation.
- Portable rule-package schema/export support for `offset`, `gain`, and `invert` as linear pointwise software transforms, with unsupported nonlinear/baseline/windowed transforms still rejected.
- Shared borrowed-slice runtime-compatible `offset`, `gain`, and `invert` semantics in `ferrisoxide-rule-engine`, plus desktop-vs-runtime parity coverage.
- Positive and negative rule-package compatibility fixtures for the linear pointwise subset and unsupported `clamp` transform.
- M24 runtime-loader design gate documenting accepted subset, memory constraints, failure modes, checksum scope, target checks, and implementation stop condition.
- M25-M36 comprehensive filter and simulated signal-conditioning roadmap covering transform registry, data cleaning, nonlinear conditioning, smoothing, filters, resampling, calculations, statistics, spectral analysis, fault injection, ADC/DAC simulation, multi-channel/sensor packs, and completion gates.
- M25 transform catalog and completeness contract with a core source-of-truth registry, `transforms` CLI output, metadata/catalog tests, and catalog-driven rule-package export support checks.
- M26 desktop data-cleaning and timing-conditioning filters: `timestamp_sort`, `dedupe_timestamps`, `nan_interpolate`, `nan_remove`, `crop`, `fixed_delay`, `gap_fill`, `resample_fixed`, and `channel_delay`, with config/CLI examples, catalog metadata tests, and rule-package export rejection coverage.
- M27 desktop pointwise, normalization, and nonlinear conditioning filters: `absolute_value`, `square`, `square_root`, `log`, `exp`, `normalize`, `tanh`, `sigmoid`, `soft_limit`, `piecewise_linear`, and `polynomial`, with config/CLI examples, catalog metadata tests, formula/domain tests, and rule-package export rejection coverage.
- M28 desktop smoothing, detrending, baseline, Hampel, and spike-cleanup filters: `weighted_moving_average`, `exponential_moving_average`, `boxcar_smoothing`, `gaussian_smoothing`, `savitzky_golay`, `centered_moving_median`, `rolling_mean_baseline`, `rolling_median_baseline`, `linear_detrend`, `polynomial_detrend`, `hampel_filter`, and `spike_remove`, with config/CLI examples, catalog metadata tests, edge/drift/spike tests, and rule-package export rejection coverage.
- M29 desktop standard frequency filters: `fir_filter`, `zero_phase_fir_filter`, `iir_biquad`, `zero_phase_iir_biquad`, `high_pass`, `band_pass`, `band_stop`, `notch`, `comb_filter`, `butterworth_low_pass`, `butterworth_high_pass`, `chebyshev1_low_pass`, `chebyshev2_low_pass`, and `bessel_low_pass`, with config/CLI examples, catalog metadata tests, generated frequency-response tests, stability/sample-rate validation, and rule-package export rejection coverage.
- M30 desktop resampling and timing-alignment filters: `resample`, `downsample`, `decimate`, `upsample`, `interpolate`, `rational_resample`, `sample_and_hold`, `zero_order_hold`, `first_order_hold`, `fractional_delay`, `cross_correlation_delay`, `jitter_correction`, and `clock_drift_correction`, with config/CLI examples, catalog metadata tests, anti-alias/timing validation, alignment confidence metadata, and rule-package export rejection coverage.
- M31 desktop envelope, energy, and calculus support: `half_wave_rectify`, `full_wave_rectify`, `envelope`, `moving_rms`, `peak_hold`, `first_derivative`, `second_derivative`, `integral`, `cumulative_integral`, `leaky_integrator`, and `slope_detection` filters plus `feature_records` for RMS, peak-to-peak, crest factor, energy, power, area, and impulse estimate, with config/CLI examples, report schema docs, catalog metadata tests, unit evidence, invalid-input checks, and package-export rejection coverage.
- M32 desktop statistics and correlation support: rolling statistics, z-score, outlier detection, quantile clipping, scalar statistics, histogram bin records, covariance, correlation, autocorrelation, and cross-correlation, with method context, config/CLI examples, catalog metadata tests, invalid-input checks, and package-export rejection coverage.
- M33 desktop spectrum, window, and time-frequency support: window coefficient records, DFT, FFT, IFFT, power spectrum, PSD, Welch PSD, cross-spectrum, coherence, transfer estimates, harmonic analysis, THD, SNR, SINAD, ENOB, STFT, spectrogram, spectral centroid, bandwidth, rolloff, and band power, with dependency-free spectral routines, sine/square/harmonic/noise fixtures, config/CLI examples, report schema docs, catalog metadata tests, invalid-input checks, and validation evidence.
- M34 desktop deterministic fault-injection and ADC/DAC simulation support: seeded white/Gaussian/uniform/pink/brown noise, impulse/salt-pepper/quantization noise, periodic/hum interference, ground-bounce/thermal/random-walk drift, dropout/missing/saturation/stuck-at/flatline/intermittent faults, quantizer variants, dithering, companding, sample-clock jitter, missing-code/INL/DNL/gain/offset simulation, simulation-only metadata, config/CLI examples, catalog metadata tests, invalid-input checks, and package-export rejection coverage.
- M35 desktop multi-channel, sensor, vibration, and control conditioning support: channel arithmetic, differential/common-mode, vector/euclidean norm, matrix transform, coordinate rotation, software sensor conversions, vibration integration/severity, and control transforms, with explicit output channels, output units, config/CLI examples, catalog metadata tests, invalid-input checks, and package-export rejection coverage; phase-difference, gain/phase matching, advanced acoustic features, and advanced calibration packs remain dependency/design-gated.
- M36 comprehensive-suite closure: catalog, CLI catalog output, config reference, validation corpus index, package/runtime compatibility map, benchmark-readiness evidence, release readiness, community messaging, retrospective, README, roadmap, state, requirements, traceability, validation log, stale-reference closure, and PR #175 mainline merge evidence for the M25-M36 comprehensive filter and simulated signal-conditioning suite, without dependency, release, hardware, runtime-loader, or certification scope expansion.
