# Next Milestones Issue Planning Report

Date: 2026-06-01

Status: M10 GitHub milestone and issues are closed through PR #138. M11 GitHub milestone #11 and issues #140 through #146 are closed through PR #147. M12 milestone #12 and issues #149 through #155 are closed through PR #156. M13 milestone #13 and issues #158 through #163 are closed through PR #164; PR #165 records M13 release/community closure. M14 milestone #14 and issues #167 through #172 are closed through PR #173.

## Scope

This report converts the transform taxonomy and current FerrisOxide project state into local issue placeholders for M10 through M14.

It intentionally stops before external GitHub actions, implementation, dependency changes, live DAQ work, HAL/RTOS work, binary packaging, signing, hardware validation, or certification claims.

## Planned Milestones

| Milestone | Version | Proposal | Status |
|---|---|---|---|
| M10 | v0.8.0 | `docs/v0.8.0-transform-architecture-milestone-proposal.md` | Complete; PR #138 merged and milestone #10 closed |
| M11 | v0.9.0 | `docs/v0.9.0-pointwise-windowed-transform-mvp-milestone-proposal.md` | GitHub milestone #11 closed |
| M12 | v0.10.0 | `docs/v0.10.0-event-validation-transform-milestone-proposal.md` | GitHub milestone #12 closed |
| M13 | v0.11.0 | `docs/v0.11.0-transform-runtime-profile-validation-milestone-proposal.md` | GitHub milestone #13 closed |
| M14 | v0.12.0 | `docs/v0.12.0-high-pass-baseline-correction-milestone-proposal.md` | Complete through PR #173; milestone #14 closed |

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

## M13 GitHub Issues

| Issue | Title | Requirement Links |
|---|---|---|
| #158 / M13-001 | Add transform runtime-profile validator API and structured error model | WRA-RQ-087 |
| #159 / M13-002 | Validate sample-timing evidence for runtime-profile exposure | WRA-RQ-089 |
| #160 / M13-003 | Add runtime-profile tests for waveform transform metadata | WRA-RQ-088 |
| #161 / M13-004 | Add runtime-profile tests for event and validation transform metadata | WRA-RQ-090 |
| #162 / M13-005 | Document rule-package, deployment, and legacy export guardrails | WRA-RQ-091 |
| #163 / M13-006 | Update traceability, risk, and pipeline evidence for M13 closure | WRA-RQ-092 |

## M14 Planned GitHub Issues

| Issue | Title | Requirement Links |
|---|---|---|
| #167 / M14-001 | Add config/model support for high-pass baseline correction | WRA-RQ-093 |
| #168 / M14-002 | Implement first-order high-pass baseline transform and timing validation | WRA-RQ-094, WRA-RQ-095 |
| #169 / M14-003 | Add synthetic drift, raw-preservation, and metadata tests | WRA-RQ-094, WRA-RQ-096 |
| #170 / M14-004 | Add CLI/config and rule-package export guardrail coverage | WRA-RQ-097 |
| #171 / M14-005 | Document high-pass behavior, metadata, and runtime-profile limits | WRA-RQ-098 |
| #172 / M14-006 | Update traceability, risk, and pipeline evidence for M14 closure | WRA-RQ-098 |

## Gate Decisions

| Gate | Decision | Evidence | Next Owner |
|---|---|---|---|
| Intake Gate | Pass | User supplied transform taxonomy and requested next milestones. | Project Coordinator |
| Issue Planning Gate | Pass for M10 | GitHub milestone #10 and issues #132 through #137 were created for M10 and are now closed. | GitHub Maintainer Specialist |
| Issue Planning Gate | Pass for M12 | GitHub milestone #12 and issues #149 through #155 were created after explicit M12 approval. | GitHub Maintainer Specialist |
| Requirements Gate | Pass for proposal | WRA-RQ-070 through WRA-RQ-098 added as proposed or implemented requirements. | Software Architect |
| Scope Gate | Pass locally | Dependencies, live DAQ, HAL/RTOS, signing, hardware validation, certification, and M15+ work remain gated. | Project Orchestrator |
| Human Approval Gate | Pass for M10 issue creation | User approved M10 issue creation on 2026-06-01. | Project Coordinator |
| Human Approval Gate | Pass for M11 issue creation and implementation | User requested continuing the pipeline with the next milestone on 2026-06-01. | Project Coordinator |
| Human Approval Gate | Pass for M12 issue creation and implementation | User approved M12 on 2026-06-01. | Project Coordinator |
| Human Approval Gate | Pass for M13 planning and issue creation | User approved continuing after M12 closure on 2026-06-01. | Project Coordinator |
| Human Approval Gate | Pass for M14 planning, issue creation, and implementation | User approved continuing after M13 closure on 2026-06-01. | Project Coordinator |
| Issue Planning Gate | Pass for M13 | GitHub milestone #13 and issues #158 through #163 created. | GitHub Maintainer Specialist |
| Issue Planning Gate | Pass for M14 | GitHub milestone #14 and issues #167 through #172 created, then closed through PR #173. | GitHub Maintainer Specialist |
| Implementation Gate | Pass for M14 | `high_pass_baseline` config/model support, first-order transform, tests, docs, traceability, risk, and pipeline evidence merged in PR #173. | Core Software Engineer |
| Testing Gate | Pass for M14 | Focused M14 tests, full workspace tests, clippy, formatting, diff check, local Markdown link scan, and PR #173 protected `rust` CI pass. | Test Automation Engineer |
| Release Gate | Pass for M14 | PR #173 merged after required `rust` CI passed; squash commit `a17cd4c0ae7af5ab768688c9301484e5eb4799cf`. | GitHub Maintainer Specialist |
| Community Gate | Pass for M14 | Issues #167 through #172 closed and milestone #14 closed with 6 closed issues and 0 open issues. | Project Coordinator |
| Implementation Gate | Pass for M12 | M12 event/validation implementation, examples, and docs merged in PR #156. | Core Software Engineer |
| Implementation Gate | Pass for M13 | M13 runtime-profile validation implementation, docs, and tests merged in PR #164. | Core Software Engineer |
| Release Gate | Pass for M12 | PR #156 merged after required `rust` CI passed; squash commit `a4885578de9d136cd8df213e1da489a7232cf702`. | GitHub Maintainer Specialist |
| Community Gate | Pass for M12 | Issues #149 through #155 closed and milestone #12 closed with 8 closed items and 0 open items. | Project Coordinator |
| Release Gate | Pass for M13 | PR #164 merged after required `rust` CI passed; squash commit `ae0366dcd20a81a71262f38d2409dc2b85774051`. | GitHub Maintainer Specialist |
| Community Gate | Pass for M13 | Issues #158 through #163 closed and milestone #13 closed with 6 closed items and 0 open items. | Project Coordinator |
| Release Gate | Pass for M11 | PR #147 merged after required `rust` CI passed; squash commit `793a2ab1323526b2695fa7b59a1246f2e29d9c43`. | GitHub Maintainer Specialist |
| Community Gate | Pass for M11 | Issues #140 through #146 are closed and milestone #11 is closed with 8 closed items and 0 open items. | Project Coordinator |

## Stop Conditions

Stop and ask for approval before:

- creating additional GitHub milestones or issues beyond M14
- starting M15 implementation or changing M14 scope
- adding or changing dependencies
- changing public report schema incompatibly
- adding live DAQ, vendor SDK, HAL, RTOS SDK, unsafe FFI, target hardware execution, or global setup
- claiming hardware validation, real-time readiness, certification, signing, authentication, or production controller readiness

## Hand-Off Note

Role: Project Orchestrator / GitHub Maintainer Specialist
Goal: Convert next milestone proposals into local issue placeholders.
Files changed: This report, milestone proposals, requirements, traceability, risk, orchestration, README, architecture, and project state.
Checks run: Documentation and traceability inspection.
Status: M10, M11, M12, M13, and M14 complete; milestone #14 is closed with issues #167 through #172 closed by PR #173.
Known gaps: Runtime loaders, hardware evidence, certification evidence, and M15+ work remain separately gated.
Next recommended step: Hold before M15 or new scope until explicit approval.
