# Code Review

Date: 2026-05-31

Owner Role: Code Reviewer

## Current Status

This is the initial publication code review record. Later feature PRs have their own PR review and CI evidence; current repository state is summarized in `project-state.md`.

## Findings

No blocking code-review findings.

## M4 Signal Accuracy And Validation Update

No blocking code-review findings for M4. Review notes:

- Tolerance and metadata changes reuse existing `model`, `config`, `analysis`, and `report` boundaries.
- Time-axis validation is scoped to duration-dependent criteria.
- Validation fixtures are small, explicit, and paired with expected measurement notes and exact JSON reports.
- Benchmarking uses project-local Cargo tooling and adds no dependency surface.

## M5 SVG Plotting Update

No blocking code-review findings for M5. Review notes:

- Plotting is isolated in `ferrisoxide-plot`; `ferrisoxide-core` and `ferrisoxide-signal` do not import Plotters.
- `ferrisoxide-cli` exposes plotting as a separate `plot` subcommand, preserving the existing `analyze` path.
- Error handling returns structured plot errors and CLI strings instead of panics for missing channels, invalid dimensions, missing z-columns, and missing output parent directories.
- Tests cover direct SVG rendering, CLI file output, optional third-axis behavior, and negative paths.
- Dependency scope is constrained to Plotters SVG line rendering.

## M3 RTOS Adapter And Prototype Update

No blocking code-review findings for the M3 RTOS follow-up branch. Review notes:

- `ferrisoxide-embedded` is isolated from `ferrisoxide-core`, `ferrisoxide-cli`, and `ferrisoxide-plot`; it depends only on `ferrisoxide-signal`.
- Adapter traits keep source, sink, and runtime concerns explicit without binding to a specific RTOS API.
- QEMU proof code is host-checkable, fixed-data, and no_std.
- Zephyr prototype is intentionally a non-built feasibility sketch with no SDK, HAL, unsafe FFI, or workspace dependency.
- Tests cover threshold streaming, transient-event streaming, empty input, non-monotonic timestamps, and the QEMU proof outcome.

## M6 Measurement Engine Update

No blocking code-review findings for the M6 measurement-engine extraction. Review notes:

- `ferrisoxide-measurements` is isolated from CSV parsing, TOML config, plotting, reporting, file I/O, DAQ, RTOS SDKs, and plugin runtime concerns.
- `ferrisoxide-core` criteria evaluation now calls reusable measurement primitives while continuing to own pass/fail policy, tolerance application, and evidence wording.
- `SignalState` and `EdgeDirection` are re-exported through `ferrisoxide_core::criteria`, preserving the existing caller path.
- Golden JSON reports pass unchanged, which protects evidence values and tie behavior.
- No new third-party dependency or unsafe Rust is added.

## M6-003 Report Measurement Schema Update

No blocking code-review findings for the M6-003 report measurement schema branch. Review notes:

- Existing `evaluate_criteria` and `evaluate_criteria_with_tolerances` APIs remain available for callers that only need results.
- New `evaluate_criteria_with_measurements` returns both measurement records and criteria results without duplicating criteria scans.
- `AnalysisReport` owns a `measurements` vector and renders it before criteria results in text and JSON.
- `AnalysisResult.measurement_id` gives a stable report-local link back to measurement evidence.
- Exact golden JSON reports were intentionally updated and continue to compare output exactly.

## M6 Completion Update

No blocking code-review findings for the M6 completion branch. Review notes:

- `EvidenceOverlay` is derived from existing `AnalysisResult` and `MeasurementRecord` values.
- `ferrisoxide-signal plot --config` reuses the same config/filter/criteria evaluation path as analysis before rendering annotations.
- The criteria DSL work is documentation-only and does not alter runtime TOML parsing.
- The measurement-engine fixture is exact-report tested and documents expected values independently.
- No new third-party dependency or unsafe Rust is added.

## Review Notes

| Area | Result |
|---|---|
| Error handling | Pass: malformed user input returns errors instead of panics in the reviewed paths. |
| Raw data preservation | Pass: filters return derived waveforms. |
| Dependency scope | Pass: dependency additions match approved review. |
| CLI scope | Pass: config and explicit flags are narrow and understandable. |
| Tests | Pass for MVP: unit, fixture, and smoke paths exist. |
| M4 validation | Pass: known-answer, environmental, tolerance, time-axis, report, and benchmark evidence exist. |
| M5 plotting | Pass: optional SVG plotting is isolated, tested, and documented. |
| M3 RTOS follow-up | Pass: adapter/prototype work is isolated, no_std, tested, and documented. |
| M6 measurement extraction | Pass: measurement primitives are isolated, tested, and report-compatible. |
| M6-003 report schema | Pass: measurement records and result links are tested, documented, and golden-protected. |
| M6 completion | Pass: overlays, DSL docs, and measurement fixtures are tested, documented, and scope-controlled. |

## Gate Decision

- Gate: Code Review Gate.
- Decision: Pass.
- Reason: No blocking defects found and validation is green for the current MVP, M4 validation work, M5 plotting slice, M3 RTOS adapter/prototype slice, M6 measurement extraction slice, M6-003 report schema branch, and M6 completion branch.
- Residual risk: CLI parsing is still hand-rolled; a future CLI parser crate could improve UX after review. Downstream schema migration feedback, visual regression, target execution, and RTOS SDK validation are not yet automated.
- Next owner: Evaluation Engineer.

## Hand-Off Note

Role: Code Reviewer
Goal: Review MVP code for the initial public publication gate.
Files changed: `docs/code-review.md`
Checks run: Code inspection plus validation evidence review.
Status: Pass.
Known gaps: More negative-path tests, downstream schema migration feedback, visual regression tests, target execution tests, RTOS SDK validation, and CLI UX polish are future work.
Next recommended step: Evaluation.
