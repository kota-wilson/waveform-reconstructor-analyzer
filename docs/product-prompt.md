# Product Prompt

## Product Name

Waveform Reconstructor and Analyzer

## Project Type

Open-source contribution/product repository; Rust systems and signal-analysis tool.

## Product Goal

Create a Rust-centered library and CLI that can import CSV waveform data, reconstruct analog waveform channels, apply simulated filters, evaluate signals against configurable pass/fail criteria, and export analysis outputs.

## Current Status

This is the historical intake prompt for the project. The repository has since been published publicly, the MIT license decision is recorded, approved dependencies are pinned in `Cargo.lock`, and current implementation status is tracked in `requirements.md`, `traceability-matrix.md`, and `project-state.md`.

## Target User

Test engineers, validation engineers, and open-source contributors working with time-series electrical waveform data from DAQ or test equipment.

## Platform

Cross-platform CLI first. Future GUI and bindings should be possible but are out of MVP scope.

## Recommended Tech Stack

- Rust Cargo workspace.
- `wra-core` library crate.
- `wra-cli` CLI crate.
- Standard library only for initial skeleton.
- Future dependency candidates require dependency review.

## Development Environment

- Project-local workspace.
- Cargo-managed build artifacts.
- No global dependencies.
- No system file modifications.
- Local-first tooling.

## Initial Features

1. CSV waveform input interface.
2. Time and signal channel mapping.
3. Waveform and channel data model.
4. Low-pass and moving-average filter extension points.
5. Pass/fail criteria model.
6. CLI skeleton.
7. Text/JSON report planning.
8. Example CSV fixture.

## Non-Goals

- GUI.
- Real-time DAQ.
- Cloud storage.
- Multi-user accounts.
- Certification claims.
- Hardware control.
- Proprietary format support.

## Assumptions

- CSV inputs are local files.
- Time values are numeric seconds unless configured otherwise.
- Signal values are numeric volts unless configured otherwise.
- The MVP can use a simple parser interface before selecting a production CSV dependency.
- MIT license is acceptable for the initial scaffold; the publication decision is now recorded in `decisions/ADR-002-license-assumption.md`.

## Risks

- CSV dialect variability.
- Incorrect units or sample-rate assumptions.
- Filter phase or latency effects being misunderstood.
- Overbroad MVP scope.
- Dependency/license choices before review.

## Acceptance Criteria

- [x] Project scaffold includes professional open-source files.
- [x] Requirements and traceability exist.
- [x] Architecture names modules, APIs, tests, and risks.
- [x] Rust workspace exists with core and CLI crates.
- [x] MVP milestone is defined.
- [x] No full implementation starts before architecture and MVP requirements exist.

## Granularity Requirements

- Expected first architecture zoom level: 1-3.
- Concrete artifacts required before implementation: module map, public API outline, tests, validation commands.
- Abstraction review required: Yes.

## Required Roles

- Intake Engineer.
- Project Coordinator.
- Project Orchestrator.
- Open Source Research Engineer.
- Software Architect.
- Systems Engineer.
- Core Software Engineer.
- Test Automation Engineer.
- Quality Assurance Engineer.
- Security Engineer.
- Performance Engineer.
- Documentation Engineer.
- Developer Experience Engineer.
- Code Review Engineer.
- Evaluation Engineer.
- GitHub Maintainer Specialist.
- Release Engineer.

## Required Standards

- Definition of Ready.
- Development Environment Standard.
- Requirements traceability.
- Risk register.
- Granularity standards.
- Definition of Done.
- Rust knowledge.
- Signal-processing knowledge.

## First Task

Run project creation pipeline and produce the initial repository skeleton, documentation set, architecture proposal, and MVP implementation plan.
