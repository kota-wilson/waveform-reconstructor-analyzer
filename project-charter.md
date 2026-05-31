# Project Charter

Project Name: FerrisOxide

Project Type: Rust-centered open-source product repository

Date Created: 2026-05-30

Owner: Project Coordinator

## Primary Goal

Build a robust, modular Rust application/library platform whose current FerrisOxide Signal slice imports CSV waveform data, reconstructs analog waveforms from recorded channel samples, applies simulated signal filters, and evaluates waveforms against configurable pass/fail criteria.

## Target User

Test engineers and validation engineers who analyze raw CSV data from DAQ or test equipment during environmental testing workflows.

## Target Platforms

- Primary: Cross-platform CLI on macOS, Linux, and Windows.
- Secondary: Future desktop UI and language bindings.

## Tech Stack

- Language: Rust.
- Framework: Cargo workspace with core library plus CLI crate.
- Tooling: Cargo, rustfmt, clippy, GitHub Actions.
- Runtime: Native command-line executable.

## Scope

In scope:

- CSV waveform input.
- Time column and signal channel mapping.
- Waveform data model.
- Low-pass filter and moving-average filter interfaces.
- Pass/fail criteria model.
- CLI skeleton.
- JSON/text report planning.
- Example data and tests.

Out of scope:

- Full GUI.
- Real-time DAQ integration.
- Cloud storage.
- Multi-user accounts.
- Aerospace certification claims.
- Hardware control.
- Proprietary file formats beyond extension points.

## Quality Requirements

- Modular architecture.
- Strong error handling.
- Clear public APIs.
- Unit tests and integration tests.
- Example CSV files.
- Reproducible builds.
- Documentation-first design.
- Open-source contribution readiness.
- No global system modifications.

## Applicable Standards

- `knowledge/standards/compliance-order.md`.
- `knowledge/standards/development-environment.md`.
- `knowledge/rust.md`.
- `knowledge/signal-processing.md`.
- `domains/signal-processing/`.
- ISO 9001-inspired quality management.
- ISO/IEC/IEEE 12207-inspired lifecycle process.
- ISO/IEC 25010-inspired software quality attributes.
- ISO/IEC/IEEE 29119-inspired test process.
- ISO/IEC 27001-inspired security-aware development.
- ISO 31000-inspired risk management.

## Selected Roles

- Intake Engineer: product prompt.
- Project Coordinator: gates and state.
- Project Orchestrator: milestone sequencing.
- Open Source Research Engineer: open-source readiness.
- Software Architect: architecture and API boundaries.
- Systems Engineer: Rust systems design.
- Core Software Engineer: implementation.
- Test Automation Engineer: test matrix.
- Quality Assurance Engineer: acceptance review.
- Security Engineer: input and file safety.
- Performance Engineer: data-size and filter performance.
- Documentation Engineer: user and contributor docs.
- Developer Experience Engineer: local setup and CI.
- Code Review Engineer: review.
- Evaluation Engineer: scorecard and gaps.
- GitHub Maintainer Specialist: repository conventions.
- Release Engineer: versioning and release readiness.

## Development Environment

- Isolation level: Level 1 project-local workspace.
- Environment path: `/Users/kota/Desktop/softwareai/projects/ferrisoxide`.
- Dependency manager: Cargo.
- Lockfile strategy: Commit `Cargo.lock` for CLI/application reproducibility.

## First Implementation Task

Task: Build only the Rust core data model, CSV parser interface, waveform model, and CLI skeleton.

Owner: Core Software Engineer.

Done when: `cargo test --workspace` passes and public APIs are documented enough for architecture review.

## Orchestration Plan

- Execution tier: Tier 2 MVP implementation after approval.
- Selected workflow: `workflows/project-orchestration-pipeline.md` plus `workflows/open-source-library.md` and `workflows/data-analysis.md`.
- First milestone: MVP foundation.
- Approval gates before full implementation: architecture gate, dependency gate for third-party crates, release gate before public publication.

## Granularity Plan

- Expected architecture zoom level: 1-3.
- Expected implementation zoom level: 3-5.
- Concrete artifacts required before implementation: requirements, architecture, traceability, task breakdown, test plan, acceptance criteria.
- Abstraction review required: Yes.
