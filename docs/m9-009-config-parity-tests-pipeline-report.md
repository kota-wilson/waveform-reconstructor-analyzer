# M9-009 Config Parity Tests Pipeline Report

Date: 2026-06-01

Contribution / Project: FerrisOxide / issue #85, `M9-009 Add config parity tests`

Branch: `m9-009-config-parity-tests`

## Objective

Add config and behavior parity tests for controller-in-the-loop workflows. The same production control config, test verification config, channel map, and waveform input must produce matching desktop and embedded-compatible state, pass/fail, timing, channel, and evidence outputs, or document approved schema differences.

## Pipeline Stages

| Stage | Owner Role | Artifact | Gate | Decision |
|---|---|---|---|---|
| Intake | Intake Engineer | Issue #85 acceptance criteria and milestone context | Intake Gate | Pass |
| Requirements | Requirements Engineer / V&V Engineer | WRA-RQ-059 update | Requirements Traceability Gate | Pass |
| Architecture | Software Architect / V&V Engineer | CLI workflow parity test over existing simulator and borrowed-rule engine | Architecture Gate | Pass |
| Abstraction Review | Abstraction Review Engineer | Software-only parity boundary; no target runtime or hardware claim | Granularity Gate | Pass |
| Implementation | Test Automation Engineer | Focused parity test, docs, project memory | Implementation Gate | Pass locally |
| Testing | Test Automation Engineer | Focused parity test plus workspace validation | Testing Gate | Pass locally |
| V&V | Verification and Validation Engineer | Matching state trace and criteria evidence fields | V&V Gate | Pass locally |
| QA | QA Engineer | README/docs/traceability updates | QA Gate | Pass locally |
| Security | Security Engineer | No new third-party dependency or hardware surface | Security Gate | Pass locally |
| Performance | Performance Engineer | Fixture-sized parity check; no runtime timing claim | Performance Gate | Pass locally |
| Documentation | Documentation Engineer | `docs/controller-config-parity.md` and `tests/controller_parity/README.md` | Documentation Gate | Pass locally |
| Code Review | Code Review Engineer | Local review of parity assertions and schema-difference wording | Code Review Gate | Pass locally |
| Evaluation | Evaluation Engineer | Definition of Done review in this report | Evaluation Gate | Pass locally |
| Release | Release Engineer | Branch, issue link, intended PR body, validation evidence | Release Gate | Pending PR |
| Community | GitHub Maintainer Specialist | PR, CI, merge, issue close | Community Gate | Pending PR/CI |
| Retrospective | Project Coordinator | This report captures lessons and residual risk | Retrospective Gate | Pass locally |

## Requirements And Acceptance Mapping

| Acceptance Item | Implementation Evidence | Status |
|---|---|---|
| Same production control config | Test loads `examples/control-config/production-control-config.toml`. | Pass locally |
| Same test verification config | Test loads `examples/test-verification-config/test-verification-config.toml`. | Pass locally |
| Same channel map | Test loads `examples/simulation/heated-actuator-channel-map.toml`. | Pass locally |
| Same waveform input | Test loads `tests/e2e/heated_actuator/input/passing_run.csv`. | Pass locally |
| Matching desktop and embedded-compatible state | Test compares portable simulator state trace projection fields. | Pass locally |
| Matching pass/fail, timing, channel, and evidence | Test compares desktop report evidence to embedded-compatible borrowed-rule summaries. | Pass locally |
| Approved schema differences documented | `docs/controller-config-parity.md` documents no target runtime yet and desktop-only reason/metadata differences. | Pass locally |
| Software validation only | Docs and test notes exclude target firmware, live DAQ, hardware timing, and certification evidence. | Pass locally |
| Workspace checks pass | Validation commands recorded below. | Pass locally |

## Local Validation

| Command | Result | Notes |
|---|---|---|
| `cargo test -p ferrisoxide-cli controller_config_and_behavior_paths_match_portable_parity_evidence` | Passed | Focused parity test passed. |
| `cargo tree -p ferrisoxide-cli` | Passed | New direct test-only dependency is local `ferrisoxide-rule-engine`; no new third-party dependency, GUI, DAQ SDK, HAL, RTOS SDK, target runtime, signing, or hardware dependency appears. |
| `cargo fmt --check` | Passed | Formatting clean. |
| `cargo test --workspace` | Passed | 172 workspace unit, integration, and doctest checks passed. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |
| README/parity/pipeline local Markdown link-target scan | Passed | Local links in README, parity docs, controller workflow, documentation review, and pipeline report resolved. |
| `git diff --check` | Passed | No whitespace errors. |

## Hand-Off Note

Role: Verification and Validation Engineer / Test Automation Engineer
Goal: Implement issue #85 config and behavior parity tests.
Files changed: `crates/ferrisoxide-cli/`, `docs/controller-config-parity.md`, `tests/controller_parity/README.md`, README, architecture/controller workflow docs, requirements, traceability, documentation review, validation log, pipeline report, changelog, and project state.
Checks run: See validation log.
Status: Pass locally after validation; PR, protected CI, merge, and issue #85 closure pending.
Known gaps: No embedded controller runtime output, runtime loader, target hardware execution, live DAQ, hardware timing evidence, or certification evidence.
Next recommended step: Open PR with `Fixes #85`, wait for required CI, and merge only after checks pass.
