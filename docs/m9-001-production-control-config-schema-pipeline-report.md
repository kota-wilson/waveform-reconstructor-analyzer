# M9-001 Production Control Config Schema Pipeline Report

Date: 2026-05-31

Contribution / Project: FerrisOxide / issue #77, `M9-001 Define production control config schema`

Owner Role: Software Architect / Core Software Engineer / Documentation Engineer

## Scope

Define a production control config schema for future controller-in-the-loop workflows. This is a schema and validation boundary only. It does not add controller simulation, DAQ integration, controller I/O, HALs, RTOS bindings, runtime package loading, hardware execution, or certification evidence.

## Stage Gate Summary

| Stage | Owner Role | Artifact / Evidence | Gate | Decision |
|---|---|---|---|---|
| Research | Software Architect | Issue #77, controller workflow docs, rule-schema patterns | Target Intake Gate | Pass |
| Requirements | V&V Engineer | WRA-RQ-051 acceptance criteria already in `requirements.md`; updated status/evidence | Requirements Traceability Gate | Pass |
| Architecture | Software Architect | `ferrisoxide-control-schema` crate boundary and `docs/control-config-schema.md` | Architecture Gate | Pass |
| Abstraction Review | Abstraction Review Engineer | Schema fields, validation rules, and example TOML named explicitly | Granularity Gate | Pass |
| Implementation | Core Software Engineer | New local crate, schema structs, parser helpers, validation report/errors, example config | Implementation Gate | Pass |
| Testing | Test Automation Engineer | Schema unit tests, example TOML/JSON round trip, invalid-reference tests, workspace validation | Testing Gate | Pass locally |
| V&V | V&V Engineer | Acceptance mapping in this report and traceability row update | V&V Gate | Pass locally |
| QA | QA Engineer | Docs label schema-only scope and non-runtime limits | QA Gate | Pass locally |
| Security | Security Engineer | No new third-party dependencies, credentials, network paths, HALs, SDKs, or unsafe code | Security Gate | Pass locally |
| Performance | Performance Engineer | Data-only schema; no runtime path or performance claim | Performance Gate | Not Applicable |
| Documentation | Documentation Engineer | README, architecture, controller workflow, control schema docs, validation log | Documentation Gate | Pass locally |
| Code Review | Code Review Engineer | Local diff review and scoped crate boundary | Code Review Gate | Pass locally |
| Evaluation | Evaluation Engineer | Definition of Done reviewed except external PR/CI/merge | Evaluation Gate | Pass locally |
| Release | Release Engineer | Branch, commit, PR body, validation evidence | Release Gate | Pending PR |
| Community | GitHub Maintainer Specialist | PR, required CI, merge, issue closure | Community Gate | Pending PR/CI |
| Retrospective | Project Coordinator | Lesson captured in this report | Retrospective Gate | Pass locally |

## Acceptance Mapping

| Acceptance Criteria | Evidence | Result |
|---|---|---|
| Schema captures inputs. | `ControlInput` in `crates/ferrisoxide-control-schema/src/lib.rs`; `examples/control-config/production-control-config.toml` | Pass |
| Schema captures outputs. | `ControlOutput` and `OutputValue`; example safe states and output actions | Pass |
| Schema captures thresholds. | `ControlThreshold` with role, value, hysteresis, and input reference validation | Pass |
| Schema captures state machine definitions. | `StateMachine`, `StateDefinition`, `StateTransition`, `TransitionCondition` | Pass |
| Schema captures timing rules. | `ControlTiming` and `TimingRule` | Pass |
| Schema captures control actions. | `ControlAction` variants for output, mode entry, fault raise, and no-op | Pass |
| Schema captures fault responses. | `FaultResponse` with severity, latch, safe mode, and actions | Pass |
| Schema captures modes. | `ControlMode` with initial/enabled state machines and entry/exit actions | Pass |
| Schema captures version and approval metadata. | `ControlPackageMetadata`, `ApprovalMetadata`, schema version constant, approval validation | Pass |
| Schema remains separate from test verification config. | New `ferrisoxide-control-schema` crate separate from `ferrisoxide-rule-schema` | Pass |
| No desktop-only DAQ, plotting, reports, vendor SDKs, HALs, or RTOS bindings introduced. | Cargo dependencies only existing workspace Serde/JSON/TOML; docs state no runtime scope | Pass |
| Docs and schema tests are included. | `docs/control-config-schema.md`; crate unit tests | Pass |
| Workspace fmt, tests, clippy, and diff check pass. | `docs/validation-log.md` M9-001 update | Pass locally |

## Design Decisions

| Decision | Reason | Alternatives Considered |
|---|---|---|
| Add `ferrisoxide-control-schema` as a separate crate. | Production control behavior should not be mixed into test verification rule packages. | Extending `ferrisoxide-rule-schema`, rejected because it would conflate production behavior and verification criteria. |
| Keep the crate data-only. | Issue #77 asks for schema, not simulation/runtime behavior. | Adding a simulator now, rejected because issue #78 owns that work. |
| Use existing Serde/TOML/JSON workspace dependencies. | Config schemas already rely on those approved dependencies. | Adding a new schema/parser dependency, rejected because no new dependency is needed. |
| Validate references and units in schema crate. | Bad production config should fail before simulation or deployment work consumes it. | Deferring all validation to simulator, rejected because later issues need a trustworthy schema boundary. |

## Validation Commands

Validation commands:

```text
cargo test -p ferrisoxide-control-schema
cargo tree -p ferrisoxide-control-schema
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
git diff --check
```

## Gate Decisions

| Gate | Decision | Reason | Residual Risk | Owner |
|---|---|---|---|---|
| Target Intake Gate | Pass | Issue #77 is clear and belongs first in M9 dependency order. | None. | Project Coordinator |
| Requirements Traceability Gate | Pass | WRA-RQ-051 maps to concrete schema files and tests. | Later M9 issues must refine behavior. | V&V Engineer |
| Architecture Gate | Pass | New crate keeps production control schema separate from verification schema. | Future shared unit model may be worth extracting later. | Software Architect |
| Granularity Gate | Pass | Fields, validation rules, files, tests, and non-goals are explicit. | None. | Abstraction Review Engineer |
| Implementation Gate | Pass | Schema structs, parser helpers, validation errors, example config, and tests exist. | No execution engine yet by design. | Core Software Engineer |
| Testing Gate | Pass locally | Focused crate tests, formatting, workspace tests, clippy, and whitespace checks passed. | Protected GitHub CI pending. | Test Automation Engineer |
| V&V Gate | Pass locally | Acceptance criteria map directly to schema fields and tests. | No external user review yet. | V&V Engineer |
| QA Gate | Pass locally | Docs clearly label schema-only scope. | Users may still expect simulation; README/docs point to future issues. | QA Engineer |
| Security Gate | Pass locally | No new third-party dependencies, secrets, auth, HALs, SDKs, network calls, or unsafe code. | Future deployment/runtime work needs fresh review. | Security Engineer |
| Performance Gate | Not Applicable | Data-only schema change; no runtime path. | None. | Performance Engineer |
| Documentation Gate | Pass locally | New docs and README/architecture/controller workflow updates exist. | Future examples need updates as simulator work lands. | Documentation Engineer |
| Code Review Gate | Pass locally | Local review found scoped schema work and no unrelated runtime expansion. | PR review/CI pending. | Code Review Engineer |
| Evaluation Gate | Pass locally | Local Definition of Done is satisfied except PR/CI/merge. | Community gate pending. | Evaluation Engineer |
| Release Gate | Pending PR | Branch must be pushed and PR opened. | Protected CI pending. | Release Engineer |
| Community Gate | Pending PR/CI | PR must pass required `rust` check and merge to close #77. | Maintainer feedback possible. | GitHub Maintainer Specialist |
| Retrospective Gate | Pass locally | Lesson: M9 runtime issues should consume the schema rather than redefining config fields. | None. | Project Coordinator |

## Files Changed

| File | Purpose |
|---|---|
| `Cargo.toml` | Adds `crates/ferrisoxide-control-schema` to the workspace. |
| `crates/ferrisoxide-control-schema/` | New production control config schema crate. |
| `examples/control-config/production-control-config.toml` | Parse-tested example production control config. |
| `docs/control-config-schema.md` | Human-readable schema guide. |
| `README.md` | Adds crate and docs references. |
| `docs/architecture.md` | Adds production control schema crate boundary. |
| `docs/controller-in-the-loop-workflow.md` | Updates planned/existing crate status. |
| `docs/validation-log.md` | Validation evidence for M9-001. |
| `requirements.md`, `traceability-matrix.md`, `risk-register.md`, `project-state.md` | Requirements, traceability, risk, and state updates. |

## Hand-Off Note

Role: Software Architect / Core Software Engineer
Goal: Complete M9-001 production control config schema.
Files changed: `Cargo.toml`, `crates/ferrisoxide-control-schema/`, `examples/control-config/production-control-config.toml`, README, architecture/controller workflow docs, control-schema docs, validation log, requirements, traceability, risk register, project state, and this pipeline report.
Checks run: `cargo test -p ferrisoxide-control-schema`; `cargo tree -p ferrisoxide-control-schema`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Pass locally; PR, protected CI, merge, and issue #77 closure pending.
Known gaps: No simulator, DAQ abstraction, controller I/O abstraction, deployment package runtime, or hardware execution yet; those are later M9 issues.
Next recommended step: Open PR with `Fixes #77`, wait for required CI, merge, then continue M9-002 / issue #80.
