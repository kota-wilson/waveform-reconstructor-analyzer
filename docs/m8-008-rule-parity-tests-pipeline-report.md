# M8-008 Rule Parity Tests Pipeline Report

Date: 2026-05-31

Repository: `kota-wilson/ferrisoxide`

Branch: `feature/m8-008-rule-parity-tests`

Issue: #74, `M8-008 Add desktop-vs-embedded parity tests`

Requirement: WRA-RQ-050

Owner Roles: Verification and Validation Engineer / Core Software Engineer / Embedded RTOS Engineer

## Objective

Prove that the desktop core path and embedded-compatible borrowed-rule path produce exact matching portable evidence fields from the same waveform and same FerrisOxide Rule Package.

## Scope Boundaries

In scope:

- Add `tests/parity/waveform_001.csv`.
- Add `tests/parity/rules_001.toml`.
- Add `tests/parity/expected_result.json`.
- Add an integration test that parses the rule package, evaluates the desktop path through `ferrisoxide-core`, evaluates the embedded-compatible path through `evaluate_borrowed_rule`, and compares portable evidence exactly.
- Document the schema reason for excluding desktop-only human-readable reason text and method-context detail from parity comparison.

Out of scope:

- Hardware qualification, certification evidence, runtime package loaders, `rules.bin`, signing, DAQ integration, hardware HALs, RTOS SDKs, QEMU boot images, or target hardware execution.
- New criteria behavior, new measurement types, new report schema fields, or CLI behavior changes.

## Stage Log

| Stage | Gate | Decision | Artifact / Evidence | Residual Risk | Next Owner |
|---|---|---|---|---|---|
| Intake | Intake Gate | Pass | Issue #74 requests desktop-vs-embedded parity tests with `tests/parity/` fixtures and exact expected JSON. | None for issue selection. | Project Orchestrator |
| Project Creation | Project Creation Gate | Not Applicable | Existing FerrisOxide repository and M8 project package already exist. | No new project package needed. | Project Coordinator |
| Project Orchestration | Orchestration Gate | Pass | #74 follows schema, export, manifest, shared engine, and no_std boundary work. | Runtime loaders remain future work. | Project Orchestrator |
| Research | Research Gate | Pass | Reviewed `ferrisoxide-core` analysis adapter, `ferrisoxide-rule-engine` borrowed API, `ferrisoxide-rule-schema`, existing golden tests, and issue #74. | Parity currently covers one focused software fixture. | Software Architect |
| Requirements | Requirements Gate | Pass | WRA-RQ-050 updated in `requirements.md` and `traceability-matrix.md`. | Requirement remains local until PR/CI/merge. | Verification and Validation Engineer |
| Architecture | Architecture Gate | Pass | Fixture files live under `tests/parity/`; integration test lives in `crates/ferrisoxide-core/tests/rule_parity.rs`. | Package-level runtime loader remains future. | Software Architect |
| Abstraction Review | Granularity Gate | Pass | Files, test path, compared fields, schema exclusion, and non-goals are explicit. | More fixtures should be added before broad runtime claims. | Abstraction Review Engineer |
| Approval | Human Approval Gate | Pass | User approved continuing M8 issues through PR pipeline. | No new third-party dependency approval required. | Project Coordinator |
| Dependency | Dependency Gate | Pass | `docs/dependency-review.md` records only a local `ferrisoxide-rule-schema` dev-dependency for tests. | Future runtime deps still require review. | Security Engineer |
| Implementation | Implementation Gate | Pass locally | Added parity fixtures and integration test that compares desktop and embedded-compatible evidence. | Test maps the rule package to borrowed criteria in test code; runtime loader remains future. | Core Software Engineer |
| Testing | Testing Gate | Pass locally | Targeted parity test, workspace tests, clippy, and diff check passed. | GitHub CI pending until PR. | Test Automation Engineer |
| V&V | V&V Gate | Pass locally | Same waveform, same rule package, and same expected JSON drive both paths; pass/fail and evidence fields match exactly. | Hardware/RTOS validation remains out of scope. | Verification and Validation Engineer |
| QA | QA Gate | Pass locally | No CLI behavior, report schema, or user command change. | Reviewers should inspect fixture naming and expected JSON readability. | QA Engineer |
| Security | Security Gate | Pass | No new third-party crates, network behavior, credentials, signing, file overwrite behavior, HALs, SDKs, or unsafe code. | Future runtime package authenticity remains separate. | Security Engineer |
| Performance | Performance Gate | Not Applicable | One small static fixture and integration test; no performance-sensitive path or claim. | Larger parity suites may need runtime control later. | Performance Engineer |
| Documentation | Documentation Gate | Pass locally | README, architecture, dependency review, rule package docs, requirements, traceability, risk register, project state, validation log, and this report updated. | Docs must be rechecked after PR review. | Documentation Engineer |
| Code Review | Code Review Gate | Pass locally | Local review confirmed the test compares exact portable fields and documents the schema reason for excluded desktop-only fields. | External review occurs through protected PR. | Code Reviewer |
| Evaluation | Evaluation Gate | Pass locally | Acceptance criteria mapped below and local validation passed. | Release readiness depends on PR and required CI. | Evaluation Engineer |
| Release | Release Gate | Blocked until PR | Branch must be pushed and required `rust` CI must pass before merge. | GitHub CI may find environment-specific issues. | GitHub Maintainer Specialist |
| Community | Community Gate | Blocked until PR | PR body should include `Fixes #74`; issue closes after protected merge. | Milestone #8 can be checked after merge. | Community Engineering Lead |
| Retrospective | Retrospective Gate | Pass locally | Lesson: compare a portable evidence projection instead of requiring embedded summaries to carry desktop-only reason/context strings. | Add more parity cases when runtime package loaders exist. | Project Coordinator |

## Acceptance Criteria Mapping

| Acceptance Criterion | Implementation |
|---|---|
| `tests/parity` contains waveform fixtures, rule packages, and expected results. | Added `tests/parity/waveform_001.csv`, `tests/parity/rules_001.toml`, and `tests/parity/expected_result.json`. |
| Same waveform and same rule package are used by desktop and embedded-compatible paths. | `rule_parity.rs` parses `rules_001.toml`, parses `waveform_001.csv` once through `SimpleCsvParser`, evaluates the desktop core path from that waveform, and passes slices from the same parsed waveform to the embedded-compatible borrowed-rule path. |
| Pass/fail, measured value, required value, sample index, timestamp, channel, and evidence identifiers match exactly. | `PortableEvidence` includes `criterion_id`, `outcome`, `failed_criterion`, `measurement_id`, `method`, `channel`, `measured_value`, `required_value`, `tolerance_used`, `unit`, `sample_index`, and `timestamp`; desktop and embedded vectors are compared for exact equality and exact expected JSON. |
| Schema differences are documented. | `expected_result.json` includes `schema_note` explaining that desktop reason text and method context are desktop/report-facing details outside the portable evidence projection. |
| Validation passes. | Targeted parity test, workspace tests, clippy, formatting, and diff check pass locally. |

## Validation Commands

| Command | Result | Notes |
|---|---|---|
| `cargo test -p ferrisoxide-core --test rule_parity` | Passed | 1 parity integration test passed. |
| `cargo fmt --check` | Passed | Formatting is clean. |
| `cargo test --workspace` | Passed | Workspace tests passed across CLI, core, embedded, measurements, plot, rule engine, rule schema, signal, and doctests. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |
| `git diff --check` | Passed | No whitespace errors. |

## Review Notes

- The desktop path intentionally uses `ferrisoxide-core::analysis::evaluate_criteria_with_measurements`.
- The embedded-compatible path intentionally uses `ferrisoxide_rule_engine::evaluate_borrowed_rule`.
- The parity projection synthesizes the same measurement ID convention for embedded summaries: `<criterion_id>_measurement`.

## Hand-Off Note

Role: Verification and Validation Engineer / Core Software Engineer
Goal: Implement M8-008 desktop-vs-embedded parity tests.
Files changed: `crates/ferrisoxide-core/Cargo.toml`, `crates/ferrisoxide-core/tests/rule_parity.rs`, `tests/parity/`, README, architecture docs, dependency review, rule package docs, requirements, traceability, risk register, validation log, pipeline report, and project state.
Checks run: `cargo test -p ferrisoxide-core --test rule_parity`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Pass locally; PR, required CI, merge, issue #74 closure, and final M8 open-issue check pending.
Known gaps: Runtime package loaders, binary package serialization, signing, hardware execution, and certification evidence remain out of scope.
Next recommended step: Open a protected PR with `Fixes #74`, wait for required `rust` CI, merge, then confirm no open milestone #8 issues remain.
