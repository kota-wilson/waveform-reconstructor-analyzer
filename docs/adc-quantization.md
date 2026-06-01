# ADC Quantization Transform

Date: 2026-05-31

## Scope

The ADC quantization transform simulates an ideal analog-to-digital conversion step before pass/fail criteria are evaluated. It is part of the current ordered `[[filters]]` config pipeline for compatibility, and part of the broader transform capability model because it derives a new waveform from parsed samples without mutating the raw CSV input.

## Configuration

TOML config uses an ordered `[[filters]]` step:

```toml
[[filters]]
type = "adc_quantize"
bits = 8
min_v = 0.0
max_v = 5.0
```

The CLI equivalent is:

```bash
--adc-quantize 8:0.0:5.0
```

## Behavior

- `bits` defines the ADC resolution and must be between 1 and 24.
- `min_v` and `max_v` define the endpoint-inclusive input range.
- Samples below `min_v` clip to `min_v`.
- Samples above `max_v` clip to `max_v`.
- Samples inside the range snap to the nearest ideal ADC code level.
- Output samples remain voltage-domain values so existing voltage criteria can run after digitization.
- The transform applies to every selected channel in the same way.

## Ordering

Filters run in the order they appear in TOML or CLI arguments. This means:

- Moving-average then ADC quantization simulates smoothing before digitization.
- ADC quantization then moving-average simulates smoothing a digitized waveform.
- Criteria always evaluate the final derived waveform after all filter steps.

## Limitations

This is an ideal quantizer only. It does not model sample-and-hold behavior, aperture jitter, conversion latency, offset error, gain error, integral nonlinearity, differential nonlinearity, missing codes, aliasing, or hardware calibration. It is not a DAQ integration or hardware validation feature.

## Verification

Coverage lives in:

- `crates/ferrisoxide-core/src/filter.rs` for quantizer behavior, clipping, raw-data preservation, and ordered chain execution.
- `crates/ferrisoxide-core/src/config.rs` for TOML conversion and missing-field validation.
- `crates/ferrisoxide-cli/src/main.rs` for CLI parsing and pre-criteria execution.
