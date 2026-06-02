# M28 Smoothing, Detrending, And Baseline Pipeline Report

Date: 2026-06-02

Status: Complete locally for desktop `[[filters]]` smoothing, detrending, baseline correction, Hampel filtering, and spike cleanup. No GitHub issue, external PR, release tag, dependency addition, live DAQ, HAL/RTOS, runtime-loader implementation, target hardware execution, hardware qualification, or certification evidence was added.

## Scope

M28 implements the third post-catalog transform expansion using the M25 source-of-truth catalog contract and the M26/M27 implementation pattern.

In scope:

- `weighted_moving_average` with explicit positive weights.
- `exponential_moving_average` with `alpha`.
- `boxcar_smoothing`, `gaussian_smoothing`, `savitzky_golay`, and `centered_moving_median`.
- `rolling_mean_baseline` and `rolling_median_baseline`.
- `linear_detrend` and `polynomial_detrend`.
- `hampel_filter` and `spike_remove`.
- TOML config support, CLI fixture coverage, catalog metadata tests, formula/edge tests, docs, traceability, and risk updates.

Out of scope:

- Rule-package export support for M28 transforms.
- Broad frequency filter design, FIR/IIR families, standard low/high/band/notch filters, and zero-phase frequency filters; these remain M29.
- Advanced resampling/timing alignment; remains M30.
- New dependencies, runtime loaders, hardware, live DAQ, HAL/RTOS, signing, or certification evidence.

## Files

| Area | Evidence |
|---|---|
| Core transforms | `crates/ferrisoxide-core/src/filter.rs` |
| Config parsing | `crates/ferrisoxide-core/src/config.rs` |
| Catalog | `crates/ferrisoxide-core/src/transform_catalog.rs` |
| CLI tests/package guard | `crates/ferrisoxide-cli/src/main.rs` |
| Fixture/example | `examples/m28-smoothing-waveform.csv`, `examples/m28-smoothing-config.toml` |
| Docs | `docs/config-reference.md`, `docs/current-transform-metadata-mapping.md`, `docs/transform-catalog.md`, `docs/transform-package-compatibility.md` |
| Governance | `requirements.md`, `traceability-matrix.md`, `risk-register.md`, `orchestration-plan.md`, `project-state.md`, README, CHANGELOG |

## Gate Decisions

| Stage | Gate Decision | Evidence | Residual Risk | Owner |
|---|---|---|---|---|
| Intake | Pass | User requested comprehensive filters and pre-approved human gates for the active goal. | Scope must remain sampled-waveform software conditioning. | Project Coordinator |
| Research | Pass | M28 roadmap scope, existing M11/M14 smoothing/baseline behavior, M25 catalog, and M26/M27 pipeline pattern reviewed. | Advanced frequency-filter and dependency choices remain future milestones. | Open Source Research Engineer |
| Requirements | Pass | WRA-RQ-113 updated to implemented locally with concrete transform names. | Future frequency filters and spectral calculations still need separate requirements evidence. | Software Architect |
| Architecture | Pass | M28 uses existing `Filter`/`FilterStep`, `Waveform` derived lineage, `TransformStepMetadata`, `TransformCatalogEntry`, and TOML config surfaces. | Offline centered transforms are not portable runtime semantics. | Software Architect |
| Abstraction Review | Pass | Transform names, parameters, categories, causal/offline behavior, package support, output kind, and evidence level are explicit in catalog entries. | Baseline/detrending transforms can hide low-frequency failures if users ignore raw lineage. | Abstraction Review Engineer |
| Human Approval | Pass | User pre-approved all human gates for this active goal on 2026-06-02. | Approval does not approve dependencies, hardware, release, or certification scope. | User / Project Coordinator |
| Issue Planning | Not Applicable | Local pipeline execution continues without GitHub issue creation by instruction/context. | External tracking may still be useful before public PR work. | GitHub Maintainer Specialist |
| Implementation | Pass | Core transforms, config parser support, catalog entries, CLI tests, and fixture examples implemented. | None for scoped desktop transforms. | Core Software Engineer |
| Testing | Pass | Focused M28 core/config/catalog and CLI tests passed; full validation is recorded in `docs/validation-log.md`. | None after local validation. | Test Automation Engineer |
| V&V | Pass | Known-answer tests cover causal edges, centered offline edges, Savitzky-Golay quadratic preservation, rolling baseline subtraction, detrending, Hampel/spike replacement, invalid inputs, catalog metadata, and CLI evidence. | Fixture coverage is software-only and not hardware calibration evidence. | V&V Engineer |
| QA | Pass | Formatting, workspace tests, clippy, diff checks, whitespace scan, and Markdown link scan are recorded in `docs/validation-log.md`. | Automated config-doc drift checks remain future tooling. | QA Engineer |
| Security | Not Applicable | No new dependencies, network behavior, credentials, signing, auth, unsafe FFI, SDKs, or permission changes. | Future dependency-gated transforms still need security review. | Security Engineer |
| Performance | Not Applicable | No benchmark or throughput claim added. | Large-waveform smoothing/filter performance remains future benchmark work. | Performance Engineer |
| Documentation | Pass | Config reference, metadata mapping, transform catalog, package compatibility, roadmap, README, and changelog updated. | Future docs must keep M28 package/runtime rejection visible. | Documentation Engineer |
| Code Review | Pass | Local review checked raw preservation, edge behavior, non-finite handling, package rejection, and catalog metadata parity. | External maintainer review remains future PR work. | Code Reviewer |
| Evaluation | Pass | WRA-RQ-113 has implementation, tests, docs, traceability, risk, state, and validation evidence. | Completion is local until external PR/CI is run. | Evaluation Engineer |
| Release | Not Applicable | No external PR or release requested for this local pipeline slice. | Release messaging must not claim M29-M36 implementation. | GitHub Maintainer Specialist |
| Community | Not Applicable | No public issue, milestone, or release was opened. | Community follow-up may be needed before publishing. | Project Coordinator |
| Retrospective | Pass | M28 confirmed dependency-free small polynomial fitting and robust cleanup can fit the catalog-first pattern. | Future M29 filter families may need dependency review or narrower design gates. | Project Coordinator |

## Acceptance Criteria

| Criterion | Status | Evidence |
|---|---|---|
| Edge behavior is documented and tested. | Pass | Core tests cover trailing weighted/EMA edges, centered boxcar/Gaussian/median edges, and fixed-window Savitzky-Golay/Hampel/spike edge behavior. |
| Phase/latency behavior is explicit. | Pass | Catalog and metadata rows distinguish `delay`, `none`, and `nonlinear` phase effects. |
| Causal and offline variants are separated. | Pass | Causal transforms emit `causal = true`, `streaming_supported = true`; centered/offline transforms emit `offline_only = true`. |
| Detrending/baseline transforms include overfiltering warnings. | Pass | Risk register, package compatibility, metadata mapping, and docs state baseline/detrending can hide failures and are not calibrated drift removal. |
| Synthetic drift and noise fixtures prove expected behavior. | Pass | Core tests cover detrending to zero residual, rolling baseline subtraction, quadratic shape preservation, and spike replacement; the CLI M28 fixture runs the full transform chain. |
| Rule-package export remains guarded. | Pass | CLI rejection matrix includes every M28 transform. |

## Validation Summary

Focused M28 checks:

```text
cargo test -p ferrisoxide-core m28 -- --nocapture
cargo test -p ferrisoxide-core transform_catalog -- --nocapture
cargo test -p ferrisoxide-core filter_config_covers_m28 -- --nocapture
cargo test -p ferrisoxide-cli analyzes_config_with_m28_smoothing_baseline_filters -- --nocapture
cargo test -p ferrisoxide-cli lists_transform_catalog -- --nocapture
cargo test -p ferrisoxide-cli rule_package_export_rejects_remaining_desktop_only_transform_matrix -- --nocapture
cargo run --quiet --bin ferrisoxide-signal -- analyze --input examples/m28-smoothing-waveform.csv --config examples/m28-smoothing-config.toml --format json
```

Full validation commands and results are recorded in `docs/validation-log.md`.

## Hand-Off Note

Role: Core Software Engineer / V&V Engineer / Documentation Engineer
Goal: Implement M28 smoothing, detrending, baseline, Hampel, and spike-cleanup conditioning transforms.
Files changed: Core filters/config/catalog, CLI tests, M28 example fixture/config, config and transform docs, requirements, traceability, risk, orchestration, project state, README, CHANGELOG, validation log, and this report.
Checks run: See `docs/validation-log.md`.
Status: Complete locally.
Known gaps: M29 standard frequency filters remain next; M30-M35 algorithm families remain planned; no package/runtime/hardware/certification support was added.
Next recommended step: Implement M29 standard frequency filters using the M25 catalog and M26-M28 implementation pattern.
