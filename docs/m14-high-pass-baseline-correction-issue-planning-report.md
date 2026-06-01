# M14 High-Pass Baseline Correction Issue Planning Report

Date: 2026-06-01

Status: GitHub milestone #14 and issues #167 through #172 created after M13 closure and explicit user approval to continue; local implementation and validation are complete, with PR/CI/closure pending.

## Scope

This report converts the M14 high-pass baseline correction proposal into issue-ready work.

It intentionally stops before dependency changes, broad filter-family work, live DAQ work, HAL/RTOS work, rule-package export expansion, binary packaging, signing, hardware validation, or certification claims.

## Planned Milestone

| Milestone | Version | Proposal | Status |
|---|---|---|---|
| M14 | v0.12.0 | `docs/v0.12.0-high-pass-baseline-correction-milestone-proposal.md` | GitHub milestone #14 open with issues #167 through #172 |

## Planned GitHub Issues

| Placeholder | Title | Requirement Links |
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
| Intake Gate | Pass | Deferred WRA-RQ-078 and user approval after M13 closure. | Project Coordinator |
| Requirements Gate | Pass for proposal | WRA-RQ-093 through WRA-RQ-098 added as proposed requirements. | Software Architect |
| Scope Gate | Pass locally | Excludes dependencies, broad filter families, live DAQ, HAL/RTOS, target hardware, rule-package export expansion, GUI, signing, and certification claims. | Project Orchestrator |
| Human Approval Gate | Pass for M14 issue creation and implementation | User approved continuing after M13 closure on 2026-06-01. | Project Coordinator |
| Issue Planning Gate | Pass | GitHub milestone #14 and issues #167 through #172 created. | GitHub Maintainer Specialist |
| Implementation Gate | Pass locally | Code, docs, tests, traceability, risk, and pipeline evidence are complete in the M14 implementation branch. | Core Software Engineer |
| Testing Gate | Pass locally | Focused M14 tests, full workspace tests, clippy, formatting, diff check, and local Markdown link scan pass locally. | Test Automation Engineer |
| Release Gate | Pending | PR and required `rust` CI pending. | GitHub Maintainer Specialist |
| Community Gate | Pending | Issue and milestone closure pending. | Project Coordinator |

## Stop Conditions

Stop and ask for approval before:

- creating additional GitHub milestones or issues beyond M14
- starting M15 implementation or changing M14 scope
- adding or changing dependencies
- changing public report/config schema incompatibly
- adding rule-package or deployment-package transform export support
- adding live DAQ, vendor SDK, HAL, RTOS SDK, unsafe FFI, target hardware execution, or global setup
- claiming hardware validation, real-time readiness, certification, signing, authentication, or production controller readiness

## Hand-Off Note

Role: Project Orchestrator / GitHub Maintainer Specialist
Goal: Convert M14 proposal into issue-ready work.
Files changed: This report plus proposal, requirements, traceability, risk, roadmap, orchestration, and project state.
Checks run: Documentation and code-path inspection.
Status: Local implementation and validation complete; GitHub milestone #14 and issues #167 through #172 remain open until PR merge.
Known gaps: PR, required `rust` CI, issue closure, and milestone closure remain pending.
Next recommended step: Open the M14 implementation PR and merge after required CI passes.
