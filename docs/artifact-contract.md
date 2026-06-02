# FerrisOxide Artifact Contract

Date: 2026-06-01

Status: M16 contract for implemented local artifacts. This document defines current compatibility expectations for desktop evidence artifacts. It does not define binary deployment, cryptographic signing, live hardware acquisition, hardware qualification, or certification evidence.

## Artifact Families

| Artifact | Producer | Stability Expectation | Compatibility Rule |
|---|---|---|---|
| Text analysis report | `ferrisoxide-signal analyze --format text` | Human-readable; not a machine schema. | Add lines only when useful; do not remove core pass/fail, measurement, and criterion evidence without a docs update. |
| JSON analysis report | `ferrisoxide-signal analyze --format json`; rule-package export validation report; batch per-run JSON reports | Machine-readable and protected by golden tests. | Additive fields require report docs and expected artifact updates; removals/renames require schema compatibility approval. |
| SVG plot | `ferrisoxide-signal plot` | Visual local evidence. | SVG structure is not a stable API; rendered evidence should remain inspectable and generated from report/criteria evidence when overlays are enabled. |
| Rule package `rules.toml` | `ferrisoxide-signal export-rule-package` | Human review artifact. | Must parse as the current `ferrisoxide-rule-schema` model. |
| Rule package `rules.json` | `ferrisoxide-signal export-rule-package` | Automation artifact. | Must represent the same package model as `rules.toml`. |
| Rule package `validation-report.json` | `ferrisoxide-signal export-rule-package` | Analysis evidence artifact. | Same JSON report contract as analysis reports. |
| Rule package `manifest.json` | `ferrisoxide-signal export-rule-package` | Package inventory artifact. | Must list exported artifacts, source config, validation evidence, schema version, target profile, and checksum metadata. |
| Rule package `checksum.txt` | `ferrisoxide-signal export-rule-package` | Deterministic drift-detection artifact. | Non-cryptographic checksum evidence only; not signing, authentication, tamper proofing, hardware qualification, or certification. |
| Batch per-run reports | `ferrisoxide-signal batch` | Same as analysis reports. | Each successful or failed analysis run writes its configured report format. Error runs do not write partial reports. |
| Batch summary | `ferrisoxide-signal batch` | Machine-readable JSON summary plus optional text stdout. | Summary fields are stable for automation: counts, per-run status, per-run report paths, and errors. |
| Qualification evidence report | `ferrisoxide-deployment` fixtures and docs | Schema-managed software evidence. | Must include non-certification scope notes. |

## JSON Analysis Report Contract

The JSON analysis report schema is documented in `docs/report-schema.md`. Current top-level fields are:

- `input_name`
- `waveform_metadata`
- `evidence_context`
- `overall_outcome`
- `measurements`
- `event_records` when present
- `event_validations` when present
- `results`

Compatibility expectations:

- Keep existing field names stable until an explicit schema compatibility gate approves a breaking migration.
- Add new fields only with updated docs and golden fixtures.
- Preserve `measurements[]` and `results[]` links through `measurement_id`.
- Preserve event evidence separation: `event_records[]` are evidence; `event_validations[]` are pass/fail decisions.
- Preserve report confidence notes that state software evidence only and not hardware qualification or certification evidence.

## Batch Summary Contract

`ferrisoxide-signal batch` writes a JSON summary file. Default name: `batch-summary.json`.

Current fields:

| Field | Type | Meaning |
|---|---|---|
| `kind` | string | Always `batch_analysis`. |
| `manifest` | string | Manifest path passed to the CLI. |
| `output_dir` | string | Output directory used for reports and summary. |
| `total_runs` | integer | Number of manifest runs. |
| `passed_runs` | integer | Runs whose criteria/event validations passed. |
| `failed_runs` | integer | Runs that completed analysis and produced `overall_outcome = fail`. |
| `error_runs` | integer | Runs that could not read, parse, analyze, render, or write a report. |
| `overall_outcome` | `pass` or `fail` | `fail` when any run failed or errored. |
| `runs` | array | Per-run evidence. |

Per-run fields:

| Field | Type | Meaning |
|---|---|---|
| `id` | string | Manifest run ID. |
| `input` | string | Resolved CSV path. |
| `config` | string | Resolved analysis config path. |
| `report` | string or null | Report path for completed runs. |
| `status` | `pass`, `fail`, or `error` | Batch-level run status. |
| `outcome` | `pass`, `fail`, or null | Analysis outcome when a report was produced. |
| `error` | string or null | Error message for error runs. |

## Golden Artifact Matrix

| Artifact Set | Location | Protection |
|---|---|---|
| Basic criteria golden reports | `tests/golden/*.json` | Core/CLI exact report tests. |
| Heated actuator reports | `tests/e2e/heated_actuator/expected/*.json` | E2E exact report tests. |
| Rule-package export artifacts | `tests/expected/rule-package-basic/` | CLI exact artifact tests. |
| Validation reports | `validation/reports/*.json` | Known-answer validation tests. |
| Batch workflow summary and per-run reports | Generated from `examples/batch-analysis.toml` and M17 CLI tests | CLI batch tests and docs. |

## Generated Output Rules

- Exported rule packages refuse to overwrite existing artifacts.
- Batch analysis refuses to overwrite existing reports and summaries unless `--overwrite` is passed.
- Generated outputs should live under caller-selected directories such as `target/`, a validation artifact folder, or a temp directory used by tests.
- Generated output directories are not hardware evidence repositories unless a later hardware validation process defines that role.

## Breaking-Change Gate

The schema compatibility gate is required before:

- removing or renaming JSON report fields,
- changing artifact file names,
- changing checksum algorithm, scope, or security wording,
- changing batch summary field names or status vocabulary,
- changing rule-package transform export semantics,
- changing config fields in a way that breaks existing examples or fixtures.

## Hand-Off Note

Role: Documentation Engineer / Verification and Validation Engineer
Goal: Stabilize report and generated-artifact contract expectations before MVP exit.
Files changed: `docs/artifact-contract.md`, report/config/batch docs, validation and traceability updates.
Checks run: See `docs/validation-log.md`.
Status: M16 artifact contract complete locally.
Known gaps: No cryptographic signing or binary package contract exists; those remain separately gated.
Next recommended step: Update this contract with every future schema or generated-artifact change.
