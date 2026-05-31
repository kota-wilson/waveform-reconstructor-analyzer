# Changelog

All notable changes to this project will be documented here.

The format follows Keep a Changelog principles, and the project intends to use semantic versioning once public releases begin.

## Unreleased

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
- `wra-signal` `no_std` crate with fixed-size buffers, streaming threshold checks, and transient event detection for future embedded adapters.
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
- `wra-embedded` `no_std` adapter boundary plus ARM64 QEMU and Zephyr feasibility prototype artifacts.
