# Event Validation Transforms

Date: 2026-06-01

Status: M12 implementation artifact for issues #149 through #155, closed by PR #156.

## Scope

M12 adds a software-only event and validation layer for sampled analog waveforms. It is intended for switch, relay, actuator, and controller-test fixtures where engineers need to see event evidence before pass/fail decisions.

This layer does not add live DAQ, vendor SDKs, HAL/RTOS bindings, target hardware execution, real-time guarantees, hardware qualification, or certification evidence.

## Config Surface

Event work uses two additive TOML tables:

| Table | Purpose |
|---|---|
| `[[event_transforms]]` | Converts waveform samples into event records. |
| `[[event_validations]]` | Produces pass/fail validation records from event/state evidence. |

Implemented event transforms:

| Type | Required fields | Output |
|---|---|---|
| `schmitt_trigger` | `id`, `channel`, `on_threshold_v`, `off_threshold_v`, `initial_state` | State-transition event records and an internal state trace. |
| `debounce` | `id`, `channel`, `min_duration_s` | Rejected-pulse records for internal pulses shorter than the minimum duration. |
| `glitch_removal` | `id`, `channel`, `max_duration_s` | Rejected-pulse records for internal pulses shorter than the maximum duration. |
| `edge_extraction` | `id`, `channel` | Rising/falling edge records from the active state trace. |
| `bounce_detection` | `id`, `channel`, `window_s` | Bounce records with count, duration, and linked source transition IDs. |

Implemented event validations:

| Type | Required fields | Decision |
|---|---|---|
| `missing_pulse` | `id`, `channel`, `direction`; optional `expected_count` defaults to `1` | Passes when enough matching events are observed. |
| `extra_pulse` | `id`, `channel`, `direction`, `max_count` | Fails when matching events exceed the maximum. |
| `dwell_time` | `id`, `channel`, `state`, `min_duration_s` | Passes when the state persists long enough. |
| `timeout` | `id`, `channel`, `direction`, `max_time_s`; optional `start_time_s` defaults to `0.0` | Passes when a matching event occurs before the deadline. |

Direction values are `rising`, `falling`, or `any`. State values are `high` or `low`.

## Report Evidence

JSON reports now include optional top-level arrays when event analysis is configured:

| Field | Meaning |
|---|---|
| `event_records` | Event evidence with ID, transform, kind, channel, sample index, timestamp, state, thresholds, optional duration/count, linked source event IDs, and transform metadata. |
| `event_validations` | Pass/fail validation evidence with requirement ID, validation type, measured and required values, unit, linked event IDs, reason, and transform metadata. |

Event validations participate in `overall_outcome`; a failed event validation fails the report even when ordinary criteria pass.

When explicit edge records exist for a channel/direction, validations count those records. If no explicit edge records exist, validations fall back to Schmitt state-transition records. This prevents double-counting the same transition when both transforms are configured.

## Known-Answer Fixture

The M12 fixture pair is:

- `examples/switch-bounce-waveform.csv`
- `examples/m12-event-validation-config.toml`

Run it:

```bash
cargo run -p ferrisoxide-cli --bin ferrisoxide-signal -- analyze \
  --input examples/switch-bounce-waveform.csv \
  --config examples/m12-event-validation-config.toml \
  --format json
```

Expected fixture behavior:

| Evidence | Expected value |
|---|---|
| Rising edges | 3 explicit `edge` records at 0.001 s, 0.003 s, and 0.005 s. |
| Falling edges | 2 explicit `edge` records at 0.002 s and 0.004 s. |
| Bounce count | 4 post-actuation transitions inside the 0.004 s window. |
| Missing-pulse validation | Pass; at least one rising edge exists. |
| Extra-pulse validation | Pass for the example because `max_count = 3`. |
| Dwell-time validation | Pass; high state persists for 0.001 s. |
| Timeout validation | Pass; first rising edge occurs within 0.002 s. |

## Embedded-Compatible Boundary

The Schmitt state primitive lives in `ferrisoxide-rule-engine` and remains `#![no_std]` compatible over caller-provided slices. The desktop event pipeline in `ferrisoxide-core` performs allocation, report construction, TOML parsing, and JSON rendering, so it remains desktop-only until a later bounded-buffer event runtime is designed and proven.

## Hand-Off Note

Role: Core Software Engineer / Verification and Validation Engineer
Goal: Document M12 event and validation transform behavior.
Files changed: `crates/ferrisoxide-core/src/event.rs`, `crates/ferrisoxide-core/src/config.rs`, `crates/ferrisoxide-core/src/report.rs`, `crates/ferrisoxide-cli/src/main.rs`, `crates/ferrisoxide-rule-engine/src/lib.rs`, `examples/switch-bounce-waveform.csv`, `examples/m12-event-validation-config.toml`, and this document.
Checks run: Targeted event, config, report, CLI, and rule-engine tests; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; local Markdown link-target scan; stale current M12 wording scan; `git diff --check`.
Status: Implemented in PR #156; issues #149 through #155 and milestone #12 are closed.
Known gaps: No live hardware, DAQ SDK, target runtime, or certification evidence.
Next recommended step: Hold before M13 or new scope until explicit approval.
