# Heated Actuator Qualification Suite

Status: software-only validation fixture.

Issue: #117, `TEST-001 Add heated actuator qualification test suite`

## Purpose

This suite models a heated actuator controller workflow without hardware. It exercises the desktop analysis path as an engineering authoring and validation tool:

```text
simulated command and UUT response
-> DAQ-style CSV
-> FerrisOxide Signal config-driven analysis
-> criteria evidence
-> JSON report and SVG evidence
-> portable rule package export smoke test
```

It is not a live DAQ test, controller-in-the-loop test, hardware qualification result, real-time runtime result, or certification artifact.

## Files

| Path | Purpose |
|---|---|
| `tests/e2e/heated_actuator/input/passing_run.csv` | Command rises at 1.000 s, feedback reaches high at 1.020 s, supply stays in range. |
| `tests/e2e/heated_actuator/input/failing_late_response.csv` | Feedback reaches high at 1.070 s and violates the 50 ms response limit. |
| `tests/e2e/heated_actuator/input/failing_transient_event.csv` | Feedback has a 2.4 ms false-low transient after reaching high. |
| `tests/e2e/heated_actuator/input/failing_supply_dropout.csv` | Supply drops to 4.60 V and violates the 4.75 V minimum. |
| `tests/e2e/heated_actuator/configs/test-verification-config.toml` | Executable FerrisOxide Signal verification config. |
| `tests/e2e/heated_actuator/configs/production-control-config.toml` | Human-readable control-config example; not executed by the current CLI. |
| `tests/e2e/heated_actuator/expected/*.json` | Exact golden JSON report outputs. |

## Criteria

| ID | Criterion | Evidence |
|---|---|---|
| `REQ-001` | `response_latency`: `actuator_feedback_v` must reach high within 50 ms after `command_v` reaches high. | measured latency, required latency, sample index, timestamp, target channel |
| `REQ-002` | `stable_state_duration`: feedback high duration must be at least 500 ms. | longest high run duration |
| `REQ-003` | `transient_event`: feedback must not have a low transient longer than 1 ms after the expected high state is first observed. | longest armed false-low transient |
| `REQ-004-min` | `minimum_voltage`: supply must be at least 4.75 V. | minimum sample |
| `REQ-004-max` | `maximum_voltage`: supply must be at most 5.25 V. | maximum sample |

`REQ-003` uses `start_time_s` plus `arm_after_first_expected_state = true`. That prevents the normal pre-response low state from being counted as a false-low transient while still detecting post-response low events.

## Verification

Primary tests:

```text
cargo test -p ferrisoxide-rule-engine
cargo test -p ferrisoxide-rule-schema
cargo test -p ferrisoxide-core --test heated_actuator
cargo test -p ferrisoxide-cli
```

The core golden tests compare exact JSON for all four scenarios. The CLI tests also prove:

- configured JSON analysis runs,
- failing evidence renders into SVG overlays,
- the verification config exports as a portable rule package with `response_latency`.

## Scope Limits

- No GUI.
- No live DAQ.
- No final controller runtime.
- No RTOS deployment.
- No hardware qualification or certification claim.
