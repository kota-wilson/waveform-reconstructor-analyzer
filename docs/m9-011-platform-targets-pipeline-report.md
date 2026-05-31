# M9-011 Platform Targets Pipeline Report

Date: 2026-05-31

Branch: `planning/platform-target-profiles`

Milestone: #9, `v0.7.0: Controller Simulation and Deployment Config System`

Issue: #89, `M9-011 Define Apple Silicon desktop and Raspberry Pi 5 bare-metal platform profiles`

Pull request: #90, `Define platform target profiles`

Status: PR #90 open; protected CI and merge pending.

## Scope

This slice defines first-class platform profiles for WRA.

In scope:

- Document Apple Silicon macOS as the desktop authoring target.
- Document Raspberry Pi 5 bare-metal ARM64 as the first-class embedded runtime target.
- Record Rust target triples `aarch64-apple-darwin` and `aarch64-unknown-none`.
- Identify current `std` and `no_std` crate boundaries.
- Verify desktop workspace compile for `aarch64-apple-darwin`.
- Verify embedded crates compile for `aarch64-unknown-none`.
- Define desktop/embedded parity expectations.

Out of scope:

- Raspberry Pi 5 hardware boot.
- Board support package work.
- Vendor DAQ SDKs.
- Hardware HALs.
- RTOS production integration.
- Zephyr production support.
- Real-time, safety, hardware qualification, or certification claims.

## Requirements

- Owner role: Software Architect / Embedded RTOS Engineer
- Artifact: WRA-RQ-061 in `requirements.md`.
- Gate: Requirements Gate.
- Decision: Pass.
- Residual risk: CI target checks remain future work until workflow updates are separately approved.
- Next owner: Embedded RTOS Engineer.

## Architecture

- Owner role: Software Architect / Embedded RTOS Engineer
- Artifact: `docs/platform-targets.md`, `docs/architecture.md`, `docs/controller-in-the-loop-workflow.md`, and `docs/embedded-roadmap.md`.
- Decision: Apple Silicon macOS is the `std` desktop authoring target; Raspberry Pi 5 bare-metal ARM64 is the `no_std` embedded target; RTOS compatibility is a later layer.
- Gate: Architecture Gate.
- Decision: Pass.
- Residual risk: Future runtime adapters must not reintroduce generic RTOS assumptions without Raspberry Pi 5 target checks.
- Next owner: Verification and Validation Engineer.

## Verification

- Owner role: Verification and Validation Engineer
- Evidence:
  - `rustc --print target-list` includes `aarch64-apple-darwin`.
  - `rustc --print target-list` includes `aarch64-unknown-none`.
  - `rustup target list --installed` includes `aarch64-apple-darwin`.
  - `rustup target list --installed` includes `aarch64-unknown-none`.
  - `cargo check --workspace --target aarch64-apple-darwin`: Pass.
  - `cargo check -p wra-signal --target aarch64-unknown-none`: Pass.
  - `cargo check -p wra-embedded --target aarch64-unknown-none`: Pass.
  - `cargo fmt --check`: Pass.
  - `cargo test --workspace`: Pass.
  - `cargo clippy --workspace --all-targets -- -D warnings`: Pass.
  - `git diff --check`: Pass.
- Gate: Target Verification Gate.
- Decision: Pass locally.
- Residual risk: Target checks are not yet required in GitHub CI.
- Next owner: QA Engineer.

## QA

- Owner role: QA Engineer
- Artifact: Documentation review.
- Evidence: The platform split clearly separates desktop `std` responsibilities from embedded `no_std` responsibilities and avoids production readiness claims.
- Gate: QA Gate.
- Decision: Pass locally.
- Residual risk: Raspberry Pi 5 board-level validation remains future work.
- Next owner: Security Engineer.

## Security

- Owner role: Security Engineer
- Artifact: Scope review.
- Evidence: No new dependencies, DAQ SDKs, HALs, unsafe FFI, credentials, network behavior, signing, or binary package format are added.
- Gate: Security Gate.
- Decision: Pass locally.
- Residual risk: Future target/SDK work requires dependency and environment review.
- Next owner: Performance Engineer.

## Performance

- Owner role: Performance Engineer
- Artifact: Scope review.
- Evidence: No runtime code path or benchmark claim changes in this slice.
- Gate: Performance Gate.
- Decision: Not Applicable.
- Reason: Documentation and target compile checks only.
- Residual risk: Raspberry Pi 5 runtime performance remains unmeasured until implementation exists.
- Next owner: Documentation Engineer.

## Documentation

- Owner role: Documentation Engineer
- Artifact: `docs/platform-targets.md`.
- Evidence: File defines desktop target, embedded target, shared requirements, architecture implication, target verification direction, and scope boundaries.
- Gate: Documentation Gate.
- Decision: Pass locally.
- Residual risk: CI docs should be updated when target checks are added to workflow.
- Next owner: Code Reviewer.

## Release

- Owner role: Release Engineer
- Artifact: Local validation evidence.
- Gate: Release Gate.
- Decision: Pending PR/CI/merge.
- Residual risk: Branch work is not released until protected `rust` CI passes and the PR is merged.
- Next owner: GitHub Maintainer Specialist.

## Hand-Off Note

Role: Software Architect / Embedded RTOS Engineer
Goal: Complete M9-011 / issue #89 platform profile definition.
Files changed: `docs/platform-targets.md`, architecture docs, requirements, traceability, risk register, project state, and this report.
Checks run: `rustc --print target-list`; `rustup target list --installed`; `cargo check --workspace --target aarch64-apple-darwin`; `cargo check -p wra-signal --target aarch64-unknown-none`; `cargo check -p wra-embedded --target aarch64-unknown-none`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: PR #90 open; protected CI and merge pending.
Known gaps: CI target checks, Raspberry Pi 5 hardware boot, HALs, deployment loader, and parity test implementation remain future work.
Next recommended step: Wait for required CI and merge issue #89.
