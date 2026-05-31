# Waveform Reconstructor and Analyzer

Waveform Reconstructor and Analyzer is a Rust-centered open-source tool for importing CSV time-series waveform data, reconstructing analog signal channels, applying simulated filters, and evaluating waveform behavior against configurable pass/fail criteria.

The first MVP is a CLI and core library slice. It focuses on CSV waveform loading, channel mapping, waveform data structures, low-pass and moving-average filters, waveform criteria, TOML config files, and text/JSON report output.

## Current Status

This repository is in MVP implementation stage. The Rust workspace builds a small core library and CLI that can analyze simple CSV fixtures with either TOML config files or explicit command-line criteria.

## MVP Scope

- Load CSV waveform data.
- Map one time column and one or more signal channels.
- Reconstruct typed waveform objects.
- Apply basic low-pass and moving-average filters as derived waveform outputs.
- Define pass/fail criteria for voltage limits, state transitions, pulse width, transient event duration, stable-state duration, and rise/fall time.
- Run analysis from a CLI.
- Produce text and JSON reports.
- Include tests and example data.

## Non-Goals

- Full GUI.
- Real-time DAQ integration.
- Cloud storage.
- Multi-user accounts.
- Aerospace certification claims.
- Hardware control.
- Proprietary file formats.

## Repository Layout

```text
crates/wra-core/        Rust core library
crates/wra-cli/         CLI entry point
docs/                  Product, architecture, and MVP docs
examples/              Example CSV and config files
tests/fixtures/        Shared test fixtures
tests/golden/          Expected JSON reports
.github/               Issue templates, PR template, CI
```

## Local Development

Prerequisite: Rust toolchain with Cargo.

```bash
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

No global package installation is required.

## MVP Usage

```bash
cargo run --bin wra -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/basic-config.toml \
  --format text
```

JSON output is also available:

```bash
cargo run --bin wra -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/basic-config.toml \
  --format json
```

For quick one-off checks, criteria can still be supplied through CLI flags:

```bash
cargo run --bin wra -- analyze \
  --input examples/basic-waveform.csv \
  --time-column time \
  --channels input_v \
  --moving-average 2 \
  --min input_v:0.0 \
  --max input_v:5.5
```

The current CSV/config/report surface is intentionally small and should be expanded through focused issues.

## Waveform Criteria Example

Transient event detection checks for unintended short state changes. This dropout example fails because `supply_v` drops below `2.5 V` for `0.002 s`, while the config allows only `0.001 s`.

```bash
cargo run --bin wra -- analyze \
  --input tests/fixtures/dropout_event.csv \
  --config tests/configs/transient-event-dropout-fail.toml \
  --format text
```

Expected output:

```text
Waveform Analysis Report
Input: tests/fixtures/dropout_event.csv
Overall: Fail
Criteria:
- supply_dropout_max_1ms: Fail channel=supply_v measured=0.002000 s required=0.001000 s sample_index=3 timestamp=0.003000 reason=longest unintended low dropout duration was 0.002000 s
```

The JSON report includes the same evidence fields for automation: outcome, failed criterion, measured value, required value, sample index, timestamp, and channel.

See [environmental test use cases](docs/environmental-test-use-cases.md) for fixture intent and scope limits.

## License

License: MIT.
