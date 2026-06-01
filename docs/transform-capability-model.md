# Transform Capability Model

Date: 2026-06-01

Status: M10-001 / issue #132 architecture artifact. This document defines vocabulary and capability boundaries; it does not implement new transform algorithms.

## Purpose

FerrisOxide is moving from a small filter-oriented surface toward a broader transform architecture. The capability model prevents a taxonomy entry from being mistaken for implemented product support.

Every transform family, transform implementation, and future runtime exposure should be described with stable vocabulary before it appears in config, reports, rule packages, or deployment packages.

## Scope

In scope for this document:

- Transform category names.
- Transform capability metadata fields.
- Runtime profile names and meanings.
- Capability status and evidence-level vocabulary.
- A capability matrix separating implemented behavior, proposed M12 work, and future-gated work.
- Code-design vocabulary for later implementation.

Out of scope for this document:

- New transform algorithms.
- Config schema changes.
- Report schema changes.
- Runtime profile validation code.
- Deployment package exposure.
- Live DAQ, HAL, RTOS SDK, target hardware, real-time, hardware qualification, or certification claims.

## Category Vocabulary

Category names are stable PascalCase strings for docs and eventual code enums. They describe the transform's role, not its implementation status.

| Category | Scope Boundary |
|---|---|
| `PointwiseTransform` | Per-sample operations that do not require neighboring samples. |
| `WindowedTransform` | Operations over a local sample window. |
| `StatefulTransform` | Operations whose output depends on previous samples or internal state. |
| `FrequencyFilterTransform` | Time-domain filters that select or reject frequency bands. |
| `FilterDesignTransform` | Named filter design families or adaptive estimators that need separate validation. |
| `TimingTransform` | Sample-grid, delay, alignment, resampling, or clock-correction operations. |
| `WindowFunctionTransform` | FFT/spectral preprocessing windows. |
| `FrequencyTransform` | Frequency-domain analysis and metrics. |
| `TimeFrequencyTransform` | Time-varying frequency analysis. |
| `EnvelopeEnergyTransform` | Amplitude, envelope, RMS, energy, and power operations. |
| `CalculusTransform` | Differentiation, integration, slope, and area operations. |
| `EventTransform` | Analog-to-state, edge, latch, debounce, and event extraction operations. |
| `PulseEventTransform` | Pulse, duty-cycle, period, bounce, settling, and ringing operations. |
| `StatisticalTransform` | Summary, distribution, correlation, and rolling-statistic operations. |
| `FaultInjectionTransform` | Synthetic noise, drift, dropout, saturation, stuck-at, and fault scenarios. |
| `QuantizationTransform` | ADC/DAC quantization, code conversion, companding, dithering, and ADC defect simulation. |
| `ModulationTransform` | Modulation, demodulation, mixing, synchronous detection, and phase tracking. |
| `MultiChannelTransform` | Operations that combine, compare, align, or decompose multiple channels. |
| `CalibrationTransform` | Sensor-specific conversion or correction into engineering units. |
| `VibrationAcousticTransform` | Acoustic, vibration, rotating-machinery, and shock-analysis operations. |
| `ControlTransform` | Controller, compensation, actuator limit, and control-loop operations. |
| `NonlinearTransform` | Saturation, deadzone, hysteresis, compression, lookup, and nonlinear mapping. |
| `DataCleaningTransform` | Invalid sample handling, cropping, gap repair, timestamp repair, and schema normalization. |
| `FeatureTransform` | Evidence-producing feature extraction that does not itself decide pass/fail. |
| `ValidationTransform` | Requirement checks that decide pass/fail against measured or event evidence. |

## Capability Metadata Fields

These fields define the minimum vocabulary for a transform capability record.

| Field | Required | Meaning |
|---|---|---|
| `name` | Yes | Stable snake_case transform identifier used in config/docs/report mappings. |
| `category` | Yes | One category from the category vocabulary. |
| `input_channels` | Yes | Channel cardinality and constraints, such as one channel, channel pair, or channel set. |
| `output_channels` | Yes | Derived channel, event record, feature record, validation record, or in-place-like derived output behavior. |
| `parameters` | Yes | Named parameters with type, unit, default, allowed range, and validation rule. |
| `sample_rate_required` | Yes | Whether valid sample timing or uniform sample rate is required. |
| `stateful` | Yes | Whether previous samples or internal state affect output. |
| `causal` | Yes | Whether output at sample N depends only on samples up to N. |
| `phase_effect` | Yes | One of `none`, `delay`, `nonlinear`, `zero_phase`, or `not_applicable`. |
| `streaming_supported` | Yes | Whether bounded streaming execution is possible. |
| `offline_only` | Yes | Whether full-record access or forward/backward passes are required. |
| `runtime_profiles` | Yes | Supported or candidate runtime profiles. |
| `capability_status` | Yes | Current support status: implemented, planned, research, or gated. |
| `evidence_level` | Yes | Strongest available evidence level. |

## Runtime Profiles

Runtime profiles identify where a transform may be exposed. They are not hardware validation claims.

| Runtime Profile | Meaning | Explicit Non-Claims |
|---|---|---|
| `desktop` | `std` desktop authoring and offline analysis on the supported workstation profile. | Does not imply live DAQ, hardware qualification, or production controller support. |
| `pi5_no_std_candidate` | Candidate for Raspberry Pi 5 bare-metal ARM64 after no_std, bounded-buffer, and parity evidence exist. | Does not imply Raspberry Pi boot evidence, HAL support, RTOS production readiness, or timing guarantees. |
| `pico2_candidate` | Candidate for the optional Pico 2 micro-runtime subset after fixed-buffer and compact-rule evidence exist. | Does not imply desktop-equivalent support, large waveform support, HAL support, or production readiness. |
| `future_gated` | Requires a later architecture, dependency, environment, hardware, or validation gate before exposure. | Does not imply support in current code or docs beyond planning. |

## Capability Status

Capability status answers whether FerrisOxide supports a transform now or only recognizes it as a future planning item.

| Status | Meaning |
|---|---|
| `implemented` | Implemented in the repository with tests and docs for its current behavior. |
| `planned` | Accepted into a proposed or open milestone but not implemented yet. |
| `research` | Taxonomy item retained for future analysis; no milestone is committed. |
| `dependency_gated` | Requires dependency review before implementation. |
| `hardware_gated` | Requires live hardware, HAL, SDK, or environment approval before implementation or validation. |
| `certification_gated` | Requires separate certification or regulatory evidence planning before claims. |

## Evidence Levels

Evidence level answers how strongly the current behavior has been proven.

| Evidence Level | Meaning |
|---|---|
| `documented_only` | Documented as taxonomy or design direction only. |
| `unit_tested` | Covered by focused unit tests. |
| `fixture_tested` | Covered by deterministic signal fixtures. |
| `golden_report_tested` | Covered by exact report or artifact comparisons. |
| `parity_tested` | Covered across desktop and embedded-compatible paths where applicable. |
| `validated` | Covered by the project's V&V artifacts for the stated software scope. |

## Initial Capability Matrix

This matrix records M10-001 vocabulary boundaries. Later issues own implementation, metadata wiring, validation rules, and tests.

| Capability Area | Examples | Category | Status | Evidence Level | Runtime Profiles |
|---|---|---|---|---|---|
| Moving average | `moving_average` | `WindowedTransform` | `implemented` | `unit_tested`, `fixture_tested`, `golden_report_tested` | `desktop` |
| First-order low-pass | `low_pass` | `FrequencyFilterTransform` | `implemented` | `unit_tested`, `fixture_tested`, `golden_report_tested` | `desktop` |
| Ideal ADC quantization | `adc_quantize` | `QuantizationTransform` | `implemented` | `unit_tested`, `fixture_tested`, `golden_report_tested` | `desktop` |
| Existing measurement evidence | extrema, transitions, pulse width, rise/fall time | `FeatureTransform` | `implemented` | `unit_tested`, `fixture_tested`, `golden_report_tested`, `parity_tested` where rule-engine paths apply | `desktop`, `pi5_no_std_candidate` where no_std primitives already exist |
| Existing criteria evidence | voltage ranges, response latency, stable state, transient event | `ValidationTransform` | `implemented` | `unit_tested`, `fixture_tested`, `golden_report_tested`, `parity_tested` where rule-engine paths apply | `desktop`, `pi5_no_std_candidate` where no_std rule-engine paths apply |
| Pointwise MVP | `offset`, `gain`, `invert`, `clamp`, `deadband` | `PointwiseTransform` | `implemented` | `unit_tested`, `fixture_tested`, `golden_report_tested` | `desktop`; embedded candidacy requires later evidence |
| Baseline MVP | `dc_remove`, `baseline_subtract` | `BaselineTransform` | `implemented` | `unit_tested`, `fixture_tested`, `golden_report_tested` | `desktop`; embedded candidacy requires later evidence |
| Moving median MVP | `moving_median` | `WindowedTransform` | `implemented` | `unit_tested`, `fixture_tested`, `golden_report_tested` | `desktop`; embedded candidacy requires later evidence |
| High-pass baseline correction | first-order high-pass baseline correction | `StatefulTransform` | `planned` | `documented_only` | `desktop`; deferred from M11 pending separate timing behavior issue |
| Event MVP | Schmitt trigger, debounce, glitch removal, edges, bounce | `EventTransform`, `PulseEventTransform`, `StatefulTransform` | `planned` | `documented_only` | `desktop`; `pi5_no_std_candidate` only after parity evidence |
| Event validation MVP | missing pulse, extra pulse, dwell-time, timeout | `ValidationTransform` | `planned` | `documented_only` | `desktop`; `pi5_no_std_candidate` only after parity evidence |
| Spectral analysis | FFT, PSD, coherence, THD, ENOB | `FrequencyTransform` | `research` | `documented_only` | `future_gated` |
| Time-frequency analysis | STFT, spectrogram, wavelets | `TimeFrequencyTransform` | `research` | `documented_only` | `future_gated` |
| Resampling and timing repair | decimation, interpolation, clock-drift correction | `TimingTransform` | `research` | `documented_only` | `future_gated` |
| Sensor-specific calibration | thermocouple, RTD, strain gauge, load cell, LVDT | `CalibrationTransform` | `research` | `documented_only` | `future_gated` |
| Fault injection | noise, drift, dropout, stuck-at, saturation | `FaultInjectionTransform` | `research` | `documented_only` | `future_gated` |
| Vibration/acoustic analysis | A-weighting, order tracking, cepstrum, shock response spectrum | `VibrationAcousticTransform` | `research` | `documented_only` | `future_gated` |

## Code-Design Vocabulary

M10-001 does not add Rust types. The later M10 issues should use this vocabulary when implementing code.

Expected future model shape:

```rust
pub struct TransformCapability {
    pub name: &'static str,
    pub category: TransformCategory,
    pub input_channels: ChannelRequirement,
    pub output_channels: OutputBehavior,
    pub parameters: &'static [TransformParameter],
    pub sample_rate_required: bool,
    pub stateful: bool,
    pub causal: bool,
    pub phase_effect: PhaseEffect,
    pub streaming_supported: bool,
    pub offline_only: bool,
    pub runtime_profiles: &'static [RuntimeProfile],
    pub capability_status: CapabilityStatus,
    pub evidence_level: EvidenceLevel,
}
```

Likely enum vocabulary:

- `TransformCategory`
- `ChannelRequirement`
- `OutputBehavior`
- `TransformParameter`
- `PhaseEffect`
- `RuntimeProfile`
- `CapabilityStatus`
- `EvidenceLevel`

Issue ownership:

- M10-002 / issue #133 owns structured transform metadata design.
- M10-003 / issue #134 owns mappings for current moving average, low-pass, and ADC quantization behavior.
- M10-004 / issue #135 owns runtime profile compatibility rules.
- M10-005 / issue #136 owns documentation wording updates.
- M10-006 / issue #137 owns compatibility and golden-report test coverage.

## Runtime Boundary Rules

- Runtime profile compatibility and planned validation errors are detailed in `docs/transform-runtime-profile-compatibility.md`.
- A transform may be documented in the taxonomy without being exposed in config.
- A transform may be implemented for desktop without being exposed to deployment packages.
- A transform may be a no_std candidate without being validated on hardware.
- A transform may be a Pico 2 candidate only if fixed-buffer and compact-rule constraints are explicit.
- Deployment package exposure requires both capability status and runtime profile validation.
- Certification, hardware qualification, real-time, signing, and tamper-proof claims require separate gates.

## Hand-Off Note

Role: Software Architect
Goal: Complete M10-001 / issue #132 by defining transform capability vocabulary and matrix boundaries.
Files changed: `docs/transform-capability-model.md`
Checks run: Documentation and traceability review.
Status: Updated by M11 implementation in PR #147; milestone #11 is closed.
Known gaps: Runtime-profile validator code, M12 event/validation transform implementation, and high-pass baseline correction remain future gated work.
Next recommended step: Hold before M12 issue creation until explicit approval.
