# Virtual Controller Simulator

Status: implemented engine boundary for M9-003 / issue #78.

Crate: `crates/ferrisoxide-simulator`

Primary input schema: `crates/ferrisoxide-control-schema`

## Purpose

The virtual controller simulator runs production control state-machine logic against deterministic abstract sample frames. It lets engineers exercise controller behavior before final controller hardware, DAQ wiring, HAL bindings, or RTOS integration exist.

Use the simulator for:

- desktop controller-in-the-loop design studies,
- deterministic state-machine tests,
- command/feedback timing scenarios,
- output-state trace evidence,
- fault-response trace evidence,
- future desktop simulation workflow plumbing.

Do not use the simulator for:

- live DAQ acquisition,
- hardware output control,
- real-time timing guarantees,
- HAL or RTOS integration,
- certified controller qualification,
- proof that firmware timing will match hardware timing.

## Engine Boundary

`ferrisoxide-simulator` consumes:

- `ProductionControlConfig` from `ferrisoxide-control-schema`,
- a mode ID such as `normal`,
- ordered `SimulationInputFrame` values supplied by the caller.

It produces:

- package name/version,
- initial mode,
- one `ControlStateTrace` per input frame,
- active state-machine state,
- transition evidence,
- action evidence,
- output-state evidence,
- fault-response evidence.

The crate does not parse files. CSV loading, DAQ input, CLI wiring, and report rendering belong to later workflow layers.

## Example Flow

The heated actuator example can be simulated with abstract samples:

```text
t=0.000 s: command low, feedback low -> idle
t=1.000 s: command high, feedback low -> command_to_heating -> heating
t=1.020 s: command high, feedback high -> feedback_reached -> idle
```

Timeout behavior is also deterministic:

```text
t=1.000 s: command high, feedback low -> heating
t=1.052 s: feedback still low -> response_timeout -> faulted -> safe mode
```

## Current Evaluation Semantics

The simulator evaluates one transition per state machine per input frame. Transitions are evaluated in config order for the current state.

Supported transition conditions:

| Condition | Behavior |
|---|---|
| `input_above` | Analog input must be greater than the referenced threshold value. |
| `input_below` | Analog input must be less than the referenced threshold value. |
| `input_state` | Digital input must match the required `low` or `high` state. |
| `timer_elapsed` | Elapsed time since state entry must be greater than or equal to the configured duration. |
| `mode_is` | Current mode ID must match the configured mode. |
| `always` | Transition is always eligible. |

Supported actions:

| Action | Behavior |
|---|---|
| `set_output` | Updates the simulated output value and records an action trace. |
| `enter_mode` | Updates the current simulated mode and records an action trace. |
| `raise_fault` | Records a fault trace and applies the referenced fault response. |
| `no_op` | Records an action trace without changing outputs. |

Fault responses may set a safe mode and execute configured safe-output actions. The simulator records the fault response so later report formats can cite the fault evidence.

## Validation And Errors

Before simulation, the engine calls `ProductionControlConfig::validate()`. During simulation it returns structured errors for:

- empty input,
- non-monotonic time,
- unknown mode,
- missing state machines or states,
- missing frame inputs,
- missing thresholds,
- missing actions,
- missing outputs,
- missing fault responses,
- analog/digital input type mismatches,
- non-finite analog values,
- invalid timer conditions.

## Current Limits

The simulator is an engine boundary, not a finished desktop workflow. It does not yet load CSV files, connect the DAQ abstraction to controller input mapping, run test verification criteria, emit JSON reports, render SVG evidence, export deployment packages, or compare desktop-vs-embedded runtime traces. Those are later M9 issues.
