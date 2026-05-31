# M8-006 Shared Rule Engine Pipeline Report

Date: 2026-05-31

Repository: `kota-wilson/ferrisoxide`

Branch: `feature/m8-006-shared-rule-engine`

Issue: #73, `M8-006 Add shared rule execution engine`

Requirement: WRA-RQ-048

Owner Roles: Software Architect / Core Software Engineer / Embedded RTOS Engineer / Verification and Validation Engineer

## Objective

Move criteria execution semantics into one shared rule engine so desktop analysis and embedded-compatible host tests do not maintain separate rule behavior.

## Scope Boundaries

In scope:

- New local `ferrisoxide-rule-engine` workspace crate.
- Criteria evaluation over caller-provided time/sample slices.
- Pass/fail result and measurement evidence records compatible with existing report paths.
- Desktop `ferrisoxide-core` adapter from waveform/config criteria types into the shared engine.
- Embedded-compatible host test that evaluates fixed slices through the shared engine.
- Dependency review showing no new third-party crates.

Out of scope:

- no_std rule-engine compatibility claim; owned by #72.
- Exact desktop-vs-embedded parity fixtures; owned by #74.
- Rule package binary serialization, signing, runtime package loading, DAQ integration, HALs, RTOS SDKs, hardware control, production readiness, hardware qualification, or certification claims.
- New criteria, new unit conversions, shorthand DSL parsing, or report schema migration.

## Stage Log

| Stage | Gate | Decision | Artifact / Evidence | Residual Risk | Next Owner |
|---|---|---|---|---|---|
| Intake | Intake Gate | Pass | Issue #73 in milestone #8 requests a shared rule execution engine. | None for issue selection. | Project Orchestrator |
| Project Creation | Project Creation Gate | Not Applicable | Existing FerrisOxide repository and M8 project package already exist. | No new project package needed. | Project Coordinator |
| Project Orchestration | Orchestration Gate | Pass | #73 follows schema/export/manifest work and precedes no_std boundary plus parity tests. | #72 and #74 remain open. | Project Orchestrator |
| Research | Research Gate | Pass | Reviewed existing `ferrisoxide-core` criteria evaluator, `ferrisoxide-measurements`, `ferrisoxide-embedded`, rule package docs, and issue #73. | Future no_std work may require allocation changes. | Software Architect |
| Requirements | Requirements Gate | Pass | WRA-RQ-048 updated in `requirements.md` and `traceability-matrix.md`. | Requirement remains local until PR/CI/merge. | Verification and Validation Engineer |
| Architecture | Architecture Gate | Pass | Shared semantics live in `ferrisoxide-rule-engine`; `ferrisoxide-core` is an adapter; embedded-compatible tests call the engine directly over slices. | Exact parity fixtures remain #74. | Software Architect |
| Abstraction Review | Granularity Gate | Pass | Crate, files, adapter boundary, tests, and non-goals are named at file/component level. | no_std constraints need next-issue design detail. | Abstraction Review Engineer |
| Approval | Human Approval Gate | Pass | User approved continuing M8 issues through PR pipeline. | No new dependency approval required because no new third-party crate is added. | Project Coordinator |
| Dependency | Dependency Gate | Pass | `docs/dependency-review.md` records local crate dependencies only. | no_std feature or allocator changes must be reviewed in #72. | Security Engineer |
| Implementation | Implementation Gate | Pass locally | Added `crates/ferrisoxide-rule-engine`; rewired `crates/ferrisoxide-core/src/analysis.rs`; added embedded-compatible host test in `crates/ferrisoxide-embedded/src/lib.rs`. | Engine currently uses owned strings/vectors; no_std boundary remains #72. | Core Software Engineer |
| Testing | Testing Gate | Pass locally | `cargo tree -p ferrisoxide-rule-engine`; targeted rule-engine/core/embedded tests; workspace tests; clippy; diff check passed. | GitHub CI pending until PR. | Test Automation Engineer |
| V&V | V&V Gate | Pass locally | Existing golden tests preserve desktop report evidence; embedded-compatible test proves fixed slices call same engine. | Exact desktop-vs-embedded fixture parity remains #74. | Verification and Validation Engineer |
| QA | QA Gate | Pass locally | Public CLI/report behavior is preserved while implementation ownership moves into the shared engine. | Reviewers should inspect that old core evaluator was not left active. | QA Engineer |
| Security | Security Gate | Pass | No new third-party crates, file I/O, parsing, plotting, network behavior, SDK/HAL, unsafe code, signing, or binary format added. | Future runtime loading/signing remains separately gated. | Security Engineer |
| Performance | Performance Gate | Pass locally | Adapter allocates criterion/channel vectors once per evaluation, matching existing desktop allocation style; no performance claim added. | Allocation-free embedded path remains #72. | Performance Engineer |
| Documentation | Documentation Gate | Pass locally | README, architecture, dependency review, requirements, traceability, risk register, project state, validation log, and this report updated. | Rule package format docs still point to #72/#74 for runtime readiness. | Documentation Engineer |
| Code Review | Code Review Gate | Pass locally | Local review confirmed desktop core delegates to `evaluate_rule_set`, the old active core evaluator was removed, and embedded-compatible tests exercise the shared crate. | External review occurs through protected PR. | Code Reviewer |
| Evaluation | Evaluation Gate | Pass locally | Acceptance criteria mapped below and local validation passed. | Release readiness depends on PR and required CI. | Evaluation Engineer |
| Release | Release Gate | Blocked until PR | Branch must pass final checks, then PR must pass required `rust` CI. | GitHub CI may find environment-specific issues. | GitHub Maintainer Specialist |
| Community | Community Gate | Blocked until PR | PR body should include `Fixes #73`; issue closes after protected merge. | M8 milestone remains open for #72 and #74. | Community Engineering Lead |
| Retrospective | Retrospective Gate | Pass locally | Lesson: extract behavior first, then harden no_std and parity boundaries in smaller follow-up PRs. | Update if PR review changes architecture. | Project Coordinator |

## Acceptance Criteria Mapping

| Acceptance Criterion | Implementation |
|---|---|
| Shared rule-engine semantics exist. | `crates/ferrisoxide-rule-engine/src/lib.rs` evaluates criteria over `RuleWaveform` slices and emits result plus measurement evidence. |
| Desktop/CLI path uses the shared engine. | `crates/ferrisoxide-core/src/analysis.rs` maps core waveform and criteria types into `evaluate_rule_set`. CLI analysis uses `ferrisoxide-core`, so it inherits the shared path. |
| Embedded-compatible path uses the same semantics. | `crates/ferrisoxide-embedded/src/lib.rs` host test `shared_rule_engine_evaluates_embedded_compatible_slices` evaluates fixed slices through `ferrisoxide-rule-engine`. |
| Existing report behavior is preserved. | Existing `ferrisoxide-core` unit tests and exact golden criteria/report tests continue to pass. |
| No desktop-only scope is added to embedded runtime behavior. | `ferrisoxide-embedded` uses the engine as a dev-dependency for host tests; runtime dependencies remain `ferrisoxide-signal` only. |
| no_std and parity remain correctly scoped. | This report, README, architecture docs, and project state keep #72 and #74 as separate gates. |

## Validation Commands

| Command | Result | Notes |
|---|---|
| `cargo tree -p ferrisoxide-rule-engine` | Passed | Shows local `ferrisoxide-measurements` dependency only. |
| `cargo tree -p ferrisoxide-embedded` | Passed | Runtime dependency remains `ferrisoxide-signal`; `ferrisoxide-rule-engine` appears only under dev-dependencies for host tests. |
| `cargo test -p ferrisoxide-rule-engine` | Passed | 4 shared engine tests passed plus doctests. |
| `cargo test -p ferrisoxide-core` | Passed | 55 unit tests, 15 criteria/golden tests, 1 CSV fixture test, and doctests passed. |
| `cargo test -p ferrisoxide-embedded` | Passed | 5 embedded tests passed, including shared-engine fixed-slice coverage. |
| `cargo fmt --check` | Passed | Formatting is clean. |
| `cargo test --workspace` | Passed | 128 workspace tests passed across CLI, core, embedded, measurements, plot, rule engine, rule schema, signal, and doctests. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |
| `git diff --check` | Passed | No whitespace errors. |

## Review Notes

- `ferrisoxide-core/src/analysis.rs` now owns report-facing adapter types and conversion only; criteria semantics live in `ferrisoxide-rule-engine`.
- `ferrisoxide-rule-engine` intentionally avoids CSV parsing, TOML parsing, report rendering, plotting, file I/O, DAQ/controller I/O, HALs, SDKs, unsafe code, and certification claims.
- The engine currently uses owned result strings and vectors. Removing heap requirements is intentionally left to M8-007 / issue #72.

## Hand-Off Note

Role: Core Software Engineer / Verification and Validation Engineer
Goal: Implement M8-006 shared rule execution engine.
Files changed: `Cargo.toml`, `Cargo.lock`, `README.md`, `crates/ferrisoxide-rule-engine/`, `crates/ferrisoxide-core/Cargo.toml`, `crates/ferrisoxide-core/src/analysis.rs`, `crates/ferrisoxide-embedded/Cargo.toml`, `crates/ferrisoxide-embedded/src/lib.rs`, `docs/architecture.md`, `docs/dependency-review.md`, `docs/validation-log.md`, `docs/m8-006-shared-rule-engine-pipeline-report.md`, `requirements.md`, `traceability-matrix.md`, `risk-register.md`, `project-state.md`.
Checks run: `cargo tree -p ferrisoxide-rule-engine`; `cargo tree -p ferrisoxide-embedded`; `cargo test -p ferrisoxide-rule-engine`; `cargo test -p ferrisoxide-core`; `cargo test -p ferrisoxide-embedded`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Pass locally; PR, required CI, merge, and issue #73 closure pending.
Known gaps: no_std rule-engine boundary, allocation-free embedded execution, and exact desktop-vs-embedded parity fixtures remain M8 follow-up issues.
Next recommended step: Run final validation, open a protected PR with `Fixes #73`, merge after required CI, then implement M8-007 / issue #72.
