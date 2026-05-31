# Platform Targets

Date: 2026-05-31

Status: Planned platform profile; initial desktop and Raspberry Pi 5 target checks passed locally. Raspberry Pi Pico 2 is documented as a future optional microcontroller profile only.

## Desktop Target

The desktop application targets Apple Silicon macOS systems.

Platform profile:

- Hardware: Apple Silicon Mac.
- CPU: ARM64 / AArch64.
- OS: macOS.
- Rust target: `aarch64-apple-darwin`.
- Language: Rust.
- Runtime profile: `std`.

Primary responsibilities:

- Rule authoring.
- Controller simulation.
- CSV/DAQ waveform analysis.
- SVG/report generation.
- Deployment package export.

Desktop crates are allowed to use `std`, local files, CSV parsing, TOML/JSON parsing, SVG plotting, report rendering, CLI workflows, and future desktop UI surfaces after separate approval.

Current desktop/std crates:

| Crate | Runtime Profile | Notes |
|---|---|---|
| `wra-core` | `std` | CSV, config, criteria, analysis, filters, and report model. |
| `wra-cli` | `std` | CLI analysis, plotting, benchmark, and future export workflows. |
| `wra-plot` | `std` | Desktop SVG plotting with Plotters. |

## Embedded Target

The embedded runtime targets Raspberry Pi 5 bare-metal ARM64 deployments.

Platform profile:

- Hardware: Raspberry Pi 5.
- CPU: ARM64 / AArch64.
- OS: bare metal initially, RTOS-compatible later.
- Rust target: `aarch64-unknown-none`.
- Language: Rust.
- Runtime profile: `no_std`.

Primary responsibilities:

- Load approved deployment package.
- Run production control config.
- Run test verification config.
- Evaluate live signal inputs.
- Produce pass/fail or control outputs.

Embedded crates must avoid desktop-only dependencies and runtime assumptions. The first-class embedded target is Raspberry Pi 5 bare-metal ARM64; RTOS work is a later compatibility layer around that target, not a generic substitute for it.

Current embedded/no_std crates:

| Crate | Runtime Profile | Notes |
|---|---|---|
| `wra-signal` | `no_std` | Fixed-size buffers, threshold evaluation, transient event primitives. |
| `wra-embedded` | `no_std` | Sample source, event sink, runtime hook, and streaming adapter traits around `wra-signal`. |

Planned embedded-compatible crates:

| Crate | Runtime Profile | Notes |
|---|---|---|
| `wra-rule-schema` | `no_std`-compatible subset expected | Portable rule package schema from v0.6.0 planning. |
| `wra-rule-engine` | `no_std`-compatible subset expected | Shared rule execution semantics from v0.6.0 planning. |
| `wra-control-schema` | `no_std`-compatible subset expected | Production control config schema from v0.7.0 planning. |

## Optional Microcontroller Target

Raspberry Pi Pico 2 is useful as a constrained microcontroller runtime target, not as a replacement for the Raspberry Pi 5 controller-computer runtime.

Platform profile:

- Hardware: Raspberry Pi Pico 2.
- Microcontroller: RP2350.
- CPU: selectable dual Arm Cortex-M33 cores or dual Hazard3 RISC-V cores.
- Clock class: up to 150 MHz.
- Memory: 520 KB SRAM.
- Board flash: 4 MB onboard QSPI flash.
- I/O class: GPIO, ADC-capable pins, SPI, I2C, UART, PWM, USB, and PIO state machines.
- Language: Rust.
- Runtime profile: `no_std`.
- Rust target direction: Cortex-M33 `thumbv8m` target selection remains future board-support work; no Pico 2 target compile check is claimed yet.

Primary responsibilities:

- Simple real-time controller logic.
- Threshold detection.
- State transition detection.
- Pulse width checks.
- Transient event detection.
- GPIO/PWM control actions.
- Modest ADC sampling.
- Local pass/fail flagging.
- Production-test fixture logic.

Pico 2 should not be used for:

- Large waveform storage.
- CSV parsing.
- SVG/report generation.
- Complex filtering.
- Large multi-channel analysis.
- Heavy desktop-style simulation.

Future optional crate:

| Crate | Runtime Profile | Notes |
|---|---|---|
| `wra-pico-runtime` | `no_std` | Future microcontroller adapter for fixed-size buffers, compact binary config, threshold/timing criteria, simple moving average, GPIO/PWM output actions, and no heap requirement where practical. |

`wra-pico-runtime` is intentionally not created yet. It should be added only after the portable rule package, shared rule engine, and controller I/O boundaries define the minimum deployable rule subset for microcontrollers.

Spec references reviewed for this profile:

- Raspberry Pi Pico 2 product page: <https://www.raspberrypi.com/products/raspberry-pi-pico-2/>
- RP2350 product page: <https://www.raspberrypi.com/products/rp2350/>
- Raspberry Pi Pico 2 datasheet: <https://datasheets.raspberrypi.com/pico/pico-2-datasheet.pdf>

### Pi 5 vs Pico 2 Boundary

| Target | Use For | Avoid |
|---|---|---|
| Raspberry Pi 5 bare-metal ARM64 | Richer embedded/controller deployment, larger buffers, logging, package loading, networking, and complex runtime behavior. | Treating it as a tiny deterministic MCU loop without documenting OS/runtime timing assumptions. |
| Raspberry Pi Pico 2 | Deterministic small control loops, direct GPIO/PWM/ADC interaction, threshold/timing checks, and local pass/fail outputs. | Desktop-equivalent analysis, report generation, large fixtures, and complex multi-channel workflows. |

## Shared Requirement

Desktop, Raspberry Pi 5 embedded, and future Pico 2 micro-runtime targets must use the same applicable:

- rule schema
- control schema
- criteria engine
- timing model
- test vectors
- parity tests

The goal is not to create separate desktop, embedded, and microcontroller rule behavior. The desktop target authors, simulates, analyzes, reports, and exports. The Raspberry Pi 5 bare-metal target consumes approved artifacts and runs deterministic production/test logic with constrained resources. The Pico 2 target should consume only a compact approved subset that is proven equivalent for its supported criteria.

## Architecture Implication

Keep this separation:

```text
macOS desktop
  -> std
  -> UI/CLI workflows
  -> file I/O
  -> CSV/TOML/JSON parsing
  -> SVG/report generation
  -> deployment package export

Raspberry Pi 5 bare metal
  -> no_std
  -> fixed buffers
  -> deterministic runtime
  -> live controller I/O adapters
  -> compact pass/fail or control outputs

Raspberry Pi Pico 2 micro-runtime
  -> no_std
  -> fixed buffers
  -> compact binary config subset
  -> threshold and timing criteria
  -> simple filters
  -> GPIO/PWM pass/fail or control outputs
```

Desktop-only crates must not be pulled into the embedded runtime path. Embedded-compatible crates must keep CSV parsing, plotting, rich reports, vendor SDKs, hardware HALs, RTOS APIs, heap requirements for basic evaluation, and certification claims out of the core target boundary unless a future gate explicitly approves them.

Pico 2 support must be stricter than the Raspberry Pi 5 embedded path. It must keep CSV parsing, text/JSON/SVG reporting, large waveform buffers, dynamic allocation requirements, complex simulation, target HALs, USB/probe tooling, and PIO/ADC drivers out of shared rule logic unless a future hardware-specific gate explicitly approves them.

## Target Verification Direction

Target checks should be added to CI only after the required target toolchains are available in the project workflow.

Initial host-checkable commands:

```sh
cargo check --workspace --target aarch64-apple-darwin
cargo check -p wra-signal --target aarch64-unknown-none
cargo check -p wra-embedded --target aarch64-unknown-none
```

Local evidence recorded on 2026-05-31:

| Check | Result |
|---|---|
| `rustc --print target-list` includes `aarch64-apple-darwin` and `aarch64-unknown-none` | Pass |
| `rustup target list --installed` includes `aarch64-apple-darwin` and `aarch64-unknown-none` | Pass |
| `cargo check --workspace --target aarch64-apple-darwin` | Pass |
| `cargo check -p wra-signal --target aarch64-unknown-none` | Pass |
| `cargo check -p wra-embedded --target aarch64-unknown-none` | Pass |

Future parity tests should prove that Apple Silicon desktop logic and Raspberry Pi 5 bare-metal-compatible logic consume the same rule/control definitions and produce matching expected results for shared test vectors.

Future Pico 2 parity tests should prove only the supported micro-runtime subset. Unsupported desktop or Raspberry Pi 5 features must fail validation with clear target-profile errors instead of silently degrading behavior.

## Scope Boundaries

In scope:

- Apple Silicon macOS as the desktop authoring target.
- Raspberry Pi 5 bare-metal ARM64 as the first-class embedded runtime target.
- Raspberry Pi Pico 2 as a documented future optional microcontroller runtime target.
- Clear `std` vs `no_std` crate boundaries.
- Target compile checks for supported crates.
- Desktop/embedded parity test definitions.

Out of scope until separate approval:

- Generic RTOS target claims.
- Vendor DAQ SDKs.
- Raspberry Pi hardware boot or board support package work.
- Hardware HAL implementation.
- QEMU boot images beyond existing host-checkable sketches.
- Zephyr production support.
- Real-time timing guarantees.
- Safety, hardware qualification, or certification claims.
- Creating `wra-pico-runtime`, adding Pico 2 HAL support, or claiming Pico 2 runtime validation.

## Hand-Off Note

Role: Software Architect / Embedded RTOS Engineer
Goal: Define the Apple Silicon desktop and Raspberry Pi 5 bare-metal platform profiles.
Files changed: `docs/platform-targets.md`.
Checks run: `rustc --print target-list`; `rustup target list --installed`; `cargo check --workspace --target aarch64-apple-darwin`; `cargo check -p wra-signal --target aarch64-unknown-none`; `cargo check -p wra-embedded --target aarch64-unknown-none`.
Status: Planned platform profile.
Known gaps: CI target checks, Raspberry Pi 5 boot validation, target HALs, deployment loader, and parity test implementation remain future work.
Next recommended step: Route issue #89 through implementation when v0.7.0 platform validation is prioritized.
