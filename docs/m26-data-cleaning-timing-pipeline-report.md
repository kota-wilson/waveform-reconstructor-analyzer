# M26 Data Cleaning And Timing Conditioning Pipeline Report

Date: 2026-06-02

Status: Complete locally for desktop `[[filters]]` data-cleaning and timing-conditioning transforms. No GitHub issue, external PR, release tag, dependency addition, live DAQ, HAL/RTOS, runtime-loader implementation, target hardware execution, hardware qualification, or certification evidence was added.

## Scope

M26 implements the first post-catalog transform expansion using the M25 source-of-truth catalog contract.

In scope:

- `timestamp_sort` for stable ascending timestamp repair.
- `dedupe_timestamps` with `keep_first` policy.
- `nan_interpolate` with time-based linear interpolation and endpoint hold.
- `nan_remove` for row removal when any channel contains NaN.
- `crop` for inclusive time-window selection.
- `fixed_delay` for timestamp shifting.
- `gap_fill` for fixed-grid linear interpolation.
- `resample_fixed` for fixed-rate normalization.
- `channel_delay` for offline single-channel delay alignment.
- TOML config support, CLI fixture coverage, catalog metadata tests, docs, traceability, and risk updates.

Out of scope:

- `split_by_event`; deferred to M36 because it creates multiple segment artifacts rather than one derived waveform.
- Spike removal and Hampel filtering; deferred to M28 smoothing/baseline cleanup.
- Anti-aliasing, rational resampling, clock-drift correction, and fractional delay; remain M30 work.
- Rule-package export support for M26 transforms.
- New dependencies, runtime loaders, hardware, live DAQ, HAL/RTOS, signing, or certification evidence.

## Files

| Area | Evidence |
|---|---|
| Core transforms | `crates/ferrisoxide-core/src/filter.rs` |
| Config parsing | `crates/ferrisoxide-core/src/config.rs` |
| Catalog | `crates/ferrisoxide-core/src/transform_catalog.rs` |
| CLI tests/package guard | `crates/ferrisoxide-cli/src/main.rs` |
| Fixture/example | `examples/m26-data-cleaning-waveform.csv`, `examples/m26-data-cleaning-config.toml` |
| Docs | `docs/config-reference.md`, `docs/current-transform-metadata-mapping.md`, `docs/transform-catalog.md`, `docs/transform-package-compatibility.md` |
| Governance | `requirements.md`, `traceability-matrix.md`, `risk-register.md`, `orchestration-plan.md`, `project-state.md`, README, CHANGELOG |

## Gate Decisions

| Stage | Gate Decision | Evidence | Residual Risk | Owner |
|---|---|---|---|---|
| Intake | Pass | User requested the next comprehensive filter milestone path and pre-approved human gates for the active goal. | Scope must remain sampled-waveform software conditioning. | Project Coordinator |
| Research | Pass | Existing filter/config/catalog/report code, M25 catalog, and M26 roadmap scope reviewed. | Advanced DSP and dependency choices remain future milestones. | Open Source Research Engineer |
| Requirements | Pass | WRA-RQ-111 updated to implemented locally with concrete transform names and `split_by_event` deferral. | Later segmentation work needs a multi-artifact design. | Software Architect |
| Architecture | Pass | M26 uses existing `Filter`/`FilterStep`, `Waveform` derived lineage, `TransformStepMetadata`, `TransformCatalogEntry`, and TOML config surfaces. | Some timing transforms are desktop-only until runtime/package semantics exist. | Software Architect |
| Abstraction Review | Pass | Transform names, parameters, categories, timing behavior, package support, output kind, and evidence level are explicit in catalog entries. | `split_by_event` cannot fit the current single-derived-waveform abstraction. | Abstraction Review Engineer |
| Human Approval | Pass | User pre-approved all human gates for this active goal on 2026-06-02. | Approval does not approve dependencies, hardware, release, or certification scope. | User / Project Coordinator |
| Issue Planning | Not Applicable | Local pipeline execution continues without GitHub issue creation by instruction/context. | External tracking may still be useful before public PR work. | GitHub Maintainer Specialist |
| Implementation | Pass | Core transforms, config parser support, catalog entries, CLI tests, and fixture examples implemented. | None for the scoped desktop transforms. | Core Software Engineer |
| Testing | Pass | Focused M26 core/config/catalog and CLI tests passed; full validation is recorded in `docs/validation-log.md`. | None after local validation. | Test Automation Engineer |
| V&V | Pass | Known-answer tests cover NaN repair/removal, timestamp sort/dedupe, crop/delay, gap fill/resampling, channel delay, invalid input rejection, and CLI evidence. | Fixture coverage is software-only and not hardware acquisition validation. | V&V Engineer |
| QA | Pass | Formatting, workspace tests, clippy, diff checks, whitespace scan, and Markdown link scan are recorded in `docs/validation-log.md`. | Automated config-doc drift checks remain future tooling. | QA Engineer |
| Security | Not Applicable | No new dependencies, network behavior, credentials, signing, auth, unsafe FFI, SDKs, or permission changes. | Future dependency-gated transforms still need security review. | Security Engineer |
| Performance | Not Applicable | No benchmark or throughput claim added; fixed-grid output is bounded by an explicit maximum sample guard. | Large-waveform performance benchmarks remain future work. | Performance Engineer |
| Documentation | Pass | Config reference, metadata mapping, transform catalog, package compatibility, roadmap, README, and changelog updated. | Future docs must keep M26 package/runtime rejection visible. | Documentation Engineer |
| Code Review | Pass | Local review checked raw preservation, time-axis behavior, non-finite handling, package rejection, and catalog metadata parity. | External maintainer review remains future PR work. | Code Reviewer |
| Evaluation | Pass | WRA-RQ-111 has implementation, tests, docs, traceability, risk, state, and validation evidence. | Completion is local until external PR/CI is run. | Evaluation Engineer |
| Release | Not Applicable | No external PR or release requested for this local pipeline slice. | Release messaging must not claim M27-M36 implementation. | GitHub Maintainer Specialist |
| Community | Not Applicable | No public issue, milestone, or release was opened. | Community follow-up may be needed before publishing. | Project Coordinator |
| Retrospective | Pass | M26 showed the M25 catalog catches metadata/package drift during implementation. | Future milestones should add catalog rows before exposing new config types. | Project Coordinator |

## Acceptance Criteria

| Criterion | Status | Evidence |
|---|---|---|
| Invalid data handling is explicit and reported. | Pass | Core tests reject all-NaN interpolation channels, invalid time windows, missing channels, zero sample intervals, non-finite data, and empty crops. |
| Raw data remains preserved. | Pass | M26 transforms create derived waveforms through the existing lineage path; tests assert source samples remain unchanged. |
| Cleaning actions record lineage. | Pass | `transform_history` and structured `transform_steps` are emitted for every M26 transform. |
| Known-answer fixtures cover messy DAQ exports. | Pass | `examples/m26-data-cleaning-waveform.csv` plus CLI smoke covers unordered timestamps, duplicate timestamps, NaN repair, gaps, and criteria evaluation after cleaning. |
| Rule-package export remains guarded. | Pass | CLI rejection matrix includes every M26 transform. |
| Multi-artifact segmentation is not overclaimed. | Pass | `split_by_event` is cataloged as planned/deferred to M36 with reason and residual risk. |

## Validation Summary

Focused M26 checks:

```text
cargo test -p ferrisoxide-core m26 -- --nocapture
cargo test -p ferrisoxide-core transform_catalog -- --nocapture
cargo test -p ferrisoxide-core filter_config_covers_m26 -- --nocapture
cargo test -p ferrisoxide-cli analyzes_config_with_m26_data_cleaning_filters -- --nocapture
cargo test -p ferrisoxide-cli lists_transform_catalog -- --nocapture
cargo test -p ferrisoxide-cli rule_package_export_rejects_remaining_desktop_only_transform_matrix -- --nocapture
```

Full validation commands and results are recorded in `docs/validation-log.md`.

## Hand-Off Note

Role: Core Software Engineer / V&V Engineer / Documentation Engineer
Goal: Implement M26 data-cleaning and timing-conditioning transforms.
Files changed: Core filters/config/catalog, CLI tests, M26 example fixture/config, config and transform docs, requirements, traceability, risk, orchestration, project state, README, CHANGELOG, validation log, and this report.
Checks run: See `docs/validation-log.md`.
Status: Complete locally.
Known gaps: `split_by_event` remains deferred to M36; spike/Hampel cleanup remains deferred to M28; advanced resampling/timing remains M30; no package/runtime/hardware/certification support was added.
Next recommended step: Implement M27 pointwise, normalization, and nonlinear conditioning using the M25 catalog and M26 implementation pattern.
