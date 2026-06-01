# Controller Config Parity

Status: implemented software-only parity test for M9-009 / issue #85.

## Purpose

Controller-in-the-loop workflows need proof that the same package inputs do not produce divergent desktop and embedded-compatible evidence.

The M9-009 parity test uses the same:

- production control config,
- test verification config,
- channel map,
- waveform input,
- selected control mode.

It then compares portable evidence fields between:

```text
desktop simulation + desktop report path
embedded-compatible borrowed-rule criteria path
```

## Test

Focused test:

```bash
cargo test -p ferrisoxide-cli controller_config_and_behavior_paths_match_portable_parity_evidence
```

The test fixture references:

| Input | Path |
|---|---|
| Waveform | `tests/e2e/heated_actuator/input/passing_run.csv` |
| Production control config | `examples/control-config/production-control-config.toml` |
| Test verification config | `examples/test-verification-config/test-verification-config.toml` |
| Channel map | `examples/simulation/heated-actuator-channel-map.toml` |

## Compared Fields

State trace parity:

- sample index,
- timestamp,
- selected mode,
- state-machine ID,
- state,
- transition ID,
- transition from/to states,
- output ID,
- output value.

Criteria evidence parity:

- criterion ID,
- pass/fail outcome,
- failed criterion,
- measurement ID,
- measurement method,
- channel,
- measured value,
- required value,
- tolerance,
- sample index,
- timestamp,
- unit.

## Approved Schema Differences

No embedded controller runtime exists yet. The current state parity check compares the portable state trace projection emitted by the deterministic desktop simulator. A future runtime parity test must replace the embedded-compatible state projection with target/runtime output once a loader and runtime exist.

Desktop reports include reason strings and richer metadata. The borrowed-rule engine returns the portable evidence subset needed by constrained embedded-compatible paths.

## Non-Goals

This test does not prove:

- target firmware execution,
- live DAQ acquisition,
- hardware timing,
- real-time scheduling,
- RTOS loader behavior,
- hardware qualification,
- flight certification.

## Hand-Off Note

Role: Verification and Validation Engineer
Goal: Document controller config and behavior parity tests.
Files changed: `docs/controller-config-parity.md`, `tests/controller_parity/README.md`.
Checks run: See `docs/validation-log.md`.
Status: Implemented locally; PR, protected CI, merge, and issue #85 closure pending.
Known gaps: No embedded controller runtime output, target hardware execution, live DAQ, RTOS loader, hardware timing evidence, or certification evidence.
Next recommended step: Open PR with `Fixes #85`, wait for required CI, and merge only after checks pass.
