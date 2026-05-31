# Embedded Roadmap

Date: 2026-05-31

Owner Role: Software Architect

## Scope

The embedded path is a separate module track. It must not live inside the desktop CLI path and must not pull CSV parsing, file I/O, plotting, or report generation into embedded crates.

## Modular Split

| Module | Responsibility | Status |
|---|---|---|
| `wra-core` | `std`-capable shared domain models for the CLI and desktop analysis path. | Existing |
| `wra-signal` | `no_std` signal-processing primitives: fixed buffers, streaming ingestion, thresholds, transient events. | M3-RTOS-001 |
| `wra-criteria` | Future `no_std` pass/fail criteria engine if criteria outgrow `wra-signal`. | Future |
| `wra-cli` | Desktop CSV/config/report command-line interface. | Existing |
| `wra-embedded` | Future RTOS/ARM64 adapter layer around `wra-signal`. | Future |

## Adapter Order

1. M3-RTOS-001: Extract `wra-signal` `no_std` signal primitives.
2. M3-RTOS-002: Add ARM64 QEMU embedded demo.
3. M3-RTOS-003: Add RTOS adapter abstraction.
4. M3-RTOS-004: Add Zephyr feasibility prototype.

## Current Non-Goals

- No Zephyr implementation in M3-RTOS-001.
- No Embassy or RTIC adapter in M3-RTOS-001.
- No DAQ integration.
- No GUI.
- No aerospace or hardware certification claims.
- No heap requirement for the basic analysis path.

## Architecture Decision

Start with `wra-signal` before `wra-embedded`. This keeps reusable math and evaluation logic small, testable on desktop, and independent of RTOS runtime decisions. ARM64 and RTOS adapters should wrap the crate later.

## Gate Decision

- Gate: Architecture Gate.
- Decision: Pass.
- Reason: The embedded track has a separate crate boundary and explicit non-goals, matching the M3-RTOS-001 acceptance criteria.
- Residual risk: Future RTOS crates may need feature flags or adapter traits once hardware-facing APIs are introduced.
- Next owner: Core Software Engineer.
