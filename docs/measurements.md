# Measurement Engine

Date: 2026-05-31

## Scope

The v0.4.0 measurement engine starts with `wra-measurements`, a reusable `#![no_std]` crate for deterministic measurements over time/sample slices.

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
| Measurement primitives | `crates/wra-measurements` | No CSV, TOML, plotting, reporting, file I/O, allocation, or third-party dependencies. |
| Criteria decisions | `crates/wra-core/src/analysis.rs` | Calls `wra-measurements` and applies tolerances, operators, pass/fail, and evidence wording. |
| Criteria definitions | `crates/wra-core/src/criteria.rs` | Re-exports `SignalState` and `EdgeDirection` so existing callers can keep using `wra_core::criteria`. |
| Reports | `crates/wra-core/src/report.rs` | Unchanged in M6-001. Report schema expansion is tracked by issue #45. |
| SVG evidence overlays | `crates/wra-plot` | Deferred to issue #44. |

## Supported Measurements

| Measurement | Function | Evidence Returned |
|---|---|---|
| Minimum sample | `minimum_sample` | Sample index, timestamp, value. |
| Maximum sample | `maximum_sample` | Sample index, timestamp, value. |
| State transition count | `count_state_transitions` | Count plus first transition index/timestamp. |
| State run duration | `state_run_extremum` | Shortest or longest run for a requested high/low state. |
| Rise time | `measure_rise_time` | Start/end index, start/end timestamp, duration. |
| Fall time | `measure_fall_time` | Start/end index, start/end timestamp, duration. |

## Compatibility

M6-001 preserves the existing CLI behavior and JSON report shape. The golden JSON tests continue to compare exact output.

The extraction intentionally preserves existing tie behavior for equal-duration longest runs: the later run is selected, matching the previous criteria evaluator and existing report evidence.

## Out Of Scope

- New TOML DSL syntax.
- Report `measurements` schema section.
- Annotated SVG evidence overlays.
- Batch analysis.
- Plugin runtime.
- GUI, DAQ, RTOS expansion, hardware qualification, or certification claims.
