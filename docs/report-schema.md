# Analysis Report Schema

Date: 2026-05-31

## Scope

This document describes the MVP JSON report shape used by golden tests and validation reports.

## Top-Level Fields

| Field | Type | Meaning |
|---|---|---|
| `input_name` | string | Input path or display name passed to the report. |
| `waveform_metadata` | object | Source, unit, count, time-axis, lineage, and transform context for the analyzed waveform. |
| `evidence_context` | object | Report-level engineering-validation context, evidence source, tolerance policy, and confidence notes. |
| `overall_outcome` | `pass` or `fail` | `fail` when any criterion fails. |
| `measurements` | array | Reusable measurement evidence records with stable IDs. |
| `results` | array | Per-criterion evidence rows. |

## Waveform Metadata Fields

| Field | Type | Meaning |
|---|---|---|
| `source_name` | string or null | Source path or display name when known. |
| `test_run_id` | string or null | Optional local validation or test-run identifier from config metadata. |
| `acquisition_notes` | string or null | Optional local notes about the fixture or acquisition context. |
| `environment` | string or null | Optional local environment descriptor. |
| `operator` | string or null | Optional operator or automation descriptor. |
| `time_unit` | string | Unit used for the waveform time axis. |
| `sample_count` | integer | Number of waveform samples. |
| `channel_count` | integer | Number of analyzed channels. |
| `channels` | array | Channel names and units present in the waveform. |
| `sample_interval` | object or null | Minimum, maximum, nominal, unit, and uniformity summary for adjacent time samples. |
| `nominal_sample_rate_hz` | number or null | Derived sample rate when the time unit is seconds and the nominal interval is positive. |
| `lineage` | `raw` or `derived` | Whether criteria evaluated raw parsed samples or a derived waveform. |
| `transform_history` | array | Ordered transform descriptions applied before criteria evaluation. |
| `tolerance_policy` | object or null | Voltage/time tolerance policy attached to the analyzed waveform. |

## Evidence Context Fields

| Field | Type | Meaning |
|---|---|---|
| `validation_profile` | string | Current profile is `engineering_validation`, meaning local software-analysis evidence. |
| `evidence_source` | string | Current source is `local_file_analysis`, meaning a local CSV/config produced the report. |
| `tolerance_policy` | object | Report-level voltage/time tolerance policy applied to criteria evaluation. |
| `confidence_notes` | array | Human-readable scope notes. Current notes explicitly say the report is not hardware qualification or certification evidence. |

## Measurement Fields

The `measurements` array separates measured signal evidence from pass/fail criteria decisions. Each criterion result that was evaluated from a measurement references one record by `measurement_id`.

| Field | Type | Meaning |
|---|---|---|
| `id` | string | Stable report-local measurement ID. Current IDs use `{criterion_id}_measurement`. |
| `channel` | string | Channel measured. |
| `method` | string | Measurement method, such as `minimum_sample`, `maximum_sample`, `state_transition_count`, `state_run_duration`, `response_latency`, or `edge_time`. |
| `measured_value` | number | Observed value before criteria policy is applied. |
| `unit` | string | Unit for the measured value. |
| `sample_index` | integer | Evidence sample index. |
| `timestamp` | number | Evidence timestamp in seconds. |
| `method_context` | object | Method parameters needed to interpret the measurement. |

## Measurement Method Context Fields

| Field | Type | Meaning |
|---|---|---|
| `source` | string | Measurement implementation source, such as `ferrisoxide-measurements` or `ferrisoxide-rule-engine`. |
| `threshold_v` | number or null | Threshold used for state-based measurements. |
| `low_threshold_v` | number or null | Lower edge threshold for rise/fall measurements. |
| `high_threshold_v` | number or null | Upper edge threshold for rise/fall measurements. |
| `state` | string or null | Measured state such as `high` or `low` when applicable. |
| `expected_state` | string or null | Expected steady state for transient-event style measurements. |
| `event_kind` | string or null | Transient subtype, such as `dropout` or `contact bounce`, when applicable. |
| `direction` | string or null | Edge direction, `rise` or `fall`, when applicable. |
| `selection` | string or null | Run-selection policy, such as `shortest`, `longest`, or `first_response`, when applicable. |

## Result Fields

| Field | Type | Meaning |
|---|---|---|
| `criterion_id` | string | Stable criterion identifier from config or CLI. |
| `outcome` | `pass` or `fail` | Per-criterion result. |
| `failed_criterion` | string or null | Criterion ID when failed, otherwise null. |
| `measurement_id` | string | Stable ID of the measurement record used as evidence. |
| `channel` | string | Channel evaluated. |
| `measured_value` | number | Observed value used for the decision. |
| `required_value` | number | Required value from config. |
| `tolerance_used` | number | Voltage or time tolerance used by that criterion. State-transition count uses `0.0`. |
| `unit` | string | Unit for measured and required values. |
| `sample_index` | integer | Evidence sample index. |
| `timestamp` | number | Evidence timestamp in seconds. |
| `reason` | string | Human-readable decision reason. |

## Stability

Golden tests in `tests/golden/` compare JSON output exactly. Any intentional schema change should update this document, the golden files, and release notes together.

## Criteria DSL Evidence Note

Measurement-backed DSL criteria use the same report schema as legacy criteria. A DSL config changes how the criterion is written in TOML, not how measurement evidence appears in JSON.

For equivalent configs:

- `criterion_id` remains the configured `id`.
- `measurement_id` remains `{criterion_id}_measurement`.
- `measurements[]` records the measured evidence with method and context.
- `results[]` records the pass/fail decision with measured value, required value, tolerance, sample index, timestamp, channel, and reason.

The current parity tests compare representative DSL reports against legacy reports and existing golden JSON exactly. For `state_transition_count`, the DSL requirement unit is `count`, while the report evidence unit remains `transitions` for existing report compatibility.

## M6-003 Migration Note

M6-003 adds the top-level `measurements` array and the per-result `measurement_id` field. Existing result fields remain present so older consumers can keep reading criterion-level pass/fail evidence while newer consumers can de-duplicate measurement evidence and render richer report/SVG annotations.
