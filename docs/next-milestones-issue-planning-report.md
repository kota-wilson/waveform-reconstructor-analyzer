# Next Milestones Issue Planning Report

Date: 2026-06-01

Status: M10 GitHub milestone and issues are closed through PR #138. M11 GitHub milestone #11 and issues #140 through #146 are closed through PR #147. M12 milestone #12 and issues #149 through #155 are closed through PR #156.

## Scope

This report converts the transform taxonomy and current FerrisOxide project state into local issue placeholders for M10, M11, and M12.

It intentionally stops before external GitHub actions, implementation, dependency changes, live DAQ work, HAL/RTOS work, binary packaging, signing, hardware validation, or certification claims.

## Planned Milestones

| Milestone | Version | Proposal | Status |
|---|---|---|---|
| M10 | v0.8.0 | `docs/v0.8.0-transform-architecture-milestone-proposal.md` | Complete; PR #138 merged and milestone #10 closed |
| M11 | v0.9.0 | `docs/v0.9.0-pointwise-windowed-transform-mvp-milestone-proposal.md` | GitHub milestone #11 closed |
| M12 | v0.10.0 | `docs/v0.10.0-event-validation-transform-milestone-proposal.md` | GitHub milestone #12 closed |

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

## M12 GitHub Issues

| Issue | Title | Requirement Links |
|---|---|---|
| #149 / M12-001 | Define event record schema and event-transform evidence model | WRA-RQ-081 |
| #150 / M12-002 | Implement dual-threshold/Schmitt trigger state transform | WRA-RQ-082 |
| #151 / M12-003 | Implement debounce and glitch removal over event/state streams | WRA-RQ-083 |
| #152 / M12-004 | Implement edge extraction and bounce detection | WRA-RQ-084 |
| #153 / M12-005 | Implement missing/extra pulse, dwell-time, and timeout validation transforms | WRA-RQ-085 |
| #154 / M12-006 | Add switch/bounce known-answer fixture suite and docs | WRA-RQ-086 |
| #155 / M12-007 | Add desktop-vs-embedded-compatible parity tests where practical | WRA-RQ-086 |

## Gate Decisions

| Gate | Decision | Evidence | Next Owner |
|---|---|---|---|
| Intake Gate | Pass | User supplied transform taxonomy and requested next milestones. | Project Coordinator |
| Issue Planning Gate | Pass for M10 | GitHub milestone #10 and issues #132 through #137 were created for M10 and are now closed. | GitHub Maintainer Specialist |
| Issue Planning Gate | Pass for M12 | GitHub milestone #12 and issues #149 through #155 were created after explicit M12 approval. | GitHub Maintainer Specialist |
| Requirements Gate | Pass for proposal | WRA-RQ-070 through WRA-RQ-086 added as proposed requirements. | Software Architect |
| Scope Gate | Pass locally | Dependencies, live DAQ, HAL/RTOS, signing, hardware validation, certification, and M13+ work remain gated. | Project Orchestrator |
| Human Approval Gate | Pass for M10 issue creation | User approved M10 issue creation on 2026-06-01. | Project Coordinator |
| Human Approval Gate | Pass for M11 issue creation and implementation | User requested continuing the pipeline with the next milestone on 2026-06-01. | Project Coordinator |
| Human Approval Gate | Pass for M12 issue creation and implementation | User approved M12 on 2026-06-01. | Project Coordinator |
| Implementation Gate | Pass for M12 | M12 event/validation implementation, examples, and docs merged in PR #156. | Core Software Engineer |
| Release Gate | Pass for M12 | PR #156 merged after required `rust` CI passed; squash commit `a4885578de9d136cd8df213e1da489a7232cf702`. | GitHub Maintainer Specialist |
| Community Gate | Pass for M12 | Issues #149 through #155 closed and milestone #12 closed with 8 closed items and 0 open items. | Project Coordinator |
| Release Gate | Pass for M11 | PR #147 merged after required `rust` CI passed; squash commit `793a2ab1323526b2695fa7b59a1246f2e29d9c43`. | GitHub Maintainer Specialist |
| Community Gate | Pass for M11 | Issues #140 through #146 are closed and milestone #11 is closed with 8 closed items and 0 open items. | Project Coordinator |

## Stop Conditions

Stop and ask for approval before:

- creating additional GitHub milestones or issues beyond M12
- starting M13 implementation
- adding or changing dependencies
- changing public report schema incompatibly
- adding live DAQ, vendor SDK, HAL, RTOS SDK, unsafe FFI, target hardware execution, or global setup
- claiming hardware validation, real-time readiness, certification, signing, authentication, or production controller readiness

## Hand-Off Note

Role: Project Orchestrator / GitHub Maintainer Specialist
Goal: Convert next milestone proposals into local issue placeholders.
Files changed: This report, milestone proposals, requirements, traceability, risk, orchestration, README, architecture, and project state.
Checks run: Documentation and traceability inspection.
Status: M10, M11, and M12 complete.
Known gaps: No GitHub release tag was published for M12; M13 and hardware/runtime work remain unapproved.
Next recommended step: Hold before M13 or new scope until explicit approval.
