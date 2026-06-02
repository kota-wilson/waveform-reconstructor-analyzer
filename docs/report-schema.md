# Analysis Report Schema

Date: 2026-06-02

## Scope

This document describes the MVP JSON report shape used by golden tests and validation reports. Generated artifact compatibility expectations are recorded in `docs/artifact-contract.md`.

## Top-Level Fields

| Field | Type | Meaning |
|---|---|---|
| `input_name` | string | Input path or display name passed to the report. |
| `waveform_metadata` | object | Source, unit, count, time-axis, lineage, and transform context for the analyzed waveform. |
| `evidence_context` | object | Report-level engineering-validation context, evidence source, tolerance policy, and confidence notes. |
| `overall_outcome` | `pass` or `fail` | `fail` when any criterion or event validation fails. |
| `measurements` | array | Reusable measurement evidence records with stable IDs. |
| `feature_records` | array, omitted when empty | Scalar feature evidence emitted by configured `[[feature_transforms]]`. Feature records do not affect `overall_outcome`. |
| `event_records` | array, omitted when empty | Event evidence records emitted by configured event transforms. |
| `event_validations` | array, omitted when empty | Pass/fail evidence emitted by configured event validation transforms. |
| `results` | array | Per-criterion evidence rows. |

## Waveform Metadata Fields

| Field | Type | Meaning |
|---|---|---|
| `source_name` | string or null | Source path or display name when known. |
| `test_run_id` | string or null | Optional local validation or test-run identifier from config metadata. |
| `acquisition_notes` | string or null | Optional local notes about the fixture or acquisition context. |
| `environment` | string or null | Optional local environment descriptor. |
| `operator` | string or null | Optional operator or automation descriptor. |
| `time_unit` | string | Unit used for the waveform time axis. |
| `sample_count` | integer | Number of waveform samples. |
| `channel_count` | integer | Number of analyzed channels. |
| `channels` | array | Channel names and units present in the waveform. |
| `sample_interval` | object or null | Minimum, maximum, nominal, unit, and uniformity summary for adjacent time samples. |
| `nominal_sample_rate_hz` | number or null | Derived sample rate when the time unit is seconds and the nominal interval is positive. |
| `lineage` | `raw` or `derived` | Whether criteria evaluated raw parsed samples or a derived waveform. |
| `transform_history` | array | Ordered transform descriptions applied before criteria evaluation. |
| `transform_steps` | array, omitted when empty | Structured transform metadata emitted for transformed waveforms. |
| `tolerance_policy` | object or null | Voltage/time tolerance policy attached to the analyzed waveform. |

## Structured Transform Metadata Field

M10-006 implements the additive `waveform_metadata.transform_steps` field described in `docs/structured-transform-metadata.md`. The compatibility rule is:

- Keep `transform_history` unchanged.
- Emit `transform_steps` only when at least one structured transform record exists.
- Do not remove or rename existing waveform metadata fields.
- Skip `transform_steps` for raw or untransformed waveform reports unless a later schema migration explicitly changes that rule.
- Update exact golden reports whenever structured transform metadata intentionally changes.

Current waveform transform records cover `moving_average`, `low_pass`, `adc_quantize`, `offset`, `gain`, `invert`, `clamp`, `deadband`, `dc_remove`, `baseline_subtract`, `high_pass_baseline`, `moving_median`, M26 data-cleaning/timing transforms, M27 pointwise/nonlinear transforms, M28 smoothing/baseline transforms, M29 frequency filters, M30 resampling/timing transforms, M31 envelope/calculus waveform filters, M32 statistics waveform filters, and M34 fault-injection/ADC-DAC simulation filters. Each record includes `sequence_index`, `history_label`, `name`, `category`, channel behavior, parameters with units, sample-rate requirement, statefulness, causality, phase effect, streaming/offline flags, runtime profile, capability status, and evidence level.

M12 event reports reuse the same metadata shape inside `event_records[].transform_metadata` and `event_validations[].transform_metadata`. Event transforms use `output_channels.kind = event_records`; validation transforms use `output_channels.kind = validation_records`. M31-M33 feature reports reuse the same metadata shape inside `feature_records[].transform_metadata` and use `output_channels.kind = feature_records`.

## Evidence Context Fields

| Field | Type | Meaning |
|---|---|---|
| `validation_profile` | string | Current profile is `engineering_validation`, meaning local software-analysis evidence. |
| `evidence_source` | string | Current source is `local_file_analysis`, meaning a local CSV/config produced the report. |
| `tolerance_policy` | object | Report-level voltage/time tolerance policy applied to criteria evaluation. |
| `confidence_notes` | array | Human-readable scope notes. Current notes explicitly say the report is not hardware qualification or certification evidence. |

## Measurement Fields

The `measurements` array separates measured signal evidence from pass/fail criteria decisions. Each criterion result that was evaluated from a measurement references one record by `measurement_id`.

| Field | Type | Meaning |
|---|---|---|
| `id` | string | Stable report-local measurement ID. Current IDs use `{criterion_id}_measurement`. |
| `channel` | string | Channel measured. |
| `method` | string | Measurement method, such as `minimum_sample`, `maximum_sample`, `state_transition_count`, `state_run_duration`, `response_latency`, or `edge_time`. |
| `measured_value` | number | Observed value before criteria policy is applied. |
| `unit` | string | Unit for the measured value. |
| `sample_index` | integer | Evidence sample index. |
| `timestamp` | number | Evidence timestamp in seconds. |
| `method_context` | object | Method parameters needed to interpret the measurement. |

## Measurement Method Context Fields

| Field | Type | Meaning |
|---|---|---|
| `source` | string | Measurement implementation source, such as `ferrisoxide-measurements` or `ferrisoxide-rule-engine`. |
| `threshold_v` | number or null | Threshold used for state-based measurements. |
| `low_threshold_v` | number or null | Lower edge threshold for rise/fall measurements. |
| `high_threshold_v` | number or null | Upper edge threshold for rise/fall measurements. |
| `state` | string or null | Measured state such as `high` or `low` when applicable. |
| `expected_state` | string or null | Expected steady state for transient-event style measurements. |
| `event_kind` | string or null | Transient subtype, such as `dropout` or `contact bounce`, when applicable. |
| `direction` | string or null | Edge direction, `rise` or `fall`, when applicable. |
| `selection` | string or null | Run-selection policy, such as `shortest`, `longest`, or `first_response`, when applicable. |

## Feature Record Fields

`feature_records` is emitted only when `[[feature_transforms]]` are configured. Feature records are evidence, not pass/fail decisions.

| Field | Type | Meaning |
|---|---|---|
| `id` | string | Stable config-provided feature ID. |
| `transform` | string | Feature transform name, such as `rms`, `energy`, `mean`, `histogram`, `correlation`, `fft`, `welch_psd`, `spectrogram`, or `band_power`. |
| `channel` | string | Source channel after configured filters have been applied. |
| `value` | number | Calculated scalar feature value. |
| `unit` | string | Source unit, `count`, `ratio`, `Hz`, `dB`, `bits`, `<unit>^2*s`, `<unit>^2`, `<unit>^2/Hz`, `<unit>*s`, `<unit>*<other_unit>`, or `<other_unit>/<unit>` depending on feature type. |
| `method_context` | object, omitted when empty | Optional method-specific context for percentile/quantile, histogram bins, comparison channel, lag, frequency bins, windows, harmonics, bands, or time-frequency segments. |
| `transform_metadata` | object | Structured metadata for the feature transform. |

## Feature Method Context Fields

`method_context` is additive and appears only when a feature transform needs method-specific evidence.

| Field | Type | Meaning |
|---|---|---|
| `percentile` | number | Percentile used by a `percentile` feature. |
| `quantile` | number | Quantile used by a `quantile` feature. |
| `bins` | integer | Histogram bin count. |
| `bin_index` | integer | Zero-based histogram bin index. |
| `bin_min` | number | Lower edge for the histogram bin. |
| `bin_max` | number | Upper edge for the histogram bin. |
| `other_channel` | string | Comparison channel for covariance/correlation features. |
| `lag_samples` | integer | Lag used by autocorrelation or cross-correlation. The convention is `channel[t]` compared with `other_channel[t + lag_samples]`; autocorrelation uses the same channel on both sides. |
| `frequency_hz` | number | Frequency represented by a spectral record. |
| `bin_frequency_hz` | number | Frequency-bin center in Hz. |
| `bin_width_hz` | number | Frequency-bin width in Hz. |
| `window` | string | Window function used by a spectral or time-frequency feature. |
| `window_index` | integer | Zero-based window coefficient index for `window_function`. |
| `window_samples` | integer | Segment/window length in samples. |
| `overlap_samples` | integer | Segment overlap in samples for Welch/STFT/spectrogram records. |
| `sample_index` | integer | Reconstructed sample or window coefficient index. |
| `segment_index` | integer | Zero-based time-frequency segment index. |
| `segment_start_s` | number | Segment start time in seconds. |
| `segment_end_s` | number | Segment end time in seconds. |
| `real` | number | Real component for complex spectral evidence. |
| `imaginary` | number | Imaginary component for complex spectral evidence. |
| `magnitude` | number | Magnitude or amplitude associated with the record. |
| `phase_rad` | number | Phase in radians for complex spectral evidence. |
| `harmonic_index` | integer | Harmonic number or harmonic-count context. |
| `fundamental_hz` | number | Fundamental frequency used by harmonic metrics. |
| `band_low_hz` | number | Lower edge for a band-power record. |
| `band_high_hz` | number | Upper edge for a band-power record. |
| `rolloff_percent` | number | Cumulative power percentage used by `spectral_rolloff`. |
| `normalization` | string | Scaling convention, such as `one_sided_peak_amplitude`, `one_sided_power_spectral_density`, or `welch_averaged_power_spectral_density`. |

## Result Fields

| Field | Type | Meaning |
|---|---|---|
| `criterion_id` | string | Stable criterion identifier from config or CLI. |
| `outcome` | `pass` or `fail` | Per-criterion result. |
| `failed_criterion` | string or null | Criterion ID when failed, otherwise null. |
| `measurement_id` | string | Stable ID of the measurement record used as evidence. |
| `channel` | string | Channel evaluated. |
| `measured_value` | number | Observed value used for the decision. |
| `required_value` | number | Required value from config. |
| `tolerance_used` | number | Voltage or time tolerance used by that criterion. State-transition count uses `0.0`. |
| `unit` | string | Unit for measured and required values. |
| `sample_index` | integer | Evidence sample index. |
| `timestamp` | number | Evidence timestamp in seconds. |
| `reason` | string | Human-readable decision reason. |

## Event Record Fields

`event_records` is emitted only when the configured analysis produces event evidence.

| Field | Type | Meaning |
|---|---|---|
| `id` | string | Stable report-local event ID. |
| `transform` | string | Transform that emitted the event, such as `schmitt_trigger`, `edge_extraction`, `debounce`, `glitch_removal`, or `bounce_detection`. |
| `kind` | string | Event type: `state_transition`, `edge`, `rejected_pulse`, or `bounce`. |
| `channel` | string | Source channel. |
| `sample_index` | integer | Evidence sample index. |
| `timestamp` | number | Evidence timestamp in seconds. |
| `state` | string | Event state, such as `high` or `low`. |
| `previous_state` | string or omitted | Previous state when applicable. |
| `direction` | string or omitted | `rising` or `falling` when applicable. |
| `on_threshold_v` / `off_threshold_v` | number or omitted | Schmitt thresholds used to produce the state trace. |
| `duration_s` | number or omitted | Duration for rejected-pulse or bounce evidence. |
| `count` | integer or omitted | Count for aggregate event evidence such as bounce. |
| `source_event_ids` | array | Event IDs used as source evidence. |
| `transform_metadata` | object | Structured metadata for the transform that emitted this event. |

## Event Validation Fields

`event_validations` is emitted only when configured event validations run. A failed event validation contributes to top-level `overall_outcome = fail`.

| Field | Type | Meaning |
|---|---|---|
| `requirement_id` | string | Configured validation ID. |
| `validation` | string | Validation type: `missing_pulse`, `extra_pulse`, `dwell_time`, or `timeout`. |
| `outcome` | `pass` or `fail` | Validation decision. |
| `channel` | string | Channel evaluated. |
| `measured_value` | number | Observed event count or duration. |
| `required_value` | number | Required count or duration. |
| `unit` | string | `events` or `s`. |
| `linked_event_ids` | array | Event records used as direct evidence when applicable. |
| `reason` | string | Human-readable decision reason. |
| `transform_metadata` | object | Structured metadata for the validation transform. |

## Stability

Golden tests in `tests/golden/` compare JSON output exactly. Any intentional schema change should update this document, the golden files, and release notes together.

## Batch Summary Note

`ferrisoxide-signal batch` does not change the per-run analysis report schema. Each completed batch run writes the same text or JSON report produced by `analyze`. The batch summary is a separate artifact with `kind = "batch_analysis"` and stable run-count/status fields documented in `docs/artifact-contract.md`.

## Criteria DSL Evidence Note

Measurement-backed DSL criteria use the same report schema as legacy criteria. A DSL config changes how the criterion is written in TOML, not how measurement evidence appears in JSON.

For equivalent configs:

- `criterion_id` remains the configured `id`.
- `measurement_id` remains `{criterion_id}_measurement`.
- `measurements[]` records the measured evidence with method and context.
- `results[]` records the pass/fail decision with measured value, required value, tolerance, sample index, timestamp, channel, and reason.

The current parity tests compare representative DSL reports against legacy reports and existing golden JSON exactly. For `state_transition_count`, the DSL requirement unit is `count`, while the report evidence unit remains `transitions` for existing report compatibility.

## M12 Event Validation Note

M12 adds `event_records` and `event_validations` as additive top-level fields. Existing reports without event config omit both fields. Event validation failures affect `overall_outcome`, while ordinary `results[]` remains the existing criteria-evidence array.

## M6-003 Migration Note

M6-003 adds the top-level `measurements` array and the per-result `measurement_id` field. Existing result fields remain present so older consumers can keep reading criterion-level pass/fail evidence while newer consumers can de-duplicate measurement evidence and render richer report/SVG annotations.
