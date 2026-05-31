# Architecture Proposal

Date: 2026-05-30

## Summary

The system is a Rust Cargo workspace with a reusable core library and a small CLI. The architecture separates raw data ingestion, waveform modeling, derived waveform transformations, criteria evaluation, and report rendering so future GUI or language bindings can reuse the core without depending on CLI concerns.

Current status: This proposal has been implemented through the validated MVP feature baseline. Current implementation details and remaining open issues are tracked in `requirements.md`, `traceability-matrix.md`, and `project-state.md`.

## Context Reviewed

| Source | Finding |
|---|---|
| User request | Rust must be core; CSV, reconstruction, filters, criteria, reports, and open-source readiness are required. |
| `knowledge/rust.md` | Use Cargo validation, avoid unnecessary dependencies, document public APIs. |
| `knowledge/signal-processing.md` | State units, preserve raw data, use synthetic tests, document filter assumptions. |
| `domains/signal-processing/` | Accuracy, reproducibility, tolerances, and sample-rate assumptions are core quality concerns. |

## Proposed Crates

| Crate | Path | Responsibility | Public Surface |
|---|---|---|---|
| `ferrisoxide-core` | `crates/ferrisoxide-core` | Data model, CSV parser interface, filters, criteria, analysis results, report model. | Library API for CLI, future GUI, and bindings. |
| `ferrisoxide-cli` | `crates/ferrisoxide-cli` | Command-line argument handling and orchestration. | `ferrisoxide-signal` binary. |
| `ferrisoxide-embedded` | `crates/ferrisoxide-embedded` | `no_std` adapter traits and streaming helpers for ARM64/RTOS wrappers. | Embedded adapters around `ferrisoxide-signal`. |
| `ferrisoxide-measurements` | `crates/ferrisoxide-measurements` | `no_std` measurement primitives over time/sample slices. | Extrema, transition count, state-run duration, and rise/fall measurements used by criteria evidence. |
| `ferrisoxide-plot` | `crates/ferrisoxide-plot` | Desktop SVG plotting for waveform data and 2D evidence overlays. | SVG plot renderer used by the CLI. |
| `ferrisoxide-rule-engine` | `crates/ferrisoxide-rule-engine` | `no_std` shared rule execution semantics over caller-provided time/sample slices. | Owned criteria/evidence API for desktop adapters plus borrowed summary API with borrowed/static errors for constrained embedded-compatible paths. |
| `ferrisoxide-rule-schema` | `crates/ferrisoxide-rule-schema` | Versioned portable FerrisOxide Rule Package data model and validator. | Package metadata, target profile, sample timing, channels, units, thresholds, filters, measurement-backed criteria definitions, parse helpers, and structured validation errors. |
| `ferrisoxide-signal` | `crates/ferrisoxide-signal` | `no_std` signal primitives for future embedded adapters. | Dependency-free embedded-oriented primitives. |

Portable rule package validator, export, checksum, shared-engine, no_std-boundary, and exact desktop-vs-embedded parity fixture work is implemented through M8-008. Runtime loaders and binary package work remain future scope in `decisions/ADR-004-portable-rule-package-architecture.md` and `docs/v0.6.0-portable-rule-package-milestone-proposal.md`.

Future controller-in-the-loop simulation and deployment config modules are planned in `docs/controller-in-the-loop-workflow.md` and `docs/v0.7.0-controller-simulation-deployment-config-milestone-proposal.md`. They are not implemented yet.

Platform targets are documented in `docs/platform-targets.md`. The desktop authoring platform is Apple Silicon macOS using `aarch64-apple-darwin`; the first-class embedded runtime target is Raspberry Pi 5 bare-metal ARM64 using `aarch64-unknown-none`; Raspberry Pi Pico 2 is a future optional microcontroller profile for constrained rule execution.

## Module Map

| Module | Path | Responsibility |
|---|---|---|
| `model` | `crates/ferrisoxide-core/src/model.rs` | Units, samples, channels, waveform structures, and metadata. |
| `csv` | `crates/ferrisoxide-core/src/csv.rs` | CSV parser and parser options backed by the `csv` crate. |
| `config` | `crates/ferrisoxide-core/src/config.rs` | TOML-deserializable analysis configuration model. |
| `filter` | `crates/ferrisoxide-core/src/filter.rs` | Filter trait, ordered filter-step enum, low-pass filter, moving-average filter, and ADC quantization transform. |
| `criteria` | `crates/ferrisoxide-core/src/criteria.rs` | Pass/fail criteria definitions. |
| `analysis` | `crates/ferrisoxide-core/src/analysis.rs` | Analysis results, measurement records, and adapter from desktop waveform/config types into `ferrisoxide-rule-engine`. |
| `report` | `crates/ferrisoxide-core/src/report.rs` | Report model with text and JSON rendering, including reusable measurement evidence. |
| `error` | `crates/ferrisoxide-core/src/error.rs` | Project error types. |
| `ferrisoxide-embedded` | `crates/ferrisoxide-embedded/src/lib.rs` | `SampleSource`, `EventSink`, `RuntimeHooks`, and no_std streaming helper loops. |
| `ferrisoxide-measurements` | `crates/ferrisoxide-measurements/src/lib.rs` | Slice-based measurement functions with no allocation, file I/O, parsing, plotting, or reporting. |
| `ferrisoxide-plot` | `crates/ferrisoxide-plot/src/lib.rs` | SVG plotting with 2D evidence overlays and optional third-axis 3D line rendering. |
| `ferrisoxide-rule-engine` | `crates/ferrisoxide-rule-engine/src/lib.rs` | `no_std` criteria execution over slices; owned evidence API uses `alloc`, borrowed summary API avoids owned criterion/result strings and borrowed-path heap allocation for basic checks; avoids CSV parsing, TOML parsing, plotting, report rendering, file I/O, DAQ/controller I/O, HALs, and SDKs. |
| `ferrisoxide-rule-schema` | `crates/ferrisoxide-rule-schema/src/lib.rs` | Versioned portable rule package schema types and validation helpers; no CSV, CLI, plotting, report rendering, package export, checksum algorithm, controller I/O, HAL, SDK, or rule execution behavior. |

## Core Data Flow

```text
CSV input
  -> Parser options and channel mapping
  -> Waveform model
  -> Ordered transform chain produces derived waveform
  -> Measurement primitives
  -> Criteria evaluator
  -> Analysis report
  -> CLI output

CSV input
  -> Parser options and plotting channel mapping
  -> Waveform model
  -> SVG plot renderer
  -> CLI output path

Embedded sample source
  -> ferrisoxide-embedded adapter traits
  -> ferrisoxide-signal threshold/transient primitive
  -> ferrisoxide-embedded event sink
  -> platform wrapper

Future portable deployment flow
  -> Desktop FerrisOxide Signal authors and validates criteria
  -> FerrisOxide Rule Package schema captures rules, units, channels, and timing assumptions
  -> Shared no_std rule engine executes the same semantics for desktop and embedded-compatible paths
  -> Controller runtime consumes constrained deployment artifacts

Future controller-in-the-loop flow
  -> Desktop simulator loads production control config and test verification config
  -> DAQ abstraction, fixtures, or generated waveforms provide UUT signals
  -> Virtual controller runs shared state-machine logic
  -> Qualification evidence and deployment package are generated
  -> RTOS/controller runtime consumes approved configs in production or verification mode

Platform split
  -> Apple Silicon macOS desktop uses std, files, reports, plotting, and export workflows
  -> Raspberry Pi 5 bare-metal ARM64 uses no_std, fixed buffers, deterministic runtime, and compact outputs
  -> Raspberry Pi Pico 2 micro-runtime uses no_std, fixed buffers, compact rule subsets, threshold/timing checks, and GPIO/PWM outputs
```

## Public API Outline

| Type / Trait | Location | Contract |
|---|---|---|
| `Waveform` | `model.rs` | Owns time axis, channels, and metadata. Validates aligned sample lengths. |
| `WaveformMetadata` | `model.rs` | Records source name, units, sample count, channel count, channel units, sample interval summary, nominal sample rate, raw/derived lineage, and transform history. |
| `Channel` | `model.rs` | Named signal channel with unit and samples. |
| `CsvParseOptions` | `csv.rs` | Defines delimiter, header behavior, time column, and channel columns. |
| `AnalysisConfig` | `config.rs` | Defines input mapping, optional metadata context, tolerance policy, filters, and criteria parsed from TOML by the CLI. |
| `WaveformParser` | `csv.rs` | Parses input into `Waveform`. |
| `Filter` | `filter.rs` | Applies a transformation to a waveform and returns derived output. |
| `FilterStep` | `filter.rs` | Enum-backed ordered pipeline step for config-driven transforms. |
| `minimum_sample`, `maximum_sample`, `count_state_transitions`, `state_run_extremum`, `measure_rise_time`, `measure_fall_time` | `ferrisoxide-measurements/src/lib.rs` | Reusable measurement primitives used by `ferrisoxide-core` criteria evaluation. |
| `Criterion` | `criteria.rs` | Defines a measurable pass/fail rule. |
| `evaluate_rule_set` | `ferrisoxide-rule-engine/src/lib.rs` | Executes shared rule criteria over time/sample slices and returns owned pass/fail plus measurement evidence for desktop adapters. |
| `evaluate_borrowed_rule` | `ferrisoxide-rule-engine/src/lib.rs` | Executes one borrowed rule over time/sample slices and returns a compact `RuleSummary` or borrowed/static error for embedded-compatible no-heap basic evaluation. |
| `MeasurementRecord` | `analysis.rs` | Records reusable measurement evidence with stable ID, method context, measured value, unit, channel, sample index, and timestamp. |
| `AnalysisResult` | `analysis.rs` | Records criterion outcome, linked `measurement_id`, measured value, threshold, applied tolerance, sample index, timestamp, channel, and reason. |
| `EvidenceOverlay` | `ferrisoxide-plot/src/lib.rs` | Plot-facing annotation data derived from report measurement evidence. |
| `SampleSource`, `EventSink`, `RuntimeHooks` | `ferrisoxide-embedded/src/lib.rs` | Define source, sink, and runtime boundaries for future embedded adapters. |
| `PlotOptions` | `ferrisoxide-plot/src/lib.rs` | Defines SVG output path, title, plotted channels, optional third-axis channel, and dimensions. |

## MVP Error Handling

- Use explicit `WaveformError` enum.
- Avoid panics in library code for malformed user input.
- Return structured errors for empty CSV, missing columns, non-numeric values, mismatched sample lengths, and unsupported criteria.

## Data And Units

- Time unit defaults to seconds.
- Voltage unit defaults to volts.
- Samples are represented as `f64`.
- Future unit conversion belongs behind explicit APIs, not implicit parser behavior.

## Transform Assumptions

- Raw waveform data is preserved.
- Filters and ADC quantization return derived waveform outputs.
- Derived waveform metadata records transform history so reports can show which data was evaluated.
- Moving average uses a trailing window that includes the current sample.
- Low-pass uses a simple first-order smoothing implementation over a strictly increasing time axis.
- ADC quantization uses an ideal endpoint-inclusive code model, clips outside the configured voltage range, and keeps output values in volts for downstream criteria.
- M1 filter chains use config-driven enum pipeline steps; trait-based extension is deferred behind the implementation boundary.
- Edge behavior, latency, and sample-rate assumptions must be documented before filter algorithms are considered production-stable.
- Implemented transform equations are documented in `docs/filter-behavior.md`.
- Time-axis validation and tolerance semantics are documented in `docs/time-axis-and-tolerances.md`.
- Embedded adapters are bounded by `ferrisoxide-embedded`; `ferrisoxide-signal` remains runtime-independent.
- Measurement primitives are bounded by `ferrisoxide-measurements`; `ferrisoxide-core` applies criteria policy and report wording.
- Reports expose top-level measurement records and per-result `measurement_id` links so measured evidence and pass/fail decisions remain auditable separately.
- Plotting is a desktop-only SVG renderer in `ferrisoxide-plot`; 2D evidence overlays reuse report measurement evidence; `ferrisoxide-core` and `ferrisoxide-signal` do not depend on Plotters.
- Criteria DSL direction is documented in `docs/criteria-dsl.md`; existing `[[criteria]]` entries remain the runtime compatibility baseline.
- Portable rule package direction is documented in `decisions/ADR-004-portable-rule-package-architecture.md`; the initial package format is documented in `docs/rule-package-format.md`; `ferrisoxide-rule-engine` now owns shared `no_std` rule semantics for desktop and embedded-compatible paths.
- Controller-in-the-loop direction is documented in `docs/controller-in-the-loop-workflow.md`; production control config and test verification config remain separate but linked through manifests and parity evidence.
- Platform target direction is documented in `docs/platform-targets.md`; RTOS compatibility is a later layer around the Raspberry Pi 5 bare-metal ARM64 first-class embedded target, and Pico 2 support is a later optional microcontroller subset rather than a full runtime replacement.

## Test Plan

| Scenario | Test Location | Expected Result |
|---|---|---|
| Waveform aligned lengths | `crates/ferrisoxide-core/src/model.rs` unit tests | Valid waveform is accepted. |
| Waveform mismatched lengths | `model.rs` tests | Structured error. |
| Empty CSV | `csv.rs` tests | Structured error. |
| Basic CSV fixture | `tests/fixtures/basic_waveform.csv` integration test | Parsed time and channels match expected values. |
| Filter chain preserves raw data | `filter.rs` tests | Input waveform remains unchanged. |
| ADC quantization | `filter.rs`, `config.rs`, and `ferrisoxide-cli` tests | Samples quantize to ideal code levels before criteria evaluation. |
| Time-axis validation | `analysis.rs`, `model.rs`, and validation fixture tests | Duplicate/decreasing duration inputs are rejected; increasing non-uniform inputs are accepted and recorded in metadata. |
| Tolerance policy | `analysis.rs`, `config.rs`, and validation reports | Voltage/time tolerances affect criteria decisions and are recorded in result/report metadata. |
| Measurement extraction | `crates/ferrisoxide-measurements` tests and existing golden criteria tests | Measurement primitives produce the same evidence values currently expected by criteria reports. |
| Report measurement schema | `analysis.rs`, `report.rs`, CLI tests, and exact golden JSON tests | Reports contain reusable measurement records and criteria results reference them by stable ID. |
| SVG evidence overlays | `ferrisoxide-plot` and `ferrisoxide-cli` tests plus CLI smoke command | 2D SVG plots include pass/fail status, threshold lines, and failed-criterion labels from measurement evidence. |
| Measurement-engine validation fixture | `validation/measurement_engine/` and exact report test | Known-answer values cover transition count, pulse width, transient duration, stable-state duration, and rise/fall time. |
| CLI smoke | `crates/ferrisoxide-cli` tests and `cargo run --bin ferrisoxide-signal -- analyze ...` | CLI loads a fixture, applies optional filters, evaluates criteria, and renders text. |
| Embedded adapter boundary | `crates/ferrisoxide-embedded` tests and QEMU demo manifest check | no_std source/sink/runtime traits wrap `ferrisoxide-signal` without desktop file I/O. |
| Shared rule engine | `crates/ferrisoxide-rule-engine`, `crates/ferrisoxide-core`, and `crates/ferrisoxide-embedded` tests | Desktop analysis delegates criteria semantics to the shared engine, and embedded-compatible tests evaluate fixed slices through the same engine. |
| no_std rule boundary | `crates/ferrisoxide-rule-engine` target checks and dependency trees | Rule-engine and embedded crates compile for `aarch64-unknown-none`, and dependency trees show no desktop CSV, TOML, plotting, report, HAL, SDK, or file-I/O crates in the embedded-compatible path. |
| Desktop-vs-embedded parity | `tests/parity/` and `crates/ferrisoxide-core/tests/rule_parity.rs` | The same waveform and rule package produce exact matching portable evidence fields from the desktop core path and embedded-compatible borrowed-rule path. |
| SVG plotting | `crates/ferrisoxide-plot` tests and `ferrisoxide-cli` plot tests | CLI writes 2D and 3D SVG files from CSV fixtures. |

## Dependency Strategy

The current MVP slice uses approved third-party crates for CSV parsing, serialization, JSON reports, TOML config parsing, and desktop SVG plotting. Candidate future crates such as CLI argument parsers or additional plotting backends require dependency approval with license and security review.

## Out Of Scope

- GUI.
- DAQ integration.
- Certification claims.
- Cloud service.
- Plugin runtime.
- Python/C# bindings.
- Embedded/RTOS plotting.
- Interactive plotting controls.
- Production RTOS integration or hardware HALs.
- Zephyr production support.
- Pico 2 runtime crate, Pico HAL support, ADC/PIO drivers, probe tooling, or microcontroller production readiness.

## Handoff

- Next role: Abstraction Review Engineer.
- Required gate: Granularity Gate before expanding implementation.
