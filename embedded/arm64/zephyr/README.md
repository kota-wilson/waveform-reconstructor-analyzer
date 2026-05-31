# Zephyr Feasibility Prototype

Status: M3-RTOS-004 feasibility prototype.

This folder documents how a future Zephyr adapter could wrap `wra-embedded` without making Zephyr a dependency of `wra-signal`, `wra-core`, or `wra-cli`.

## Prototype Boundary

```text
Zephyr sensor / GPIO / ADC driver
  -> ZephyrSampleSource
  -> wra-embedded RuntimeHooks
  -> wra-embedded streaming helper
  -> wra-signal threshold / transient primitive
  -> ZephyrEventSink
```

The prototype sketch is in `zephyr_adapter_sketch.rs`. It is intentionally not wired into the workspace build because no Zephyr SDK, board, or C/Rust binding layer is approved or required for M3.

## Toolchain Assumptions

Potential future validation would require:

- Zephyr SDK installed outside this repository.
- A selected ARM64 board or QEMU Zephyr target.
- A Rust-for-Zephyr build path selected and reviewed.
- C/Rust FFI or binding strategy reviewed for safety and maintenance.
- CI runner with the required SDK image if automated builds become necessary.

## Unsupported Areas

- No Zephyr kernel module is built in this milestone.
- No device tree, Kconfig, CMake, or west workspace is added.
- No hardware HAL or board-specific driver is added.
- No unsafe FFI is added.
- No heap, file I/O, CSV parsing, plotting, or report generation is introduced.

## Production Readiness Risks

- Zephyr Rust support and binding stability must be reviewed before production use.
- Scheduler timing and interrupt behavior can affect sample ordering and latency.
- Error mapping from Zephyr drivers to signal-analysis errors needs explicit design.
- Toolchain provenance and SDK version pinning are required before repeatable validation.
- This feasibility note is not hardware qualification, RTOS readiness evidence, or certification evidence.
