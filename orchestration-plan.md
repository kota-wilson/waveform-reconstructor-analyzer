# Orchestration Plan

Project: FerrisOxide

Project folder: `/Users/kota/Desktop/codexprojects/softwaredev/projects/ferrisoxide`

Execution tier: Tier 2 MVP plus roadmap-controlled follow-on milestones

Current objective: Record M10 transform architecture closure after PR #138 merged.

Current stage: M10 GitHub milestone #10 is closed; PR #138 merged by squash commit `69b8b1a4a7c963316a74130655667ea3ff1481d5`; issues #132 through #137 are closed.

Selected workflow: `workflows/project-orchestration-pipeline.md`

Selected mode: `modes/rust-systems.md` plus `modes/signal-analysis.md`

## Inputs Reviewed

- Product prompt: `docs/product-prompt.md`
- Project charter: `project-charter.md`
- Requirements: `requirements.md`
- Risk register: `risk-register.md`
- Traceability matrix: `traceability-matrix.md`
- Project state: `project-state.md`
- Transform taxonomy: `docs/analog-transform-taxonomy.md`
- Next milestone roadmap: `docs/next-milestones-roadmap.md`
- Selected standards: Rust, signal-processing, open-source library, data-analysis, environment, granularity.

## Milestones

| Milestone | Goal | Owner Role | Entry Gate | Exit Evidence | Status |
|---|---|---|---|---|---|
| M1-M9 | Validated MVP, embedded/no_std foundation, validation, plotting, measurement/evidence, DSL, portable rule package, controller simulation/deployment config | Multiple roles | Historical gates | Implemented requirements WRA-RQ-001 through WRA-RQ-069 and closed M9 | Complete |
| M10 / v0.8.0 | Transform architecture and capability metadata | Software Architect | Human approval and issue creation | Metadata model, existing-transform mappings, compatibility tests, docs, merged PR #138, closed issues #132 through #137, closed milestone #10 | Complete |
| M11 / v0.9.0 | Pointwise and windowed transform MVP | Core Software Engineer / Systems Engineer | M10 architecture accepted | Pointwise, baseline, moving median, metadata, raw-preservation tests | Proposed locally |
| M12 / v0.10.0 | Event and validation transform MVP | Core Software Engineer / V&V Engineer | M10 accepted and M11 compatibility path established | Event records, Schmitt trigger, debounce, glitch removal, event validation, fixtures | Proposed locally |

## Zoom-Level Plan

| Stage | Expected Level | Required Artifacts | Abstraction Review Needed |
|---|---:|---|---|
| M10 architecture | 1-3 | Capability model, metadata fields, runtime profiles, compatibility path, tests | Yes |
| M10 implementation | 3-5 | Files, structs/enums, config adapters, report fields, tests | Yes |
| M11 implementation | 3-5 | Transform modules, config validation, CLI/report integration, fixtures | Yes |
| M12 implementation | 3-5 | Event records, validation records, known-answer fixtures, parity tests | Yes |

## Task Queue

| Task ID | Task | Owner Role | Inputs | Deliverables | Gate | Status |
|---|---|---|---|---|---|---|
| WRA-TASK-009 | Create next milestone roadmap | Project Coordinator / Product Architect | Taxonomy and project state | `docs/next-milestones-roadmap.md` | Roadmap Gate | Complete locally |
| WRA-TASK-010 | Create M10 transform architecture proposal | Software Architect | Taxonomy, current filter/config/report model | `docs/v0.8.0-transform-architecture-milestone-proposal.md`; WRA-RQ-070 through WRA-RQ-074 | Requirements Gate | Complete locally |
| WRA-TASK-011 | Create M11 pointwise/windowed transform proposal | Software Architect / Systems Engineer | M10 scope and taxonomy | `docs/v0.9.0-pointwise-windowed-transform-mvp-milestone-proposal.md`; WRA-RQ-075 through WRA-RQ-080 | Requirements Gate | Complete locally |
| WRA-TASK-012 | Create M12 event/validation transform proposal | Software Architect / V&V Engineer | M10 scope and switch/test-validation taxonomy | `docs/v0.10.0-event-validation-transform-milestone-proposal.md`; WRA-RQ-081 through WRA-RQ-086 | Requirements Gate | Complete locally |
| WRA-TASK-013 | Convert proposals into local issue placeholders | GitHub Maintainer Specialist | M10-M12 proposals | `docs/next-milestones-issue-planning-report.md` | Issue Planning Gate | Complete locally |
| WRA-TASK-014 | Approve M10 and create GitHub issues | Project Coordinator / GitHub Maintainer Specialist | M10 proposal and placeholders M10-001 through M10-006 | GitHub milestone #10 and issues #132 through #137 | Human Approval Gate | Complete |
| WRA-TASK-015 | Implement M10-001 transform capability vocabulary | Software Architect / Documentation Engineer | Issue #132 | `docs/transform-capability-model.md`, docs links, traceability updates, pipeline report | Implementation Gate | Complete locally |
| WRA-TASK-016 | Implement M10-002 structured transform metadata design | Software Architect / Documentation Engineer | Issue #133 and M10-001 vocabulary | `docs/structured-transform-metadata.md`, report-schema note, docs links, traceability updates, pipeline report | Implementation Gate | Complete locally |
| WRA-TASK-017 | Implement M10-003 current transform metadata mappings | Systems Engineer / Documentation Engineer | Issue #134, M10-001 vocabulary, M10-002 design | `docs/current-transform-metadata-mapping.md`, docs links, traceability updates, pipeline report | Implementation Gate | Complete locally |
| WRA-TASK-018 | Implement M10-004 runtime profile compatibility rules | Embedded RTOS Engineer / Documentation Engineer | Issue #135 and current mappings | `docs/transform-runtime-profile-compatibility.md`, docs links, traceability updates, pipeline report | Implementation Gate | Complete locally |
| WRA-TASK-019 | Implement M10-005 transform docs wording update | Documentation Engineer | Issue #136 and M10 docs | README/architecture/taxonomy/filter wording cleanup, traceability updates, pipeline report | Implementation Gate | Complete locally |
| WRA-TASK-020 | Implement M10-006 transform metadata compatibility and golden-report tests | Verification and Validation Engineer / Core Software Engineer | Issue #137, M10 metadata design, current mappings, runtime compatibility rules | Additive `transform_steps` metadata, compatibility/golden-report tests, traceability updates, pipeline report | Implementation Gate | Complete locally |

## Approval Gates

| Gate | Trigger | Required Approver | Evidence Needed | Status |
|---|---|---|---|---|
| M10 issue creation approval | Before creating GitHub issues for transform metadata | User / Project Coordinator | M10 proposal, requirements, risk, traceability, issue placeholders | Passed |
| M10 implementation approval | Before editing code for transform metadata | User / Project Coordinator | GitHub milestone #10, issues #132-#137, user request to start completing open issues through the pipeline | Passed for local implementation |
| Dependency approval | Before adding third-party crates | User / Security Engineer | Dependency reason, license, alternatives, no_std impact | Pending |
| Schema compatibility approval | Before incompatible report/config schema changes | Project Coordinator / V&V Engineer | Migration plan, golden tests, compatibility statement | Pending |
| Hardware/runtime approval | Before live DAQ, HAL, RTOS SDK, target hardware, unsafe FFI, or global setup | User / Technical Director | Environment plan, risk review, rollback plan, validation scope | Pending |

## Risks To Monitor

| Risk | Owner | Mitigation | Review Trigger |
|---|---|---|---|
| Taxonomy overclaiming | Product Architect / Documentation Engineer | Evidence-level metadata and docs that separate implemented, planned, research, and gated support | Transform docs or roadmap changes |
| Report/config compatibility drift | Core Software Engineer / Documentation Engineer | Additive metadata first; golden-report and config compatibility tests | M10 implementation |
| Baseline transforms hiding real failures | Systems Engineer / V&V Engineer | Preserve raw data, require transform metadata and known-answer fixtures | M11 implementation |
| Desktop/embedded event drift | Embedded RTOS Engineer / V&V Engineer | Shared deterministic logic where practical and parity tests | M12 implementation |

## State Updates Required

- Update project state after every milestone stage.
- Update risk register when transform scope, report schema, runtime profiles, or validation claims change.
- Update traceability matrix after requirements, implementation, and tests.
- Update documentation before any public support claim.
- Record durable architecture decisions if M10 changes public config or report strategy.

## Next Role Ticket

You are the Software Architect / Core Software Engineer.

Purpose

Prepare M11/M12 issue-creation or implementation handoff only after explicit approval.

Responsibilities

- Keep changes inside this project.
- Do not add third-party crates without dependency approval.
- Do not create additional GitHub milestones/issues without approval.
- Do not start M11 or M12 issue creation or implementation without explicit user approval.
- Preserve raw waveform data and avoid unsupported algorithm, hardware, runtime, or certification claims.

Deliverables

- Requirements, traceability, risk, docs, tests, and pipeline reports updated for closed issues #132 through #137.
- Next milestone handoff for M11 or M12 only after approval.
- Handoff note.

Expected format to receive deliverables

Use the shared handoff note format from root `AGENTS.md`.

## Stop Conditions

- Stop before incompatible report/config schema changes without schema compatibility approval.
- Stop before adding dependencies.
- Stop before live DAQ, HAL, RTOS SDK, unsafe FFI, target hardware, GUI, plugin runtime, binary package signing, hardware validation, certification, or public production-readiness claims.
