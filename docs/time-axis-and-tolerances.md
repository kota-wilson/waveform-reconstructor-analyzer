# Time Axis And Tolerances

Date: 2026-05-31

## Time Axis Rules

The waveform model records the time unit, sample interval summary, and nominal sample rate in `WaveformMetadata`.

For adjacent samples:

```text
interval[i] = time[i] - time[i-1]
sample_interval.min = minimum interval
sample_interval.max = maximum interval
sample_interval.nominal = average interval
sample_interval.uniform = abs(max - min) <= 1e-12
nominal_sample_rate_hz = 1 / nominal when time_unit == "s" and nominal > 0
```

Duration-dependent criteria require strictly increasing timestamps before evaluation:

- `pulse_width`
- `transient_duration`
- `transient_event`
- `stable_state_duration`
- `rise_fall_time`

Duplicate or decreasing timestamps return a clear `InvalidWaveform` error before criteria evaluation. Non-uniform but increasing timestamps are accepted and measured using the actual time axis. The current implementation does not resample, interpolate, synchronize channels, or model DAQ clock behavior.

## Tolerance Policy

Tolerances are configured in TOML:

```toml
[tolerances]
voltage_v = 0.01
time_s = 0.0005
```

Both values default to zero when omitted. Both must be finite and greater than or equal to zero.

Voltage tolerance is applied to voltage comparisons:

- Minimum voltage passes when `measured + voltage_v >= required`.
- Maximum voltage passes when `measured - voltage_v <= required`.
- Threshold-state classification treats a sample as high when `sample + voltage_v >= threshold_v`.

Time tolerance is applied to duration comparisons:

- Minimum duration passes when `measured + time_s >= required`.
- Maximum duration passes when `measured - time_s <= required`.

Reports record the tolerance used for each criterion result and the report-level tolerance policy. Tolerances are engineering decision margins only; they do not represent uncertainty propagation, calibration records, tool qualification, or certification evidence.

## Test Coverage

Relevant tests:

- `analysis::tests::applies_voltage_and_time_tolerances`
- `analysis::tests::still_fails_beyond_configured_tolerance`
- `analysis::tests::rejects_duplicate_or_decreasing_time_for_duration_criteria`
- `analysis::tests::allows_non_uniform_but_increasing_time_axis`
- `config::tests::rejects_invalid_tolerance_config`
- `model::tests::computes_sample_interval_and_rate_metadata`
- `validation_known_answer_square_wave_matches_expected_report`
