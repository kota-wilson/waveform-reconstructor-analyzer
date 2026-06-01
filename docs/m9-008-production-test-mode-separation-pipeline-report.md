# M9-008 Production/Test Mode Separation Pipeline Report

Date: 2026-06-01

Contribution / Project: FerrisOxide / issue #84, `M9-008 Add production-vs-test mode separation`

Branch: `m9-008-production-test-mode-separation`

## Objective

Separate production control, test verification, and signal-validation modes in controller-in-the-loop package planning. Mode definitions must prevent production control behavior from being confused with test verification or signal-validation behavior, invalid combinations must return clear errors, and docs must explain allowed modes and non-goals.

## Pipeline Stages

| Stage | Owner Role | Artifact | Gate | Decision |
|---|---|---|---|---|
| Intake | Intake Engineer | Issue #84 acceptance criteria and milestone context | Intake Gate | Pass |
| Requirements | Requirements Engineer / V&V Engineer | WRA-RQ-058 update | Requirements Traceability Gate | Pass |
| Architecture | Software Architect / Embedded RTOS Engineer | Manifest-level `mode_profiles` model in `ferrisoxide-deployment` | Architecture Gate | Pass |
| Abstraction Review | Abstraction Review Engineer | Mode validation boundary; no runtime mode switcher or target loader | Granularity Gate | Pass |
| Implementation | Core Software Engineer | Mode purpose enum, mode profile schema, validation rules, fixture updates, docs | Implementation Gate | Pass locally |
| Testing | Test Automation Engineer | Focused deployment mode-profile tests | Testing Gate | Pass locally |
| V&V | Verification and Validation Engineer | Invalid mixed production/test mode combinations return structured errors | V&V Gate | Pass locally |
| QA | QA Engineer | README, controller workflow, deployment format docs, operating-mode docs | QA Gate | Pass locally |
| Security | Security Engineer | No new third-party dependencies, signing, auth, HAL, SDK, or hardware access | Security Gate | Pass locally |
| Performance | Performance Engineer | Small in-memory manifest validation; no runtime timing claim | Performance Gate | Pass locally |
| Documentation | Documentation Engineer | `docs/controller-operating-modes.md` and traceability updates | Documentation Gate | Pass locally |
| Code Review | Code Review Engineer | Local review of mode policy validation behavior | Code Review Gate | Pass locally |
| Evaluation | Evaluation Engineer | Definition of Done review in this report | Evaluation Gate | Pass locally |
| Release | Release Engineer | Branch, issue link, intended PR body, validation evidence | Release Gate | Pending PR |
| Community | GitHub Maintainer Specialist | PR, CI, merge, issue close | Community Gate | Pending PR/CI |
| Retrospective | Project Coordinator | This report captures lessons and residual risk | Retrospective Gate | Pass locally |

## Requirements And Acceptance Mapping

| Acceptance Item | Implementation Evidence | Status |
|---|---|---|
| Production, test, and signal-validation modes are separate | `DeploymentModePurpose` defines `production_control`, `test_verification`, and `signal_validation`; validator requires all three purposes. | Pass locally |
| Production control behavior cannot be confused with test verification | Production mode requires `control_mode` and `production_control_config`; test/signal modes reject `control_mode` and `production_control_config`. | Pass locally |
| Invalid combinations return clear errors | Validator returns `DeploymentValidationError` values with `invalid_mode_artifact_combination`, field, and message. | Pass locally |
| Docs explain allowed modes and non-goals | `docs/controller-operating-modes.md`, README, deployment format docs, and controller workflow docs describe mode policy and runtime non-goals. | Pass locally |
| Workspace checks pass | Validation commands recorded below. | Pass locally |

## Local Validation

| Command | Result | Notes |
|---|---|---|
| `cargo test -p ferrisoxide-deployment` | Passed | 6 deployment manifest tests passed, including required mode-purpose coverage and mixed production/test mode rejection. |
| `cargo tree -p ferrisoxide-deployment` | Passed | Runtime dependency is existing approved `serde`; dev-dependency is existing approved `serde_json`; no GUI, DAQ SDK, HAL, RTOS SDK, signing, target hardware, or runtime loader dependency appears. |
| `cargo fmt --check` | Passed | Formatting clean. |
| `cargo test --workspace` | Passed | 171 tests passed across workspace unit, integration, and doctest targets. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |
| README/mode/deployment/pipeline local Markdown link-target scan | Passed | Local links in README and relevant mode/deployment docs resolved. |
| `git diff --check` | Passed | No whitespace errors. |

## Hand-Off Note

Role: Software Architect / Verification and Validation Engineer
Goal: Implement issue #84 production/test/signal-validation mode separation.
Files changed: `crates/ferrisoxide-deployment/`, `examples/deployment-package/heated-actuator/manifest.json`, README, architecture/controller workflow docs, deployment format docs, operating-mode docs, requirements, traceability, risk register, documentation review, validation log, pipeline report, changelog, and project state.
Checks run: See validation log.
Status: Pass locally after validation; PR, protected CI, merge, and issue #84 closure pending.
Known gaps: No runtime mode switcher, target loader, live DAQ workflow, HAL/SDK adapter, hardware timing evidence, or certification evidence.
Next recommended step: Open PR with `Fixes #84`, wait for required CI, and merge only after checks pass.
