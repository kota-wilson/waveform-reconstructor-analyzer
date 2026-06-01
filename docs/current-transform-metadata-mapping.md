# Current Transform Metadata Mapping

Date: 2026-06-01

Status: M10-003 / issue #134 mapping artifact, implemented in report metadata and tests by M10-006 / issue #137.

## Purpose

FerrisOxide currently implements three transform steps in `crates/ferrisoxide-core/src/filter.rs`:

- `moving_average`
- `low_pass`
- `adc_quantize`

This document defines the structured metadata values these transforms emit in `waveform_metadata.transform_steps`.

## Compatibility Rule

The existing `transform_history` strings remain the compatibility field:

- `moving_average(window_samples={window_samples})`
- `low_pass(cutoff_hz={cutoff_hz})`
- `adc_quantize(bits={bits},min_v={min_v},max_v={max_v})`

Structured metadata must keep `history_label` equal to the matching `transform_history` entry for the same sequence index.

## Shared Mapping Rules

| Field | Rule |
|---|---|
| `sequence_index` | Zero-based position in the applied filter/transform chain. |
| `input_channels.kind` | `all_channels` for all current transforms. |
| `output_channels.kind` | `derived_channels` for all current transforms. |
| `output_channels.preserves_names` | `true` for all current transforms. |
| `runtime_profiles` | `["desktop"]` for current implementation support. |
| `capability_status` | `implemented` for all current transforms. |
| `evidence_level` | `golden_report_tested` for all current transforms because current behavior is covered by unit/fixture/golden report paths. |

No current transform is exposed as Raspberry Pi 5 no_std or Pico 2 runtime support by this mapping. Those profiles require later explicit code, target, and parity evidence.

## Moving Average

| Field | Value |
|---|---|
| `name` | `moving_average` |
| `category` | `WindowedTransform` |
| `history_label` | `moving_average(window_samples={window_samples})` |
| `parameters` | `window_samples` as integer with unit `samples` |
| `sample_rate_required` | `false` |
| `stateful` | `true` |
| `causal` | `true` |
| `phase_effect` | `delay` |
| `streaming_supported` | `true` |
| `offline_only` | `false` |

Behavior notes:

- Current implementation uses a trailing window that includes the current sample.
- `window_samples` must be greater than zero.
- Edge behavior uses shorter windows at the beginning of the signal.
- It preserves channel names and units.

Example emitted metadata:

```json
{
  "sequence_index": 0,
  "history_label": "moving_average(window_samples=2)",
  "name": "moving_average",
  "category": "WindowedTransform",
  "input_channels": {
    "kind": "all_channels"
  },
  "output_channels": {
    "kind": "derived_channels",
    "preserves_names": true
  },
  "parameters": [
    {
      "name": "window_samples",
      "value": 2,
      "unit": "samples"
    }
  ],
  "sample_rate_required": false,
  "stateful": true,
  "causal": true,
  "phase_effect": "delay",
  "streaming_supported": true,
  "offline_only": false,
  "runtime_profiles": [
    "desktop"
  ],
  "capability_status": "implemented",
  "evidence_level": "golden_report_tested"
}
```

## Low-Pass

| Field | Value |
|---|---|
| `name` | `low_pass` |
| `category` | `FrequencyFilterTransform` |
| `history_label` | `low_pass(cutoff_hz={cutoff_hz})` |
| `parameters` | `cutoff_hz` as number with unit `Hz` |
| `sample_rate_required` | `true` |
| `stateful` | `true` |
| `causal` | `true` |
| `phase_effect` | `delay` |
| `streaming_supported` | `true` |
| `offline_only` | `false` |

Behavior notes:

- Current implementation is a first-order low-pass smoothing filter.
- `cutoff_hz` must be greater than zero.
- Time samples must be strictly increasing.
- The equation uses adjacent timestamp deltas and assumes the configured time axis is compatible with seconds when `cutoff_hz` is interpreted as Hz.
- It preserves channel names and units.

Example emitted metadata:

```json
{
  "sequence_index": 0,
  "history_label": "low_pass(cutoff_hz=10)",
  "name": "low_pass",
  "category": "FrequencyFilterTransform",
  "input_channels": {
    "kind": "all_channels"
  },
  "output_channels": {
    "kind": "derived_channels",
    "preserves_names": true
  },
  "parameters": [
    {
      "name": "cutoff_hz",
      "value": 10.0,
      "unit": "Hz"
    }
  ],
  "sample_rate_required": true,
  "stateful": true,
  "causal": true,
  "phase_effect": "delay",
  "streaming_supported": true,
  "offline_only": false,
  "runtime_profiles": [
    "desktop"
  ],
  "capability_status": "implemented",
  "evidence_level": "golden_report_tested"
}
```

## ADC Quantization

| Field | Value |
|---|---|
| `name` | `adc_quantize` |
| `category` | `QuantizationTransform` |
| `history_label` | `adc_quantize(bits={bits},min_v={min_v},max_v={max_v})` |
| `parameters` | `bits` as integer with unit `bits`; `min_v` and `max_v` as numbers with unit `V` |
| `sample_rate_required` | `false` |
| `stateful` | `false` |
| `causal` | `true` |
| `phase_effect` | `none` |
| `streaming_supported` | `true` |
| `offline_only` | `false` |

Behavior notes:

- Current implementation simulates ideal endpoint-inclusive ADC code quantization.
- `bits` must be between 1 and 24.
- `min_v` and `max_v` must be finite, and `max_v` must be greater than `min_v`.
- Samples outside the configured voltage range are clipped before code conversion.
- Output values remain in volts for downstream criteria.
- It preserves channel names and units.

Example emitted metadata:

```json
{
  "sequence_index": 0,
  "history_label": "adc_quantize(bits=8,min_v=0,max_v=5)",
  "name": "adc_quantize",
  "category": "QuantizationTransform",
  "input_channels": {
    "kind": "all_channels"
  },
  "output_channels": {
    "kind": "derived_channels",
    "preserves_names": true
  },
  "parameters": [
    {
      "name": "bits",
      "value": 8,
      "unit": "bits"
    },
    {
      "name": "min_v",
      "value": 0.0,
      "unit": "V"
    },
    {
      "name": "max_v",
      "value": 5.0,
      "unit": "V"
    }
  ],
  "sample_rate_required": false,
  "stateful": false,
  "causal": true,
  "phase_effect": "none",
  "streaming_supported": true,
  "offline_only": false,
  "runtime_profiles": [
    "desktop"
  ],
  "capability_status": "implemented",
  "evidence_level": "golden_report_tested"
}
```

## Chained Transform Mapping

For a chain such as:

```text
moving_average(window_samples=2) -> adc_quantize(bits=8,min_v=0,max_v=5)
```

`transform_steps` should have:

- two records
- `sequence_index` values `0` and `1`
- `history_label` values that exactly match `transform_history[0]` and `transform_history[1]`
- channel/output behavior repeated for each step unless a later transform changes output kind

## Hand-Off Note

Role: Systems Engineer / Software Architect
Goal: Complete M10-003 / issue #134 by mapping current implemented transforms to structured metadata values.
Files changed: `docs/current-transform-metadata-mapping.md`
Checks run: Documentation and compatibility review.
Status: Implemented locally as mapping and emitted report metadata for current transforms.
Known gaps: Runtime profile validation code and embedded/no_std transform exposure remain future work.
Next recommended step: Use this mapping for future transform additions and runtime-profile validation code.
