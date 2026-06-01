# Qualification Evidence Report Format

Status: implemented as a schema and exact JSON fixture for M9-010 / issue #86.

## Purpose

The qualification evidence report is the audit package that ties a controller-in-the-loop run together. It is meant to answer:

- what production control config was simulated,
- what test verification config judged the signals,
- what channel map connected CSV/DAQ signals to logical controller inputs,
- what simulator trace was produced,
- what criteria evidence was produced,
- what deployment package metadata and checksums were in scope,
- when the evidence was generated,
- what the evidence does not claim.

The format lives in `crates/ferrisoxide-deployment` as `QualificationEvidenceReport`.

## Example

The current example fixture is:

```text
examples/deployment-package/heated-actuator/qualification-report.json
```

The exact JSON fixture is parsed, validated, serialized back with `serde_json::to_string_pretty`, and compared byte-for-byte by:

```bash
cargo test -p ferrisoxide-deployment example_qualification_evidence_report_validates_and_matches_exact_json
```

## Top-Level Fields

| Field | Meaning |
|---|---|
| `report_version` | Version of the qualification evidence report schema. |
| `generated_at` | Timestamp for the evidence report. |
| `overall_outcome` | Overall `pass` or `fail` result for the qualification run. |
| `qualification` | Case ID, workflow name, and input waveform path. |
| `production_control_config` | Linked production control config name, version, path, and checksum. |
| `test_verification_config` | Linked test verification config name, version, path, and checksum. |
| `channel_map` | Linked channel map path and checksum. |
| `simulation_trace` | Controller simulation trace frames, transitions, state-machine states, and outputs. |
| `criteria_evidence` | Per-criterion pass/fail measurement evidence. |
| `deployment_package` | Deployment package name, version, format version, target, manifest path, timestamp, and mode profile references. |
| `checksum_evidence` | Checksum file metadata plus role/path/checksum entries for deployment artifacts. |
| `report_artifact` | Report artifact path and checksum. |
| `visual_evidence` | Human-reviewable SVG evidence artifact path and checksum. |
| `generated_at_artifact` | Timestamp artifact path and checksum. |
| `scope_notes` | Explicit scope notes, including the required non-certification statement. |

## Validation Rules

`QualificationEvidenceReport::validate()` checks:

- required strings are non-empty,
- report version matches the current schema version,
- deployment package format version matches the deployment manifest format version,
- simulation trace contains at least one frame,
- criteria evidence contains at least one record,
- numeric evidence values are finite,
- deployment mode references are present,
- checksum evidence includes all linked deployment artifact roles,
- scope notes explicitly state the report is not flight certification evidence.

## Scope Limits

This format is software evidence only. It does not claim:

- live DAQ acquisition,
- target firmware execution,
- RTOS loader behavior,
- real-time scheduling,
- hardware timing,
- hardware qualification,
- flight certification.

## Hand-Off Note

Role: Documentation Engineer / Verification and Validation Engineer
Goal: Document the qualification evidence report schema for controller-in-the-loop workflows.
Files changed: `docs/qualification-evidence-report.md`, `crates/ferrisoxide-deployment/`, `examples/deployment-package/heated-actuator/qualification-report.json`.
Checks run: See `docs/validation-log.md`.
Status: Implemented locally; PR, protected CI, merge, and issue #86 closure pending.
Known gaps: No CLI exporter, live DAQ, hardware-target runtime, SVG generation into deployment packages, cryptographic signing, or certification evidence.
Next recommended step: Open PR with `Fixes #86`, wait for required CI, and merge only after checks pass.
