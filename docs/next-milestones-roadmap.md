# Next Milestones Roadmap

Date: 2026-06-01

Status: M10, M11, M12, M13, and M14 complete. PR #138 merged M10 and closed milestone #10; PR #147 merged M11 and closed milestone #11; PR #156 merged M12 and closed milestone #12; PR #164 merged M13 and closed milestone #13 with issues #158 through #163; PR #165 records M13 release/community closure; PR #173 merged M14 and closed milestone #14 with issues #167 through #172.

## Purpose

FerrisOxide has completed the validated MVP, measurement/evidence engine, portable rule package, and controller simulation/deployment config milestones through M9. The next useful path is to turn the analog transform taxonomy into a staged transform architecture without claiming broad algorithm support before code and tests exist.

This roadmap sequences the transform milestones:

| Milestone | Working Version | Goal | Status |
|---|---|---|---|
| M10 | v0.8.0 | Transform architecture and capability metadata | Complete; PR #138 merged and milestone #10 closed |
| M11 | v0.9.0 | Pointwise and windowed transform MVP | Complete in PR #147; milestone #11 closed |
| M12 | v0.10.0 | Event and validation transform MVP | Complete in PR #156; milestone #12 closed |
| M13 | v0.11.0 | Transform runtime-profile validation | Complete in PR #164; milestone #13 closed |
| M14 | v0.12.0 | High-pass baseline correction | Complete in PR #173; milestone #14 closed |

## Sequencing Rationale

M10 comes first because the current implementation exposes a filter chain, string transform history, and a small set of implemented transforms. The taxonomy is broader: it includes pointwise transforms, windowed transforms, event transforms, feature extraction, validation transforms, calibration transforms, fault injection, and runtime/deployment constraints. A structured capability model is needed before adding more algorithms.

M11 adds low-risk transforms that can be implemented without new third-party crates and without changing the project's raw-data preservation rule. These transforms directly support DAQ cleanup workflows: offset/gain correction, clamping, deadband, DC removal, baseline subtraction, and moving median smoothing.

M12 adds test-oriented event and validation transforms for switch, relay, and controller-test workflows. These transforms should produce auditable event records first, then validation results linked to those records. That keeps signal interpretation separate from pass/fail decisions.

M13 follows because M10 defined runtime-profile compatibility rules but left validator code as a future gap. M11 and M12 now emit richer transform metadata, so FerrisOxide needs a code-level validator that can reject unsupported desktop, embedded-candidate, Pico-candidate, and future-gated transform exposure before future package or runtime claims are made.

M14 implements the deferred WRA-RQ-078 high-pass baseline correction in a narrow desktop-only slice. That closes the M11 timing-behavior gap without expanding into broad filter design, rule-package transform export, DAQ, HAL/RTOS, target hardware, or certification scope.

## Near-Term Milestones

### M10 / v0.8.0: Transform Architecture And Capability Metadata

Primary artifact: `docs/v0.8.0-transform-architecture-milestone-proposal.md`

Goal:

- Define the transform category vocabulary.
- Define transform capability metadata.
- Map existing moving average, low-pass, and ADC quantization behavior into the new metadata model.
- Preserve existing config and report compatibility.
- Validate desktop/offline, Raspberry Pi 5 no_std, and Pico 2 micro-runtime capability boundaries before exposing transforms to deployment packages.

Exit evidence:

- Requirements WRA-RQ-070 through WRA-RQ-074 accepted or revised.
- Capability metadata design reviewed.
- Existing transform behavior remains unchanged.
- Docs distinguish implemented transforms from planned transform families.

### M11 / v0.9.0: Pointwise And Windowed Transform MVP

Primary artifact: `docs/v0.9.0-pointwise-windowed-transform-mvp-milestone-proposal.md`

Goal:

- Add config and implementation support for initial pointwise transforms.
- Add DC removal and baseline subtraction as baseline transforms.
- Add moving median as the first new windowed transform.
- Preserve raw data and record structured transform metadata for every derived waveform.
- Defer first-order high-pass baseline correction until a separate timing-behavior issue.

Exit evidence:

- Requirements WRA-RQ-075 through WRA-RQ-080 accepted or revised.
- New transforms have unit, config, CLI, golden-report, and raw-preservation tests.
- Edge behavior and time-axis assumptions are documented.

### M12 / v0.10.0: Event And Validation Transform MVP

Primary artifact: `docs/v0.10.0-event-validation-transform-milestone-proposal.md`

Goal:

- Add event records as first-class evidence.
- Implement dual-threshold/Schmitt trigger state conversion.
- Implement debounce, glitch removal, edge extraction, and bounce detection.
- Add validation transforms for missing/extra pulse, dwell-time, and timeout checks.

Exit evidence:

- Requirements WRA-RQ-081 through WRA-RQ-086 accepted or revised.
- Event records link sample index, timestamp, channel, thresholds, and state transitions.
- Validation results link back to event records.
- Known-answer switch/bounce fixtures prove expected event and validation behavior.
- Event validation failures contribute to top-level report failure.

### M13 / v0.11.0: Transform Runtime Profile Validation

Primary artifact: `docs/v0.11.0-transform-runtime-profile-validation-milestone-proposal.md`

Goal:

- Add a structured transform runtime-profile validator API.
- Reject unsupported `desktop`, `pi5_no_std_candidate`, `pico2_candidate`, and `future_gated` profile requests with stable errors.
- Validate timing metadata for sample-rate-required transform exposure.
- Prove current waveform, event, and validation transform metadata remains desktop-only unless later bounded runtime work is approved.
- Document that current legacy rule-package filter export is not a broad transform runtime support claim.

Exit evidence:

- Requirements WRA-RQ-087 through WRA-RQ-092 accepted or revised.
- Runtime-profile validator unit tests cover accepted and rejected profile requests.
- Timing evidence tests cover missing and invalid metadata.
- Event/validation metadata tests preserve desktop-only scope.
- Docs separate runtime-profile validation from live DAQ, HAL/RTOS, target hardware, and certification support.

### M14 / v0.12.0: High-Pass Baseline Correction

Primary artifact: `docs/v0.12.0-high-pass-baseline-correction-milestone-proposal.md`

Goal:

- Add a `high_pass_baseline` desktop transform using the existing `[[filters]]` config surface.
- Implement a documented causal first-order high-pass recurrence.
- Require finite positive `cutoff_hz`, strictly increasing timestamps, and finite samples.
- Preserve raw data and emit structured metadata for timing, phase, statefulness, and desktop-only runtime profile.
- Keep rule-package export unsupported until a later package-semantics milestone.

Exit evidence:

- Requirements WRA-RQ-093 through WRA-RQ-098 accepted or revised.
- Unit, config, CLI, metadata, and export-guardrail tests pass.
- Docs describe equation, edge behavior, phase effect, and non-goals.
- No new dependencies, report/config schema migration, DAQ, HAL/RTOS, hardware, or certification scope.

## Future-Gated Work

The following remain outside M10 through M14 unless a fresh proposal and approval gate moves them forward:

- Advanced FIR/IIR filter families beyond the current simple first-order filters.
- FFT, spectrum analysis, time-frequency analysis, and wavelet workflows.
- Resampling, clock drift correction, and timestamp-grid repair.
- Sensor-specific calibration libraries for thermocouples, RTDs, strain gauges, load cells, LVDTs, microphones, or photodiodes.
- Live DAQ vendor SDKs, drivers, or hardware input.
- HAL, RTOS SDK, target hardware bindings, unsafe FFI, or real-time runtime guarantees.
- Binary deployment package serialization, cryptographic signing, authentication, or tamper-proof claims.
- Hardware qualification, flight certification, or regulatory compliance evidence.
- GUI, web UI, plugin runtime, batch analysis, and embedded plotting.
- Enforcing new runtime/package behavior beyond M13 validator scope or M14 rule-package guardrails unless a separate schema/export migration is approved.

## Gate Decisions

| Gate | Decision | Evidence | Next Owner |
|---|---|---|---|
| Intake Gate | Pass | User supplied analog-transform taxonomy and asked to move forward with next milestones. | Project Coordinator |
| Roadmap Gate | Pass locally | This document sequences M10 through M13 from the taxonomy and current project state. | Project Orchestrator |
| Requirements Gate | Pass for proposal | WRA-RQ-070 through WRA-RQ-086 are proposed in the milestone proposals and project requirements. | Software Architect |
| Scope Gate | Pass locally | The roadmap keeps implementation beyond approved M12, dependencies, live DAQ, HAL/RTOS, and certification claims behind later approval. | Project Coordinator |
| Human Approval Gate | Pass for M10 issue creation and implementation | User approved M10 issue creation and later approved external PR/issue/milestone actions on 2026-06-01. | Project Coordinator |
| Issue Planning Gate | Pass for M10 | GitHub milestone #10 and issues #132 through #137 were created, then closed through PR #138 and milestone closure. | GitHub Maintainer Specialist |
| Issue Planning Gate | Pass for M12 | GitHub milestone #12 and issues #149 through #155 were created after approval. | GitHub Maintainer Specialist |
| M13 Planning Gate | Pass | M13 proposal and issue-planning report define WRA-RQ-087 through WRA-RQ-092; GitHub milestone #13 and issues #158 through #163 created after user approval, then closed by PR #164. | Project Coordinator |
| M14 Planning Gate | Pass | M14 proposal and issue-planning report define WRA-RQ-093 through WRA-RQ-098; GitHub milestone #14 and issues #167 through #172 created after user approval, then closed by PR #173. | Project Coordinator |
| Implementation Gate | Pass for M14 | `high_pass_baseline` filter/config support, first-order recurrence, timing validation, metadata, CLI/config coverage, export guardrail coverage, docs, traceability, and risk updates merged in PR #173. | Core Software Engineer |
| Testing Gate | Pass for M14 | Focused M14 tests, full workspace tests, clippy, formatting, diff check, local Markdown link scan, and PR #173 protected `rust` CI pass. | Test Automation Engineer |
| Release Gate | Pass for M14 | PR #173 merged after required `rust` CI passed; squash commit `a17cd4c0ae7af5ab768688c9301484e5eb4799cf`. | GitHub Maintainer Specialist |
| Community Gate | Pass for M14 | Issues #167 through #172 closed and milestone #14 closed with 6 closed issues and 0 open issues. | Project Coordinator |
| Implementation Gate | Pass for M13 | M13 implementation merged in PR #164 with runtime-profile validator, timing evidence, tests, docs, and traceability. | Core Software Engineer |
| Release Gate | Pass for M13 | PR #164 merged after required `rust` CI passed; squash commit `ae0366dcd20a81a71262f38d2409dc2b85774051`. | GitHub Maintainer Specialist |
| Community Gate | Pass for M13 | Issues #158 through #163 closed and milestone #13 closed with 6 closed items and 0 open items. | Project Coordinator |
| Implementation Gate | Pass for M12 | M12 implementation merged in PR #156 with event records, validation records, examples, docs, and tests. | Core Software Engineer |
| Release Gate | Pass for M12 | PR #156 merged after required `rust` CI passed; squash commit `a4885578de9d136cd8df213e1da489a7232cf702`. | GitHub Maintainer Specialist |
| Community Gate | Pass for M12 | Issues #149 through #155 closed and milestone #12 closed with 8 closed items and 0 open items. | Project Coordinator |
| Release Gate | Pass for M11 | PR #147 merged after required `rust` CI passed; squash commit `793a2ab1323526b2695fa7b59a1246f2e29d9c43`. | GitHub Maintainer Specialist |
| Community Gate | Pass for M11 | Issues #140 through #146 are closed and milestone #11 is closed with 8 closed items and 0 open items. | Project Coordinator |

## Hand-Off Note

Role: Project Coordinator / Product Architect
Goal: Convert the transform taxonomy into a staged local milestone roadmap.
Files changed: This roadmap plus M10, M11, M12, issue-planning, requirements, traceability, risk, orchestration, and state files.
Checks run: Documentation and traceability inspection.
Status: M10, M11, M12, M13, and M14 complete; milestone #14 is closed with issues #167 through #172 closed by PR #173.
Known gaps: No GitHub release tag was published for M14; live DAQ, runtime loaders, hardware validation, certification evidence, and M15+ work remain separately gated.
Next recommended step: Hold before M15 or new scope until explicit approval.
