# Project State

Last updated: 2026-05-31

## Current Objective

Correct the main repository host to `ferrisoxide`, then work the remaining open v0.6.0 portable rule package issues through the protected-branch pipeline.

## Current Stage

Milestone #7, `v0.5.0: Measurement-Backed Criteria DSL`, is closed after PRs #101 through #105 completed issues #57 through #61 and the required `rust` CI passed. M8-001 / issue #67 is complete through PR #106: `crates/ferrisoxide-rule-schema` owns the first versioned portable FerrisOxide Rule Package schema types for package metadata, target profiles, sample timing, channels, units, thresholds, filters, measurement-backed criteria, and unit-bearing requirements. M8-002 / issue #71 is complete through PR #107: `docs/rule-package-format.md` defines `rules.toml`, `rules.json`, future `rules.bin`, `manifest.json`, `checksum.txt`, `validation-report.json`, and `qualification-evidence.svg` roles, with parse-tested examples in `examples/rule-package/`. M8-003 / issue #68 is complete through PR #108: `ferrisoxide-rule-schema` validates packages before export or execution with structured error kind, field, and message values. M8-004 / issue #69 is complete through PR #109: the CLI exports `rules.toml`, `rules.json`, and `validation-report.json` from validated config and analysis evidence, validates the package before writing, compares expected artifacts in tests, and refuses to overwrite existing artifacts. M8-005 / issue #70 is complete through PR #110: the schema crate defines deterministic manifest/checksum metadata, the CLI export also writes `manifest.json` and `checksum.txt`, and tests cover deterministic checksum output, checksum mismatch errors, manifest metadata, and exact exported artifacts. REPO-001 / issue #111 is complete through PR #112: the main repository host, local workspace path, and current repository metadata are corrected to `kota-wilson/ferrisoxide`, while `ferrisoxide-signal` remains the signal-analysis crate and CLI binary. M8-006 / issue #73 is complete through PR #113: `crates/ferrisoxide-rule-engine` owns shared criteria semantics over caller-provided slices, `ferrisoxide-core` delegates desktop analysis to it, and `ferrisoxide-embedded` has host tests that evaluate embedded-compatible slices through the same engine. M8-007 / issue #72 is complete through PR #114: `ferrisoxide-rule-engine` is `#![no_std]`, desktop-style owned evidence uses `alloc`, and the borrowed summary API returns borrowed/static result and error data for basic no-heap embedded-compatible evaluation where practical. M8-008 / issue #74 is implemented locally on branch `feature/m8-008-rule-parity-tests`: `tests/parity/` contains a waveform fixture, rule package, and expected JSON report, and `crates/ferrisoxide-core/tests/rule_parity.rs` verifies exact portable evidence parity between the desktop core path and embedded-compatible borrowed-rule path. PR #75 planned milestone #8, `v0.6.0: Portable Rule Package System`, with issues #67 through #74 for desktop rule authoring/export and embedded/controller deployment through one schema and one shared rule engine. PR #87 planned milestone #9, `v0.7.0: Controller Simulation and Deployment Config System`, with issues #77 through #86 for desktop digital-twin simulation, separate production control and test verification configs, deployment packages, and RTOS verification mode. PR #90 closed issue #89 and defined Apple Silicon desktop authoring plus Raspberry Pi 5 bare-metal ARM64 embedded runtime platform profiles. PR #93 closed issue #92 and defined Raspberry Pi Pico 2 as a future optional microcontroller runtime profile for compact deterministic rule execution. PR #96 closed issue #95 and documented FerrisOxide as a proposed umbrella brand. PR #99 closed issue #98 and completed BRAND-002 adoption of FerrisOxide Signal across in-repository package names, CLI binary, docs, examples, and scripts. GUI, live DAQ vendor SDKs, embedded plotting, hardware HALs, Pico 2 runtime implementation, unsafe FFI, RTOS SDK integration, plugin runtime, batch analysis, production readiness, certification claims, binary package serialization, cryptographic signing, and external brand expansion remain out of scope until separately gated.

## Open Risks

- Risk: CSV dialect and units may vary across DAQ exports.
  Owner: Software Architect
- Risk: Filter outputs may be misinterpreted if phase, latency, and sample-rate assumptions are undocumented.
  Owner: Systems Engineer
- Risk: MVP scope could expand into GUI, real-time DAQ, or certification claims.
  Owner: Project Coordinator
- Risk: Approved third-party crates may introduce transitive license or supply-chain risk.
  Owner: Security Engineer
- Risk: Future RTOS adapters may accidentally pull desktop-only concerns such as CSV parsing, file I/O, or report generation into embedded crates.
  Owner: Software Architect
- Risk: Users may overread MVP validation evidence as hardware validation, RTOS readiness, or certification support.
  Owner: Project Coordinator / Documentation Engineer
- Risk: ADC quantization settings may hide analog excursions when the configured input range or resolution is unrealistic.
  Owner: Electrical Signal Integrity Engineer / Documentation Engineer
- Risk: Feature completion may be mistaken for scientifically validated signal-analysis accuracy.
  Owner: Verification and Validation Engineer / Software Architect
- Risk: Software validation fixtures may be mistaken for hardware qualification or certification evidence.
  Owner: Verification and Validation Engineer / Documentation Engineer
- Risk: Benchmark results may be overread as production performance guarantees.
  Owner: Performance Engineer
- Risk: Plotting dependencies or future plotting backends may expand desktop scope into GUI, bitmap, or embedded paths.
  Owner: Software Architect / Security Engineer
- Risk: Embedded prototype artifacts may be mistaken for production RTOS, hardware, or certification readiness.
  Owner: Embedded RTOS Engineer / Documentation Engineer
- Risk: Target-specific ARM64 and Zephyr toolchains may drift or require global setup if adopted too early.
  Owner: DX Engineer / Security Engineer
- Risk: Measurement extraction may subtly change evidence sample selection or reported values.
  Owner: Verification and Validation Engineer / Core Software Engineer
- Risk: Report schema migration may break consumers that assumed all evidence lived only in results.
  Owner: Documentation Engineer / Core Software Engineer
- Risk: Annotated SVG evidence may be overread as complete engineering proof instead of visual software evidence.
  Owner: Documentation Engineer / Verification and Validation Engineer
- Risk: Criteria DSL expansion may introduce ambiguous operators or unit parsing.
  Owner: Software Architect / Core Software Engineer
- Risk: Criteria DSL implementation may break existing TOML configs or report consumers.
  Owner: Core Software Engineer / V&V Engineer
- Risk: Desktop and embedded/controller rule behavior may drift if rule semantics are duplicated.
  Owner: Software Architect / Verification and Validation Engineer
- Risk: Deployment packages may be mistaken for certified controller releases or hardware qualification evidence.
  Owner: Documentation Engineer / Flight Certification Assurance Engineer
- Risk: Manifest, checksum, or binary package work may create security/dependency overclaims.
  Owner: Security Engineer / Core Software Engineer
- Risk: Production control config and test verification config may be conflated.
  Owner: Software Architect / Verification and Validation Engineer
- Risk: DAQ or controller I/O abstractions may pull in vendor SDK, HAL, or hardware assumptions too early.
  Owner: Systems Engineer / Security Engineer
- Risk: Desktop virtual controller behavior may drift from RTOS runtime behavior.
  Owner: Embedded RTOS Engineer / Core Software Engineer
- Risk: Treating the embedded target as generic RTOS may obscure Raspberry Pi 5 bare-metal constraints.
  Owner: Embedded RTOS Engineer / DX Engineer
- Risk: Pico 2 may be over-scoped as a full controller-computer runtime despite microcontroller memory and I/O constraints.
  Owner: Embedded RTOS Engineer / Documentation Engineer
- Risk: FerrisOxide repository, product-family, and crate names may be confused, break links or scripts, overclaim Rust affiliation, or conflict with unavailable external names.
  Owner: Product Architect / GitHub Maintainer Specialist

## Pending Decisions

- Decision: Confirm MIT license before external publication.
  Owner: Project Coordinator
  Status: Accepted in `decisions/ADR-002-license-assumption.md`.
- Decision: Select production CSV parsing crate after dependency review.
  Owner: Software Architect / Security Engineer
  Status: Accepted in `docs/dependency-review.md`.
- Decision: Choose report format priority: text-first or JSON-first.
  Owner: Product / Documentation Engineer
  Status: Text and JSON are both implemented for MVP.
- Decision: Start the embedded path with `ferrisoxide-signal` before `ferrisoxide-embedded`, QEMU, Embassy-style, or Zephyr integration.
  Owner: Software Architect
  Status: Accepted in `docs/embedded-roadmap.md`.
- Decision: Add optional desktop SVG plotting through an isolated `ferrisoxide-plot` crate using Plotters SVG line rendering.
  Owner: Software Architect / Security Engineer
  Status: Accepted for M5 in `docs/dependency-review.md` and `docs/m5-plotting-pipeline-report.md`.
- Decision: Keep M3 RTOS follow-up work host-checkable and adapter/prototype-only until a fresh target-toolchain gate.
  Owner: Embedded RTOS Engineer / Verification and Validation Engineer
  Status: Accepted in `docs/m3-rtos-follow-up-pipeline-report.md`.
- Decision: Start v0.4.0 with a local `ferrisoxide-measurements` crate before report schema, annotated SVG, batch, or plugin work.
  Owner: Software Architect / Verification and Validation Engineer
  Status: Accepted in `docs/m6-measurement-engine-pipeline-report.md`.
- Decision: Treat embedded/controller support as a deployment target for one portable rule schema and one shared rule engine, not as a forked rule implementation.
  Owner: Software Architect / Embedded RTOS Engineer
  Status: Accepted for future milestone planning in `decisions/ADR-004-portable-rule-package-architecture.md`.
- Decision: Keep production control config and test verification config separate, linked by a deployment manifest and parity evidence.
  Owner: Software Architect / Verification and Validation Engineer
  Status: Accepted for future milestone planning in `docs/controller-in-the-loop-workflow.md`.
- Decision: Define Apple Silicon macOS as the desktop authoring platform and Raspberry Pi 5 bare-metal ARM64 as the first-class embedded runtime target.
  Owner: Software Architect / Embedded RTOS Engineer
  Status: Accepted for future milestone planning in `docs/platform-targets.md`.
- Decision: Define Raspberry Pi Pico 2 as an optional future microcontroller runtime profile, not a replacement for the Raspberry Pi 5 embedded runtime.
  Owner: Software Architect / Embedded RTOS Engineer
  Status: Accepted for future milestone planning in `docs/platform-targets.md`; runtime crate remains deferred.
- Decision: Use FerrisOxide Signal as the adopted in-repository product identity while keeping external namespace/legal checks gated.
  Owner: Product Architect / GitHub Maintainer Specialist
  Status: Implemented through BRAND-002 / issue #98 / PR #99 in `decisions/ADR-006-ferrisoxide-signal-identity-adoption.md`; organization, domain, crates.io, trademark, logo, and legal-suitability checks remain deferred.
- Decision: Use FerrisOxide as the main repository host while keeping FerrisOxide Signal as the current signal-analysis crate and CLI identity.
  Owner: Product Architect / GitHub Maintainer Specialist
  Status: Implemented through REPO-001 / issue #111 / PR #112 in `decisions/ADR-007-repository-host-ferrisoxide.md`.

## Next Responsible Role

Role: Project Orchestrator / Core Software Engineer

Expected deliverable: Complete M8-008 PR/CI/merge and verify milestone #8 has no open issues.

## Orchestration Status

- Execution tier: Tier 2 MVP.
- Selected workflow: Project orchestration plus open-source library and data-analysis workflows.
- Repository URL: `https://github.com/kota-wilson/ferrisoxide`.
- Current milestone: #8, `v0.6.0: Portable Rule Package System`; M7 is closed; M8-005 / issue #70 is complete through PR #110; BRAND-002 / issue #98 is complete; REPO-001 / issue #111 is complete through PR #112; M8-006 / issue #73 is complete through PR #113; M8-007 / issue #72 is complete through PR #114; M8-008 / issue #74 is implemented locally; future milestone #9, `v0.7.0: Controller Simulation and Deployment Config System`, is planned; issue #89 added platform profiles to milestone #9 and issue #92 added the optional Pico 2 micro-runtime profile.
- Completed recent milestones: Dependency-reviewed MVP slice; `M3: RTOS / embedded no_std foundation`; `M4: Signal Accuracy and Validation`; `M5: Plotting and Visualization`; `v0.4.0: Measurement & Evidence Engine`.
- Next gate: Validate and merge the M8-008 parity-test PR, then confirm no open M8 issues remain.
- Stop condition: Stop before adding target toolchains, SDKs, HALs, unsafe FFI, QEMU boot image work, Pico 2 runtime crate work, new third-party dependencies, GUI/DAQ/embedded plotting/certification work, plugin runtime, batch analysis, binary package serialization, cryptographic signing, rule execution behavior, unit shorthand parsing, new measurements, expanded annotated SVG features, or external brand expansion without a fresh issue/gate.

## Granularity Status

- Current expected zoom level: levels 1-3 for architecture, levels 3-5 for first implementation task.
- Required artifacts: project charter, requirements, risk register, traceability matrix, architecture, orchestration plan, repository MVP slice.
- Abstraction review status: Required after architecture plan.

## Environment Status

- Project root: `/Users/kota/Desktop/softwareai/projects/ferrisoxide`.
- Isolation level: Level 1 Cargo workspace.
- Local environment: Rust/Cargo; no global dependencies installed.
- Dependency status: Approved crates added and pinned in `Cargo.lock`; see `docs/dependency-review.md`. M3 follow-up adds no third-party dependencies; `ferrisoxide-embedded` depends only on local `ferrisoxide-signal`. M6-001 adds no third-party dependencies; `ferrisoxide-measurements` is local and dependency-free.

## Traceability Status

- Requirements: `requirements.md`.
- Traceability matrix: `traceability-matrix.md`.
- Verification matrix: `traceability-matrix.md` updated with current MVP, M3-RTOS-001, WRA-RQ-018 ADC quantization evidence, M1 metadata evidence, M4 requirements WRA-RQ-019 through WRA-RQ-026, M5 requirement WRA-RQ-027, M3 follow-up requirements WRA-RQ-028 through WRA-RQ-030, M6 requirements WRA-RQ-031 through WRA-RQ-035, WRA-RQ-036 release evidence for issue #55, WRA-RQ-037 and WRA-RQ-038 release evidence for issue #56, WRA-RQ-039 release evidence for issue #57 / PR #101, WRA-RQ-040 release evidence for issue #58 / PR #102, WRA-RQ-041 release evidence for issue #59 / PR #103, WRA-RQ-042 release evidence for issues #60 and #61 / PRs #104 and #105, WRA-RQ-043 release evidence for issues #67 and #71, WRA-RQ-044 release evidence for issue #71 / PR #107, WRA-RQ-045 release evidence for issue #68 / PR #108, WRA-RQ-046 release evidence for issue #69 / PR #109, WRA-RQ-047 release evidence for issue #70 / PR #110, WRA-RQ-048 release evidence for issue #73 / PR #113, WRA-RQ-049 release evidence for issue #72 / PR #114, WRA-RQ-050 local evidence for issue #74, WRA-RQ-051 through WRA-RQ-060 mapped to issues #77 through #86, WRA-RQ-061 release evidence for issue #89, WRA-RQ-062 release evidence for issue #92, WRA-RQ-063 release evidence for issue #95, WRA-RQ-064 release evidence for issue #98 / PR #99, and WRA-RQ-065 release evidence for issue #111 / PR #112.

## Gate Decisions

| Gate | Decision | Evidence | Next Owner |
|---|---|---|---|
| Intake Gate | Pass | `docs/product-prompt.md` | Project Coordinator |
| Project Creation Gate | Pass | Required project files and repository structure exist | Project Orchestrator |
| Environment Gate | Pass | No global setup; Cargo workspace only | DX Engineer |
| Architecture Gate | Pass for dependency-free MVP slice | `docs/architecture.md` | Abstraction Review Engineer |
| Granularity Gate | Pass | `docs/abstraction-review.md` | Project Orchestrator |
| Implementation Gate | Pass | `docs/implementation-report.md` | Test Automation Engineer |
| Testing Gate | Pass | `docs/validation-log.md` | Project Orchestrator |
| Dependency Gate | Pass | `docs/dependency-review.md` | Core Software Engineer |
| Release Gate | Pass | `docs/release-readiness.md`; public repository created and initial CI passed | Community Engineering Lead |
| V&V Gate | Pass | `docs/verification-validation-report.md` | QA Engineer |
| QA Gate | Pass | `docs/qa-review.md` | Security Engineer |
| Security Gate | Pass | `docs/security-review.md` | Performance Engineer |
| Performance Gate | Pass for MVP | `docs/performance-review.md` | Documentation Engineer |
| Documentation Gate | Pass | `docs/documentation-review.md` | Code Reviewer |
| Code Review Gate | Pass | `docs/code-review.md` | Evaluation Engineer |
| Evaluation Gate | Pass | `docs/evaluation-report.md` | Community Engineering Lead |
| Community Gate | Pass | `docs/community-report.md` | Project Coordinator |
| Retrospective Gate | Pass | `docs/retrospective.md` | Community Engineering Lead |
| Architecture Decision Gate | Pass | `decisions/ADR-003-filter-pipeline-architecture.md` | Core Software Engineer |
| GitHub Issue Planning Gate | Pass | M1 issues #1-#7 created under `M1: Validated MVP` | Project Orchestrator |
| M1-001 Requirements Gate | Pass | Issue #1 acceptance criteria captured in `docs/m1-001-csv-parser-edge-cases.md` | Software Architect |
| M1-001 Implementation Gate | Pass | `crates/ferrisoxide-core/src/csv.rs`, `docs/implementation-report.md` | Test Automation Engineer |
| M1-001 Testing Gate | Pass | `docs/validation-log.md`; targeted parser tests, workspace tests, fmt, and clippy passed | Project Orchestrator |
| M1-001 Release Gate | Pass | PR #22 merged after `rust` CI passed: `https://github.com/kota-wilson/ferrisoxide-signal/pull/22` | Community Engineering Lead |
| M1-001 Community Gate | Pass | PR #22 body links issue #1 and validation commands | Project Coordinator |
| v0.2.0 Planning Gate | Pass | M2 issues #8-#15 created under `v0.2.0: waveform criteria engine` | Core Software Engineer |
| v0.2.0 Implementation Gate | Pass | PR #16 merged with waveform fixtures, criteria, config validation, and golden JSON tests | Test Automation Engineer |
| M3 Issue Planning Gate | Pass | M3 milestone plus issues #17-#20 created | Project Orchestrator |
| M3-RTOS-001 Requirements Gate | Pass | `requirements.md`, `traceability-matrix.md` include WRA-RQ-017 | Software Architect |
| M3-RTOS-001 Architecture Gate | Pass | `docs/embedded-roadmap.md`, `crates/ferrisoxide-signal/no_std-design.md` | Core Software Engineer |
| M3-RTOS-001 Implementation Gate | Pass | `docs/implementation-report.md`, `crates/ferrisoxide-signal/` | Test Automation Engineer |
| M3-RTOS-001 Testing Gate | Pass | `docs/validation-log.md` | Verification and Validation Engineer |
| M3-RTOS-001 V&V Gate | Pass | `docs/m3-rtos-001-pipeline-report.md` | QA Engineer |
| M3-RTOS-001 QA Gate | Pass | `docs/m3-rtos-001-pipeline-report.md` | Security Engineer |
| M3-RTOS-001 Security Gate | Pass | `docs/m3-rtos-001-pipeline-report.md`, `cargo tree -p ferrisoxide-signal` | Performance Engineer |
| M3-RTOS-001 Performance Gate | Pass | `docs/m3-rtos-001-pipeline-report.md`, fixed-size and O(1) streaming state inspection | Documentation Engineer |
| M3-RTOS-001 Documentation Gate | Pass | `README.md`, `CHANGELOG.md`, `docs/embedded-roadmap.md`, `crates/ferrisoxide-signal/README.md` | Code Reviewer |
| M3-RTOS-001 Code Review Gate | Pass | `docs/m3-rtos-001-pipeline-report.md` | Evaluation Engineer |
| M3-RTOS-001 Evaluation Gate | Pass | `docs/m3-rtos-001-pipeline-report.md` | Release Engineer |
| M3-RTOS-001 Release Gate | Pass | PR #21 merged after `rust` CI passed: `https://github.com/kota-wilson/ferrisoxide-signal/pull/21` | Community Engineering Lead |
| M3-RTOS-001 Community Gate | Pass | PR #21 body links issue #20 and follow-up issues #17-#19 | Project Coordinator |
| M3-RTOS-001 Retrospective Gate | Pass | `docs/m3-rtos-001-pipeline-report.md` | Project Orchestrator |
| Domain End-User Review Gate | Pass | `docs/end-user-design-review.md`; PR #23 | Documentation Engineer |
| ADC Quantization Implementation Gate | Pass | `crates/ferrisoxide-core/src/filter.rs`, `crates/ferrisoxide-core/src/config.rs`, `crates/ferrisoxide-cli/src/main.rs`, `docs/implementation-report.md` | Test Automation Engineer |
| ADC Quantization Testing Gate | Pass | `docs/validation-log.md`; unit, config, CLI, and workspace tests passed | Release Engineer |
| ADC Quantization Release Gate | Pass | PR #25 merged after required `rust` CI passed | Project Orchestrator |
| Documentation Accuracy Gate | Pass | `docs/documentation-audit-2026-05-31.md`; fmt, workspace tests, clippy, whitespace, link-target, and stale-status scans passed | Project Coordinator |
| M1 Metadata / README Implementation Gate | Pass | `crates/ferrisoxide-core/src/model.rs`, `crates/ferrisoxide-core/src/report.rs`, README, golden JSON reports, `docs/report-schema.md` | Test Automation Engineer |
| v0.3.0 Planning Gate | Pass | `docs/v0.3.0-validation-roadmap.md`, `validation/`, M4 issues #27-#34 | GitHub Maintainer Specialist |
| M4 Requirements Gate | Pass | `requirements.md`, `traceability-matrix.md` | Software Architect |
| M4 Architecture Gate | Pass | `docs/architecture.md`, `docs/filter-behavior.md`, `docs/time-axis-and-tolerances.md` | Abstraction Review Engineer |
| M4 Implementation Gate | Pass | `docs/implementation-report.md`, core/CLI code, validation fixtures, benchmark helper | Test Automation Engineer |
| M4 Testing Gate | Pass | `docs/validation-log.md`; fmt, workspace tests, clippy, diff check, CLI smokes, benchmark command | Verification and Validation Engineer |
| M4 V&V Gate | Pass | `docs/verification-validation-report.md`, known-answer expected measurements, exact-report tests | QA Engineer |
| M4 QA Gate | Pass | `docs/qa-review.md` | Security Engineer |
| M4 Security Gate | Pass | `docs/security-review.md`; no new dependencies or unsafe/network surface | Performance Engineer |
| M4 Performance Gate | Pass | `docs/performance-review.md`, `docs/benchmarking.md`, `ferrisoxide-signal-bench` | Documentation Engineer |
| M4 Documentation Gate | Pass | `docs/documentation-review.md`, README, report schema, validation docs | Code Reviewer |
| M4 Code Review Gate | Pass for PR creation | `docs/code-review.md`, `docs/m4-signal-validation-pipeline-report.md` | Evaluation Engineer |
| M4 Evaluation Gate | Pass | `docs/evaluation-report.md` | Release Engineer |
| M4 Release Gate | Pass | PR #36 merged after required `rust` CI passed; merge commit `a0d381556ff5f5d044f230217b335b73b3b57608` | GitHub Maintainer Specialist |
| M4 Community Gate | Pass | Issues #27-#34 closed; M4 milestone #4 closed with 8 closed issues and 0 open issues | Project Coordinator |
| M5 Requirements Gate | Pass | `requirements.md` WRA-RQ-027; issue #38 | Software Architect |
| M5 Architecture Gate | Pass | `docs/architecture.md`, `docs/plotting.md`, `docs/dependency-review.md` | Abstraction Review Engineer |
| M5 Human Approval Gate | Pass | User approved adding the Plotters dependency and PR creation | Core Software Engineer |
| M5 Implementation Gate | Pass | `crates/ferrisoxide-plot/`, `crates/ferrisoxide-cli/src/main.rs`, plotting fixture and docs | Test Automation Engineer |
| M5 Testing Gate | Pass | `docs/validation-log.md`; fmt, workspace tests, clippy, 2D/3D CLI smokes, metadata/tree inspection, diff check | Verification and Validation Engineer |
| M5 V&V Gate | Pass | `docs/verification-validation-report.md`, WRA-RQ-027 traceability | QA Engineer |
| M5 QA Gate | Pass | `docs/qa-review.md` | Security Engineer |
| M5 Security Gate | Pass | `docs/security-review.md`, `docs/dependency-review.md`, `cargo tree -p ferrisoxide-plot` | Performance Engineer |
| M5 Performance Gate | Pass | `docs/performance-review.md`; no unsupported performance claims | Documentation Engineer |
| M5 Documentation Gate | Pass | README, `docs/usage-mvp.md`, `docs/plotting.md` | Code Reviewer |
| M5 Code Review Gate | Pass | `docs/code-review.md`, `docs/m5-plotting-pipeline-report.md` | Evaluation Engineer |
| M5 Evaluation Gate | Pass | `docs/evaluation-report.md` | Release Engineer |
| M5 Release Gate | Pass | PR #39 merged after required `rust` CI passed; merge commit `9bc3d53bf416fff7e280abbcc24840c34811918f` | GitHub Maintainer Specialist |
| M5 Community Gate | Pass | Issue #38 closed; M5 milestone #5 closed with 1 closed issue and 0 open issues | Project Coordinator |
| M3 RTOS Follow-Up Requirements Gate | Pass | `requirements.md` WRA-RQ-028 through WRA-RQ-030; issues #17-#19 | Software Architect |
| M3 RTOS Follow-Up Architecture Gate | Pass | `docs/architecture.md`, `docs/embedded-roadmap.md`, `crates/ferrisoxide-embedded/no_std-design.md` | Abstraction Review Engineer |
| M3 RTOS Follow-Up Implementation Gate | Pass | `crates/ferrisoxide-embedded/`, `embedded/arm64/qemu/`, `embedded/arm64/zephyr/` | Test Automation Engineer |
| M3 RTOS Follow-Up Testing Gate | Pass | `docs/validation-log.md`; workspace tests, QEMU demo manifest test, clippy, dependency tree | Verification and Validation Engineer |
| M3 RTOS Follow-Up V&V Gate | Pass | `docs/verification-validation-report.md`, WRA-RQ-028 through WRA-RQ-030 traceability | QA Engineer |
| M3 RTOS Follow-Up QA Gate | Pass | `docs/qa-review.md` | Security Engineer |
| M3 RTOS Follow-Up Security Gate | Pass | `docs/security-review.md`, `cargo tree -p ferrisoxide-embedded` | Performance Engineer |
| M3 RTOS Follow-Up Performance Gate | Pass | `docs/performance-review.md`; no unsupported target/RTOS performance claims | Documentation Engineer |
| M3 RTOS Follow-Up Documentation Gate | Pass | README, `docs/embedded-roadmap.md`, `crates/ferrisoxide-embedded/README.md`, QEMU and Zephyr READMEs | Code Reviewer |
| M3 RTOS Follow-Up Code Review Gate | Pass | `docs/code-review.md`, `docs/m3-rtos-follow-up-pipeline-report.md` | Evaluation Engineer |
| M3 RTOS Follow-Up Evaluation Gate | Pass | `docs/evaluation-report.md` | Release Engineer |
| M3 RTOS Follow-Up Release Gate | Pass | PR #41 merged after required `rust` CI passed; merge commit `36e6d20523c14441e493f7fd48d4776e891f894a` | GitHub Maintainer Specialist |
| M3 RTOS Follow-Up Community Gate | Pass | Issues #17-#20 closed; M3 milestone #3 closed with 4 closed issues and 0 open issues | Project Coordinator |
| M6 Issue Planning Gate | Pass | Milestone #6 and issues #43-#47 created | Software Architect |
| M6 Requirements Gate | Pass | `requirements.md` WRA-RQ-031; issue #43 | Software Architect |
| M6 Architecture Gate | Pass | `docs/architecture.md`, `docs/measurements.md`, `docs/m6-measurement-engine-pipeline-report.md` | Abstraction Review Engineer |
| M6 Implementation Gate | Pass | `crates/ferrisoxide-measurements/`, `crates/ferrisoxide-core/src/analysis.rs`, `crates/ferrisoxide-core/src/criteria.rs` | Test Automation Engineer |
| M6 Testing Gate | Pass | `docs/validation-log.md`; fmt, workspace tests, clippy, dependency tree, and diff check passed | Verification and Validation Engineer |
| M6 V&V Gate | Pass | `docs/verification-validation-report.md`, WRA-RQ-031 traceability | QA Engineer |
| M6 QA Gate | Pass | `docs/qa-review.md` | Security Engineer |
| M6 Security Gate | Pass | `docs/security-review.md`, `docs/dependency-review.md`, `cargo tree -p ferrisoxide-measurements` | Performance Engineer |
| M6 Performance Gate | Pass | `docs/performance-review.md`; no unsupported performance claims | Documentation Engineer |
| M6 Documentation Gate | Pass | README, `docs/measurements.md`, `crates/ferrisoxide-measurements/README.md` | Code Reviewer |
| M6 Code Review Gate | Pass | `docs/code-review.md`, `docs/m6-measurement-engine-pipeline-report.md` | Evaluation Engineer |
| M6 Evaluation Gate | Pass | `docs/evaluation-report.md` | Release Engineer |
| M6 Release Gate | Pass | PR #48 merged after required `rust` CI passed; merge commit `559c96151f6f1d9a99d3d399a0e6bd046bfe5f51` | GitHub Maintainer Specialist |
| M6 Community Gate | Pass for M6-001 | Issue #43 closed; remaining milestone #6 issues later closed by PR #50 and PR #52 | Project Coordinator |
| M6-003 Requirements Gate | Pass | `requirements.md` WRA-RQ-032; issue #45 | Software Architect |
| M6-003 Architecture Gate | Pass | `docs/report-schema.md`, `docs/measurements.md`, `docs/m6-report-measurement-schema-pipeline-report.md` | Abstraction Review Engineer |
| M6-003 Implementation Gate | Pass | `crates/ferrisoxide-core/src/analysis.rs`, `crates/ferrisoxide-core/src/report.rs`, `crates/ferrisoxide-cli/src/main.rs`, exact golden reports | Test Automation Engineer |
| M6-003 Testing Gate | Pass | Measurement-link unit test, report tests, CLI tests, exact golden JSON tests, workspace tests | Verification and Validation Engineer |
| M6-003 V&V Gate | Pass | `docs/m6-report-measurement-schema-pipeline-report.md`, WRA-RQ-032 traceability | QA Engineer |
| M6-003 Release Gate | Pass | PR #50 merged after required `rust` CI passed; merge commit `f7e21695f501890669d591d0d7cbc9b731a541bb` | GitHub Maintainer Specialist |
| M6-003 Community Gate | Pass | Issue #45 closed; remaining milestone #6 issues later closed by PR #52 | Project Coordinator |
| M6 Completion Requirements Gate | Pass | `requirements.md` WRA-RQ-033 through WRA-RQ-035; issues #44, #46, and #47 | Software Architect |
| M6 Completion Architecture Gate | Pass | `docs/architecture.md`, `docs/plotting.md`, `docs/criteria-dsl.md`, `docs/m6-completion-pipeline-report.md` | Abstraction Review Engineer |
| M6 Completion Implementation Gate | Pass | `crates/ferrisoxide-plot/src/lib.rs`, `crates/ferrisoxide-cli/src/main.rs`, `validation/measurement_engine/`, docs | Test Automation Engineer |
| M6 Completion Testing Gate | Pass | Workspace tests, annotated SVG CLI smoke, exact measurement-engine report test, and PR #52 required `rust` CI | Verification and Validation Engineer |
| M6 Completion V&V Gate | Pass | `docs/m6-completion-pipeline-report.md`, WRA-RQ-033 through WRA-RQ-035 traceability | QA Engineer |
| M6 Completion Release Gate | Pass | PR #52 merged after required `rust` CI passed; merge commit `dd9c4bf39a5866f8a2cf903247db2ca0ded6a2b9` | GitHub Maintainer Specialist |
| M6 Completion Community Gate | Pass | Issues #43-#47 closed; milestone #6 closed with 5 closed issues and 0 open issues; repository issue list empty | Project Coordinator |
| v0.5.0 Proposal Requirements Gate | Pass | `docs/v0.5.0-criteria-dsl-milestone-proposal.md`; WRA-RQ-036 through WRA-RQ-042 | Project Coordinator |
| v0.5.0 Human Approval Gate | Pass | User approved the milestone proposal before GitHub issue creation | Project Coordinator |
| v0.5.0 Issue Planning Gate | Pass | Milestone #7 and issues #55 through #61 created | Core Software Engineer |
| M7-001 Requirements Gate | Pass | WRA-RQ-036; issue #55 | Software Architect |
| M7-001 Architecture Gate | Pass | Config-boundary schema and compatibility adapter in `crates/ferrisoxide-core/src/config.rs` | Abstraction Review Engineer |
| M7-001 Implementation Gate | Pass | DSL config structs, shape validation, legacy conversion preservation, CLI invalid-config fixture | Test Automation Engineer |
| M7-001 Testing Gate | Pass | `cargo fmt --check`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `git diff --check`, and PR #63 required `rust` CI passed | Verification and Validation Engineer |
| M7-001 Release Gate | Pass | PR #63 merged after required `rust` CI passed; merge commit `9a8b0e667f9d829a1083168b7875db967ca4e960` | GitHub Maintainer Specialist |
| M7-001 Community Gate | Pass | Issue #55 closed; issues #56-#61 remain open under milestone #7 | Project Coordinator |
| M7-002 Requirements Gate | Pass | WRA-RQ-037 and WRA-RQ-038; issue #56 | Software Architect |
| M7-002 Architecture Gate | Pass | Config-boundary operator and unit validation in `crates/ferrisoxide-core/src/config.rs` | Abstraction Review Engineer |
| M7-002 Implementation Gate | Pass | Operator enum, measurement kind output units, explicit unit validation, CLI invalid-config fixtures | Test Automation Engineer |
| M7-002 Testing Gate | Pass | `cargo fmt --check`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `git diff --check`, and PR #65 required `rust` CI passed | Verification and Validation Engineer |
| M7-002 Release Gate | Pass | PR #65 merged after required `rust` CI passed; merge commit `37cff043ff9ed16d7bb27ae2ddf315732ed20203` | GitHub Maintainer Specialist |
| M7-002 Community Gate | Pass | Issue #56 closed; issues #57-#61 remain open under milestone #7 | Project Coordinator |
| v0.6.0 Portable Rule Package Intake Gate | Pass | User described desktop-to-embedded rule deployment direction and portable rule package concept | Project Coordinator |
| v0.6.0 Portable Rule Package Requirements Gate | Pass for proposal | WRA-RQ-043 through WRA-RQ-050 in `requirements.md` | Software Architect |
| v0.6.0 Portable Rule Package Architecture Gate | Pass for proposal | `decisions/ADR-004-portable-rule-package-architecture.md` | Abstraction Review Engineer |
| v0.6.0 Portable Rule Package Scope Gate | Pass | Proposal excludes GUI, live DAQ, controller SDK/HAL, RTOS production integration, certification claims, and hardware qualification | Project Orchestrator |
| v0.6.0 Portable Rule Package Issue Planning Gate | Pass | GitHub milestone #8 and issues #67 through #74 created | GitHub Maintainer Specialist |
| v0.6.0 Portable Rule Package Release Gate | Pass | PR #75 merged after required `rust` CI passed; merge commit `3dadc38f591ffe2faa3c2c62016f07e9c46ecab0` | Project Coordinator |
| v0.6.0 Portable Rule Package Community Gate | Pass | Milestone #8 open with issues #67 through #74; milestone #7 is closed | Project Orchestrator |
| M8-001 Requirements Gate | Pass | WRA-RQ-043; issue #67 acceptance criteria | Software Architect |
| M8-001 Architecture Gate | Pass | `decisions/ADR-004-portable-rule-package-architecture.md`; schema-only boundary in `crates/ferrisoxide-rule-schema/README.md` | Abstraction Review Engineer |
| M8-001 Dependency Gate | Pass | Reuses approved `serde` and `serde_json` dev-dependency only; no new third-party crates | Security Engineer |
| M8-001 Implementation Gate | Pass locally | `crates/ferrisoxide-rule-schema/src/lib.rs`, workspace manifest, architecture/docs traceability updates | Core Software Engineer |
| M8-001 Testing Gate | Pass locally | `cargo tree -p ferrisoxide-rule-schema`; `cargo test -p ferrisoxide-rule-schema`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check` | Verification and Validation Engineer |
| M8-001 Release Gate | Pass | PR #106 merged after required `rust` CI passed; issue #67 closed | GitHub Maintainer Specialist |
| M8-002 Requirements Gate | Pass | WRA-RQ-043 and WRA-RQ-044; issue #71 acceptance criteria | Software Architect |
| M8-002 Documentation Gate | Pass locally | `docs/rule-package-format.md`, `examples/rule-package/rules.toml`, `examples/rule-package/rules.json`, `README.md` | Documentation Engineer |
| M8-002 Dependency Gate | Pass | Reuses approved `toml` dev-dependency only to parse-test examples; no new third-party crates | Security Engineer |
| M8-002 Testing Gate | Pass locally | `cargo tree -p ferrisoxide-rule-schema`; `cargo test -p ferrisoxide-rule-schema`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check` | Verification and Validation Engineer |
| M8-002 Release Gate | Pass | PR #107 merged after required `rust` CI passed; issue #71 closed | GitHub Maintainer Specialist |
| M8-003 Requirements Gate | Pass | WRA-RQ-045; issue #68 acceptance criteria | Software Architect |
| M8-003 Architecture Gate | Pass | Validator remains in `ferrisoxide-rule-schema`; no CLI, DAQ, HAL, SDK, runtime controller, or report path added | Abstraction Review Engineer |
| M8-003 Dependency Gate | Pass | Reuses approved `serde_json` and `toml` as parser dependencies; no new third-party crates | Security Engineer |
| M8-003 Implementation Gate | Pass locally | `crates/ferrisoxide-rule-schema/src/lib.rs` parse helpers, `RulePackage::validate`, `validate_for_target`, checksum comparison, structured error types | Core Software Engineer |
| M8-003 Testing Gate | Pass locally | `cargo tree -p ferrisoxide-rule-schema`; `cargo test -p ferrisoxide-rule-schema`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check` | Verification and Validation Engineer |
| M8-003 Release Gate | Pass | PR #108 merged after required `rust` CI passed; issue #68 closed | GitHub Maintainer Specialist |
| M8-004 Requirements Gate | Pass | WRA-RQ-046; issue #69 acceptance criteria | Software Architect |
| M8-004 Architecture Gate | Pass | Export is isolated to `ferrisoxide-cli` and consumes `ferrisoxide-rule-schema`; no analyze/plot behavior change | Abstraction Review Engineer |
| M8-004 Dependency Gate | Pass | Adds local `ferrisoxide-rule-schema` dependency and approved `serde_json`; no new third-party crates | Security Engineer |
| M8-004 Implementation Gate | Pass locally | `export-rule-package` command writes `rules.toml`, `rules.json`, and `validation-report.json` after config, analysis, and package validation | Core Software Engineer |
| M8-004 Testing Gate | Pass locally | `docs/validation-log.md`; `cargo tree -p ferrisoxide-cli`; `cargo test -p ferrisoxide-cli`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check` | Verification and Validation Engineer |
| M8-004 Release Gate | Pass | PR #109 merged after required `rust` CI passed; issue #69 closed | Project Orchestrator |
| M8-005 Requirements Gate | Pass | WRA-RQ-047; issue #70 acceptance criteria | Software Architect |
| M8-005 Architecture Gate | Pass | Manifest/checksum model is in `ferrisoxide-rule-schema`; desktop file writing remains isolated to `ferrisoxide-cli` | Abstraction Review Engineer |
| M8-005 Dependency Gate | Pass | No new third-party crates; dependency-free non-cryptographic checksum helper documented in `docs/dependency-review.md` | Security Engineer |
| M8-005 Implementation Gate | Pass locally | `manifest.json` and `checksum.txt` are exported with deterministic metadata after package validation | Core Software Engineer |
| M8-005 Testing Gate | Pass locally | `docs/validation-log.md`; `cargo tree -p ferrisoxide-rule-schema`; `cargo tree -p ferrisoxide-cli`; `cargo test -p ferrisoxide-rule-schema`; `cargo test -p ferrisoxide-cli`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check` | Verification and Validation Engineer |
| M8-006 Requirements Gate | Pass | WRA-RQ-048; issue #73 acceptance criteria | Software Architect |
| M8-006 Architecture Gate | Pass | Shared criteria semantics live in `ferrisoxide-rule-engine`; desktop `ferrisoxide-core` adapts to it; embedded-compatible host tests call it over fixed slices | Abstraction Review Engineer |
| M8-006 Dependency Gate | Pass | Adds local `ferrisoxide-rule-engine`; no new third-party crates; `ferrisoxide-embedded` uses the engine as a host-test dev-dependency only | Security Engineer |
| M8-006 Implementation Gate | Pass locally | `crates/ferrisoxide-rule-engine`, `crates/ferrisoxide-core/src/analysis.rs`, and `crates/ferrisoxide-embedded/src/lib.rs` share one rule semantics path | Core Software Engineer |
| M8-006 Testing Gate | Pass locally | `cargo tree -p ferrisoxide-rule-engine`; `cargo tree -p ferrisoxide-embedded`; targeted crate tests; workspace tests; clippy; diff check | Verification and Validation Engineer |
| M8-006 Release Gate | Pass | PR #113 merged after required `rust` CI passed; issue #73 closed | GitHub Maintainer Specialist |
| M8-007 Requirements Gate | Pass | WRA-RQ-049; issue #72 acceptance criteria | Embedded RTOS Engineer |
| M8-007 Architecture Gate | Pass | `ferrisoxide-rule-engine` is `#![no_std]`; owned desktop evidence uses `alloc`; borrowed summary API returns borrowed/static result and error data for constrained embedded-compatible evaluation | Abstraction Review Engineer |
| M8-007 Dependency Gate | Pass | No new third-party crates; target dependency trees show no CSV, TOML, plotting, report, HAL, SDK, DAQ, or file-I/O crates in the embedded-compatible path | Security Engineer |
| M8-007 Implementation Gate | Pass locally | `crates/ferrisoxide-rule-engine/src/lib.rs` adds no_std crate boundary, borrowed criteria, borrowed errors, borrowed summary output, and no_std-safe evidence rounding | Core Software Engineer |
| M8-007 Testing Gate | Pass locally | Targeted rule-engine test and target checks pass; full workspace validation pending before PR | Verification and Validation Engineer |
| M8-007 Release Gate | Pass | PR #114 merged after required `rust` CI passed; issue #72 closed | GitHub Maintainer Specialist |
| M8-008 Requirements Gate | Pass | WRA-RQ-050; issue #74 acceptance criteria | Verification and Validation Engineer |
| M8-008 Architecture Gate | Pass | `tests/parity/` owns waveform, rule package, and expected JSON fixtures; `crates/ferrisoxide-core/tests/rule_parity.rs` compares desktop core output to embedded-compatible borrowed-rule output | Abstraction Review Engineer |
| M8-008 Dependency Gate | Pass | Adds only local `ferrisoxide-rule-schema` dev-dependency for `ferrisoxide-core` tests; no new third-party crates or runtime dependencies | Security Engineer |
| M8-008 Implementation Gate | Pass locally | Parity fixture and integration test compare pass/fail, failed criterion, measurement ID, method, channel, measured value, required value, tolerance, sample index, and timestamp exactly | Core Software Engineer |
| M8-008 Testing Gate | Pass locally | `cargo test -p ferrisoxide-core --test rule_parity` passed; full workspace validation pending before PR | Verification and Validation Engineer |
| v0.7.0 Controller-In-The-Loop Intake Gate | Pass | User described controller simulation, DAQ observation, production/test config separation, RTOS modes, and digital-twin direction | Project Coordinator |
| v0.7.0 Controller-In-The-Loop Requirements Gate | Pass for proposal | WRA-RQ-051 through WRA-RQ-060 in `requirements.md` | Software Architect |
| v0.7.0 Controller-In-The-Loop Architecture Gate | Pass for proposal | `docs/controller-in-the-loop-workflow.md` | Abstraction Review Engineer |
| v0.7.0 Controller-In-The-Loop Scope Gate | Pass | Proposal excludes GUI, vendor DAQ SDKs, hardware HALs, production RTOS integration, real-time guarantees, safety certification, and hardware qualification claims | Project Orchestrator |
| v0.7.0 Controller-In-The-Loop Issue Planning Gate | Pass | GitHub milestone #9 and issues #77 through #86 created | GitHub Maintainer Specialist |
| v0.7.0 Controller-In-The-Loop Release Gate | Pass | PR #87 merged after required `rust` CI passed; merge commit `ac5733a5fb3d65d36278a0e98d0cb1c9566ac3dc` | Project Coordinator |
| v0.7.0 Controller-In-The-Loop Community Gate | Pass | Milestone #9 open with issues #77 through #86; M8 issues #67 through #74 remain open or in progress | Project Orchestrator |
| v0.7.0 Platform Profile Requirements Gate | Pass | WRA-RQ-061 in `requirements.md`; issue #89 | Software Architect |
| v0.7.0 Platform Profile Architecture Gate | Pass | `docs/platform-targets.md`; `aarch64-apple-darwin` and `aarch64-unknown-none` target names verified locally | Embedded RTOS Engineer |
| v0.7.0 Platform Profile Target Check Gate | Pass locally | `cargo check --workspace --target aarch64-apple-darwin`; `cargo check -p ferrisoxide-signal --target aarch64-unknown-none`; `cargo check -p ferrisoxide-embedded --target aarch64-unknown-none` | Verification and Validation Engineer |
| v0.7.0 Platform Profile Release Gate | Pass | PR #90 merged after required `rust` CI passed; merge commit `d55969ba4c7ca7115dd87f5b379afefbded1fc8a` | GitHub Maintainer Specialist |
| v0.7.0 Platform Profile Community Gate | Pass | Issue #89 closed; milestone #9 remains open for issues #77 through #86 | Project Coordinator |
| BRAND-002 Requirements Gate | Pass | `requirements.md` WRA-RQ-064; issue #98 | Product Architect |
| BRAND-002 Architecture Gate | Pass | `docs/brand-architecture.md`; ADR-006 | GitHub Maintainer Specialist |
| BRAND-002 Implementation Gate | Pass locally | `Cargo.toml`, `Cargo.lock`, `crates/ferrisoxide-*`, README, docs, scripts, fixtures | Core Software Engineer |
| BRAND-002 Testing Gate | Pass locally | `docs/validation-log.md`; metadata, fmt, tests, QEMU-demo test, clippy, CLI smokes, benchmark smoke, diff check, identifier scan | Verification and Validation Engineer |
| BRAND-002 Release Gate | Pass | PR #99 merged after required `rust` CI passed; issue #98 closed; repository host was renamed to `kota-wilson/ferrisoxide-signal` and is now amended by REPO-001 | GitHub Maintainer Specialist |
| REPO-001 Requirements Gate | Pass | WRA-RQ-065; issue #111 | Product Architect |
| REPO-001 Architecture Gate | Pass | ADR-007 separates repository host identity from `ferrisoxide-signal` crate and CLI identity | Software Architect |
| REPO-001 Implementation Gate | Pass locally | `Cargo.toml`, README, ADRs, brand docs, project memory, requirements, traceability, risk, and environment docs updated | Core Software Engineer |
| REPO-001 Testing Gate | Pass locally | `gh repo view`; `git remote -v`; Cargo metadata; docs scan; fmt; workspace tests; clippy; diff check | Verification and Validation Engineer |
| REPO-001 Release Gate | Pending PR | Open protected PR with `Fixes #111`, wait for required `rust` CI, merge, and close issue #111 | GitHub Maintainer Specialist |

## Update Rules

Update this file whenever objective, stage, risk, decision, environment status, traceability status, or next owner changes.
