# Controller-In-The-Loop Workflow

Date: 2026-05-31

Status: Partially implemented through fixture-driven desktop simulation, a reviewable deployment package format, manifest-level operating mode separation, and software-only config parity tests; runtime work remains planned.

## Summary

WRA can evolve from waveform analysis into a controller-in-the-loop qualification platform. The desktop environment should support controller simulation, DAQ acquisition, rule authoring, test verification, and evidence generation. The RTOS/controller environment should later run production control and verification modes using the same approved configs and shared logic where practical.

This direction builds on the portable rule package architecture in `decisions/ADR-004-portable-rule-package-architecture.md`.

The controlling idea is:

```text
Desktop: controller simulation + DAQ acquisition + rule authoring + qualification evidence
RTOS/controller runtime: production controller runtime + test/verification runtime using the same approved rules
Pico 2 micro-runtime: constrained deterministic executor for small fixtures and simple controllers
```

## Standard Workflow

1. Subcomponents exist.
2. The final controller does not exist yet.
3. Subcomponents connect to DAQ or test equipment.
4. Desktop software simulates controller logic.
5. Desktop software observes UUT signals.
6. Engineer tunes control configuration.
7. Engineer applies pass/fail criteria.
8. UUT passes environmental or production-style tests.
9. Approved configuration is exported.
10. RTOS/controller runtime consumes approved configuration.
11. Production controller runs normal mode.
12. Test stands run verification mode.

## Configuration Types

Controller-in-the-loop work requires two linked but separate config types.

### Production Control Config

Purpose: define how the controller behaves during normal operation.

Expected contents:

- inputs
- outputs
- thresholds
- state machine definitions
- timing rules
- control actions
- fault responses
- mode definitions
- version and approval metadata

Example artifact:

```text
production-control-config.toml
```

### Test Verification Config

Purpose: define how signals are judged during qualification, environmental testing, or production test.

Expected contents:

- expected transitions
- voltage limits
- pulse widths
- transient limits
- dropout limits
- stable-state requirements
- timing windows
- evidence/report settings
- version and approval metadata

Example artifact:

```text
test-verification-config.toml
```

### Linkage

The configs should be linked through a deployment manifest, not collapsed into one file.

```text
control-config.toml
test-config.toml
deployment-manifest.json
```

The manifest should record compatible versions, target profile, channel map, checksum metadata, generated timestamp, and evidence references.

## Product Architecture

```text
Desktop Simulator
  -> DAQ input layer
  -> virtual controller
  -> control config editor
  -> test criteria editor
  -> waveform analyzer
  -> environmental test evidence reports
  -> deployment package exporter

Shared Core
  -> rule schema
  -> control logic schema
  -> waveform model
  -> criteria engine
  -> simulation engine
  -> parity tests

RTOS Runtime
  -> production mode
  -> test mode
  -> signal validation mode
  -> config loader
  -> controller I/O adapter

Optional Pico 2 Micro-Runtime
  -> compact approved rule subset
  -> fixed-size sample buffers
  -> threshold and timing checks
  -> simple filters
  -> GPIO/PWM output actions
```

Recommended future crate/module boundaries:

| Crate | Responsibility | Status |
|---|---|---|
| `ferrisoxide-core` | Desktop-capable waveform, criteria, analysis, and report models. | Exists. |
| `ferrisoxide-signal` | no_std signal primitives. | Exists. |
| `ferrisoxide-rule-schema` | Portable test verification rule package schema. | Exists. |
| `ferrisoxide-rule-engine` | Shared test verification rule execution semantics. | Exists. |
| `ferrisoxide-control-schema` | Production control config schema and state-machine model. | Exists as an M9-001 schema boundary; execution remains future work. |
| `ferrisoxide-verification-schema` | Test verification config schema for expected transitions, limits, timing windows, evidence, and report settings. | Exists as an M9-002 schema boundary; execution remains future work. |
| `ferrisoxide-simulator` | Desktop virtual controller simulation engine over production control config and abstract sample frames. | Exists as an M9-003 engine boundary and is wired into the M9-006 fixture-driven CLI simulation workflow. |
| `ferrisoxide-deployment` | Deployment package manifest schema, required artifact roles, operating mode profiles, validation helpers, and checksum drift-detection wording. | Exists as an M9-007/M9-008 format and mode-separation boundary; export command and runtime loader remain future work. |
| `ferrisoxide-daq` | DAQ input abstraction, initially host/test double friendly. | Exists as an M9-004 fixture/test-double boundary; vendor SDKs and live hardware remain future gated work. |
| `ferrisoxide-controller-io` | Host-checkable controller input/output abstraction. | Exists as an M9-005 fake I/O boundary; HAL and RTOS SDK adapters remain future gated work. |
| `ferrisoxide-embedded` | no_std adapter boundaries for runtime integration. | Exists as foundation. |
| `ferrisoxide-pico-runtime` | Optional Pico 2 microcontroller adapter for compact configs, fixed buffers, threshold/timing criteria, simple filters, and GPIO/PWM actions. | Future issue #92 work; not implemented. |
| `ferrisoxide-cli` | CLI workflows for analysis, plotting, export, and fixture-driven desktop simulation. | Exists; `simulate` loads production control config, test verification config, a channel map, and fixture CSV input. |

Platform profiles are defined in `docs/platform-targets.md`. Controller-in-the-loop implementation should treat Apple Silicon macOS as the desktop authoring platform, Raspberry Pi 5 bare-metal ARM64 as the first-class richer embedded runtime target, and Raspberry Pi Pico 2 as a later optional microcontroller target for constrained deterministic rule execution.

## Operating Modes

### Desktop Simulation Mode

Purpose: simulate controller behavior before the physical controller exists.

Inputs:

- DAQ/test-equipment signal input, fixture input, or generated waveform.
- Production control config.
- Channel map.
- Optional test verification config.

Outputs:

- simulated controller outputs
- waveform analysis results
- control-state trace
- evidence report

Rules:

- Desktop simulation must not create a desktop-only interpretation of controller logic.
- Simulation logic should use the same state-machine definitions intended for RTOS/controller deployment.
- DAQ integration requires a separate dependency, security, and environment gate.

### Qualification Mode

Purpose: judge observed UUT behavior against test verification criteria.

Inputs:

- waveform data from CSV, DAQ abstraction, or recorded fixture
- test verification config
- channel map
- tolerance policy

Outputs:

- pass/fail report
- failed criterion evidence
- measured value and required value
- sample index, timestamp, and channel
- qualification evidence SVG where applicable

Rules:

- Qualification evidence is software evidence unless separately qualified.
- Environmental test language must not imply certification or hardware qualification authority.

### Production Deployment Mode

Purpose: export approved control and verification configs for controller/runtime consumption.

Inputs:

- production control config
- test verification config
- channel map
- validation/qualification evidence

Outputs:

```text
deployment-package/
  production-control-config.toml
  test-verification-config.toml
  channel-map.toml
  manifest.json
  checksum.txt
  qualification-report.json
  qualification-evidence.svg
  generated-at.txt
```

Rules:

- Production config and test verification config remain separate.
- The manifest links versions and checksums.
- Checksums are integrity evidence, not signing, security certification, or release approval.
- The current implemented package format is documented in `docs/rtos-deployment-package-format.md` and represented by `crates/ferrisoxide-deployment`.
- Mode profiles are documented in `docs/controller-operating-modes.md`; production control, test verification, and signal validation purposes must remain separate.

### RTOS Verification Mode

Purpose: let a test stand or embedded runtime verify live signals using approved test rules.

Inputs:

- constrained deployment package subset
- live samples from controller I/O or DAQ-like adapter
- runtime mode selection

Outputs:

- pass/fail status
- fault or test response
- compact evidence event stream where supported

Rules:

- RTOS verification mode must use shared rule semantics.
- Runtime code must not depend on desktop CSV parsing, plotting, rich report rendering, or GUI concerns.
- Raspberry Pi 5 bare-metal ARM64 is the first-class embedded target.
- RTOS compatibility is a later layer around the Raspberry Pi 5 bare-metal target, not a generic replacement target.
- Target SDK, HAL, QEMU execution, Zephyr production support, and real hardware validation require separate gates.

### Pico 2 Micro-Runtime Mode

Purpose: run a small deterministic subset of approved rules on Raspberry Pi Pico 2 for production-test fixtures, subcomponent validators, or simple controller loops.

Inputs:

- compact approved rule subset
- fixed-size sample buffer or streaming ADC sample input
- GPIO/PWM output mapping where applicable

Outputs:

- pass/fail flag
- compact event/status record where supported
- GPIO/PWM control response where configured

Rules:

- Pico 2 is not the full controller-computer target.
- The Pico 2 path must remain `no_std` and avoid heap requirements where practical.
- The Pico 2 path must not include CSV parsing, SVG/report generation, desktop simulation, large waveform storage, complex filtering, or large multi-channel analysis.
- Unsupported rule/package features must return target-profile validation errors before deployment.
- HALs, ADC drivers, PIO drivers, probe tooling, compact binary config loaders, and hardware validation require separate future gates.

## Desktop And RTOS Parity Rules

The desktop simulator and RTOS/controller runtime must share:

- same production control config schema
- same test verification config schema
- same rule engine
- same state-machine definitions
- same timing assumptions
- same pass/fail logic
- same channel map semantics
- same version and checksum validation rules

The Pico 2 micro-runtime must share the same semantics for its supported compact subset. Features outside that subset must be rejected during package validation instead of reimplemented with approximate or divergent behavior.

Required parity test shape:

```text
tests/controller_parity/
  waveform_001.csv
  production-control-config.toml
  test-verification-config.toml
  channel-map.toml
  expected-result.json
```

Both desktop and embedded-compatible paths must produce matching results for:

- pass/fail
- active mode
- state transition trace where applicable
- measured value
- required value
- sample index
- timestamp
- channel
- evidence identifier

Any difference must be documented as an approved schema difference before merge.

The first software-only controller parity test is implemented as `controller_config_and_behavior_paths_match_portable_parity_evidence` in `crates/ferrisoxide-cli/src/main.rs` and documented in `docs/controller-config-parity.md`. It uses the heated-actuator fixture, production control config, test verification config, and channel map. Until an embedded controller runtime exists, the approved schema difference is that state parity compares the portable simulator trace projection while criteria parity compares desktop report evidence against the embedded-compatible borrowed-rule engine.

## Scope Boundaries

In scope for the future milestone:

- production control config schema
- test verification config schema
- virtual controller simulation engine
- DAQ input abstraction with test doubles
- controller I/O abstraction
- desktop simulation workflow
- RTOS deployment package format, now implemented as a schema and example fixture boundary
- production-vs-test mode separation, now implemented at the deployment manifest validation boundary
- config parity tests, now implemented as software-only desktop-vs-embedded-compatible evidence parity
- qualification evidence report format

Out of scope until separate approval:

- GUI
- live DAQ vendor SDKs
- hardware HALs
- target-specific RTOS integration
- Zephyr production support
- real-time timing guarantee
- cryptographic signing
- safety certification
- hardware qualification claim
- flight certification claim

## Relationship To v0.6.0

The v0.6.0 portable rule package work should establish the rule package schema, shared rule engine boundary, and desktop-vs-embedded parity tests for test verification rules.

The controller-in-the-loop milestone builds on that by adding production control configuration, state-machine simulation, DAQ/controller I/O abstractions, operating modes, and deployment package separation.

## Hand-Off Note

Role: Software Architect / Embedded RTOS Engineer
Goal: Define the controller-in-the-loop workflow and deployment configuration architecture.
Files changed: `docs/controller-in-the-loop-workflow.md`.
Checks run: Architecture review by inspection.
Status: Partially implemented; fixture-driven desktop simulation, deployment package format, manifest-level mode separation, and software-only config parity boundaries exist, while runtime work remains planned.
Known gaps: No controller deployment export command, live DAQ SDK integration, HAL/RTOS controller I/O adapter, RTOS verification runtime, runtime mode switcher, target-runtime parity output, or formal qualification evidence report schema exists yet.
Next recommended step: Continue M9 issue work with qualification evidence reports.
