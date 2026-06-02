# M32 Statistics And Correlation Pipeline Report

Date: 2026-06-02

Status: Complete locally for desktop `[[filters]]` statistics transforms and `[[feature_transforms]]` statistical/correlation feature records. No GitHub issue, external PR, release tag, dependency addition, live DAQ, HAL/RTOS, runtime-loader implementation, target hardware execution, hardware qualification, or certification evidence was added.

## Scope

M32 implements the seventh post-catalog transform expansion using the M25 source-of-truth catalog contract and the M26-M31 implementation pattern.

In scope:

- Waveform filters: `rolling_mean`, `rolling_variance`, `rolling_stddev`, `rolling_min`, `rolling_max`, `z_score`, `outlier_detection`, and `quantile_clip`.
- Feature records: `mean`, `median`, `mode`, `min`, `max`, `variance`, `standard_deviation`, `skewness`, `kurtosis`, `percentile`, `quantile`, `histogram`, `covariance`, `correlation`, `autocorrelation`, and `cross_correlation`.
- Method context for percentiles, quantiles, histogram bins, comparison channels, and lag samples.
- TOML config support, JSON/text report rendering through existing `feature_records`, CLI fixture coverage, catalog metadata tests, invalid config/domain tests, docs, traceability, and risk updates.

Out of scope:

- Rule-package export support for M32 filters or feature transforms.
- FFT, PSD, Welch, windows, STFT, spectrogram, or spectral features; remain M33 and dependency-gated.
- Fault injection, ADC/DAC expansion, domain sensor packs, runtime loaders, hardware, live DAQ, HAL/RTOS, signing, or certification evidence.

## Files

| Area | Evidence |
|---|---|
| Core waveform transforms | `crates/ferrisoxide-core/src/filter.rs` |
| Feature records | `crates/ferrisoxide-core/src/feature.rs`, `crates/ferrisoxide-core/src/report.rs` |
| Config parsing | `crates/ferrisoxide-core/src/config.rs` |
| Catalog | `crates/ferrisoxide-core/src/transform_catalog.rs` |
| CLI tests/package guard | `crates/ferrisoxide-cli/src/main.rs` |
| Fixtures/examples | `examples/m32-statistics-waveform.csv`, `examples/m32-statistics-config.toml`, `examples/m32-statistics-filters-config.toml` |
| Docs | `docs/config-reference.md`, `docs/current-transform-metadata-mapping.md`, `docs/report-schema.md`, `docs/transform-catalog.md`, `docs/transform-package-compatibility.md`, `docs/validation-corpus-index.md` |
| Governance | `requirements.md`, `traceability-matrix.md`, `risk-register.md`, `orchestration-plan.md`, `project-state.md`, README, CHANGELOG |

## Gate Decisions

| Stage | Gate Decision | Evidence | Residual Risk | Owner |
|---|---|---|---|---|
| Intake | Pass | User requested comprehensive filters/calculations and pre-approved human gates for the active goal. | Scope must remain sampled-waveform software conditioning. | Project Coordinator |
| Research | Pass | M32 roadmap scope, M25 catalog, M26-M31 transform patterns, report schema, and feature/validation separation reviewed. | Spectral/time-frequency statistics remain M33 and dependency-gated. | Open Source Research Engineer / Signal Processing Engineer |
| Requirements | Pass | WRA-RQ-117 updated to implemented locally with concrete filter and feature names. | M33-M36 requirements remain separate and planned. | Software Architect |
| Architecture | Pass | M32 uses `FilterStep` for derived waveform transforms and `FeatureTransformStep`/`FeatureRecord` for scalar, histogram, and correlation evidence. | Feature records are desktop analysis evidence, not portable runtime rules. | Software Architect |
| Abstraction Review | Pass | Transform names, parameters, output kinds, method context, lag convention, package support, and evidence level are explicit in catalog/docs. | Users may overinterpret correlation as causality or validation. | Abstraction Review Engineer |
| Human Approval | Pass | User pre-approved all human gates for this active goal on 2026-06-02. | Approval does not approve dependencies, hardware, release, or certification scope. | User / Project Coordinator |
| Issue Planning | Not Applicable | Local pipeline execution continues without GitHub issue creation by instruction/context. | External tracking may still be useful before public PR work. | GitHub Maintainer Specialist |
| Implementation | Pass | Core waveform transforms, feature record expansion, config parser support, catalog entries, CLI tests, and fixture examples implemented. | FFT/STFT and advanced spectral outputs remain dependency-gated. | Core Software Engineer / V&V Engineer |
| Testing | Pass | Focused M32 core/config/feature/catalog and CLI tests passed; full validation is recorded in `docs/validation-log.md`. | None for scoped desktop transforms after local validation. | Test Automation Engineer |
| V&V | Pass | Known-answer tests cover rolling statistics, z-score, outlier flags, quantile clipping, scalar statistics, histogram bins, covariance, correlation, autocorrelation, cross-correlation, method context, and invalid inputs. | Fixture coverage is software-only and not process-capability or hardware evidence. | V&V Engineer |
| QA | Pass | Formatting, workspace tests, clippy, diff checks, whitespace scan, and Markdown link scan are recorded in `docs/validation-log.md`. | Automated generated-doc drift checks remain future tooling. | QA Engineer |
| Security | Pass | No new dependencies, network behavior, credentials, signing, auth, unsafe FFI, SDKs, or permission changes were added. | Future FFT/noise/math dependencies still require security review. | Security Engineer |
| Performance | Not Applicable | No benchmark or throughput claim added. | Large-waveform histogram/correlation performance remains future benchmark work. | Performance Engineer |
| Documentation | Pass | Config reference, report schema, metadata mapping, transform catalog, package compatibility, validation corpus, roadmap, README, and changelog updated. | Future docs must keep feature/validation separation and lag convention visible. | Documentation Engineer |
| Code Review | Pass | Local review checked raw preservation, finite/domain validation, method context, lag convention, feature record separation, package rejection, and catalog metadata parity. | External maintainer review remains future PR work. | Code Reviewer |
| Evaluation | Pass | WRA-RQ-117 has implementation, tests, docs, traceability, risk, state, and validation evidence. | Completion is local until external PR/CI is run. | Evaluation Engineer |
| Release | Not Applicable | No external PR or release requested for this local pipeline slice. | Release messaging must not claim M33-M36 implementation. | GitHub Maintainer Specialist |
| Community | Not Applicable | No public issue, milestone, or release was opened. | Community follow-up may be needed before publishing. | Project Coordinator |
| Retrospective | Pass | M32 confirmed histogram multi-record features can coexist with scalar feature records and validation results. | M33 needs dependency review and explicit spectral scaling conventions before implementation. | Project Coordinator |

## Acceptance Criteria

| Criterion | Status | Evidence |
|---|---|---|
| Empty, constant, NaN, and non-finite inputs are explicit. | Pass | M32 config/core/feature tests reject invalid windows, quantiles, histogram ranges, constant z-score/skew/correlation inputs, invalid lag values, non-finite samples, and unknown channels. |
| Rolling window behavior is deterministic. | Pass | M32 filter tests assert trailing shrinking-window outputs for mean, variance, stddev, min, and max. |
| Correlation lag conventions are documented. | Pass | Config/report docs and method context define `channel[t]` versus `other_channel[t + lag_samples]`. |
| Feature records include method context. | Pass | Percentile, quantile, histogram, covariance/correlation, autocorrelation, and cross-correlation records include method context where applicable. |
| Validation examples separate features from pass/fail. | Pass | M32 examples emit `feature_records` separately while criteria still control `overall_outcome`. |

## Validation Summary

Focused M32 checks:

```text
cargo test -p ferrisoxide-core m32 -- --nocapture
cargo test -p ferrisoxide-cli m32 -- --nocapture
cargo test -p ferrisoxide-cli lists_transform_catalog -- --nocapture
cargo test -p ferrisoxide-cli rule_package_export_rejects_remaining_desktop_only_transform_matrix -- --nocapture
cargo run --quiet -p ferrisoxide-cli --bin ferrisoxide-signal -- analyze --input examples/m32-statistics-waveform.csv --config examples/m32-statistics-config.toml --format json
cargo run --quiet -p ferrisoxide-cli --bin ferrisoxide-signal -- analyze --input examples/m32-statistics-waveform.csv --config examples/m32-statistics-filters-config.toml --format json
```

Full validation commands and results are recorded in `docs/validation-log.md`.

## Hand-Off Note

Role: Core Software Engineer / V&V Engineer / Documentation Engineer
Goal: Implement M32 desktop statistics and correlation filters/features.
Files changed: Core filters/features/config/catalog/report-adjacent metadata, CLI tests, M32 example fixtures/configs, config/report/transform docs, requirements, traceability, risk, orchestration, project state, README, CHANGELOG, validation log, and this report.
Checks run: See `docs/validation-log.md`.
Status: Complete locally.
Known gaps: M33-M36 algorithm families remain planned; FFT/STFT/spectral work requires dependency review; no package/runtime/hardware/certification support was added.
Next recommended step: Implement M33 spectrum, windows, and time-frequency analysis after dependency review.
