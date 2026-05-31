# Measurement Engine

Date: 2026-05-31

## Scope

The v0.4.0 measurement engine starts with `ferrisoxide-measurements`, a reusable `#![no_std]` crate for deterministic measurements over time/sample slices.

This is the first step toward measurement-backed engineering evidence:

```text
CSV
  -> Waveform
  -> Transformations
  -> Measurements
  -> Criteria
  -> Evidence
  -> Report / SVG
```

## Current Boundary

| Item | Owner | Notes |
|---|---|---|
| Measurement primitives | `crates/ferrisoxide-measurements` | No CSV, TOML, plotting, reporting, file I/O, allocation, or third-party dependencies. |
| Criteria decisions | `crates/ferrisoxide-core/src/analysis.rs` | Calls `ferrisoxide-measurements` and applies tolerances, operators, pass/fail, and evidence wording. |
| Criteria definitions | `crates/ferrisoxide-core/src/criteria.rs` | Re-exports `SignalState` and `EdgeDirection` so existing callers can keep using `ferrisoxide_core::criteria`. |
| Reports | `crates/ferrisoxide-core/src/report.rs` | M6-003 adds reusable measurement records and per-result `measurement_id` links. |
| SVG evidence overlays | `crates/ferrisoxide-plot` | Deferred to issue #44. |

## Supported Measurements

| Measurement | Function | Evidence Returned |
|---|---|---|
| Minimum sample | `minimum_sample` | Sample index, timestamp, value. |
| Maximum sample | `maximum_sample` | Sample index, timestamp, value. |
| State transition count | `count_state_transitions` | Count plus first transition index/timestamp. |
| State run duration | `state_run_extremum` | Shortest or longest run for a requested high/low state. |
| Rise time | `measure_rise_time` | Start/end index, start/end timestamp, duration. |
| Fall time | `measure_fall_time` | Start/end index, start/end timestamp, duration. |

## Validation Fixtures

The M6-005 known-answer fixture in `validation/measurement_engine/` covers state transition count, pulse width, transient/dropout duration, stable-state duration, rise time, fall time, time tolerance behavior, and strictly increasing time-axis assumptions.

Expected values are documented in `validation/measurement_engine/expected-measurements.md` and compared exactly through `validation/reports/measurement_engine_known_answer.json`.

## Report Evidence

M6-003 separates measured evidence from criteria decisions in report output.

- `measurements[]` contains reusable evidence records with stable report-local IDs.
- `results[].measurement_id` references the measurement used by each criterion decision.
- Criterion results keep `measured_value`, `required_value`, `tolerance_used`, sample index, timestamp, channel, and reason fields for compatibility with existing report readers.
- `method_context` records threshold, state, edge, event-kind, and selection parameters needed to audit how each measurement was produced.

## Compatibility

M6-001 preserved the existing CLI behavior and JSON report shape while extracting primitives. M6-003 intentionally migrates the JSON schema by adding `measurements[]` and `measurement_id` while preserving existing result fields. The golden JSON tests continue to compare exact output.

The extraction intentionally preserves existing tie behavior for equal-duration longest runs: the later run is selected, matching the previous criteria evaluator and existing report evidence.

## Out Of Scope

- New TOML DSL syntax.
- Annotated SVG evidence overlays.
- Batch analysis.
- Plugin runtime.
- GUI, DAQ, RTOS expansion, hardware qualification, or certification claims.
