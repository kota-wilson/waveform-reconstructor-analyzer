# Test Verification Config Schema

Status: implemented schema boundary for M9-002 / issue #80.

Crate: `crates/ferrisoxide-verification-schema`

Example: `examples/test-verification-config/test-verification-config.toml`

## Purpose

The test verification config schema defines how observed signals are judged during qualification, production-test, or controller-in-the-loop workflows. It is separate from the production control config schema and from the lower-level portable rule package.

Use this schema for:

- expected state transitions,
- voltage limits,
- pulse-width requirements,
- transient event limits,
- dropout limits,
- stable-state duration requirements,
- timing windows,
- channel definitions,
- evidence artifact requests,
- report field requirements,
- version and approval metadata.

Do not use this schema for:

- production controller state-machine behavior,
- hardware output actions,
- DAQ SDK integration,
- CSV parsing,
- report rendering,
- SVG plotting,
- RTOS deployment package loading,
- certified test-system approval.

## Relationship To Other Configs

FerrisOxide now has three related but separate config families:

| Config family | Purpose | Current crate |
|---|---|---|
| Production control config | Defines how controller logic should behave. | `ferrisoxide-control-schema` |
| Test verification config | Defines how observed UUT signals should be judged. | `ferrisoxide-verification-schema` |
| Portable rule package | Defines deployment-oriented shared rule artifacts exported from verified config and evidence. | `ferrisoxide-rule-schema` |

The test verification config may link to a production control config only through manifest metadata:

```toml
[production_control]
package_name = "heated-actuator-production-control"
package_version = "0.1.0"
schema_version = "0.1.0"
manifest_artifact = "deployment/manifest.json"
checksum = "fnv1a64:0123456789abcdef"
```

The schema intentionally does not embed production controller states, actions, outputs, HAL names, or runtime bindings. That keeps qualification criteria reviewable without making them part of production behavior.

## Top-Level Shape

```toml
[package]
name = "heated-actuator-qualification"
version = "0.1.0"
schema_version = "0.1.0"

[approval]
status = "draft"

[sample_timing]
sample_rate_hz = 1000.0
nominal_sample_period_s = 0.001

[evidence]
include_failed_criteria = true
include_measurements = true
include_sample_index = true
include_timestamp = true
include_channel = true

[report]
formats = ["text", "json"]
include_overall_status = true
include_failed_criterion = true
include_measured_value = true
include_required_value = true
include_sample_index = true
include_timestamp = true
include_channel = true
```

Top-level sections:

| Section | Required | Purpose |
|---|---:|---|
| `package` | Yes | Name, version, schema version, and optional description. |
| `production_control` | No | Manifest-only link to a production control config package. |
| `approval` | Yes | Draft/reviewed/approved/retired status plus approval evidence. |
| `sample_timing` | Yes | Sample-rate and time-axis assumptions. |
| `channels` | Yes | Channel IDs, CSV columns, units, and optional state thresholds. |
| `timing_windows` | No | Named time ranges referenced by criteria. |
| `expected_transitions` | No | Required state transitions and optional response-latency limits. |
| `voltage_limits` | No | Min/max voltage limits by channel and optional window. |
| `pulse_widths` | No | Min/max pulse-width requirements by channel/state. |
| `transient_limits` | No | Transient, spurious transition, contact bounce, false transition, noise-induced transition, and threshold-crossing limits. |
| `dropout_limits` | No | Brief interruption limits for expected-state channels. |
| `stable_state_requirements` | No | Required stable-state durations. |
| `evidence` | Yes | Evidence fields and artifact requests. |
| `report` | Yes | Required report formats and report evidence fields. |

## Validation Rules

`TestVerificationConfig::validate()` checks:

- package name and version are not empty,
- schema version matches `0.1.0`,
- production control links are manifest metadata only and include package, schema, artifact, and checksum fields,
- approved configs identify approver and approval time,
- sample timing has a positive sample rate or nominal sample period,
- channels are present and have unique IDs,
- optional channel thresholds are finite and low threshold is below high threshold,
- timing windows have finite non-negative start times and end after start,
- all criteria IDs are unique across criterion families,
- criteria reference existing channels and timing windows,
- voltage limits define at least one finite min/max value,
- pulse-width criteria define at least one positive min/max duration,
- transient and dropout limits define positive max durations,
- stable-state requirements define positive minimum durations,
- evidence settings include failed criteria, measurements, sample index, timestamp, and channel fields,
- report settings include PASS/FAIL, failed criterion, measured value, required value, sample index, timestamp, and channel fields.

Validation returns a `VerificationConfigValidationReport` with structured errors. It should not panic on malformed config data.

## Example Qualification Criteria

Representative transition criterion:

```toml
[[expected_transitions]]
id = "REQ-001"
channel = "feedback"
from_state = "low"
to_state = "high"
reference_channel = "command"
reference_state = "high"
max_latency_s = 0.050
window = "commanded-open"
required = true
```

Representative stable-state criterion:

```toml
[[stable_state_requirements]]
id = "REQ-002"
channel = "feedback"
state = "high"
min_duration_s = 0.500
threshold_v = 2.5
window = "commanded-open"
```

Representative transient-event criterion:

```toml
[[transient_limits]]
id = "REQ-003"
channel = "feedback"
event_kind = "spurious_transition"
expected_state = "high"
max_duration_s = 0.001
allowed_count = 0
window = "commanded-open"
arm_after_first_expected_state = true
```

Representative report settings:

```toml
[report]
formats = ["text", "json"]
include_overall_status = true
include_failed_criterion = true
include_measured_value = true
include_required_value = true
include_sample_index = true
include_timestamp = true
include_channel = true
```

## Current Limits

The schema is intentionally data-only. It does not execute criteria, transform waveforms, generate reports, render plots, simulate controllers, acquire DAQ data, load deployment packages, or prove timing on hardware.

Future M9 issues should build on this in order:

1. Add a virtual controller simulation engine.
2. Add DAQ and controller I/O abstractions.
3. Add desktop simulation workflow.
4. Add deployment package format and parity tests.
5. Add qualification evidence report format.
