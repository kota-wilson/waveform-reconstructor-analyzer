# Validation Log

Date: 2026-05-30

Updated: 2026-05-31

Project: Waveform Reconstructor and Analyzer

Stage: Validation audit trail

Owner Role: Test Automation Engineer

## Reading This Log

This file is an audit trail. The newest validation snapshot is listed first, and older sections preserve point-in-time command evidence from earlier PRs. Historical test counts are intentionally not rewritten unless the original entry was wrong at the time it was recorded.

## Environment

- Working directory: `/Users/kota/Desktop/softwareai/projects/waveform-reconstructor-analyzer`
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`
- Rust: `rustc 1.95.0 (59807616e 2026-04-14)`
- Dependencies: `csv`, `serde`, `serde_json`, `toml`, `plotters`; resolved versions are pinned in `Cargo.lock`.

## M3 RTOS Adapter And Prototype Branch

Current as of the M3 RTOS adapter/prototype branch on 2026-05-31.

| Command | Result | Notes |
|---|---|---|
| `cargo fmt` | Passed | Rust sources formatted after adding `wra-embedded` and the QEMU proof crate. |
| `cargo test --workspace` | Passed | 75 tests passed: 9 CLI, 38 core, 9 criteria-engine fixture/golden/validation tests, 1 CSV fixture integration test, 4 `wra-embedded`, 5 `wra-plot`, 9 `wra-signal`, plus doctests. |
| `cargo test --manifest-path embedded/arm64/qemu/Cargo.toml` | Passed | 1 QEMU proof-slice test passed, exercising the no_std threshold path through `wra-embedded` and `wra-signal`. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings after factoring the embedded stream result type. |
| `cargo tree -p wra-embedded` | Passed | Dependency tree is limited to `wra-embedded` -> `wra-signal`; no external crates added. |
| `cargo fmt --check` | Passed | Formatting remained clean after documentation updates. |
| `git diff --check` | Passed | No whitespace errors in the branch diff. |

### Exact Tests Added

| Test | Coverage |
|---|---|
| `wra_embedded::tests::threshold_stream_uses_source_runtime_sink_and_signal_core` | Verifies sample source, runtime hooks, event sink, and threshold primitive integration. |
| `wra_embedded::tests::transient_event_stream_records_longest_event` | Verifies transient-event streaming through the adapter boundary. |
| `wra_embedded::tests::empty_threshold_stream_returns_signal_error_without_sink_record` | Empty source returns `SignalError::EmptyInput` and does not record sink evidence. |
| `wra_embedded::tests::non_monotonic_stream_propagates_signal_error` | Non-monotonic timestamps propagate as signal errors. |
| `wra_arm64_qemu_demo::tests::qemu_demo_exercises_no_std_threshold_path` | Host-checkable QEMU proof slice uses fixed samples and no desktop file I/O. |

### Gate Decision

- Gate: Testing Gate for M3-RTOS-002 through M3-RTOS-004.
- Decision: Pass.
- Reason: Workspace tests, standalone QEMU demo test, clippy, and dependency inspection pass without adding external dependencies, target toolchains, RTOS SDKs, or desktop path coupling.
- Residual risk: This is host-checkable software evidence, not an ARM64 target execution, QEMU image boot, Zephyr build, hardware qualification, RTOS readiness, or certification artifact.
- Owner for residual risk: Embedded RTOS Engineer / Verification and Validation Engineer.

### Hand-Off Note

Role: Test Automation Engineer
Goal: Validate M3 RTOS adapter, ARM64 QEMU proof slice, and Zephyr feasibility prototype.
Files changed: `docs/validation-log.md`
Checks run: `cargo fmt`; `cargo test --workspace`; `cargo test --manifest-path embedded/arm64/qemu/Cargo.toml`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo tree -p wra-embedded`; `cargo fmt --check`; `git diff --check`.
Status: Pass.
Known gaps: No ARM64 target build, QEMU boot image, Zephyr SDK build, hardware HAL, unsafe FFI review, or RTOS timing validation.
Next recommended step: V&V and protected-branch PR review.

## M5 SVG Plotting Branch

Current as of the M5 plotting branch on 2026-05-31.

| Command | Result | Notes |
|---|---|---|
| `cargo fmt` | Passed | Rust sources formatted after adding `wra-plot` and CLI plotting tests. |
| `cargo test --workspace` | Passed | 71 tests passed: 9 CLI, 38 core, 9 criteria-engine fixture/golden/validation tests, 1 CSV fixture integration test, 5 `wra-plot`, 9 `wra-signal`, plus doctests. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |
| `cargo run --quiet --bin wra -- plot --input examples/basic-waveform.csv --time-column time --channels input_v,output_v --output /private/tmp/wra-plot-2d.svg` | Passed | Wrote a 21,670 byte SVG containing `<svg`, `input_v`, and `output_v`. |
| `cargo run --quiet --bin wra -- plot --input tests/fixtures/plot_three_axis.csv --time-column time_s --channels signal_v --z-column temperature_c --output /private/tmp/wra-plot-3d.svg --title "Three Axis Validation Plot"` | Passed | Wrote a 21,782 byte SVG containing `<svg`, `Three Axis Validation Plot`, and `signal_v vs temperature_c`. |
| `cargo fmt --check` | Passed | Formatting remained clean after validation commands. |
| `git diff --check` | Passed | No whitespace errors in the branch diff. |
| `cargo metadata --format-version 1 --no-deps` | Passed | Confirms `wra-plot` depends on `plotters` with `default-features = false` and features `svg_backend`, `line_series`. |
| `cargo tree -p wra-plot` | Passed | Native plotting dependency tree is limited to `plotters`, `plotters-backend`, `plotters-svg`, `num-traits`, `autocfg`, and existing `wra-core` dependencies. |

### Exact Tests Added

| Test | Coverage |
|---|---|
| `wra_plot::tests::renders_2d_svg_for_selected_channel` | SVG string renderer includes the selected 2D channel and default title. |
| `wra_plot::tests::renders_3d_svg_with_third_axis_channel` | SVG string renderer includes a 3D line-series label using the auxiliary axis channel. |
| `wra_plot::tests::rejects_missing_plot_channel` | Missing plotted channels produce a structured `PlotError::MissingChannel`. |
| `wra_plot::tests::rejects_z_channel_reuse_as_plot_channel` | Third-axis channel must be separate from plotted channels. |
| `wra_plot::tests::rejects_output_path_with_missing_parent_directory` | Missing output parent directories return clear errors before rendering. |
| `wra_cli::tests::renders_2d_plot_to_svg_file` | `wra plot` renders a local 2D SVG file from the basic example CSV. |
| `wra_cli::tests::renders_3d_plot_with_z_column_to_svg_file` | `wra plot --z-column` renders a local 3D SVG file from the three-axis fixture. |
| `wra_cli::tests::plot_reports_missing_z_column` | Missing auxiliary axis columns fail with a clear parser error. |

### Gate Decision

- Gate: Testing Gate for M5.
- Decision: Pass.
- Reason: Formatting, workspace tests, clippy, CLI smoke plots, dependency metadata/tree inspection, and whitespace checks passed.
- Residual risk: The validation proves SVG generation from fixtures, not visual perceptual quality, GUI behavior, live data acquisition, or certification suitability.
- Owner for residual risk: Test Automation Engineer / Documentation Engineer.

### Hand-Off Note

Role: Test Automation Engineer
Goal: Validate optional desktop SVG plotting with an optional third axis.
Files changed: `docs/validation-log.md`
Checks run: `cargo fmt`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; 2D/3D `wra plot` smoke commands; `cargo fmt --check`; `git diff --check`; `cargo metadata --format-version 1 --no-deps`; `cargo tree -p wra-plot`.
Status: Pass.
Known gaps: No visual regression testing or browser/SVG raster comparison yet.
Next recommended step: V&V and protected-branch PR review.

## M4 Signal Accuracy And Validation Branch

Current as of the M4 signal-validation branch on 2026-05-31.

| Command | Result | Notes |
|---|---|---|
| `cargo fmt --check` | Passed | Rust formatting clean. |
| `cargo test --workspace` | Passed | 63 tests passed: 6 CLI, 38 core, 9 criteria-engine fixture/golden/validation tests, 1 CSV fixture integration test, 9 `wra-signal`, plus doctests. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |
| `git diff --check` | Passed | No whitespace errors in the branch diff. |
| `cargo run --quiet --bin wra -- analyze --input validation/known_answer/square_wave_tolerance.csv --config validation/known_answer/square_wave_tolerance.toml --format json` | Passed | Known-answer tolerance case produced the expected pass report with metadata, tolerance policy, and evidence context. |
| `cargo run --quiet --bin wra -- analyze --input validation/environmental_cases/dropout_event.csv --config validation/environmental_cases/dropout_event.toml --format json` | Passed | Dropout validation case produced the expected fail report with 2 ms dropout evidence. |
| `cargo run --quiet --bin wra -- analyze --input examples/basic-waveform.csv --config tests/configs/invalid-negative-tolerance.toml --format json` | Passed | Command exited with code 2 and clear error: `invalid config tolerances: invalid parameter \`tolerances.time_s\`: must be greater than or equal to zero`. |
| `sh scripts/benchmark-large-csv.sh 100000 3` | Passed | Generated a 100k-sample CSV under `target/wra-benchmark/` and recorded read, parse, transform, criteria, report, and total timing averages in `docs/benchmarking.md`. |

### Exact Tests Added

| Test | Coverage |
|---|---|
| `analysis::tests::applies_voltage_and_time_tolerances` | Pass-at-boundary voltage and duration tolerance behavior. |
| `analysis::tests::still_fails_beyond_configured_tolerance` | Fail-beyond-tolerance voltage behavior. |
| `analysis::tests::rejects_duplicate_or_decreasing_time_for_duration_criteria` | Duplicate and decreasing timestamps return structured errors before duration criteria evaluation. |
| `analysis::tests::allows_non_uniform_but_increasing_time_axis` | Non-uniform increasing timestamps are accepted and measured using actual sample times. |
| `config::tests::rejects_invalid_tolerance_config` | Invalid TOML tolerance values are rejected without panics. |
| `model::tests::stores_optional_validation_context_and_tolerances` | Optional metadata context and tolerance policy are preserved in waveform metadata. |
| `validation_known_answer_square_wave_matches_expected_report` | Known-answer square-wave tolerance fixture matches exact JSON report. |
| `validation_dropout_environmental_case_matches_expected_report` | Environmental dropout fixture matches exact JSON report. |
| `validation_contact_bounce_environmental_case_matches_expected_report` | Environmental contact-bounce fixture matches exact JSON report. |

### Benchmark Snapshot

```text
wra_benchmark
input=target/wra-benchmark/large_square_wave_100000.csv
config=target/wra-benchmark/large_square_wave_100000.toml
iterations=3
samples=100000
channels=1
report_bytes=2395
read_ms_avg=0.316
parse_ms_avg=157.890
transform_ms_avg=5.725
criteria_ms_avg=5.084
report_ms_avg=0.070
total_ms_avg=169.084
```

### Gate Decision

- Gate: Testing Gate for M4.
- Decision: Pass.
- Reason: Formatting, workspace tests, clippy, whitespace check, known-answer CLI smoke, environmental validation CLI smoke, invalid tolerance error check, and repeatable benchmark command passed.
- Residual risk: Validation remains software-only and does not prove hardware, DAQ, environmental qualification, or certification behavior.
- Owner for residual risk: Verification and Validation Engineer / Documentation Engineer.

### Hand-Off Note

Role: Test Automation Engineer
Goal: Validate M4 signal accuracy and validation branch.
Files changed: `docs/validation-log.md`
Checks run: `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`; validation CLI smoke commands; invalid tolerance CLI command; `sh scripts/benchmark-large-csv.sh 100000 3`
Status: Pass.
Known gaps: No external hardware capture corpus, DAQ integration, or certification evidence.
Next recommended step: Protected-branch PR review and CI.

## M1 Metadata And README Usage Branch Validation

Current as of M1 metadata and README usage review on 2026-05-31.

| Command | Result | Notes |
|---|---|---|
| `cargo fmt --check` | Passed | Rust formatting clean. |
| `cargo test --workspace` | Passed | 53 tests passed: 6 CLI, 31 core, 6 criteria-engine fixture/golden tests, 1 CSV fixture integration test, 9 `wra-signal`, plus doctests. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |
| `git diff --check` | Passed | No whitespace errors in the branch diff. |
| `cargo run --quiet --bin wra -- analyze --input examples/basic-waveform.csv --config examples/basic-config.toml --format text` | Passed | Text output includes metadata, transform history, overall outcome, and criterion evidence matching README. |
| `cargo run --quiet --bin wra -- analyze --input examples/basic-waveform.csv --config examples/basic-config.toml --format json` | Passed | JSON output includes `waveform_metadata` and criterion evidence matching README. |
| `cargo run --quiet --bin wra -- analyze --input examples/basic-waveform.csv --config examples/adc-quantized-config.toml --format text` | Passed | ADC usage output includes metadata, transform history, overall outcome, and criterion evidence matching `docs/usage-mvp.md`. |
| `cargo run --quiet --bin wra -- analyze --input tests/fixtures/dropout_event.csv --config tests/configs/transient-event-dropout-fail.toml --format text` | Passed | Dropout report includes waveform metadata and failed criterion evidence. |
| M4 milestone and issue inspection | Passed | Milestone `M4: Signal Accuracy and Validation` created with issues #27-#34. |

## Documentation Accuracy Branch Validation

Current as of documentation accuracy review on 2026-05-31.

| Command | Result | Notes |
|---|---|---|
| `cargo fmt --check` | Passed | Rust formatting clean. |
| `cargo test --workspace` | Passed | 50 tests passed: 6 CLI, 28 core, 6 criteria-engine fixture/golden tests, 1 CSV fixture integration test, 9 `wra-signal`, plus doctests. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |
| `git diff --check` | Passed | No whitespace errors in the documentation review diff. |
| README local-link target checks | Passed | `docs/adc-quantization.md` and `docs/environmental-test-use-cases.md` exist. |
| Stale-status and conflict-marker scan | Passed | Only intentional audit references and the product prompt abstraction-review line matched. |

## Feature Baseline Validation Snapshot

Current as of PR #25 merge on 2026-05-31.

| Command | Result | Notes |
|---|---|---|
| `cargo fmt --check` | Passed | Rust formatting clean. |
| `cargo test --workspace` | Passed | 50 tests passed: 6 CLI, 28 core, 6 criteria-engine fixture/golden tests, 1 CSV fixture integration test, 9 `wra-signal`, plus doctests. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |
| `cargo run --bin wra -- analyze --input examples/basic-waveform.csv --config examples/adc-quantized-config.toml --format text` | Passed | Config-driven ADC quantization produced `Overall: Pass` with `input_max_after_adc` evidence. |
| GitHub Actions `rust` check for PR #25 | Passed | Required status check passed before merge. |

## Historical MVP Commands And Results

| Command | Result | Notes |
|---|---|---|
| `cargo fmt --check` | Passed | Rustfmt formatting clean after applying `cargo fmt`. |
| `cargo test --workspace` | Passed | 26 tests passed: 19 unit tests, 6 criteria-engine fixture/golden tests, and 1 CSV fixture integration test. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |
| `cargo run --bin wra -- analyze --input examples/basic-waveform.csv --time-column time --channels input_v --moving-average 2 --min input_v:0.0 --max input_v:5.5` | Passed | CLI produced a text report with overall `Pass`. |
| `cargo run --bin wra -- analyze --input examples/basic-waveform.csv --config examples/basic-config.toml --format text` | Passed | Config-driven CLI produced a text report with overall `Pass`. |
| `cargo run --bin wra -- analyze --input examples/basic-waveform.csv --config examples/basic-config.toml --format json` | Passed | Config-driven CLI produced JSON with `overall_outcome: pass`. |
| `cargo run --bin wra -- analyze --input tests/fixtures/dropout_event.csv --config tests/configs/transient-event-dropout-fail.toml --format text` | Passed | Transient event report includes failed criterion, measured duration, required duration, sample index, timestamp, and channel. |
| Golden JSON tests | Passed | `criteria_engine_pass.json`, `transient_event_dropout_fail.json`, and `slow_rise_fail.json` matched exactly. |

## Gate Decision

- Gate: Testing Gate.
- Decision: Pass.
- Reason: Formatting, workspace tests, clippy, explicit-flag CLI smoke, config text/json smoke, invalid config tests, fixture criteria tests, and golden JSON tests passed with project-local Cargo tooling.
- Residual risk: No large-file performance corpus or certified signal-processing validation.
- Owner for residual risk: Test Automation Engineer.

## Handoff

- Next owner: Project Orchestrator.
- Expected deliverable: PR review for v0.2.0 criteria engine.
- Required next gate: Protected-branch PR review and CI.

## M1-001 CSV Parser Edge-Case Validation

Date: 2026-05-31

Stage: Testing M1-001 CSV parser edge cases

Owner Role: Test Automation Engineer

### Commands And Results

| Command | Result | Notes |
|---|---|---|
| `cargo test -p wra-core csv::tests -- --nocapture` | Passed | 10 CSV parser unit tests passed. |
| `cargo fmt --check` | Passed | Rust formatting clean. |
| `cargo test --workspace` | Passed | 24 tests passed: 3 CLI unit tests, 20 core unit tests, and 1 CSV fixture integration test. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |

### Exact Tests Added

| Test | Coverage |
|---|---|
| `csv::tests::rejects_empty_input` | Empty/whitespace input returns `WaveformError::EmptyInput` with CLI-useful display text. |
| `csv::tests::rejects_header_without_samples_as_empty_input` | Header-only CSV returns `WaveformError::EmptyInput`. |
| `csv::tests::reports_missing_time_column` | Missing configured time column returns `WaveformError::MissingColumn { column: "time" }`. |
| `csv::tests::reports_missing_channel_column` | Missing configured channel column returns `WaveformError::MissingColumn { column: "input_v" }`. |
| `csv::tests::reports_malformed_numeric_values_with_column_context` | Bad numeric data returns `WaveformError::InvalidNumber` with column and value context. |
| `csv::tests::reports_inconsistent_record_lengths_as_csv_errors` | Short records return structured `WaveformError::Csv` with record-length context from the CSV parser. |
| `csv::tests::ignores_blank_lines_between_records` | Blank lines between records are accepted and ignored by the parser. |
| `csv::tests::supports_configured_ascii_delimiters` | Semicolon-delimited CSV parses when `CsvParseOptions.delimiter` is set to `';'`. |
| `csv::tests::rejects_non_ascii_delimiters_with_parameter_error` | Unsupported non-ASCII delimiters return `WaveformError::InvalidParameter`. |

### Gate Decision

- Gate: Testing Gate for M1-001.
- Decision: Pass.
- Reason: The added tests cover every issue #1 acceptance criterion plus delimiter validation, and full workspace validation passed.
- Residual risk: Broader DAQ-specific CSV dialect coverage remains future work.
- Owner for residual risk: Test Automation Engineer.

### Hand-Off Note

Role: Test Automation Engineer
Goal: Validate M1-001 CSV parser edge cases.
Files changed: `crates/wra-core/src/csv.rs`, `docs/validation-log.md`
Checks run: `cargo test -p wra-core csv::tests -- --nocapture`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`
Status: Pass.
Known gaps: No external DAQ export corpus included.
Next recommended step: Historical M1-001 validation handoff is complete; use future parser issues for broader CSV dialect coverage.

## M3-RTOS-001 Validation Update

Date: 2026-05-31

Stage: Testing embedded `no_std` signal primitives

Owner Role: Test Automation Engineer

### Environment

- Working directory: `/Users/kota/Desktop/softwareai/projects/waveform-reconstructor-analyzer`
- Isolation: Project-local Cargo workspace; no Python packages or global tools installed.
- New dependencies: None.

### Commands And Results

| Command | Result | Notes |
|---|---|---|
| `cargo fmt --check` | Passed | Rust formatting clean. |
| `cargo test --workspace` | Passed | 24 tests passed: 3 CLI, 11 core, 1 integration fixture, 9 `wra-signal`, plus doctests. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |
| `cargo tree -p wra-signal` | Passed | Output shows only `wra-signal v0.1.0`, confirming no crate dependencies. |

### Gate Decision

- Gate: Testing Gate.
- Decision: Pass.
- Reason: Formatting, tests, clippy, and dependency-tree inspection passed for the new `wra-signal` crate and existing workspace.
- Residual risk: Desktop unit tests prove the `no_std` crate compiles and behaves locally, but embedded target builds are future M3 issues.
- Owner for residual risk: Test Automation Engineer / Embedded Systems Engineer.

### Hand-Off Note

Role: Test Automation Engineer
Goal: Validate M3-RTOS-001 against workspace checks and no-dependency expectations.
Files changed: `docs/validation-log.md`
Checks run: `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo tree -p wra-signal`
Status: Pass.
Known gaps: No ARM64 QEMU or embedded-target compile yet; tracked by follow-up issues.
Next recommended step: V&V Gate for M3-RTOS-001.

## ADC Quantization Validation Update

Date: 2026-05-31

Stage: Testing simulated ADC quantization transform

Owner Role: Test Automation Engineer

### Environment

- Working directory: `/Users/kota/Desktop/softwareai/projects/waveform-reconstructor-analyzer`
- Isolation: Project-local Cargo workspace; no Python packages or global tools installed.
- New dependencies: None.

### Commands And Results

| Command | Result | Notes |
|---|---|---|
| `cargo fmt --check` | Passed | Rust formatting clean. |
| `cargo test --workspace` | Passed | 50 tests passed: 6 CLI, 28 core, 6 criteria-engine, 1 CSV fixture, 9 `wra-signal`, plus doctests. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |
| `cargo run --bin wra -- analyze --input examples/basic-waveform.csv --config examples/adc-quantized-config.toml --format text` | Passed | Config-driven ADC quantization produced `Overall: Pass` with `input_max_after_adc` evidence. |
| `git diff --check` | Passed | No whitespace errors. |
| Conflict-marker and terminology scan | Passed | `rg` found no conflict markers or informal event wording. |

### Exact Tests Added

| Test | Coverage |
|---|---|
| `filter::tests::adc_quantizer_snaps_samples_to_code_levels_without_mutating_input` | Quantizes to ideal code levels, clips outside range, and preserves raw samples. |
| `filter::tests::adc_quantizer_rejects_invalid_parameters` | Rejects zero bit depth, excessive bit depth, and invalid voltage range. |
| `filter::tests::filter_chain_applies_steps_in_order` | Proves ordered pre-criteria pipeline execution with moving average followed by ADC quantization. |
| `config::tests::converts_adc_quantizer_config_to_filter_step` | Converts TOML-style config into the enum-backed filter step. |
| `config::tests::rejects_incomplete_adc_quantizer_config` | Returns a structured missing-field error for incomplete ADC config. |
| `wra-cli::tests::runs_analysis_with_adc_quantization_before_criteria` | Proves CLI criteria evaluate the derived quantized waveform. |

### Gate Decision

- Gate: Testing Gate.
- Decision: Pass.
- Reason: Unit, config, CLI, and workspace tests validate the requested ADC quantization behavior with no new dependencies.
- Residual risk: This validates ideal quantization behavior only, not hardware-specific ADC effects.
- Owner for residual risk: Test Automation Engineer / Electrical Signal Integrity Engineer.

### Hand-Off Note

Role: Test Automation Engineer
Goal: Validate simulated ADC quantization before pass/fail criteria.
Files changed: `docs/validation-log.md`
Checks run: `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; ADC config CLI smoke; `git diff --check`; conflict-marker and terminology scan.
Status: Pass.
Known gaps: No hardware ADC model validation.
Next recommended step: Documentation and final workspace validation.
