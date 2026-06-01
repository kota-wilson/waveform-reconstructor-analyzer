# Controller Config Parity Tests

Status: implemented for M9-009 / issue #85.

The controller parity test uses the existing heated-actuator software fixture:

- waveform input: `tests/e2e/heated_actuator/input/passing_run.csv`
- production control config: `examples/control-config/production-control-config.toml`
- test verification config: `examples/test-verification-config/test-verification-config.toml`
- channel map: `examples/simulation/heated-actuator-channel-map.toml`

The focused test lives in `crates/ferrisoxide-cli/src/main.rs`:

```text
controller_config_and_behavior_paths_match_portable_parity_evidence
```

It compares:

- portable controller state trace fields,
- pass/fail outcomes,
- measured values,
- required values,
- sample indices,
- timestamps,
- channels,
- measurement IDs,
- measurement methods.

## Approved Schema Difference

No embedded controller runtime exists yet. For M9-009, controller state parity compares the portable state trace projection emitted by the deterministic desktop simulator. Criteria parity compares desktop analysis evidence against the embedded-compatible borrowed-rule engine over the same waveform and verification config.

This is software validation evidence only. It is not target firmware execution, hardware timing evidence, or certification evidence.
