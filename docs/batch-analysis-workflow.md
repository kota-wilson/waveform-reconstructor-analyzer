# Desktop Batch Analysis Workflow

Date: 2026-06-01

Status: M17 implemented desktop workflow. This is a local file-based batch runner. It is not live DAQ, GUI, hosted service, scheduler, database workflow, hardware-in-the-loop, or certification evidence.

## Purpose

The batch workflow runs repeated `analyze` operations over multiple CSV/config pairs and writes:

- one report per completed run,
- one deterministic JSON summary,
- optional text or JSON summary to stdout.

It is designed for local fixture sets, regression evidence, and repeated engineering reviews where each run must remain independently inspectable.

## Command

```bash
cargo run --quiet --bin ferrisoxide-signal -- batch \
  --manifest examples/batch-analysis.toml \
  --output-dir target/ferrisoxide-batch-example \
  --format json
```

Supported flags:

| Flag | Required | Meaning |
|---|---|---|
| `--manifest <toml>` | Yes | Batch manifest path. |
| `--output-dir <dir>` | No when manifest has `output_dir` | Output directory for reports and summary. Overrides manifest `output_dir`. |
| `--format text\|json` | No | Summary format printed to stdout. Default is `json`. |
| `--overwrite` | No | Allows existing per-run reports and summary file to be replaced. Without this flag, existing outputs are rejected. |

## Manifest

Example:

```toml
default_format = "json"
summary_file = "batch-summary.json"

[[runs]]
id = "basic_config"
input = "basic-waveform.csv"
config = "basic-config.toml"
report = "basic-config.json"
```

Relative `input` and `config` paths resolve from the manifest directory. Manifest `output_dir` paths resolve from the manifest directory; CLI `--output-dir` paths resolve from the current working directory. Relative report paths resolve under the output directory.

## Run Status

| Status | Meaning |
|---|---|
| `pass` | Analysis completed and `overall_outcome` was `pass`. A report was written. |
| `fail` | Analysis completed and `overall_outcome` was `fail`. A report was written. |
| `error` | The run could not complete because of read, parse, validation, analysis, render, or write error. No report is written for that run. |

The batch command continues after individual run errors so one bad fixture does not hide the rest of the batch evidence. The summary `overall_outcome` is `fail` if any run failed or errored.

## Summary Contract

The JSON summary is documented in `docs/artifact-contract.md`. Current stable fields include run counts, top-level outcome, and per-run status, outcome, report path, and error message.

## Safety Boundaries

- The batch workflow does not watch directories.
- The batch workflow does not run on a schedule.
- The batch workflow does not call DAQ SDKs or hardware.
- The batch workflow does not use a database or hosted service.
- The batch workflow does not merge evidence from different runs into a single pass/fail report; each run keeps its own report.
- Existing output files are not overwritten unless `--overwrite` is passed.

## Verification

M17 adds CLI unit coverage for:

- one passing analysis run,
- one failing analysis run,
- one error run with a missing input,
- deterministic summary counts,
- per-run report creation for completed runs,
- no partial report for error runs,
- empty-manifest rejection.

## Hand-Off Note

Role: Core Software Engineer / Test Automation Engineer
Goal: Add local batch analysis workflow before MVP exit.
Files changed: CLI batch implementation, `examples/batch-analysis.toml`, docs, tests, validation and traceability updates.
Checks run: See `docs/validation-log.md`.
Status: M17 implemented locally.
Known gaps: No live DAQ, GUI, hosted service, database, scheduler, hardware workflow, or batch plotting/export orchestration.
Next recommended step: Use the batch workflow for validation corpus runs and keep future workflow expansion behind explicit gates.
