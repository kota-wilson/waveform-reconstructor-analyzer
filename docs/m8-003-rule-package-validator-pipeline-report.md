# M8-003 Rule Package Validator Pipeline Report

Date: 2026-05-31

Repository: `kota-wilson/ferrisoxide-signal`

Branch: `feature/m8-003-rule-package-validator`

Issue: #68, `M8-003 Create rule package validator`

Requirement: WRA-RQ-045

Owner Roles: Core Software Engineer / Verification and Validation Engineer

## Objective

Validate FerrisOxide Rule Packages before export or execution and return structured errors for invalid package definitions.

## Scope Boundaries

In scope:

- In-memory `RulePackage::validate()`.
- Target compatibility validation with `RulePackage::validate_for_target()`.
- String parse helpers for TOML and JSON packages.
- Structured error kind, field, and message values.
- Supplied-string checksum comparison helper.
- Tests for accepted packages and the rejection classes listed in issue #68.

Out of scope:

- Export command.
- Manifest generation.
- Checksum algorithm.
- Binary package format.
- Rule execution engine.
- Runtime controller integration.
- Hardware HAL, DAQ, GUI, RTOS production integration, hardware qualification, or certification claim.

## Stage Log

| Stage | Gate | Decision | Artifact / Evidence | Residual Risk | Next Owner |
|---|---|---|---|---|---|
| Intake | Intake Gate | Pass | Issue #68 exists in milestone #8 and follows #67 schema plus #71 package format. | None for issue selection. | Project Orchestrator |
| Project Creation | Project Creation Gate | Not Applicable | Existing repository and milestone package already exist. | No new project package needed. | Project Coordinator |
| Project Orchestration | Orchestration Gate | Pass | #68 selected after schema and format work so invalid packages can be rejected before #69 export. | #69 and later M8 issues remain open. | Project Orchestrator |
| Research | Research Gate | Pass | Reviewed issue #68, `ferrisoxide-rule-schema`, and `docs/rule-package-format.md`. | Future export may add additional validation context. | Software Architect |
| Requirements | Requirements Gate | Pass | WRA-RQ-045 updated in `requirements.md` and `traceability-matrix.md`. | Requirement remains local until PR/CI/merge. | Software Architect |
| Architecture | Architecture Gate | Pass | Validator remains in `ferrisoxide-rule-schema` and does not alter CLI/core analysis behavior. | no_std boundary remains #72. | Abstraction Review Engineer |
| Abstraction Review | Granularity Gate | Pass | Validation API names, error kinds, tests, and exclusions are concrete and scoped. | Future manifest/checksum implementation must not overclaim this helper. | Abstraction Review Engineer |
| Approval | Human Approval Gate | Pass | User asked to continue open issues through the pipeline and previously approved PR creation. | None for this scoped validation slice. | Project Coordinator |
| Dependency | Dependency Gate | Pass | Reuses approved `serde_json` and `toml` as parser dependencies; no new third-party crates. | Future checksum/binary dependencies require fresh review. | Security Engineer |
| Implementation | Implementation Gate | Pass | `crates/ferrisoxide-rule-schema/src/lib.rs` adds parse helpers, validation report/errors, package validation, target validation, checksum comparison, and tests. | No exporter or runtime consumes the validator yet. | Core Software Engineer |
| Testing | Testing Gate | Pass | `cargo tree -p ferrisoxide-rule-schema`; `cargo test -p ferrisoxide-rule-schema`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`. | Protected GitHub CI pending until PR creation. | Verification and Validation Engineer |
| V&V | V&V Gate | Pass | Tests cover accepted package, missing channel reference, unsupported unit parse, unknown filter parse, unknown criterion parse, invalid timing/sample-rate assumptions, checksum mismatch, incompatible target profile, invalid filter, and invalid criterion. | This validates package structure, not rule execution results. | V&V Engineer |
| QA | QA Gate | Pass | Validation returns structured error fields and does not panic in covered invalid cases. | Additional real-world invalid fixtures may be added with export work. | QA Engineer |
| Security | Security Gate | Pass | No checksum algorithm, signing, binary serialization, file export, SDK, HAL, unsafe code, or runtime loader added. | Future package integrity work needs deterministic tests and security review. | Security Engineer |
| Performance | Performance Gate | Not Applicable | Validator runs on package metadata and definitions, not waveform hot paths. | Large package validation benchmarks are not needed yet. | Performance Engineer |
| Documentation | Documentation Gate | Pass | README, crate README, architecture, rule package format, dependency review, risk register, traceability, validation log, and this report updated. | API docs may need expansion when export uses validator. | Documentation Engineer |
| Code Review | Code Review Gate | Pass locally | Local review checked error taxonomy, scope boundary, and test coverage against issue #68. | External review occurs through protected PR. | Code Reviewer |
| Evaluation | Evaluation Gate | Pass | Issue #68 acceptance criteria are mapped below. | Remaining M8 work still required for a usable package system. | Evaluation Engineer |
| Release | Release Gate | Blocked until PR | Local branch passes required checks; release requires PR, required `rust` CI, and protected merge. | GitHub CI may find environment-specific issues. | GitHub Maintainer Specialist |
| Community | Community Gate | Blocked until PR | Issue #68 will close via PR body `Fixes #68`. | Milestone #8 remains open after this issue. | Community Engineering Lead |
| Retrospective | Retrospective Gate | Pass locally | Lessons recorded below. | Update if PR review requires changes. | Project Coordinator |

## Acceptance Criteria Mapping

| Acceptance Criterion | Implementation |
|---|---|
| Missing channels return structured errors. | `RulePackage::validate()` reports `MissingChannel` for empty package channel list and undefined filter/criterion channel references. |
| Unsupported units return structured errors. | `parse_rule_package_toml()` / `parse_rule_package_json()` classify unsupported unit enum parse errors as `UnsupportedUnit`. |
| Unknown filters return structured errors. | Parse helpers classify unknown filter tags as `UnknownFilter`. |
| Unknown criteria return structured errors. | Parse helpers classify unknown measurement tags as `UnknownCriterion`. |
| Invalid timing/sample-rate assumptions return structured errors. | `validate_sample_timing()` reports `InvalidTimingAssumption`. |
| Checksum mismatch returns structured errors. | `validate_checksum_match()` reports `ChecksumMismatch` when supplied strings differ. |
| Incompatible target profiles return structured errors. | `RulePackage::validate_for_target()` reports `IncompatibleTargetProfile`. |
| Validator tests cover accepted packages. | `validates_accepted_package_for_expected_target`. |
| Validator tests cover rejected packages. | Rejection tests cover every issue-listed invalid class plus invalid filter and criterion parameters. |
| Workspace fmt, tests, clippy, and diff check pass. | Validation commands below. |

## Validation Commands

| Command | Result | Notes |
|---|---|---|
| `cargo tree -p ferrisoxide-rule-schema` | Passed | Runtime dependencies are approved `serde`, `serde_json`, and `toml`; no CLI, plotting, DAQ, HAL, SDK, or runtime dependency appears. |
| `cargo test -p ferrisoxide-rule-schema` | Passed | 12 schema/validator tests passed plus doctests. |
| `cargo fmt --check` | Passed | Formatting is clean. |
| `cargo test --workspace` | Passed | 118 tests passed across workspace plus doctests. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |
| `git diff --check` | Passed | No whitespace errors. |

## Review Notes

- The validator does not execute rules or inspect waveform data.
- The checksum helper compares supplied strings only; M8-005 owns checksum algorithm and manifest behavior.
- Parse helper error classification covers the current Serde/TOML/JSON messages for unsupported units and unknown tagged enum variants.
- M8-004 export should call the validator before writing package artifacts.

## Retrospective

What worked:

- Keeping validation next to the schema avoided duplicated field rules.
- Using parse-tested examples as accepted-package fixtures kept the tests close to the documented format.

What to watch:

- If package parsing changes, parse-error classification tests should guard the user-facing error kinds.
- M8-007 no_std work may need to feature-gate TOML/JSON parse helpers away from embedded-compatible subsets.

## Hand-Off Note

Role: Core Software Engineer / Verification and Validation Engineer
Goal: Add structured rule package validation for issue #68.
Files changed: `README.md`, `crates/ferrisoxide-rule-schema/Cargo.toml`, `crates/ferrisoxide-rule-schema/README.md`, `crates/ferrisoxide-rule-schema/src/lib.rs`, `docs/architecture.md`, `docs/dependency-review.md`, `docs/rule-package-format.md`, `docs/validation-log.md`, `docs/m8-003-rule-package-validator-pipeline-report.md`, `requirements.md`, `traceability-matrix.md`, `risk-register.md`, `project-state.md`.
Checks run: `cargo tree -p ferrisoxide-rule-schema`; `cargo test -p ferrisoxide-rule-schema`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Pass locally; PR/CI/merge pending.
Known gaps: Export command, manifest/checksum algorithm, binary package, shared rule engine, no_std rule-engine boundary, and desktop-vs-embedded parity tests remain future M8 issues.
Next recommended step: Open a protected-branch PR with `Fixes #68`, wait for required `rust` CI, merge, then implement M8-004 / issue #69.
