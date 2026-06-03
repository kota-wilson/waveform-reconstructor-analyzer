# Orchestration Plan

Project: FerrisOxide

Project folder: `/Users/kota/Desktop/codexprojects/softwaredev/projects/ferrisoxide`

Execution tier: Tier 2 MVP plus roadmap-controlled follow-on milestones

Current objective: Record the completed and merged M43-M53 native egui workflow shell, Source/Config/Run/Plot-page UX refinements, scalable Plot-page rendering, and WRA-RQ-139 Run-page output-directory picker after PR #190 reached `main`.

Current stage: M10 through M14 are complete through merged PR evidence; M15 through M36 are complete and merged through PR #175. M25 is complete as a source-of-truth transform registry and completeness contract; M26 is complete as the desktop data-cleaning/timing-conditioning implementation slice; M27 is complete as the desktop pointwise/nonlinear conditioning implementation slice; M28 is complete as the desktop smoothing/baseline conditioning implementation slice; M29 is complete as the desktop standard frequency-filter implementation slice; M30 is complete as the desktop resampling/timing-alignment implementation slice; M31 is complete as the desktop envelope/energy/calculus implementation slice; M32 is complete as the desktop statistics/correlation implementation slice; M33 is complete as the desktop spectrum/window/time-frequency implementation slice; M34 is complete as the desktop deterministic fault-injection/ADC-DAC simulation implementation slice; M35 is complete as the desktop multi-channel/sensor/domain conditioning implementation slice; M36 is complete as the catalog, UX, compatibility, validation-corpus, benchmark, release-readiness, community, retrospective, and PR #175 merge-evidence closure slice. M37-M42 are complete and merged through PR #177 as the desktop workflow path. M43-M53 plus WRA-RQ-139 are complete and merged through PR #190 as the optional native egui workflow shell: dependency gate, shared workflow APIs, app shell, source/config panels, Source-page file selector/header loading/unit selectors, channel-based Config builder, Run-page output directory picker, run/results review, plot review with channel selectors, scalable Plot-page rendering, macOS GUI CI, closed issues #179-#189, and closed GitHub milestone #15. This stage adds no release publication, live DAQ, HAL/RTOS, hardware, binary signing, runtime-loader implementation, packaging, installer, or certification scope.

Selected workflow: `workflows/project-orchestration-pipeline.md`

Selected mode: `modes/rust-systems.md` plus `modes/signal-analysis.md`

## Inputs Reviewed

- Product prompt: `docs/product-prompt.md`
- Project charter: `project-charter.md`
- Requirements: `requirements.md`
- Risk register: `risk-register.md`
- Traceability matrix: `traceability-matrix.md`
- Project state: `project-state.md`
- Transform taxonomy: `docs/analog-transform-taxonomy.md`
- Next milestone roadmap: `docs/next-milestones-roadmap.md`
- MVP exit roadmap: `docs/mvp-exit-roadmap.md`
- MVP exit pipeline report: `docs/m15-m20-mvp-exit-pipeline-report.md`
- MVP exit readiness report: `docs/mvp-exit-readiness-report.md`
- Post-MVP roadmap: `docs/post-mvp-roadmap.md`
- Runtime path pipeline report: `docs/m21-m24-runtime-path-pipeline-report.md`
- Runtime loader design gate: `docs/runtime-loader-design-gate.md`
- Comprehensive conditioning roadmap: `docs/comprehensive-filter-signal-conditioning-roadmap.md`
- Desktop user workflow roadmap: `docs/desktop-user-workflow-roadmap.md`
- Native egui workflow shell roadmap: `docs/egui-workflow-shell-roadmap.md`
- Native egui workflow shell pipeline report: `docs/m43-m48-egui-workflow-shell-pipeline-report.md`
- M13 proposal: `docs/v0.11.0-transform-runtime-profile-validation-milestone-proposal.md`
- M14 proposal: `docs/v0.12.0-high-pass-baseline-correction-milestone-proposal.md`
- Selected standards: Rust, signal-processing, open-source library, data-analysis, environment, granularity.

## Milestones

| Milestone | Goal | Owner Role | Entry Gate | Exit Evidence | Status |
|---|---|---|---|---|---|
| M1-M9 | Validated MVP, embedded/no_std foundation, validation, plotting, measurement/evidence, DSL, portable rule package, controller simulation/deployment config | Multiple roles | Historical gates | Implemented requirements WRA-RQ-001 through WRA-RQ-069 and closed M9 | Complete |
| M10 / v0.8.0 | Transform architecture and capability metadata | Software Architect | Human approval and issue creation | Metadata model, existing-transform mappings, compatibility tests, docs, merged PR #138, closed issues #132 through #137, closed milestone #10 | Complete |
| M11 / v0.9.0 | Pointwise and windowed transform MVP | Core Software Engineer / Systems Engineer | M10 architecture accepted and user requested next milestone | Pointwise, baseline, moving median, metadata, raw-preservation tests, docs, PR #147, closed issues #140 through #146, closed milestone #11 | Complete |
| M12 / v0.10.0 | Event and validation transform MVP | Core Software Engineer / V&V Engineer | M10 accepted, M11 compatibility path established, and user approved M12 | Event records, Schmitt trigger, debounce, glitch removal, event validation, fixtures, docs, PR #156, closed issues #149 through #155, closed milestone #12 | Complete |
| M13 / v0.11.0 | Transform runtime-profile validation | Software Architect / Core Software Engineer / V&V Engineer | M12 closed and user approved continuing | Runtime-profile validator API, timing-evidence checks, waveform/event metadata rejection tests, docs guardrails, PR #164, closed issues #158 through #163, closed milestone #13 | Complete |
| M14 / v0.12.0 | High-pass baseline correction | Systems Engineer / Core Software Engineer / V&V Engineer | M13 closed and user approved continuing | `high_pass_baseline` config, first-order high-pass recurrence, invalid timing checks, metadata tests, CLI/config guardrails, docs, traceability, PR, closed issues, closed milestone | Complete through PR #173; milestone #14 closed |
| M15 / v0.13.0 | Config and schema reference hardening | Documentation Engineer / Software Architect | MVP-exit roadmap approved | Config reference, invalid-config matrix, example index, compatibility/deprecation policy, docs links, tests | Complete; merged in PR #175 |
| M16 / v0.14.0 | Report and artifact contract stabilization | Documentation Engineer / V&V Engineer | M15 reference complete | Report/artifact contract docs, golden artifact matrix, compatibility gate language, expected artifacts | Complete; merged in PR #175 |
| M17 / v0.15.0 | Desktop batch analysis workflow MVP | Core Software Engineer / Test Automation Engineer | M15/M16 compatibility baselines accepted | Batch input docs, deterministic per-run/aggregate outputs, partial-failure tests, unchanged single-run behavior | Complete; merged in PR #175 |
| M18 / v0.16.0 | Rule-package transform semantics | Software Architect / Embedded RTOS Engineer | M13/M14 guardrails accepted and package behavior approved | Transform/package compatibility matrix, supported/rejected export tests, runtime-profile guardrails | Complete; merged in PR #175 |
| M19 / v0.17.0 | Validation corpus and benchmark baseline expansion | V&V Engineer / Performance Engineer | M15-M18 behavior surfaces known | Validation index, known-answer fixtures, negative cases, exact reports, benchmark log scope | Complete; merged in PR #175 |
| M20 / v0.18.0 | MVP exit readiness review | Project Coordinator / Evaluation Engineer | M15-M19 complete | MVP exit readiness report, requirements/traceability/risk audit, docs/onboarding audit, release/community/retrospective gates | Complete; merged in PR #175 |
| M21 | Portable linear pointwise package semantics | Software Architect / Core Software Engineer | User selected runtime path and linear pointwise package semantics | Rule-schema/export support for `offset`, `gain`, and `invert`; rejection retained for unsupported transforms | Complete; merged in PR #175 |
| M22 | Shared runtime-compatible linear transform semantics | Embedded RTOS Engineer / Core Software Engineer | M21 subset accepted | Borrowed-slice runtime helper, caller-owned output buffers, finite validation, desktop parity test | Complete; merged in PR #175 |
| M23 | Package compatibility corpus | V&V Engineer / Test Automation Engineer | M21/M22 behavior implemented | Positive TOML/JSON fixtures and unsupported-transform negative fixtures | Complete; merged in PR #175 |
| M24 | Runtime loader design gate | Embedded RTOS Engineer / Project Coordinator | M21-M23 evidence complete | Accepted subset, memory constraints, failure modes, checksum role, target checks, implementation stop condition | Complete as design only; merged in PR #175 |
| M25 | Transform registry and completeness contract | Software Architect / Documentation Engineer | M25 approval after roadmap review | Transform catalog, metadata contract, docs generation/source-of-truth, package/runtime compatibility rules | Complete; merged in PR #175 |
| M26 | Data cleaning and timing conditioning | Core Software Engineer / V&V Engineer | M25 registry complete | NaN/gap/timestamp repair, timestamp sort/dedupe, trimming/cropping, delay/alignment, fixed-grid normalization, catalog metadata, CLI fixture, package rejection guardrail; `split_by_event` future-gated as multi-artifact segmentation | Complete; merged in PR #175 |
| M27 | Pointwise, normalization, and nonlinear suite | Systems Engineer / Core Software Engineer | M25 registry complete | Absolute/square/square-root/log/exp, normalization modes, tanh/sigmoid, soft limit, piecewise/polynomial transforms, catalog metadata, CLI fixture, package rejection guardrail | Complete; merged in PR #175 |
| M28 | Smoothing, detrending, and baseline suite | Systems Engineer / V&V Engineer | M25 registry complete | Weighted/EMA/boxcar/Gaussian/Savitzky-Golay/median smoothing, rolling baseline, detrending, Hampel/spike cleanup, catalog metadata, CLI fixture, package rejection guardrail | Complete; merged in PR #175 |
| M29 | Standard frequency filter suite | Signal Processing Engineer / Security Engineer | M25 registry and dependency review | FIR/IIR representation, high/band/notch/comb filters, named families, zero-phase offline support, sample-rate validation, catalog metadata, CLI fixture, package rejection guardrail | Complete; merged in PR #175 |
| M30 | Resampling and timing alignment suite | Core Software Engineer / Performance Engineer | M25 registry and dependency review if needed | Fixed-grid resampling/interpolation, down/upsampling, decimation, rational resampling, holds, fractional delay, cross-correlation alignment, jitter correction, clock-drift correction | Complete; merged in PR #175 |
| M31 | Envelope, energy, and calculus suite | Core Software Engineer / V&V Engineer | M25 registry complete | Rectification, RMS, energy/power, derivatives, integrals, area, impulse | Complete; merged in PR #175 |
| M32 | Statistical and correlation suite | Core Software Engineer / V&V Engineer | M25 registry complete | Rolling statistics, z-score/outliers, quantile clipping, scalar statistics, histograms, covariance/correlation feature records | Complete; merged in PR #175 |
| M33 | Spectrum, windows, and time-frequency suite | Signal Processing Engineer / Security Engineer | M25 registry and dependency review | Windows, FFT/IFFT, PSD/Welch, cross-spectrum, coherence, STFT/spectrogram, spectral features | Complete; merged in PR #175 |
| M34 | Fault injection and ADC/DAC simulation suite | Test Automation Engineer / Electrical Signal Integrity Engineer | M25 registry and RNG/noise review complete | Seeded noise/fault/drift/interference simulation, quantizer variants, dithering, companding, jitter, ADC defects, simulation-only docs | Complete; merged in PR #175 |
| M35 | Multi-channel, sensor, and domain conditioning packs | Domain Specialists / Software Architect | M25 registry and domain review | Differential/common-mode, vector/matrix/coordinate, sensor conversions, vibration/control packs, config/CLI fixture, catalog metadata, dependency-gated advanced domain packs | Complete; merged in PR #175 |
| M36 | Completeness, UX, and compatibility closure | Project Coordinator / Evaluation Engineer | M25-M35 complete | Catalog/docs/examples/corpus/benchmark/package-runtime compatibility/release readiness closure | Complete; merged in PR #175 |
| M37 | Desktop user workflow contract | Product Architect / Project Coordinator | User supplied desktop flow direction after M36 merge | Workflow roadmap, README update, requirements WRA-RQ-122 through WRA-RQ-127, risk/state/traceability updates | Complete; merged in PR #177 |
| M38 | Signal source intake and inspect | Core Software Engineer / V&V Engineer | M37 workflow contract accepted | CSV and simulated-source inspection, unsupported live/realtime mode guardrails, tests/docs | Complete; merged in PR #177 |
| M39 | Channel labeling and config scaffold | Core Software Engineer / Documentation Engineer | M38 source inspection behavior accepted | Starter TOML config scaffold with channel labels, units, roles, metadata, transform placeholders, criteria placeholders | Complete; merged in PR #177 |
| M40 | Transform and criteria authoring UX | Product Architect / Documentation Engineer | M39 scaffold accepted | Recipes/templates tying transform catalog, filters, feature/event transforms, event validations, and per-channel criteria together | Complete; merged in PR #177 |
| M41 | Evaluation run bundle | Core Software Engineer / QA Engineer | M40 authoring path accepted | Deterministic results directory contract with source summary, config copy, report text/JSON, optional SVG, transform catalog, and triage notes | Complete; merged in PR #177 |
| M42 | Desktop workflow polish and validation corpus | V&V Engineer / Documentation Engineer | M41 bundle behavior accepted | End-to-end CSV and simulation workflow docs, fixtures, validation corpus entries, and docs validation | Complete; validated and merged in PR #177 |
| M43 | Native GUI gate and dependency review | Product Architect / Security Engineer | User GUI gate approval | Dependency review, exact egui pins, MSRV rationale, GitHub milestone #15 / issue #179, non-goals | Complete; merged in PR #190 |
| M44 | Shared desktop workflow APIs | Software Architect / Core Software Engineer | M43 gate accepted | `ferrisoxide-workflow` shared APIs and CLI delegation | Complete; merged in PR #190 |
| M45 | Optional native egui app shell | Core Software Engineer / DX Engineer | M44 API boundary accepted | `ferrisoxide-gui` default tests and `native` feature app shell | Complete; merged in PR #190 |
| M46 | GUI source inspection and config scaffolding | Core Software Engineer / Documentation Engineer | M45 shell accepted | Source/config panels backed by shared workflow APIs | Complete; merged in PR #190 |
| M47 | GUI run controls and results review | Core Software Engineer / QA Engineer | M46 source/config flow accepted | Analysis/evaluate-bundle controls, status/error output, artifact/result review | Complete; merged in PR #190 |
| M48 | GUI plotting and validation closure | V&V Engineer / Test Automation Engineer | M47 run/results flow accepted | `egui_plot` CSV series review, macOS GUI CI, validation docs | Complete; merged in PR #190 |
| M49 | GUI Source-page CSV picker and header loading | Core Software Engineer / Security Engineer | User Source-page UX gate and dependency pre-approval | Optional native `rfd` file selector, shared CSV header loading API, `Load Channels` action | Complete; merged in PR #190 |
| M50 | GUI Source-page time and channel unit selectors | Core Software Engineer / UX Reviewer | M49 header-loading behavior accepted | Header-populated Time Column dropdown, Time Unit dropdown, per-channel unit rows, GUI state tests | Complete; merged in PR #190 |
| M51 | GUI Plot-page channel selectors | Core Software Engineer / UX Reviewer | User Plot-page UX request and M50 source channel state | Plot-channel checkboxes derived from Source channel state; selected channels filter series without mutating Source assignment | Complete; merged in PR #190 |
| M52 | GUI Plot-page scalable rendering | Core Software Engineer / Performance Engineer | User large-dataset plot performance request and M51 plot selectors | Resolution control, viewport-aware min/max decimation, cached render points, and multiresolution plot pyramids | Complete; merged in PR #190 |
| M53 | GUI Config-page channel builder | Core Software Engineer / UX Reviewer | User Config-page UX request and M50 source channel state | Source-derived channel sections, dropdown-driven filter/action and criterion rows, numeric-only value fields, generated TOML preview, and GUI state tests | Complete; merged in PR #190 |

## Zoom-Level Plan

| Stage | Expected Level | Required Artifacts | Abstraction Review Needed |
|---|---:|---|---|
| M10 architecture | 1-3 | Capability model, metadata fields, runtime profiles, compatibility path, tests | Yes |
| M10 implementation | 3-5 | Files, structs/enums, config adapters, report fields, tests | Yes |
| M11 implementation | 3-5 | Transform modules, config validation, CLI/report integration, fixtures | Yes |
| M12 implementation | 3-5 | Event records, validation records, known-answer fixtures, parity tests | Yes |
| M13 implementation | 3-5 | Runtime validator module, structured errors, timing evidence, transform metadata tests, docs | Yes |
| M14 implementation | 3-5 | Filter enum/config wiring, high-pass recurrence, invalid timing tests, metadata tests, CLI/config coverage, export guardrail test, docs | Yes |
| M15 planning and docs | 2-4 | Config reference, invalid-config matrix, example index, compatibility/deprecation policy | Yes |
| M16 artifact contracts | 2-4 | Report/artifact contract, golden artifact matrix, compatibility gate definitions | Yes |
| M17 batch workflow | 3-5 | Batch input schema, CLI or local runner behavior, aggregate summary schema, tests | Yes |
| M18 package transform semantics | 2-5 | Transform/package matrix, export validator behavior, supported/rejected fixtures, docs | Yes |
| M19 validation corpus | 2-5 | Validation index, known-answer docs, exact reports, negative cases, benchmark log | Yes |
| M20 MVP exit review | 1-3 | Readiness report, gate decisions, risk/traceability closure, post-MVP backlog | Yes |
| M21 package semantics | 3-5 | Schema variants, CLI export mapping, export tests, docs | Yes |
| M22 runtime semantics | 3-5 | Borrowed-slice helper, finite validation, desktop parity coverage | Yes |
| M23 corpus | 2-4 | Positive and negative TOML/JSON fixtures, corpus index | Yes |
| M24 loader design gate | 1-3 | Accepted subset, memory/failure/checksum/target constraints, stop condition | Yes |
| M25 registry | 2-4 | Transform catalog, metadata contract, completeness matrix, docs source-of-truth | Yes |
| M26-M28 conditioning | 3-5 | Transform implementations, config/report integration, fixtures, negative cases, docs | Yes |
| M30 timing suite | 2-5 | Dependency decisions, algorithms, timing fixtures, benchmarks, docs | Yes |
| M31-M32 calculations | 3-5 | Feature records, unit behavior, known-answer fixtures, validation examples | Yes |
| M33 spectrum suite | 2-5 | Dependency review, spectral conventions, known-answer fixtures, offline-only docs | Yes |
| M34 simulation suite | 3-5 | Seed policy, deterministic fixtures, fault/ADC/DAC docs, no-hardware wording | Yes |
| M35 domain packs | 2-5 | Domain assumptions, sensor formulas, unit/alignment tests, calibration disclaimers | Yes |
| M36 closure | 1-3 | Catalog completeness, validation corpus, docs, compatibility, release readiness | Yes |
| M37 workflow contract | 1-3 | Desktop workflow roadmap, README/state/requirements/traceability/risk updates | Yes |
| M38-M39 source and scaffold | 3-5 | Source inspect behavior, starter config schema/output, tests, docs | Yes |
| M40-M42 authoring, bundle, corpus | 2-5 | Recipes, bundle contract, end-to-end fixtures, validation corpus, docs | Yes |
| M43-M53 GUI shell | 2-5 | Dependency review, shared workflow APIs, GUI crate, native feature, Source-page file selector/header loading/unit selectors, channel-based Config builder, Run-page output directory picker, Plot-page channel selectors, scalable Plot-page rendering, app state tests, macOS CI, docs | Yes |

## Task Queue

| Task ID | Task | Owner Role | Inputs | Deliverables | Gate | Status |
|---|---|---|---|---|---|---|
| WRA-TASK-009 | Create next milestone roadmap | Project Coordinator / Product Architect | Taxonomy and project state | `docs/next-milestones-roadmap.md` | Roadmap Gate | Complete locally |
| WRA-TASK-010 | Create M10 transform architecture proposal | Software Architect | Taxonomy, current filter/config/report model | `docs/v0.8.0-transform-architecture-milestone-proposal.md`; WRA-RQ-070 through WRA-RQ-074 | Requirements Gate | Complete locally |
| WRA-TASK-011 | Create M11 pointwise/windowed transform proposal | Software Architect / Systems Engineer | M10 scope and taxonomy | `docs/v0.9.0-pointwise-windowed-transform-mvp-milestone-proposal.md`; WRA-RQ-075 through WRA-RQ-080 | Requirements Gate | Complete locally |
| WRA-TASK-012 | Create M12 event/validation transform proposal | Software Architect / V&V Engineer | M10 scope and switch/test-validation taxonomy | `docs/v0.10.0-event-validation-transform-milestone-proposal.md`; WRA-RQ-081 through WRA-RQ-086 | Requirements Gate | Complete locally |
| WRA-TASK-013 | Convert proposals into local issue placeholders | GitHub Maintainer Specialist | M10-M12 proposals | `docs/next-milestones-issue-planning-report.md` | Issue Planning Gate | Complete locally |
| WRA-TASK-014 | Approve M10 and create GitHub issues | Project Coordinator / GitHub Maintainer Specialist | M10 proposal and placeholders M10-001 through M10-006 | GitHub milestone #10 and issues #132 through #137 | Human Approval Gate | Complete |
| WRA-TASK-015 | Implement M10-001 transform capability vocabulary | Software Architect / Documentation Engineer | Issue #132 | `docs/transform-capability-model.md`, docs links, traceability updates, pipeline report | Implementation Gate | Complete locally |
| WRA-TASK-016 | Implement M10-002 structured transform metadata design | Software Architect / Documentation Engineer | Issue #133 and M10-001 vocabulary | `docs/structured-transform-metadata.md`, report-schema note, docs links, traceability updates, pipeline report | Implementation Gate | Complete locally |
| WRA-TASK-017 | Implement M10-003 current transform metadata mappings | Systems Engineer / Documentation Engineer | Issue #134, M10-001 vocabulary, M10-002 design | `docs/current-transform-metadata-mapping.md`, docs links, traceability updates, pipeline report | Implementation Gate | Complete locally |
| WRA-TASK-018 | Implement M10-004 runtime profile compatibility rules | Embedded RTOS Engineer / Documentation Engineer | Issue #135 and current mappings | `docs/transform-runtime-profile-compatibility.md`, docs links, traceability updates, pipeline report | Implementation Gate | Complete locally |
| WRA-TASK-019 | Implement M10-005 transform docs wording update | Documentation Engineer | Issue #136 and M10 docs | README/architecture/taxonomy/filter wording cleanup, traceability updates, pipeline report | Implementation Gate | Complete locally |
| WRA-TASK-020 | Implement M10-006 transform metadata compatibility and golden-report tests | Verification and Validation Engineer / Core Software Engineer | Issue #137, M10 metadata design, current mappings, runtime compatibility rules | Additive `transform_steps` metadata, compatibility/golden-report tests, traceability updates, pipeline report | Implementation Gate | Complete locally |
| WRA-TASK-021 | Approve M11 and create GitHub issues | Project Coordinator / GitHub Maintainer Specialist | M11 proposal and placeholders M11-001 through M11-007 | GitHub milestone #11 and issues #140 through #146 | Human Approval Gate | Complete |
| WRA-TASK-022 | Implement M11 pointwise/windowed transform MVP | Core Software Engineer / Systems Engineer / V&V Engineer / Documentation Engineer | Issues #140 through #146, M10 metadata model | `crates/ferrisoxide-core`, CLI config test, `examples/m11-transform-config.toml`, docs, traceability, and pipeline report | Implementation/Release Gate | Complete |
| WRA-TASK-023 | Approve M12 and create GitHub issues | Project Coordinator / GitHub Maintainer Specialist | M12 proposal and placeholders M12-001 through M12-007 | GitHub milestone #12 and issues #149 through #155 | Human Approval Gate | Complete |
| WRA-TASK-024 | Implement M12 event/validation transform MVP | Core Software Engineer / V&V Engineer / Documentation Engineer | Issues #149 through #155, M10/M11 metadata model | `crates/ferrisoxide-core/src/event.rs`, config/report/CLI integration, rule-engine Schmitt primitive, examples, docs, traceability, and pipeline report | Implementation/Release Gate | Complete through PR #156 |
| WRA-TASK-025 | Create M13 runtime-profile validation proposal | Software Architect / Project Coordinator | M10/M12 known gaps, transform metadata model, user approval to continue | `docs/v0.11.0-transform-runtime-profile-validation-milestone-proposal.md`; WRA-RQ-087 through WRA-RQ-092 | Requirements Gate | Complete |
| WRA-TASK-026 | Approve M13 and create GitHub issues | Project Coordinator / GitHub Maintainer Specialist | M13 proposal and placeholders M13-001 through M13-006 | GitHub milestone #13 and issues #158 through #163 | Human Approval Gate | Complete |
| WRA-TASK-027 | Implement M13 runtime-profile validator | Core Software Engineer / V&V Engineer / Documentation Engineer | Issues #158 through #163, M10 metadata model, M12 event metadata | `crates/ferrisoxide-core/src/runtime_profile.rs`, docs, tests, traceability, and pipeline report | Implementation/Release Gate | Complete through PR #164 |
| WRA-TASK-028 | Create M14 high-pass baseline correction proposal | Systems Engineer / Project Coordinator | Deferred WRA-RQ-078, M11/M13 metadata and runtime-profile model, user approval to continue | `docs/v0.12.0-high-pass-baseline-correction-milestone-proposal.md`; WRA-RQ-093 through WRA-RQ-098 | Requirements Gate | Complete locally |
| WRA-TASK-029 | Approve M14 and create GitHub issues | Project Coordinator / GitHub Maintainer Specialist | M14 proposal and placeholders M14-001 through M14-006 | GitHub milestone #14 and issues #167 through #172 | Human Approval Gate | Complete |
| WRA-TASK-030 | Implement M14 high-pass baseline correction | Core Software Engineer / V&V Engineer / Documentation Engineer | Issues #167 through #172, existing `[[filters]]` config, M10 metadata model, M13 runtime-profile guardrails | `crates/ferrisoxide-core/src/filter.rs`, config/CLI tests, example config, docs, traceability, risk, and pipeline report | Implementation/Release Gate | Complete through PR #173 |
| WRA-TASK-031 | Create MVP-exit roadmap | Project Coordinator / Product Architect | M14 closure state, current risks, user request to flesh out roadmap before leaving MVP | `docs/mvp-exit-roadmap.md`; updated roadmap, requirements, traceability, risk, orchestration, state | Roadmap Gate | Complete; merged in PR #175 |
| WRA-TASK-032 | Approve M15-M20 local MVP-exit implementation | User / Project Coordinator | `docs/mvp-exit-roadmap.md`, WRA-RQ-099 through WRA-RQ-105, risk updates | Approval to continue through implementation locally | Human Approval Gate | Complete |
| WRA-TASK-033 | Plan M15-M20 local artifacts without GitHub issues | Project Coordinator / GitHub Maintainer Specialist | Approved MVP-exit scope | Local milestone artifacts and explicit no-GitHub-issue decision | Issue Planning Gate | Not Applicable; local implementation approved |
| WRA-TASK-034 | Implement M15 config reference hardening | Documentation Engineer / Software Architect | Current CLI/config/examples/tests | `docs/config-reference.md`, invalid-config matrix, example index, compatibility/deprecation policy | Implementation/Docs Gate | Complete; merged in PR #175 |
| WRA-TASK-035 | Plan M16 artifact contract | Project Coordinator / V&V Engineer | M15 reference | Local artifact contract scope | Issue Planning Gate | Not Applicable; local implementation approved |
| WRA-TASK-036 | Implement M16 report/artifact contract stabilization | Documentation Engineer / V&V Engineer | Report/schema/package/SVG artifacts | `docs/artifact-contract.md`, golden artifact matrix, compatibility gate language | Implementation/Docs Gate | Complete; merged in PR #175 |
| WRA-TASK-037 | Plan M17 batch workflow | Project Coordinator / Core Software Engineer | M15/M16 closure | Local batch workflow design | Issue Planning Gate | Not Applicable; local implementation approved |
| WRA-TASK-038 | Implement M17 desktop batch workflow MVP | Core Software Engineer / Test Automation Engineer | CLI/config/report contracts | Batch input docs, workflow implementation, aggregate summary, tests | Implementation/Testing Gate | Complete; merged in PR #175 |
| WRA-TASK-039 | Plan M18 transform/package semantics | Project Coordinator / Software Architect | M13/M14 guardrails, M16 artifact contract | Local transform/package matrix scope | Issue Planning Gate | Not Applicable; local implementation approved |
| WRA-TASK-040 | Implement M18 rule-package transform semantics | Software Architect / Core Software Engineer / Embedded RTOS Engineer | Runtime-profile validator, package export path | Compatibility matrix, export validator behavior, supported/rejected fixtures, docs | Implementation/V&V Gate | Complete; merged in PR #175 |
| WRA-TASK-041 | Plan M19 validation corpus index | Project Coordinator / V&V Engineer | M15-M18 state and approved validation scope | Local validation corpus index scope | Issue Planning Gate | Not Applicable; local implementation approved |
| WRA-TASK-042 | Implement M19 validation corpus and benchmark baseline | V&V Engineer / Performance Engineer | Existing fixtures/reports/benchmarks | `docs/validation-corpus-index.md`, known-answer mapping, negative cases, benchmark scope | V&V/Performance Gate | Complete; merged in PR #175 |
| WRA-TASK-043 | Plan M20 readiness review | Project Coordinator / Evaluation Engineer | M15-M19 closure state | Readiness gate scope and post-MVP separation | Issue Planning Gate | Not Applicable; local implementation approved |
| WRA-TASK-044 | Run M20 MVP exit readiness review | Project Coordinator / Evaluation Engineer / GitHub Maintainer Specialist | M15-M19 evidence, requirements, traceability, risk, docs, validation, release/community artifacts | `docs/mvp-exit-readiness-report.md`, gate decisions, post-MVP backlog separation | MVP Exit Gate | Complete; merged in PR #175 |
| WRA-TASK-045 | Implement M21 portable linear pointwise package semantics | Software Architect / Core Software Engineer | User-selected runtime path, package semantics, linear pointwise subset | Rule-schema variants, CLI export mapping, export tests, compatibility docs | Implementation/V&V Gate | Complete; merged in PR #175 |
| WRA-TASK-046 | Implement M22 shared runtime-compatible transform semantics | Embedded RTOS Engineer / Core Software Engineer | M21 subset and existing no_std rule engine | Borrowed-slice transform helper, validation errors, desktop parity test | Implementation/V&V Gate | Complete; merged in PR #175 |
| WRA-TASK-047 | Implement M23 package compatibility corpus | V&V Engineer / Test Automation Engineer | M21/M22 behavior | Positive linear pointwise TOML/JSON fixtures, unsupported clamp TOML/JSON fixtures, schema tests, corpus index | V&V Gate | Complete; merged in PR #175 |
| WRA-TASK-048 | Complete M24 runtime loader design gate | Embedded RTOS Engineer / Project Coordinator | M21-M23 evidence and target-profile guardrails | `docs/runtime-loader-design-gate.md`, pipeline report, stop condition for loader implementation | Design Gate | Complete; merged in PR #175 |
| WRA-TASK-049 | Plan M25-M36 comprehensive filter and signal-conditioning path | Product Architect / Software Architect | User request for comprehensive filters and simulated signal conditioning | `docs/comprehensive-filter-signal-conditioning-roadmap.md`, requirements WRA-RQ-110 through WRA-RQ-121, risk and state updates | Roadmap Gate | Complete; merged in PR #175 |
| WRA-TASK-050 | Approve M25 issue creation | User / Project Coordinator | M25-M36 roadmap and scope/risk review | Local M25 scope accepted under user pre-approval for the active goal | Human Approval Gate | Passed for local implementation |
| WRA-TASK-051 | Implement M25 transform registry and completeness contract | Software Architect / Documentation Engineer | Approved M25 scope | Transform catalog, metadata contract, catalog docs, compatibility checks | Implementation Gate | Complete; merged in PR #175 |
| WRA-TASK-052 | Implement M26 data cleaning and timing conditioning | Core Software Engineer / V&V Engineer | M25 registry complete | NaN/gap/timestamp repair, trimming/cropping, alignment, fixed-grid normalization, catalog metadata, CLI fixture, docs, and validation evidence; `split_by_event` future-gated as multi-artifact segmentation | Implementation/V&V Gate | Complete; merged in PR #175 |
| WRA-TASK-053 | Implement M27 pointwise, normalization, and nonlinear conditioning | Systems Engineer / Core Software Engineer | M25 registry complete; M26 closure evidence | Pointwise/nonlinear transforms, normalization modes, formula/domain tests, config/CLI fixture, catalog metadata, docs, and package rejection evidence | Implementation/V&V Gate | Complete; merged in PR #175 |
| WRA-TASK-054 | Implement M28 smoothing, detrending, and baseline conditioning | Systems Engineer / V&V Engineer | M25 registry complete; M27 closure evidence | Smoothing/baseline transforms, edge/drift/spike tests, config/CLI fixture, catalog metadata, docs, and package rejection evidence | Implementation/V&V Gate | Complete; merged in PR #175 |
| WRA-TASK-055 | Implement M29 standard frequency filters | Signal Processing Engineer / Security Engineer | M25 registry complete; dependency review complete | Standard desktop frequency filters, generated response tests, config/CLI fixture, catalog metadata, docs, and package rejection evidence | Implementation/V&V Gate | Complete; merged in PR #175 |
| WRA-TASK-056 | Implement M30 resampling and timing alignment | Core Software Engineer / Performance Engineer | M25 registry complete; M29 closure evidence | Resampling/timing transforms, anti-alias and alignment metadata tests, config/CLI fixture, docs, and package rejection evidence | Implementation/V&V Gate | Complete; merged in PR #175 |
| WRA-TASK-057 | Implement M31 envelope, energy, and calculus calculations | Core Software Engineer / V&V Engineer | M25 registry complete; M30 closure evidence | Waveform filters, feature records, unit/known-answer tests, config/CLI fixture, docs, and package rejection evidence | Implementation/V&V Gate | Complete; merged in PR #175 |
| WRA-TASK-058 | Implement M32 statistics and correlation calculations | Core Software Engineer / V&V Engineer | M25 registry complete; M31 closure evidence | Rolling statistics, scalar statistics, histogram, covariance/correlation feature records, config/CLI fixtures, docs, and package rejection evidence | Implementation/V&V Gate | Complete; merged in PR #175 |
| WRA-TASK-059 | Implement M33 spectrum, windows, and time-frequency analysis | Signal Processing Engineer / Security Engineer | M25 registry complete; dependency review complete | Window coefficients, spectrum/PSD/Welch, paired spectra, harmonic metrics, STFT/spectrogram, spectral features, config/CLI fixture, docs, and validation evidence | Implementation/V&V Gate | Complete; merged in PR #175 |
| WRA-TASK-060 | Implement M34 fault injection and ADC/DAC simulation | Test Automation Engineer / Electrical Signal Integrity Engineer | M25 registry complete; M33 closure evidence; RNG/noise dependency review complete | Seeded fault/noise/ADC-DAC transforms, simulation-only docs, config/CLI fixture, catalog metadata, risk updates, and validation evidence | Implementation/V&V Gate | Complete; merged in PR #175 |
| WRA-TASK-061 | Implement M35 multi-channel, sensor, and domain conditioning packs | Domain Specialists / Software Architect | M25 registry complete; M34 closure evidence; calibration/domain dependency review if needed | Multi-channel transforms, sensor/unit conversions, vibration/control transforms, config/CLI fixtures, catalog metadata, docs, validation evidence, and dependency-gated advanced domain entries | Implementation/V&V Gate | Complete; merged in PR #175 |
| WRA-TASK-062 | Close M36 catalog, UX, compatibility, and release-readiness evidence | Project Coordinator / Evaluation Engineer | M25-M35 complete with validation evidence | Catalog completeness audit, config/doc searchability, validation corpus index, negative-case matrix, benchmark/readiness review, package/runtime compatibility map, release/community/retrospective closure, and stale-reference scan | Full Pipeline Closure Gate | Complete; merged in PR #175 |
| WRA-TASK-063 | Plan M37-M42 desktop user workflow path | Product Architect / Project Coordinator | User desktop-flow direction and M25-M36 closure | `docs/desktop-user-workflow-roadmap.md`, README, requirements WRA-RQ-122 through WRA-RQ-127, traceability, risk, orchestration, state, validation log | Roadmap Gate | Complete; merged in PR #177 |
| WRA-TASK-064 | Implement M38 signal source intake and inspect | Core Software Engineer / V&V Engineer | M37 workflow contract | CSV and simulated-source inspection behavior, unsupported live/realtime guardrails, docs, tests | Implementation/V&V Gate | Complete; merged in PR #177 |
| WRA-TASK-065 | Implement M39 channel labeling and config scaffold | Core Software Engineer / Documentation Engineer | M38 source inspection | Starter TOML scaffold, channel/unit/role validation, docs, tests | Implementation/Docs Gate | Complete; merged in PR #177 |
| WRA-TASK-066 | Implement M40 transform and criteria authoring UX | Product Architect / Documentation Engineer | M39 scaffold | Recipes/templates, transform catalog links, channel criteria examples, raw-lineage warnings | Documentation/QA Gate | Complete; merged in PR #177 |
| WRA-TASK-067 | Implement M41 evaluation run bundle | Core Software Engineer / QA Engineer | M40 authoring path | Bundle contract, command support or wrapper, deterministic artifacts, overwrite/partial failure tests | Implementation/QA Gate | Complete; merged in PR #177 |
| WRA-TASK-068 | Close M42 desktop workflow polish and validation corpus | V&V Engineer / Documentation Engineer | M41 bundle behavior | End-to-end CSV and simulation workflow assets, corpus entries, README/docs polish, validation evidence | Full Workflow Gate | Complete; validated and merged in PR #177 |
| WRA-TASK-069 | Gate M43 native GUI dependencies and tracking | Product Architect / Security Engineer | User GUI gate approval and egui plan | Dependency review, GitHub milestone #15 and issues #179-#184, roadmap, MSRV-compatible pins | Dependency/Human Gate | Complete; merged in PR #190 |
| WRA-TASK-070 | Implement M44 shared workflow API extraction | Software Architect / Core Software Engineer | M37-M42 CLI workflow behavior | `ferrisoxide-workflow` APIs and CLI delegation | Architecture/Implementation Gate | Complete; merged in PR #190 |
| WRA-TASK-071 | Implement M45 optional native egui shell | Core Software Engineer / DX Engineer | M44 APIs and dependency gate | `ferrisoxide-gui` default state crate plus `native` egui app | Implementation Gate | Complete; merged in PR #190 |
| WRA-TASK-072 | Implement M46 GUI source/config panels | Core Software Engineer / Documentation Engineer | M45 app shell | Source inspection and config scaffold/edit/save panels | Implementation/Docs Gate | Complete; merged in PR #190 |
| WRA-TASK-073 | Implement M47 GUI run/results review | Core Software Engineer / QA Engineer | M46 source/config panels | Analyze/evaluate-bundle controls, status, summaries, artifact list | Implementation/QA Gate | Complete; merged in PR #190 |
| WRA-TASK-074 | Close M48 GUI plot and validation evidence | V&V Engineer / Test Automation Engineer | M47 run/results review | Interactive CSV plot series, macOS GUI CI, validation log, pipeline report | V&V/CI Gate | Complete; merged in PR #190 |
| WRA-TASK-075 | Implement M49 GUI CSV picker and channel loading | Core Software Engineer / Security Engineer | User Source-page UX request and dependency pre-approval | `rfd` dependency review, file selector UI, shared header-loading API, GUI state test | Dependency/Implementation Gate | Complete; merged in PR #190 |
| WRA-TASK-076 | Implement M50 GUI Source-page selectors | Core Software Engineer / UX Reviewer | M49 header-loading state | Time Column dropdown, Time Unit dropdown, per-channel unit selectors, request-state tests | Implementation/UX Gate | Complete; merged in PR #190 |
| WRA-TASK-077 | Implement M51 GUI Plot-page selectors | Core Software Engineer / UX Reviewer | M50 Source channel state | Plot-channel checkboxes, filtered plot-series loading, empty-selection error, GUI state tests | Implementation/UX Gate | Complete; merged in PR #190 |
| WRA-TASK-078 | Implement M52 scalable GUI plot rendering | Core Software Engineer / Performance Engineer | M51 Plot-page selector state | Resolution control, viewport min/max decimation, render cache, plot pyramid, GUI state tests | Implementation/Performance Gate | Complete; merged in PR #190 |
| WRA-TASK-079 | Implement M53 GUI Config-page builder | Core Software Engineer / UX Reviewer | M50 Source channel state | Config channel sections, dropdown-only action/criteria choices, numeric controls, generated TOML preview, GUI state tests | Implementation/UX Gate | Complete; merged in PR #190 |

## Approval Gates

| Gate | Trigger | Required Approver | Evidence Needed | Status |
|---|---|---|---|---|
| M10 issue creation approval | Before creating GitHub issues for transform metadata | User / Project Coordinator | M10 proposal, requirements, risk, traceability, issue placeholders | Passed |
| M10 implementation approval | Before editing code for transform metadata | User / Project Coordinator | GitHub milestone #10, issues #132-#137, user request to start completing open issues through the pipeline | Passed for local implementation |
| M11 issue creation and implementation approval | Before creating GitHub issues and editing code for pointwise/windowed transforms | User / Project Coordinator | User request to continue the pipeline with the next milestone; M11 proposal and M10 closure evidence | Passed |
| M12 issue creation and implementation approval | Before creating GitHub issues and editing code for event/validation transforms | User / Project Coordinator | User message "M12 approved" on 2026-06-01; M12 proposal, M10/M11 closure evidence | Passed |
| M13 issue creation and implementation approval | Before creating GitHub issues and editing code for runtime-profile validation | User / Project Coordinator | User approved continuing after M12 closure on 2026-06-01; M13 proposal, M12 closure evidence | Passed for planning and issue creation |
| M14 issue creation and implementation approval | Before creating GitHub issues and editing code for high-pass baseline correction | User / Project Coordinator | User approved continuing after M13 closure on 2026-06-01; M14 proposal, M13 closure evidence | Passed for planning, issue creation, and implementation |
| M15-M20 issue creation approval | Before creating GitHub milestones/issues for MVP-exit work | User / Project Coordinator | `docs/mvp-exit-roadmap.md`, updated requirements, traceability, risk, orchestration plan | Not Applicable; no GitHub issues created |
| M15-M20 implementation approval | Before editing code or opening PRs for MVP-exit milestones | User / Project Coordinator | User approval to continue implementation through MVP exit | Passed for local implementation |
| MVP exit approval | Before claiming the project has moved out of MVP | User / Project Coordinator / Evaluation Engineer | M20 readiness report with explicit `Pass`, `Fail`, or `Blocked` decision and completed release/community/retrospective gates | Passed locally |
| M21-M24 runtime-path implementation approval | Before editing code/docs for the selected runtime-path follow-up | User / Project Coordinator | User selected runtime path, package semantics, and linear pointwise subset, then requested implementation | Passed for local implementation |
| M25 issue creation approval | Before creating issues or implementation tasks for comprehensive conditioning | User / Project Coordinator | User pre-approval for the active goal; `docs/comprehensive-filter-signal-conditioning-roadmap.md`; WRA-RQ-110 through WRA-RQ-121; risk review | Passed for local implementation |
| M25 implementation approval | Before editing code for transform registry and completeness contract | User / Project Coordinator | User pre-approval for the active goal and scope-limited M25 acceptance criteria | Passed for local implementation |
| M26-M36 implementation approval | Before starting each later comprehensive-suite milestone | User / Project Coordinator | User pre-approval for the active goal, prior milestone closure evidence, dependency review where needed, V&V plan | Passed for local implementation under active goal |
| M37 desktop workflow planning approval | Before recording the desktop workflow roadmap and state updates | User / Project Coordinator | User supplied desktop flow direction after M36 mainline merge | Passed for local planning |
| M38-M42 implementation approval | Before editing code for source inspection, config scaffolding, authoring UX, evaluation bundles, or workflow validation corpus | User / Project Coordinator | M37 workflow contract, requirements WRA-RQ-122 through WRA-RQ-127, risk review, V&V plan, and user pre-approval for remaining milestones | Passed for local implementation |
| M43-M53 GUI shell approval | Before adding GUI dependencies, workflow extraction, native app crate, GUI CI, file-dialog dependency, Source-page UX refinements, Config-page builder, Run-page output directory picker, Plot-page channel selectors, or scalable Plot-page rendering | User / Project Coordinator / Security Engineer / UX Reviewer / Performance Engineer | User approved GUI milestone gate, egui implementation plan, Source-page UX request, Config-page UX request, Run-page output directory request, Plot-page UX request, scalable plot rendering request, and pre-approved human gates where dependency review applied; dependency review records exact pins and residual risk; PR #190 merged after `rust` and `gui-macos` checks passed | Passed; merged in PR #190 |
| Runtime loader implementation approval | Before adding a runtime loader, binary package format, target execution, HAL/RTOS integration, or new runtime crate | User / Technical Director / Embedded RTOS Engineer | Reviewed `docs/runtime-loader-design-gate.md`, implementation plan, V&V plan, target checks, risk review | Pending |
| Dependency approval | Before adding third-party crates | User / Security Engineer | Dependency reason, license, alternatives, no_std impact | Pending |
| Schema compatibility approval | Before incompatible report/config schema changes | Project Coordinator / V&V Engineer | Migration plan, golden tests, compatibility statement | Pending |
| Hardware/runtime approval | Before live DAQ, HAL, RTOS SDK, target hardware, unsafe FFI, or global setup | User / Technical Director | Environment plan, risk review, rollback plan, validation scope | Pending |

## Risks To Monitor

| Risk | Owner | Mitigation | Review Trigger |
|---|---|---|---|
| Taxonomy overclaiming | Product Architect / Documentation Engineer | Evidence-level metadata and docs that separate implemented, planned, research, and gated support | Transform docs or roadmap changes |
| Report/config compatibility drift | Core Software Engineer / Documentation Engineer | Additive metadata first; golden-report and config compatibility tests | M10 implementation |
| Baseline transforms hiding real failures | Systems Engineer / V&V Engineer | Preserve raw data, require transform metadata and known-answer fixtures | M11 implementation |
| Desktop/embedded event drift | Embedded RTOS Engineer / V&V Engineer | Shared deterministic logic where practical and parity tests | M12 implementation |
| Runtime-profile validation overclaim | Software Architect / Documentation Engineer | Keep M13 scoped to metadata rejection evidence and require separate gates for runtime execution, hardware, or certification claims | M13 implementation |
| High-pass baseline correction hiding failures | Systems Engineer / V&V Engineer | Preserve raw data, require explicit config, reject invalid timing/cutoff values, and document phase/edge behavior | M14 implementation |
| MVP-exit planning overread as external-release approval | Project Coordinator / GitHub Maintainer Specialist | M15-M20 are merged through PR #175 without GitHub issue creation or release publication | M15-M20 issue creation, release, or post-MVP issue planning |
| Config/report/artifact compatibility drift | Documentation Engineer / V&V Engineer | M15 and M16 docs now exist; keep them updated with future schema/config changes | Config docs, report schema, artifact contract, MVP exit review |
| Batch workflow scope creep | Core Software Engineer / Project Coordinator | M17 is file-based and local; separately gate DAQ, GUI, services, databases, schedulers, and hardware | Batch workflow changes or post-MVP expansion |
| Linear transform package overclaim | Electrical Signal Integrity Engineer / Documentation Engineer | M21 docs label `offset` and `gain` as software transforms only, not calibration or hardware evidence | Rule-package docs, config examples, calibration wording |
| Runtime-loader design overread | Embedded RTOS Engineer / Project Coordinator | M24 is design only; implementation, target execution, and binary package loading remain separately gated | Runtime loader proposal, embedded crate changes, target checks |
| Comprehensive-suite scope explosion | Product Architect / Project Coordinator | M25-M36 split work into bounded families and requires M25 registry before new algorithms | M25-M36 planning, issue creation, release messaging |
| Advanced numeric dependency risk | Security Engineer / Performance Engineer | Require dependency review, known-answer fixtures, benchmarks, and rollback plans for advanced filters/spectral/domain work | M29, M30, M33, or M35 dependency proposals |
| Simulation evidence overclaim | Test Automation Engineer / Documentation Engineer | M34 records deterministic seeds, `evidence_scope = simulation_only`, derived lineage, and no hardware/certification claims | Fault injection, ADC/DAC simulation, validation reports, release messaging |
| Desktop workflow overclaim | Product Architect / Documentation Engineer | M37-M42 label CSV and fixture simulation as current support and keep live/realtime DAQ, GUI, runtime-loader, hardware, release, and certification scope gated | Workflow roadmap, README, source inspection, config scaffold, evaluation bundle, release messaging |
| GUI shell overclaim | Product Architect / Security Engineer / Documentation Engineer | M43-M53 keep native dependencies optional, limit native file/folder selection to local CSV input, TOML config open/save, and evaluation output-directory selection, limit Config builder behavior to generated software config text from loaded Source channels, limit Plot-page selectors to already-derived channels, limit scalable plotting to render-only loaded CSV series, and explicitly exclude packaging, live DAQ, runtime, hardware, release, and certification scope | GUI docs, dependency changes, packaging proposal, live DAQ proposal, release messaging |

## State Updates Required

- Update project state after every milestone stage.
- Update risk register when transform scope, report schema, runtime profiles, or validation claims change.
- Update traceability matrix after requirements, implementation, and tests.
- Update documentation before any public support claim.
- Record durable architecture decisions if M10 changes public config or report strategy.

## Next Role Ticket

You are the Project Orchestrator / Project Coordinator.

Purpose

Select the next explicitly gated follow-up after PR #190 merged the M43-M53 native egui workflow shell; keep release publication, runtime-loader implementation, additional dependencies beyond the reviewed native GUI scope, GUI packaging, live/realtime DAQ, and hardware/certification claims behind their explicit gates.

Responsibilities

- Keep changes inside this project.
- Do not add third-party crates beyond the reviewed optional GUI pins without dependency approval.
- Do not create additional GitHub milestones/issues beyond closed milestone #15 without approval.
- Preserve final M42 validation evidence, PR #177 merge evidence, M43-M53 GUI shell validation evidence, and PR #190 merge evidence.
- Keep runtime-loader implementation, post-MVP hardware/runtime work, GUI packaging, live/realtime DAQ, release publication, and dependency additions separately gated.
- Preserve raw waveform data and avoid unsupported algorithm, hardware, runtime, or certification claims.

Deliverables

- M14 high-pass baseline correction is implemented, validated, merged in PR #173, and closed with milestone #14.
- M15 through M20 are complete through `docs/m15-m20-mvp-exit-pipeline-report.md` and `docs/mvp-exit-readiness-report.md`, and merged in PR #175.
- M21 through M24 are complete through `docs/m21-m24-runtime-path-pipeline-report.md` and `docs/runtime-loader-design-gate.md`, and merged in PR #175.
- M25 is complete through `docs/m25-transform-registry-pipeline-report.md` and `docs/transform-catalog.md`; M26 is complete through `docs/m26-data-cleaning-timing-pipeline-report.md`; M27 is complete through `docs/m27-pointwise-normalization-nonlinear-pipeline-report.md`; M28 is complete through `docs/m28-smoothing-baseline-pipeline-report.md`; M29 is complete through `docs/m29-standard-frequency-filter-pipeline-report.md`; M30 is complete through `docs/m30-resampling-timing-pipeline-report.md`; M31 is complete through `docs/m31-envelope-energy-calculus-pipeline-report.md`; M32 is complete through `docs/m32-statistics-correlation-pipeline-report.md`; M33 is complete through `docs/m33-spectrum-time-frequency-pipeline-report.md`; M34 is complete through `docs/m34-fault-injection-adc-dac-pipeline-report.md`; M35 is complete through `docs/m35-multi-channel-sensor-domain-pipeline-report.md`; M36 is complete through `docs/m36-comprehensive-suite-closure-pipeline-report.md`; M25-M36 are merged in PR #175.
- `docs/post-mvp-roadmap.md` separates future work from the MVP-exit decision.
- `docs/desktop-user-workflow-roadmap.md` defines M37-M42 for source intake, channel labeling, transform/criteria authoring, evaluation bundles, result review, and workflow validation; PR #177 merged this path to `main`.
- `docs/desktop-user-workflow.md`, README, `examples/m42-desktop-workflow-waveform.csv`, and `examples/m42-desktop-workflow-config.toml` document and exercise the implemented CLI desktop workflow on `main`.
- `docs/egui-workflow-shell-roadmap.md`, `docs/m43-m48-egui-workflow-shell-pipeline-report.md`, `ferrisoxide-workflow`, `ferrisoxide-gui`, PR #190, and closed GitHub milestone #15 / closed issues #179-#189 record the M43-M53 native GUI shell, Source/Config/Run/Plot-page UX refinements, and scalable Plot-page rendering.
- Handoff note.

Expected format to receive deliverables

Use the shared handoff note format from root `AGENTS.md`.

## Stop Conditions

- Stop before incompatible report/config schema changes without schema compatibility approval.
- Stop before adding dependencies beyond the reviewed optional GUI pins without review.
- Stop before creating GitHub milestones/issues beyond closed milestone #15, opening external PRs, or publishing releases for post-MVP work without explicit approval.
- Stop before adding live/realtime DAQ, runtime-loader implementation, binary package loading, HAL, RTOS SDK, unsafe FFI, target hardware, GUI packaging/installers, plugin runtime, hosted service, database-backed workflow, scheduler, binary package signing, hardware validation, certification, external PRs, release publication, or public production-readiness claims.
