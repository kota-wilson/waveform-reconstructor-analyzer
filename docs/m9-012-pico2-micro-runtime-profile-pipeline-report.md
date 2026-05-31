# M9-012 Pico 2 Micro-Runtime Profile Pipeline Report

Date: 2026-05-31

Issue: #92, `M9-012 Define Raspberry Pi Pico 2 micro-runtime profile`

Status: Implemented and merged through PR #93. Runtime crate is intentionally deferred.

## Summary

Raspberry Pi Pico 2 is documented as an optional future microcontroller runtime target for compact deterministic rule execution. It is not a replacement for the Raspberry Pi 5 bare-metal ARM64 embedded runtime and is not a desktop-equivalent analysis target.

## Source Review

Official Raspberry Pi sources reviewed:

- Raspberry Pi Pico 2 product page: <https://www.raspberrypi.com/products/raspberry-pi-pico-2/>
- RP2350 product page: <https://www.raspberrypi.com/products/rp2350/>
- Raspberry Pi Pico 2 datasheet: <https://datasheets.raspberrypi.com/pico/pico-2-datasheet.pdf>

Profile facts captured:

- RP2350 microcontroller.
- Selectable dual Arm Cortex-M33 cores or dual Hazard3 RISC-V cores.
- Up to 150 MHz class operation.
- 520 KB SRAM.
- 4 MB onboard QSPI flash on Pico 2.
- GPIO, ADC-capable pins, SPI, I2C, UART, PWM, USB, and PIO-oriented I/O class.

## Scope Decision

Pico 2 is suitable for:

- simple real-time controller logic
- threshold detection
- state transition detection
- pulse width checks
- transient event detection
- GPIO/PWM control
- modest ADC sampling
- local pass/fail flagging
- production-test fixture logic

Pico 2 is out of scope for:

- large waveform storage
- CSV parsing
- SVG/report generation
- complex filtering
- large multi-channel analysis
- heavy desktop-style simulation
- HAL, probe, USB, PIO, or ADC driver implementation in this issue

## Future Module Boundary

Future crate:

```text
crates/ferrisoxide-pico-runtime/
```

Expected scope:

- `no_std`
- fixed-size buffers
- no heap where practical
- compact binary config subset
- threshold criteria
- timing criteria
- simple moving average
- GPIO/PWM output actions

The crate should be added only after the portable rule package, shared rule engine, controller I/O abstraction, and target-profile validation rules are far enough along to prevent a forked microcontroller rule implementation.

## Gate Decisions

| Gate | Decision | Evidence | Residual Risk | Next Owner |
|---|---|---|---|---|
| Intake Gate | Pass | User requested Pico 2 as optional micro-runtime target. | Scope could drift into hardware support. | Project Coordinator |
| Requirements Gate | Pass | WRA-RQ-062 added. | Future runtime acceptance criteria will need target-specific tests. | Software Architect |
| Architecture Gate | Pass | `docs/platform-targets.md`, `docs/embedded-roadmap.md`, `docs/controller-in-the-loop-workflow.md`. | Pico 2 target triple and BSP choice remain future work. | Embedded RTOS Engineer |
| Scope Gate | Pass | Runtime crate, HALs, drivers, and hardware validation are explicitly out of scope. | Users may still overread the profile as production readiness. | Documentation Engineer |
| Security Gate | Pass | No dependencies, HALs, SDKs, drivers, USB/probe tooling, or unsafe code added. | Future hardware-facing work will need fresh review. | Security Engineer |
| Performance Gate | Pass | Docs describe microcontroller memory limits and avoid throughput claims. | Real timing evidence requires hardware or simulator validation later. | Performance Engineer |
| V&V Gate | Pass | Traceability entry requires future target-profile validation and parity for supported subset. | No Pico hardware execution evidence exists. | V&V Engineer |
| Documentation Gate | Pass | Profile, roadmap, workflow, risk, and traceability docs updated. | Automated link checking is still absent. | Documentation Engineer |
| Release Gate | Pass | PR #93 merged after required `rust` CI passed; issue #92 closed. | Runtime implementation remains future work. | Project Orchestrator |

## Verification Commands

Completed for this documentation-only change:

```sh
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
git diff --check
```

## Hand-Off Note

Role: Software Architect / Embedded RTOS Engineer
Goal: Document Raspberry Pi Pico 2 as an optional future microcontroller runtime profile.
Files changed: `docs/platform-targets.md`, `docs/embedded-roadmap.md`, `docs/controller-in-the-loop-workflow.md`, `docs/architecture.md`, `docs/v0.7.0-controller-simulation-deployment-config-milestone-proposal.md`, `requirements.md`, `traceability-matrix.md`, `risk-register.md`, `project-state.md`, and this report.
Checks run: `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Documentation implementation validated locally and merged through PR #93.
Known gaps: `ferrisoxide-pico-runtime`, target triple selection, BSP/HAL selection, compact binary config loader, GPIO/PWM/ADC integration, Pico 2 hardware tests, and parity tests remain future work.
Next recommended step: Continue M7-003 / issue #57 unless v0.6.0 or v0.7.0 is explicitly reprioritized.
