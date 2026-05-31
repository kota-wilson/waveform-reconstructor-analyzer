# Environmental Test Use Cases

Date: 2026-05-31

## Purpose

Waveform Reconstructor and Analyzer is meant to help review CSV waveform captures from environmental or bench testing. It is not certified validation software and does not replace equipment procedures or engineering judgment.

## Fixture Intent

| Fixture | Purpose |
|---|---|
| `tests/fixtures/clean_square_wave.csv` | Baseline digital-like waveform with expected transitions and pulse widths. |
| `tests/fixtures/noisy_square_wave.csv` | Square wave with small voltage noise around stable states. |
| `tests/fixtures/analog_switch_bounce_transients.csv` | Short unintended state changes representing switch bounce or transient behavior. |
| `tests/fixtures/dropout_event.csv` | Supply-like waveform with a dropout transient event. |
| `tests/fixtures/slow_rise_fall_signal.csv` | Analog signal with slow rise and fall transitions. |
| `tests/fixtures/multi_channel.csv` | Multi-channel capture with supply, control, and output channels. |

The v0.3.0 validation examples add audit-ready fixture/config/report sets under `validation/`:

| Validation Case | Fixture | Expected Report | Interpretation Notes |
|---|---|---|---|
| Known-answer square wave with tolerances | `validation/known_answer/square_wave_tolerance.csv` | `validation/reports/square_wave_tolerance.json` | `validation/known_answer/expected-measurements.md` |
| Dropout event | `validation/environmental_cases/dropout_event.csv` | `validation/reports/environmental_dropout_fail.json` | `validation/environmental_cases/expected-measurements.md` |
| Contact bounce event | `validation/environmental_cases/contact_bounce.csv` | `validation/reports/environmental_contact_bounce_fail.json` | `validation/environmental_cases/expected-measurements.md` |

## Criteria Mapping

| Criterion | Example Use |
|---|---|
| State transitions | Confirm an expected number of low/high changes occurred. |
| Pulse width | Check high or low state duration against min/max limits. |
| Transient event duration | Catch unintended short state changes such as spurious transitions, contact bounce, dropouts, noise-induced transitions, or threshold crossing events. |
| Stable-state duration | Confirm a signal stayed high or low long enough. |
| Rise/fall time | Check transition speed between configured voltage thresholds. |

## Non-Goals

- GUI.
- Real-time DAQ integration.
- Equipment control.
- Aerospace certification logic.
- Regulatory or safety qualification claims.

## Validation Commands

```bash
cargo run --quiet --bin wra -- analyze \
  --input validation/environmental_cases/dropout_event.csv \
  --config validation/environmental_cases/dropout_event.toml \
  --format json
```

```bash
cargo run --quiet --bin wra -- analyze \
  --input validation/environmental_cases/contact_bounce.csv \
  --config validation/environmental_cases/contact_bounce.toml \
  --format json
```

The expected outputs are stored under `validation/reports/` and compared exactly by `crates/wra-core/tests/criteria_engine.rs`.
