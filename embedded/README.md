# Embedded

This folder is reserved for embedded and RTOS adapter work that wraps no_std signal primitives without changing the desktop CLI path.

Planned order:

1. `crates/wra-signal`: no_std signal primitives.
2. `embedded/arm64/qemu`: ARM64 QEMU proof.
3. `crates/wra-embedded`: RTOS/ARM64 adapter layer.
4. `embedded/arm64/zephyr`: optional feasibility prototype.

Current status:

- `crates/wra-signal`: implemented by M3-RTOS-001.
- `crates/wra-embedded`: implemented by M3-RTOS-003 as a no_std adapter boundary.
- `embedded/arm64/qemu`: M3-RTOS-002 host-checkable proof slice exists.
- `embedded/arm64/zephyr`: M3-RTOS-004 feasibility sketch exists, but Zephyr is not integrated.

No GUI, DAQ integration, CSV parsing, file I/O, plotting, or report generation should be introduced here without a new milestone decision.
