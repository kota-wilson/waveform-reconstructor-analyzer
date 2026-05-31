# Criteria DSL Direction

Date: 2026-05-31

Issue: #46, `M6-004 Document criteria DSL direction for engineering measurements`

Runtime status: M7 added config parsing, operator/unit validation, runtime evaluation, parity tests, and invalid-config tests for the initial measurement-backed DSL. See `docs/criteria-dsl-migration.md` for current user-facing migration guidance.

## Scope

This document records the design direction for the measurement-backed criteria DSL. The initial runtime subset is implemented, but this document still preserves the original scope limits: no plugin runtime, GUI, DAQ integration, ML, RTOS expansion, hardware qualification, or certification claim.

## Current Compatibility Baseline

Existing `[[criteria]]` TOML entries remain supported and should continue to work:

```toml
[[criteria]]
id = "rise_time_tolerance"
type = "rise_fall_time"
channel = "rise_v"
direction = "rise"
low_threshold_v = 0.5
high_threshold_v = 4.5
max_duration_s = 0.0015
```

DSL work is additive. Existing explicit fields should not be silently reinterpreted.

## Measurement-Backed Criteria Concept

The criteria model separates three concepts:

| Concept | Meaning | Example |
|---|---|---|
| Measurement | What is measured from the waveform. | `rise_time`, `pulse_width`, `state_transition_count` |
| Comparator | How the measured value is compared. | `less_than_or_equal` |
| Requirement value | The required engineering value with explicit unit. | `{ value = 0.005, unit = "s" }` |

Example:

```toml
[[criteria]]
id = "rise_time_max_5ms"
channel = "switch_v"

[criteria.measurement]
type = "rise_time"
low_threshold = { value = 0.5, unit = "V" }
high_threshold = { value = 4.5, unit = "V" }

[criteria.requirement]
operator = "less_than_or_equal"
value = 0.005
unit = "s"
```

## Accepted DSL Shape

Each DSL criterion is a `[[criteria]]` table with `id`, `channel`, a `[criteria.measurement]` section, and a `[criteria.requirement]` section:

```toml
[[criteria]]
id = "rise_time_max_5ms"
channel = "switch_v"

[criteria.measurement]
type = "rise_time"
low_threshold = { value = 0.5, unit = "V" }
high_threshold = { value = 4.5, unit = "V" }

[criteria.requirement]
operator = "less_than_or_equal"
value = 0.005
unit = "s"
```

Top-level criterion fields:

| Field | Required | Meaning |
|---|---:|---|
| `id` | Yes | Stable criterion identifier used in reports. |
| `channel` | Yes | Waveform channel to measure. |
| `measurement` | Yes | Measurement section that defines what evidence to compute. |
| `requirement` | Yes | Requirement section that defines the comparison and required value. |

Legacy fields such as `type`, `threshold_v`, `state`, `direction`, `max_duration_s`, or `min_duration_s` cannot be mixed into a DSL criterion. Keep them in legacy criteria entries only.

Measurement fields:

| Field | Applies To | Required | Meaning |
|---|---|---:|---|
| `type` | all DSL measurements | Yes | Measurement type. |
| `threshold` | `state_transition_count`, `pulse_width`, `stable_state_duration`, `transient_event_duration` | Yes | State threshold as `{ value, unit = "V" }`. |
| `low_threshold` | `rise_time`, `fall_time` | Yes | Lower edge threshold as `{ value, unit = "V" }`. |
| `high_threshold` | `rise_time`, `fall_time` | Yes | Upper edge threshold as `{ value, unit = "V" }`. |
| `state` | `pulse_width`, `stable_state_duration` | Yes | `high` or `low`. |
| `expected_state` | `transient_event_duration` | Yes | Expected steady state, `high` or `low`; the measured transient is the opposite state. |
| `event_kind` | `transient_event_duration` | No | Defaults to `transient_event`; accepted values match transient event kinds below. |
| `selection` | `pulse_width`, `stable_state_duration`, `transient_event_duration` | Sometimes | `shortest` or `longest`; required for `equal_to` pulse width, inferred for other pulse-width operators, and must be `longest` if supplied for stable/transient duration. |

Accepted `event_kind` values:

- `transient_event`
- `spurious_transition`
- `contact_bounce`
- `dropout`
- `noise_induced_transition`
- `threshold_crossing_event`

Requirement fields:

| Field | Required | Meaning |
|---|---:|---|
| `operator` | Yes | One of the supported operators below. |
| `value` | Yes | Numeric required value. |
| `unit` | Yes | Explicit unit matching the measurement output: `V`, `s`, or `count`. |

## Initial Operator Vocabulary

The first operator vocabulary is small and auditable:

| Operator | Meaning |
|---|---|
| `less_than` | Measured value must be strictly below the required value. |
| `less_than_or_equal` | Measured value may equal or be below the required value. |
| `greater_than` | Measured value must be strictly above the required value. |
| `greater_than_or_equal` | Measured value may equal or be above the required value. |
| `equal_to` | Measured value must equal the required value after the configured tolerance model is applied. |

Tolerance handling remains explicit and reportable. The report shows the measured value, required value, unit, tolerance used, sample index, timestamp, channel, and measurement ID.

## Explicit Units

Numeric values with explicit `unit` fields are required. Shorthand strings such as `10ms` are not supported.

Preferred:

```toml
value = 0.010
unit = "s"
```

Rejected shorthand:

```toml
value = "10ms"
```

Reasons:

- Explicit units are easier to validate with TOML deserialization.
- Errors can identify the exact missing or unsupported field.
- Reports can preserve units without parsing ambiguity.
- Engineering review is clearer when unit conversion rules are explicit.
- Shorthand strings require a unit parser, rounding policy, and compatibility tests.

## Measurement Types And Evaluation Mapping

Initial DSL measurement types map to existing measurement and evaluator behavior:

| DSL measurement type | Existing backing logic | Report method | Report context |
|---|---|---|---|
| `minimum_sample` | `minimum_sample` | `minimum_sample` | no thresholds |
| `maximum_sample` | `maximum_sample` | `maximum_sample` | no thresholds |
| `state_transition_count` | `count_state_transitions` | `state_transition_count` | `threshold_v` |
| `pulse_width` | `state_run_extremum` over selected state | `state_run_duration` | `threshold_v`, `state`, `selection` |
| `stable_state_duration` | `state_run_extremum` using `longest` selected state | `state_run_duration` | `threshold_v`, `state`, `selection = "longest"` |
| `transient_event_duration` | `state_run_extremum` using `longest` run of the opposite state | `state_run_duration` | `threshold_v`, transient `state`, `expected_state`, `event_kind`, `selection = "longest"` |
| `rise_time` | `measure_rise_time` | `edge_time` | `low_threshold_v`, `high_threshold_v`, `direction = "rise"` |
| `fall_time` | `measure_fall_time` | `edge_time` | `low_threshold_v`, `high_threshold_v`, `direction = "fall"` |

Future measurement types such as duty cycle, frequency, period, overshoot, undershoot, RMS, peak-to-peak, and noise floor should be added only after known-answer validation fixtures exist.

## Report Evidence Behavior

DSL criteria use the same report schema as legacy criteria:

- `measurements[]` contains reusable measured evidence.
- Each result references evidence with `measurement_id`.
- Current measurement IDs use `{criterion_id}_measurement`.
- `results[]` contains pass/fail decisions, required values, tolerance, sample index, timestamp, channel, and reason.
- Equivalent DSL and legacy configs should preserve the same `id` values when exact `measurement_id` compatibility matters.

Example JSON result fields:

```json
{
  "criterion_id": "input_max_voltage",
  "outcome": "pass",
  "failed_criterion": null,
  "measurement_id": "input_max_voltage_measurement",
  "channel": "input_v",
  "measured_value": 5.0,
  "required_value": 5.5,
  "tolerance_used": 0.0,
  "unit": "V",
  "sample_index": 4,
  "timestamp": 0.004,
  "reason": "maximum observed voltage was 5.000000 V"
}
```

For `state_transition_count`, TOML requirements use `unit = "count"` while the current report evidence unit remains `transitions` for legacy report compatibility.

## Unsupported Syntax And Current Rejections

Current DSL rejects:

- mixed legacy and DSL fields in one criterion,
- missing `measurement` or `requirement` sections,
- missing `requirement.operator`, `requirement.value`, or `requirement.unit`,
- unsupported operators or units,
- mismatched requirement units for the selected measurement,
- missing threshold fields required by the selected measurement,
- `rise_time` / `fall_time` thresholds where `low_threshold >= high_threshold`,
- invalid states other than `high` or `low`,
- `pulse_width` with `operator = "equal_to"` and no explicit `selection`,
- unit shorthand strings such as `value = "5ms"`,
- arithmetic expressions or compound requirement expressions.

Future work must keep unsupported syntax separate from current behavior until it has validation, parity tests, and documentation.

## Non-Goals

- No plugin runtime.
- No GUI or interactive form builder.
- No DAQ integration.
- No machine learning.
- No RTOS expansion.
- No unit-shorthand parser in this documentation slice.
- No expression language.
- No certification, hardware qualification, or flight-readiness claim.

## Gate Decision

- Gate: Documentation Gate for M6-004.
- Decision: Pass.
- Reason: The future DSL direction defines concepts, operator vocabulary, unit policy, compatibility expectations, and non-goals without changing runtime behavior.
- Residual risk: User-facing schema details and report evidence notes continue in M7 documentation issues.
- Next owner: Software Architect / Core Software Engineer.

## Hand-Off Note

Role: Software Architect / Documentation Engineer
Goal: Document the criteria DSL direction before expanding syntax.
Files changed: `docs/criteria-dsl.md`
Checks run: Documentation review plus workspace validation in the M6 completion branch.
Status: Ready for review.
Known gaps: Full schema and report evidence notes remain in M7 documentation follow-up work.
Next recommended step: Keep existing `[[criteria]]` compatibility while expanding DSL docs and fixtures.
