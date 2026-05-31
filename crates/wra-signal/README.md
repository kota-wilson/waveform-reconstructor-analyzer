# wra-signal

`wra-signal` contains no_std signal-analysis primitives for embedded and RTOS-oriented adapters.

This crate intentionally excludes:

- CSV parsing.
- File I/O.
- Plotting.
- Text or JSON report generation.
- Heap allocation for basic analysis.
- GUI, DAQ, or certification logic.

## Current Scope

- Fixed-size sample buffers.
- Streaming sample ingestion.
- Streaming min/max threshold tracking.
- Min/max threshold evaluation.
- Transient event detection with typed event kinds.
- Pass/fail result structs.

## Evidence Fields

Threshold and transient event evaluations return compact evidence suitable for embedded adapters:

- pass/fail status
- failed threshold check when applicable
- measured value or duration
- required value or duration
- sample index
- timestamp
- observed state for transient events

The first target is a small, testable signal core. ARM64 QEMU, Embassy-style adapters, and Zephyr feasibility work should wrap this crate later instead of pushing RTOS concerns into it.
