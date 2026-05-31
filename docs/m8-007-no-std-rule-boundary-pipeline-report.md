# M8-007 no_std Rule Boundary Pipeline Report

Date: 2026-05-31

Repository: `kota-wilson/ferrisoxide`

Branch: `feature/m8-007-no-std-rule-boundary`

Issue: #72, `M8-007 Add no_std compatibility boundary`

Requirement: WRA-RQ-049

Owner Roles: Embedded RTOS Engineer / Software Architect / Core Software Engineer / Verification and Validation Engineer

## Objective

Define and verify the no_std-compatible rule execution boundary for embedded rule-package consumption while keeping desktop-only concerns out of the embedded-compatible path.

## Scope Boundaries

In scope:

- Make `crates/ferrisoxide-rule-engine` compile as `#![no_std]`.
- Keep the full desktop evidence API available through `alloc`-backed owned results.
- Add borrowed criterion, borrowed/static error, and compact summary types for basic no-heap embedded-compatible evaluation where practical.
- Verify `ferrisoxide-rule-engine` and `ferrisoxide-embedded` compile for `aarch64-unknown-none`.
- Check constrained dependency trees for desktop-only dependencies.

Out of scope:

- Exact desktop-vs-embedded parity fixtures; owned by #74.
- Rule package binary serialization, runtime package loading, signing, DAQ integration, hardware HALs, target SDKs, RTOS production integration, real-time guarantees, hardware qualification, or certification claims.
- New criteria behavior, new filters, new measurements, or CLI/report schema changes.

## Stage Log

| Stage | Gate | Decision | Artifact / Evidence | Residual Risk | Next Owner |
|---|---|---|---|---|---|
| Intake | Intake Gate | Pass | Issue #72 in milestone #8 requests a no_std rule boundary. | None for issue selection. | Project Orchestrator |
| Project Creation | Project Creation Gate | Not Applicable | Existing FerrisOxide repository and M8 project package already exist. | No new project package needed. | Project Coordinator |
| Project Orchestration | Orchestration Gate | Pass | #72 follows shared engine extraction and precedes exact parity tests. | #74 remains open. | Project Orchestrator |
| Research | Research Gate | Pass | Reviewed `ferrisoxide-rule-engine`, `ferrisoxide-measurements`, `ferrisoxide-embedded`, rule package docs, and issue #72. | Future runtime loaders may need stricter allocator policy. | Software Architect |
| Requirements | Requirements Gate | Pass | WRA-RQ-049 updated in `requirements.md` and `traceability-matrix.md`. | Requirement remains local until PR/CI/merge. | Verification and Validation Engineer |
| Architecture | Architecture Gate | Pass | The engine is `#![no_std]`; owned API uses `alloc`; borrowed API uses borrowed/static summary and error data. | Exact package parity remains #74. | Software Architect |
| Abstraction Review | Granularity Gate | Pass | Crate boundary, functions, public types, checks, and non-goals are named at file/API level. | Runtime binary format and loaders remain unspecified. | Abstraction Review Engineer |
| Approval | Human Approval Gate | Pass | User approved continuing M8 issues through PR pipeline. | No new dependency approval required because no new third-party crate is added. | Project Coordinator |
| Dependency | Dependency Gate | Pass | `docs/dependency-review.md` records no new dependencies and target dependency-tree checks. | Future signing/binary serialization needs fresh review. | Security Engineer |
| Implementation | Implementation Gate | Pass locally | `crates/ferrisoxide-rule-engine/src/lib.rs` adds `#![no_std]`, borrowed criteria, borrowed errors, compact summaries, and no_std-safe rounding. | Borrowed API covers one rule at a time; package loading remains future work. | Core Software Engineer |
| Testing | Testing Gate | Pass locally | Targeted tests, target checks, dependency trees, workspace tests, clippy, and diff check passed. | GitHub CI pending until PR. | Test Automation Engineer |
| V&V | V&V Gate | Pass locally | Acceptance criteria mapped below; no desktop-only parsing, plotting, report, HAL, SDK, or file-I/O dependencies are pulled into the constrained path. | Exact parity fixtures remain #74. | Verification and Validation Engineer |
| QA | QA Gate | Pass locally | No CLI/report behavior changes are introduced; constrained API is additive. | Reviewers should inspect no certification or runtime-readiness overclaim. | QA Engineer |
| Security | Security Gate | Pass | No new dependencies, unsafe code, networking, file I/O, signing, binary format, HAL, SDK, or credential changes. | Future runtime package authenticity remains separate. | Security Engineer |
| Performance | Performance Gate | Pass locally | Borrowed summary API avoids owned criterion/result strings and borrowed-path heap allocation for basic evaluation where practical. | No benchmark claim added. | Performance Engineer |
| Documentation | Documentation Gate | Pass locally | README, architecture, dependency review, rule package docs, requirements, traceability, risk register, project state, validation log, and this report updated. | Docs must be rechecked after PR review. | Documentation Engineer |
| Code Review | Code Review Gate | Pass locally | Local review confirmed borrowed-path error handling no longer allocates owned strings, and target checks pass. | External review occurs through protected PR. | Code Reviewer |
| Evaluation | Evaluation Gate | Pass locally | Acceptance criteria mapped below and local validation passed. | Release readiness depends on PR and required CI. | Evaluation Engineer |
| Release | Release Gate | Blocked until PR | Branch must be pushed and required `rust` CI must pass before merge. | GitHub CI may find environment-specific issues. | GitHub Maintainer Specialist |
| Community | Community Gate | Blocked until PR | PR body should include `Fixes #72`; issue closes after protected merge. | M8 milestone remains open for #74. | Community Engineering Lead |
| Retrospective | Retrospective Gate | Pass locally | Lesson: keep the full desktop evidence API and constrained borrowed API explicit rather than pretending one interface fits all runtimes. | Update if PR review changes API names. | Project Coordinator |

## Acceptance Criteria Mapping

| Acceptance Criterion | Implementation |
|---|---|
| Embedded-compatible rule execution avoids CSV parsing, file I/O, plotting, report generation, hardware HALs, target SDKs, and DAQ/controller I/O. | `ferrisoxide-rule-engine` depends only on local `ferrisoxide-measurements`; target dependency-tree checks show no desktop parser, plotting, report, HAL, SDK, or file-I/O crates in the constrained path. |
| The shared engine preserves a no_std boundary. | `crates/ferrisoxide-rule-engine/src/lib.rs` uses `#![no_std]` and compiles for `aarch64-unknown-none`. |
| Desktop evidence remains available. | Existing `evaluate_rule_set` API remains available and uses `alloc`-backed owned result and measurement records for `ferrisoxide-core`. |
| Basic embedded-compatible evaluation avoids heap requirements where practical. | `evaluate_borrowed_rule` accepts borrowed criterion data and returns `RuleSummary` plus `BorrowedRuleError` values without owned criterion/result strings on basic paths. |
| Existing behavior remains stable. | Existing rule-engine, core, embedded, golden, workspace, and clippy checks pass. |

## Validation Commands

| Command | Result | Notes |
|---|---|---|
| `cargo test -p ferrisoxide-rule-engine` | Passed | 7 rule-engine tests passed, including borrowed summary and borrowed/static error coverage. |
| `cargo check -p ferrisoxide-rule-engine --target aarch64-unknown-none` | Passed | Rule engine compiles for the bare-metal ARM64 target. |
| `cargo check -p ferrisoxide-embedded --target aarch64-unknown-none` | Passed | Embedded adapter crate still compiles for the bare-metal ARM64 target. |
| `cargo tree -p ferrisoxide-rule-engine --target aarch64-unknown-none` | Passed | Shows local `ferrisoxide-measurements` dependency only. |
| `cargo tree -p ferrisoxide-embedded --target aarch64-unknown-none` | Passed | Runtime dependency remains local `ferrisoxide-signal`; dev dependency path includes local `ferrisoxide-rule-engine` only. |
| `cargo fmt --check` | Passed | Formatting is clean. |
| `cargo test --workspace` | Passed | Workspace tests passed across CLI, core, embedded, measurements, plot, rule engine, rule schema, signal, and doctests. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |
| `git diff --check` | Passed | No whitespace errors. |

## Review Notes

- The crate is no_std, but the full evidence API still intentionally uses `alloc` so desktop/core report adapters can keep owned records.
- The constrained path is `evaluate_borrowed_rule`, `BorrowedRuleCriterion`, `BorrowedRuleCriterionCheck`, `RuleSummary`, and `BorrowedRuleError`.
- The borrowed API evaluates one criterion at a time. Package-level iteration and compact binary package loading remain future work.

## Hand-Off Note

Role: Embedded RTOS Engineer / Core Software Engineer / Verification and Validation Engineer
Goal: Implement M8-007 no_std rule boundary.
Files changed: `crates/ferrisoxide-rule-engine/src/lib.rs`, `crates/ferrisoxide-rule-engine/README.md`, README, architecture docs, dependency review, rule package docs, requirements, traceability, risk register, validation log, pipeline report, and project state.
Checks run: `cargo test -p ferrisoxide-rule-engine`; `cargo check -p ferrisoxide-rule-engine --target aarch64-unknown-none`; `cargo check -p ferrisoxide-embedded --target aarch64-unknown-none`; target dependency-tree checks; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Pass locally; PR, required CI, merge, and issue #72 closure pending.
Known gaps: exact desktop-vs-embedded parity fixtures, runtime package loaders, binary package serialization, signing, HAL/SDK integration, and certification evidence remain out of scope.
Next recommended step: Open a protected PR with `Fixes #72`, wait for required `rust` CI, merge, then implement M8-008 / issue #74.
