# Controller-In-The-Loop Workflow

Date: 2026-05-31

Status: Planned architecture direction; not yet implemented

## Summary

WRA can evolve from waveform analysis into a controller-in-the-loop qualification platform. The desktop environment should support controller simulation, DAQ acquisition, rule authoring, test verification, and evidence generation. The RTOS/controller environment should later run production control and verification modes using the same approved configs and shared logic where practical.

This direction builds on the portable rule package architecture in `decisions/ADR-004-portable-rule-package-architecture.md`.

The controlling idea is:

```text
Desktop: controller simulation + DAQ acquisition + rule authoring + qualification evidence
RTOS: production controller runtime + test/verification runtime using the same approved rules
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
```

Recommended future crate/module boundaries:

| Crate | Responsibility | Status |
|---|---|---|
| `wra-core` | Desktop-capable waveform, criteria, analysis, and report models. | Exists. |
| `wra-signal` | no_std signal primitives. | Exists. |
| `wra-rule-schema` | Portable test verification rule package schema. | Planned in v0.6.0. |
| `wra-rule-engine` | Shared test verification rule execution semantics. | Planned in v0.6.0. |
| `wra-control-schema` | Production control config schema and state-machine model. | Future v0.7.0 work. |
| `wra-simulator` | Desktop virtual controller simulation engine. | Future v0.7.0 work. |
| `wra-deployment` | Deployment package manifest/export model. | Future v0.7.0 work. |
| `wra-daq` | DAQ input abstraction, initially host/test double friendly. | Future v0.7.0 work. |
| `wra-embedded` | no_std adapter boundaries for runtime integration. | Exists as foundation. |
| `wra-cli` | CLI workflows for analysis, plotting, export, and future simulation commands. | Exists. |

Platform profiles are defined in `docs/platform-targets.md`. Controller-in-the-loop implementation should treat Apple Silicon macOS as the desktop authoring platform and Raspberry Pi 5 bare-metal ARM64 as the first-class embedded runtime target.

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

## Scope Boundaries

In scope for the future milestone:

- production control config schema
- test verification config schema
- virtual controller simulation engine
- DAQ input abstraction with test doubles
- controller I/O abstraction
- desktop simulation workflow
- RTOS deployment package format
- production-vs-test mode separation
- config parity tests
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
Status: Planned architecture direction; not implemented.
Known gaps: No production control schema, simulator, DAQ abstraction, controller I/O abstraction, deployment package implementation, or RTOS verification runtime exists yet.
Next recommended step: Create a future milestone and issues after preserving active v0.5.0 and planned v0.6.0 traceability.
