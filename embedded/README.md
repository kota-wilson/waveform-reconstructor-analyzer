# Embedded

This folder is reserved for embedded and RTOS adapter work that wraps no_std signal primitives without changing the desktop CLI path.

Planned order:

1. `crates/wra-signal`: no_std signal primitives.
2. `embedded/arm64/qemu`: ARM64 QEMU proof.
3. `crates/wra-embedded`: RTOS/ARM64 adapter layer.
4. `embedded/arm64/zephyr`: optional feasibility prototype.

No GUI, DAQ integration, CSV parsing, file I/O, plotting, or report generation should be introduced here without a new milestone decision.
