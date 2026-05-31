# Analysis Report Schema

Date: 2026-05-31

## Scope

This document describes the MVP JSON report shape used by golden tests.

## Top-Level Fields

| Field | Type | Meaning |
|---|---|---|
| `input_name` | string | Input path or display name passed to the report. |
| `overall_outcome` | `pass` or `fail` | `fail` when any criterion fails. |
| `results` | array | Per-criterion evidence rows. |

## Result Fields

| Field | Type | Meaning |
|---|---|---|
| `criterion_id` | string | Stable criterion identifier from config or CLI. |
| `outcome` | `pass` or `fail` | Per-criterion result. |
| `failed_criterion` | string or null | Criterion ID when failed, otherwise null. |
| `channel` | string | Channel evaluated. |
| `measured_value` | number | Observed value used for the decision. |
| `required_value` | number | Required value from config. |
| `unit` | string | Unit for measured and required values. |
| `sample_index` | integer | Evidence sample index. |
| `timestamp` | number | Evidence timestamp in seconds. |
| `reason` | string | Human-readable decision reason. |

## Stability

Golden tests in `tests/golden/` compare JSON output exactly. Any intentional schema change should update this document, the golden files, and release notes together.
