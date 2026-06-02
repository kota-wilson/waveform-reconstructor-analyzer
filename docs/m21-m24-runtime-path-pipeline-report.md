# M21-M24 Runtime Path Pipeline Report

Date: 2026-06-01

Status: Complete locally. This report records the approved runtime-path follow-up after MVP exit. It does not open GitHub issues, publish a release, add dependencies, implement a runtime loader, execute target hardware, add HAL/RTOS SDK scope, or create certification evidence.

## Scope

| Milestone | Goal | Artifacts | Gate Decision |
|---|---|---|---|
| M21 | Portable linear pointwise transform semantics in rule packages. | `crates/ferrisoxide-rule-schema/src/lib.rs`, `crates/ferrisoxide-cli/src/main.rs`, `examples/m21-linear-pointwise-package-config.toml` | Pass |
| M22 | Shared runtime-compatible linear transform semantics and parity coverage. | `crates/ferrisoxide-rule-engine/src/lib.rs`, CLI parity test | Pass |
| M23 | Package compatibility corpus for positive and negative transform fixtures. | `examples/rule-package/linear-pointwise-rules.*`, `examples/rule-package/unsupported-clamp-rules.*`, `docs/validation-corpus-index.md` | Pass |
| M24 | Runtime loader design gate. | `docs/runtime-loader-design-gate.md` | Pass for design; implementation remains blocked pending approval |

## Stage Decisions

| Stage | Decision | Evidence | Residual Risk |
|---|---|---|---|
| Research | Pass | Existing transform taxonomy, runtime-profile docs, and package compatibility docs were used as inputs. | More DSP families remain out of scope. |
| Requirements | Pass | WRA-RQ-106 through WRA-RQ-109 added. | Future rejected-transform support needs new requirements. |
| Architecture | Pass | Linear pointwise transforms are represented as explicit package filters; runtime helper uses borrowed slices and caller-owned output. | Loader implementation not started. |
| Implementation | Pass | Schema/export/runtime helper and fixtures implemented locally. | No package parser/loader for embedded runtime. |
| Testing | Pass locally | Focused tests for schema, runtime helper, CLI export, and parity passed before full validation. | Full workspace validation is recorded separately in `docs/validation-log.md`. |
| V&V | Pass locally | Positive and negative package fixtures prove supported and rejected behavior. | No hardware V&V, target timing evidence, or certification evidence. |
| QA | Pass locally | Unsupported transforms continue to fail clearly instead of being silently dropped. | External PR review/protected CI pending if this branch is opened. |
| Security | Pass | No new dependencies, signing, authentication, network, secrets, or global setup added. | Checksum remains drift detection only. |
| Performance | Not Applicable | Runtime helper is bounded over input length and caller buffers; no benchmark claim added. | Future loader needs bounded-capacity measurement. |
| Documentation | Pass | Rule-package format, compatibility matrix, corpus index, roadmap, and loader design gate updated. | Automated docs drift checks remain future work. |
| Release | Not Applicable | No release tag or package publication requested. | Release publication remains separately gated. |
| Community | Not Applicable | No external announcement or issue creation performed. | Messaging must avoid runtime/hardware/certification overclaims. |
| Retrospective | Pass | Lessons captured through this report and updated roadmap/risk records. | Future runtime work should stay issue-scoped. |

## Scope Controls

- No dependencies added.
- No global tooling or system configuration changed.
- No live DAQ, vendor SDK, HAL, RTOS SDK, unsafe FFI, target hardware, binary package loader, cryptographic signing, hardware qualification, flight certification, regulatory compliance, safety certification, or airworthiness evidence added.
- `offset`, `gain`, and `invert` are software transforms only; they are not calibrated sensor accuracy or span-correction evidence.

## Hand-Off Note

Role: Project Orchestrator / Embedded RTOS Engineer
Goal: Implement the first narrow runtime-path follow-up after MVP exit.
Files changed: rule schema, CLI export, rule engine, package fixtures, config example, docs, requirements, traceability, risks, and state.
Checks run: See `docs/validation-log.md`.
Status: M21-M24 complete locally.
Known gaps: No runtime loader implementation, binary package format, target execution, hardware evidence, or release publication.
Next recommended step: Review this branch; only then decide whether to open a PR or approve a loader implementation issue.
