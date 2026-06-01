# M9-010 Qualification Evidence Report Pipeline Report

Date: 2026-06-01

Contribution / Project: FerrisOxide / issue #86, `M9-010 Add qualification evidence report format`

Branch: `m9-010-qualification-evidence-report`

## Objective

Define a qualification evidence report format for controller-in-the-loop workflows. The report must link production control config version, test verification config version, channel map, simulation trace, criteria evidence, deployment package metadata, checksum evidence, generated timestamp, and explicit non-certification scope notes.

## Pipeline Stages

| Stage | Owner Role | Artifact | Gate | Decision |
|---|---|---|---|---|
| Intake | Intake Engineer | Issue #86 acceptance criteria and milestone context | Intake Gate | Pass |
| Requirements | Requirements Engineer / V&V Engineer | WRA-RQ-060 update | Requirements Traceability Gate | Pass |
| Architecture | Software Architect / V&V Engineer | `QualificationEvidenceReport` schema in `ferrisoxide-deployment` | Architecture Gate | Pass |
| Abstraction Review | Abstraction Review Engineer | Report-format boundary; no exporter, runtime loader, HAL, SDK, or certification claim | Granularity Gate | Pass |
| Implementation | Core Software Engineer | Typed report structs, validation helpers, exact JSON fixture | Implementation Gate | Pass locally |
| Testing | Test Automation Engineer | Exact JSON round-trip and validation tests | Testing Gate | Pass locally |
| V&V | Verification and Validation Engineer | Links among config versions, channel map, trace, criteria, package, checksum, timestamp, and scope notes | V&V Gate | Pass locally |
| QA | QA Engineer | README/docs/traceability updates | QA Gate | Pass locally |
| Security | Security Engineer | No new dependencies, signing, authentication, target loader, or hardware access | Security Gate | Pass locally |
| Performance | Performance Engineer | Small serialized report schema; no runtime timing or throughput claim | Performance Gate | Pass locally |
| Documentation | Documentation Engineer | `docs/qualification-evidence-report.md` | Documentation Gate | Pass locally |
| Code Review | Code Review Engineer | Local review of schema fields, validation rules, and non-certification wording | Code Review Gate | Pass locally |
| Evaluation | Evaluation Engineer | Definition of Done review in this report | Evaluation Gate | Pass locally |
| Release | Release Engineer | PR #130, issue link, validation evidence, protected CI | Release Gate | Pass |
| Community | GitHub Maintainer Specialist | PR #130 merged; issue #86 closed; milestone #9 closed | Community Gate | Pass |
| Retrospective | Project Coordinator | This report captures lessons and residual risk | Retrospective Gate | Pass locally |

## Requirements And Acceptance Mapping

| Acceptance Item | Implementation Evidence | Status |
|---|---|---|
| Production control config version linked | `QualificationVersionedArtifactLink` and example `production_control_config.version`. | Pass locally |
| Test verification config version linked | `QualificationVersionedArtifactLink` and example `test_verification_config.version`. | Pass locally |
| Channel map linked | `QualificationArtifactLink` and checksum validation for `channel_map`. | Pass locally |
| Simulation trace linked | `QualificationSimulationTrace` with frames, state-machine states, transitions, and outputs. | Pass locally |
| Criteria evidence linked | `QualificationCriterionEvidence` records with outcome, criterion ID, measured/required values, sample index, timestamp, channel, and unit. | Pass locally |
| Deployment package metadata linked | `QualificationDeploymentPackageEvidence` includes package metadata, manifest path, target, generated timestamp, and mode profiles. | Pass locally |
| Checksum evidence linked | `QualificationChecksumEvidence` requires checksum entries for every linked deployment artifact role. | Pass locally |
| Generated timestamp linked | Top-level `generated_at`, deployment package `generated_at`, and `generated_at_artifact` are included. | Pass locally |
| Explicit non-certification notes | `scope_notes` validation requires "not flight certification evidence". | Pass locally |
| Exact report tests | Example JSON parses, validates, serializes, and compares exactly. | Pass locally |
| Workspace checks pass | Validation commands recorded below. | Pass locally |

## Local Validation

| Command | Result | Notes |
|---|---|---|
| `cargo test -p ferrisoxide-deployment` | Passed | 10 deployment tests passed, including exact qualification report JSON, missing checksum link, missing non-certification scope note, and empty trace/criteria validation. |
| `cargo tree -p ferrisoxide-deployment` | Passed | Runtime dependency is existing approved `serde`; dev-dependency is existing approved `serde_json`; no new third-party dependency, GUI, DAQ SDK, HAL, RTOS SDK, target runtime, signing, or hardware dependency appears. |
| `cargo fmt --check` | Passed | Formatting clean. |
| `cargo test --workspace` | Passed | 176 workspace unit, integration, and doctest checks passed. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |
| README/evidence/deployment/pipeline local Markdown link-target scan | Passed | Local links in README, qualification evidence docs, controller workflow, deployment README, documentation review, and pipeline report resolved. |
| `git diff --check` | Passed | No whitespace errors. |
| PR #130 protected `rust` CI | Passed | Required GitHub status check passed before merge. |
| Milestone #9 closure | Passed | GitHub milestone #9 closed with 12 closed issues and 0 open issues. |

## Hand-Off Note

Role: Software Architect / Verification and Validation Engineer
Goal: Implement issue #86 qualification evidence report format.
Files changed: `crates/ferrisoxide-deployment/`, `examples/deployment-package/heated-actuator/qualification-report.json`, `docs/qualification-evidence-report.md`, README, architecture/controller workflow docs, requirements, traceability, risk register, documentation review, validation log, pipeline report, changelog, and project state.
Checks run: See validation log.
Status: Pass; PR #130 merged, issue #86 closed, and milestone #9 closed.
Known gaps: No CLI export command for deployment packages, live DAQ SDK, RTOS loader, target hardware execution, cryptographic signing, hardware timing evidence, or certification evidence.
Next recommended step: Start a fresh milestone proposal for deployment export/runtime follow-up work.
