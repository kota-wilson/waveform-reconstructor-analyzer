# Architecture Proposal

Date: 2026-05-30

## Summary

The system is a Rust Cargo workspace with a reusable core library and a small CLI. The architecture separates raw data ingestion, waveform modeling, derived waveform transformations, criteria evaluation, and report rendering so future GUI or language bindings can reuse the core without depending on CLI concerns.

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

## Module Map

| Module | Path | Responsibility |
|---|---|---|
| `model` | `crates/wra-core/src/model.rs` | Units, samples, channels, waveform structures. |
| `csv` | `crates/wra-core/src/csv.rs` | CSV parser and parser options backed by the `csv` crate. |
| `config` | `crates/wra-core/src/config.rs` | TOML-deserializable analysis configuration model. |
| `filter` | `crates/wra-core/src/filter.rs` | Filter trait, ordered filter-step enum, low-pass filter, moving-average filter, and ADC quantization transform. |
| `criteria` | `crates/wra-core/src/criteria.rs` | Pass/fail criteria definitions. |
| `analysis` | `crates/wra-core/src/analysis.rs` | Analysis results and evaluator interface. |
| `report` | `crates/wra-core/src/report.rs` | Report model with text and JSON rendering. |
| `error` | `crates/wra-core/src/error.rs` | Project error types. |

## Core Data Flow

```text
CSV input
  -> Parser options and channel mapping
  -> Waveform model
  -> Ordered transform chain produces derived waveform
  -> Criteria evaluator
  -> Analysis report
  -> CLI output
```

## Public API Outline

| Type / Trait | Location | Contract |
|---|---|---|
| `Waveform` | `model.rs` | Owns time axis, channels, and metadata. Validates aligned sample lengths. |
| `Channel` | `model.rs` | Named signal channel with unit and samples. |
| `CsvParseOptions` | `csv.rs` | Defines delimiter, header behavior, time column, and channel columns. |
| `AnalysisConfig` | `config.rs` | Defines input mapping, filters, and criteria parsed from TOML by the CLI. |
| `WaveformParser` | `csv.rs` | Parses input into `Waveform`. |
| `Filter` | `filter.rs` | Applies a transformation to a waveform and returns derived output. |
| `FilterStep` | `filter.rs` | Enum-backed ordered pipeline step for config-driven transforms. |
| `Criterion` | `criteria.rs` | Defines a measurable pass/fail rule. |
| `AnalysisResult` | `analysis.rs` | Records criterion outcome, measured value, threshold, and reason. |

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
- Moving average uses a trailing window that includes the current sample.
- Low-pass uses a simple first-order smoothing implementation over a strictly increasing time axis.
- ADC quantization uses an ideal endpoint-inclusive code model, clips outside the configured voltage range, and keeps output values in volts for downstream criteria.
- M1 filter chains use config-driven enum pipeline steps; trait-based extension is deferred behind the implementation boundary.
- Edge behavior, latency, and sample-rate assumptions must be documented before filter algorithms are considered production-stable.

## Test Plan

| Scenario | Test Location | Expected Result |
|---|---|---|
| Waveform aligned lengths | `crates/wra-core/src/model.rs` unit tests | Valid waveform is accepted. |
| Waveform mismatched lengths | `model.rs` tests | Structured error. |
| Empty CSV | `csv.rs` tests | Structured error. |
| Basic CSV fixture | `tests/fixtures/basic_waveform.csv` integration test | Parsed time and channels match expected values. |
| Filter chain preserves raw data | `filter.rs` tests | Input waveform remains unchanged. |
| ADC quantization | `filter.rs`, `config.rs`, and `wra-cli` tests | Samples quantize to ideal code levels before criteria evaluation. |
| CLI smoke | `crates/wra-cli` tests and `cargo run --bin wra -- analyze ...` | CLI loads a fixture, applies optional filters, evaluates criteria, and renders text. |

## Dependency Strategy

The current MVP slice uses approved third-party crates for CSV parsing, serialization, JSON reports, and TOML config parsing. Candidate future crates such as CLI argument parsers require dependency approval with license and security review.

## Out Of Scope

- GUI.
- DAQ integration.
- Certification claims.
- Cloud service.
- Plugin runtime.
- Python/C# bindings.

## Handoff

- Next role: Abstraction Review Engineer.
- Required gate: Granularity Gate before expanding implementation.
