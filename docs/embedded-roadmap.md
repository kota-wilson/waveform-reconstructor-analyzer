# Embedded Roadmap

Date: 2026-05-31

Owner Role: Software Architect

## Scope

The embedded path is a separate module track. It must not live inside the desktop CLI path and must not pull CSV parsing, file I/O, plotting, or report generation into embedded crates.

The first-class embedded target is Raspberry Pi 5 bare-metal ARM64 using Rust target `aarch64-unknown-none`. RTOS compatibility remains a later layer around that target rather than a generic embedded target replacement. Platform profiles are defined in `docs/platform-targets.md`.

## Modular Split

| Module | Responsibility | Status |
|---|---|---|
| `wra-core` | `std`-capable shared domain models for the CLI and desktop analysis path. | Existing |
| `wra-signal` | `no_std` signal-processing primitives: fixed buffers, streaming ingestion, thresholds, transient events. | M3-RTOS-001 |
| `wra-criteria` | Future `no_std` pass/fail criteria engine if criteria outgrow `wra-signal`. | Future |
| `wra-cli` | Desktop CSV/config/report command-line interface. | Existing |
| `wra-embedded` | `no_std` RTOS/ARM64 adapter boundary around `wra-signal` sample sources, event sinks, and runtime hooks. | M3-RTOS-003 |

## Adapter Order

1. M3-RTOS-001: Extract `wra-signal` `no_std` signal primitives.
2. M3-RTOS-002: Add host-checkable ARM64 QEMU embedded demo.
3. M3-RTOS-003: Add embedded adapter abstraction.
4. M3-RTOS-004: Add Zephyr feasibility prototype.
5. M9-011: Define Apple Silicon desktop and Raspberry Pi 5 bare-metal platform profiles.

## M3 Follow-Up Status

| Issue | Artifact | Status |
|---|---|---|
| M3-RTOS-002 | `embedded/arm64/qemu/` | Host-checkable ARM64 QEMU proof slice added; full QEMU image remains future work. |
| M3-RTOS-003 | `crates/wra-embedded/` | Adapter traits and streaming helpers added. |
| M3-RTOS-004 | `embedded/arm64/zephyr/` | Feasibility sketch and production-readiness risks documented. |

## Current Non-Goals

- No production Zephyr implementation.
- No Embassy or RTIC adapter implementation.
- No DAQ integration.
- No GUI.
- No aerospace or hardware certification claims.
- No heap requirement for the basic analysis path.
- No Raspberry Pi 5 hardware boot claim until separate board-support validation exists.
- No generic RTOS target claim replacing the Raspberry Pi 5 bare-metal first-class target.

## Architecture Decision

Start with `wra-signal`, then add `wra-embedded` as a small adapter boundary before any runtime-specific implementation. This keeps reusable math and evaluation logic small, testable on desktop, and independent of RTOS runtime decisions. ARM64/QEMU and Zephyr artifacts remain wrapper/prototype layers.

## Gate Decision

- Gate: Architecture Gate.
- Decision: Pass.
- Reason: The embedded track has separate signal, adapter, QEMU proof, and Zephyr feasibility boundaries with explicit non-goals.
- Residual risk: Future RTOS crates may need feature flags, target CI, unsafe FFI review, and hardware-facing API review once real runtimes are introduced.
- Next owner: Core Software Engineer.
