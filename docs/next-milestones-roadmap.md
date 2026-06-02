# Next Milestones Roadmap

Date: 2026-06-02

Status: M10 through M14 complete through merged PR evidence. M15 through M20 are complete as the MVP-exit sequence. M21 through M24 are complete as the first runtime-path follow-up. M25 through M36 are complete as the comprehensive filter and simulated signal-conditioning suite and merged to `main` through PR #175. No GitHub milestones/issues for M15-M36, release publication, dependency addition, runtime-loader implementation, live DAQ, HAL/RTOS, target hardware, or certification scope is created by this update.

## Purpose

FerrisOxide has completed the validated MVP, measurement/evidence engine, portable rule package, controller simulation/deployment config milestones through M9, transform/event/runtime-profile milestones through M14, MVP-exit hardening through M20, and the first narrow runtime-path follow-up through M24. M10 through M14 turned the analog transform taxonomy into implemented metadata, transform, event, validation, runtime-profile, and high-pass baseline correction work without claiming broad DSP support. M25 through M36 are the planned path to make FerrisOxide comprehensive for sampled DAQ/test waveform conditioning and calculations.

This roadmap records the completed transform milestones, completed local MVP-exit/runtime-path milestones, and the planned comprehensive conditioning milestones:

| Milestone | Working Version | Goal | Status |
|---|---|---|---|
| M10 | v0.8.0 | Transform architecture and capability metadata | Complete; PR #138 merged and milestone #10 closed |
| M11 | v0.9.0 | Pointwise and windowed transform MVP | Complete in PR #147; milestone #11 closed |
| M12 | v0.10.0 | Event and validation transform MVP | Complete in PR #156; milestone #12 closed |
| M13 | v0.11.0 | Transform runtime-profile validation | Complete in PR #164; milestone #13 closed |
| M14 | v0.12.0 | High-pass baseline correction | Complete in PR #173; milestone #14 closed |
| M15 | v0.13.0 | Config and schema reference hardening | Complete; merged in PR #175 |
| M16 | v0.14.0 | Report and artifact contract stabilization | Complete; merged in PR #175 |
| M17 | v0.15.0 | Desktop batch analysis workflow MVP | Complete; merged in PR #175 |
| M18 | v0.16.0 | Rule-package transform semantics | Complete; merged in PR #175 |
| M19 | v0.17.0 | Validation corpus and benchmark baseline expansion | Complete; merged in PR #175 |
| M20 | v0.18.0 | MVP exit readiness review | Complete; merged in PR #175 |
| M21 | mainline | Portable linear pointwise package semantics | Complete; merged in PR #175 |
| M22 | mainline | Shared runtime-compatible linear transform semantics | Complete; merged in PR #175 |
| M23 | mainline | Package compatibility corpus | Complete; merged in PR #175 |
| M24 | mainline | Runtime loader design gate | Complete as design only; merged in PR #175 |
| M25 | mainline | Transform registry and completeness contract | Complete; merged in PR #175 |
| M26 | mainline | Data cleaning and timing conditioning | Complete; merged in PR #175 |
| M27 | mainline | Pointwise, normalization, and nonlinear suite | Complete; merged in PR #175 |
| M28 | mainline | Smoothing, detrending, and baseline suite | Complete; merged in PR #175 |
| M29 | mainline | Standard frequency filter suite | Complete; merged in PR #175 |
| M30 | mainline | Resampling and timing alignment suite | Complete; merged in PR #175 |
| M31 | mainline | Envelope, energy, and calculus suite | Complete; merged in PR #175 |
| M32 | mainline | Statistical and correlation suite | Complete; merged in PR #175 |
| M33 | mainline | Spectrum, windows, and time-frequency suite | Complete; merged in PR #175 |
| M34 | mainline | Fault injection and ADC/DAC simulation suite | Complete; merged in PR #175 |
| M35 | mainline | Multi-channel, sensor, and domain conditioning packs | Complete; merged in PR #175 |
| M36 | mainline | Completeness, UX, and compatibility closure | Complete; merged in PR #175 |

## Sequencing Rationale

M10 comes first because the current implementation exposes a filter chain, string transform history, and a small set of implemented transforms. The taxonomy is broader: it includes pointwise transforms, windowed transforms, event transforms, feature extraction, validation transforms, calibration transforms, fault injection, and runtime/deployment constraints. A structured capability model is needed before adding more algorithms.

M11 adds low-risk transforms that can be implemented without new third-party crates and without changing the project's raw-data preservation rule. These transforms directly support DAQ cleanup workflows: offset/gain correction, clamping, deadband, DC removal, baseline subtraction, and moving median smoothing.

M12 adds test-oriented event and validation transforms for switch, relay, and controller-test workflows. These transforms should produce auditable event records first, then validation results linked to those records. That keeps signal interpretation separate from pass/fail decisions.

M13 follows because M10 defined runtime-profile compatibility rules but left validator code as a future gap. M11 and M12 now emit richer transform metadata, so FerrisOxide needs a code-level validator that can reject unsupported desktop, embedded-candidate, Pico-candidate, and future-gated transform exposure before future package or runtime claims are made.

M14 implements the deferred WRA-RQ-078 high-pass baseline correction in a narrow desktop-only slice. That closes the M11 timing-behavior gap without expanding into broad filter design, rule-package transform export, DAQ, HAL/RTOS, target hardware, or certification scope.

M15 through M20 are deliberately not a new algorithm sprint. They harden the existing surface before MVP exit: config reference, report/artifact contracts, local batch execution, rule-package transform semantics, validation corpus expansion, and an explicit MVP-exit readiness review.

M21 through M24 prove a narrow runtime-path slice for `offset`, `gain`, and `invert` package semantics, shared borrowed-slice transform semantics, positive/negative package fixtures, and a runtime-loader design gate.

M25 through M36 are the comprehensive filter and signal-conditioning path. M25 came first because broad algorithm expansion needed a central transform registry and completeness contract. M26 through M35 then added transform families in dependency-aware groups. M36 closed the suite by proving docs, fixtures, package/runtime compatibility, validation corpus, benchmark readiness, and user-facing completeness; PR #175 merged the suite to `main`. The next decision is a separately gated advanced follow-up or release-publication plan.

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

## MVP Exit Milestones

Primary artifact: `docs/mvp-exit-roadmap.md`

These milestones are complete for MVP exit and merged through PR #175. GitHub issue creation for M15-M20 and release publication still require separate approval.

| Milestone | Goal | Required Exit Evidence |
|---|---|---|
| M15 / v0.13.0 | Harden config and schema reference material. | Config reference, invalid-config matrix, example index, compatibility/deprecation policy, docs links, and existing tests passing. |
| M16 / v0.14.0 | Stabilize report and generated-artifact contracts. | Report/artifact contract docs, golden artifact matrix, compatibility gate language, and exact expected artifacts or approved additive changes. |
| M17 / v0.15.0 | Add a desktop batch analysis workflow MVP. | Batch input docs, deterministic per-run and aggregate outputs, partial-failure behavior, and tests proving existing single-run behavior remains unchanged. |
| M18 / v0.16.0 | Define rule-package transform semantics. | Transform/package compatibility matrix, supported/rejected export behavior, runtime-profile guardrail tests, and no binary/signing/runtime/hardware claims. |
| M19 / v0.17.0 | Expand validation corpus and benchmark baseline. | Validation index, known-answer fixtures, negative cases, exact report evidence, benchmark log, and scope-limited performance wording. |
| M20 / v0.18.0 | Run explicit MVP exit readiness review. | MVP exit readiness report, requirements/traceability/risk audit, docs/onboarding audit, release/community/retrospective gates, and a `Pass`, `Fail`, or `Blocked` decision. |

## Future-Gated Work

The following remain outside M10 through M20 unless a fresh proposal and approval gate moves them forward:

- Advanced FIR/IIR filter families beyond the current simple first-order filters.
- FFT, spectrum analysis, time-frequency analysis, and wavelet workflows.
- Resampling, clock drift correction, and timestamp-grid repair.
- Sensor-specific calibration libraries for thermocouples, RTDs, strain gauges, load cells, LVDTs, microphones, or photodiodes.
- Live DAQ vendor SDKs, drivers, or hardware input.
- HAL, RTOS SDK, target hardware bindings, unsafe FFI, or real-time runtime guarantees.
- Binary deployment package serialization, cryptographic signing, authentication, or tamper-proof claims.
- Hardware qualification, flight certification, or regulatory compliance evidence.
- GUI, web UI, plugin runtime, hosted service, database-backed workflow, scheduler, and embedded plotting.
- Enforcing new runtime/package behavior beyond M13 validator scope or M14 rule-package guardrails unless a separate schema/export migration is approved.

## Gate Decisions

| Gate | Decision | Evidence | Next Owner |
|---|---|---|---|
| Intake Gate | Pass | User supplied analog-transform taxonomy and asked to move forward with next milestones. | Project Coordinator |
| Roadmap Gate | Pass locally | This document sequences M10 through M20 from the taxonomy, current project state, and MVP-exit risks. | Project Orchestrator |
| Requirements Gate | Pass | WRA-RQ-070 through WRA-RQ-105 are implemented or locally closed in milestone proposals, project requirements, and the MVP-exit roadmap. | Software Architect |
| Scope Gate | Pass locally | The roadmap keeps implementation beyond approved M14, dependencies, live DAQ, HAL/RTOS, hardware, release publication, and certification claims behind later approval. | Project Coordinator |
| Human Approval Gate | Pass for M10 issue creation and implementation | User approved M10 issue creation and later approved external PR/issue/milestone actions on 2026-06-01. | Project Coordinator |
| Issue Planning Gate | Pass for M10 | GitHub milestone #10 and issues #132 through #137 were created, then closed through PR #138 and milestone closure. | GitHub Maintainer Specialist |
| Issue Planning Gate | Pass for M12 | GitHub milestone #12 and issues #149 through #155 were created after approval. | GitHub Maintainer Specialist |
| M13 Planning Gate | Pass | M13 proposal and issue-planning report define WRA-RQ-087 through WRA-RQ-092; GitHub milestone #13 and issues #158 through #163 created after user approval, then closed by PR #164. | Project Coordinator |
| M14 Planning Gate | Pass | M14 proposal and issue-planning report define WRA-RQ-093 through WRA-RQ-098; GitHub milestone #14 and issues #167 through #172 created after user approval, then closed by PR #173. | Project Coordinator |
| MVP Exit Roadmap Gate | Pass locally | `docs/mvp-exit-roadmap.md` defines M15 through M20, MVP-exit criteria, outside-MVP scope, and gate decisions. | Project Coordinator |
| Human Approval Gate For M15-M20 | Pass for local implementation and later PR #175 merge | User approved continuing the implementation pipeline through MVP exit on 2026-06-01, then requested the documentation update and mainline merge on 2026-06-02. GitHub milestones/issues and release publication remain separately gated. | User / Project Coordinator |
| M15-M20 Implementation Gate | Pass locally | `docs/config-reference.md`, `docs/artifact-contract.md`, `docs/batch-analysis-workflow.md`, `docs/transform-package-compatibility.md`, `docs/validation-corpus-index.md`, `docs/mvp-exit-readiness-report.md`, and `crates/ferrisoxide-cli/src/main.rs` complete the local MVP-exit scope. | Project Orchestrator |
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
Goal: Track the staged FerrisOxide milestone roadmap from transform architecture through comprehensive-suite closure.
Files changed: This roadmap plus comprehensive-suite roadmap, MVP-exit/runtime-path reports, requirements, traceability, risk, orchestration, and state files.
Checks run: See `docs/validation-log.md`.
Status: M10 through M14 are complete through GitHub issue/milestone evidence; M15 through M36 are complete and merged through PR #175.
Known gaps: No GitHub release tag was published for M14 or M15-M36; GitHub milestones/issues are not created for M15 through M36; live DAQ, runtime loaders, hardware validation, certification evidence, and advanced follow-up work remain separately gated.
Next recommended step: Choose one gated advanced follow-up or a separate release-publication plan.
