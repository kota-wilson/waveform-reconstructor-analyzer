# FerrisOxide

FerrisOxide is a Rust-centered open-source workspace for engineering signal validation. The current implemented product slice is **FerrisOxide Signal**, a command-line and library workflow for turning CSV waveform data into auditable pass/fail evidence.

At a practical level, FerrisOxide Signal helps an engineer answer questions like:

- Did this signal stay inside its required voltage range?
- Did a feedback channel respond quickly enough after a command?
- Was a state stable long enough?
- Did a dropout, contact-bounce event, spurious transition, or other transient event exceed the allowed duration?
- Did filtering or ideal ADC quantization change the waveform before criteria were applied?
- Can the evidence be rendered as text, JSON, SVG, and a portable rule-package artifact?

The main repository is `kota-wilson/ferrisoxide`. The current CLI binary is still named `ferrisoxide-signal` because the active product slice is the signal-analysis slice of the broader FerrisOxide platform idea.

## Table Of Contents

- [What FerrisOxide Does](#what-ferrisoxide-does)
- [What It Looks Like In A Real Workflow](#what-it-looks-like-in-a-real-workflow)
- [Why This Is Useful](#why-this-is-useful)
- [Current Status](#current-status)
- [What Is In Scope](#what-is-in-scope)
- [What Is Out Of Scope](#what-is-out-of-scope)
- [Repository Layout](#repository-layout)
- [Data Flow](#data-flow)
- [Install And Local Development](#install-and-local-development)
- [Quick Start](#quick-start)
- [CSV Inputs](#csv-inputs)
- [Configuration Files](#configuration-files)
- [Filters And ADC Quantization](#filters-and-adc-quantization)
- [Criteria And Measurements](#criteria-and-measurements)
- [Reports](#reports)
- [SVG Plotting](#svg-plotting)
- [Heated Actuator Example](#heated-actuator-example)
- [Desktop Simulation Workflow](#desktop-simulation-workflow)
- [Portable Rule Packages](#portable-rule-packages)
- [Controller Deployment Package Format](#controller-deployment-package-format)
- [Embedded And no_std Boundary](#embedded-and-nostd-boundary)
- [Validation Assets](#validation-assets)
- [Testing The Repository](#testing-the-repository)
- [Documentation Map](#documentation-map)
- [Contributing](#contributing)
- [License](#license)

## What FerrisOxide Does

FerrisOxide Signal currently implements this desktop workflow:

```text
CSV waveform data
  -> channel mapping
  -> waveform model and metadata
  -> optional derived transforms
  -> reusable measurements
  -> pass/fail criteria
  -> text, JSON, SVG, and rule-package evidence
```

The tool is deliberately documentation-heavy because the target audience is engineering work where evidence matters. A report should not only say `FAIL`; it should identify the failed criterion, measured value, required value, sample index, timestamp, channel, tolerance, and confidence context.

The workspace also contains early embedded-facing crates. Those crates do not make the desktop CLI an RTOS application. They establish a clean boundary so future controller runtimes can reuse small, deterministic signal and rule logic without inheriting CSV parsing, file I/O, plotting, or report generation.

## What It Looks Like In A Real Workflow

FerrisOxide sits between subcomponent development, controller design, and qualification-style test analysis.

```text
Subcomponents exist
  -> final controller does not exist yet
  -> connect subcomponents to DAQ or test equipment
  -> capture CSV waveform data
  -> use FerrisOxide Signal to simulate analysis rules
  -> tune control assumptions and signal criteria
  -> run software-only validation and environmental-style fixtures
  -> generate evidence reports and annotated SVGs
  -> export approved rule/config package artifacts
  -> later deploy equivalent logic to an RTOS or bare-metal controller runtime
```

The desktop side is the authoring and validation environment. Engineers use it to inspect recorded or simulated signals, refine criteria, compare expected and actual behavior, and produce evidence artifacts that can be reviewed by other engineers.

The embedded side is the future runtime direction. The intent is that an embedded controller can eventually consume a constrained rule package and evaluate live samples using the same core semantics. That prevents the dangerous split where desktop analysis says `PASS` but the controller runtime says `FAIL` for the same waveform and rule definition.

### Example Workflow In Plain English

Imagine a heated actuator subsystem:

1. A command line goes high at `1.000 s`.
2. The actuator feedback should go high within `50 ms`.
3. The feedback should remain stable high for at least `500 ms`.
4. A false low transient longer than `1 ms` is not allowed after the feedback has reached high.
5. The `5 V` supply rail must stay between `4.75 V` and `5.25 V`.

FerrisOxide lets you put those rules in a TOML file, run them against DAQ-style CSV data, and produce output that says exactly which rule passed or failed. The same scenario is included in this repository under `tests/e2e/heated_actuator/`.

## Why This Is Useful

FerrisOxide is useful when signal review needs to be repeatable, inspectable, and automation-friendly.

| Benefit | What it means in practice |
|---|---|
| Repeatable analysis | The same CSV and same TOML config produce the same report. |
| Reviewable criteria | Requirements are written as explicit criteria instead of being hidden in a spreadsheet or one-off script. |
| Evidence-rich failures | Reports include measured value, required value, sample index, timestamp, channel, and reason. |
| Human and machine output | Text is readable in a terminal; JSON is suitable for golden tests and automation. |
| Visual evidence | SVG plots can show waveform traces, threshold overlays, and failed-criterion markers. |
| Raw-data preservation | Filters and ADC quantization produce derived waveforms instead of mutating source data. |
| Desktop-to-embedded direction | Rule schema and rule engine work is structured so future runtimes can share semantics. |
| Scope discipline | The repo explicitly separates software validation evidence from hardware qualification or certification claims. |

## Current Status

FerrisOxide is in a validated MVP stage. It is not a finished commercial product, not a certified test system, and not a live controller runtime.

Implemented today:

- Rust Cargo workspace.
- CSV waveform loading.
- Named time and channel mapping.
- Waveform metadata for units, source, sample count, sample interval, nominal sample rate, lineage, transform history, validation context, and tolerance policy.
- Moving-average, first-order low-pass, and ideal ADC quantization transforms.
- Measurement primitives for extrema, state transitions, pulse width, stable-state duration, transient duration, and rise/fall time.
- Criteria for min/max voltage, state transitions, response latency, pulse width, transient events, stable-state duration, and rise/fall time.
- TOML config parsing with clear errors for invalid config.
- Text and JSON analysis reports.
- Exact golden JSON tests.
- SVG waveform plotting with optional third-axis 3D line plots.
- 2D SVG evidence overlays from report evidence.
- Portable rule package schema, validation, manifest, checksum, and export command.
- Production control and test verification config schema boundaries for future controller-in-the-loop workflows.
- Virtual controller simulation engine over deterministic abstract sample frames.
- Fixture/test-double DAQ input abstraction for deterministic sample sources.
- Host-checkable controller I/O abstraction for portable input/output boundaries.
- Desktop simulation workflow that loads production control config, test verification config, a channel map, and fixture CSV input.
- RTOS/controller deployment package format schema, validator, manifest, required artifact roles, checksum wording, and example package fixture.
- Manifest-level production, test-verification, and signal-validation mode profiles that reject mixed production/test behavior.
- `no_std` signal, measurement, rule-engine, and embedded-boundary crates.
- Desktop-vs-embedded-compatible parity tests for rule evidence.
- Software-only heated actuator qualification scenario.

Planned or future:

- Runtime loaders.
- Raspberry Pi 5 bare-metal runtime work.
- Optional Pico 2 micro-runtime profile.

## What Is In Scope

The current repo focuses on engineering signal analysis from local files:

- Local CSV input.
- Local TOML config.
- Local text/JSON reports.
- Local SVG plots.
- Local rule-package export.
- Software-only validation fixtures.
- Rust library and CLI development.
- `no_std` reusable primitives where they make architectural sense.

## What Is Out Of Scope

The repo intentionally does not claim or implement:

- Full GUI.
- Live DAQ integration.
- Vendor DAQ SDK support.
- Hardware control.
- Hardware-in-the-loop execution.
- Production RTOS runtime.
- Certified aerospace validation.
- Safety certification.
- Cloud storage.
- Multi-user accounts.
- Plugin runtime.
- Machine learning analysis.
- Proprietary binary waveform formats.

If a future issue adds any of those, it should go through a fresh requirements, architecture, dependency, security, and validation gate.

## Repository Layout

```text
crates/ferrisoxide-core/         Desktop core library:
                                  CSV parsing, waveform model, config,
                                  filters, criteria adapter, reports.

crates/ferrisoxide-cli/          CLI entry point.
                                  Builds the `ferrisoxide-signal` binary,
                                  including analyze, plot, export, and
                                  desktop simulation workflows.

crates/ferrisoxide-control-schema/
                                  Production control config schema for future
                                  controller-in-the-loop workflows.

crates/ferrisoxide-verification-schema/
                                  Test verification config schema for
                                  controller-in-the-loop workflows.

crates/ferrisoxide-simulator/    Deterministic virtual controller simulation
                                  over production control configs.

crates/ferrisoxide-daq/          Fixture/test-double DAQ sample-source
                                  abstraction.

crates/ferrisoxide-controller-io/
                                  Host-checkable controller input/output
                                  abstraction.

crates/ferrisoxide-deployment/   Deployment package manifest schema and
                                  validator for controller/runtime packages.

crates/ferrisoxide-plot/         Desktop SVG plotting support.
                                  Isolated so core and embedded crates do not
                                  depend on plotting.

crates/ferrisoxide-measurements/ no_std measurement primitives.
                                  Extrema, state runs, transitions, rise/fall.

crates/ferrisoxide-rule-engine/  no_std shared rule execution semantics.
                                  Used by desktop and embedded-compatible tests.

crates/ferrisoxide-rule-schema/  Portable rule package schema, validator,
                                  manifest, checksum helper, and package types.

crates/ferrisoxide-signal/       no_std signal primitives.
                                  Fixed buffers, streaming samples, threshold
                                  checks, transient event detection.

crates/ferrisoxide-embedded/     no_std embedded adapter boundary.
                                  SampleSource, EventSink, RuntimeHooks.

examples/                        Small example CSV and TOML configs.

tests/fixtures/                  Shared waveform fixtures.

tests/golden/                    Expected JSON reports for regression tests.

tests/e2e/heated_actuator/       Software-only controller-style qualification
                                  scenario with fixtures, configs, and expected
                                  JSON reports.

tests/parity/                    Desktop-vs-embedded-compatible rule parity
                                  fixtures.

validation/                      Known-answer and environmental validation
                                  assets.

embedded/                        ARM64/QEMU and Zephyr feasibility notes.

docs/                            Architecture, plotting, rule package,
                                  validation, pipeline, review, and roadmap docs.

decisions/                       Architecture decision records.

scripts/                         Project-local helper scripts.

.github/                         CI, issue templates, and PR template.
```

## Data Flow

### Analyze Flow

```text
CSV file
  -> CsvParseOptions
  -> Waveform
  -> optional FilterStep chain
  -> ferrisoxide-measurements primitives
  -> ferrisoxide-rule-engine criteria semantics
  -> AnalysisReport
  -> text or JSON
```

### Plot Flow

```text
CSV file
  -> Waveform
  -> optional config-driven criteria evaluation
  -> SVG renderer
  -> plain waveform plot or evidence overlay plot
```

### Rule Package Export Flow

```text
CSV file + TOML config
  -> analysis evidence
  -> portable rule package schema
  -> package validation
  -> rules.toml
  -> rules.json
  -> validation-report.json
  -> manifest.json
  -> checksum.txt
```

### Desktop Simulation Flow

```text
production control config
  + test verification config
  + channel map
  + fixture CSV input
  -> fixture DAQ frames
  -> virtual controller simulation trace
  -> waveform verification evidence
  -> JSON or text workflow report
```

### Controller Deployment Package Flow

```text
production control config
  + test verification config
  + channel map
  + qualification report
  + qualification evidence SVG
  -> deployment package manifest
  -> checksum index for drift detection
  -> generated-at timestamp
  -> reviewable controller/runtime package artifact set
```

### Embedded Direction

```text
fixed time/sample slices or streaming samples
  -> no_std signal primitives
  -> no_std measurement and rule engine
  -> compact pass/fail summary or runtime event sink
```

The embedded direction deliberately excludes CSV, TOML parsing, JSON rendering, SVG plotting, file I/O, hardware HALs, SDKs, and certification evidence from the core embedded-compatible path.

## Install And Local Development

Prerequisite: Rust with Cargo. The workspace uses Rust 2021 and pins the workspace Rust version to `1.76` in `Cargo.toml`.

No Python virtual environment is required for normal FerrisOxide development because this repo is a Cargo workspace and does not install Python packages. Do not install global packages for this project.

Clone and enter the repository:

```bash
git clone https://github.com/kota-wilson/ferrisoxide.git
cd ferrisoxide
```

Run the standard checks:

```bash
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Run the CLI from source:

```bash
cargo run --quiet --bin ferrisoxide-signal -- --help
```

Expected help shape:

```text
FerrisOxide Signal

Usage:
  ferrisoxide-signal analyze --input <csv> --config examples/basic-config.toml --format text
  ferrisoxide-signal analyze --input <csv> --time-column time --channels input_v --moving-average 3 --low-pass 25 --adc-quantize 12:0.0:5.0 --min input_v:0.0 --max input_v:5.5 --format json
  ferrisoxide-signal plot --input <csv> --time-column time --channels input_v --output waveform.svg
  ferrisoxide-signal plot --input <csv> --config examples/basic-config.toml --output annotated.svg
  ferrisoxide-signal plot --input <csv> --time-column time --channels input_v --z-column temp_c --output waveform-3d.svg
  ferrisoxide-signal export-rule-package --input <csv> --config examples/basic-dsl-config.toml --output-dir deployment --package-name switch-rule --package-version 1.0.0 --target controller_runtime
  ferrisoxide-signal simulate --input tests/e2e/heated_actuator/input/passing_run.csv --control-config examples/control-config/production-control-config.toml --verification-config examples/test-verification-config/test-verification-config.toml --channel-map examples/simulation/heated-actuator-channel-map.toml --format json
```

## Quick Start

The fastest useful command is an analysis run against the included example CSV and TOML config:

```bash
cargo run --quiet --bin ferrisoxide-signal -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/basic-config.toml \
  --format text
```

Expected output:

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

The output tells you:

- Which input file was analyzed.
- Whether the waveform was raw or derived.
- How many samples and channels were present.
- What the sample interval and nominal sample rate were.
- Which transforms were applied.
- What evidence context applies.
- Which reusable measurements were produced.
- Which criteria passed or failed.

For JSON output, change `--format text` to `--format json`:

```bash
cargo run --quiet --bin ferrisoxide-signal -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/basic-config.toml \
  --format json
```

The JSON report includes:

- `input_name`
- `waveform_metadata`
- `evidence_context`
- `overall_outcome`
- `measurements`
- `results`

That shape is intended for automation, regression tests, and evidence comparison.

## CSV Inputs

FerrisOxide expects CSV data with one time column and one or more signal columns.

Example:

```csv
time,input_v,output_v
0.000,0.0,0.0
0.001,1.0,0.8
0.002,2.5,2.0
0.003,4.0,3.2
0.004,5.0,4.8
```

The TOML config or CLI flags tell FerrisOxide which column is time and which columns are signal channels. Time defaults to seconds and signal values default to volts unless configured otherwise.

The parser and model record:

- Source file name.
- Time unit.
- Channel names.
- Channel units.
- Sample count.
- Channel count.
- Sample interval summary.
- Nominal sample rate when calculable.
- Raw or derived lineage.
- Transform history.

Malformed CSV should return structured errors instead of panics. Covered cases include empty input, missing columns, non-numeric fields, inconsistent records, blank lines, and configured delimiters.

## Configuration Files

FerrisOxide configs are TOML files. The current config surface supports input mapping, optional metadata, tolerances, ordered filters, legacy criteria, and measurement-backed DSL criteria.

### Basic Config

`examples/basic-config.toml`:

```toml
[input]
time_column = "time"
channels = ["input_v", "output_v"]
time_unit = "s"
signal_unit = "V"

[[filters]]
type = "moving_average"
window_samples = 2

[[criteria]]
id = "input_min_voltage"
type = "minimum_voltage"
channel = "input_v"
threshold_v = 0.0

[[criteria]]
id = "input_max_voltage"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
```

This config means:

- Read `time` as the time axis.
- Read `input_v` and `output_v` as signal channels.
- Treat time as seconds and signals as volts.
- Apply a two-sample moving average before criteria.
- Require `input_v` to be at least `0.0 V`.
- Require `input_v` to be at most `5.5 V`.

### Metadata And Tolerances

Configs can include engineering context:

```toml
[metadata]
test_run_id = "heated-actuator-software-qualification"
acquisition_notes = "Software-only simulated DAQ fixture."
environment = "simulated vibration/thermal qualification profile"
operator = "FerrisOxide automated test suite"

[tolerances]
voltage_v = 0.0
time_s = 0.0
```

Metadata appears in reports. Tolerances are applied to pass/fail decisions and shown in evidence.

### Measurement-Backed DSL Config

The DSL form expresses a criterion as a measurement plus a requirement:

```toml
[[criteria]]
id = "input_max_voltage"
channel = "input_v"

[criteria.measurement]
type = "maximum_sample"

[criteria.requirement]
operator = "less_than_or_equal"
value = 5.5
unit = "V"
```

Use the DSL form when you want the config to read like:

```text
Measure the maximum sample on input_v.
Require it to be less than or equal to 5.5 V.
```

See [criteria DSL migration](docs/criteria-dsl-migration.md) and [criteria DSL reference](docs/criteria-dsl.md) for supported operators, explicit unit rules, migration notes, and non-goals.

### CLI Flags For One-Off Checks

You can also run quick checks without a TOML file:

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

Use config files for repeatable engineering evidence. Use CLI flags for quick local exploration.

## Filters And ADC Quantization

Filters are ordered pre-criteria transforms. They produce derived waveform data and preserve the raw input.

| Transform | Config type | Purpose |
|---|---|---|
| Moving average | `moving_average` | Smooth samples with a trailing window that includes the current sample. |
| First-order low-pass | `low_pass` | Apply a simple low-pass smoothing model over a strictly increasing time axis. |
| Ideal ADC quantization | `adc_quantize` | Clip and snap analog values to ideal ADC code levels before criteria evaluation. |

Example ADC quantization config:

```toml
[[filters]]
type = "adc_quantize"
bits = 8
min_v = 0.0
max_v = 5.0
```

Run it:

```bash
cargo run --quiet --bin ferrisoxide-signal -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/adc-quantized-config.toml \
  --format text
```

Expected output:

```text
Waveform Analysis Report
Input: examples/basic-waveform.csv
Samples: 5 Channels: 1 Lineage: Derived
Sample Interval: nominal=0.001000000 s min=0.001000000 max=0.001000000 uniform=true
Nominal Sample Rate: 1000.000000 Hz
Transforms: adc_quantize(bits=8,min_v=0,max_v=5)
Validation Profile: engineering_validation
Evidence Source: local_file_analysis
Tolerance Policy: voltage=0.000000 V time=0.000000000 s
Confidence Notes: software validation evidence only; not hardware qualification or certification evidence
Overall: Pass
Measurements:
- input_max_after_adc_measurement: method=maximum_sample channel=input_v measured=5.000000 V sample_index=3 timestamp=0.003000
Criteria:
- input_max_after_adc: Pass measurement_id=input_max_after_adc_measurement channel=input_v measured=5.000000 V required=5.000000 V tolerance=0.000000 sample_index=3 timestamp=0.003000 reason=maximum observed voltage was 5.000000 V
```

Important ADC assumptions:

- The current quantizer is idealized.
- It clips values outside the configured range.
- It snaps in-range samples to endpoint-inclusive code levels.
- It outputs volts so voltage criteria still work normally.
- It is not a physical ADC noise, aperture, offset, gain, INL, DNL, or sampling model.

See [ADC quantization](docs/adc-quantization.md) and [filter behavior](docs/filter-behavior.md) for equations and limits.

## Criteria And Measurements

FerrisOxide separates measurement evidence from criteria decisions.

A measurement answers:

```text
What did the signal do?
```

A criterion answers:

```text
Was that behavior acceptable?
```

Supported criteria include:

| Criterion | Typical use |
|---|---|
| `minimum_voltage` | Check that a rail or signal never drops below a lower limit. |
| `maximum_voltage` | Check that a rail or signal never exceeds an upper limit. |
| `state_transition` | Count or require threshold-based state changes. |
| `response_latency` | Measure time from a source channel entering a state to a target channel entering an expected state. |
| `pulse_width` | Measure how long a signal remains in a high or low state. |
| `transient_event` | Detect unintended short state changes such as dropouts, contact bounce, or spurious transitions. |
| `stable_state_duration` | Require a signal to stay high or low for a minimum duration. |
| `rise_time` | Measure low-to-high transition time between thresholds. |
| `fall_time` | Measure high-to-low transition time between thresholds. |

Example transient event failure:

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
Measurements:
- supply_dropout_max_1ms_measurement: method=state_run_duration channel=supply_v measured=0.002000 s sample_index=3 timestamp=0.003000
Criteria:
- supply_dropout_max_1ms: Fail measurement_id=supply_dropout_max_1ms_measurement channel=supply_v measured=0.002000 s required=0.001000 s tolerance=0.000000 sample_index=3 timestamp=0.003000 reason=longest unintended low dropout duration was 0.002000 s
```

That failure is useful because it does not just say "bad waveform." It says:

- Which criterion failed.
- Which channel failed.
- How long the dropout lasted.
- What the allowed duration was.
- Where the evidence occurs in the sample stream.
- What timestamp to inspect.

## Reports

FerrisOxide reports are designed to be both human-readable and auditable.

### Text Reports

Use text when a person is reading terminal output or attaching a short result to a review:

```bash
cargo run --quiet --bin ferrisoxide-signal -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/basic-config.toml \
  --format text
```

Text reports include:

- Input file.
- Sample and channel counts.
- Lineage.
- Sample interval summary.
- Nominal sample rate.
- Transform history.
- Validation profile.
- Evidence source.
- Tolerance policy.
- Confidence notes.
- Overall pass/fail.
- Measurements.
- Criteria results.

### JSON Reports

Use JSON when another tool, test, or review process needs structured evidence:

```bash
cargo run --quiet --bin ferrisoxide-signal -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/basic-config.toml \
  --format json
```

Representative shape:

```json
{
  "input_name": "examples/basic-waveform.csv",
  "waveform_metadata": {
    "source_name": "examples/basic-waveform.csv",
    "time_unit": "s",
    "sample_count": 5,
    "channel_count": 2,
    "lineage": "derived",
    "transform_history": [
      "moving_average(window_samples=2)"
    ]
  },
  "evidence_context": {
    "validation_profile": "engineering_validation",
    "evidence_source": "local_file_analysis",
    "confidence_notes": [
      "software validation evidence only",
      "not hardware qualification or certification evidence"
    ]
  },
  "overall_outcome": "pass",
  "measurements": [],
  "results": []
}
```

Exact expected JSON reports are stored under `tests/golden/`, `validation/reports/`, and `tests/e2e/heated_actuator/expected/`.

See [report schema](docs/report-schema.md) for field details.

## SVG Plotting

FerrisOxide can render desktop SVG plots. This is not a GUI and does not add DAQ or embedded plotting scope.

### 2D Plot

```bash
cargo run --quiet --bin ferrisoxide-signal -- plot \
  --input examples/basic-waveform.csv \
  --time-column time \
  --channels input_v,output_v \
  --output basic-waveform.svg
```

### 3D Plot With Auxiliary Axis

Use `--z-column` when a third CSV column should be rendered as an auxiliary axis, such as temperature, pressure, command state, vibration level, or another test condition.

```bash
cargo run --quiet --bin ferrisoxide-signal -- plot \
  --input tests/fixtures/plot_three_axis.csv \
  --time-column time_s \
  --channels signal_v \
  --z-column temperature_c \
  --output three-axis.svg
```

### 2D Evidence Overlay

Use `--config` to evaluate criteria and render threshold/failure evidence on the SVG:

```bash
cargo run --quiet --bin ferrisoxide-signal -- plot \
  --input tests/fixtures/dropout_event.csv \
  --config tests/configs/transient-event-dropout-fail.toml \
  --output dropout-evidence.svg
```

Evidence overlays are currently 2D only. See [SVG plotting](docs/plotting.md) for supported behavior and limits.

## Heated Actuator Example

The heated actuator suite is the clearest end-to-end software-only example in the repository.

Files:

```text
tests/e2e/heated_actuator/
  input/
    passing_run.csv
    failing_late_response.csv
    failing_transient_event.csv
    failing_supply_dropout.csv
  configs/
    production-control-config.toml
    test-verification-config.toml
  expected/
    passing_report.json
    failing_late_response_report.json
    failing_transient_event_report.json
    failing_supply_dropout_report.json
  output/
```

The executable verification config checks:

| ID | Rule |
|---|---|
| `REQ-001` | Feedback must reach high within `50 ms` after command reaches high. |
| `REQ-002` | Feedback must remain high for at least `500 ms`. |
| `REQ-003` | Feedback must not have a post-response false-low transient longer than `1 ms`. |
| `REQ-004-min` | Supply must stay at or above `4.75 V`. |
| `REQ-004-max` | Supply must stay at or below `5.25 V`. |

Run the transient-event failing case:

```bash
cargo run --quiet --bin ferrisoxide-signal -- analyze \
  --input tests/e2e/heated_actuator/input/failing_transient_event.csv \
  --config tests/e2e/heated_actuator/configs/test-verification-config.toml \
  --format text
```

Expected output:

```text
Waveform Analysis Report
Input: tests/e2e/heated_actuator/input/failing_transient_event.csv
Samples: 10 Channels: 3 Lineage: Raw
Sample Interval: nominal=0.200000000 s min=0.001000000 max=0.500000000 uniform=false
Nominal Sample Rate: 5.000000 Hz
Validation Profile: engineering_validation
Evidence Source: local_file_analysis
Tolerance Policy: voltage=0.000000 V time=0.000000000 s
Confidence Notes: software validation evidence only; not hardware qualification or certification evidence
Overall: Fail
Measurements:
- REQ-001_measurement: method=response_latency channel=actuator_feedback_v measured=0.020000 s sample_index=4 timestamp=1.020000
- REQ-002_measurement: method=state_run_duration channel=actuator_feedback_v measured=0.597600 s sample_index=7 timestamp=1.202400
- REQ-003_measurement: method=state_run_duration channel=actuator_feedback_v measured=0.002400 s sample_index=6 timestamp=1.200000
- REQ-004-min_measurement: method=minimum_sample channel=supply_v measured=4.990000 V sample_index=8 timestamp=1.520000
- REQ-004-max_measurement: method=maximum_sample channel=supply_v measured=5.010000 V sample_index=5 timestamp=1.100000
Criteria:
- REQ-001: Pass measurement_id=REQ-001_measurement channel=actuator_feedback_v measured=0.020000 s required=0.050000 s tolerance=0.000000 sample_index=4 timestamp=1.020000 reason=actuator_feedback_v reached high 0.020000 s after command_v reached high
- REQ-002: Pass measurement_id=REQ-002_measurement channel=actuator_feedback_v measured=0.597600 s required=0.500000 s tolerance=0.000000 sample_index=7 timestamp=1.202400 reason=longest stable high duration was 0.597600 s
- REQ-003: Fail measurement_id=REQ-003_measurement channel=actuator_feedback_v measured=0.002400 s required=0.001000 s tolerance=0.000000 sample_index=6 timestamp=1.200000 reason=longest unintended low transient event duration was 0.002400 s
- REQ-004-min: Pass measurement_id=REQ-004-min_measurement channel=supply_v measured=4.990000 V required=4.750000 V tolerance=0.000000 sample_index=8 timestamp=1.520000 reason=minimum observed voltage was 4.990000 V
- REQ-004-max: Pass measurement_id=REQ-004-max_measurement channel=supply_v measured=5.010000 V required=5.250000 V tolerance=0.000000 sample_index=5 timestamp=1.100000 reason=maximum observed voltage was 5.010000 V
```

What this proves in software:

- CSV import works.
- Channel mapping works.
- Waveform reconstruction works.
- Threshold interpretation works.
- Response latency is measured across channels.
- Stable-state duration is measured.
- Transient event duration is measured after arming.
- Supply range checks work.
- Text and JSON reports expose evidence.
- SVG evidence and rule-package export have smoke coverage.

What it does not prove:

- Real actuator behavior.
- DAQ driver correctness.
- Hardware timing accuracy.
- Controller-loop correctness.
- RTOS runtime behavior.
- Certification compliance.

See [heated actuator qualification suite](docs/heated-actuator-qualification-suite.md) for the suite rationale and scope limits.

## Desktop Simulation Workflow

The `simulate` command is the first controller-in-the-loop desktop workflow. It is still software-only, but it connects the milestone pieces into one command:

```text
production control config
+ test verification config
+ channel map
+ fixture CSV
-> virtual controller trace
-> verification evidence
```

Run the included heated actuator workflow:

```bash
cargo run --quiet --bin ferrisoxide-signal -- simulate \
  --input tests/e2e/heated_actuator/input/passing_run.csv \
  --control-config examples/control-config/production-control-config.toml \
  --verification-config examples/test-verification-config/test-verification-config.toml \
  --channel-map examples/simulation/heated-actuator-channel-map.toml \
  --format text
```

Expected excerpt:

```text
Desktop Simulation Workflow
Mode: normal
Simulation Frames: 9
Verification Overall: Pass
Simulation Transitions:
- sample_index=3 timestamp=1.000000 machine=actuator_control transition=command_to_heating idle -> heating
- sample_index=4 timestamp=1.020000 machine=actuator_control transition=feedback_reached heating -> idle
Verification Criteria:
- REQ-001: Pass channel=feedback measured=0.020000 required=0.050000 sample_index=4 timestamp=1.020000
```

Use `--format json` when the output needs to be consumed by automation. The JSON document includes `workflow`, `simulation_trace`, and `verification_evidence` sections. Use `--output-json <path>` to write that JSON to a new file; the command refuses to overwrite an existing artifact.

The channel map is the bridge between CSV fixture columns, logical verification channels, and production-control input IDs:

```toml
[simulation]
mode = "normal"
time_column = "time_s"
time_unit = "s"

[[channels]]
id = "command"
column = "command_v"
unit = "V"

[[control_inputs]]
input = "command"
channel = "command"
```

Current simulation workflow limits:

- Fixture CSV input only.
- No live DAQ SDK.
- No GUI.
- No production RTOS binding.
- No hardware timing guarantee.
- No certification claim.

See [desktop simulation workflow](docs/desktop-simulation-workflow.md) and [controller-in-the-loop workflow](docs/controller-in-the-loop-workflow.md).

## Portable Rule Packages

FerrisOxide can export a reviewable rule package from a validated config and analysis run:

Use a fresh output directory because the exporter refuses to overwrite existing package artifacts.

```bash
cargo run --quiet --bin ferrisoxide-signal -- export-rule-package \
  --input tests/e2e/heated_actuator/input/passing_run.csv \
  --config tests/e2e/heated_actuator/configs/test-verification-config.toml \
  --output-dir /tmp/ferrisoxide-heated-actuator-package \
  --package-name heated-actuator-qualification \
  --package-version 0.1.0 \
  --target controller_runtime
```

Expected artifact set:

```text
rules.toml
rules.json
validation-report.json
manifest.json
checksum.txt
```

The package is meant to be inspected by humans and consumed by future tooling. It is not a signed release, not a binary runtime package, and not certification evidence.

`manifest.json` records:

- Manifest version.
- Generator.
- Schema version.
- Package name and version.
- Target kind.
- Source input and config.
- Validation report artifact.
- Checksum algorithm.
- Artifact metadata.

`checksum.txt` uses a deterministic non-cryptographic checksum for artifact drift detection. It is not cryptographic signing.

See [rule package format](docs/rule-package-format.md) and [ADR-004 portable rule package architecture](decisions/ADR-004-portable-rule-package-architecture.md).

## Controller Deployment Package Format

FerrisOxide also defines the controller/RTOS deployment package artifact set used by the controller-in-the-loop workflow. This is separate from the current `export-rule-package` CLI command: the CLI exports portable verification rules, while the controller deployment package format links production control config, test verification config, channel map, package manifest, checksum index, qualification report, evidence SVG, and generated timestamp.

Example package:

```text
examples/deployment-package/heated-actuator/
  production-control-config.toml
  test-verification-config.toml
  channel-map.toml
  manifest.json
  checksum.txt
  qualification-report.json
  qualification-evidence.svg
  generated-at.txt
```

The manifest validator checks that every required artifact role is present, artifact paths are unique, production and test configs stay separate, and the checksum index appears in the artifact list.

The manifest also defines explicit operating mode profiles:

| Mode purpose | Behavior |
|---|---|
| `production_control` | Selects a production control config mode and does not consume test-verification artifacts. |
| `test_verification` | Evaluates test criteria and does not select production control behavior. |
| `signal_validation` | Evaluates signal criteria without commanding production outputs. |

Invalid mixed mode profiles return structured validation errors before a runtime can consume the package.

The checksum index is only for artifact drift detection. It is not signing, authentication, hardware qualification, flight certification, or production readiness evidence.

See [RTOS deployment package format](docs/rtos-deployment-package-format.md) and [controller operating modes](docs/controller-operating-modes.md).

## Embedded And no_std Boundary

FerrisOxide has embedded-oriented crates, but the project is not yet an embedded runtime product.

| Crate | Boundary |
|---|---|
| `ferrisoxide-signal` | `#![no_std]` signal primitives: fixed buffers, streaming ingestion, threshold checks, transient events. |
| `ferrisoxide-measurements` | `#![no_std]` measurement primitives over slices. |
| `ferrisoxide-rule-engine` | `#![no_std]` rule execution semantics over caller-provided time/sample slices. |
| `ferrisoxide-embedded` | `#![no_std]` adapter traits for sample sources, event sinks, and runtime hooks. |
| `ferrisoxide-control-schema` | Production control config schema for future controller-in-the-loop workflows; not a runtime executor. |
| `ferrisoxide-verification-schema` | Test verification config schema for qualification criteria, timing windows, evidence settings, and report settings; not a criteria executor. |
| `ferrisoxide-deployment` | Deployment package manifest schema and validator for future controller/runtime package workflows; not an RTOS loader. |

Desktop-only concerns stay out of those crates:

- CSV parsing.
- TOML parsing.
- JSON report rendering.
- SVG plotting.
- File I/O.
- GUI.
- DAQ SDKs.
- HALs.
- RTOS SDKs.
- Certification artifacts.

Current target direction:

- Desktop authoring: Apple Silicon macOS, `aarch64-apple-darwin`.
- First-class embedded runtime target: Raspberry Pi 5 bare-metal ARM64, `aarch64-unknown-none`.
- Future optional micro-runtime: Raspberry Pi Pico 2 / RP2350 for constrained rule subsets.

See [embedded roadmap](docs/embedded-roadmap.md), [platform targets](docs/platform-targets.md), [controller-in-the-loop workflow](docs/controller-in-the-loop-workflow.md), [production control config schema](docs/control-config-schema.md), and [test verification config schema](docs/test-verification-config-schema.md).

## Validation Assets

The repository has several layers of validation evidence.

| Location | Purpose |
|---|---|
| `examples/` | Small starter CSV/config examples. |
| `tests/fixtures/` | Unit and integration test waveform fixtures. |
| `tests/golden/` | Exact JSON outputs for criteria regression tests. |
| `validation/known_answer/` | Known-answer waveform validation assets. |
| `validation/environmental_cases/` | Dropout and contact-bounce style cases. |
| `validation/measurement_engine/` | Known-answer measurement-engine fixture. |
| `validation/reports/` | Expected validation reports. |
| `tests/e2e/heated_actuator/` | Software-only controller-style qualification scenario. |
| `tests/parity/` | Desktop-vs-embedded-compatible rule evidence parity. |

Useful targeted checks:

```bash
cargo test -p ferrisoxide-core --test criteria_engine
cargo test -p ferrisoxide-core --test heated_actuator
cargo test -p ferrisoxide-core --test rule_parity
cargo test -p ferrisoxide-rule-engine
cargo test -p ferrisoxide-rule-schema
```

## Testing The Repository

Run the full local validation set before opening a PR:

```bash
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
git diff --check
```

Common focused checks:

```bash
cargo test -p ferrisoxide-core
cargo test -p ferrisoxide-cli
cargo test -p ferrisoxide-plot
cargo test -p ferrisoxide-measurements
cargo test -p ferrisoxide-rule-engine
cargo test -p ferrisoxide-rule-schema
cargo test -p ferrisoxide-signal
cargo test -p ferrisoxide-embedded
```

Embedded boundary checks used by prior pipeline work include:

```bash
cargo check -p ferrisoxide-rule-engine --target aarch64-unknown-none
cargo check -p ferrisoxide-embedded --target aarch64-unknown-none
```

Those target checks require the Rust target to be installed. Do not install global tooling or target SDKs without an explicit environment gate.

## Documentation Map

Start here:

- [Architecture](docs/architecture.md)
- [MVP usage sketch](docs/usage-mvp.md)
- [Report schema](docs/report-schema.md)
- [Criteria DSL](docs/criteria-dsl.md)
- [Criteria DSL migration](docs/criteria-dsl-migration.md)
- [Measurements](docs/measurements.md)
- [Filter behavior](docs/filter-behavior.md)
- [ADC quantization](docs/adc-quantization.md)
- [Time axis and tolerances](docs/time-axis-and-tolerances.md)
- [SVG plotting](docs/plotting.md)
- [Rule package format](docs/rule-package-format.md)
- [Heated actuator qualification suite](docs/heated-actuator-qualification-suite.md)
- [Environmental test use cases](docs/environmental-test-use-cases.md)
- [Embedded roadmap](docs/embedded-roadmap.md)
- [Platform targets](docs/platform-targets.md)
- [Controller-in-the-loop workflow](docs/controller-in-the-loop-workflow.md)
- [Production control config schema](docs/control-config-schema.md)
- [Test verification config schema](docs/test-verification-config-schema.md)
- [Desktop simulation workflow](docs/desktop-simulation-workflow.md)
- [RTOS deployment package format](docs/rtos-deployment-package-format.md)
- [Controller operating modes](docs/controller-operating-modes.md)
- [Validation log](docs/validation-log.md)
- [Traceability matrix](traceability-matrix.md)
- [Requirements](requirements.md)
- [Risk register](risk-register.md)

Architecture decisions:

- [ADR-001 initial architecture](decisions/ADR-001-initial-architecture.md)
- [ADR-003 filter pipeline architecture](decisions/ADR-003-filter-pipeline-architecture.md)
- [ADR-004 portable rule package architecture](decisions/ADR-004-portable-rule-package-architecture.md)
- [ADR-005 FerrisOxide brand architecture](decisions/ADR-005-ferrisoxide-brand-architecture.md)
- [ADR-006 FerrisOxide Signal identity adoption](decisions/ADR-006-ferrisoxide-signal-identity-adoption.md)
- [ADR-007 repository host FerrisOxide](decisions/ADR-007-repository-host-ferrisoxide.md)

## Contributing

FerrisOxide is managed as an open-source engineering studio project. See:

- [Contributing](CONTRIBUTING.md)
- [Security](SECURITY.md)
- [Code of conduct](CODE_OF_CONDUCT.md)
- [Changelog](CHANGELOG.md)

Before making a meaningful change:

1. Create or select a GitHub issue.
2. Keep the scope small enough to review.
3. Update requirements, traceability, risk, and docs when behavior or evidence changes.
4. Add or update tests for code changes.
5. Run local validation.
6. Open a PR and let required CI pass before merge.

Do not add new dependencies, target toolchains, SDKs, hardware assumptions, unsafe FFI, GUI scope, DAQ scope, or certification claims without an explicit approval gate.

## License

License: MIT. See [LICENSE](LICENSE).
