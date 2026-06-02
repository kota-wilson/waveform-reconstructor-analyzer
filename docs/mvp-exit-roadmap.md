# MVP Exit Roadmap

Date: 2026-06-01

Status: Implemented locally for MVP exit. M10 through M14 are complete through merged PR evidence; M15 through M20 are complete in this local branch as documentation, CLI batch workflow, tests, traceability, risk, and readiness artifacts. No GitHub milestones/issues, external PR, release publication, dependency change, live DAQ work, HAL/RTOS work, target-hardware work, binary package/signing work, or certification claim is added by this MVP-exit implementation.

## Purpose

FerrisOxide has enough implemented surface area that the next work should reduce product risk before claiming the MVP stage is done. The project now has CSV analysis, criteria evaluation, reports, plotting, rule packages, controller-style simulation, transform metadata, event validation, runtime-profile validation, and high-pass baseline correction. The remaining MVP-exit work is not "more transforms by default"; it is compatibility hardening, artifact contract clarity, a repeatable desktop batch workflow, rule-package transform semantics, validation coverage, and an explicit exit review.

## MVP Exit Definition

FerrisOxide can move out of MVP only when all of the following are true:

| Criterion | Required Evidence | Owner |
|---|---|---|
| Config surfaces are reviewable | Authoritative config reference covers CLI flags, TOML config sections, criteria DSL, legacy criteria, filters/transforms, simulation inputs, plotting inputs, export inputs, examples, invalid cases, defaults, units, and compatibility notes. | Documentation Engineer / Software Architect |
| Report and artifact contracts are explicit | JSON report schema, text report expectations, SVG evidence limits, rule-package artifact roles, validation report fixtures, checksum scope, and additive/breaking compatibility policy are documented and covered by golden fixtures. | Documentation Engineer / V&V Engineer |
| Desktop workflows cover repeated local analysis | A local batch workflow can process multiple CSV/config pairs, preserve per-run evidence, produce deterministic aggregate status, and report partial failures without live DAQ, GUI, or hardware dependencies. | Core Software Engineer / Test Automation Engineer |
| Transform/package support boundaries are enforced | Every implemented transform has capability metadata, runtime-profile behavior, package-export behavior, and docs that distinguish desktop analysis support from portable runtime support. | Software Architect / Embedded RTOS Engineer |
| Validation corpus is representative and reproducible | Known-answer fixtures cover current core workflows, transform edge behavior, event/validation behavior, rule-package export guardrails, and at least one large-file benchmark baseline with exact commands and expected outputs. | V&V Engineer / Performance Engineer |
| User-facing docs support a new contributor or evaluator | README, usage docs, config reference, report schema, examples index, troubleshooting notes, and release/readiness notes let a reviewer run and understand the MVP without private context. | Documentation Engineer / GitHub Maintainer Specialist |
| Risks and non-goals are controlled | High-impact active risks have owners and mitigations; post-MVP work is separated from MVP exit; no docs imply hardware qualification, RTOS production readiness, live DAQ support, airworthiness, regulatory compliance, or production safety certification. | Project Coordinator / Evaluation Engineer |

## Proposed Milestone Sequence

| Milestone | Working Version | Focus | Primary Exit Artifact | Status |
|---|---:|---|---|---|
| M15 | v0.13.0 | Config and schema reference hardening | `docs/config-reference.md` plus invalid-config matrix and compatibility notes | Complete locally |
| M16 | v0.14.0 | Report and artifact contract stabilization | `docs/artifact-contract.md`, expanded report-schema links, and golden artifact matrix | Complete locally |
| M17 | v0.15.0 | Desktop batch analysis workflow MVP | Batch manifest/config docs, CLI behavior, aggregate summary schema, and failure-handling tests | Complete locally |
| M18 | v0.16.0 | Rule-package transform semantics | Transform export compatibility matrix and validator/export guardrail tests | Complete locally |
| M19 | v0.17.0 | Validation corpus and benchmark baseline expansion | Validation corpus index, negative-case matrix, benchmark scope, and current evidence map | Complete locally |
| M20 | v0.18.0 | MVP exit readiness review | `docs/mvp-exit-readiness-report.md`, risk/traceability closure, release gate decision | Complete locally |

## M15 / v0.13.0: Config And Schema Reference Hardening

Goal:

- Create one authoritative config reference for supported CLI/config surfaces.
- Inventory existing `analyze`, `plot`, `export`, and `simulate` entry points and the TOML sections they accept.
- Document default values, units, required fields, optional fields, validation errors, legacy criteria compatibility, criteria DSL compatibility, transform config compatibility, and runtime-profile limitations.
- Add or organize invalid-config examples into a reviewable matrix.
- Define config compatibility language for additive changes, breaking changes, and deprecation.

Acceptance evidence:

- `docs/config-reference.md` exists and links from README or relevant usage docs.
- Examples and invalid-config fixtures map to documented behavior.
- Existing CLI/config tests still pass.
- No new dependencies, incompatible schema migration, live DAQ, HAL/RTOS, target hardware, GUI, or certification scope is introduced.

Suggested issue themes after approval:

- M15-001 config surface inventory and reference skeleton.
- M15-002 invalid-config matrix and error-message expectations.
- M15-003 example index and command verification.
- M15-004 compatibility/deprecation policy for config changes.
- M15-005 docs, traceability, risk, and closure report.

## M16 / v0.14.0: Report And Artifact Contract Stabilization

Goal:

- Stabilize how JSON reports, text reports, SVG evidence, validation report fixtures, rule-package artifacts, manifests, and checksum evidence should be read by users and tests.
- Separate additive report fields from breaking report migrations.
- Document artifact retention expectations for local evidence directories and generated outputs.
- Ensure golden artifacts cover implemented report structures: waveform metadata, `transform_steps`, `event_records`, measurement evidence, validation evidence, runtime-profile notes, and package export guardrails.

Acceptance evidence:

- A report/artifact contract doc exists or `docs/report-schema.md` is expanded to cover the full current artifact set.
- Golden artifact matrix names representative fixtures and the command or test that protects them.
- Compatibility policy states when a schema compatibility approval gate is required.
- Existing golden reports and artifact tests pass, or approved additive changes are documented with exact expected outputs.

Suggested issue themes after approval:

- M16-001 report and artifact contract inventory.
- M16-002 golden artifact matrix.
- M16-003 text/JSON/SVG/package compatibility policy.
- M16-004 generated-output retention and naming guidance.
- M16-005 docs, traceability, risk, and closure report.

## M17 / v0.15.0: Desktop Batch Analysis Workflow MVP

Goal:

- Add a local desktop batch workflow for repeated analysis over multiple CSV/config pairs.
- Preserve per-run evidence while producing deterministic aggregate status.
- Make partial failures explicit without hiding individual run errors.
- Keep the workflow file-based and local; do not add live DAQ, GUI, web UI, worker service, cloud storage, database, scheduler, or hardware integration.

Acceptance evidence:

- Batch input shape is documented and validated.
- CLI or documented workflow produces per-run reports and an aggregate summary with stable fields.
- Tests cover all-pass, one-fail, invalid-entry, missing-file, and partial-failure behavior.
- Batch output does not alter existing single-run analysis, plotting, simulation, or export behavior.

Suggested issue themes after approval:

- M17-001 batch workflow design and input schema.
- M17-002 batch CLI execution path or documented local runner.
- M17-003 aggregate summary schema and partial-failure behavior.
- M17-004 batch fixture tests and exact expected summaries.
- M17-005 docs, traceability, risk, and closure report.

## M18 / v0.16.0: Rule-Package Transform Semantics

Goal:

- Decide which implemented transforms can be represented in portable rule packages and which must remain desktop-only.
- Connect transform capability metadata, runtime-profile validation, and rule-package export behavior into one compatibility matrix.
- Keep unsupported transforms rejected with clear errors instead of silently dropping or approximating them.
- Preserve the current no-hardware and no-runtime-loader boundary.

Acceptance evidence:

- Transform/package compatibility matrix documents every implemented transform.
- Export tests prove supported transform behavior, rejected transform behavior, and runtime-profile error messages.
- `high_pass_baseline` and other timing/stateful transforms remain rejected for package/runtime profiles unless explicit semantics and evidence are approved in the milestone.
- Docs state that package export is software evidence and not binary deployment, signing, hardware qualification, or certification.

Suggested issue themes after approval:

- M18-001 transform export compatibility matrix.
- M18-002 rule-package export validator integration.
- M18-003 supported-transform package fixture, if any support is approved.
- M18-004 rejected-transform package fixture and error evidence.
- M18-005 docs, traceability, risk, and closure report.

## M19 / v0.17.0: Validation Corpus And Benchmark Baseline Expansion

Goal:

- Expand validation evidence so current MVP workflows are covered by known-answer fixtures and negative cases.
- Add or organize fixtures for pointwise transforms, baseline transforms, high-pass baseline behavior, event validation, multi-channel timing, rule-package export guardrails, batch workflow summaries, and large-file benchmark behavior.
- Keep benchmark wording limited to local baseline evidence, not production throughput guarantees.

Acceptance evidence:

- Validation index maps each current major workflow to fixture files, configs, expected reports, expected error cases, and exact commands or tests.
- Known-answer docs define expected values before analyzer execution.
- Benchmark log records environment, fixture-generation method, command, timing categories, and scope limits.
- Workspace tests, focused validation tests, and link/diff checks pass.

Suggested issue themes after approval:

- M19-001 validation corpus index and coverage gap review.
- M19-002 transform known-answer fixtures.
- M19-003 event/validation and batch known-answer fixtures.
- M19-004 benchmark baseline refresh with scope limits.
- M19-005 docs, traceability, risk, and closure report.

## M20 / v0.18.0: MVP Exit Readiness Review

Goal:

- Run the explicit MVP exit gate after M15 through M19 are complete.
- Review requirements, traceability, risks, known issues, docs, examples, validation evidence, CI status, and release/community readiness.
- Produce a clear decision: `Pass`, `Fail`, or `Blocked`.
- If passed, define the first post-MVP roadmap without pulling post-MVP scope into the MVP exit decision.

Acceptance evidence:

- `docs/mvp-exit-readiness-report.md` records every gate decision.
- Requirements and traceability identify implemented, deferred, blocked, and post-MVP items.
- Risk register has no unowned high-impact MVP-exit risk.
- Existing workspace validation commands pass.
- Release, community, and retrospective artifacts are written before claiming MVP exit.

Suggested issue themes after approval:

- M20-001 MVP exit checklist and evidence inventory.
- M20-002 requirements, traceability, and risk closure audit.
- M20-003 documentation and onboarding audit.
- M20-004 release/community/retrospective gate artifacts.
- M20-005 post-MVP backlog separation.

## Outside MVP Exit

The following work remains post-MVP or separately gated even if M15 through M20 pass:

- GUI, web UI, plugin runtime, hosted service, database-backed workflows, or scheduler.
- Live DAQ vendor SDKs, drivers, acquisition hardware, or hardware-in-the-loop.
- HAL bindings, RTOS SDK integration, unsafe FFI, target-board execution, or real-time guarantees.
- Binary package serialization, cryptographic signing, authentication, tamper-proof claims, or secure update flows.
- Hardware qualification, flight certification, regulatory compliance, production safety certification, or airworthiness evidence.
- Broad advanced DSP coverage such as general FIR/IIR design families, FFT/spectrum workflows, resampling, wavelets, calibration libraries, or sensor-specific engineering-unit packages unless a later approved milestone scopes them.

## Local Implementation Evidence

| Milestone | Evidence |
|---|---|
| M15 | `docs/config-reference.md` documents CLI commands, TOML config sections, transform/filter fields, criteria DSL, event transforms, batch manifests, compatibility policy, and invalid-config behavior. |
| M16 | `docs/artifact-contract.md` defines JSON/text/SVG/rule-package/batch artifact contracts, generated-output rules, and breaking-change gates; `docs/report-schema.md` links to the artifact contract. |
| M17 | `crates/ferrisoxide-cli/src/main.rs` implements `ferrisoxide-signal batch`; `examples/batch-analysis.toml` provides a manifest example; CLI tests cover pass/fail/error summary behavior, empty manifests, and early bad-format rejection. |
| M18 | `docs/transform-package-compatibility.md` records the implemented transform/package matrix; CLI export tests reject desktop-only transform export with stable errors. |
| M19 | `docs/validation-corpus-index.md` maps implemented workflows to fixtures, expected evidence, negative cases, and benchmark-scope limits. |
| M20 | `docs/mvp-exit-readiness-report.md` records explicit readiness gates, residual risks, release/community/retrospective decisions, and post-MVP separation. |

## Gate Decisions

| Gate | Decision | Evidence | Next Owner |
|---|---|---|---|
| Intake Gate | Pass | User requested a more detailed roadmap and needed milestones before leaving MVP. | Project Coordinator |
| Roadmap Gate | Pass locally | This document defines MVP-exit criteria and records the completed local M15 through M20 sequence. | Project Orchestrator |
| Requirements Gate | Pass | WRA-RQ-099 through WRA-RQ-105 map to implemented local artifacts, tests, and readiness evidence. | Software Architect |
| Scope Gate | Pass locally | M15 through M20 add no dependencies, live DAQ, hardware, HAL/RTOS, binary signing, release publication, or certification scope. | Project Coordinator |
| Human Approval Gate | Pass for local implementation | User approved continuing through the MVP-exit milestones on 2026-06-01. External release publication and GitHub milestone/issue creation remain separately gated. | User / Project Coordinator |
| Implementation Gate | Pass locally | M15 through M20 artifacts, batch CLI implementation, package-boundary tests, docs, traceability, risk, and project state are complete in the working branch. | Project Orchestrator |
| Release Gate | Pass locally, no publication | `docs/mvp-exit-readiness-report.md` records MVP-exit release readiness without publishing a tag or external release. | GitHub Maintainer Specialist |
| Community Gate | Pass locally | `docs/community-report.md` records MVP-exit community messaging and non-goals without external publication. | Community Engineering Lead |

## Hand-Off Note

Role: Project Coordinator / Product Architect
Goal: Define the pre-MVP-exit roadmap after M14 closure.
Files changed: `docs/mvp-exit-roadmap.md`, M15-M20 docs, CLI batch implementation, example manifest, readiness/pipeline artifacts, and linked state/traceability/risk updates.
Checks run: See `docs/validation-log.md`.
Status: M15 through M20 are complete locally and FerrisOxide is ready to leave MVP scope in this branch.
Known gaps: No GitHub milestones/issues, external PR, release tag, hardware evidence, live DAQ evidence, target-runtime evidence, binary signing, or certification evidence is added.
Next recommended step: Review the MVP-exit branch, then decide whether to open/merge a repository PR or create post-MVP issues under a fresh gate.
