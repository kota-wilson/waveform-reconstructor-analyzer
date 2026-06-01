# Filter Behavior

Current transform metadata mappings for these implemented filters and ADC quantization are defined in `docs/current-transform-metadata-mapping.md`.

Date: 2026-05-31

## Scope

This document describes the implemented transform equations used by `crates/ferrisoxide-core/src/filter.rs`. The file keeps the current `filter` naming because the compatible config surface is still `[[filters]]`, but the documented behavior maps into the broader transform vocabulary. These are software model definitions for review and testing. They are not validated hardware models, frequency-response guarantees, ADC calibration models, or DAQ behavior claims.

## Moving Average

Implementation: `MovingAverageFilter`

For input samples `x[i]` and configured window length `w`, the output sample is:

```text
start(i) = max(0, i + 1 - w)
y[i] = sum(x[k] for k = start(i)..i) / (i - start(i) + 1)
```

Behavior:

- The window is trailing and includes the current sample.
- Beginning samples use a partial window rather than padding.
- `window_samples` must be greater than zero.
- The filter applies independently to every channel and returns a derived waveform.
- Raw input waveform samples are not mutated.

Corresponding tests:

- `filter::tests::moving_average_filters_each_channel_without_mutating_input`
- `filter::tests::moving_average_rejects_zero_window`
- `filter::tests::filter_chain_applies_steps_in_order`

## First-Order Low-Pass

Implementation: `LowPassFilter`

For cutoff frequency `f_c`, adjacent time step `dt[i] = t[i] - t[i-1]`, and `tau = 1 / (2*pi*f_c)`:

```text
y[0] = x[0]
alpha[i] = dt[i] / (tau + dt[i])
y[i] = y[i-1] + alpha[i] * (x[i] - y[i-1])
```

Behavior:

- `cutoff_hz` must be greater than zero.
- The time axis must be strictly increasing.
- Non-uniform but increasing time steps are accepted by the equation.
- The first output sample equals the first input sample.
- The filter is a simple one-pole smoother and does not claim validated frequency response.

Corresponding tests:

- `filter::tests::low_pass_smooths_step_input`
- `filter::tests::filter_chain_applies_steps_in_order`

## Ideal ADC Quantization

Implementation: `AdcQuantizer`

For `bits`, input range `[min_v, max_v]`, and input sample `x[i]`:

```text
max_code = 2^bits - 1
normalized = clamp((x[i] - min_v) / (max_v - min_v), 0, 1)
code = round(normalized * max_code)
y[i] = min_v + (code / max_code) * (max_v - min_v)
```

Behavior:

- `bits` must be in the range `1..=24`.
- `min_v` and `max_v` must be finite, and `max_v` must be greater than `min_v`.
- Samples below `min_v` clip to `min_v`.
- Samples above `max_v` clip to `max_v`.
- Endpoints are included.
- Output remains in volts so normal voltage criteria can evaluate the digitized waveform.
- The model does not include ADC nonlinearity, jitter, aperture effects, aliasing, calibration, or conversion latency.

Corresponding tests:

- `filter::tests::adc_quantizer_snaps_samples_to_code_levels_without_mutating_input`
- `filter::tests::adc_quantizer_rejects_invalid_parameters`
- `config::tests::converts_adc_quantizer_config_to_filter_step`
- `ferrisoxide-cli::tests::runs_analysis_with_adc_quantization_before_criteria`
