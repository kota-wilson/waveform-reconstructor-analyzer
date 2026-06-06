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
- [Desktop User Flow Direction](#desktop-user-flow-direction)
- [Desktop Workflow Commands](#desktop-workflow-commands)
- [Native GUI Workflow Shell](#native-gui-workflow-shell)
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
- [Transforms, Filters, And ADC Quantization](#transforms-filters-and-adc-quantization)
- [Criteria And Measurements](#criteria-and-measurements)
- [Reports](#reports)
- [SVG Plotting](#svg-plotting)
- [Batch Analysis](#batch-analysis)
- [Heated Actuator Example](#heated-actuator-example)
- [Desktop Simulation Workflow](#desktop-simulation-workflow)
- [Portable Rule Packages](#portable-rule-packages)
- [Controller Deployment Package Format](#controller-deployment-package-format)
- [Qualification Evidence Reports](#qualification-evidence-reports)
- [Embedded And no_std Boundary](#embedded-and-nostd-boundary)
- [Validation Assets](#validation-assets)
- [Testing The Repository](#testing-the-repository)
- [Documentation Map](#documentation-map)
- [Contributing](#contributing)
- [License](#license)

## What FerrisOxide Does

FerrisOxide Signal currently implements this desktop workflow, with CSV and simulated-source paths implemented and live/realtime source work still future-gated:

```text
CSV waveform data or software-only simulated signal data
  -> signal source selection
  -> channel identification and labeling
  -> waveform model and metadata
  -> optional derived transforms, filters, and feature/event calculations
  -> reusable measurements
  -> pass/fail criteria
  -> evaluation run
  -> text, JSON, SVG, batch, package, and evidence artifacts
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

## Desktop User Flow Direction

The next product-flow direction is to make the desktop experience read as one sequence:

```text
run program
  -> use CSV data, simulated signals, or future-gated realtime input
  -> identify and label channels
  -> apply transforms and filters
  -> add pass/fail criteria for each relevant channel
  -> run evaluation
  -> review results
```

Current support and remaining gated gaps:

| Step | Current support | Remaining gated gaps |
|---|---|---|
| Run program | `ferrisoxide-signal` CLI and optional `ferrisoxide-gui --features native` workflow shell. | GUI packaging, installers, release publication, and broader product polish remain separately gated. |
| Choose source | CSV via `analyze`, `plot`, `batch`, `inspect-source`, and `evaluate-bundle`; software-only simulation via `simulate`, `inspect-source --source simulation`, and `evaluate-bundle --source simulation`. | Live/realtime DAQ remains separately gated. |
| Identify and label channels | TOML `[input]` mappings, simulation channel maps, `inspect-source`, and `scaffold-config`. | Hardware channel discovery and vendor SDK mapping remain future work. |
| Apply transforms and filters | `[[filters]]`, `[[feature_transforms]]`, `[[event_transforms]]`, `[[event_validations]]`, the transform catalog, and `workflow-template`. | Dependency-heavy or hardware-specific transforms remain gated where marked in the catalog. |
| Add pass/fail criteria | Legacy criteria, measurement-backed DSL criteria, event validations, scaffolded observed-bound starter criteria, and workflow templates. | Requirement approval and hardware calibration evidence remain outside the CLI scaffold. |
| Run evaluation | `analyze`, `simulate`, `batch`, and `evaluate-bundle`. | Runtime-loader execution remains future work. |
| Get results | Text, JSON, SVG overlays, batch summaries, rule packages, deployment fixtures, qualification evidence reports, bundle summaries, source summaries, config copies, and failure-triage notes. | Release publication and certification evidence remain separately gated. |

See [desktop user workflow guide](docs/desktop-user-workflow.md) for the implemented M38-M42 CLI workflow, [desktop user workflow roadmap](docs/desktop-user-workflow-roadmap.md) for the milestone rationale, and [native egui workflow shell roadmap](docs/egui-workflow-shell-roadmap.md) for the optional M43-M53 GUI shell. This direction does not implement vendor DAQ SDKs, live hardware acquisition, HAL/RTOS adapters, runtime loaders, release publication, installers, or certification evidence.

## Desktop Workflow Commands

The workflow-oriented commands are:

| Command | Purpose |
|---|---|
| `inspect-source` | Summarize CSV or software-only simulation source timing, headers, channel IDs, units, sample counts, observed ranges, warnings, and scope notes. |
| `scaffold-config` | Generate a starter TOML analysis config from a CSV source, including `[input]`, metadata, tolerances, transform placeholders, and observed-bound starter criteria. |
| `workflow-template` | Render TOML authoring starters for `supply-rail`, `switch-bounce`, `response-latency`, `sensor-cleanup`, `simulated-fault`, and `multi-channel` workflows. |
| `evaluate-bundle` | Write a predictable result directory with source summary, config copies, text/JSON reports or simulation workflow evidence, optional SVG, failure-triage notes, and `bundle-summary.json`. |

Start with the M42 fixture:

```bash
cargo run --quiet --bin ferrisoxide-signal -- inspect-source \
  --input examples/m42-desktop-workflow-waveform.csv \
  --format text
```

Then run a full bundle:

```bash
cargo run --quiet --bin ferrisoxide-signal -- evaluate-bundle \
  --input examples/m42-desktop-workflow-waveform.csv \
  --config examples/m42-desktop-workflow-config.toml \
  --output-dir m42-bundle \
  --plot
```

## Native GUI Workflow Shell

The first native GUI shell is optional and macOS-first. It calls the same shared workflow APIs as the CLI for source inspection, CSV header loading, config scaffolding, CSV analysis, evaluation bundles, result summaries, artifact lists, and interactive CSV plot review. It is a local workflow shell, not a packaged product or live DAQ application.

Build or launch it explicitly with the `native` feature:

```bash
cargo check -p ferrisoxide-gui --features native
cargo run -p ferrisoxide-gui --features native
```

Recommended CSV workflow:

1. Open `Source`, choose `CSV`, click the `CSV File` picker, select a local CSV, then click `Load Channels`.
2. Choose `Time Column`, `Time Unit`, enabled channel checkboxes, and each channel `Unit`, then click `Inspect`.
3. Open `Config`, click `Load From Source` to build channel sections, or click `Open TOML` to load an existing config.
4. In `Config`, select a channel, use `Add Filter` / `Add Criterion`, choose dropdown options, adjust numeric values, then click `Generate`.
5. Use `Save As` to choose a TOML path and write the generated config, or `Save` to update the active config file.
6. Open `Run`, click the `Output Dir` picker, choose `Overwrite` and `SVG Plot Artifact` as needed, then click `Analyze` or `Evaluate Bundle`.
7. Open `Results` to review the latest report preview, bundle outcome, output path, and artifact list.
8. Open `Plot`, click `Load Series`, choose plotted channel checkboxes, and choose `Fast`, `Balanced`, `Detailed`, or `Full` resolution for interactive rendering.

The Source page supports native CSV file selection. The Config page supports `Open TOML`, `Save As`, and `Save`. The Run page supports native output-directory selection. The Plot page has channel checkboxes beside `Load Series`, plus Fast/Balanced/Detailed/Full render modes for large loaded CSV series. Plot render optimization is GUI-only: raw loaded data remains unchanged for analysis/export.

See [desktop user workflow guide](docs/desktop-user-workflow.md#native-gui-button-and-control-reference) for the complete page-by-page reference covering every visible native GUI button, selector, checkbox, numeric control, picker, and display area. Current fixture-simulation path fields remain manual text fields. The GUI does not add installers, app signing, live DAQ, vendor SDKs, hardware acquisition, runtime-loader execution, release publication, or certification evidence.

## Why This Is Useful

FerrisOxide is useful when signal review needs to be repeatable, inspectable, and automation-friendly.

| Benefit | What it means in practice |
|---|---|
| Repeatable analysis | The same CSV and same TOML config produce the same report. |
| Reviewable criteria | Requirements are written as explicit criteria instead of being hidden in a spreadsheet or one-off script. |
| Evidence-rich failures | Reports include measured value, required value, sample index, timestamp, channel, and reason. |
| Human and machine output | Text is readable in a terminal; JSON is suitable for golden tests and automation. |
| Visual evidence | SVG plots can show waveform traces, threshold overlays, and failed-criterion markers. |
| Raw-data preservation | Implemented transforms produce derived waveforms instead of mutating source data. |
| Desktop-to-embedded direction | Rule schema and rule engine work is structured so future runtimes can share semantics. |
| Scope discipline | The repo explicitly separates software validation evidence from hardware qualification or certification claims. |

## Current Status

FerrisOxide has passed the local MVP-exit review for its desktop software workflow. It is not a finished commercial product, not a certified test system, not a live DAQ system, and not a live controller runtime.

Implemented today:

- Rust Cargo workspace.
- CSV waveform loading.
- Named time and channel mapping.
- Waveform metadata for units, source, sample count, sample interval, nominal sample rate, lineage, legacy transform history, structured transform steps, validation context, and tolerance policy.
- Moving-average, moving-median, first-order low-pass, high-pass baseline correction, ideal ADC quantization, pointwise, baseline, smoothing, detrending, Hampel, and spike-cleanup transforms.
- Transform architecture docs that classify current support, planned transform families, and runtime compatibility boundaries.
- Measurement primitives for extrema, state transitions, pulse width, stable-state duration, transient duration, and rise/fall time.
- Criteria for min/max voltage, state transitions, response latency, pulse width, transient events, stable-state duration, and rise/fall time.
- TOML config parsing with clear errors for invalid config.
- Text and JSON analysis reports.
- Local batch analysis over multiple CSV/config pairs with per-run reports and deterministic summary output.
- Desktop source inspection for CSV and fixture-backed simulation sources.
- CSV config scaffolding with starter channel criteria and transform placeholders.
- Workflow template rendering for common signal-conditioning and validation use cases.
- Evaluation bundle output for CSV and software-only simulation workflows.
- Optional native egui workflow shell for source/config/run/results/plot review.
- Exact golden JSON tests.
- SVG waveform plotting with optional third-axis 3D line plots.
- 2D SVG evidence overlays from report evidence.
- Portable rule package schema, validation, manifest, checksum, and export command, including package-export support for the linear pointwise `offset`, `gain`, and `invert` software transform subset.
- Production control and test verification config schema boundaries for future controller-in-the-loop workflows.
- Virtual controller simulation engine over deterministic abstract sample frames.
- Fixture/test-double DAQ input abstraction for deterministic sample sources.
- Host-checkable controller I/O abstraction for portable input/output boundaries.
- Desktop simulation workflow that loads production control config, test verification config, a channel map, and fixture CSV input.
- RTOS/controller deployment package format schema, validator, manifest, required artifact roles, checksum wording, and example package fixture.
- Manifest-level production, test-verification, and signal-validation mode profiles that reject mixed production/test behavior.
- Controller config and behavior parity test comparing desktop simulation evidence with embedded-compatible borrowed-rule evidence over the same configs, channel map, and waveform input.
- Qualification evidence report schema with exact JSON fixture tests linking configs, channel map, simulation trace, criteria evidence, deployment metadata, checksum evidence, timestamp, and non-certification scope notes.
- `no_std` signal, measurement, rule-engine, and embedded-boundary crates.
- Desktop-vs-embedded-compatible parity tests for rule evidence.
- Software-only heated actuator qualification scenario.
- Source-of-truth transform catalog with CLI inspection through `ferrisoxide-signal transforms --format text` or `--format json`.
- Comprehensive desktop waveform-conditioning suite covering data cleaning, timing repair, pointwise/nonlinear transforms, smoothing, detrending, baseline correction, standard frequency filters, resampling/timing alignment, envelope/energy/calculus, statistics/correlation, spectrum/window/time-frequency feature records, deterministic fault injection, ADC/DAC simulation, multi-channel math, software sensor conversions, vibration conditioning, and control-signal conditioning.
- Local M25-M36 comprehensive-suite closure artifacts for catalog completeness, config/searchability, validation corpus, benchmark-readiness, package/runtime compatibility, release-readiness messaging, community notes, and retrospective evidence.

Post-MVP or future-gated:

- Runtime loaders.
- Raspberry Pi 5 bare-metal runtime work.
- Optional Pico 2 micro-runtime profile.
- Automated config/report drift checks.
- Broader validation-corpus and benchmark refresh automation.
- Advanced phase/gain matching, acoustic analysis packs, advanced sensor calibration packs, optimized FFT/Hilbert/polyphase/exact elliptic work, broader package/runtime transform exposure, external release publication, hardware validation, and certification claims.
- GUI packaging, live/realtime source acquisition, vendor SDK integration, and hardware channel discovery.

## What Is In Scope

The current repo focuses on engineering signal analysis from local files:

- Local CSV input.
- Local TOML config.
- Local text/JSON reports.
- Local SVG plots.
- Local batch reports and summaries.
- Local rule-package export.
- Optional native GUI workflow shell for local desktop review.
- Software-only validation fixtures.
- Rust library and CLI development.
- `no_std` reusable primitives where they make architectural sense.

## What Is Out Of Scope

The repo intentionally does not claim or implement:

- Full packaged GUI product.
- Live DAQ integration.
- Vendor DAQ SDK support.
- Hardware control.
- HAL or RTOS SDK adapters.
- Hardware-in-the-loop execution.
- Hardware validation.
- Production RTOS runtime.
- Certified aerospace validation.
- Safety certification.
- Cloud storage.
- Hosted batch service, scheduler, or database-backed workflow.
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
                                  desktop simulation and batch workflows.

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

crates/ferrisoxide-deployment/   Deployment package manifest and
                                  qualification evidence report schemas.

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
                                  Includes `batch-analysis.toml`.

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

### Desktop Evaluation Bundle Flow

```text
CSV input + analysis config
  or fixture CSV + control config + verification config + channel map
  -> source summary
  -> existing analysis or simulation workflow
  -> report or simulation evidence
  -> optional SVG evidence
  -> failure triage notes
  -> bundle-summary.json
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
  ferrisoxide-signal batch --manifest examples/batch-analysis.toml --output-dir batch-output --format json
  ferrisoxide-signal transforms --format text
  ferrisoxide-signal inspect-source --input examples/basic-waveform.csv --format text
  ferrisoxide-signal inspect-source --source simulation --input tests/e2e/heated_actuator/input/passing_run.csv --channel-map examples/simulation/heated-actuator-channel-map.toml --format json
  ferrisoxide-signal scaffold-config --input examples/basic-waveform.csv --output analysis.toml
  ferrisoxide-signal workflow-template --use-case switch-bounce --format toml
  ferrisoxide-signal evaluate-bundle --input examples/basic-waveform.csv --config examples/basic-config.toml --output-dir evaluation-bundle --plot
  ferrisoxide-signal evaluate-bundle --source simulation --input tests/e2e/heated_actuator/input/passing_run.csv --control-config examples/control-config/production-control-config.toml --verification-config examples/test-verification-config/test-verification-config.toml --channel-map examples/simulation/heated-actuator-channel-map.toml --output-dir simulation-bundle
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

To exercise the complete desktop workflow fixture, inspect the source and then write a bundle:

```bash
cargo run --quiet --bin ferrisoxide-signal -- inspect-source \
  --input examples/m42-desktop-workflow-waveform.csv \
  --format text

cargo run --quiet --bin ferrisoxide-signal -- evaluate-bundle \
  --input examples/m42-desktop-workflow-waveform.csv \
  --config examples/m42-desktop-workflow-config.toml \
  --output-dir m42-bundle \
  --plot
```

The bundle contains `source-summary.json`, `config.toml`, `report.json`, `report.txt`, `failure-triage.md`, optional `evidence.svg`, and `bundle-summary.json`.

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

## Transforms, Filters, And ADC Quantization

FerrisOxide currently implements ordered pre-criteria transforms for smoothing, filtering, quantization, pointwise correction, baseline handling, data cleaning, timing repair, frequency filtering, resampling, envelope/energy/calculus, statistics, deterministic fault/ADC-DAC simulation, multi-channel math, software sensor conversion, vibration conditioning, control-signal conditioning, and event/validation analysis. Waveform transforms are exposed through the existing `[[filters]]` config table for compatibility. Feature calculations use `[[feature_transforms]]`. Event analysis uses additive `[[event_transforms]]` and `[[event_validations]]` tables. The architecture treats all of them as transform capabilities with auditable derived evidence while preserving raw input.

The source of truth for support status is the transform catalog. Inspect it with:

```bash
cargo run --quiet --bin ferrisoxide-signal -- transforms --format text
```

The catalog currently reports 219 entries, including implemented desktop support, package-export decisions, runtime profile boundaries, dependency-gated entries, and future-gated entries such as `split_by_event`. See [analog transform taxonomy](docs/analog-transform-taxonomy.md), [comprehensive filter and signal conditioning roadmap](docs/comprehensive-filter-signal-conditioning-roadmap.md), [transform catalog](docs/transform-catalog.md), [config reference](docs/config-reference.md), [transform package compatibility](docs/transform-package-compatibility.md), [structured transform metadata](docs/structured-transform-metadata.md), [current transform metadata mapping](docs/current-transform-metadata-mapping.md), and [transform runtime profile compatibility](docs/transform-runtime-profile-compatibility.md) for vocabulary, support status, metadata, package compatibility, and runtime-boundary direction.

| Transform | Config type | Purpose |
|---|---|---|
| Offset | `offset` | Add a configured value in signal units. |
| Gain | `gain` | Multiply samples by a configured scalar. |
| Invert | `invert` | Flip signal polarity. |
| Clamp | `clamp` | Bound samples between configured minimum and maximum values. |
| Deadband | `deadband` | Set small values around zero to zero. |
| DC removal | `dc_remove` | Subtract the channel mean over the full waveform. |
| Baseline subtraction | `baseline_subtract` | Subtract a configured baseline value in signal units. |
| High-pass baseline correction | `high_pass_baseline` | Reduce slow baseline wander with a causal first-order high-pass recurrence over a strictly increasing time axis. |
| Moving average | `moving_average` | Smooth samples with a trailing window that includes the current sample. |
| Moving median | `moving_median` | Smooth spikes with a trailing median window. |
| Weighted moving average | `weighted_moving_average` | Smooth samples with a trailing positive-weight window. |
| Exponential moving average | `exponential_moving_average` | Smooth samples with a causal single-pole recurrence. |
| Boxcar smoothing | `boxcar_smoothing` | Smooth samples offline with a centered uniform window. |
| Gaussian smoothing | `gaussian_smoothing` | Smooth samples offline with a centered Gaussian kernel. |
| Savitzky-Golay smoothing | `savitzky_golay` | Smooth samples offline with a local polynomial fit. |
| Centered moving median | `centered_moving_median` | Smooth samples offline with a centered median window. |
| Rolling mean baseline | `rolling_mean_baseline` | Subtract a causal trailing mean baseline. |
| Rolling median baseline | `rolling_median_baseline` | Subtract a causal trailing median baseline. |
| Linear detrend | `linear_detrend` | Remove a fitted linear trend over a strictly increasing time axis. |
| Polynomial detrend | `polynomial_detrend` | Remove a fitted polynomial trend over a strictly increasing time axis. |
| Hampel filter | `hampel_filter` | Replace robust outliers using centered median/MAD windows. |
| Spike removal | `spike_remove` | Replace thresholded spikes using a centered median window. |
| First-order low-pass | `low_pass` | Apply a simple low-pass smoothing model over a strictly increasing time axis. |
| Ideal ADC quantization | `adc_quantize` | Clip and snap analog values to ideal ADC code levels before criteria evaluation. |
| Schmitt trigger | `schmitt_trigger` in `[[event_transforms]]` | Convert analog samples into a hysteretic state trace and state-transition records. |
| Edge extraction | `edge_extraction` in `[[event_transforms]]` | Emit rising/falling edge records from a state trace. |
| Bounce detection | `bounce_detection` in `[[event_transforms]]` | Emit aggregate bounce evidence with count, duration, and linked source transitions. |
| Event validation | `missing_pulse`, `extra_pulse`, `dwell_time`, `timeout` in `[[event_validations]]` | Emit pass/fail validation records linked to event evidence. |

Timing and unit assumptions remain transform-specific:

- Moving average uses a sample-count window and does not require a nominal sample rate.
- Moving median uses a trailing sample-count window, records nonlinear phase behavior, and does not require a nominal sample rate.
- M28 causal smoothers and rolling baselines record phase delay; centered smoothing, detrending, Hampel, and spike cleanup are offline-only derived analysis views.
- Baseline, detrending, Hampel, and spike-cleanup transforms can hide real failures if criteria are interpreted without raw waveform lineage.
- Low-pass uses a cutoff in hertz and requires a strictly increasing time axis.
- High-pass baseline correction uses a cutoff in hertz, requires a strictly increasing time axis, records phase delay, and remains a derived software transform rather than calibrated drift removal.
- ADC quantization uses a configured voltage range and outputs volts; it is not a calibrated physical ADC model.
- DC removal is offline-only because it subtracts the mean of the full waveform.
- Event validations contribute to report-level pass/fail outcome and remain software-only evidence.
- Portable rule-package export supports `moving_average`, `low_pass`, `adc_quantize`, `offset`, `gain`, and `invert`. `offset` and `gain` are software transforms only, not calibration or sensor-accuracy evidence. Runtime-loader implementation and broader transform exposure remain separately gated.

Example ADC quantization config:

```toml
[[filters]]
type = "adc_quantize"
bits = 8
min_v = 0.0
max_v = 5.0
```

Example M11 transform config:

```toml
[[filters]]
type = "offset"
offset_v = -2.0

[[filters]]
type = "gain"
gain = 2.0

[[filters]]
type = "moving_median"
window_samples = 3
```

Example M14 high-pass baseline config:

```toml
[[filters]]
type = "high_pass_baseline"
cutoff_hz = 50.0
```

Example M12 event validation config:

```toml
[[event_transforms]]
id = "switch_state"
type = "schmitt_trigger"
channel = "switch_v"
on_threshold_v = 3.0
off_threshold_v = 2.0
initial_state = "low"

[[event_transforms]]
id = "switch_edges"
type = "edge_extraction"
channel = "switch_v"

[[event_validations]]
id = "must_rise"
type = "missing_pulse"
channel = "switch_v"
direction = "rising"
expected_count = 1
```

Run the switch/bounce fixture:

```bash
cargo run -p ferrisoxide-cli --bin ferrisoxide-signal -- analyze \
  --input examples/switch-bounce-waveform.csv \
  --config examples/m12-event-validation-config.toml \
  --format json
```

Run the ADC example:

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

## Batch Analysis

FerrisOxide can run repeated local analyses from a batch manifest. This is a desktop file workflow for CSV/config pairs, not a live DAQ runner, scheduler, hosted service, or hardware workflow.

```bash
cargo run --quiet --bin ferrisoxide-signal -- batch \
  --manifest examples/batch-analysis.toml \
  --output-dir target/ferrisoxide-batch-example \
  --format json
```

The command writes one report per completed run plus a `batch-summary.json` file. A run can have status `pass`, `fail`, or `error`; the batch summary fails if any run fails or errors. Existing outputs are not overwritten unless `--overwrite` is passed.

See [batch analysis workflow](docs/batch-analysis-workflow.md), [config reference](docs/config-reference.md), and [artifact contract](docs/artifact-contract.md).

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
- No live hardware simulation controls; the optional native GUI shell can run fixture simulation bundles through shared workflow APIs.
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

Current package-export filter support is limited to `moving_average`, `low_pass`, `adc_quantize`, `offset`, `gain`, and `invert`. Nonlinear, baseline, high-pass baseline, moving-median, M26 cleaning/timing, M27 pointwise/nonlinear, M28 smoothing/baseline, M29 frequency-filter, M30 resampling/timing, and M31 envelope/calculus transforms are rejected for rule-package export until a future compatibility milestone approves their schema, runtime semantics, and validation evidence. M31 `feature_transforms` are analysis evidence records and are not exported as rule-package filters.

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

See [rule package format](docs/rule-package-format.md), [transform package compatibility](docs/transform-package-compatibility.md), [runtime loader design gate](docs/runtime-loader-design-gate.md), and [ADR-004 portable rule package architecture](decisions/ADR-004-portable-rule-package-architecture.md).

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

See [RTOS deployment package format](docs/rtos-deployment-package-format.md), [controller operating modes](docs/controller-operating-modes.md), and [qualification evidence report](docs/qualification-evidence-report.md).

## Qualification Evidence Reports

FerrisOxide now defines a qualification evidence report format for controller-in-the-loop workflows. The report is a software evidence artifact, not a certification artifact. It is designed to make one simulated qualification run auditable by linking:

- production control config name, version, path, and checksum,
- test verification config name, version, path, and checksum,
- channel map path and checksum,
- controller simulation trace frames,
- per-criterion pass/fail measurement evidence,
- deployment package metadata and mode profiles,
- checksum evidence for package artifacts,
- generated timestamp,
- explicit non-certification scope notes.

Example report:

```text
examples/deployment-package/heated-actuator/qualification-report.json
```

The exact fixture is tested by parsing, validating, serializing, and comparing the JSON byte-for-byte:

```bash
cargo test -p ferrisoxide-deployment example_qualification_evidence_report_validates_and_matches_exact_json
```

See [qualification evidence report](docs/qualification-evidence-report.md).

## Embedded And no_std Boundary

FerrisOxide has embedded-oriented crates, but the project is not yet an embedded runtime product.

| Crate | Boundary |
|---|---|
| `ferrisoxide-signal` | `#![no_std]` signal primitives: fixed buffers, streaming ingestion, threshold checks, transient events. |
| `ferrisoxide-measurements` | `#![no_std]` measurement primitives over slices. |
| `ferrisoxide-rule-engine` | `#![no_std]` rule execution semantics over caller-provided time/sample slices. |
| `ferrisoxide-embedded` | `#![no_std]` adapter traits for sample sources, event sinks, and runtime hooks. |
| `ferrisoxide-workflow` | Shared desktop workflow APIs for CLI and GUI source/config/run/result behavior. |
| `ferrisoxide-gui` | Optional native egui workflow shell; not a live DAQ, runtime loader, packaged app, or certification artifact. |
| `ferrisoxide-control-schema` | Production control config schema for future controller-in-the-loop workflows; not a runtime executor. |
| `ferrisoxide-verification-schema` | Test verification config schema for qualification criteria, timing windows, evidence settings, and report settings; not a criteria executor. |
| `ferrisoxide-deployment` | Deployment package manifest schema and validator for future controller/runtime package workflows; not an RTOS loader. |

Desktop-only concerns stay out of those crates:

- CSV parsing.
- TOML parsing.
- JSON report rendering.
- SVG plotting.
- File I/O.
- Native GUI runtime dependencies.
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
| `tests/controller_parity/` | Controller-in-the-loop config and behavior parity notes. |

Useful targeted checks:

```bash
cargo test -p ferrisoxide-core --test criteria_engine
cargo test -p ferrisoxide-core --test heated_actuator
cargo test -p ferrisoxide-core --test rule_parity
cargo test -p ferrisoxide-cli controller_config_and_behavior_paths_match_portable_parity_evidence
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
- [FerrisOxide system overview diagram](docs/architecture/ferrisoxide-overview.md)
- [MVP usage sketch](docs/usage-mvp.md)
- [Report schema](docs/report-schema.md)
- [Config reference](docs/config-reference.md)
- [Artifact contract](docs/artifact-contract.md)
- [Batch analysis workflow](docs/batch-analysis-workflow.md)
- [Desktop user workflow and native GUI button reference](docs/desktop-user-workflow.md)
- [Criteria DSL](docs/criteria-dsl.md)
- [Criteria DSL migration](docs/criteria-dsl-migration.md)
- [Analog transform taxonomy](docs/analog-transform-taxonomy.md)
- [Transform capability model](docs/transform-capability-model.md)
- [Structured transform metadata](docs/structured-transform-metadata.md)
- [Current transform metadata mapping](docs/current-transform-metadata-mapping.md)
- [Event validation transforms](docs/event-validation-transforms.md)
- [Transform runtime profile compatibility](docs/transform-runtime-profile-compatibility.md)
- [Transform package compatibility](docs/transform-package-compatibility.md)
- [Transform catalog](docs/transform-catalog.md)
- [Runtime loader design gate](docs/runtime-loader-design-gate.md)
- [Next milestones roadmap](docs/next-milestones-roadmap.md)
- [MVP exit roadmap](docs/mvp-exit-roadmap.md)
- [MVP exit readiness report](docs/mvp-exit-readiness-report.md)
- [M15-M20 MVP exit pipeline report](docs/m15-m20-mvp-exit-pipeline-report.md)
- [M21-M24 runtime path pipeline report](docs/m21-m24-runtime-path-pipeline-report.md)
- [Comprehensive filter and signal conditioning roadmap](docs/comprehensive-filter-signal-conditioning-roadmap.md)
- [M36 comprehensive suite closure report](docs/m36-comprehensive-suite-closure-pipeline-report.md)
- [Desktop user workflow roadmap](docs/desktop-user-workflow-roadmap.md)
- [Native egui workflow shell roadmap](docs/egui-workflow-shell-roadmap.md)
- [M43-M53 native egui workflow shell pipeline report](docs/m43-m48-egui-workflow-shell-pipeline-report.md)
- [Post-MVP roadmap](docs/post-mvp-roadmap.md)
- [v0.8.0 transform architecture proposal](docs/v0.8.0-transform-architecture-milestone-proposal.md)
- [v0.9.0 pointwise/windowed transform proposal](docs/v0.9.0-pointwise-windowed-transform-mvp-milestone-proposal.md)
- [v0.10.0 event/validation transform proposal](docs/v0.10.0-event-validation-transform-milestone-proposal.md)
- [Next milestones issue planning report](docs/next-milestones-issue-planning-report.md)
- [Measurements](docs/measurements.md)
- [Filter behavior](docs/filter-behavior.md)
- [ADC quantization](docs/adc-quantization.md)
- [Time axis and tolerances](docs/time-axis-and-tolerances.md)
- [SVG plotting](docs/plotting.md)
- [Rule package format](docs/rule-package-format.md)
- [Heated actuator qualification suite](docs/heated-actuator-qualification-suite.md)
- [Environmental test use cases](docs/environmental-test-use-cases.md)
- [Validation corpus index](docs/validation-corpus-index.md)
- [Embedded roadmap](docs/embedded-roadmap.md)
- [Platform targets](docs/platform-targets.md)
- [Controller-in-the-loop workflow](docs/controller-in-the-loop-workflow.md)
- [Production control config schema](docs/control-config-schema.md)
- [Test verification config schema](docs/test-verification-config-schema.md)
- [Desktop simulation workflow](docs/desktop-simulation-workflow.md)
- [RTOS deployment package format](docs/rtos-deployment-package-format.md)
- [Controller operating modes](docs/controller-operating-modes.md)
- [Controller config parity](docs/controller-config-parity.md)
- [Qualification evidence report](docs/qualification-evidence-report.md)
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
