# Platform Targets

Date: 2026-05-31

Status: Planned platform profile; initial target checks passed locally.

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

## Shared Requirement

Desktop and embedded targets must use the same:

- rule schema
- control schema
- criteria engine
- timing model
- test vectors
- parity tests

The goal is not to create separate desktop and embedded rule behavior. The desktop target authors, simulates, analyzes, reports, and exports. The Raspberry Pi 5 bare-metal target consumes approved artifacts and runs deterministic production/test logic with constrained resources.

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
```

Desktop-only crates must not be pulled into the embedded runtime path. Embedded-compatible crates must keep CSV parsing, plotting, rich reports, vendor SDKs, hardware HALs, RTOS APIs, heap requirements for basic evaluation, and certification claims out of the core target boundary unless a future gate explicitly approves them.

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

## Scope Boundaries

In scope:

- Apple Silicon macOS as the desktop authoring target.
- Raspberry Pi 5 bare-metal ARM64 as the first-class embedded runtime target.
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

## Hand-Off Note

Role: Software Architect / Embedded RTOS Engineer
Goal: Define the Apple Silicon desktop and Raspberry Pi 5 bare-metal platform profiles.
Files changed: `docs/platform-targets.md`.
Checks run: `rustc --print target-list`; `rustup target list --installed`; `cargo check --workspace --target aarch64-apple-darwin`; `cargo check -p wra-signal --target aarch64-unknown-none`; `cargo check -p wra-embedded --target aarch64-unknown-none`.
Status: Planned platform profile.
Known gaps: CI target checks, Raspberry Pi 5 boot validation, target HALs, deployment loader, and parity test implementation remain future work.
Next recommended step: Route issue #89 through implementation when v0.7.0 platform validation is prioritized.
