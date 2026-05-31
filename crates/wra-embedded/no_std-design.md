# wra-embedded no_std Design

Date: 2026-05-31

## Purpose

`wra-embedded` defines the runtime adapter boundary that sits between platform-specific embedded code and `wra-signal`.

The crate is intentionally small so future ARM64/QEMU, Embassy-style, RTIC, Zephyr, or bare-metal adapters can be evaluated without pulling those runtimes into the signal-analysis core.

## Boundary

```text
platform driver / RTOS task
  -> SampleSource
  -> wra-embedded streaming helper
  -> wra-signal primitive
  -> EventSink
  -> platform logging / telemetry / assertion layer
```

## Traits

| Trait | Responsibility | Non-goals |
|---|---|---|
| `SampleSource` | Provide monotonic timestamp/value samples to the analyzer. | CSV parsing, file I/O, DAQ driver ownership. |
| `EventSink` | Receive threshold and transient-event results. | JSON/text report formatting, heap-backed logs. |
| `RuntimeHooks` | Provide scheduler/runtime lifecycle hooks around polling and sample ingestion. | Owning an async runtime, HAL, or RTOS API. |

## Allocation Policy

The adapter crate does not require `alloc`. Demos and tests use fixed slices and `Option` fields.

## Error Policy

Platform-specific source, sink, and runtime errors remain associated types. `AdapterError` keeps those errors distinct from `wra_signal::SignalError` so hardware/runtime failures do not look like signal-analysis failures.

## Current Limits

- Host unit tests validate the adapter boundary.
- ARM64 target build and QEMU execution are documented but not required in CI.
- Zephyr remains a feasibility prototype, not a production integration.
