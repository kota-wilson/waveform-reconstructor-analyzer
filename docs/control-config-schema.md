# Production Control Config Schema

Status: implemented schema boundary for M9-001 / issue #77.

Crate: `crates/ferrisoxide-control-schema`

Example: `examples/control-config/production-control-config.toml`

## Purpose

The production control config schema defines how a future controller should behave during normal operation. It is separate from the test verification config and portable rule package schema.

Use this schema for:

- controller inputs,
- controller outputs,
- thresholds used by control logic,
- state machine definitions,
- timing rules,
- control actions,
- fault responses,
- operating modes,
- version and approval metadata.

Do not use this schema for:

- waveform CSV parsing,
- test verification criteria,
- JSON analysis report rendering,
- SVG plotting,
- DAQ SDK integration,
- hardware HAL binding,
- RTOS production integration,
- certified controller release approval.

## Relationship To Test Verification Config

FerrisOxide now has two distinct config families:

| Config family | Purpose | Current crate |
|---|---|---|
| Production control config | Defines how controller logic behaves. | `ferrisoxide-control-schema` |
| Test verification config | Defines how observed waveform behavior should be judged during qualification or production-test workflows. | `ferrisoxide-verification-schema` |
| Portable rule package | Defines executable/shared rule package artifacts exported from verified config and analysis evidence. | `ferrisoxide-rule-schema` |

The two config families may be linked later by a deployment manifest, but they should not be collapsed into one file. This keeps production behavior separate from qualification or production-test evidence.

## Top-Level Shape

```toml
[package]
name = "heated-actuator-production-control"
version = "0.1.0"
schema_version = "0.1.0"

[target]
kind = "controller_runtime"
identifier = "raspberry-pi-5-bare-metal"

[approval]
status = "draft"

[timing]
clock = "simulated"
control_loop_period_s = 0.001
```

Top-level sections:

| Section | Required | Purpose |
|---|---:|---|
| `package` | Yes | Name, version, schema version, and optional description. |
| `target` | Yes | Target kind and identifier. |
| `approval` | Yes | Draft/reviewed/approved/retired status plus approval evidence. |
| `timing` | Yes | Clock source and timing assumptions. |
| `inputs` | Yes | Control inputs, sources, signal types, units, and sample periods. |
| `outputs` | Yes | Control outputs, sinks, signal types, units, and safe states. |
| `thresholds` | No | Named thresholds referenced by state transitions. |
| `modes` | Yes | Operating modes and enabled state machines. |
| `state_machines` | Yes | States, transitions, conditions, and transition actions. |
| `timing_rules` | No | Timing limits for loops, state machines, transitions, or actions. |
| `actions` | No | Reusable output, mode, fault, or no-op actions. |
| `fault_responses` | No | Fault severity, latching behavior, safe mode, and actions. |

## Validation Rules

`ProductionControlConfig::validate()` checks:

- package name and version are not empty,
- schema version matches `0.1.0`,
- approved configs identify approver and approval time,
- timing values are finite and non-negative where applicable,
- at least one input, output, mode, and state machine exists,
- IDs are unique inside each collection,
- thresholds reference existing inputs,
- threshold units match referenced input units,
- modes reference existing state machines and actions,
- actions reference existing outputs, modes, or fault responses,
- state machines reference existing states,
- transitions reference existing states, inputs, thresholds, modes, actions, and fault responses,
- PWM duty cycle values are finite and between `0.0` and `1.0`.

Validation returns a `ControlConfigValidationReport` with structured errors. It should not panic on malformed config data.

## Example Control Logic

The example heated actuator config models a simple controller:

```text
idle
  -> command high
  -> heating
  -> feedback high
  -> idle

heating
  -> response timeout
  -> faulted
```

Representative transition:

```toml
[[state_machines.transitions]]
id = "command_to_heating"
from = "idle"
to = "heating"
actions = ["heater_on", "pwm_drive"]
condition = { type = "input_above", input = "command", threshold = "command_high" }
```

Representative action:

```toml
[[actions]]
type = "set_output"
id = "heater_on"
output = "heater_enable"
value = { type = "digital", state = "high" }
```

Representative fault response:

```toml
[[fault_responses]]
id = "actuator_response_fault"
severity = "critical"
latch = true
safe_mode = "safe"
actions = ["heater_off", "pwm_zero"]
```

## Current Limits

The schema is intentionally data-only. It does not execute the state machine, command hardware, load packages on a target, or prove timing on real devices.

Future M9 issues should build on this in order:

1. Add a virtual controller simulation engine.
2. Add DAQ and controller I/O abstractions.
3. Add desktop simulation workflow.
4. Add deployment package format and parity tests.
