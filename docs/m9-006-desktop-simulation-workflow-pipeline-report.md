# M9-006 Desktop Simulation Workflow Pipeline Report

Date: 2026-06-01

Contribution / Project: FerrisOxide / issue #82, `M9-006 Add desktop simulation workflow`

Branch: `m9-006-desktop-simulation-workflow`

## Objective

Add a desktop simulation workflow for controller-in-the-loop analysis. The workflow must load a production control config, test verification config, channel map, and fixture/abstract input; output simulation trace and verification evidence; preserve existing analyze/plot behavior; document command usage; and avoid GUI, live DAQ SDK, production RTOS binding, or certification claims.

## Pipeline Stages

| Stage | Owner Role | Artifact | Gate | Decision |
|---|---|---|---|---|
| Intake | Intake Engineer | Issue #82 acceptance criteria and milestone context | Intake Gate | Pass |
| Requirements | Requirements Engineer / V&V Engineer | WRA-RQ-056 update | Requirements Traceability Gate | Pass |
| Architecture | Software Architect | CLI workflow over existing schemas, DAQ fixture source, simulator, and criteria evidence | Architecture Gate | Pass |
| Abstraction Review | Abstraction Review Engineer | Fixture-first workflow; no GUI/live DAQ/RTOS/hardware/certification scope | Granularity Gate | Pass |
| Implementation | Core Software Engineer | `simulate` command, channel-map example, docs, project memory | Implementation Gate | Pass locally |
| Testing | Test Automation Engineer | CLI simulation workflow test and smoke command | Testing Gate | Pass locally |
| V&V | Verification and Validation Engineer | Simulation trace plus verification evidence in JSON/text output | V&V Gate | Pass locally |
| QA | QA Engineer | README and workflow docs | QA Gate | Pass locally |
| Security | Security Engineer | No new third-party dependencies or live hardware/device permissions | Security Gate | Pass locally |
| Performance | Performance Engineer | Fixture-driven bounded workflow; no hardware timing claim | Performance Gate | Pass locally |
| Documentation | Documentation Engineer | README, architecture docs, desktop simulation docs, validation log | Documentation Gate | Pass locally |
| Code Review | Code Review Engineer | Local review of workflow boundaries and validation behavior | Code Review Gate | Pass locally |
| Evaluation | Evaluation Engineer | Definition of Done review in this report | Evaluation Gate | Pass locally |
| Release | Release Engineer | PR #126, issue link, validation evidence | Release Gate | Pass |
| Community | GitHub Maintainer Specialist | PR #126 merged after required CI; issue #82 closed | Community Gate | Pass |
| Retrospective | Project Coordinator | This report captures lessons and residual risk | Retrospective Gate | Pass locally |

## Requirements And Acceptance Mapping

| Acceptance Item | Implementation Evidence | Status |
|---|---|---|
| Loads production control config | `--control-config` parses and validates `ProductionControlConfig`. | Pass locally |
| Loads test verification config | `--verification-config` parses and validates `TestVerificationConfig`. | Pass locally |
| Loads channel map | `--channel-map` validates fixture columns, logical channels, and control input mappings. | Pass locally |
| Loads fixture/abstract input | Fixture CSV is parsed into logical waveform channels and `ferrisoxide-daq` fixture frames. | Pass locally |
| Outputs simulation trace | JSON `simulation_trace` and text transition summary come from `ferrisoxide-simulator`. | Pass locally |
| Outputs verification evidence | JSON `verification_evidence` and text criteria summary come from `AnalysisReport`. | Pass locally |
| Existing analyze/plot compatible | Existing CLI tests pass alongside the new simulation test. | Pass locally |
| Docs show workflow usage | README and `docs/desktop-simulation-workflow.md` include command and channel-map examples. | Pass locally |
| Scope limits preserved | Docs and JSON scope note exclude GUI, live DAQ SDK, RTOS binding, hardware timing guarantee, and certification evidence. | Pass locally |

## Local Validation

| Command | Result | Notes |
|---|---|---|
| `cargo tree -p ferrisoxide-cli` | Passed | Existing local crates and approved workspace dependencies only; no GUI, live DAQ SDK, HAL, RTOS SDK, or target hardware dependency. |
| `cargo test -p ferrisoxide-cli runs_desktop_simulation_workflow_with_fixture_input` | Passed | Focused simulation workflow test passed. |
| `cargo run --quiet --bin ferrisoxide-signal -- simulate --input tests/e2e/heated_actuator/input/passing_run.csv --control-config examples/control-config/production-control-config.toml --verification-config examples/test-verification-config/test-verification-config.toml --channel-map examples/simulation/heated-actuator-channel-map.toml --format text` | Passed | Text smoke output included 9 frames, PASS verification, transitions, and criteria evidence. |
| `cargo fmt --check` | Passed | Formatting clean. |
| `cargo test --workspace` | Passed | 165 tests passed across workspace unit, integration, and doctest targets. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |
| README/desktop simulation/pipeline local Markdown link-target scan | Passed | Local links resolved. |
| `git diff --check` | Passed | No whitespace errors. |

## Hand-Off Note

Role: Software Architect / Core Software Engineer / V&V Engineer
Goal: Implement issue #82 desktop simulation workflow.
Files changed: `crates/ferrisoxide-cli/`, `examples/simulation/heated-actuator-channel-map.toml`, README, architecture/controller workflow docs, desktop simulation docs, requirements, traceability, risk register, validation log, pipeline report, and project state.
Checks run: See validation log.
Status: Pass; PR #126 merged and issue #82 closed.
Known gaps: No GUI, live DAQ SDK, deployment package, production RTOS binding, hardware timing evidence, or certification evidence.
Next recommended step: Continue M9-007 deployment package format work.
