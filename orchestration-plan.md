# Orchestration Plan

Project: FerrisOxide

Project folder: `/Users/kota/Desktop/codexprojects/softwaredev/projects/ferrisoxide`

Execution tier: Tier 2 MVP plus roadmap-controlled follow-on milestones

Current objective: Hold after M14 high-pass baseline correction closure and wait for explicit approval before M15 or new scope.

Current stage: M14 is complete through PR #173; issues #167 through #172 and milestone #14 are closed.

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
- M13 proposal: `docs/v0.11.0-transform-runtime-profile-validation-milestone-proposal.md`
- M14 proposal: `docs/v0.12.0-high-pass-baseline-correction-milestone-proposal.md`
- Selected standards: Rust, signal-processing, open-source library, data-analysis, environment, granularity.

## Milestones

| Milestone | Goal | Owner Role | Entry Gate | Exit Evidence | Status |
|---|---|---|---|---|---|
| M1-M9 | Validated MVP, embedded/no_std foundation, validation, plotting, measurement/evidence, DSL, portable rule package, controller simulation/deployment config | Multiple roles | Historical gates | Implemented requirements WRA-RQ-001 through WRA-RQ-069 and closed M9 | Complete |
| M10 / v0.8.0 | Transform architecture and capability metadata | Software Architect | Human approval and issue creation | Metadata model, existing-transform mappings, compatibility tests, docs, merged PR #138, closed issues #132 through #137, closed milestone #10 | Complete |
| M11 / v0.9.0 | Pointwise and windowed transform MVP | Core Software Engineer / Systems Engineer | M10 architecture accepted and user requested next milestone | Pointwise, baseline, moving median, metadata, raw-preservation tests, docs, PR #147, closed issues #140 through #146, closed milestone #11 | Complete |
| M12 / v0.10.0 | Event and validation transform MVP | Core Software Engineer / V&V Engineer | M10 accepted, M11 compatibility path established, and user approved M12 | Event records, Schmitt trigger, debounce, glitch removal, event validation, fixtures, docs, PR #156, closed issues #149 through #155, closed milestone #12 | Complete |
| M13 / v0.11.0 | Transform runtime-profile validation | Software Architect / Core Software Engineer / V&V Engineer | M12 closed and user approved continuing | Runtime-profile validator API, timing-evidence checks, waveform/event metadata rejection tests, docs guardrails, PR #164, closed issues #158 through #163, closed milestone #13 | Complete |
| M14 / v0.12.0 | High-pass baseline correction | Systems Engineer / Core Software Engineer / V&V Engineer | M13 closed and user approved continuing | `high_pass_baseline` config, first-order high-pass recurrence, invalid timing checks, metadata tests, CLI/config guardrails, docs, traceability, PR, closed issues, closed milestone | Complete through PR #173; milestone #14 closed |

## Zoom-Level Plan

| Stage | Expected Level | Required Artifacts | Abstraction Review Needed |
|---|---:|---|---|
| M10 architecture | 1-3 | Capability model, metadata fields, runtime profiles, compatibility path, tests | Yes |
| M10 implementation | 3-5 | Files, structs/enums, config adapters, report fields, tests | Yes |
| M11 implementation | 3-5 | Transform modules, config validation, CLI/report integration, fixtures | Yes |
| M12 implementation | 3-5 | Event records, validation records, known-answer fixtures, parity tests | Yes |
| M13 implementation | 3-5 | Runtime validator module, structured errors, timing evidence, transform metadata tests, docs | Yes |
| M14 implementation | 3-5 | Filter enum/config wiring, high-pass recurrence, invalid timing tests, metadata tests, CLI/config coverage, export guardrail test, docs | Yes |

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
| WRA-TASK-021 | Approve M11 and create GitHub issues | Project Coordinator / GitHub Maintainer Specialist | M11 proposal and placeholders M11-001 through M11-007 | GitHub milestone #11 and issues #140 through #146 | Human Approval Gate | Complete |
| WRA-TASK-022 | Implement M11 pointwise/windowed transform MVP | Core Software Engineer / Systems Engineer / V&V Engineer / Documentation Engineer | Issues #140 through #146, M10 metadata model | `crates/ferrisoxide-core`, CLI config test, `examples/m11-transform-config.toml`, docs, traceability, and pipeline report | Implementation/Release Gate | Complete |
| WRA-TASK-023 | Approve M12 and create GitHub issues | Project Coordinator / GitHub Maintainer Specialist | M12 proposal and placeholders M12-001 through M12-007 | GitHub milestone #12 and issues #149 through #155 | Human Approval Gate | Complete |
| WRA-TASK-024 | Implement M12 event/validation transform MVP | Core Software Engineer / V&V Engineer / Documentation Engineer | Issues #149 through #155, M10/M11 metadata model | `crates/ferrisoxide-core/src/event.rs`, config/report/CLI integration, rule-engine Schmitt primitive, examples, docs, traceability, and pipeline report | Implementation/Release Gate | Complete through PR #156 |
| WRA-TASK-025 | Create M13 runtime-profile validation proposal | Software Architect / Project Coordinator | M10/M12 known gaps, transform metadata model, user approval to continue | `docs/v0.11.0-transform-runtime-profile-validation-milestone-proposal.md`; WRA-RQ-087 through WRA-RQ-092 | Requirements Gate | Complete |
| WRA-TASK-026 | Approve M13 and create GitHub issues | Project Coordinator / GitHub Maintainer Specialist | M13 proposal and placeholders M13-001 through M13-006 | GitHub milestone #13 and issues #158 through #163 | Human Approval Gate | Complete |
| WRA-TASK-027 | Implement M13 runtime-profile validator | Core Software Engineer / V&V Engineer / Documentation Engineer | Issues #158 through #163, M10 metadata model, M12 event metadata | `crates/ferrisoxide-core/src/runtime_profile.rs`, docs, tests, traceability, and pipeline report | Implementation/Release Gate | Complete through PR #164 |
| WRA-TASK-028 | Create M14 high-pass baseline correction proposal | Systems Engineer / Project Coordinator | Deferred WRA-RQ-078, M11/M13 metadata and runtime-profile model, user approval to continue | `docs/v0.12.0-high-pass-baseline-correction-milestone-proposal.md`; WRA-RQ-093 through WRA-RQ-098 | Requirements Gate | Complete locally |
| WRA-TASK-029 | Approve M14 and create GitHub issues | Project Coordinator / GitHub Maintainer Specialist | M14 proposal and placeholders M14-001 through M14-006 | GitHub milestone #14 and issues #167 through #172 | Human Approval Gate | Complete |
| WRA-TASK-030 | Implement M14 high-pass baseline correction | Core Software Engineer / V&V Engineer / Documentation Engineer | Issues #167 through #172, existing `[[filters]]` config, M10 metadata model, M13 runtime-profile guardrails | `crates/ferrisoxide-core/src/filter.rs`, config/CLI tests, example config, docs, traceability, risk, and pipeline report | Implementation/Release Gate | Complete through PR #173 |

## Approval Gates

| Gate | Trigger | Required Approver | Evidence Needed | Status |
|---|---|---|---|---|
| M10 issue creation approval | Before creating GitHub issues for transform metadata | User / Project Coordinator | M10 proposal, requirements, risk, traceability, issue placeholders | Passed |
| M10 implementation approval | Before editing code for transform metadata | User / Project Coordinator | GitHub milestone #10, issues #132-#137, user request to start completing open issues through the pipeline | Passed for local implementation |
| M11 issue creation and implementation approval | Before creating GitHub issues and editing code for pointwise/windowed transforms | User / Project Coordinator | User request to continue the pipeline with the next milestone; M11 proposal and M10 closure evidence | Passed |
| M12 issue creation and implementation approval | Before creating GitHub issues and editing code for event/validation transforms | User / Project Coordinator | User message "M12 approved" on 2026-06-01; M12 proposal, M10/M11 closure evidence | Passed |
| M13 issue creation and implementation approval | Before creating GitHub issues and editing code for runtime-profile validation | User / Project Coordinator | User approved continuing after M12 closure on 2026-06-01; M13 proposal, M12 closure evidence | Passed for planning and issue creation |
| M14 issue creation and implementation approval | Before creating GitHub issues and editing code for high-pass baseline correction | User / Project Coordinator | User approved continuing after M13 closure on 2026-06-01; M14 proposal, M13 closure evidence | Passed for planning, issue creation, and implementation |
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
| Runtime-profile validation overclaim | Software Architect / Documentation Engineer | Keep M13 scoped to metadata rejection evidence and require separate gates for runtime execution, hardware, or certification claims | M13 implementation |
| High-pass baseline correction hiding failures | Systems Engineer / V&V Engineer | Preserve raw data, require explicit config, reject invalid timing/cutoff values, and document phase/edge behavior | M14 implementation |

## State Updates Required

- Update project state after every milestone stage.
- Update risk register when transform scope, report schema, runtime profiles, or validation claims change.
- Update traceability matrix after requirements, implementation, and tests.
- Update documentation before any public support claim.
- Record durable architecture decisions if M10 changes public config or report strategy.

## Next Role Ticket

You are the Project Orchestrator / Core Software Engineer.

Purpose

Hold after M14 high-pass baseline correction closure and wait for explicit approval before M15 or new scope.

Responsibilities

- Keep changes inside this project.
- Do not add third-party crates without dependency approval.
- Do not create additional GitHub milestones/issues beyond M14 without approval.
- Do not start M15 or hardware/runtime work without explicit user approval.
- Preserve raw waveform data and avoid unsupported algorithm, hardware, runtime, or certification claims.

Deliverables

- M14 high-pass baseline correction is implemented, validated, merged in PR #173, and closed with milestone #14.
- High-pass baseline correction remains desktop-only and did not add rule-package export, live DAQ, HAL/RTOS, target hardware, dependencies, or certification claims.
- Handoff note.

Expected format to receive deliverables

Use the shared handoff note format from root `AGENTS.md`.

## Stop Conditions

- Stop before incompatible report/config schema changes without schema compatibility approval.
- Stop before adding dependencies.
- Stop before live DAQ, HAL, RTOS SDK, unsafe FFI, target hardware, GUI, plugin runtime, binary package signing, hardware validation, certification, or public production-readiness claims.
