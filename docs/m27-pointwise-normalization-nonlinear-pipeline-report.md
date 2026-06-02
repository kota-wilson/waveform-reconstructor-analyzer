# M27 Pointwise, Normalization, And Nonlinear Pipeline Report

Date: 2026-06-02

Status: Complete locally for desktop `[[filters]]` pointwise, normalization, and nonlinear conditioning transforms. No GitHub issue, external PR, release tag, dependency addition, live DAQ, HAL/RTOS, runtime-loader implementation, target hardware execution, hardware qualification, or certification evidence was added.

## Scope

M27 implements the second post-catalog transform expansion using the M25 source-of-truth catalog contract and the M26 implementation pattern.

In scope:

- `absolute_value`, `square`, `square_root`, `log`, and `exp`.
- `normalize` with `zero_to_one`, `minus_one_to_one`, `z_score`, and configured `range` modes.
- `tanh`, `sigmoid`, and `soft_limit`.
- `piecewise_linear` lookup-style mapping and `polynomial` correction.
- Existing M11 pointwise support remains part of the suite: `offset`, `gain`, `invert`, `clamp`, and `deadband`.
- TOML config support, CLI fixture coverage, catalog metadata tests, formula/domain tests, docs, traceability, and risk updates.

Out of scope:

- Sensor calibration evidence, hardware accuracy claims, live DAQ, target hardware, HAL/RTOS, runtime-loader work, signing, or certification evidence.
- Rule-package export support for M27 transforms.
- Smoothing, detrending, and baseline cleanup beyond existing M11/M14 transforms; this remains M28.
- Standard frequency filters, advanced resampling, spectrum, fault injection, and domain packs; these remain M29-M35.

## Files

| Area | Evidence |
|---|---|
| Core transforms | `crates/ferrisoxide-core/src/filter.rs` |
| Config parsing | `crates/ferrisoxide-core/src/config.rs` |
| Catalog | `crates/ferrisoxide-core/src/transform_catalog.rs` |
| CLI tests/package guard | `crates/ferrisoxide-cli/src/main.rs` |
| Fixture/example | `examples/m27-pointwise-waveform.csv`, `examples/m27-pointwise-config.toml` |
| Docs | `docs/config-reference.md`, `docs/current-transform-metadata-mapping.md`, `docs/transform-catalog.md`, `docs/transform-package-compatibility.md` |
| Governance | `requirements.md`, `traceability-matrix.md`, `risk-register.md`, `orchestration-plan.md`, `project-state.md`, README, CHANGELOG |

## Gate Decisions

| Stage | Gate Decision | Evidence | Residual Risk | Owner |
|---|---|---|---|---|
| Intake | Pass | User requested comprehensive filters and pre-approved human gates for the active goal. | Scope must remain sampled-waveform software conditioning. | Project Coordinator |
| Research | Pass | Existing M11 pointwise transforms, M25 catalog, M26 pipeline pattern, and M27 roadmap scope reviewed. | Advanced DSP and dependency choices remain future milestones. | Open Source Research Engineer |
| Requirements | Pass | WRA-RQ-112 updated to implemented locally with concrete transform names. | Later sensor-specific calibration needs separate evidence. | Software Architect |
| Architecture | Pass | M27 uses existing `Filter`/`FilterStep`, `Waveform` derived lineage, `TransformStepMetadata`, `TransformCatalogEntry`, and TOML config surfaces. | Normalization is offline-only; package/runtime semantics remain gated. | Software Architect |
| Abstraction Review | Pass | Transform names, parameters, categories, domain checks, package support, output kind, and evidence level are explicit in catalog entries. | Nonlinear transforms can change interpretation if users ignore lineage. | Abstraction Review Engineer |
| Human Approval | Pass | User pre-approved all human gates for this active goal on 2026-06-02. | Approval does not approve dependencies, hardware, release, or certification scope. | User / Project Coordinator |
| Issue Planning | Not Applicable | Local pipeline execution continues without GitHub issue creation by instruction/context. | External tracking may still be useful before public PR work. | GitHub Maintainer Specialist |
| Implementation | Pass | Core transforms, config parser support, catalog entries, CLI tests, and fixture examples implemented. | None for scoped desktop transforms. | Core Software Engineer |
| Testing | Pass | Focused M27 core/config/catalog and CLI tests passed; full validation is recorded in `docs/validation-log.md`. | None after local validation. | Test Automation Engineer |
| V&V | Pass | Known-answer tests cover formulas, normalization modes, domain errors, invalid config, catalog metadata, CLI evidence, and raw preservation. | Fixture coverage is software-only and not hardware calibration evidence. | V&V Engineer |
| QA | Pass | Formatting, workspace tests, clippy, diff checks, whitespace scan, and Markdown link scan are recorded in `docs/validation-log.md`. | Automated config-doc drift checks remain future tooling. | QA Engineer |
| Security | Not Applicable | No new dependencies, network behavior, credentials, signing, auth, unsafe FFI, SDKs, or permission changes. | Future dependency-gated transforms still need security review. | Security Engineer |
| Performance | Not Applicable | No benchmark or throughput claim added. | Large-waveform performance benchmarks remain future work. | Performance Engineer |
| Documentation | Pass | Config reference, metadata mapping, transform catalog, package compatibility, roadmap, README, and changelog updated. | Future docs must keep M27 package/runtime rejection visible. | Documentation Engineer |
| Code Review | Pass | Local review checked raw preservation, domain handling, normalization behavior, non-finite handling, package rejection, and catalog metadata parity. | External maintainer review remains future PR work. | Code Reviewer |
| Evaluation | Pass | WRA-RQ-112 has implementation, tests, docs, traceability, risk, state, and validation evidence. | Completion is local until external PR/CI is run. | Evaluation Engineer |
| Release | Not Applicable | No external PR or release requested for this local pipeline slice. | Release messaging must not claim M28-M36 implementation. | GitHub Maintainer Specialist |
| Community | Not Applicable | No public issue, milestone, or release was opened. | Community follow-up may be needed before publishing. | Project Coordinator |
| Retrospective | Pass | M27 confirmed the catalog-first pattern works for broad algorithm additions. | Future milestones should keep package/runtime gating explicit. | Project Coordinator |

## Acceptance Criteria

| Criterion | Status | Evidence |
|---|---|---|
| Domain errors are structured and deterministic. | Pass | Core tests reject negative square-root inputs, non-positive log inputs, invalid log/exp bases, constant normalization inputs, invalid soft-limit parameters, invalid piecewise points, and empty coefficients. |
| Unit behavior is documented for every transform. | Pass | Config reference, metadata mapping, catalog docs, and transform package compatibility docs list parameters and units. |
| Known-answer tests prove formulas. | Pass | M27 core tests cover absolute/square/square-root/log/exp/tanh/sigmoid/soft-limit/piecewise/polynomial formulas and all normalization modes. |
| Config examples cover common workflows. | Pass | `examples/m27-pointwise-waveform.csv` and `examples/m27-pointwise-config.toml` run through CLI analysis with pass/fail criteria. |
| Calibration wording stays scoped. | Pass | Docs and risk updates state M27 support is software-derived conditioning, not calibration, hardware accuracy, or certification evidence. |
| Rule-package export remains guarded. | Pass | CLI rejection matrix includes every M27 transform. |

## Validation Summary

Focused M27 checks:

```text
cargo test -p ferrisoxide-core m27 -- --nocapture
cargo test -p ferrisoxide-core transform_catalog -- --nocapture
cargo test -p ferrisoxide-core filter_config_covers_m27 -- --nocapture
cargo test -p ferrisoxide-cli analyzes_config_with_m27_pointwise_filters -- --nocapture
cargo test -p ferrisoxide-cli lists_transform_catalog -- --nocapture
cargo test -p ferrisoxide-cli rule_package_export_rejects_remaining_desktop_only_transform_matrix -- --nocapture
```

Full validation commands and results are recorded in `docs/validation-log.md`.

## Hand-Off Note

Role: Core Software Engineer / V&V Engineer / Documentation Engineer
Goal: Implement M27 pointwise, normalization, and nonlinear conditioning transforms.
Files changed: Core filters/config/catalog, CLI tests, M27 example fixture/config, config and transform docs, requirements, traceability, risk, orchestration, project state, README, CHANGELOG, validation log, and this report.
Checks run: See `docs/validation-log.md`.
Status: Complete locally.
Known gaps: M28 smoothing/detrending/baseline cleanup remains next; M29-M35 algorithm families remain planned; no package/runtime/hardware/certification support was added.
Next recommended step: Implement M28 smoothing, detrending, and baseline conditioning using the M25 catalog and M26/M27 implementation pattern.
