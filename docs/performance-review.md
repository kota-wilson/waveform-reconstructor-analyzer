# Performance Review

Date: 2026-05-31

Owner Role: Performance Engineer

## Current Status

This is the initial publication performance review record plus the M4 large-CSV benchmark update. The repository still avoids production performance claims.

## Scope

Review MVP performance claims and known limits.

## Findings

No blocking performance issue found for the small-fixture MVP. No production performance claim is made.

## M4 Benchmark Update

M4 adds repeatable large-CSV measurement tooling without new dependencies:

- Helper binary: `crates/wra-cli/src/bin/wra-bench.rs`
- Fixture script: `scripts/benchmark-large-csv.sh`
- Baseline notes: `docs/benchmarking.md`

Latest local command:

```bash
sh scripts/benchmark-large-csv.sh 100000 3
```

Observed average timings are recorded in `docs/benchmarking.md`. Parse/reconstruction dominates the current local 100k-sample CSV path.

## M5 SVG Plotting Update

M5 plotting is validated as a small-fixture desktop SVG renderer only:

- 2D smoke output: `/private/tmp/wra-plot-2d.svg`, 21,670 bytes from `examples/basic-waveform.csv`.
- 3D smoke output: `/private/tmp/wra-plot-3d.svg`, 21,782 bytes from `tests/fixtures/plot_three_axis.csv`.
- No large-file plotting benchmark, interactivity benchmark, GUI frame-rate claim, DAQ throughput claim, or embedded performance claim is made.
- Plotting uses full in-memory parsed waveform data through the existing CLI parser path; streaming plot generation is future work if large-capture visualization becomes a requirement.

## Gate Decision

- Gate: Performance Gate.
- Decision: Pass for MVP and M4 baseline measurement.
- Reason: The current implementation handles example fixtures, M4 records a repeatable large-CSV baseline, M5 renders small fixture SVGs, and the documentation avoids performance guarantees.
- Residual risk: Memory profiling, streaming analysis, large-plot benchmarks, cross-platform benchmarks, and DAQ throughput remain future work.
- Next owner: Documentation Engineer.

## Hand-Off Note

Role: Performance Engineer
Goal: Prevent unsupported performance claims for the initial publication gate.
Files changed: `docs/performance-review.md`
Checks run: Reviewed scope, README, and risk register.
Status: Pass for MVP and M4 baseline.
Known gaps: No memory profiler baseline, streaming redesign, large-plot benchmark, or cross-platform benchmark matrix yet.
Next recommended step: Documentation review.
