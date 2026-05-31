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

## Initial Operator Vocabulary

The first operator vocabulary is small and auditable:

| Operator | Meaning |
|---|---|
| `less_than` | Measured value must be strictly below the required value. |
| `less_than_or_equal` | Measured value may equal or be below the required value. |
| `greater_than` | Measured value must be strictly above the required value. |
| `greater_than_or_equal` | Measured value may equal or be above the required value. |
| `equal_to` | Measured value must equal the required value after the configured tolerance model is applied. |

Tolerance handling should remain explicit and reportable. The report must show the measured value, required value, unit, tolerance used, sample index, timestamp, channel, and measurement ID.

## Explicit Units Before Shorthand Strings

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

## Candidate Measurement Types

Initial DSL measurement types should map to existing measurement primitives:

| Measurement type | Existing backing logic |
|---|---|
| `minimum_sample` | `minimum_sample` |
| `maximum_sample` | `maximum_sample` |
| `state_transition_count` | `count_state_transitions` |
| `pulse_width` | `state_run_extremum` with `shortest` or `longest` selection |
| `stable_state_duration` | `state_run_extremum` |
| `transient_event_duration` | `state_run_extremum` over the opposite state |
| `rise_time` | `measure_rise_time` |
| `fall_time` | `measure_fall_time` |

Future measurement types such as duty cycle, frequency, period, overshoot, undershoot, RMS, peak-to-peak, and noise floor should be added only after known-answer validation fixtures exist.

## Non-Goals

- No plugin runtime.
- No GUI or interactive form builder.
- No DAQ integration.
- No machine learning.
- No RTOS expansion.
- No unit-shorthand parser in this documentation slice.
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
