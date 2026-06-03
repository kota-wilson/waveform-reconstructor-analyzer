# Project State

Last updated: 2026-06-03

## Current Objective

Record the completed and merged M43-M53 native egui workflow shell, Source/Config/Run/Plot-page UX refinements, scalable Plot-page rendering, and WRA-RQ-139 Run-page output-directory picker after PR #190 reached `main`.

M10 through M14 are complete through merged PR evidence. M15-M20 are complete through config reference, artifact contract, local batch workflow, transform-package compatibility, validation corpus index, MVP-exit pipeline report, readiness report, and post-MVP roadmap artifacts. M21-M24 are complete through portable linear pointwise package export, shared borrowed-slice runtime-compatible semantics, positive/negative package fixtures, and a runtime-loader design gate.

M25 is complete through the source-of-truth transform catalog, CLI catalog output, metadata/catalog tests, package-support checks, and catalog docs. M26 is complete through desktop data-cleaning and timing-conditioning filters, config/CLI fixtures, catalog metadata tests, and package-export rejection guardrails. M27 is complete through desktop pointwise, normalization, and nonlinear conditioning filters, config/CLI fixtures, catalog metadata tests, formula/domain tests, and package-export rejection guardrails. M28 is complete through desktop smoothing, detrending, baseline, Hampel, and spike-cleanup filters, config/CLI fixtures, catalog metadata tests, edge/drift/spike tests, and package-export rejection guardrails. M29 is complete through desktop standard frequency filters, config/CLI fixtures, generated frequency-response tests, sample-rate/stability tests, catalog metadata tests, and package-export rejection guardrails. M30 is complete through desktop resampling and timing-alignment filters, config/CLI fixtures, anti-alias/timing tests, catalog metadata tests, alignment confidence metadata, and package-export rejection guardrails. M31 is complete through desktop envelope/energy/calculus filters, feature records, config/CLI fixtures, unit/known-answer tests, catalog metadata tests, report schema updates, and package-export rejection guardrails. M32 is complete through desktop statistics/correlation filters, feature records, histogram method context, lag convention docs, config/CLI fixtures, known-answer tests, catalog metadata tests, report schema updates, and package-export rejection guardrails. M33 is complete through desktop spectrum, window, and time-frequency feature records, dependency-free spectral routines, config/CLI fixtures, known-answer sine/square/harmonic/noise tests, report schema updates, and package/runtime guardrails. M34 is complete through desktop deterministic fault-injection and ADC/DAC simulation filters, dependency-free RNG/noise logic, config/CLI fixtures, simulation-only metadata, catalog metadata tests, and package/runtime guardrails. M35 is complete through desktop multi-channel arithmetic, vector/matrix/coordinate transforms, software sensor conversions, vibration/control conditioning, config/CLI fixtures, catalog metadata tests, and package/runtime guardrails.

M36 is complete through catalog, UX, compatibility, validation-corpus, benchmark-readiness, release-readiness, community, retrospective, stale-reference closure artifacts, and PR #175 mainline merge evidence. M37-M42 are complete and merged through PR #177 as the desktop workflow path with source intake/inspect, channel labeling/config scaffolding, transform/criteria authoring templates, evaluation bundles, and workflow validation corpus coverage. M43-M53 plus WRA-RQ-139 are complete and merged through PR #190 as the optional native egui workflow shell with shared workflow APIs, GUI source/config/run/results/plot panels, Source-page CSV file selection/header loading/unit selectors, channel-based Config builder, Run-page output directory picker, Plot-page channel selectors, render-only scalable Plot-page handling, dependency review, macOS GUI CI, and closed GitHub milestone #15 / closed issues #179-#189. No release tag, crate publication, installer, live DAQ, HAL/RTOS, hardware, runtime-loader, certification work, or public announcement was added by this update.

## Current Stage

The M10-M53 sequence is complete through the approved runtime-path follow-up, the full comprehensive conditioning suite, PR #175 mainline merge, PR #177 desktop workflow merge, and PR #190 native egui workflow shell plus Source/Config/Run/Plot-page UX and scalable rendering merge. `docs/next-milestones-roadmap.md` defines the historical sequence and now records M37-M42 as the merged desktop workflow path plus M43-M53 as the merged GUI shell path. `docs/v0.8.0-transform-architecture-milestone-proposal.md` covers WRA-RQ-070 through WRA-RQ-074, `docs/v0.9.0-pointwise-windowed-transform-mvp-milestone-proposal.md` covers WRA-RQ-075 through WRA-RQ-080, `docs/v0.10.0-event-validation-transform-milestone-proposal.md` covers WRA-RQ-081 through WRA-RQ-086, `docs/v0.11.0-transform-runtime-profile-validation-milestone-proposal.md` covers WRA-RQ-087 through WRA-RQ-092, `docs/v0.12.0-high-pass-baseline-correction-milestone-proposal.md` covers WRA-RQ-093 through WRA-RQ-098, M15-M20 docs cover WRA-RQ-099 through WRA-RQ-105, M21-M24 docs cover WRA-RQ-106 through WRA-RQ-109, M25 docs cover WRA-RQ-110, `docs/comprehensive-filter-signal-conditioning-roadmap.md` covers WRA-RQ-111 through WRA-RQ-121, `docs/desktop-user-workflow-roadmap.md` covers WRA-RQ-122 through WRA-RQ-127, and `docs/egui-workflow-shell-roadmap.md` covers WRA-RQ-128 through WRA-RQ-139. GitHub milestone #10 is closed with M10-001 through M10-006 as closed issues #132 through #137 and merged PR #138. GitHub milestone #11 is closed with M11-001 through M11-007 as closed issues #140 through #146 and merged PR #147. GitHub milestone #12 is closed with M12-001 through M12-007 as closed issues #149 through #155 and merged PR #156. GitHub milestone #13 is closed with M13-001 through M13-006 as closed issues #158 through #163 closed by PR #164; PR #165 records release/community closure artifacts and PR #166 records explicit M13 V&V/QA gates. GitHub milestone #14 is closed with M14-001 through M14-006 as closed issues #167 through #172 closed by PR #173. M15-M36 are complete and merged through PR #175 without GitHub issue creation. M37-M42 are complete and merged through PR #177 without GitHub issue creation. GitHub milestone #15 is closed with M43-M53 issues #179 through #189 closed by PR #190. Release publication, runtime-loader implementation, live/realtime DAQ, full GUI packaging, hardware/runtime work, or certification scope remain separately gated.

Historical closure context:

Milestone #7, `v0.5.0: Measurement-Backed Criteria DSL`, is closed after PRs #101 through #105 completed issues #57 through #61 and the required `rust` CI passed. M8-001 / issue #67 is complete through PR #106: `crates/ferrisoxide-rule-schema` owns the first versioned portable FerrisOxide Rule Package schema types for package metadata, target profiles, sample timing, channels, units, thresholds, filters, measurement-backed criteria, and unit-bearing requirements. M8-002 / issue #71 is complete through PR #107: `docs/rule-package-format.md` defines `rules.toml`, `rules.json`, future `rules.bin`, `manifest.json`, `checksum.txt`, `validation-report.json`, and `qualification-evidence.svg` roles, with parse-tested examples in `examples/rule-package/`. M8-003 / issue #68 is complete through PR #108: `ferrisoxide-rule-schema` validates packages before export or execution with structured error kind, field, and message values. M8-004 / issue #69 is complete through PR #109: the CLI exports `rules.toml`, `rules.json`, and `validation-report.json` from validated config and analysis evidence, validates the package before writing, compares expected artifacts in tests, and refuses to overwrite existing artifacts. M8-005 / issue #70 is complete through PR #110: the schema crate defines deterministic manifest/checksum metadata, the CLI export also writes `manifest.json` and `checksum.txt`, and tests cover deterministic checksum output, checksum mismatch errors, manifest metadata, and exact exported artifacts. REPO-001 / issue #111 is complete through PR #112: the main repository host, local workspace path, and current repository metadata are corrected to `kota-wilson/ferrisoxide`, while `ferrisoxide-signal` remains the signal-analysis crate and CLI binary. M8-006 / issue #73 is complete through PR #113: `crates/ferrisoxide-rule-engine` owns shared criteria semantics over caller-provided slices, `ferrisoxide-core` delegates desktop analysis to it, and `ferrisoxide-embedded` has host tests that evaluate embedded-compatible slices through the same engine. M8-007 / issue #72 is complete through PR #114: `ferrisoxide-rule-engine` is `#![no_std]`, desktop-style owned evidence uses `alloc`, and the borrowed summary API returns borrowed/static result and error data for basic no-heap embedded-compatible evaluation where practical. M8-008 / issue #74 is complete through PR #115: `tests/parity/` contains a waveform fixture, rule package, and expected JSON report, and `crates/ferrisoxide-core/tests/rule_parity.rs` verifies exact portable evidence parity between the desktop core path and embedded-compatible borrowed-rule path. Milestone #8, `v0.6.0: Portable Rule Package System`, is closed with 8 closed issues and 0 open issues. TEST-001 / issue #117 is complete through PR #118: `tests/e2e/heated_actuator/` includes passing and failing CSVs, exact expected JSON reports, production-control and test-verification config examples, plus CLI smokes for analysis, SVG evidence, and response-latency package export. DOCS-001 / issue #119 is complete through PR #120 and expands the main README into a complete human-readable product and repository guide. M9-001 / issue #77 is complete through PR #121: `crates/ferrisoxide-control-schema` adds a data-only production control config schema crate plus docs and parse-tested example config. M9-002 / issue #80 is complete through PR #122: `crates/ferrisoxide-verification-schema` adds a data-only test verification config schema crate plus docs and parse-tested example config. M9-003 / issue #78 is complete through PR #123: `crates/ferrisoxide-simulator` adds a deterministic virtual controller simulation engine over production control configs and abstract sample frames. M9-004 / issue #79 is complete through PR #124: `crates/ferrisoxide-daq` adds fixture/test-double DAQ input abstraction. M9-005 / issue #81 is complete through PR #125: `crates/ferrisoxide-controller-io` adds host-checkable controller I/O abstraction. M9-006 / issue #82 is complete through PR #126: the CLI adds the fixture-driven desktop simulation workflow. M9-007 / issue #83 is complete through PR #127: `ferrisoxide-deployment` adds the RTOS/controller deployment package format boundary. M9-008 / issue #84 is complete through PR #128: deployment manifest mode profiles separate production control, test verification, and signal validation. M9-009 / issue #85 is complete through PR #129: software-only controller config parity coverage compares desktop workflow and embedded-compatible borrowed-rule evidence. M9-010 / issue #86 is complete through PR #130: `ferrisoxide-deployment` adds the qualification evidence report format. Milestone #9, `v0.7.0: Controller Simulation and Deployment Config System`, is closed with 12 closed issues and 0 open issues. PR #90 closed issue #89 and defined Apple Silicon desktop authoring plus Raspberry Pi 5 bare-metal ARM64 embedded runtime platform profiles. PR #93 closed issue #92 and defined Raspberry Pi Pico 2 as a future optional microcontroller runtime profile for compact deterministic rule execution. PR #96 closed issue #95 and documented FerrisOxide as a proposed umbrella brand. PR #99 closed issue #98 and completed BRAND-002 adoption of FerrisOxide Signal across in-repository package names, CLI binary, docs, examples, and scripts. GUI, live DAQ vendor SDKs, embedded plotting, hardware HALs, Pico 2 runtime implementation, unsafe FFI, RTOS SDK integration, plugin runtime, hosted services, database-backed workflows, schedulers, certification claims, binary package serialization, cryptographic signing, and external brand expansion remain out of scope until separately gated. Local file-based batch analysis is now implemented for M17.

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
- Risk: Software-only controller-style scenarios may be mistaken for live DAQ, control-loop, or hardware qualification coverage.
  Owner: Verification and Validation Engineer / Documentation Engineer
- Risk: A highly verbose README may drift from implementation or imply stronger maturity than the code supports.
  Owner: Documentation Engineer / Project Coordinator
- Risk: Production control config schema may be mistaken for implemented controller simulation or runtime execution.
  Owner: Software Architect / Documentation Engineer
- Risk: Test verification config schema may be mistaken for an executable criteria engine, live DAQ workflow, or certified qualification format.
  Owner: Software Architect / Verification and Validation Engineer / Documentation Engineer
- Risk: Virtual controller simulation may be overread as firmware parity, real-time behavior, or hardware qualification evidence.
  Owner: Core Software Engineer / Embedded RTOS Engineer / Verification and Validation Engineer
- Risk: DAQ abstraction may be mistaken for live hardware support or may invite premature vendor SDK/global setup work.
  Owner: Systems Engineer / Security Engineer / DX Engineer
- Risk: Controller I/O abstraction may be mistaken for HAL, RTOS SDK, or hardware timing support.
  Owner: Embedded RTOS Engineer / Security Engineer / Core Software Engineer
- Risk: Desktop simulation workflow may be mistaken for live DAQ, firmware parity, hardware-in-the-loop, or certification evidence.
  Owner: Core Software Engineer / Verification and Validation Engineer / Documentation Engineer
- Risk: The analog transform taxonomy may be mistaken for implemented transform support instead of future planning input.
  Owner: Product Architect / Documentation Engineer
- Risk: Structured transform metadata may break existing report/config consumers if implemented incompatibly.
  Owner: Core Software Engineer / Documentation Engineer
- Risk: Pointwise and baseline transforms may be mistaken for calibrated sensor accuracy or may hide real signal failures.
  Owner: Electrical Signal Integrity Engineer / Systems Engineer / Documentation Engineer
- Risk: Event, debounce, and glitch-removal transforms may hide failures or drift between desktop and embedded-compatible behavior.
  Owner: Embedded RTOS Engineer / Core Software Engineer / Verification and Validation Engineer
- Risk: Runtime-profile validation may be mistaken for embedded runtime, hardware, or certification support.
  Owner: Software Architect / Embedded RTOS Engineer / Documentation Engineer
- Risk: High-pass baseline correction may hide real low-frequency failures or be mistaken for calibrated drift removal.
  Owner: Systems Engineer / V&V Engineer / Documentation Engineer
- Risk: MVP-exit and comprehensive-suite implementation may be mistaken for approval to create M15-M36 GitHub issues, publish releases, or expand runtime/hardware scope.
  Owner: Project Coordinator / GitHub Maintainer Specialist
- Risk: Config, report, and artifact contracts may drift after MVP exit.
  Owner: Documentation Engineer / V&V Engineer
- Risk: Batch workflow expansion may drift into live DAQ, GUI, hosted services, databases, schedulers, or hardware workflows too early.
  Owner: Core Software Engineer / Project Coordinator
- Risk: Portable `offset` and `gain` package support may be mistaken for calibrated sensor accuracy or span correction.
  Owner: Electrical Signal Integrity Engineer / Documentation Engineer
- Risk: Runtime-loader design documentation may be mistaken for an implemented embedded package loader.
  Owner: Embedded RTOS Engineer / Project Coordinator
- Risk: The comprehensive filter/signal-conditioning roadmap may be mistaken for immediate implementation approval or unbounded DSP scope.
  Owner: Product Architect / Project Coordinator
- Risk: Advanced filter, resampling, and spectral milestones may introduce dependency, numerical correctness, or performance risk.
  Owner: Security Engineer / Performance Engineer / Signal Processing Engineer
- Risk: Fault injection and simulated signal conditioning may be mistaken for observed hardware evidence.
  Owner: Test Automation Engineer / Documentation Engineer
- Risk: Resampling and timing alignment can hide timing defects or be mistaken for DAQ clock calibration.
  Owner: Performance Engineer / V&V Engineer / Documentation Engineer
- Risk: The native GUI shell may be mistaken for a packaged GUI product, live DAQ, realtime hardware acquisition, runtime loader, release artifact, or certification implementation.
  Owner: Product Architect / Project Coordinator / Documentation Engineer

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
- Decision: Use the analog signal transform taxonomy as planning input for future transform architecture milestones rather than adding algorithms ad hoc.
  Owner: Product Architect / Software Architect
  Status: Accepted for local milestone planning in `docs/analog-transform-taxonomy.md`, `docs/next-milestones-roadmap.md`, and the M10-M13 proposals; M10 GitHub issues #132 through #137 are closed through PR #138.
- Decision: Sequence transform work as M10 architecture first, M11 pointwise/windowed transforms second, M12 event/validation transforms third, and M13 runtime-profile validation fourth.
  Owner: Project Coordinator / Software Architect
  Status: Accepted; M13 is complete through PR #164, and milestone #13 is closed.
- Decision: Sequence pre-MVP-exit work as M15 config reference, M16 report/artifact contracts, M17 local desktop batch workflow, M18 rule-package transform semantics, M19 validation corpus/benchmark expansion, and M20 MVP exit readiness review.
  Owner: Project Coordinator / Product Architect
  Status: Implemented through `docs/m15-m20-mvp-exit-pipeline-report.md` and `docs/mvp-exit-readiness-report.md`, then merged in PR #175; no GitHub issue creation or release publication was performed.
- Decision: Use the runtime path as the first post-MVP follow-up, starting with package semantics for linear pointwise transforms only.
  Owner: Software Architect / Embedded RTOS Engineer
  Status: Implemented through M21-M24 and merged in PR #175; `offset`, `gain`, and `invert` export and borrowed-slice semantics are supported, while runtime loader implementation remains gated by `docs/runtime-loader-design-gate.md`.
- Decision: Plan the next post-MVP implementation path as a comprehensive sampled-waveform conditioning suite, but require M25 registry/completeness work before adding more algorithms.
  Owner: Product Architect / Software Architect
  Status: M25 implemented in `crates/ferrisoxide-core/src/transform_catalog.rs` and `docs/transform-catalog.md`; M26 implemented for desktop data-cleaning/timing-conditioning filters; M27 implemented for desktop pointwise/nonlinear conditioning filters; M28 implemented for desktop smoothing/baseline conditioning filters; M29 implemented for desktop standard frequency filters; M30 implemented for desktop resampling/timing-alignment filters; M31 implemented for desktop envelope/energy/calculus filters and feature records; M32 implemented for desktop statistics/correlation filters and feature records; M33 implemented for desktop spectrum/window/time-frequency feature records; M34 implemented for desktop deterministic fault-injection and ADC/DAC simulation filters; M35 implemented for desktop multi-channel/sensor/domain conditioning filters; M36 implemented for catalog, UX, compatibility, validation-corpus, benchmark-readiness, release-readiness, community, and retrospective closure; M25-M36 merged in PR #175.
- Decision: Use M37-M42 as the next desktop user workflow path, covering source inspection, channel/config scaffolding, authoring templates, evaluation bundles, and workflow validation docs.
  Owner: Product Architect / Project Coordinator
  Status: Implemented, validated, and merged through PR #177 in `crates/ferrisoxide-cli/src/main.rs`, `docs/desktop-user-workflow.md`, `docs/desktop-user-workflow-roadmap.md`, README, and M42 examples; M37 records the workflow contract, M38 implements source intake/inspect, M39 implements channel labeling/config scaffolding, M40 implements transform/criteria authoring templates, M41 implements evaluation bundles, and M42 adds workflow validation corpus coverage. Live/realtime DAQ, GUI, runtime loader, hardware, dependencies, release publication, and certification work remain separately gated.
- Decision: Use M43-M53 as the native egui workflow shell path over shared desktop workflow APIs.
  Owner: Product Architect / Core Software Engineer
  Status: Implemented, validated, and merged through PR #190 in `ferrisoxide-workflow`, `ferrisoxide-gui`, `docs/egui-workflow-shell-roadmap.md`, and `docs/m43-m48-egui-workflow-shell-pipeline-report.md`; the Source page now includes local CSV file selection, explicit header loading, and unit selectors; the Config page includes Source-derived channel sections, dropdown-driven filter/action and criterion rows, numeric-only value fields, generated/opened TOML preview, and native open/save behavior; the Run page includes native output-directory selection; and the Plot page includes derived-channel plot selectors plus render-only scalable plotting, while GUI packaging, live/realtime DAQ, vendor SDKs, hardware acquisition, runtime-loader execution, release publication, and certification evidence remain separately gated.

## Next Responsible Role

Role: Project Coordinator / Product Architect

Expected deliverable: Select any future GUI, live DAQ, runtime, release, packaging, or hardware follow-up only after an explicit gate.

## Orchestration Status

- Execution tier: Tier 2 MVP.
- Selected workflow: Project orchestration plus open-source library and data-analysis workflows.
- Repository URL: `https://github.com/kota-wilson/ferrisoxide`.
- Current milestone: M43-M53 native egui workflow shell plus WRA-RQ-139 Run-page output directory picker are complete and merged through PR #190 after M37-M42 desktop workflow merged through PR #177 and M36 comprehensive conditioning closure merged through PR #175. M43 records the GUI dependency gate, M44 adds shared workflow APIs, M45 adds the optional native GUI shell, M46 adds source/config panels, M47 adds run/results review, M48 adds interactive CSV plotting plus macOS GUI CI, M49 adds Source-page CSV file selection/header loading, M50 adds Source-page time/channel unit selectors, M51 adds Plot-page channel selectors, M52 adds scalable render-only Plot-page handling, M53 adds the channel-based Config builder, and WRA-RQ-139 adds the Run-page output directory picker. GitHub milestone #15 is closed with M43-M53 issues #179-#189 closed by PR #190. Milestone #14, `v0.12.0: High-Pass Baseline Correction`, is closed with issues #167 through #172 closed by PR #173. Milestone #13, `v0.11.0: Transform Runtime Profile Validation`, is closed with issues #158 through #163 closed by PR #164. Milestone #12, `v0.10.0: Event And Validation Transform MVP`, is closed with issues #149 through #155 closed by PR #156. M7, M8, M9, M10, M11, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25, M26, M27, M28, M29, M30, M31, M32, M33, M34, M35, M36, M37, M38, M39, M40, M41, M42, and M43-M53 are closed; TEST-001, DOCS-001, BRAND-002, and REPO-001 are complete.
- Completed recent milestones: Dependency-reviewed MVP slice; `M3: RTOS / embedded no_std foundation`; `M4: Signal Accuracy and Validation`; `M5: Plotting and Visualization`; `v0.4.0: Measurement & Evidence Engine`.
- Next gate: Future follow-up selection requires explicit approval. Release publication, runtime-loader implementation, live DAQ SDKs, GUI packaging, HAL/RTOS adapters, cryptographic signing, hardware/certification evidence, or new scope remain separately gated.
- Stop condition: Stop before adding target toolchains, SDKs, HALs, unsafe FFI, QEMU boot image work, Pico 2 runtime crate work, additional third-party dependencies, live DAQ/embedded plotting/certification work, GUI packaging/installers, plugin runtime, hosted services, databases, schedulers, binary package serialization, cryptographic signing, live controller simulation, live/realtime DAQ integration, unit shorthand parsing, expanded annotated SVG features, or external brand expansion without a fresh issue/gate.

## Granularity Status

- Current expected zoom level: levels 1-3 for architecture, levels 3-5 for first implementation task.
- Required artifacts: project charter, requirements, risk register, traceability matrix, architecture, orchestration plan, repository MVP slice.
- Abstraction review status: Required after architecture plan.

## Environment Status

- Project root: `/Users/kota/Desktop/codexprojects/softwaredev/projects/ferrisoxide`.
- Isolation level: Level 1 Cargo workspace.
- Local environment: Rust/Cargo; no global dependencies installed.
- Dependency status: Approved crates added and pinned in `Cargo.lock`; see `docs/dependency-review.md`. M43-M53 uses the optional exact-pinned native GUI crates behind `ferrisoxide-gui --features native`, including `rfd` for local CSV selection, TOML config open/save selection, and output-directory folder selection; M52, M53, and WRA-RQ-139 add no new third-party dependency and default workspace behavior remains separate from the native GUI dependency surface.

## Traceability Status

- Requirements: `requirements.md`.
- Traceability matrix: `traceability-matrix.md`.
- Verification matrix: `traceability-matrix.md` updated with implemented evidence through WRA-RQ-139. Milestones #9, #10, #11, #12, #13, #14, and #15 are closed in GitHub; M15-M36 are complete and merged through PR #175; M37-M42 are complete and merged through PR #177; M43-M53 plus WRA-RQ-139 are complete and merged through PR #190.

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
| MVP Exit Roadmap Gate | Pass | `docs/mvp-exit-roadmap.md`; WRA-RQ-099 through WRA-RQ-105 implemented; M15-M20 pipeline complete and included in PR #175 | User / Project Coordinator |
| MVP Exit Readiness Gate | Pass | `docs/mvp-exit-readiness-report.md`; `docs/m15-m20-mvp-exit-pipeline-report.md`; `docs/post-mvp-roadmap.md`; PR #175 | GitHub Maintainer Specialist |
| M21-M24 Runtime Path Gate | Pass | `docs/m21-m24-runtime-path-pipeline-report.md`; `docs/runtime-loader-design-gate.md`; WRA-RQ-106 through WRA-RQ-109 implemented and included in PR #175 | User / Project Coordinator |
| M25 Transform Registry Gate | Pass | `crates/ferrisoxide-core/src/transform_catalog.rs`; `docs/transform-catalog.md`; WRA-RQ-110 implemented and merged in PR #175; later transform work remains catalog-gated | User / Project Coordinator |
| M26 Data Cleaning And Timing Gate | Pass | `crates/ferrisoxide-core/src/filter.rs`; `crates/ferrisoxide-core/src/config.rs`; `examples/m26-data-cleaning-*`; `docs/m26-data-cleaning-timing-pipeline-report.md`; WRA-RQ-111 implemented and merged in PR #175 with `split_by_event` future-gated as a multi-artifact segmentation workflow | User / Project Coordinator |
| M27 Pointwise And Nonlinear Gate | Pass | `crates/ferrisoxide-core/src/filter.rs`; `crates/ferrisoxide-core/src/config.rs`; `examples/m27-pointwise-*`; `docs/m27-pointwise-normalization-nonlinear-pipeline-report.md`; WRA-RQ-112 implemented and merged in PR #175 with package/runtime exposure still gated | User / Project Coordinator |
| M28 Smoothing And Baseline Gate | Pass | `crates/ferrisoxide-core/src/filter.rs`; `crates/ferrisoxide-core/src/config.rs`; `examples/m28-smoothing-*`; `docs/m28-smoothing-baseline-pipeline-report.md`; WRA-RQ-113 implemented and merged in PR #175 with package/runtime exposure still gated | User / Project Coordinator |
| M29 Standard Frequency Filter Gate | Pass | `crates/ferrisoxide-core/src/filter.rs`; `crates/ferrisoxide-core/src/config.rs`; `examples/m29-frequency-*`; `docs/m29-standard-frequency-filter-pipeline-report.md`; WRA-RQ-114 implemented and merged in PR #175 with exact elliptic/Cauer design dependency-gated | User / Project Coordinator |
| M30 Resampling And Timing Gate | Pass | `crates/ferrisoxide-core/src/filter.rs`; `crates/ferrisoxide-core/src/config.rs`; `examples/m30-resampling-*`; `docs/m30-resampling-timing-pipeline-report.md`; WRA-RQ-115 implemented and merged in PR #175 with efficient polyphase resampling dependency/performance-gated | User / Project Coordinator |
| M31 Envelope Energy Calculus Gate | Pass | `crates/ferrisoxide-core/src/filter.rs`; `crates/ferrisoxide-core/src/feature.rs`; `crates/ferrisoxide-core/src/report.rs`; `examples/m31-calculus-*`; `docs/m31-envelope-energy-calculus-pipeline-report.md`; WRA-RQ-116 implemented and merged in PR #175 with Hilbert envelope dependency/design-gated | User / Project Coordinator |
| M32 Statistics And Correlation Gate | Pass | `crates/ferrisoxide-core/src/filter.rs`; `crates/ferrisoxide-core/src/feature.rs`; `examples/m32-statistics-*`; `docs/m32-statistics-correlation-pipeline-report.md`; WRA-RQ-117 implemented and merged in PR #175 with feature records separated from pass/fail validation | User / Project Coordinator |
| M33 Spectrum Time-Frequency Gate | Pass | `crates/ferrisoxide-core/src/feature.rs`; `crates/ferrisoxide-core/src/transform_catalog.rs`; `examples/m33-spectrum-*`; `docs/m33-spectrum-time-frequency-pipeline-report.md`; WRA-RQ-118 implemented and merged in PR #175 with optimized FFT dependency/performance work still gated | User / Project Coordinator |
| M34 Fault Injection ADC/DAC Gate | Pass | `crates/ferrisoxide-core/src/filter.rs`; `crates/ferrisoxide-core/src/config.rs`; `examples/m34-fault-adc-*`; `docs/m34-fault-injection-adc-dac-pipeline-report.md`; WRA-RQ-119 implemented and merged in PR #175 with simulation-only evidence scope | User / Project Coordinator |
| M35 Multi-Channel Sensor Domain Gate | Pass | `crates/ferrisoxide-core/src/filter.rs`; `crates/ferrisoxide-core/src/config.rs`; `examples/m35-domain-*`; `docs/m35-multi-channel-sensor-domain-pipeline-report.md`; WRA-RQ-120 implemented and merged in PR #175 with advanced phase/gain/acoustic/calibration packs dependency/design-gated | User / Project Coordinator |
| M36 Comprehensive Suite Closure Gate | Pass | `docs/m36-comprehensive-suite-closure-pipeline-report.md`; `docs/validation-log.md`; WRA-RQ-121 implemented and merged in PR #175 with catalog, UX, compatibility, validation-corpus, benchmark-readiness, release-readiness, community, retrospective, and stale-reference closure | User / Project Coordinator |
| M37 Desktop User Workflow Roadmap Gate | Pass | `docs/desktop-user-workflow-roadmap.md`; README; requirements WRA-RQ-122 through WRA-RQ-127; traceability, risk, orchestration, project-state updates, and PR #177 merge evidence define the workflow path while keeping live/realtime DAQ, GUI, runtime, hardware, dependency, release, and certification work gated | Product Architect / Project Coordinator |
| M38-M42 Desktop Workflow Implementation Gate | Pass | `crates/ferrisoxide-cli/src/main.rs`; `docs/desktop-user-workflow.md`; README; `examples/m42-desktop-workflow-waveform.csv`; `examples/m42-desktop-workflow-config.toml`; focused desktop workflow CLI tests, M42 fixture smokes, workspace tests, clippy, link scan, whitespace scan, diff check, PR #177, and required `rust` CI cover source inspection, realtime rejection, config scaffolding, authoring templates, CSV bundles, and simulation bundles | V&V Engineer / Documentation Engineer |
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
| M8-007 Testing Gate | Pass | Targeted rule-engine tests, target checks, dependency trees, workspace tests, clippy, diff check, and required `rust` CI passed | Verification and Validation Engineer |
| M8-007 Release Gate | Pass | PR #114 merged after required `rust` CI passed; issue #72 closed | GitHub Maintainer Specialist |
| M8-008 Requirements Gate | Pass | WRA-RQ-050; issue #74 acceptance criteria | Verification and Validation Engineer |
| M8-008 Architecture Gate | Pass | `tests/parity/` owns waveform, rule package, and expected JSON fixtures; `crates/ferrisoxide-core/tests/rule_parity.rs` compares desktop core output to embedded-compatible borrowed-rule output | Abstraction Review Engineer |
| M8-008 Dependency Gate | Pass | Adds only local `ferrisoxide-rule-schema` dev-dependency for `ferrisoxide-core` tests; no new third-party crates or runtime dependencies | Security Engineer |
| M8-008 Implementation Gate | Pass locally | Parity fixture and integration test compare pass/fail, failed criterion, measurement ID, method, channel, measured value, required value, tolerance, sample index, and timestamp exactly | Core Software Engineer |
| M8-008 Testing Gate | Pass | Targeted parity test, workspace tests, clippy, diff check, and required `rust` CI passed | Verification and Validation Engineer |
| M8-008 Release Gate | Pass | PR #115 merged after required `rust` CI passed; issue #74 closed | GitHub Maintainer Specialist |
| v0.6.0 Portable Rule Package Milestone Gate | Pass | Milestone #8 closed with 8 closed issues and 0 open issues | Project Coordinator |
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
| v0.7.0 Platform Profile Community Gate | Pass | Issue #89 closed; milestone #9 later closed with 12 closed issues and 0 open issues | Project Coordinator |
| BRAND-002 Requirements Gate | Pass | `requirements.md` WRA-RQ-064; issue #98 | Product Architect |
| BRAND-002 Architecture Gate | Pass | `docs/brand-architecture.md`; ADR-006 | GitHub Maintainer Specialist |
| BRAND-002 Implementation Gate | Pass locally | `Cargo.toml`, `Cargo.lock`, `crates/ferrisoxide-*`, README, docs, scripts, fixtures | Core Software Engineer |
| BRAND-002 Testing Gate | Pass locally | `docs/validation-log.md`; metadata, fmt, tests, QEMU-demo test, clippy, CLI smokes, benchmark smoke, diff check, identifier scan | Verification and Validation Engineer |
| BRAND-002 Release Gate | Pass | PR #99 merged after required `rust` CI passed; issue #98 closed; repository host was renamed to `kota-wilson/ferrisoxide-signal` and is now amended by REPO-001 | GitHub Maintainer Specialist |
| REPO-001 Requirements Gate | Pass | WRA-RQ-065; issue #111 | Product Architect |
| REPO-001 Architecture Gate | Pass | ADR-007 separates repository host identity from `ferrisoxide-signal` crate and CLI identity | Software Architect |
| REPO-001 Implementation Gate | Pass locally | `Cargo.toml`, README, ADRs, brand docs, project memory, requirements, traceability, risk, and environment docs updated | Core Software Engineer |
| REPO-001 Testing Gate | Pass locally | `gh repo view`; `git remote -v`; Cargo metadata; docs scan; fmt; workspace tests; clippy; diff check | Verification and Validation Engineer |
| REPO-001 Release Gate | Pass | PR #112 merged after required `rust` CI passed; issue #111 closed | GitHub Maintainer Specialist |
| Next Milestones Intake Gate | Pass | User supplied transform taxonomy and requested next milestones | Project Coordinator |
| Next Milestones Roadmap Gate | Pass locally | `docs/next-milestones-roadmap.md` sequences M10 through M13 | Project Orchestrator |
| M10 Transform Architecture Requirements Gate | Pass for proposal | `requirements.md` WRA-RQ-070 through WRA-RQ-074; `docs/v0.8.0-transform-architecture-milestone-proposal.md` | Software Architect |
| M10 Transform Architecture Scope Gate | Pass locally | Proposal excludes new algorithms, dependencies, live DAQ, HAL/RTOS, hardware, and certification claims | Project Orchestrator |
| M10-001 Implementation Gate | Pass locally | `docs/transform-capability-model.md`, README/doc links, requirements, traceability, project state, and `docs/m10-001-transform-capability-matrix-pipeline-report.md` | Test Automation Engineer |
| M10-002 Implementation Gate | Pass locally | `docs/structured-transform-metadata.md`, report-schema note, README/doc links, requirements, traceability, project state, and `docs/m10-002-structured-transform-metadata-pipeline-report.md` | Test Automation Engineer |
| M10-003 Implementation Gate | Pass locally | `docs/current-transform-metadata-mapping.md`, filter-behavior link, README/doc links, requirements, traceability, project state, and `docs/m10-003-current-transform-metadata-mapping-pipeline-report.md` | Test Automation Engineer |
| M10-004 Implementation Gate | Pass locally | `docs/transform-runtime-profile-compatibility.md`, README/doc links, requirements, traceability, project state, and `docs/m10-004-runtime-profile-compatibility-pipeline-report.md` | Test Automation Engineer |
| M10-005 Implementation Gate | Pass locally | README, architecture, taxonomy, filter behavior, ADC quantization, requirements, traceability, project state, and `docs/m10-005-transform-docs-wording-pipeline-report.md` | Test Automation Engineer |
| M10-006 Implementation Gate | Pass locally | `crates/ferrisoxide-core` metadata/filter/report/config updates, rule-package golden artifacts, docs/report-schema updates, requirements, traceability, project state, and `docs/m10-006-transform-metadata-tests-pipeline-report.md` | Release Engineer |
| M10 PR Gate | Pass | PR #138 merged after required `rust` CI passed; squash commit `69b8b1a4a7c963316a74130655667ea3ff1481d5`; issues #132 through #137 closed | GitHub Maintainer Specialist |
| M10 Community Gate | Pass | Milestone #10 closed with 7 closed items and 0 open items | Project Coordinator |
| M11 Pointwise/Windowed Transform Requirements Gate | Pass for proposal | `requirements.md` WRA-RQ-075 through WRA-RQ-080; `docs/v0.9.0-pointwise-windowed-transform-mvp-milestone-proposal.md` | Software Architect |
| M11 Pointwise/Windowed Transform Architecture Gate | Pass locally | M10 dependency is satisfied by merged PR #138; M11 uses existing `[[filters]]` compatibility and structured metadata | Abstraction Review Engineer |
| M11 Pointwise/Windowed Transform Issue Planning Gate | Pass | GitHub milestone #11 and issues #140 through #146 were created, then closed by PR #147 | GitHub Maintainer Specialist |
| M11 Pointwise/Windowed Transform Implementation Gate | Pass | Pointwise transforms, baseline transforms, moving median, docs, example config, and focused tests merged in PR #147 | Core Software Engineer |
| M11 Pointwise/Windowed Transform Release Gate | Pass | PR #147 merged after required `rust` CI passed; squash commit `793a2ab1323526b2695fa7b59a1246f2e29d9c43` | GitHub Maintainer Specialist |
| M11 Pointwise/Windowed Transform Community Gate | Pass | Issues #140 through #146 closed; milestone #11 closed with 8 closed items and 0 open items | Project Coordinator |
| M12 Event/Validation Transform Requirements Gate | Pass | `requirements.md` WRA-RQ-081 through WRA-RQ-086; issues #149 through #155 | Software Architect |
| M12 Event/Validation Transform Architecture Gate | Pass locally | M10 dependency is satisfied by merged PR #138, M11 compatibility path is merged by PR #147, and M12 event/report architecture is documented in `docs/event-validation-transforms.md` | Abstraction Review Engineer |
| Next Milestones Issue Planning Gate | Pass for M12 | GitHub milestone #12 and issues #149 through #155 were created after approval | GitHub Maintainer Specialist |
| Next Milestones Human Approval Gate | Pass for M12 issue creation and implementation | User approved M12 on 2026-06-01 | Project Coordinator |
| Next Milestones Implementation Gate | Pass for M12 | M12 implementation, examples, docs, and full local validation merged in PR #156 | Core Software Engineer |
| M12 Event/Validation Transform Release Gate | Pass | PR #156 merged after required `rust` CI passed; squash commit `a4885578de9d136cd8df213e1da489a7232cf702` | GitHub Maintainer Specialist |
| M12 Event/Validation Transform Community Gate | Pass | Issues #149 through #155 closed; milestone #12 closed with 8 closed items and 0 open items | Project Coordinator |
| M13 Runtime Profile Validation Requirements Gate | Pass for proposal | `requirements.md` WRA-RQ-087 through WRA-RQ-092; `docs/v0.11.0-transform-runtime-profile-validation-milestone-proposal.md` | Software Architect |
| M13 Runtime Profile Validation Architecture Gate | Pass locally | M13 uses M10 transform metadata and runtime-profile vocabulary without changing report/config schema. | Abstraction Review Engineer |
| M13 Runtime Profile Validation Human Approval Gate | Pass for planning and issue creation | User approved continuing after M12 closure on 2026-06-01 | Project Coordinator |
| M13 Runtime Profile Validation Issue Planning Gate | Pass | GitHub milestone #13 and issues #158 through #163 created. | GitHub Maintainer Specialist |
| M13 Runtime Profile Validation Implementation Gate | Pass | Runtime-profile validator, timing evidence, waveform/event metadata tests, docs, and traceability merged in PR #164. | Core Software Engineer |
| M13 Runtime Profile Validation V&V Gate | Pass | WRA-RQ-087 through WRA-RQ-092 map to runtime-profile unit tests, workspace validation, protected CI, closed issues #158 through #163, and closed milestone #13 without hardware, DAQ, RTOS timing, or certification claims. | V&V Engineer |
| M13 Runtime Profile Validation QA Gate | Pass | Formatting, diff check, local Markdown link-target scan, PR #164 protected `rust` CI, and PR #165 protected `rust` CI passed. | QA Engineer |
| M13 Runtime Profile Validation Release Gate | Pass | PR #164 merged after required `rust` CI passed; squash commit `ae0366dcd20a81a71262f38d2409dc2b85774051`. | GitHub Maintainer Specialist |
| M13 Runtime Profile Validation Community Gate | Pass | Issues #158 through #163 closed; milestone #13 closed with 6 closed items and 0 open items. | Project Coordinator |
| M14 High-Pass Baseline Correction Requirements Gate | Pass for proposal | `requirements.md` WRA-RQ-093 through WRA-RQ-098; `docs/v0.12.0-high-pass-baseline-correction-milestone-proposal.md` | Software Architect |
| M14 High-Pass Baseline Correction Architecture Gate | Pass | M14 uses existing `[[filters]]`, transform metadata, runtime-profile, and report paths without schema migration. | Abstraction Review Engineer |
| M14 High-Pass Baseline Correction Human Approval Gate | Pass for planning, issue creation, and implementation | User approved continuing after M13 closure on 2026-06-01 | Project Coordinator |
| M14 High-Pass Baseline Correction Issue Planning Gate | Pass | GitHub milestone #14 and issues #167 through #172 created. | GitHub Maintainer Specialist |
| M14 High-Pass Baseline Correction Implementation Gate | Pass | `high_pass_baseline` filter/config support, first-order recurrence, invalid input checks, metadata, CLI/config coverage, export guardrail test, docs, traceability, risk, and pipeline report merged in PR #173. | Core Software Engineer |
| M14 High-Pass Baseline Correction Testing Gate | Pass | Focused M14 tests, full workspace tests, clippy, formatting, diff check, local Markdown link scan, and PR #173 protected `rust` CI pass. | Test Automation Engineer |
| M14 High-Pass Baseline Correction V&V Gate | Pass | WRA-RQ-093 through WRA-RQ-098 map to implementation, tests, docs, guardrail evidence, PR #173, closed issues, and closed milestone without hardware, DAQ, HAL/RTOS, runtime, or certification claims. | V&V Engineer |
| M14 High-Pass Baseline Correction QA Gate | Pass | M14 pipeline report records explicit QA, security, performance, docs, code-review, evaluation, release, community, and retrospective decisions. | QA Engineer |
| M14 High-Pass Baseline Correction Evaluation Gate | Pass | `docs/m14-high-pass-baseline-correction-pipeline-report.md` maps WRA-RQ-093 through WRA-RQ-098 to PR #173, tests, docs, closed issues, and closed milestone. | Evaluation Engineer |
| M14 High-Pass Baseline Correction Release Gate | Pass | PR #173 merged after required `rust` CI passed; squash commit `a17cd4c0ae7af5ab768688c9301484e5eb4799cf`. | GitHub Maintainer Specialist |
| M14 High-Pass Baseline Correction Community Gate | Pass | Issues #167 through #172 closed; milestone #14 closed with 6 closed issues and 0 open issues. | Project Coordinator |
| M14 High-Pass Baseline Correction Retrospective Gate | Pass | M14 closure review recorded that local code review caught non-finite timestamp validation before PR; no process asset changes required. | Project Coordinator |

## Update Rules

Update this file whenever objective, stage, risk, decision, environment status, traceability status, or next owner changes.
