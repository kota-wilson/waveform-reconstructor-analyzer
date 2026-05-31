# Criteria DSL Migration

Date: 2026-05-31

Issue: #60, `M7-006 Add engineering DSL examples and migration docs`

## Purpose

The measurement-backed criteria DSL separates:

- what measurement is taken from the waveform,
- how the measured value is compared,
- and the required engineering value with an explicit unit.

Legacy criteria fields remain supported. The DSL is additive and is not a report-schema migration.

## Working Example

Legacy config:

```toml
[[criteria]]
id = "input_max_voltage"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
```

Equivalent DSL config:

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

Run the checked-in DSL example:

```bash
cargo run --quiet --bin ferrisoxide-signal -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/basic-dsl-config.toml \
  --format text
```

Expected output excerpt:

```text
Overall: Pass
Measurements:
- input_min_voltage_measurement: method=minimum_sample channel=input_v measured=0.000000 V sample_index=0 timestamp=0.000000
- input_max_voltage_measurement: method=maximum_sample channel=input_v measured=5.000000 V sample_index=4 timestamp=0.004000
Criteria:
- input_min_voltage: Pass measurement_id=input_min_voltage_measurement channel=input_v measured=0.000000 V required=0.000000 V tolerance=0.000000 sample_index=0 timestamp=0.000000 reason=minimum observed voltage was 0.000000 V
- input_max_voltage: Pass measurement_id=input_max_voltage_measurement channel=input_v measured=5.000000 V required=5.500000 V tolerance=0.000000 sample_index=4 timestamp=0.004000 reason=maximum observed voltage was 5.000000 V
```

## When To Use DSL

Use DSL criteria when:

- the review focus is the measured engineering quantity,
- criteria should read as measurement plus requirement,
- parity with report measurement records is important,
- or future rule-package work should be easier to map to a portable schema.

Use legacy explicit fields when:

- maintaining older configs,
- writing a compact local fixture,
- or avoiding a migration in existing automation.

Both forms are expected to keep working. Parity fixtures in `tests/configs/*-dsl.toml` compare representative DSL configs to existing legacy golden reports exactly.

## Supported Initial Measurement Types

| DSL measurement type | Typical legacy criterion | Output unit |
|---|---|---|
| `minimum_sample` | `minimum_voltage` | `V` |
| `maximum_sample` | `maximum_voltage` | `V` |
| `state_transition_count` | `state_transitions` | `count` in TOML requirements; report evidence remains `transitions` for compatibility |
| `pulse_width` | `pulse_width` | `s` |
| `stable_state_duration` | `stable_state_duration` | `s` |
| `transient_event_duration` | `transient_event` / `transient_duration` | `s` |
| `rise_time` | `rise_fall_time` with `direction = "rise"` | `s` |
| `fall_time` | `rise_fall_time` with `direction = "fall"` | `s` |

## Operators

Supported operators:

- `less_than`
- `less_than_or_equal`
- `greater_than`
- `greater_than_or_equal`
- `equal_to`

Tolerance behavior is still explicit in the top-level `[tolerances]` table. Reports show the tolerance used for each result.

## Unit Rules

DSL values require explicit unit fields:

```toml
value = 0.005
unit = "s"
```

Supported units are:

- `V`
- `s`
- `count`

There is no unit shorthand parser. Values such as `value = "5ms"` are rejected. Unit conversion is not implemented in this slice.

## Compatibility Notes

- Legacy criteria remain supported.
- DSL criteria cannot be mixed with legacy fields inside the same `[[criteria]]` entry.
- Equivalent DSL and legacy configs should preserve criterion IDs when exact `measurement_id` compatibility matters.
- Existing JSON report fields remain unchanged.
- Bad DSL TOML is expected to fail with contextual `criteria.<id>...` error paths.

## Non-Goals

- No GUI.
- No DAQ integration.
- No plugin runtime.
- No batch analysis.
- No RTOS expansion.
- No new measurement primitives.
- No unit shorthand parser.
- No hardware qualification or certification claim.

## Hand-Off Note

Role: Documentation Engineer
Goal: Provide engineering migration notes for the measurement-backed criteria DSL.
Files changed: `docs/criteria-dsl-migration.md`
Checks run: CLI smoke for `examples/basic-dsl-config.toml`; workspace validation is recorded in `docs/validation-log.md`.
Status: Ready for review.
Known gaps: Full schema and report evidence notes remain in issue #61.
Next recommended step: Keep examples and parity fixtures synchronized when DSL behavior changes.
