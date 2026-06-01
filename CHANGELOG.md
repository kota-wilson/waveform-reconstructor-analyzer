# Changelog

All notable changes to this project will be documented here.

The format follows Keep a Changelog principles, and the project intends to use semantic versioning once public releases begin.

## Unreleased

### Changed

- Adopted FerrisOxide Signal as the in-repository product identity, including `ferrisoxide-*` workspace packages and the `ferrisoxide-signal` CLI binary.
- Expanded the main README into a complete product guide covering workflow, examples, repo layout, commands, reports, plotting, rule packages, embedded boundaries, validation assets, and contribution expectations.

### Added

- Initial repository skeleton.
- Project charter, requirements, risk register, architecture, and traceability matrix.
- Rust workspace with core library and CLI analysis path.
- Simple CSV waveform parsing with named time and channel columns.
- Moving-average and first-order low-pass filters.
- Min/max voltage criteria evaluation and text report rendering.
- TOML config parsing for input mapping, filters, and criteria.
- JSON report rendering for automation use.
- GitHub issue templates, PR template, and CI workflow.
- Real waveform fixtures for clean/noisy square waves, analog switch bounce, dropout, slow rise/fall, and multi-channel data.
- Waveform criteria for state transitions, pulse width, transient duration, transient event detection, stable-state duration, and rise/fall time.
- Report evidence fields for failed criterion, measured/required values, sample index, timestamp, and channel.
- Golden JSON report tests and invalid config validation tests.
- `ferrisoxide-signal` `no_std` crate with fixed-size buffers, streaming threshold checks, and transient event detection for future embedded adapters.
- Simulated ADC quantization as an ordered pre-criteria waveform transform for CLI and TOML workflows.
- Waveform metadata for source, units, sample interval, sample rate, lineage, and transform history in text and JSON reports.
- v0.3.0 validation roadmap and validation dataset folder structure.
- Known-answer validation fixtures, environmental dropout/contact-bounce examples, and exact expected validation report tests.
- Configurable voltage/time tolerance policy with report evidence.
- Report evidence context fields for validation profile, evidence source, tolerance policy, and confidence notes.
- Optional validation metadata context for test-run ID, acquisition notes, environment, and operator.
- Time-axis validation for duration criteria plus sample interval and nominal sample-rate documentation.
- Filter behavior documentation with moving-average, first-order low-pass, and ideal ADC quantization equations.
- Project-local large-CSV benchmark helper and baseline benchmark documentation.
- Optional desktop SVG plotting with 2D waveform plots and 3D line plots using a configured third-axis column.
- `ferrisoxide-embedded` `no_std` adapter boundary plus ARM64 QEMU and Zephyr feasibility prototype artifacts.
- `ferrisoxide-measurements` `no_std` crate with reusable extrema, state-transition, state-run duration, and rise/fall measurement primitives used by criteria evidence.
- `ferrisoxide-control-schema` crate with production control config schema types, validation helpers, and a parse-tested example config for future controller-in-the-loop workflows.
- `ferrisoxide-verification-schema` crate with test verification config schema types, validation helpers, and a parse-tested example config for future qualification and controller-in-the-loop workflows.
- `ferrisoxide-simulator` crate with a deterministic virtual controller simulation engine over production control configs and abstract sample frames.
- `ferrisoxide-daq` crate with fixture/test-double DAQ sample-source abstractions for deterministic controller-in-the-loop input.
- `ferrisoxide-controller-io` crate with host-checkable controller input/output traits and fake I/O for portable controller boundaries.
- `simulate` CLI workflow that loads production control config, test verification config, a channel map, and fixture CSV input to produce simulation trace plus verification evidence.
- `ferrisoxide-deployment` crate with RTOS/controller deployment package manifest schema, required artifact roles, validation helpers, checksum drift-detection wording, and a heated-actuator package fixture.
- Deployment manifest mode profiles that separate `production_control`, `test_verification`, and `signal_validation` purposes and reject mixed production/test artifact combinations.
- Controller config parity test comparing desktop simulation state/evidence with embedded-compatible borrowed-rule evidence over the same configs, channel map, and waveform input.
