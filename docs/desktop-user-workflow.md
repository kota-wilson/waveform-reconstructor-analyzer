# Desktop User Workflow

Date: 2026-06-03

Status: Implemented CLI workflow guide for M38 through M42. This is a local desktop workflow over CSV files and software-only fixture simulation. M43-M53 plus WRA-RQ-139 add a separate optional native egui shell over the same workflow APIs. Neither path implements live DAQ, vendor SDK, hardware input, HAL/RTOS adapter, runtime loader, release publication, packaging, or certification evidence.

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

## Native GUI Workflow

The optional native GUI shell runs the same software-only workflow APIs as the CLI. It is intended for local CSV review, config authoring, evaluation bundle runs, result review, and interactive plot inspection. It is macOS-first in this local slice and is not a packaged product, live DAQ workflow, hardware acquisition tool, runtime loader, release artifact, or certification artifact.

Launch it from the repository root:

```bash
cargo run -p ferrisoxide-gui --features native
```

The recommended CSV path is:

```text
Source -> Config -> Run -> Results -> Plot
```

1. Open `Source`, choose `CSV`, pick a CSV file, and click `Load Channels`.
2. Choose the time column, time unit, enabled channels, and per-channel units.
3. Click `Inspect` to review the loaded source summary.
4. Open `Config`, either click `Load From Source` to build a config from selected channels or click `Open TOML` to load an existing config file.
5. Add per-channel filters and criteria through dropdowns and numeric controls, then click `Generate`.
6. Use `Save As` or `Save` to write the TOML config.
7. Open `Run`, choose an output directory, and click `Analyze` or `Evaluate Bundle`.
8. Open `Results` to review the report preview and bundle artifact list.
9. Open `Plot`, click `Load Series`, choose plotted channels and render resolution, then inspect the interactive plot.

The fixture-simulation path is also exposed, but its path fields are currently manual text fields. Live/realtime DAQ, vendor SDKs, drivers, hardware acquisition, packaged installers, release publication, runtime-loader execution, and certification evidence remain future-gated.

## Native GUI Button And Control Reference

### Navigation

| Button/control | What it does |
|---|---|
| `Source` | Opens the source selection, CSV header loading, channel assignment, and inspection page. |
| `Config` | Opens the config builder, TOML open/save controls, and generated TOML preview. |
| `Run` | Opens analysis and evaluation bundle controls. |
| `Results` | Opens the latest report preview and bundle artifact list. |
| `Plot` | Opens interactive CSV plot-series loading and rendering controls. |
| Status message | Shows `Idle`, success, or failure text for the latest GUI action. |

### Source Page

| Button/control | What it does |
|---|---|
| `CSV` | Selects local CSV source mode. |
| `Simulation` | Selects fixture-simulation source mode. Current simulation path fields remain manual text entry. |
| `CSV File` picker | Opens a native file dialog and stores the selected CSV input path. |
| `Load Channels` | Reads CSV headers from the selected file and populates time/channel selectors. |
| `Time Column` dropdown | Chooses which loaded CSV header is the time axis. |
| `Time Unit` dropdown | Chooses the time-axis unit used for inspection, config generation, analysis, and plotting in CSV or Simulation mode. |
| `Use` checkbox | Enables or disables each non-time CSV header as a signal channel. |
| Per-channel `Unit` dropdown | Chooses the displayed/generated unit for an enabled channel. |
| `Inspect` | Runs source inspection and writes the source summary into the `Inspection` text area. |
| `Inspection` text area | Shows the latest source inspection output. |
| Simulation `Input` field | Sets the fixture-simulation input path when `Simulation` is selected. |
| Simulation `Time Column` field | Sets the simulation input time column. |
| Simulation `Channels` field | Sets comma-separated simulation source channels. |
| Simulation `Signal Unit` field | Sets the simulation signal unit. |
| Simulation `Channel Map` field | Sets the simulation channel-map TOML path. |

### Config Page

| Button/control | What it does |
|---|---|
| `Config File` display | Shows the selected TOML config file name, or `No config file selected` before a config path is chosen. |
| `Open TOML` | Opens a native file dialog, loads an existing TOML config, and makes it the active config path. |
| `Save As` | Opens a native save dialog and immediately writes the current TOML preview to the selected path. |
| `Load From Source` | Builds per-channel config sections from the enabled Source-page channels. |
| `Generate` | Regenerates TOML from the current typed channel/filter/criterion builder state. |
| `Save` | Writes the current TOML preview to the active config path; if no path is selected, it opens the save dialog first. |
| Channel list item | Selects which loaded channel's filters and criteria are shown for editing. |
| `Add Filter` | Adds a filter row for the selected channel. |
| Filter type dropdown | Chooses the selected channel filter/action from supported same-time-axis options. |
| Filter numeric controls | Set numeric filter parameters such as window, cutoff, gain, offset, threshold, or range. |
| Filter `Remove` | Removes that filter row from the selected channel. |
| `Add Criterion` | Adds a pass/fail criterion row for the selected channel. |
| Criterion type dropdown | Chooses the criterion kind. |
| Criterion dropdowns | Choose non-numeric options such as state, direction, event kind, normalize mode, or source channel. |
| Criterion numeric controls | Set thresholds, durations, counts, windows, and latency limits. |
| Criterion `Remove` | Removes that criterion row from the selected channel. |
| `Config TOML` preview | Shows generated or opened TOML text. It is read-only in the current GUI. |

### Run Page

| Button/control | What it does |
|---|---|
| `Output Dir` picker | Opens a native folder dialog and stores the selected evaluation output directory. |
| `Overwrite` checkbox | Allows `Evaluate Bundle` to overwrite existing output artifacts in the selected directory. |
| `SVG Plot Artifact` checkbox | Includes `evidence.svg` in evaluation bundles when supported by the selected workflow. |
| `Analyze` | Runs CSV analysis with the selected input and active config, then writes the report preview to Results state. |
| `Evaluate Bundle` | Writes a deterministic output bundle into the selected output directory. |
| Simulation `Control Config` field | Sets the production control config path for fixture-simulation bundle runs. |
| Simulation `Verification Config` field | Sets the test verification config path for fixture-simulation bundle runs. |
| Simulation `Channel Map` field | Sets the simulation channel-map TOML path for bundle runs. |
| Simulation `Mode` field | Sets the fixture-simulation mode string when needed by future-compatible simulation state. |

### Results Page

The Results page currently has no action buttons.

| Display area | What it shows |
|---|---|
| `Outcome` display | Shows the latest evaluation bundle pass/fail outcome when a bundle has been run. |
| `Output` display | Shows the latest evaluation bundle output directory. |
| Artifact list | Shows the artifact names produced by the latest evaluation bundle. |
| `Report` preview | Shows the latest text report produced by `Analyze` or `Evaluate Bundle`. |

### Plot Page

| Button/control | What it does |
|---|---|
| `Load Series` | Loads CSV plot series from the selected Source input and active config or selected Source channels. |
| `Resolution` dropdown | Chooses the render budget: `Fast`, `Balanced`, `Detailed`, or `Full`. Non-Full modes downsample only the rendered plot points. |
| Channel checkbox | Shows or hides each selected Source-derived channel in the interactive plot. |
| Render summary | Shows the current rendered-point summary when plot data has been loaded. |
| Interactive plot | Displays loaded series using viewport-aware rendering, min/max decimation, cached render points, and plot pyramids for large data. |

## Scope Notes

Filtering, smoothing, resampling, baseline correction, and simulation transforms create derived software evidence. They can make an analyzed signal look cleaner than raw source data. Review raw source inspection, transform lineage, criteria thresholds, and failure-triage notes before using a bundle as engineering evidence.

This workflow is desktop software validation evidence only. It is not live DAQ evidence, hardware qualification, RTOS runtime evidence, or certification evidence.

## Hand-Off Note

Role: Documentation Engineer / Verification and Validation Engineer
Goal: Document the implemented M38-M42 desktop user workflow.
Files changed: `docs/desktop-user-workflow.md`, README, examples, CLI tests, roadmap/state/traceability/risk artifacts, and validation log.
Checks run: See `docs/validation-log.md`.
Status: Implemented and merged through PR #177; optional native GUI shell implemented and merged through PR #190 for M43-M53 plus WRA-RQ-139.
Known gaps: GUI packaging, live/realtime DAQ, SDKs, hardware acquisition, HAL/RTOS adapters, runtime loaders, release publication, and certification evidence remain future-gated.
Next recommended step: Review the M43-M53 GUI shell, WRA-RQ-139 Run-page picker, and protected CI before closing tracking issues.
