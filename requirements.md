# Requirements

## Requirement Table

| ID | Requirement | Source | Priority | Acceptance Criteria | Owner | Status |
|---|---|---|---|---|---|---|
| WRA-RQ-001 | The system shall import CSV files containing time-series waveform data. | User request | High | A parser interface accepts CSV text or file input and returns typed samples or structured errors. | Core Software Engineer | MVP implemented |
| WRA-RQ-002 | The system shall map one time column and one or more signal channel columns. | User request | High | Channel mapping supports configured column names and records units. | Software Architect | MVP implemented |
| WRA-RQ-003 | The system shall reconstruct waveform objects from sample data. | User request | High | A `Waveform` model contains time axis, channels, units, and sample count validation. | Core Software Engineer | MVP implemented |
| WRA-RQ-004 | The system shall support multiple channels. | User request | High | A waveform can contain at least two named signal channels. | Core Software Engineer | MVP implemented |
| WRA-RQ-005 | The system shall provide filter chain extension points. | User request | High | A `Filter` trait or equivalent interface can apply transformations without mutating raw data. | Systems Engineer | MVP implemented |
| WRA-RQ-006 | The MVP shall include low-pass and moving-average filter support or stubs with acceptance tests. | User request | High | Low-pass and moving-average modules have defined parameters, error handling, and tests. | Systems Engineer | MVP implemented |
| WRA-RQ-007 | The system shall define pass/fail criteria in a config shape. | User request | High | Criteria model covers min/max voltage and leaves room for pulse, transient event, and dropout checks. | Software Architect | Partial: CLI min/max criteria implemented; config shape deferred |
| WRA-RQ-008 | The system shall generate analysis results showing pass/fail status. | User request | High | Analysis output records criterion ID, pass/fail, measured value, threshold, and reason. | Core Software Engineer | MVP implemented |
| WRA-RQ-009 | The CLI shall run analysis from local input files. | User request | High | CLI accepts input path, time/channel mappings, optional filters, and min/max criteria flags. | Core Software Engineer | MVP implemented |
| WRA-RQ-010 | The project shall include tests and example data. | User request | High | Unit tests, integration-test fixture, and example CSV exist. | Test Automation Engineer | MVP implemented |
| WRA-RQ-011 | The project shall preserve raw data and make transformations derived artifacts. | Signal-processing standards | High | Filter APIs return new waveform data or transformed channel data, not destructive mutation of source fixtures. | Systems Engineer | MVP implemented |
| WRA-RQ-012 | The project shall remain open-source ready. | User request | High | README, LICENSE, CONTRIBUTING, CODE_OF_CONDUCT, SECURITY, CHANGELOG, GitHub templates, and CI exist. | GitHub Maintainer Specialist | Implemented; external publication blocked on license confirmation |
| WRA-RQ-017 | The project shall provide an embedded foundation separate from the desktop CLI path. | User request | High | A dependency-free `wra-signal` crate builds with `#![no_std]`, has fixed-size sample buffers, streaming ingestion, min/max threshold checks, transient event detection, and desktop unit tests. | Core Software Engineer | M3-RTOS-001 implemented |
| WRA-RQ-018 | The desktop analysis path shall support simulated ADC quantization before pass/fail criteria. | User request | High | Users can configure or pass an ordered ADC quantization transform with bit depth, minimum voltage, and maximum voltage; raw samples are preserved; criteria evaluate the derived quantized waveform; invalid parameters return clear errors. | Core Software Engineer | Implemented |

## Assumptions

| ID | Assumption | Impact | Owner | Validation Plan |
|---|---|---|---|---|
| WRA-A-001 | Time unit defaults to seconds and voltage unit defaults to volts. | Affects analysis criteria and docs. | Software Architect | Validate with example fixture and docs review. |
| WRA-A-002 | Initial parser can avoid third-party crates until dependency review. | Slower CSV dialect support, lower dependency risk. | Core Software Engineer | Add parser tests and revisit dependency decision. |
| WRA-A-003 | CLI-first MVP is enough before GUI planning. | Keeps scope reviewable. | Project Coordinator | Evaluate after M1 and M2 milestones. |
| WRA-A-004 | MIT license is acceptable for initial scaffold. | Must be confirmed before publication. | Project Coordinator | Record license decision before external release. |

## Verification Plan

| Requirement | Verification Method | Planned Evidence |
|---|---|---|
| WRA-RQ-001 | Unit and fixture tests | CSV parser tests for valid, missing, malformed, and empty input. |
| WRA-RQ-002 | Unit tests | Channel mapping tests with named columns and unit metadata. |
| WRA-RQ-003 | Unit tests | Waveform construction tests for aligned and misaligned lengths. |
| WRA-RQ-004 | Unit tests | Multi-channel fixture test. |
| WRA-RQ-005 | Unit tests | Filter chain applies transformations to derived data. |
| WRA-RQ-006 | Synthetic signal tests | Step/noise fixture for low-pass and moving-average behavior. |
| WRA-RQ-007 | Config model tests | Criteria deserialization or parser tests after config format decision. |
| WRA-RQ-008 | Analysis tests | Pass/fail result table with expected measured values. |
| WRA-RQ-009 | Unit and smoke tests | CLI fixture command with explicit arguments. |
| WRA-RQ-010 | File inspection and tests | Example CSV and test fixture usage. |
| WRA-RQ-011 | Code review | Raw data fixtures untouched; APIs return derived outputs. |
| WRA-RQ-012 | File inspection and CI | Repository metadata and workflow files exist. |
| WRA-RQ-017 | Unit tests and workspace checks | `wra-signal` unit tests, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo tree -p wra-signal`. |
| WRA-RQ-018 | Unit, CLI, and config tests | ADC quantizer unit tests, filter-chain ordering tests, config conversion tests, invalid config tests, CLI pre-criteria analysis test, and workspace checks. |

## Rules

- Requirements must be verifiable.
- Requirements must have an owner.
- Acceptance criteria must be specific enough for V&V.
- If scope changes, update `traceability-matrix.md`.
