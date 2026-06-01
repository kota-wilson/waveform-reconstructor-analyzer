# Analog Signal Transform Taxonomy

Date: 2026-06-01

Status: Planning input for future FerrisOxide milestones. This document does not claim current implementation support for every transform named here.

## Purpose

FerrisOxide works on real-world analog signal data after DAQ sampling. The software can simulate, filter, classify, and validate that sampled data digitally, but it must preserve the difference between raw input, derived transforms, measured features, and pass/fail validation evidence.

The immediate architecture implication is that FerrisOxide should model transforms more broadly than "filters." A filter is one transform family. Calibration, event extraction, feature extraction, test validation, resampling, and offline spectral analysis need their own capabilities and runtime constraints.

The first capability vocabulary and support-status matrix for this taxonomy lives in `docs/transform-capability-model.md`.

This document is not the implementation support matrix. Current support is limited to the implemented baseline below unless another document links a transform family to an implemented requirement, test, and release artifact.

## Current Implemented Baseline

FerrisOxide already has a small subset of this taxonomy:

| Area | Current support |
|---|---|
| Waveform model | Time axis, named channels, units, metadata, raw/derived lineage, transform history. |
| Basic filters/transforms | Moving average, first-order low-pass, ideal ADC quantization. |
| Measurements/features | Extrema, transition count, state-run duration, transient duration, pulse width, rise/fall time evidence. |
| Criteria/validation | Range checks, state transition checks, response latency, stable state, transient event, pulse width, rise/fall time. |
| Visualization | Desktop SVG plots and evidence overlays. |
| Controller workflow | Production control config schema, test verification config schema, fixture-driven desktop simulation, deployment manifest, qualification evidence report. |
| Embedded boundary | `no_std` signal, measurement, rule-engine, and embedded adapter crates with parity tests. |

## Transform Families

Future transform work should use these families rather than expanding one monolithic filter list.

| Family | Examples | FerrisOxide role |
|---|---|---|
| `PointwiseTransform` | Offset, gain, inversion, absolute value, clamp, normalize, unit conversion, deadband, span correction, linearization. | Per-sample derived channels and engineering-unit conversion. |
| `WindowedTransform` | Moving average, weighted average, moving median, Gaussian smoothing, Savitzky-Golay smoothing, rolling statistics. | Local smoothing and robust preprocessing. |
| `StatefulTransform` | Exponential moving average, hysteretic smoothing, hysteresis, debounce, latch, pulse stretching, leaky integrator. | Streaming-compatible transforms with memory. |
| `FrequencyFilterTransform` | Low-pass, high-pass, band-pass, band-stop, notch, comb, all-pass, shelving, peaking EQ. | Conventional filter behavior with explicit phase and sample-rate assumptions. |
| `FilterDesignTransform` | FIR, IIR, Butterworth, Chebyshev, elliptic, Bessel, Gaussian, Kalman, adaptive, Wiener, matched filter. | Future design families; each requires validation evidence before product claims. |
| `TimingTransform` | Downsample, decimate, upsample, interpolate, resample, delay, fractional delay, time alignment, jitter correction, clock-drift correction. | Sample-grid and channel-timing management. |
| `WindowFunctionTransform` | Rectangular, Hann, Hamming, Blackman, flat-top, Kaiser, Tukey, Bartlett, Gaussian. | Preprocessing for FFT and spectral workflows. |
| `FrequencyTransform` | FFT, IFFT, power spectrum, PSD, Welch PSD, cross spectrum, coherence, transfer function estimate, harmonic analysis, THD, SNR, SINAD, ENOB. | Offline spectral analysis and evidence generation. |
| `TimeFrequencyTransform` | STFT, spectrogram, wavelet transforms, wavelet denoising, Hilbert-Huang, empirical mode decomposition. | Offline transient and nonstationary signal analysis. |
| `EnvelopeEnergyTransform` | Rectification, envelope detection, Hilbert envelope, RMS, moving RMS, peak hold, crest factor, energy, power, amplitude demodulation. | Amplitude, energy, and modulation evidence. |
| `CalculusTransform` | First derivative, second derivative, numerical integration, cumulative integral, slope detection, area under curve, impulse estimate. | Rate, accumulation, and physical-domain conversion. |
| `EventTransform` | Threshold, dual threshold, Schmitt trigger, debounce, glitch removal, edge detection, zero crossing, state machine, dwell-time, timeout. | Analog-to-state interpretation and controller/test emulation. |
| `PulseEventTransform` | Pulse detection, pulse width, duty cycle, period, frequency estimate, rise/fall time, settling time, overshoot, ringing, bounce, missing/extra pulse. | Switch, PWM, actuator, and transient event analysis. |
| `StatisticalTransform` | Mean, median, min/max, standard deviation, variance, percentiles, histogram, z-score, covariance, correlation, autocorrelation, cross-correlation. | Summary evidence and distribution-aware checks. |
| `FaultInjectionTransform` | White/Gaussian/uniform/pink noise, quantization noise, impulse noise, hum, drift, dropout, saturation, stuck-at, flatline, intermittent faults. | Synthetic test fixtures and fault-injection scenarios. |
| `QuantizationTransform` | ADC code conversion, resolution, saturating quantizer, rounding/floor/ceil quantizer, mid-rise, mid-tread, dithering, companding, missing code, INL, DNL, gain/offset error. | DAQ/controller emulation and ADC defect simulation. |
| `ModulationTransform` | AM/FM/PM modulation and demodulation, IQ, mixing, heterodyning, synchronous detection, lock-in, phase detector, PLL-like tracking, Hilbert analytic signal. | Specialized instrumentation and signal-recovery workflows. |
| `MultiChannelTransform` | Channel subtraction/addition, common-mode rejection, vector magnitude, sensor fusion, cross-correlation delay, phase difference, matrix transform, PCA/ICA, coordinate transform. | Differential measurement, multi-axis sensors, redundant sensors, and channel alignment. |
| `CalibrationTransform` | Thermocouple CJC, RTD conversion, thermistor conversion, strain bridge conversion, load-cell mV/V, pressure conversion, accelerometer/gyro bias, encoder/tachometer conversion. | Sensor-specific engineering-unit conversion. |
| `VibrationAcousticTransform` | A/C weighting, octave bands, order tracking, cepstrum, envelope spectrum, shock response spectrum, PSD integration, vibration severity metrics. | Future vibration/acoustic analysis; likely offline desktop first. |
| `ControlTransform` | Error signal, P/I/D terms, PID, saturation, rate limiter, slew limit, deadzone, anti-windup, feedforward, lead/lag/notch compensation, observer, low-pass derivative. | Controller simulation and deployment planning. |
| `NonlinearTransform` | Clipping, soft clipping, saturation, deadzone, hysteresis, compression, expansion, comparator, relay, lookup table, piecewise linear, polynomial, sigmoid, tanh. | Physical constraints, calibration curves, and controller emulation. |
| `DataCleaningTransform` | NaN handling, gap filling, spike removal, Hampel filter, duplicate timestamp removal, time sorting, cropping, trimming, split by events, resynchronization, schema normalization. | DAQ export cleanup and fixture preparation. |
| `FeatureTransform` | RMS, peak-to-peak, rise/fall time, settling time, overshoot, area under curve, dominant frequency, band power, entropy, crest factor, duty cycle, event count, time-in-state. | Analysis evidence separated from pass/fail decisions. |
| `ValidationTransform` | Pass/fail threshold, tolerance band, stability window, activation window, no-false-trigger, sequence validation, timing validation, state coverage, transition count, noise margin, worst-case margin. | FerrisOxide's core product differentiator: evidence-backed test validation. |

## Required Transform Metadata

Every transform should declare enough metadata for desktop, embedded, and review workflows to decide whether it is valid in a given context.

| Field | Purpose |
|---|---|
| `name` | Stable transform identifier. |
| `category` | One of the transform families above. |
| `input_channels` | Required source channels or channel groups. |
| `output_channels` | Derived channels, feature records, events, or validation records produced. |
| `parameters` | Typed parameters with units, defaults, and allowed ranges. |
| `sample_rate_required` | Whether uniform sample rate or known sample timing is required. |
| `stateful` | Whether previous samples or internal state affect current output. |
| `causal` | Whether output at sample N depends only on samples up to N. |
| `phase_effect` | `none`, `delay`, `nonlinear`, or `zero_phase`. |
| `streaming_supported` | Whether the transform can run on bounded buffers or sample streams. |
| `offline_only` | Whether the transform requires full-record access or forward/backward passes. |
| `raw_data_preserved` | Whether the transform creates derived data instead of mutating source data. |
| `runtime_profile` | Desktop-only, embedded-compatible, Pico-compatible, or future-gated. |
| `evidence_level` | Fixture-only, unit-tested, parity-tested, validated, or future research. |

## Runtime Boundary Rules

| Runtime | Supported direction | Exclusions |
|---|---|---|
| Desktop offline analysis | Full-record transforms, plotting, reports, FFT/spectral workflows, zero-phase filtering, validation reports, and package export. | Must not imply hardware qualification or certification without separate evidence. |
| Raspberry Pi 5 bare-metal ARM64 | `no_std`, deterministic, bounded-buffer transforms, shared rule semantics, compact deployment artifacts. | No CSV parsing, desktop plotting, heap-heavy spectral workflows, vendor SDK assumptions, or certification claims by default. |
| Pico 2 optional micro-runtime | Compact threshold/timing checks, simple pointwise transforms, simple filters, fixed buffers, GPIO/PWM-style outputs. | No desktop-equivalent runtime, no large rule packages, no spectral/time-frequency analysis, no production readiness claim until separately gated. |

## Suggested Milestone Sequencing

### Transform Architecture Milestone

Goal: define the transform registry and metadata contract before adding many algorithms.

Candidate issues:

1. Add a transform capability matrix doc and schema vocabulary.
2. Extend derived waveform metadata to record transform category, causality, phase effect, and runtime profile.
3. Rename user-facing docs from "filters only" to "transforms and filters" while preserving compatibility.
4. Add validation tests that reject unsupported transform/runtime combinations in package/profile metadata.

### Pointwise And Windowed MVP Milestone

Goal: implement low-risk transforms that improve DAQ/test workflows without new dependencies.

Candidate transforms:

- Offset / bias add.
- Gain / scale.
- Inversion.
- Clamp.
- Deadband.
- DC removal.
- Baseline subtraction.
- Moving median.
- High-pass first-order baseline correction.

Candidate evidence:

- Exact unit tests over small fixtures.
- Golden report tests proving raw data preservation.
- Metadata tests proving transform history and phase/runtime fields.

### Event And Validation Milestone

Goal: strengthen snap-action switch and controller-test workflows.

Candidate transforms:

- Dual threshold / Schmitt trigger.
- Debounce.
- Glitch removal.
- Latch.
- Edge event extraction.
- Bounce detection.
- Missing/extra pulse detection.
- Dwell-time and timeout validation.

Candidate evidence:

- Known-answer switch/bounce fixtures.
- Desktop-vs-embedded-compatible parity checks where practical.
- Qualification evidence report examples with explicit non-certification notes.

### Future-Gated Milestones

These require separate dependency, architecture, validation, and scope gates:

- Advanced FIR/IIR design families.
- FFT, PSD, STFT, wavelets, and spectral metrics.
- Resampling and jitter/clock-drift correction.
- Sensor-specific calibration packages.
- Vibration/acoustic workflows.
- Live DAQ SDK integrations.
- Hardware HAL/RTOS adapters.
- Cryptographic signing or certified evidence workflows.

## Design Notes

- Preserve raw waveform input data. Transforms produce derived channels, feature records, event records, or validation records.
- Keep feature extraction separate from validation. A feature computes evidence; a validation transform decides pass/fail against a requirement.
- Avoid adding dependencies for simple pointwise, windowed, event, or validation transforms.
- Treat zero-phase filters, FFT workflows, and time-frequency analysis as offline-only unless a later architecture proves a bounded streaming equivalent.
- Keep desktop and embedded rule behavior from drifting by routing shared semantics through schema, rule-engine, and parity tests where practical.
- Do not expose a transform in a deployment package unless its runtime profile and evidence level are explicit.

## Hand-Off Note

Role: Software Architect / Product Architect
Goal: Capture the analog signal transform taxonomy as a planning input for future FerrisOxide milestones.
Files changed: `docs/analog-transform-taxonomy.md`
Checks run: Documentation-only change; no code checks required.
Status: Planning artifact created; M10-001 later added `docs/transform-capability-model.md` as the capability vocabulary.
Known gaps: Most taxonomy entries remain future-gated and are not implemented support.
Next recommended step: Use the capability model before exposing new transforms in config, reports, rule packages, or deployment packages.
