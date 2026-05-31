# FerrisOxide

FerrisOxide is a Rust-centered open-source engineering signal validation workspace. Its current implemented product slice, FerrisOxide Signal, imports CSV time-series waveform data, reconstructs analog signal channels, applies simulated filters and ADC quantization, and evaluates waveform behavior against configurable pass/fail criteria.

The first MVP is a CLI and core library slice. It focuses on CSV waveform loading, channel mapping, waveform data structures, low-pass and moving-average filters, ideal ADC quantization, reusable waveform measurements, waveform criteria, TOML config files, text/JSON report output, and optional SVG plotting.

## Current Status

This repository is in validated MVP stage. The main repository is `kota-wilson/ferrisoxide`; `ferrisoxide-signal` remains the crate and CLI binary name for the current signal-analysis slice. The Rust workspace builds a small core library and CLI that can analyze simple CSV fixtures with either TOML config files or explicit command-line criteria, including waveform metadata, ordered pre-criteria transforms such as moving average, low-pass filtering, and ideal ADC quantization. Criteria consume reusable measurement primitives from `ferrisoxide-measurements`, reports expose reusable measurement records with stable result links, and the desktop CLI can also render SVG waveform plots. The workspace has `no_std` crates for signal and embedded paths: `ferrisoxide-signal`, `ferrisoxide-measurements`, `ferrisoxide-rule-engine`, and `ferrisoxide-embedded`; `ferrisoxide-rule-schema` defines the first portable rule package schema for future desktop-to-embedded workflows.

## MVP Scope

- Load CSV waveform data.
- Map one time column and one or more signal channels.
- Reconstruct typed waveform objects.
- Preserve waveform metadata for source name, optional validation context, units, sample interval, sample rate, lineage, transform history, and tolerance policy.
- Apply basic low-pass, moving-average, and ideal ADC quantization transforms as derived waveform outputs.
- Reuse measurement primitives for extrema, state-transition counts, pulse widths, transient durations, stable-state durations, and rise/fall times.
- Define pass/fail criteria for voltage limits, state transitions, pulse width, transient event duration, stable-state duration, and rise/fall time with optional voltage/time tolerances.
- Run analysis from a CLI.
- Produce text and JSON reports with reusable measurement evidence, pass/fail evidence, tolerance evidence, and engineering-validation context.
- Render optional desktop SVG plots for 2D waveform views and 3D views with a configured third-axis column.
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
crates/ferrisoxide-core/        Rust core library
crates/ferrisoxide-cli/         CLI entry point
crates/ferrisoxide-embedded/    no_std RTOS/ARM64 adapter boundaries
crates/ferrisoxide-measurements/no_std measurement primitives used by criteria evidence
crates/ferrisoxide-plot/        Desktop SVG plotting support
crates/ferrisoxide-rule-engine/ Shared rule execution semantics
crates/ferrisoxide-rule-schema/ Portable rule package schema types
crates/ferrisoxide-signal/      no_std signal primitives
docs/                  Product, architecture, and MVP docs
embedded/              Future embedded and ARM64 adapter notes
examples/              Example CSV and config files
tests/fixtures/        Shared test fixtures
tests/golden/          Expected JSON reports
validation/            v0.3.0 known-answer validation workspace
scripts/               Project-local utility scripts
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

`crates/ferrisoxide-signal` is a dependency-free `#![no_std]` crate for fixed-size waveform buffers, streaming sample ingestion, min/max threshold evaluation, and transient event detection. `crates/ferrisoxide-embedded` is a `#![no_std]` adapter-boundary crate with sample-source, event-sink, and runtime-hook traits for future ARM64 and RTOS wrappers.

The embedded track now has a host-checkable ARM64 QEMU proof slice under `embedded/arm64/qemu/` and an isolated Zephyr feasibility sketch under `embedded/arm64/zephyr/`. These prototypes intentionally exclude CSV parsing, file I/O, plotting, text/JSON reports, GUI, DAQ integration, hardware HALs, production RTOS readiness, and certification evidence.

## Portable Rule Packages

`crates/ferrisoxide-rule-schema` defines the initial versioned FerrisOxide Rule Package schema, validator, deterministic manifest model, and non-cryptographic artifact checksum helper. `crates/ferrisoxide-rule-engine` owns the shared rule execution semantics used by the desktop core adapter and embedded-compatible host tests. The engine is `#![no_std]`; desktop-style owned evidence uses `alloc`, and the borrowed summary API uses borrowed rule data plus borrowed/static errors for basic no-heap embedded-compatible evaluation. The reviewable package format is documented in `docs/rule-package-format.md`, with parse-tested examples in `examples/rule-package/`. The CLI can export `rules.toml`, `rules.json`, `validation-report.json`, `manifest.json`, and `checksum.txt` from validated config and analysis evidence. M8 includes exact desktop-vs-embedded parity fixtures under `tests/parity/`. Binary packages and runtime package loaders remain future work.

## MVP Usage

```bash
cargo run --quiet --bin ferrisoxide-signal -- analyze \
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
Validation Profile: engineering_validation
Evidence Source: local_file_analysis
Tolerance Policy: voltage=0.000000 V time=0.000000000 s
Confidence Notes: software validation evidence only; not hardware qualification or certification evidence
Overall: Pass
Measurements:
- input_min_voltage_measurement: method=minimum_sample channel=input_v measured=0.000000 V sample_index=0 timestamp=0.000000
- input_max_voltage_measurement: method=maximum_sample channel=input_v measured=5.000000 V sample_index=4 timestamp=0.004000
Criteria:
- input_min_voltage: Pass measurement_id=input_min_voltage_measurement channel=input_v measured=0.000000 V required=0.000000 V tolerance=0.000000 sample_index=0 timestamp=0.000000 reason=minimum observed voltage was 0.000000 V
- input_max_voltage: Pass measurement_id=input_max_voltage_measurement channel=input_v measured=5.000000 V required=5.500000 V tolerance=0.000000 sample_index=4 timestamp=0.004000 reason=maximum observed voltage was 5.000000 V
```

JSON output is also available and includes the same waveform metadata plus reusable measurement evidence:

```bash
cargo run --quiet --bin ferrisoxide-signal -- analyze \
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
    "test_run_id": null,
    "acquisition_notes": null,
    "environment": null,
    "operator": null,
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
    ],
    "tolerance_policy": {
      "voltage_v": 0.0,
      "time_s": 0.0
    }
  },
  "evidence_context": {
    "validation_profile": "engineering_validation",
    "evidence_source": "local_file_analysis",
    "tolerance_policy": {
      "voltage_v": 0.0,
      "time_s": 0.0
    },
    "confidence_notes": [
      "software validation evidence only",
      "not hardware qualification or certification evidence"
    ]
  },
  "overall_outcome": "pass",
  "measurements": [
    {
      "id": "input_min_voltage_measurement",
      "channel": "input_v",
      "method": "minimum_sample",
      "measured_value": 0.0,
      "unit": "V",
      "sample_index": 0,
      "timestamp": 0.0,
      "method_context": {
        "source": "ferrisoxide-measurements",
        "threshold_v": null,
        "low_threshold_v": null,
        "high_threshold_v": null,
        "state": null,
        "expected_state": null,
        "event_kind": null,
        "direction": null,
        "selection": null
      }
    },
    {
      "id": "input_max_voltage_measurement",
      "channel": "input_v",
      "method": "maximum_sample",
      "measured_value": 5.0,
      "unit": "V",
      "sample_index": 4,
      "timestamp": 0.004,
      "method_context": {
        "source": "ferrisoxide-measurements",
        "threshold_v": null,
        "low_threshold_v": null,
        "high_threshold_v": null,
        "state": null,
        "expected_state": null,
        "event_kind": null,
        "direction": null,
        "selection": null
      }
    }
  ],
  "results": [
    {
      "criterion_id": "input_min_voltage",
      "outcome": "pass",
      "failed_criterion": null,
      "measurement_id": "input_min_voltage_measurement",
      "channel": "input_v",
      "measured_value": 0.0,
      "required_value": 0.0,
      "tolerance_used": 0.0,
      "unit": "V",
      "sample_index": 0,
      "timestamp": 0.0,
      "reason": "minimum observed voltage was 0.000000 V"
    },
    {
      "criterion_id": "input_max_voltage",
      "outcome": "pass",
      "failed_criterion": null,
      "measurement_id": "input_max_voltage_measurement",
      "channel": "input_v",
      "measured_value": 5.0,
      "required_value": 5.5,
      "tolerance_used": 0.0,
      "unit": "V",
      "sample_index": 4,
      "timestamp": 0.004,
      "reason": "maximum observed voltage was 5.000000 V"
    }
  ]
}
```

Measurement-backed DSL configs can express the same checks as measurement plus requirement:

```bash
cargo run --quiet --bin ferrisoxide-signal -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/basic-dsl-config.toml \
  --format text
```

Representative output excerpt:

```text
Overall: Pass
Measurements:
- input_min_voltage_measurement: method=minimum_sample channel=input_v measured=0.000000 V sample_index=0 timestamp=0.000000
- input_max_voltage_measurement: method=maximum_sample channel=input_v measured=5.000000 V sample_index=4 timestamp=0.004000
Criteria:
- input_min_voltage: Pass measurement_id=input_min_voltage_measurement channel=input_v measured=0.000000 V required=0.000000 V tolerance=0.000000 sample_index=0 timestamp=0.000000 reason=minimum observed voltage was 0.000000 V
- input_max_voltage: Pass measurement_id=input_max_voltage_measurement channel=input_v measured=5.000000 V required=5.500000 V tolerance=0.000000 sample_index=4 timestamp=0.004000 reason=maximum observed voltage was 5.000000 V
```

See [criteria DSL migration](docs/criteria-dsl-migration.md) for before/after config examples, explicit unit rules, compatibility expectations, and non-goals.

For quick one-off checks, criteria can still be supplied through CLI flags:

```bash
cargo run --quiet --bin ferrisoxide-signal -- analyze \
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

Implemented transform equations are documented in [filter behavior](docs/filter-behavior.md). Measurement primitives are documented in [measurement engine](docs/measurements.md). Criteria DSL direction is documented in [criteria DSL direction](docs/criteria-dsl.md), with migration notes in [criteria DSL migration](docs/criteria-dsl-migration.md). Time-axis validation and tolerance semantics are documented in [time axis and tolerances](docs/time-axis-and-tolerances.md).

## Plotting

The desktop CLI can render SVG plots without adding GUI or DAQ scope:

```bash
cargo run --quiet --bin ferrisoxide-signal -- plot \
  --input examples/basic-waveform.csv \
  --time-column time \
  --channels input_v,output_v \
  --output basic-waveform.svg
```

Use `--z-column` for a 3D line plot with an auxiliary third axis:

```bash
cargo run --quiet --bin ferrisoxide-signal -- plot \
  --input tests/fixtures/plot_three_axis.csv \
  --time-column time_s \
  --channels signal_v \
  --z-column temperature_c \
  --output three-axis.svg
```

Use `--config` for a 2D SVG plot with criteria evidence overlays:

```bash
cargo run --quiet --bin ferrisoxide-signal -- plot \
  --input tests/fixtures/dropout_event.csv \
  --config tests/configs/transient-event-dropout-fail.toml \
  --output dropout-evidence.svg
```

See [SVG plotting](docs/plotting.md) for scope, commands, and limits.

## Waveform Criteria Example

Transient event detection checks for unintended short state changes. This dropout example fails because `supply_v` drops below `2.5 V` for `0.002 s`, while the config allows only `0.001 s`.

```bash
cargo run --quiet --bin ferrisoxide-signal -- analyze \
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
Validation Profile: engineering_validation
Evidence Source: local_file_analysis
Tolerance Policy: voltage=0.000000 V time=0.000000000 s
Confidence Notes: software validation evidence only; not hardware qualification or certification evidence
Overall: Fail
Criteria:
- supply_dropout_max_1ms: Fail channel=supply_v measured=0.002000 s required=0.001000 s tolerance=0.000000 sample_index=3 timestamp=0.003000 reason=longest unintended low dropout duration was 0.002000 s
```

The JSON report includes the same evidence fields for automation: outcome, failed criterion, measured value, required value, sample index, timestamp, and channel.

See [environmental test use cases](docs/environmental-test-use-cases.md) for fixture intent and scope limits. The v0.3.0 validation workspace includes known-answer fixtures and expected reports under `validation/`; see [benchmarking](docs/benchmarking.md) for repeatable large-CSV measurements.

## License

License: MIT.
