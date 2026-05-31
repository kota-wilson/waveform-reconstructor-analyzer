# wra-embedded

`wra-embedded` is the adapter-boundary crate for future ARM64, RTOS, and board-specific integrations.

It is `#![no_std]` and wraps `wra-signal` with small traits for:

- sample sources,
- event sinks,
- runtime or scheduler hooks.

## Included

- `SampleSource` for streaming samples into signal primitives.
- `EventSink` for passing threshold and transient-event evaluations out to an adapter.
- `RuntimeHooks` for scheduler/runtime integration points.
- `SliceSampleSource`, `LastResultSink`, and `NoopRuntime` for demos and tests.
- `run_threshold_stream` and `run_transient_event_stream` helper loops.

## Excluded

- CSV parsing.
- File I/O.
- Heap allocation requirements.
- Plotting.
- Text or JSON report generation.
- Hardware HALs.
- Embassy, RTIC, Zephyr, or other RTOS bindings.
- DAQ integration or certification evidence.

Runtime-specific crates should implement these traits without changing `wra-signal`.
