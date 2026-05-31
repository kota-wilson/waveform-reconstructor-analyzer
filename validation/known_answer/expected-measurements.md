# Known-Answer Expected Measurements

Date: 2026-05-31

## Scope

These fixtures are synthetic software validation inputs with expected values calculated from the CSV timestamps and samples before running the analyzer. They are not hardware validation or certification evidence.

## `square_wave_tolerance.csv`

Generation method: hand-authored square-wave samples at 1 ms intervals.

Tolerance policy:

- `voltage_v = 0.02`
- `time_s = 0.0005`

Expected measurements:

| Criterion | Expected Measurement | Required Value | Expected Outcome | Reason |
|---|---:|---:|---|---|
| `control_transition_count` | `4 transitions` | `4 transitions` | Pass | State changes occur at sample indices 2, 4, 6, and 8. |
| `control_high_pulse_width_tolerance` | `0.002 s` | `0.0025 s` | Pass | The measured high pulse is 2 ms and passes with the configured 0.5 ms time tolerance. |
| `control_max_voltage_tolerance` | `5.01 V` | `5.0 V` | Pass | The measured maximum is 10 mV above the limit and passes with the configured 20 mV voltage tolerance. |

Expected report artifact:

- `validation/reports/square_wave_tolerance.json`

Test coverage:

- `validation_known_answer_square_wave_matches_expected_report`
