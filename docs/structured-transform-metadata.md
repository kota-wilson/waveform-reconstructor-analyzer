# Structured Transform Metadata Design

Date: 2026-06-01

Status: M10-002 / issue #133 design artifact, implemented by M10-006 / issue #137 for current moving average, low-pass, and ADC quantization transforms.

## Purpose

FerrisOxide records derived waveform transforms as ordered human-readable strings in `transform_history`. That stays for compatibility, and transformed waveform reports now also include structured `transform_steps` metadata that can be inspected without parsing strings.

This design defines the additive `transform_steps` shape for waveform metadata. Existing `transform_history` remains present, and raw untransformed reports skip `transform_steps` so existing no-transform golden reports remain stable.

## Compatibility Decision

`transform_history` remains the stable compatibility field.

Structured metadata is a sibling field under `waveform_metadata`:

```json
"waveform_metadata": {
  "transform_history": [
    "moving_average(window_samples=2)"
  ],
  "transform_steps": [
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
  ]
}
```

`transform_steps` is serialized only when non-empty unless a schema migration explicitly requires empty arrays. Reports for raw untransformed waveforms remain byte-for-byte stable where no other schema changes are made.

## Field Shape

| Field | Type | Required | Meaning |
|---|---|---|---|
| `sequence_index` | integer | Yes | Zero-based transform order in the evaluated derived waveform. |
| `history_label` | string | Yes | Human-readable label also appended to `transform_history`. |
| `name` | string | Yes | Stable snake_case transform identifier. |
| `category` | string | Yes | Category from `docs/transform-capability-model.md`. |
| `input_channels` | object | Yes | Channel requirement or actual channel set consumed. |
| `output_channels` | object | Yes | Derived channel, event, feature, or validation output behavior. |
| `parameters` | array | Yes | Ordered parameter records with values and units. |
| `sample_rate_required` | boolean | Yes | Whether valid timing/sample-rate metadata is required. |
| `stateful` | boolean | Yes | Whether previous samples or internal state affect current output. |
| `causal` | boolean | Yes | Whether output at sample N depends only on samples up to N. |
| `phase_effect` | string | Yes | `none`, `delay`, `nonlinear`, `zero_phase`, or `not_applicable`. |
| `streaming_supported` | boolean | Yes | Whether bounded streaming execution is possible. |
| `offline_only` | boolean | Yes | Whether full-record access or forward/backward passes are required. |
| `runtime_profiles` | array | Yes | Runtime profiles from the capability model. |
| `capability_status` | string | Yes | Support status from the capability model. |
| `evidence_level` | string | Yes | Strongest evidence level from the capability model. |

## Parameter Records

Parameter records should avoid string parsing and preserve units when relevant.

| Field | Type | Required | Meaning |
|---|---|---|---|
| `name` | string | Yes | Stable parameter name. |
| `value` | number, integer, boolean, string, or array | Yes | Parameter value after validation/defaulting. |
| `unit` | string or null | Yes | Engineering unit or domain unit such as `Hz`, `V`, `samples`, or `s`; use null only for unitless values. |

## Channel Records

The current Rust representation supports the current all-channel derived waveform transforms and keeps the serialized shape open for future multi-channel transforms.

Suggested `input_channels.kind` values:

- `all_channels`
- `single_channel`
- `channel_pair`
- `channel_set`
- `event_stream`
- `feature_records`

Suggested `output_channels.kind` values:

- `derived_channels`
- `derived_channel`
- `event_records`
- `feature_records`
- `validation_records`

## Initial Transform Metadata Expectations

M10-003 owns final mappings for current transforms. This document records the direction those mappings should follow.

| Transform | `name` | `category` | `sample_rate_required` | `stateful` | `causal` | `phase_effect` |
|---|---|---|---|---|---|---|
| Moving average | `moving_average` | `WindowedTransform` | false | true | true | `delay` |
| Low-pass | `low_pass` | `FrequencyFilterTransform` | true | true | true | `delay` |
| ADC quantization | `adc_quantize` | `QuantizationTransform` | false | false | true | `none` |

## Report Schema Direction

`docs/report-schema.md` describes `transform_steps` as an additive emitted field. The report compatibility rule is:

- Keep `transform_history` unchanged.
- Emit `transform_steps` only when metadata mappings and records exist.
- Do not remove or rename existing waveform metadata fields.
- Do not require downstream consumers to parse `transform_history`.
- Update exact golden reports when the structured field is intentionally emitted or changed.

## Golden Report Expectations

M10-006 adds exact tests for these expectations:

- Raw waveform reports with no transforms should stay byte-for-byte stable if `transform_steps` is skipped when empty.
- Transformed waveform reports should add structured metadata in transform order.
- Each `transform_steps[].history_label` should match the corresponding `transform_history[]` entry.
- Tests should assert field values for moving average, low-pass, and ADC quantization metadata.
- Existing result and measurement evidence fields should remain unchanged unless a separate schema migration is approved.

## Rust Model

The Rust model keeps structured transform metadata in `WaveformMetadata`:

```rust
pub struct WaveformMetadata {
    pub transform_history: Vec<String>,
    pub transform_steps: Vec<TransformStepMetadata>,
    // existing fields remain unchanged
}

pub struct TransformStepMetadata {
    pub sequence_index: usize,
    pub history_label: String,
    pub name: String,
    pub category: TransformCategory,
    pub input_channels: TransformInputChannels,
    pub output_channels: TransformOutputChannels,
    pub parameters: Vec<TransformParameterMetadata>,
    pub sample_rate_required: bool,
    pub stateful: bool,
    pub causal: bool,
    pub phase_effect: TransformPhaseEffect,
    pub streaming_supported: bool,
    pub offline_only: bool,
    pub runtime_profiles: Vec<TransformRuntimeProfile>,
    pub capability_status: TransformCapabilityStatus,
    pub evidence_level: TransformEvidenceLevel,
}
```

Implementation should prefer typed enums internally and explicit `serde` names in reports.

## Hand-Off Note

Role: Software Architect
Goal: Complete M10-002 / issue #133 by defining the structured transform metadata design before implementation.
Files changed: `docs/structured-transform-metadata.md`
Checks run: Documentation and schema compatibility review.
Status: Complete through PR #138; issue #133 and milestone #10 are closed.
Known gaps: Runtime profile validation code and embedded/no_std transform exposure remain future gated work.
Next recommended step: Use this design as the compatibility baseline if M11 or M12 is approved.
