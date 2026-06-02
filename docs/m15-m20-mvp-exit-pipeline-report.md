# M15-M20 MVP Exit Pipeline Report

Date: 2026-06-01

Contribution / Project: FerrisOxide Signal

Stage: Full pipeline closure for local MVP exit

Owner Role: Project Orchestrator / Project Coordinator

## Inputs

- Input artifact: `docs/mvp-exit-roadmap.md`
- Source request: User approved continuing M15-M20 until FerrisOxide moves out of MVP.
- Prior gate decision: M14 closed through PR #173; M15-M20 local implementation approved by user on 2026-06-01.

## Work Performed

| Milestone | What | Where | How | Why |
|---|---|---|---|---|
| M15 | Config reference hardening | `docs/config-reference.md`; README | Documented CLI/config surfaces, invalid cases, compatibility policy, and examples. | Users need a reviewable config contract before MVP exit. |
| M16 | Artifact contract stabilization | `docs/artifact-contract.md`; `docs/report-schema.md` | Documented report, SVG, rule-package, checksum, validation, batch, and generated-output rules. | Downstream automation needs stable artifact expectations. |
| M17 | Local batch analysis workflow | `crates/ferrisoxide-cli/src/main.rs`; `examples/batch-analysis.toml`; `docs/batch-analysis-workflow.md` | Added `batch` subcommand, manifest validation, per-run reports, summary JSON/text output, partial-failure behavior, and tests. | Repeated local analysis should not require ad hoc shell scripts or live DAQ scope. |
| M18 | Rule-package transform semantics | `docs/transform-package-compatibility.md`; `docs/rule-package-format.md`; CLI tests | Documented support/rejection matrix and tested desktop-only transform rejection. | Unsupported transforms must not be silently exported into portable packages. |
| M19 | Validation corpus and benchmark baseline index | `docs/validation-corpus-index.md` | Mapped workflows to fixtures, expected evidence, negative cases, and benchmark limits. | MVP exit requires reproducible software validation evidence. |
| M20 | Readiness review | `docs/mvp-exit-readiness-report.md`; `docs/post-mvp-roadmap.md`; state files | Recorded pass decisions, residual risks, release/community/retrospective gates, and post-MVP separation. | Product maturity claim needs explicit evidence and boundaries. |

## Pipeline Stages

| Stage | Decision | Artifact | Evidence | Residual Risk / Next Owner |
|---|---|---|---|---|
| Intake | Pass | User request; `docs/mvp-exit-roadmap.md` | User asked to continue implementation until moving out of MVP. | None for intake; Project Coordinator owns scope control. |
| Project Creation | Not Applicable | Existing project files | FerrisOxide already exists with charter, requirements, traceability, risk, and state files. | No new project created. |
| Project Orchestration | Pass | `orchestration-plan.md` | M15-M20 tasks and gates updated for local completion. | Future post-MVP work needs fresh orchestration. |
| Research | Pass | `docs/mvp-exit-roadmap.md`; existing signal-processing taxonomy context | Work used existing project taxonomy and prior milestone evidence, not new external claims. | Advanced DSP research remains post-MVP. |
| Requirements | Pass | `requirements.md` | WRA-RQ-099 through WRA-RQ-105 implemented locally. | Future scope changes require new requirements. |
| Architecture | Pass | `docs/config-reference.md`; `docs/artifact-contract.md`; `docs/transform-package-compatibility.md` | Config/report/package boundaries and non-goals are explicit. | Portable transform export expansion remains gated. |
| Abstraction Review | Pass | This report; M15-M20 docs | Outputs identify files, commands, tests, owners, risks, and non-goals. | Automated abstraction linting is not implemented. |
| Approval Gate | Pass for local implementation | Conversation approval; `docs/mvp-exit-roadmap.md` | User approved continuing through MVP-exit milestones. | External release and GitHub issue/milestone actions remain separately gated. |
| Implementation | Pass | `crates/ferrisoxide-cli/src/main.rs`; `examples/batch-analysis.toml` | Batch workflow implemented and single-run analysis code reused. | Batch plotting/export orchestration remains post-MVP. |
| Testing | Pass locally | `docs/validation-log.md` | Focused batch and transform-package tests plus workspace validation. | Protected CI waits for external PR. |
| V&V | Pass | `docs/validation-corpus-index.md`; `docs/mvp-exit-readiness-report.md` | Current workflows map to fixtures and expected evidence. | Hardware/DAQ/certification validation excluded. |
| QA | Pass locally | `docs/validation-log.md` | Formatting, tests, clippy, diff check, whitespace scan, and Markdown link scan. | No dedicated QA automation beyond existing commands. |
| Security | Pass locally | `risk-register.md`; `Cargo.lock` | No dependencies, signing, auth, credentials, network services, or global changes added. | Future package signing/auth needs security gate. |
| Performance | Pass with limits | `docs/validation-corpus-index.md`; `docs/benchmarking.md` | Existing benchmark helper and scope limits remain the performance boundary. | Benchmark refresh is post-MVP. |
| Documentation | Pass | README; M15-M20 docs | New docs are linked and state non-goals. | External reader feedback unavailable. |
| Code Review | Pass locally | CLI diff; this report | Changes are scoped to batch CLI and package guardrail tests. | Formal external PR review not yet run. |
| Evaluation | Pass | `docs/evaluation-report.md`; `docs/mvp-exit-readiness-report.md` | Evaluation accepts local MVP-exit decision with residual risks. | Post-MVP scope must stay separated. |
| Release | Pass locally, no publication | `docs/release-readiness.md` | Release readiness update records no tag or publication. | Actual release requires separate approval. |
| Community | Pass locally | `docs/community-report.md` | Community messaging updated for post-MVP local desktop scope. | No external community feedback yet. |
| Retrospective | Pass | `docs/retrospective.md` | Lessons and next actions recorded. | Lessons need post-MVP issue planning later. |

## Evidence

| Evidence | Location / Command / Link | Result |
|---|---|---|
| Batch workflow tests | `cargo test -p ferrisoxide-cli batch -- --nocapture` | Pass |
| Transform-package matrix test | `cargo test -p ferrisoxide-cli rule_package_export_rejects_desktop_only_transform_matrix -- --nocapture` | Pass |
| Full validation | `docs/validation-log.md` | Pass locally after final workspace checks |
| MVP exit readiness | `docs/mvp-exit-readiness-report.md` | Pass locally |
| Post-MVP separation | `docs/post-mvp-roadmap.md` | Pass locally |

## Blockers

| Blocker | Impact | Required Owner | Next Action |
|---|---|---|---|
| No external PR/CI yet for M15-M20 | Local branch is not merged evidence. | GitHub Maintainer Specialist | Open PR only after user approval. |
| No release tag | MVP-exit readiness is local, not a published release. | Release Engineer | Publish only after release approval. |
| No hardware/DAQ/RTOS evidence | Product cannot claim those scopes. | Product Architect / V&V Engineer | Keep excluded from MVP-exit claims. |

## Gate Decision

- Gate: M15-M20 local MVP-exit pipeline.
- Decision: Pass locally.
- Reason: Each required stage produced an artifact, all required M15-M20 requirements map to evidence, batch implementation is tested, unsupported transform export remains guarded, and post-MVP scope is separated.
- Residual risk: External PR review, protected CI, release publication, GitHub issue closure, hardware evidence, target-runtime evidence, binary signing, and certification evidence are not part of this local closure.
- Owner for residual risk: Project Coordinator / GitHub Maintainer Specialist.

## Hand-Off Note

Role: Project Orchestrator / Project Coordinator
Goal: Complete M15-M20 local MVP-exit pipeline.
Files changed: M15-M20 docs, CLI batch implementation, example manifest, README, requirements, traceability, risk, orchestration, project state, release/community/evaluation/retrospective artifacts.
Checks run: See `docs/validation-log.md`.
Status: Pass locally.
Known gaps: External PR, protected CI, release tag, and GitHub milestone/issue evidence remain future gates.
Next recommended step: Review this branch and decide whether to open a PR or start one post-MVP planning theme.
