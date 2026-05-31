# ADR-004: Portable Rule Package Architecture

Date: 2026-05-31

Status: Accepted for future milestone planning; not yet implemented

## Context

WRA is evolving from a CSV waveform analyzer into an engineering signal validation platform. The desktop path now covers CSV loading, filtering, criteria evaluation, measurement evidence, SVG plotting, criteria DSL planning, and a separate embedded/no_std foundation.

The next architecture question is how desktop-authored criteria should reach an embedded or controller runtime without creating two independent rule interpretations.

The unacceptable failure mode is:

```text
Desktop analysis says PASS
Controller runtime says FAIL
```

or the reverse, caused by different schemas, different rule engines, unit drift, or inconsistent threshold semantics.

## Decision

WRA will treat RTOS/controller support as a deployment target for the same verified rule system, not as a fork.

The architecture direction is:

```text
Desktop WRA
  -> define criteria
  -> simulate filters
  -> validate against CSV/test data
  -> generate qualification evidence
  -> export WRA Rule Package

WRA Rule Package
  -> one portable schema
  -> versioned manifest
  -> explicit units and sample-rate assumptions
  -> checksummed deployment artifacts

RTOS / Controller Runtime
  -> load same rule package subset
  -> apply same criteria semantics to live ADC/DAQ samples
  -> emit pass/fail or control response
```

The central rule is:

```text
one rule schema
one shared rule engine
multiple frontends and runtimes
```

## Planned Crate Boundaries

| Crate | Responsibility | Constraints |
|---|---|---|
| `wra-rule-schema` | Versioned portable package model for filters, criteria, thresholds, timing limits, sample-rate assumptions, units, channel mapping, manifest fields, and checksum metadata. | Schema ownership only; no CSV parsing, plotting, controller I/O, or hardware claims. |
| `wra-rule-engine` | Executes validated rule packages using shared measurement, criteria, and filter semantics. | Must be usable by desktop/CLI and by embedded-compatible paths without duplicating behavior. |
| `wra-cli` | Desktop command entry point for analysis and future package export. | May read files and write reports; must not own rule semantics. |
| `wra-embedded` | no_std adapter layer for streaming samples and runtime hooks. | No file I/O, CSV, plotting, report rendering, or heap requirement for basic evaluation where possible. |
| `wra-controller-runtime` | Future deployment runtime adapter for controller/RTOS integration. | Target-specific work requires fresh environment, dependency, safety, and scope gates. |

`wra-desktop` is reserved as a possible future desktop application boundary. It is not introduced until a GUI or richer desktop host is separately approved.

## Rule Package Shape

The human-authored format should remain explicit and reviewable:

```toml
[package]
name = "switch-qualification-rule"
version = "1.0.0"
target = "controller"

[channel.switch_signal]
unit = "V"
sample_rate_hz = 10000
high_threshold = 4.5
low_threshold = 0.5

[[filters]]
type = "moving_average"
window_samples = 5

[[criteria]]
id = "CRIT-001"
type = "transient_event"
max_duration_s = 0.001
allowed_count = 0

[[criteria]]
id = "CRIT-002"
type = "stable_state_duration"
state = "high"
min_duration_s = 0.100
```

The project should prefer canonical units such as `V` and `s` over shorthand strings such as `10ms`.

## Deployment Package Shape

Desktop-facing package export may include:

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

Embedded/controller runtimes should consume only the minimal deployment subset needed for runtime operation, likely:

```text
rules.bin
manifest.json
checksum.txt
```

Human reviewers should inspect:

```text
rules.toml
validation-report.json
qualification-evidence.svg
```

## Consequences

- Desktop analysis, CLI analysis, and controller runtime behavior must converge on the same schema and rule engine.
- Parity tests become mandatory before controller-runtime claims.
- The embedded path remains deployment-focused and does not inherit desktop CSV parsing, plotting, report rendering, or GUI concerns.
- Any binary format, checksum algorithm, serialization dependency, controller SDK, HAL, RTOS integration, or hardware execution target requires a fresh dependency/environment/security gate.
- This ADR does not claim production hardware readiness, real-time safety, certification suitability, or controller qualification.

## Verification Expectations

- Rule package schema validation tests.
- Deterministic package export tests.
- Manifest/checksum validation tests.
- Desktop-vs-embedded parity tests using the same waveform, same package, and exact expected result.
- no_std boundary checks for embedded-compatible modules.

## Hand-Off Note

Role: Software Architect
Goal: Record the portable rule package architecture direction before implementation.
Files changed: `decisions/ADR-004-portable-rule-package-architecture.md`.
Checks run: Architecture review by inspection.
Status: Accepted for future milestone planning.
Known gaps: No `wra-rule-schema`, `wra-rule-engine`, export command, binary package, checksum, or controller runtime exists yet.
Next recommended step: Plan v0.6.0 / M8 issues after the active v0.5.0 DSL milestone is completed or explicitly reprioritized.
