# M31 Envelope, Energy, And Calculus Pipeline Report

Date: 2026-06-02

Status: Complete locally for desktop `[[filters]]` envelope/calculus transforms and `[[feature_transforms]]` scalar feature records. No GitHub issue, external PR, release tag, dependency addition, live DAQ, HAL/RTOS, runtime-loader implementation, target hardware execution, hardware qualification, or certification evidence was added.

## Scope

M31 implements the sixth post-catalog transform expansion using the M25 source-of-truth catalog contract and the M26-M30 implementation pattern.

In scope:

- `half_wave_rectify` and `full_wave_rectify`.
- Causal `envelope`, `moving_rms`, and `peak_hold`.
- `first_derivative`, `second_derivative`, `integral`, `cumulative_integral`, `leaky_integrator`, and `slope_detection`.
- Scalar `feature_records` for `rms`, `peak_to_peak`, `crest_factor`, `energy`, `power`, `area_under_curve`, and `impulse_estimate`.
- TOML `[[feature_transforms]]` support, JSON/text report rendering, CLI fixture coverage, catalog metadata tests, invalid config/domain tests, docs, traceability, and risk updates.

Out of scope:

- Rule-package export support for M31 filters or feature transforms.
- Hilbert envelope and analytic-signal workflows; remain dependency/design-gated.
- Statistical/correlation calculations; remain M32.
- Spectrum/time-frequency analysis; remains M33.
- New dependencies, runtime loaders, hardware, live DAQ, HAL/RTOS, signing, or certification evidence.

## Files

| Area | Evidence |
|---|---|
| Core waveform transforms | `crates/ferrisoxide-core/src/filter.rs` |
| Feature records | `crates/ferrisoxide-core/src/feature.rs`, `crates/ferrisoxide-core/src/report.rs` |
| Config parsing | `crates/ferrisoxide-core/src/config.rs` |
| Catalog | `crates/ferrisoxide-core/src/transform_catalog.rs` |
| CLI tests/package guard | `crates/ferrisoxide-cli/src/main.rs`, `crates/ferrisoxide-cli/src/bin/ferrisoxide-signal-bench.rs` |
| Fixture/example | `examples/m31-calculus-waveform.csv`, `examples/m31-calculus-config.toml` |
| Docs | `docs/config-reference.md`, `docs/current-transform-metadata-mapping.md`, `docs/report-schema.md`, `docs/transform-catalog.md`, `docs/transform-package-compatibility.md`, `docs/validation-corpus-index.md` |
| Governance | `requirements.md`, `traceability-matrix.md`, `risk-register.md`, `orchestration-plan.md`, `project-state.md`, README, CHANGELOG |

## Gate Decisions

| Stage | Gate Decision | Evidence | Residual Risk | Owner |
|---|---|---|---|---|
| Intake | Pass | User requested comprehensive filters/calculations and pre-approved human gates for the active goal. | Scope must remain sampled-waveform software conditioning. | Project Coordinator |
| Research | Pass | M31 roadmap scope, M25 catalog, M26-M30 transform patterns, report schema, and feature/validation separation reviewed. | Hilbert/analytic-signal workflows remain dependency/design-gated. | Open Source Research Engineer / Signal Processing Engineer |
| Requirements | Pass | WRA-RQ-116 updated to implemented locally with concrete filter and feature names. | M32-M36 requirements remain separate and planned. | Software Architect |
| Architecture | Pass | M31 uses existing `Filter`/`FilterStep` for derived waveforms and adds `FeatureTransformStep`/`FeatureRecord` for scalar evidence. | Feature records are desktop analysis evidence, not portable runtime rules. | Software Architect |
| Abstraction Review | Pass | Transform names, parameters, output kinds, units, sample-rate needs, causal/offline behavior, package support, and evidence level are explicit in catalog/docs. | Users may mistake energy/power features for calibrated physical energy. | Abstraction Review Engineer |
| Human Approval | Pass | User pre-approved all human gates for this active goal on 2026-06-02. | Approval does not approve dependencies, hardware, release, or certification scope. | User / Project Coordinator |
| Issue Planning | Not Applicable | Local pipeline execution continues without GitHub issue creation by instruction/context. | External tracking may still be useful before public PR work. | GitHub Maintainer Specialist |
| Implementation | Pass | Core waveform transforms, feature record module, config parser support, report rendering, catalog entries, CLI tests, and fixture examples implemented. | Hilbert envelope remains dependency/design-gated. | Core Software Engineer / V&V Engineer |
| Testing | Pass | Focused M31 core/config/feature/catalog and CLI tests passed; full validation is recorded in `docs/validation-log.md`. | None for scoped desktop transforms after local validation. | Test Automation Engineer |
| V&V | Pass | Known-answer tests cover rectification, envelope, moving RMS, derivatives, integrals, leaky integration, slope detection, scalar feature values, units, catalog parity, and invalid inputs. | Fixture coverage is software-only and not hardware calibration evidence. | V&V Engineer |
| QA | Pass | Formatting, workspace tests, clippy, diff checks, whitespace scan, and Markdown link scan are recorded in `docs/validation-log.md`. | Automated generated-doc drift checks remain future tooling. | QA Engineer |
| Security | Pass | No new dependencies, network behavior, credentials, signing, auth, unsafe FFI, SDKs, or permission changes were added. | Future analytic-signal/spectral dependencies still require security review. | Security Engineer |
| Performance | Not Applicable | No benchmark or throughput claim added. | Large-waveform feature calculation performance remains future benchmark work. | Performance Engineer |
| Documentation | Pass | Config reference, report schema, metadata mapping, transform catalog, package compatibility, validation corpus, roadmap, README, and changelog updated. | Future docs must keep M31 package/runtime rejection and feature/validation separation visible. | Documentation Engineer |
| Code Review | Pass | Local review checked raw preservation, finite/time validation, feature record separation, unit evidence, package rejection, and catalog metadata parity. | External maintainer review remains future PR work. | Code Reviewer |
| Evaluation | Pass | WRA-RQ-116 has implementation, tests, docs, traceability, risk, state, and validation evidence. | Completion is local until external PR/CI is run. | Evaluation Engineer |
| Release | Not Applicable | No external PR or release requested for this local pipeline slice. | Release messaging must not claim M32-M36 implementation. | GitHub Maintainer Specialist |
| Community | Not Applicable | No public issue, milestone, or release was opened. | Community follow-up may be needed before publishing. | Project Coordinator |
| Retrospective | Pass | M31 confirmed feature records can coexist with validation results while preserving the catalog-first transform pattern. | M32 needs a careful convention for rolling/statistical feature IDs and output records. | Project Coordinator |

## Acceptance Criteria

| Criterion | Status | Evidence |
|---|---|---|
| Calculation units are documented. | Pass | Feature records emit source units, `ratio`, `<unit>^2*s`, `<unit>^2`, or `<unit>*s`; docs and tests assert unit behavior. |
| Time-axis requirements are enforced. | Pass | Derivative/integral/leaky/slope and energy/area/power features validate finite strictly increasing time axes. |
| Known-answer fixtures cover calculus and energy behavior. | Pass | Core M31 tests assert known outputs for waveform filters and scalar features; CLI fixture exercises the full chain. |
| Feature records stay separate from validation decisions. | Pass | `feature_records` do not affect `overall_outcome`; criteria and event validations remain the only pass/fail sources. |
| Rule-package export remains guarded. | Pass | CLI rejection matrix includes M31 waveform filters and export rejects configs with `feature_transforms`. |

## Validation Summary

Focused M31 checks:

```text
cargo test -p ferrisoxide-core m31 -- --nocapture
cargo test -p ferrisoxide-cli m31 -- --nocapture
cargo test -p ferrisoxide-cli rule_package_export_rejects_remaining_desktop_only_transform_matrix -- --nocapture
cargo run -p ferrisoxide-cli --bin ferrisoxide-signal -- analyze --input examples/m31-calculus-waveform.csv --config examples/m31-calculus-config.toml --format json
```

Full validation commands and results are recorded in `docs/validation-log.md`.

## Hand-Off Note

Role: Core Software Engineer / V&V Engineer / Documentation Engineer
Goal: Implement M31 desktop envelope, energy, and calculus filters/features.
Files changed: Core filters/features/config/catalog/report, CLI tests, M31 example fixture/config, config/report/transform docs, requirements, traceability, risk, orchestration, project state, README, CHANGELOG, validation log, and this report.
Checks run: See `docs/validation-log.md`.
Status: Complete locally.
Known gaps: M32-M36 algorithm families remain planned; Hilbert envelope remains dependency/design-gated; no package/runtime/hardware/certification support was added.
Next recommended step: Implement M32 statistical and correlation calculations using the M25 catalog and M26-M31 implementation pattern.
