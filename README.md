# Waveform Reconstructor and Analyzer

Waveform Reconstructor and Analyzer is a Rust-centered open-source tool for importing CSV time-series waveform data, reconstructing analog signal channels, applying simulated filters and ADC quantization, and evaluating waveform behavior against configurable pass/fail criteria.

The first MVP is a CLI and core library slice. It focuses on CSV waveform loading, channel mapping, waveform data structures, low-pass and moving-average filters, ideal ADC quantization, waveform criteria, TOML config files, and text/JSON report output.

## Current Status

This repository is in validated MVP stage. The Rust workspace builds a small core library and CLI that can analyze simple CSV fixtures with either TOML config files or explicit command-line criteria, including waveform metadata, ordered pre-criteria transforms such as moving average, low-pass filtering, and ideal ADC quantization. The workspace also has an embedded foundation crate, `wra-signal`, for `no_std` signal primitives that can later be wrapped by RTOS or ARM64 adapters.

## MVP Scope

- Load CSV waveform data.
- Map one time column and one or more signal channels.
- Reconstruct typed waveform objects.
- Preserve waveform metadata for source name, units, sample interval, sample rate, lineage, and transform history.
- Apply basic low-pass, moving-average, and ideal ADC quantization transforms as derived waveform outputs.
- Define pass/fail criteria for voltage limits, state transitions, pulse width, transient event duration, stable-state duration, and rise/fall time.
- Run analysis from a CLI.
- Produce text and JSON reports.
- Include tests and example data.
- Keep embedded signal-analysis primitives separate from desktop CSV, CLI, and report paths.

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
crates/wra-signal/      no_std signal primitives
docs/                  Product, architecture, and MVP docs
embedded/              Future embedded and ARM64 adapter notes
examples/              Example CSV and config files
tests/fixtures/        Shared test fixtures
tests/golden/          Expected JSON reports
validation/            v0.3.0 known-answer validation workspace
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

## Embedded Foundation

`crates/wra-signal` is a dependency-free `#![no_std]` crate for fixed-size waveform buffers, streaming sample ingestion, min/max threshold evaluation, and transient event detection. It intentionally excludes CSV parsing, file I/O, plotting, text/JSON reports, GUI, DAQ integration, and RTOS-specific code.

The embedded track should evolve in this order: keep reusable signal primitives in `wra-signal`, add an ARM64 QEMU proof later, then introduce adapter crates such as `wra-embedded` only after the core analysis surface is stable.

## MVP Usage

```bash
cargo run --quiet --bin wra -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/basic-config.toml \
  --format text
```

Expected text output:

```text
Waveform Analysis Report
Input: examples/basic-waveform.csv
Samples: 5 Channels: 2 Lineage: Derived
Sample Interval: nominal=0.001000000 s min=0.001000000 max=0.001000000 uniform=true
Nominal Sample Rate: 1000.000000 Hz
Transforms: moving_average(window_samples=2)
Overall: Pass
Criteria:
- input_min_voltage: Pass channel=input_v measured=0.000000 V required=0.000000 V sample_index=0 timestamp=0.000000 reason=minimum observed voltage was 0.000000 V
- input_max_voltage: Pass channel=input_v measured=5.000000 V required=5.500000 V sample_index=4 timestamp=0.004000 reason=maximum observed voltage was 5.000000 V
```

JSON output is also available and includes the same waveform metadata:

```bash
cargo run --quiet --bin wra -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/basic-config.toml \
  --format json
```

Expected JSON output:

```json
{
  "input_name": "examples/basic-waveform.csv",
  "waveform_metadata": {
    "source_name": "examples/basic-waveform.csv",
    "time_unit": "s",
    "sample_count": 5,
    "channel_count": 2,
    "channels": [
      {
        "name": "input_v",
        "unit": "V"
      },
      {
        "name": "output_v",
        "unit": "V"
      }
    ],
    "sample_interval": {
      "min": 0.001,
      "max": 0.001,
      "nominal": 0.001,
      "unit": "s",
      "uniform": true
    },
    "nominal_sample_rate_hz": 1000.0,
    "lineage": "derived",
    "transform_history": [
      "moving_average(window_samples=2)"
    ]
  },
  "overall_outcome": "pass",
  "results": [
    {
      "criterion_id": "input_min_voltage",
      "outcome": "pass",
      "failed_criterion": null,
      "channel": "input_v",
      "measured_value": 0.0,
      "required_value": 0.0,
      "unit": "V",
      "sample_index": 0,
      "timestamp": 0.0,
      "reason": "minimum observed voltage was 0.000000 V"
    },
    {
      "criterion_id": "input_max_voltage",
      "outcome": "pass",
      "failed_criterion": null,
      "channel": "input_v",
      "measured_value": 5.0,
      "required_value": 5.5,
      "unit": "V",
      "sample_index": 4,
      "timestamp": 0.004,
      "reason": "maximum observed voltage was 5.000000 V"
    }
  ]
}
```

For quick one-off checks, criteria can still be supplied through CLI flags:

```bash
cargo run --quiet --bin wra -- analyze \
  --input examples/basic-waveform.csv \
  --time-column time \
  --channels input_v \
  --moving-average 2 \
  --adc-quantize 8:0.0:5.0 \
  --min input_v:0.0 \
  --max input_v:5.5
```

The current CSV/config/report surface is intentionally small and should be expanded through focused issues.

ADC quantization can also be configured as an ordered TOML filter step:

```toml
[[filters]]
type = "adc_quantize"
bits = 8
min_v = 0.0
max_v = 5.0
```

The quantizer clips samples outside the configured range, snaps in-range samples to the nearest ideal ADC code level, and keeps output samples in volts so normal voltage criteria can evaluate the digitized waveform. See [ADC quantization transform](docs/adc-quantization.md) for assumptions and limits.

## Waveform Criteria Example

Transient event detection checks for unintended short state changes. This dropout example fails because `supply_v` drops below `2.5 V` for `0.002 s`, while the config allows only `0.001 s`.

```bash
cargo run --quiet --bin wra -- analyze \
  --input tests/fixtures/dropout_event.csv \
  --config tests/configs/transient-event-dropout-fail.toml \
  --format text
```

Expected output:

```text
Waveform Analysis Report
Input: tests/fixtures/dropout_event.csv
Samples: 7 Channels: 1 Lineage: Raw
Sample Interval: nominal=0.001000000 s min=0.001000000 max=0.001000000 uniform=true
Nominal Sample Rate: 1000.000000 Hz
Overall: Fail
Criteria:
- supply_dropout_max_1ms: Fail channel=supply_v measured=0.002000 s required=0.001000 s sample_index=3 timestamp=0.003000 reason=longest unintended low dropout duration was 0.002000 s
```

The JSON report includes the same evidence fields for automation: outcome, failed criterion, measured value, required value, sample index, timestamp, and channel.

See [environmental test use cases](docs/environmental-test-use-cases.md) for fixture intent and scope limits.

## License

License: MIT.
