# Desktop User Workflow

Date: 2026-06-02

Status: Implemented CLI workflow guide for M38 through M42. This is a local desktop workflow over CSV files and software-only fixture simulation. It does not implement a GUI, live DAQ, vendor SDK, hardware input, HAL/RTOS adapter, runtime loader, release publication, or certification evidence.

## Flow

The intended desktop sequence is:

```text
run FerrisOxide Signal
  -> choose CSV data or software-only simulated signals
  -> inspect source timing and channels
  -> label channels through TOML config or simulation channel map
  -> apply transforms, filters, features, events, and validations
  -> add pass/fail criteria
  -> run evaluation
  -> review one output bundle
```

## Source Inspection

Inspect a CSV source before writing criteria:

```bash
cargo run --quiet --bin ferrisoxide-signal -- inspect-source \
  --input examples/m42-desktop-workflow-waveform.csv \
  --format text
```

For machine-readable inspection:

```bash
cargo run --quiet --bin ferrisoxide-signal -- inspect-source \
  --input examples/m42-desktop-workflow-waveform.csv \
  --format json
```

The inspection reports:

- source mode,
- input path,
- time column and unit,
- sample count,
- first and last timestamps,
- duration,
- sample interval summary,
- nominal sample rate when time is in seconds,
- headers,
- selected source columns,
- channel IDs, units, and observed min/max values,
- warnings and scope notes.

Simulation source inspection uses the existing channel map:

```bash
cargo run --quiet --bin ferrisoxide-signal -- inspect-source \
  --source simulation \
  --input tests/e2e/heated_actuator/input/passing_run.csv \
  --channel-map examples/simulation/heated-actuator-channel-map.toml \
  --format json
```

Live/realtime source names are reserved but intentionally rejected until separate DAQ, dependency, security, environment, hardware, and V&V gates approve that work.

## Config Scaffolding

Create a starter analysis config from inspected CSV data:

```bash
cargo run --quiet --bin ferrisoxide-signal -- scaffold-config \
  --input examples/m42-desktop-workflow-waveform.csv \
  --output m42-analysis.toml
```

The scaffold includes:

- `[input]` time and channel mapping,
- units,
- metadata,
- tolerances,
- commented transform placeholders,
- per-channel observed min/max starter criteria.

Review scaffolded thresholds before treating them as requirements. Observed bounds are a starting point, not proof that the waveform meets an engineering requirement.

## Authoring Templates

Render a TOML starter for common use cases:

```bash
cargo run --quiet --bin ferrisoxide-signal -- workflow-template \
  --use-case switch-bounce \
  --format toml
```

Supported templates:

- `supply-rail`
- `switch-bounce`
- `response-latency`
- `sensor-cleanup`
- `simulated-fault`
- `multi-channel`

Templates use the current config surfaces:

- `[[filters]]`
- `[[feature_transforms]]`
- `[[event_transforms]]`
- `[[event_validations]]`
- `[[criteria]]`

Use the transform catalog while authoring:

```bash
cargo run --quiet --bin ferrisoxide-signal -- transforms --format text
```

## Evaluation Bundle

Run the M42 CSV workflow fixture into one output directory:

```bash
cargo run --quiet --bin ferrisoxide-signal -- evaluate-bundle \
  --input examples/m42-desktop-workflow-waveform.csv \
  --config examples/m42-desktop-workflow-config.toml \
  --output-dir m42-bundle \
  --plot
```

The CSV bundle writes:

- `source-summary.json`
- `config.toml`
- `report.json`
- `report.txt`
- `failure-triage.md`
- `evidence.svg` when `--plot` is present
- `bundle-summary.json`

Simulation bundles use the fixture simulation inputs:

```bash
cargo run --quiet --bin ferrisoxide-signal -- evaluate-bundle \
  --source simulation \
  --input tests/e2e/heated_actuator/input/passing_run.csv \
  --control-config examples/control-config/production-control-config.toml \
  --verification-config examples/test-verification-config/test-verification-config.toml \
  --channel-map examples/simulation/heated-actuator-channel-map.toml \
  --output-dir simulation-bundle
```

The simulation bundle writes:

- `source-summary.json`
- `simulation-workflow.json`
- `simulation-workflow.txt`
- `production-control-config.toml`
- `test-verification-config.toml`
- `channel-map.toml`
- `failure-triage.md`
- `bundle-summary.json`

`evaluate-bundle` refuses to overwrite existing artifacts unless `--overwrite` is supplied. Keep raw input files outside the bundle; the bundle records source summaries and config copies while preserving source data in place.

## Scope Notes

Filtering, smoothing, resampling, baseline correction, and simulation transforms create derived software evidence. They can make an analyzed signal look cleaner than raw source data. Review raw source inspection, transform lineage, criteria thresholds, and failure-triage notes before using a bundle as engineering evidence.

This workflow is desktop software validation evidence only. It is not live DAQ evidence, hardware qualification, RTOS runtime evidence, or certification evidence.

## Hand-Off Note

Role: Documentation Engineer / Verification and Validation Engineer
Goal: Document the implemented M38-M42 desktop user workflow.
Files changed: `docs/desktop-user-workflow.md`, README, examples, CLI tests, roadmap/state/traceability/risk artifacts, and validation log.
Checks run: Pending final M42 validation.
Status: Implemented locally; final validation pending.
Known gaps: GUI, live/realtime DAQ, SDKs, hardware acquisition, HAL/RTOS adapters, runtime loaders, release publication, and certification evidence remain future-gated.
Next recommended step: Run final validation and merge through the standard repository process.
