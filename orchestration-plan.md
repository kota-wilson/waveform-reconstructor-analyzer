# Orchestration Plan

Project: FerrisOxide

Project folder: `/Users/kota/Desktop/softwareai/projects/ferrisoxide`

Execution tier: Tier 2 MVP implementation

Current objective: Route the validated dependency-free MVP slice toward dependency review, reporting/config planning, or release gating.

Current stage: Dependency-free MVP slice validated

Selected workflow: `workflows/project-orchestration-pipeline.md`

Selected mode: `modes/rust-systems.md` plus `modes/signal-analysis.md`

## Inputs Reviewed

- Product prompt: `docs/product-prompt.md`
- Project charter: `project-charter.md`
- Requirements: `requirements.md`
- Risk register: `risk-register.md`
- Traceability matrix: `traceability-matrix.md`
- Project state: `project-state.md`
- Selected standards: Rust, signal-processing, open-source library, data-analysis, environment, granularity.

## Milestones

| Milestone | Goal | Owner Role | Entry Gate | Exit Evidence | Status |
|---|---|---|---|---|---|
| M1 | MVP foundation: data model, CSV parser interface, waveform model, CLI analysis path | Core Software Engineer | Architecture Gate | Passing Cargo tests and docs | Complete |
| M2 | Filtering MVP: low-pass and moving average | Systems Engineer | M1 tests pass | Filter tests and CLI smoke run | Complete |
| M3 | Criteria and reporting MVP | Core Software Engineer | M2 tests pass | Pass/fail analysis tests and text report example | Complete |
| M4 | QA, docs, and release readiness | QA / Docs / Release | Validation passes | Release readiness report | Blocked pending license/publication decision |

## Zoom-Level Plan

| Stage | Expected Level | Required Artifacts | Abstraction Review Needed |
|---|---:|---|---|
| Architecture | 1-3 | Modules, crates, APIs, tests, risks | Yes |
| Implementation | 3-5 | Files, structs, traits, functions, tests | Yes |
| Testing | 3-5 | Test files, fixtures, commands, expected results | No |

## Task Queue

| Task ID | Task | Owner Role | Inputs | Deliverables | Gate | Status |
|---|---|---|---|---|---|---|
| WRA-TASK-001 | Create project creation package | Project Coordinator | User request | Charter, requirements, risk, traceability, state | Project Creation Gate | Complete |
| WRA-TASK-002 | Create architecture and MVP plan | Software Architect | Requirements and domain standards | `docs/architecture.md`, `docs/mvp-plan.md` | Architecture Gate | Complete |
| WRA-TASK-003 | Review abstraction level | Abstraction Review Engineer | Architecture and MVP plan | `docs/abstraction-review.md` | Granularity Gate | Complete |
| WRA-TASK-004 | Implement M1 foundation | Core Software Engineer | Architecture, skeleton | Data model, parser interface, CLI tests | Implementation Gate | Complete |
| WRA-TASK-005 | Validate M1 | Test Automation Engineer | M1 implementation | Cargo test/fmt/clippy evidence | Testing Gate | Complete |
| WRA-TASK-006 | Implement dependency-free M2/M3 continuation | Core Software Engineer / Systems Engineer | User approval to continue without dependencies | Filters, criteria evaluator, text report, CLI smoke path | Implementation Gate | Complete |
| WRA-TASK-007 | Validate dependency-free M2/M3 continuation | Test Automation Engineer | M2/M3 implementation | 12 passing tests, fmt, clippy, CLI smoke evidence | Testing Gate | Complete |
| WRA-TASK-008 | Decide dependency and publication path | Project Orchestrator / Security Engineer / Release Engineer | Current validation package | Dependency proposal or release readiness report | Dependency / Release Gate | Pending |

## Approval Gates

| Gate | Trigger | Required Approver | Evidence Needed | Status |
|---|---|---|---|---|
| Architecture approval | Before expanding implementation beyond skeleton | User / Technical Director | Architecture, MVP plan, risks, acceptance criteria | Passed for std-only MVP continuation |
| Dependency approval | Before adding third-party crates | User / Security Engineer | Dependency reason, license, alternatives | Pending |
| Release approval | Before public repository publication | User / Release Engineer | Validation, docs, license confirmation | Pending |

## Risks To Monitor

| Risk | Owner | Mitigation | Review Trigger |
|---|---|---|---|
| CSV dialect variability | Software Architect | Explicit dialect MVP and fixtures | Parser work |
| Filter misinterpretation | Systems Engineer | Document units and filter assumptions | Filter work |
| Scope creep | Project Coordinator | Enforce non-goals | Milestone planning |
| Dependency risk | Security Engineer | Dependency gate | Dependency proposal |

## State Updates Required

- Project state after every milestone.
- Risk register when data, dependencies, or scope changes.
- Traceability matrix after implementation and tests.
- Decision records for license, CSV dependency, and config format.

## Next Role Ticket

You are the Project Orchestrator.

Purpose

Route the validated dependency-free MVP slice to the next explicit gate.

Responsibilities

- Keep changes inside this project.
- Do not add third-party crates without dependency approval.
- Do not publish externally without license and release approval.
- Choose whether next work is config parsing, production CSV support, report export, or release readiness.
- Preserve validation evidence and update traceability after the next milestone.

Deliverables

- Dependency proposal, release readiness report, or next implementation ticket.
- Updated gate decision.
- Handoff note.

Expected format to recieve deliverables

Use the shared handoff note format from root `AGENTS.md`.

## Stop Conditions

- Stop before adding dependencies.
- Stop before GUI, DAQ, certification, or cloud work.
- Stop before public repository publication.
