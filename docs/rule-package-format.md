# FerrisOxide Rule Package Format

Date: 2026-05-31

Status: Initial reviewable format for M8-002 / issue #71.

Related requirements: WRA-RQ-043, WRA-RQ-044.

## Purpose

A FerrisOxide Rule Package is the portable rule definition intended to move from desktop authoring and validation toward embedded/controller consumption without duplicating rule semantics.

The package format is not a controller release, hardware qualification artifact, safety case, or certification artifact. It is a software configuration and evidence bundle that later validators, exporters, and rule engines must handle consistently.

## Artifact Set

The reviewable deployment package shape is:

```text
deployment/
  rules.toml
  rules.json
  rules.bin
  manifest.json
  checksum.txt
  validation-report.json
  qualification-evidence.svg
```

| Artifact | Status | Audience | Role |
|---|---|---|---|
| `rules.toml` | Initial format | Human reviewers, desktop tooling | Canonical human-authored rule package with package metadata, target profile, sample timing, channels, units, thresholds, filters, criteria, and timing limits. |
| `rules.json` | Initial format | Automation, tests, future export tooling | Machine-readable representation of the same `ferrisoxide-rule-schema` model as `rules.toml`. |
| `rules.bin` | Future | Embedded/controller runtime | Compact deterministic representation for constrained runtimes. Not implemented in M8-002. |
| `manifest.json` | Future | Reviewers, automation, runtime loaders | Records package artifact names, schema version, target profile, generation metadata, and checksum metadata. Final fields are implemented in M8-005. |
| `checksum.txt` | Future | Automation, runtime loaders | Records deterministic package checksum evidence. Algorithm and mismatch behavior are implemented in M8-005 after security/dependency review. |
| `validation-report.json` | Existing report family, future package role | Engineers, V&V, CI | Captures desktop validation evidence showing whether the rule package passed against known waveform data. |
| `qualification-evidence.svg` | Existing plotting family, future package role | Human reviewers | Visual evidence output with waveform, thresholds, annotations, and pass/fail context. It is software evidence only. |

M8-002 defines the artifact roles. It does not add an export command, checksum algorithm, binary format, runtime loader, DAQ integration, HAL, RTOS production integration, hardware qualification, or certification claim.

## Canonical Schema Model

The Rust schema lives in `crates/ferrisoxide-rule-schema`.

`rules.toml` and `rules.json` must deserialize to the same `RulePackage` model:

- `package`: package name, package version, schema version, and optional description.
- `target`: target profile kind, optional target identifier, and optional review notes.
- `sample_timing`: timestamp unit, nominal sample rate, optional sample-rate tolerance, optional sample interval, and optional timestamp tolerance.
- `channels`: logical channel names, optional source names, engineering units, optional per-channel sample rate, and named thresholds.
- `filters`: ordered transform definitions.
- `criteria`: measurement-backed pass/fail criteria.

Supported engineering unit strings are:

| Unit | Meaning |
|---|---|
| `V` | volts |
| `s` | seconds |
| `count` | event or transition count |
| `sample` | sample index/count |
| `Hz` | hertz |

Unit shorthand such as `10ms` is not supported in the initial package format. Use explicit numeric values with explicit unit fields.

## Example Package

Parse-tested examples live in:

- `examples/rule-package/rules.toml`
- `examples/rule-package/rules.json`

The example package includes:

- package metadata and schema version,
- Raspberry Pi 5 bare-metal target identifier as review context,
- sample-rate and timestamp assumptions,
- one logical switch channel mapped to `daq_ai0`,
- low, high, and decision thresholds,
- moving-average and ADC-quantization filters,
- transient-event/dropout duration criterion,
- stable-state duration criterion,
- state-transition count criterion.

Excerpt from `rules.toml`:

```toml
[package]
name = "switch-qualification-rule"
version = "1.0.0"
schema_version = "0.1.0"

[target]
kind = "controller_runtime"
identifier = "raspberry-pi-5-bare-metal"

[sample_timing]
timestamp_unit = "s"
nominal_sample_rate_hz = 10000.0
sample_rate_tolerance_hz = 1.0
nominal_sample_interval_s = 0.0001
timestamp_tolerance_s = 0.000001

[[channels]]
name = "switch_signal"
source_name = "daq_ai0"
unit = "V"
sample_rate_hz = 10000.0

[[channels.thresholds]]
name = "switch_decision"
role = "decision"
value = { value = 2.5, unit = "V" }

[[criteria]]
id = "CRIT-001"
channel = "switch_signal"

[criteria.measurement]
type = "transient_event_duration"
event_kind = "dropout"
expected_state = "high"
threshold = { value = 2.5, unit = "V" }

[criteria.requirement]
operator = "less_than_or_equal"
value = { value = 0.001, unit = "s" }
```

The `rules.json` example represents the same schema model for automation and future export workflows.

## Filters

Initial filter definitions are schema entries only. They describe intended transform order but are not executed by `ferrisoxide-rule-schema`.

| Type | Required fields | Notes |
|---|---|---|
| `moving_average` | `id`, `channel`, `window_samples` | Mirrors the existing desktop filter concept. |
| `low_pass` | `id`, `channel`, `cutoff` | `cutoff` is a unit-bearing value and should use `Hz`. |
| `adc_quantize` | `id`, `channel`, `bits`, `min`, `max` | `min` and `max` are unit-bearing values and should use `V` for voltage channels. |

## Criteria

Criteria are measurement-backed definitions with an explicit requirement:

```toml
[[criteria]]
id = "CRIT-002"
channel = "switch_signal"

[criteria.measurement]
type = "stable_state_duration"
state = "high"
threshold = { value = 2.5, unit = "V" }

[criteria.requirement]
operator = "greater_than_or_equal"
value = { value = 0.100, unit = "s" }
```

Supported comparison operators:

- `less_than`
- `less_than_or_equal`
- `greater_than`
- `greater_than_or_equal`
- `equal_to`

Supported measurement types:

- `minimum_sample`
- `maximum_sample`
- `state_transition_count`
- `pulse_width`
- `stable_state_duration`
- `transient_event_duration`
- `rise_time`
- `fall_time`

Timing limits are represented as criterion requirement values with unit `s`. Count limits are represented with unit `count`.

## Embedded Consumption Subset

Embedded/controller runtimes should start with the smallest deterministic subset needed for runtime execution:

```text
rules.bin
manifest.json
checksum.txt
```

Until M8-005 and M8-007 are implemented, this subset is a target architecture contract, not a working runtime package.

Embedded consumers must not require:

- CSV parsing,
- local file report rendering,
- SVG plotting,
- desktop UI behavior,
- live DAQ drivers,
- hardware HALs,
- SDK-specific RTOS bindings,
- heap allocation for basic rule evaluation where practical.

## Validation Expectations

The format is validated by parse-testing `rules.toml` and `rules.json` into `ferrisoxide-rule-schema::RulePackage`, verifying that both examples describe the same package, and running `RulePackage::validate()` before export or execution.

Later issues add:

- M8-004 CLI export,
- M8-005 manifest/checksum behavior,
- M8-006 shared rule execution,
- M8-007 no_std compatibility boundary,
- M8-008 desktop-vs-embedded parity tests.

The initial M8-003 validator returns structured errors for:

- missing channel definitions or references,
- unsupported unit strings during parsing,
- unknown filter or criterion tags during parsing,
- invalid timing and sample-rate assumptions,
- checksum mismatch when expected and actual checksum strings are provided,
- incompatible target profile expectations,
- invalid filter, threshold, or criterion parameters.

## Hand-Off Note

Role: Software Architect / Documentation Engineer
Goal: Define the initial portable rule package format and artifact roles.
Files changed: `docs/rule-package-format.md`, `examples/rule-package/rules.toml`, `examples/rule-package/rules.json`.
Checks run: Parse-tested examples and workspace validation recorded in `docs/validation-log.md`.
Status: Initial format documented and examples added.
Known gaps: Export command, manifest/checksum, binary package, shared rule engine, no_std boundary, and parity tests remain future M8 issues.
Next recommended step: Add the package export command in M8-004.
