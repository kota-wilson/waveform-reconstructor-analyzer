# Next Milestones Issue Planning Report

Date: 2026-06-01

Status: M10 GitHub milestone and issues were created and are now closed through PR #138. M11 GitHub milestone #11 and issues #140 through #146 are open. M12 remains local placeholders.

## Scope

This report converts the transform taxonomy and current FerrisOxide project state into local issue placeholders for M10, M11, and M12.

It intentionally stops before external GitHub actions, implementation, dependency changes, live DAQ work, HAL/RTOS work, binary packaging, signing, hardware validation, or certification claims.

## Planned Milestones

| Milestone | Version | Proposal | Status |
|---|---|---|---|
| M10 | v0.8.0 | `docs/v0.8.0-transform-architecture-milestone-proposal.md` | Complete; PR #138 merged and milestone #10 closed |
| M11 | v0.9.0 | `docs/v0.9.0-pointwise-windowed-transform-mvp-milestone-proposal.md` | GitHub milestone #11 open |
| M12 | v0.10.0 | `docs/v0.10.0-event-validation-transform-milestone-proposal.md` | Proposed locally |

## M10 GitHub Issues

| Issue | Title | Requirement Links |
|---|---|---|
| #132 / M10-001 | Define transform capability matrix and schema vocabulary | WRA-RQ-070 |
| #133 / M10-002 | Add structured transform metadata design | WRA-RQ-071 |
| #134 / M10-003 | Map existing filters and ADC quantization to transform metadata | WRA-RQ-074 |
| #135 / M10-004 | Add runtime profile compatibility rules for transform exposure | WRA-RQ-072 |
| #136 / M10-005 | Update docs from filters-only wording to transform capability wording | WRA-RQ-073 |
| #137 / M10-006 | Add compatibility and golden-report tests for transform metadata | WRA-RQ-071, WRA-RQ-074 |

## M11 GitHub Issues

| Issue | Title | Requirement Links |
|---|---|---|
| #140 / M11-001 | Add pointwise transform config model and compatibility adapter | WRA-RQ-075 |
| #141 / M11-002 | Implement offset, gain, inversion, and clamp transforms | WRA-RQ-075, WRA-RQ-079 |
| #142 / M11-003 | Implement deadband and DC removal transforms | WRA-RQ-075, WRA-RQ-076, WRA-RQ-079 |
| #143 / M11-004 | Implement baseline subtraction and defer high-pass baseline correction | WRA-RQ-076, WRA-RQ-078 |
| #144 / M11-005 | Implement moving median transform | WRA-RQ-077, WRA-RQ-079 |
| #145 / M11-006 | Add transform examples, report-schema notes, and docs | WRA-RQ-080 |
| #146 / M11-007 | Add metadata, raw-preservation, and golden-report tests | WRA-RQ-079 |

## M12 Local Issue Placeholders

| Placeholder | Title | Requirement Links |
|---|---|---|
| M12-001 | Define event record schema and event-transform evidence model | WRA-RQ-081 |
| M12-002 | Implement dual-threshold/Schmitt trigger state transform | WRA-RQ-082 |
| M12-003 | Implement debounce and glitch removal over event/state streams | WRA-RQ-083 |
| M12-004 | Implement edge extraction and bounce detection | WRA-RQ-084 |
| M12-005 | Implement missing/extra pulse, dwell-time, and timeout validation transforms | WRA-RQ-085 |
| M12-006 | Add switch/bounce known-answer fixture suite and docs | WRA-RQ-086 |
| M12-007 | Add desktop-vs-embedded-compatible parity tests where practical | WRA-RQ-086 |

## Gate Decisions

| Gate | Decision | Evidence | Next Owner |
|---|---|---|---|
| Intake Gate | Pass | User supplied transform taxonomy and requested next milestones. | Project Coordinator |
| Issue Planning Gate | Pass for M10 | GitHub milestone #10 and issues #132 through #137 were created for M10 and are now closed. | GitHub Maintainer Specialist |
| Issue Planning Gate | Pass for M11 | GitHub milestone #11 and issues #140 through #146 are open; M12 remains local placeholders. | GitHub Maintainer Specialist |
| Requirements Gate | Pass for proposal | WRA-RQ-070 through WRA-RQ-086 added as proposed requirements. | Software Architect |
| Scope Gate | Pass locally | Implementation, dependencies, M11/M12 external GitHub issue creation, live DAQ, HAL/RTOS, signing, hardware validation, and certification remain gated. | Project Orchestrator |
| Human Approval Gate | Pass for M10 issue creation | User approved M10 issue creation on 2026-06-01. | Project Coordinator |
| Human Approval Gate | Pass for M11 issue creation and implementation | User requested continuing the pipeline with the next milestone on 2026-06-01. | Project Coordinator |
| Implementation Gate | Pass locally for M11 | M11 implementation is local and pending PR flow; no code work has started for M12. | Core Software Engineer |

## Stop Conditions

Stop and ask for approval before:

- creating additional GitHub milestones or issues beyond M11
- starting M12 implementation
- adding or changing dependencies
- changing public report schema incompatibly
- adding live DAQ, vendor SDK, HAL, RTOS SDK, unsafe FFI, target hardware execution, or global setup
- claiming hardware validation, real-time readiness, certification, signing, authentication, or production controller readiness

## Hand-Off Note

Role: Project Orchestrator / GitHub Maintainer Specialist
Goal: Convert next milestone proposals into local issue placeholders.
Files changed: This report, milestone proposals, requirements, traceability, risk, orchestration, README, architecture, and project state.
Checks run: Documentation and traceability inspection.
Status: M10 complete; M11 issues created and local implementation pending PR flow; M12 remains local placeholders.
Known gaps: M11 PR/CI/issue closure/milestone closure remain pending, and M12 GitHub issue creation remains pending.
Next recommended step: Run full validation for M11, open a PR, and close milestone #11 only after checks pass.
