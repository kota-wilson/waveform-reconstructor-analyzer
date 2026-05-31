# Requirements

## Requirement Table

| ID | Requirement | Source | Priority | Acceptance Criteria | Owner | Status |
|---|---|---|---|---|---|---|
| WRA-RQ-001 | The system shall import CSV files containing time-series waveform data. | User request | High | A parser interface accepts CSV text or file input and returns typed samples or structured errors. | Core Software Engineer | MVP implemented |
| WRA-RQ-002 | The system shall map one time column and one or more signal channel columns. | User request | High | Channel mapping supports configured column names and records units. | Software Architect | MVP implemented |
| WRA-RQ-003 | The system shall reconstruct waveform objects from sample data. | User request | High | A `Waveform` model contains time axis, channels, units, source name, sample/channel counts, channel-unit metadata, sample interval summary, nominal sample rate when applicable, lineage, transform history, and sample count validation. | Core Software Engineer | Implemented |
| WRA-RQ-004 | The system shall support multiple channels. | User request | High | A waveform can contain at least two named signal channels. | Core Software Engineer | MVP implemented |
| WRA-RQ-005 | The system shall provide filter chain extension points. | User request | High | A `Filter` trait or equivalent interface can apply transformations without mutating raw data. | Systems Engineer | MVP implemented |
| WRA-RQ-006 | The MVP shall include low-pass and moving-average filter support or stubs with acceptance tests. | User request | High | Low-pass and moving-average modules have defined parameters, error handling, and tests. | Systems Engineer | MVP implemented |
| WRA-RQ-007 | The system shall define pass/fail criteria in a config shape. | User request | High | Criteria model covers min/max voltage, state transitions, pulse width, transient event, dropout, stable-state duration, and rise/fall time checks. | Software Architect | Implemented |
| WRA-RQ-008 | The system shall generate analysis results showing pass/fail status. | User request | High | Analysis output records criterion ID, pass/fail, measured value, threshold, reason, and waveform metadata context. | Core Software Engineer | MVP implemented |
| WRA-RQ-009 | The CLI shall run analysis from local input files. | User request | High | CLI accepts input path, time/channel mappings, optional filters, and min/max criteria flags. | Core Software Engineer | MVP implemented |
| WRA-RQ-010 | The project shall include tests and example data. | User request | High | Unit tests, integration-test fixture, and example CSV exist. | Test Automation Engineer | MVP implemented |
| WRA-RQ-011 | The project shall preserve raw data and make transformations derived artifacts. | Signal-processing standards | High | Filter APIs return new waveform data or transformed channel data, not destructive mutation of source fixtures. | Systems Engineer | MVP implemented |
| WRA-RQ-012 | The project shall remain open-source ready. | User request | High | README, LICENSE, CONTRIBUTING, CODE_OF_CONDUCT, SECURITY, CHANGELOG, GitHub templates, and CI exist. | GitHub Maintainer Specialist | Implemented; public repository published after license decision |
| WRA-RQ-017 | The project shall provide an embedded foundation separate from the desktop CLI path. | User request | High | A dependency-free `wra-signal` crate builds with `#![no_std]`, has fixed-size sample buffers, streaming ingestion, min/max threshold checks, transient event detection, and desktop unit tests. | Core Software Engineer | M3-RTOS-001 implemented |
| WRA-RQ-018 | The desktop analysis path shall support simulated ADC quantization before pass/fail criteria. | User request | High | Users can configure or pass an ordered ADC quantization transform with bit depth, minimum voltage, and maximum voltage; raw samples are preserved; criteria evaluate the derived quantized waveform; invalid parameters return clear errors. | Core Software Engineer | Implemented |
| WRA-RQ-019 | The project shall add known-answer signal accuracy validation before stronger engineering-analysis claims. | User review / M4-001 | High | Known-answer fixtures under `validation/known_answer/` record source method, independently calculated expected measurements, tolerance policy, analyzer command, and exact expected report artifacts. | Verification and Validation Engineer | Implemented |
| WRA-RQ-020 | The project shall validate time-axis assumptions for duration-dependent criteria. | M4-002 | High | Duration criteria reject duplicate or decreasing timestamps with clear errors; non-uniform increasing timestamps are accepted and metadata records sample interval uniformity and nominal sample rate. | Core Software Engineer | Implemented |
| WRA-RQ-021 | The project shall support explicit voltage and time tolerances for pass/fail criteria. | M4-003 | High | TOML `[tolerances]` supports `voltage_v` and `time_s`, defaults to zero, validates finite non-negative values, applies tolerances to voltage/duration decisions, and reports applied tolerance. | Core Software Engineer | Implemented |
| WRA-RQ-022 | The project shall document implemented transform equations and limitations. | M4-004 | Medium | Moving average, first-order low-pass, and ideal ADC quantization equations, edge behavior, assumptions, and limitations are documented and linked from README/architecture docs. | Systems Engineer | Implemented |
| WRA-RQ-023 | The project shall include report-level engineering evidence context. | M4-005 | High | JSON and text reports include validation profile, evidence source, tolerance policy, confidence notes, and unchanged per-criterion pass/fail evidence with documented schema migration. | Documentation Engineer | Implemented |
| WRA-RQ-024 | The project shall support optional validation metadata context. | M4-006 | Medium | Config metadata can populate optional test-run ID, acquisition notes, environment, and operator fields while preserving source, units, time-axis, sample rate, lineage, transform history, and tolerance policy. | Core Software Engineer | Implemented |
| WRA-RQ-025 | The project shall provide repeatable large-CSV benchmark evidence before performance claims. | M4-007 | Medium | A project-local generated-fixture benchmark records read, parse, transform, criteria, report, and total timings without new dependencies. | Performance Engineer | Implemented |
| WRA-RQ-026 | The project shall provide environmental-style validation examples with known outcomes. | M4-008 | High | Dropout and contact-bounce validation cases include fixture, config, expected measurement, tolerance policy, analyzer command, expected report artifact, and scope limits. | Verification and Validation Engineer | Implemented |
| WRA-RQ-027 | The desktop CLI shall support optional SVG waveform plotting with an optional third axis. | M5-001 / User request | High | A separate plotting crate renders 2D time/signal SVG plots and 3D time/signal/auxiliary-axis SVG plots from CSV input; CLI exposes `wra plot`; invalid columns and output paths return clear errors; `wra-core` and `wra-signal` remain plotting-free. | Core Software Engineer | Implemented |
| WRA-RQ-028 | The project shall provide a host-checkable ARM64 QEMU embedded demo path. | M3-RTOS-002 | Medium | `embedded/arm64/qemu/` contains a no_std demo crate that streams fixed sample data through `wra-embedded` and `wra-signal` without desktop file I/O, with documented ARM64/QEMU target assumptions and out-of-scope limits. | Embedded RTOS Engineer | Implemented |
| WRA-RQ-029 | The project shall define a no_std RTOS adapter abstraction before runtime-specific integrations. | M3-RTOS-003 | High | `crates/wra-embedded` defines sample source, event sink, and runtime hook traits plus no_std streaming helpers; `wra-signal` remains independent from RTOS APIs; host unit tests verify the adapter boundary. | Embedded RTOS Engineer | Implemented |
| WRA-RQ-030 | The project shall provide an isolated Zephyr feasibility prototype without production-readiness claims. | M3-RTOS-004 | Medium | `embedded/arm64/zephyr/` contains a feasibility sketch and documentation for toolchain assumptions, unsupported areas, and production-readiness risks; Zephyr remains optional and is not added to core, CLI, plotting, or signal crates. | Embedded RTOS Engineer / Documentation Engineer | Implemented |
| WRA-RQ-031 | The project shall provide reusable measurement primitives before expanding evidence reports and annotated SVGs. | M6-001 / issue #43 | High | A `wra-measurements` crate provides no_std, allocation-free primitives for extrema, state-transition count, state-run duration, and rise/fall time; `wra-core` criteria evaluation consumes those primitives while preserving current CLI behavior and exact JSON report output. | Core Software Engineer / V&V Engineer | Implemented |
| WRA-RQ-032 | Reports shall separate reusable measurement evidence from criteria decisions. | M6-003 / issue #45 | High | JSON and text reports include stable measurement records with method context; criterion results include `measurement_id`; measured value, unit, channel, sample index, timestamp, required value, tolerance, confidence notes, and pass/fail outcome remain auditable; exact golden reports are updated. | Core Software Engineer / Documentation Engineer | Implemented |
| WRA-RQ-033 | SVG plotting shall support 2D criteria evidence overlays. | M6-002 / issue #44 | High | `wra plot --config` renders 2D SVG overlays with pass/fail status, threshold lines, and failed-criterion labels containing sample index, timestamp, channel, measured value, and required value; plotting remains in `wra-plot` with no GUI, bitmap, web, DAQ, or embedded plotting scope. | Core Software Engineer / Documentation Engineer | Implemented in PR #52 |
| WRA-RQ-034 | The project shall document criteria DSL direction before expanding syntax. | M6-004 / issue #46 | Medium | Documentation defines measurement-backed criteria concepts, initial operator vocabulary, explicit unit fields, compatibility expectations for existing `[[criteria]]`, and non-goals. | Software Architect / Documentation Engineer | Implemented in PR #52 |
| WRA-RQ-035 | The project shall add measurement-engine known-answer validation fixtures. | M6-005 / issue #47 | High | Validation fixtures independently document expected values for state transition count, pulse width, transient duration, stable-state duration, rise time, fall time, tolerance expectations, and time-axis assumptions, with exact JSON report comparison. | Verification and Validation Engineer | Implemented in PR #52 |

## Assumptions

| ID | Assumption | Impact | Owner | Validation Plan |
|---|---|---|---|---|
| WRA-A-001 | Time unit defaults to seconds and voltage unit defaults to volts. | Affects analysis criteria and docs. | Software Architect | Validate with example fixture and docs review. |
| WRA-A-002 | Initial parser could avoid third-party crates until dependency review. | Dependency review later approved `csv`, `serde`, `serde_json`, and `toml`. | Core Software Engineer | Preserve `Cargo.lock` visibility and review any future dependency expansion. |
| WRA-A-003 | CLI-first MVP is enough before GUI planning. | Keeps scope reviewable. | Project Coordinator | Evaluate after M1 and M2 milestones. |
| WRA-A-004 | MIT license is acceptable for initial scaffold. | Confirmed before public repository publication. | Project Coordinator | Keep `LICENSE` and `decisions/ADR-002-license-assumption.md` aligned. |

## Verification Plan

| Requirement | Verification Method | Planned Evidence |
|---|---|---|
| WRA-RQ-001 | Unit and fixture tests | CSV parser tests for valid, missing, malformed, and empty input. |
| WRA-RQ-002 | Unit tests | Channel mapping tests with named columns and unit metadata. |
| WRA-RQ-003 | Unit and golden tests | Waveform construction, metadata, source/lineage, transform history, and JSON report metadata tests. |
| WRA-RQ-004 | Unit tests | Multi-channel fixture test. |
| WRA-RQ-005 | Unit tests | Filter chain applies transformations to derived data. |
| WRA-RQ-006 | Synthetic signal tests | Step/noise fixture for low-pass and moving-average behavior. |
| WRA-RQ-007 | Config model tests | TOML config tests for min/max, transitions, pulse width, transient event, dropout, stable-state, and rise/fall criteria. |
| WRA-RQ-008 | Analysis tests | Pass/fail result table with expected measured values. |
| WRA-RQ-009 | Unit and smoke tests | CLI fixture command with explicit arguments. |
| WRA-RQ-010 | File inspection and tests | Example CSV and test fixture usage. |
| WRA-RQ-011 | Code review | Raw data fixtures untouched; APIs return derived outputs. |
| WRA-RQ-012 | File inspection and CI | Repository metadata and workflow files exist. |
| WRA-RQ-017 | Unit tests and workspace checks | `wra-signal` unit tests, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo tree -p wra-signal`. |
| WRA-RQ-018 | Unit, CLI, and config tests | ADC quantizer unit tests, filter-chain ordering tests, config conversion tests, invalid config tests, CLI pre-criteria analysis test, and workspace checks. |
| WRA-RQ-019 | Known-answer validation tests | `validation_known_answer_square_wave_matches_expected_report`, expected measurement docs, and `validation/reports/square_wave_tolerance.json`. |
| WRA-RQ-020 | Unit tests | Duplicate/decreasing time-axis rejection tests, non-uniform increasing time-axis test, and sample interval metadata test. |
| WRA-RQ-021 | Unit, config, and golden tests | Tolerance pass/fail unit tests, invalid tolerance config test, CLI invalid config test, and expected validation report tolerance fields. |
| WRA-RQ-022 | Documentation and unit tests | `docs/filter-behavior.md` plus filter unit tests named in the document. |
| WRA-RQ-023 | Golden tests and schema docs | Golden JSON tests and `docs/report-schema.md` include `evidence_context` and `tolerance_used`. |
| WRA-RQ-024 | Unit and report tests | Metadata context unit test and validation reports with populated metadata fields. |
| WRA-RQ-025 | Benchmark command | `sh scripts/benchmark-large-csv.sh 100000 3` and `docs/benchmarking.md`. |
| WRA-RQ-026 | Validation report tests | Environmental dropout/contact-bounce expected report tests and `docs/environmental-test-use-cases.md`. |
| WRA-RQ-027 | Unit, CLI, and smoke tests | `wra-plot` SVG render tests, `wra-cli` 2D/3D plot tests, missing third-axis test, invalid output path test, and CLI smoke commands. |
| WRA-RQ-028 | Demo manifest tests and docs review | `cargo test --manifest-path embedded/arm64/qemu/Cargo.toml`, QEMU README target assumptions, and no desktop I/O inspection. |
| WRA-RQ-029 | Unit and dependency tests | `wra-embedded` unit tests, workspace tests, clippy, and `cargo tree -p wra-embedded`. |
| WRA-RQ-030 | Prototype and documentation review | Zephyr feasibility sketch, README risk section, and file inspection showing no Zephyr dependency or workspace integration. |
| WRA-RQ-031 | Unit, golden, and workspace tests | `wra-measurements` unit tests, existing `wra-core` criteria tests, exact golden JSON report tests, workspace tests, clippy, and dependency tree inspection. |
| WRA-RQ-032 | Unit, CLI, golden, and schema tests | Measurement-link unit test, report renderer tests, CLI output tests, exact golden JSON reports, report-schema documentation, workspace tests, clippy, and diff check. |
| WRA-RQ-033 | SVG unit, CLI, and smoke tests | `wra-plot` overlay SVG test, `wra-cli` annotated plot test, CLI smoke command with `--config`, plotting docs, workspace tests, clippy, and diff check. |
| WRA-RQ-034 | Documentation review | `docs/criteria-dsl.md`, README/doc links, workspace tests, fmt check, and diff check. |
| WRA-RQ-035 | Known-answer validation tests | `validation/measurement_engine/`, `expected-measurements.md`, exact report `measurement_engine_known_answer.json`, workspace tests, clippy, and diff check. |

## Rules

- Requirements must be verifiable.
- Requirements must have an owner.
- Acceptance criteria must be specific enough for V&V.
- If scope changes, update `traceability-matrix.md`.
