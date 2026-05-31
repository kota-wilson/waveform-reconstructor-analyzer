# M8-002 Rule Package Format Pipeline Report

Date: 2026-05-31

Repository: `kota-wilson/ferrisoxide-signal`

Branch: `feature/m8-002-rule-package-format`

Issue: #71, `M8-002 Define portable rule package format`

Requirements: WRA-RQ-043, WRA-RQ-044

Owner Roles: Software Architect / Documentation Engineer

## Objective

Define the reviewable portable FerrisOxide Rule Package format and document the deployment artifact roles before adding validation, export, checksum, binary, or runtime behavior.

## Scope Boundaries

In scope:

- `docs/rule-package-format.md`.
- Parse-tested `examples/rule-package/rules.toml`.
- Parse-tested `examples/rule-package/rules.json`.
- Artifact role documentation for `rules.toml`, `rules.json`, future `rules.bin`, `manifest.json`, `checksum.txt`, `validation-report.json`, and `qualification-evidence.svg`.
- Embedded consumption subset documentation.
- Traceability, dependency, validation, and project-state updates.

Out of scope:

- Export command.
- Rule package validator.
- Manifest/checksum implementation.
- Binary package format.
- Shared rule execution engine.
- no_std rule-engine compatibility claim.
- GUI, DAQ, HAL, SDK, RTOS production integration, hardware qualification, or certification claim.

## Stage Log

| Stage | Gate | Decision | Artifact / Evidence | Residual Risk | Next Owner |
|---|---|---|---|---|---|
| Intake | Intake Gate | Pass | Issue #71 exists in milestone #8 and depends on #67's schema crate. | None for issue selection. | Project Orchestrator |
| Project Creation | Project Creation Gate | Not Applicable | Existing repository and milestone package already exist. | No new project package needed. | Project Coordinator |
| Project Orchestration | Orchestration Gate | Pass | #71 selected after #67 so validator/export work has a documented package contract. | #68 and later M8 issues remain open. | Project Orchestrator |
| Research | Research Gate | Pass | Reviewed issue #71, ADR-004, v0.6.0 milestone proposal, and `ferrisoxide-rule-schema`. | Later issues may refine fields as implementation pressure appears. | Software Architect |
| Requirements | Requirements Gate | Pass | WRA-RQ-043 and WRA-RQ-044 updated in `requirements.md` and `traceability-matrix.md`. | WRA-RQ-044 remains local until PR/CI/merge. | Software Architect |
| Architecture | Architecture Gate | Pass | Format maps directly to `RulePackage` schema; artifact roles match ADR-004 and milestone proposal. | Manifest/checksum field details remain #70. | Abstraction Review Engineer |
| Abstraction Review | Granularity Gate | Pass | Docs define concrete files, fields, examples, parser tests, embedded subset, non-goals, and next owners. | Do not let #71 absorb validation or export behavior. | Abstraction Review Engineer |
| Approval | Human Approval Gate | Pass | User asked to continue open issues through the pipeline and previously approved PR creation. | None for this scoped documentation/test slice. | Project Coordinator |
| Dependency | Dependency Gate | Pass | Reuses approved `toml` as dev-dependency only for parse-testing `rules.toml`; no new third-party crates. | Future checksum/binary dependencies require fresh review. | Security Engineer |
| Implementation | Implementation Gate | Pass | `docs/rule-package-format.md`, `examples/rule-package/rules.toml`, `examples/rule-package/rules.json`, parse test in `ferrisoxide-rule-schema`. | No semantic validator yet. | Documentation Engineer / Core Software Engineer |
| Testing | Testing Gate | Pass | `cargo tree -p ferrisoxide-rule-schema`; `cargo test -p ferrisoxide-rule-schema`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`. | Protected GitHub CI pending until PR creation. | Verification and Validation Engineer |
| V&V | V&V Gate | Pass | TOML and JSON examples deserialize to equal `RulePackage` values and cover package metadata, channels, units, sample timing, filters, criteria, thresholds, and timing limits. | This verifies format consistency, not rule execution correctness. | V&V Engineer |
| QA | QA Gate | Pass | Documentation marks future artifacts as future and keeps hardware/certification claims out of scope. | Reader confusion may still occur until exporter creates real packages. | QA Engineer |
| Security | Security Gate | Pass | No checksum algorithm, signing, binary serialization, file export, SDK, HAL, or unsafe code added. | Future integrity claims need deterministic tests and security review. | Security Engineer |
| Performance | Performance Gate | Not Applicable | Documentation and examples do not add analysis hot-path behavior. | Export performance remains future work. | Performance Engineer |
| Documentation | Documentation Gate | Pass | `docs/rule-package-format.md` defines artifact roles, schema model, examples, filters, criteria, embedded subset, validation expectations, and non-goals. | Docs may need updates after validator/export implementation. | Documentation Engineer |
| Code Review | Code Review Gate | Pass locally | Local review confirmed examples are parse-tested and docs do not overclaim export/checksum/runtime readiness. | External review occurs through protected PR. | Code Reviewer |
| Evaluation | Evaluation Gate | Pass | Issue #71 acceptance criteria are mapped below. | Remaining M8 work still required for a usable package system. | Evaluation Engineer |
| Release | Release Gate | Blocked until PR | Local branch passes required checks; release requires PR, required `rust` CI, and protected merge. | GitHub CI may find environment-specific issues. | GitHub Maintainer Specialist |
| Community | Community Gate | Blocked until PR | Issue #71 will close via PR body `Fixes #71`. | Milestone #8 remains open after this issue. | Community Engineering Lead |
| Retrospective | Retrospective Gate | Pass locally | Lessons recorded below. | Update if PR review requires changes. | Project Coordinator |

## Acceptance Criteria Mapping

| Acceptance Criterion | Implementation |
|---|---|
| Docs define `rules.toml`. | `docs/rule-package-format.md`; `examples/rule-package/rules.toml`; parse test. |
| Docs define `rules.json`. | `docs/rule-package-format.md`; `examples/rule-package/rules.json`; parse test. |
| Docs define future `rules.bin`. | Artifact role table and embedded subset section identify it as future runtime input, not implemented. |
| Docs define `manifest.json`. | Artifact role table defines future manifest role and defers fields to M8-005. |
| Docs define `checksum.txt`. | Artifact role table defines future checksum role and defers algorithm/mismatch behavior to M8-005. |
| Docs define `validation-report.json`. | Artifact role table defines validation evidence role. |
| Docs define `qualification-evidence.svg`. | Artifact role table defines visual software evidence role. |
| Schema examples include package metadata. | `rules.toml` and `rules.json` include `package` section/object. |
| Schema examples include channel mapping. | Example channel has logical `name` and `source_name`. |
| Schema examples include units. | Examples use `V`, `s`, and `count`. |
| Schema examples include sample-rate assumptions. | `sample_timing` includes nominal sample rate, sample interval, and tolerances. |
| Schema examples include filters. | Examples include moving average and ADC quantization filters. |
| Schema examples include criteria. | Examples include transient-event duration, stable-state duration, and state-transition count criteria. |
| Schema examples include thresholds. | Example channel has low, high, and decision thresholds; criteria include decision thresholds. |
| Schema examples include timing limits. | Duration requirements use unit-bearing `s` values. |
| Embedded consumption subset documented. | `docs/rule-package-format.md` defines `rules.bin`, `manifest.json`, and `checksum.txt` as the future minimal subset. |
| No GUI/DAQ/HAL/SDK/RTOS/certification claim. | Scope and artifact docs explicitly exclude these areas. |

## Validation Commands

| Command | Result | Notes |
|---|---|---|
| `cargo tree -p ferrisoxide-rule-schema` | Passed | Runtime dependency is approved `serde`; dev-dependencies are approved `serde_json` and `toml`. |
| `cargo test -p ferrisoxide-rule-schema` | Passed | 3 schema tests passed, including TOML/JSON example parity. |
| `cargo fmt --check` | Passed | Formatting is clean. |
| `cargo test --workspace` | Passed | 109 tests passed across workspace plus doctests. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |
| `git diff --check` | Passed | No whitespace errors. |

## Review Notes

- `rules.toml` is the human-authored format for review.
- `rules.json` is the automation-friendly representation of the same schema.
- `rules.bin`, `manifest.json`, and `checksum.txt` are documented as future roles only.
- The examples are intentionally software-validation examples and not certified controller release packages.

## Retrospective

What worked:

- Parse-testing the examples prevents documentation drift before validator/export code exists.
- Keeping manifest/checksum/binary details as future roles avoids accidental security or runtime claims.

What to watch:

- M8-003 should validate the same field names and error surfaces documented here.
- M8-004 should export examples that remain compatible with these parse-tested files.

## Hand-Off Note

Role: Software Architect / Documentation Engineer
Goal: Define the initial portable rule package format for issue #71.
Files changed: `README.md`, `Cargo.lock`, `crates/ferrisoxide-rule-schema/Cargo.toml`, `crates/ferrisoxide-rule-schema/README.md`, `crates/ferrisoxide-rule-schema/src/lib.rs`, `docs/rule-package-format.md`, `examples/rule-package/`, `docs/dependency-review.md`, `docs/validation-log.md`, `docs/m8-002-rule-package-format-pipeline-report.md`, `requirements.md`, `traceability-matrix.md`, `risk-register.md`, `project-state.md`.
Checks run: `cargo tree -p ferrisoxide-rule-schema`; `cargo test -p ferrisoxide-rule-schema`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Pass locally; PR/CI/merge pending.
Known gaps: No validator, export command, manifest/checksum implementation, binary package, shared rule engine, no_std rule-engine boundary, or desktop-vs-embedded parity tests yet.
Next recommended step: Open a protected-branch PR with `Fixes #71`, wait for required `rust` CI, merge, then implement M8-003 / issue #68.
