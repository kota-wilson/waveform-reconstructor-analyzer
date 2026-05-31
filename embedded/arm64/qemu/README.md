# QEMU ARM64 Demo

Status: M3-RTOS-002 proof slice.

This folder contains a small `#![no_std]` Rust library that demonstrates `wra-embedded` and `wra-signal` usage without desktop file I/O, CSV parsing, plotting, or report generation.

## What This Proves

- Fixed sample data can be streamed through the embedded adapter boundary.
- `wra-signal` threshold evaluation can run behind `wra-embedded` source/sink/runtime traits.
- The proof can be checked by Cargo without installing QEMU or an ARM64 target.

## Local Verification

Host-check the demo crate:

```bash
cargo test --manifest-path embedded/arm64/qemu/Cargo.toml
```

This command validates the adapter path and demo outcome on the desktop host. It is not an ARM64 execution claim.

## ARM64/QEMU Assumptions

A future freestanding QEMU image is expected to use:

- Rust target: `aarch64-unknown-none`.
- QEMU machine: `virt`.
- Platform start code: future `bare-metal/` startup and linker files.
- Analyzer entry point: `wra_arm64_qemu_demo::run_demo()`.

Sketch command, not run by CI:

```bash
cargo build --manifest-path embedded/arm64/qemu/Cargo.toml --target aarch64-unknown-none
qemu-system-aarch64 -M virt -cpu cortex-a53 -nographic -kernel <future-demo-image>
```

## Out Of Scope

- Zephyr.
- Hardware boards.
- DAQ integration.
- GUI or plotting.
- UART drivers, linker scripts, startup assembly, or panic strategy.
- Hardware validation, tool qualification, or certification evidence.
