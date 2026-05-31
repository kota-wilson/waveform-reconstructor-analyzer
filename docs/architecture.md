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
| `wra-core` | `crates/wra-core` | Data model, CSV parser interface, filters, criteria, analysis results, report model. | Library API for CLI, future GUI, and bindings. |
| `wra-cli` | `crates/wra-cli` | Command-line argument handling and orchestration. | `wra` binary. |
| `wra-embedded` | `crates/wra-embedded` | `no_std` adapter traits and streaming helpers for ARM64/RTOS wrappers. | Embedded adapters around `wra-signal`. |
| `wra-plot` | `crates/wra-plot` | Desktop SVG plotting for waveform data. | SVG plot renderer used by the CLI. |
| `wra-signal` | `crates/wra-signal` | `no_std` signal primitives for future embedded adapters. | Dependency-free embedded-oriented primitives. |

## Module Map

| Module | Path | Responsibility |
|---|---|---|
| `model` | `crates/wra-core/src/model.rs` | Units, samples, channels, waveform structures, and metadata. |
| `csv` | `crates/wra-core/src/csv.rs` | CSV parser and parser options backed by the `csv` crate. |
| `config` | `crates/wra-core/src/config.rs` | TOML-deserializable analysis configuration model. |
| `filter` | `crates/wra-core/src/filter.rs` | Filter trait, ordered filter-step enum, low-pass filter, moving-average filter, and ADC quantization transform. |
| `criteria` | `crates/wra-core/src/criteria.rs` | Pass/fail criteria definitions. |
| `analysis` | `crates/wra-core/src/analysis.rs` | Analysis results and evaluator interface. |
| `report` | `crates/wra-core/src/report.rs` | Report model with text and JSON rendering. |
| `error` | `crates/wra-core/src/error.rs` | Project error types. |
| `wra-embedded` | `crates/wra-embedded/src/lib.rs` | `SampleSource`, `EventSink`, `RuntimeHooks`, and no_std streaming helper loops. |
| `wra-plot` | `crates/wra-plot/src/lib.rs` | SVG plotting with 2D and optional third-axis 3D line rendering. |

## Core Data Flow

```text
CSV input
  -> Parser options and channel mapping
  -> Waveform model
  -> Ordered transform chain produces derived waveform
  -> Criteria evaluator
  -> Analysis report
  -> CLI output

CSV input
  -> Parser options and plotting channel mapping
  -> Waveform model
  -> SVG plot renderer
  -> CLI output path

Embedded sample source
  -> wra-embedded adapter traits
  -> wra-signal threshold/transient primitive
  -> wra-embedded event sink
  -> platform wrapper
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
| `Criterion` | `criteria.rs` | Defines a measurable pass/fail rule. |
| `AnalysisResult` | `analysis.rs` | Records criterion outcome, measured value, threshold, applied tolerance, sample index, timestamp, channel, and reason. |
| `SampleSource`, `EventSink`, `RuntimeHooks` | `wra-embedded/src/lib.rs` | Define source, sink, and runtime boundaries for future embedded adapters. |
| `PlotOptions` | `wra-plot/src/lib.rs` | Defines SVG output path, title, plotted channels, optional third-axis channel, and dimensions. |

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
- Embedded adapters are bounded by `wra-embedded`; `wra-signal` remains runtime-independent.
- Plotting is a desktop-only SVG renderer in `wra-plot`; `wra-core` and `wra-signal` do not depend on Plotters.

## Test Plan

| Scenario | Test Location | Expected Result |
|---|---|---|
| Waveform aligned lengths | `crates/wra-core/src/model.rs` unit tests | Valid waveform is accepted. |
| Waveform mismatched lengths | `model.rs` tests | Structured error. |
| Empty CSV | `csv.rs` tests | Structured error. |
| Basic CSV fixture | `tests/fixtures/basic_waveform.csv` integration test | Parsed time and channels match expected values. |
| Filter chain preserves raw data | `filter.rs` tests | Input waveform remains unchanged. |
| ADC quantization | `filter.rs`, `config.rs`, and `wra-cli` tests | Samples quantize to ideal code levels before criteria evaluation. |
| Time-axis validation | `analysis.rs`, `model.rs`, and validation fixture tests | Duplicate/decreasing duration inputs are rejected; increasing non-uniform inputs are accepted and recorded in metadata. |
| Tolerance policy | `analysis.rs`, `config.rs`, and validation reports | Voltage/time tolerances affect criteria decisions and are recorded in result/report metadata. |
| CLI smoke | `crates/wra-cli` tests and `cargo run --bin wra -- analyze ...` | CLI loads a fixture, applies optional filters, evaluates criteria, and renders text. |
| Embedded adapter boundary | `crates/wra-embedded` tests and QEMU demo manifest check | no_std source/sink/runtime traits wrap `wra-signal` without desktop file I/O. |
| SVG plotting | `crates/wra-plot` tests and `wra-cli` plot tests | CLI writes 2D and 3D SVG files from CSV fixtures. |

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

## Handoff

- Next role: Abstraction Review Engineer.
- Required gate: Granularity Gate before expanding implementation.
