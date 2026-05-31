# no_std Design

Date: 2026-05-31

## Goal

Provide reusable signal-processing primitives that can run in embedded environments without `std`, file systems, heap allocation, or desktop reporting dependencies.

## Design Rules

- Use `#![no_std]`.
- Avoid `Vec`, `String`, file I/O, CSV parsing, JSON, TOML, and allocation.
- Prefer fixed-size buffers with const generics.
- Accept streaming samples one at a time.
- Return compact pass/fail structs with evidence fields.
- Keep runtime and RTOS adapters outside this crate.

## Current Types

| Type | Purpose |
|---|---|
| `Sample` | Timestamp/value pair. |
| `FixedSampleBuffer<N>` | Stack/static-friendly fixed-size sample storage. |
| `ThresholdLimits` | Min/max threshold check configuration. |
| `ThresholdTracker` | Streaming min/max tracker with monotonic timestamp validation. |
| `ThresholdEvaluation` | Threshold pass/fail result with measured min/max. |
| `TransientEventDetector` | Streaming detector for unintended threshold-crossing events. |
| `TransientEventKind` | Engineering event subtype such as dropout, contact bounce, or spurious transition. |

## Evidence Contract

Pass/fail result structs include sample index and timestamp evidence so adapters can surface failures without needing desktop reports. Threshold evaluations record the failed check when a min/max limit is violated. Transient event evaluations record event kind, observed state, measured duration, required duration, sample index, and timestamp.

## Adapter Direction

Later crates or folders should wrap this crate in order:

1. `embedded/arm64/qemu/` for a bare-metal proof.
2. `ferrisoxide-embedded` for RTOS/ARM64 adapter boundaries.
3. `embedded/arm64/zephyr/` for feasibility only after core primitives are stable.

No Zephyr or RTOS code belongs in this crate.
