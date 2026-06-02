# M35 Multi-Channel, Sensor, And Domain Conditioning Pipeline Report

Date: 2026-06-02

Status: Complete locally

Milestone: M35

Related requirement: WRA-RQ-120

## Scope

M35 implements desktop `[[filters]]` for common multi-channel, software sensor-conversion, vibration, and control conditioning workflows:

- `channel_add`, `channel_subtract`, `differential_channel`, `common_mode`
- `vector_magnitude`, `euclidean_norm`, `matrix_transform`, `coordinate_rotation`
- `linear_sensor_conversion`, `pressure_transducer`, `current_shunt`, `bridge_strain`, `load_cell_force`, `rtd_temperature`, `thermistor_temperature`, `tachometer_rpm`, `encoder_position`, `accelerometer_units`, `gyroscope_rate`, `hall_current`, `lvdt_position`, `microphone_spl`, `photodiode_power`
- `velocity_from_acceleration`, `displacement_from_velocity`, `vibration_severity`
- `control_error`, `proportional_control`, `pid_control`, `rate_limiter`, `slew_rate_limit`, `control_saturation`, `control_deadzone`, `feedforward_control`

M35 also records catalog-visible dependency/design-gated entries for `phase_difference`, `gain_phase_match`, `advanced_acoustic_pack`, and `advanced_sensor_calibration_pack`.

M35 does not add live DAQ, HAL/RTOS adapters, target execution, runtime-loader implementation, rule-package/runtime exposure, external PRs, release publication, hardware calibration, hardware accuracy, hardware qualification, or certification evidence.

## Dependency Review

Decision: Pass.

FerrisOxide does not add a new calibration, acoustic-analysis, phase-estimation, or numeric dependency for M35. The implemented subset uses dependency-free software formulas, explicit configuration parameters, same-unit checks for channel math, finite-parameter validation, derived output channels, and known-answer tests.

Residual risk: software sensor conversions can be mistaken for calibration evidence. M35 mitigates this through catalog notes, config-reference wording, metadata mapping, risk-register updates, and package/runtime rejection. Advanced acoustic features, phase/gain matching, and calibration packs remain gated until dependency review, estimator/calibration conventions, fixtures, and performance evidence exist.

## Pipeline Gates

| Stage | Gate | Evidence | Residual Risk | Owner |
|---|---|---|---|---|
| Intake | Pass | User requested a comprehensive filter/signal-conditioning suite that engineers can use without external scripts for normal waveform transformations. | Scope could expand into hardware calibration or certified analysis without guardrails. | Intake Engineer |
| Project Creation | Not Applicable | Existing FerrisOxide project package and M25-M34 artifacts already exist. | No new project scaffold needed. | Project Coordinator |
| Project Orchestration | Pass | M35 follows `docs/comprehensive-filter-signal-conditioning-roadmap.md` after M34 closure. | M36 remains planned. | Project Orchestrator |
| Research | Pass | M35 roadmap, catalog contract, config/report schema, M26-M34 patterns, and sensor/domain guardrails reviewed. | Advanced acoustic, phase, and calibration workflows remain dependency/design-gated. | Domain Specialists |
| Requirements | Pass | WRA-RQ-120 updated to implemented locally with concrete transform names and advanced-gated exceptions. | WRA-RQ-121 remains planned. | Software Architect |
| Architecture | Pass | M35 appends derived channels, records output units, validates channel/unit declarations, and keeps runtime/package support rejected. | Runtime/package exposure remains unimplemented. | Software Architect |
| Abstraction Review | Pass | Transform names, config fields, metadata flags, tests, fixture files, docs, and validation commands are concrete. | Advanced calibration semantics remain outside this slice. | Abstraction Review Engineer |
| Approval Gate | Pass | User pre-approved milestone implementation and human gates for the active goal; dependency review chose no new dependency. | Future dependency additions still need explicit review. | Project Coordinator |
| Implementation | Pass | `filter.rs`, `config.rs`, `model.rs`, `transform_catalog.rs`, CLI tests, and M35 examples implement the suite. | Formula assumptions are software-only and not sensor calibration evidence. | Core Software Engineer |
| Testing | Pass | Focused M35 core/config/catalog/CLI tests and direct CLI fixture analysis cover known outputs, invalid inputs, metadata, and package rejection. | Full workspace validation recorded in `docs/validation-log.md`. | Test Automation Engineer |
| V&V | Pass | Known-answer channel math, formula checks, time-axis checks, invalid-unit/channel cases, metadata parity, and package rejection are covered. | No hardware validation or certification evidence. | V&V Engineer |
| QA | Pass | Config/catalog/metadata/corpus/roadmap/state docs updated for M35 and next M36 ownership. | Generated docs drift checks remain manual until M36. | QA Engineer |
| Security | Pass | No new dependency, no network access, no credentials, and no binary/runtime loader changes. | Future dependency review remains open for advanced domain packs. | Security Engineer |
| Performance | Not Applicable | M35 uses linear per-sample/channel formulas and avoids throughput claims. | Large multi-channel domain suites may need benchmark evidence before release claims. | Performance Engineer |
| Documentation | Pass | Config reference, transform catalog, metadata mapping, roadmap, corpus, requirements, traceability, risk, and state docs updated. | M36 closure docs remain planned. | Documentation Engineer |
| Code Review | Pass | Local review focused on derived-channel naming, unit validation, formula assumptions, metadata categories, and package/runtime rejection. | External maintainer review not requested. | Code Reviewer |
| Evaluation | Pass | M35 satisfies WRA-RQ-120 without dependency expansion and avoids calibration/hardware overclaiming. | User-facing completeness closure remains M36. | Evaluation Engineer |
| Release | Not Applicable | No external PR or release requested for this local pipeline slice. | Release messaging must not claim M36 complete. | GitHub Maintainer |
| Community | Not Applicable | No upstream/community action requested. | External issue planning remains optional. | Community Manager |
| Retrospective | Pass | M35 confirmed domain conditioning can be represented as software-derived channels with explicit assumptions. | M36 must close discoverability, corpus, compatibility, and readiness. | Project Coordinator |

## Acceptance Evidence

| Acceptance Criterion | Decision | Evidence |
|---|---|---|
| Sensor transforms are separated into domain packs with assumptions and reference formulas. | Pass | `crates/ferrisoxide-core/src/filter.rs`, `crates/ferrisoxide-core/src/config.rs`, `docs/config-reference.md`, and `docs/current-transform-metadata-mapping.md`. |
| Calibration transforms are not represented as hardware calibration evidence without separate evidence. | Pass | M35 catalog notes, config reference, risk register, metadata mapping, and package/runtime rejection. |
| Multi-channel transforms validate channel units and alignment. | Pass | Core M35 invalid-input tests cover missing channels, mismatched units, duplicate outputs, and matrix shape. |
| Domain packs can be enabled incrementally without blocking the core transform suite. | Pass | 34 M35 filters are implemented without new dependencies; `phase_difference`, `gain_phase_match`, `advanced_acoustic_pack`, and `advanced_sensor_calibration_pack` remain dependency/design-gated. |
| Engineers can run a representative M35 fixture. | Pass | `examples/m35-domain-waveform.csv` and `examples/m35-domain-config.toml` produce a passing JSON analysis with all 34 M35 transform steps. |

## Hand-Off Note

Role: Domain Specialists / Software Architect
Goal: Implement M35 multi-channel, sensor, vibration, and control conditioning after domain/calibration dependency review.
Files changed: `crates/ferrisoxide-core/src/filter.rs`, `crates/ferrisoxide-core/src/config.rs`, `crates/ferrisoxide-core/src/model.rs`, `crates/ferrisoxide-core/src/transform_catalog.rs`, `crates/ferrisoxide-cli/src/main.rs`, `examples/m35-domain-waveform.csv`, `examples/m35-domain-config.toml`, docs, requirements, traceability, risk, and state files.
Checks run: See `docs/validation-log.md`.
Status: Complete locally.
Known gaps: M36 remains planned; phase-difference estimation, gain/phase matching, advanced acoustic features, advanced sensor calibration packs, package/runtime exposure, hardware validation, and certification work remain separately gated.
Next recommended step: Complete M36 catalog, UX, compatibility, validation-corpus, benchmark, release-readiness, community, and retrospective closure.
