# MVP Usage Sketch

This document describes the current MVP usage. The command shape can still change before a public release.

```bash
cargo run --quiet --bin ferrisoxide-signal -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/basic-config.toml \
  --format text
```

Current output:

```text
Waveform Analysis Report
Input: examples/basic-waveform.csv
Samples: 5 Channels: 2 Lineage: Derived
Sample Interval: nominal=0.001000000 s min=0.001000000 max=0.001000000 uniform=true
Nominal Sample Rate: 1000.000000 Hz
Transforms: moving_average(window_samples=2)
Validation Profile: engineering_validation
Evidence Source: local_file_analysis
Tolerance Policy: voltage=0.000000 V time=0.000000000 s
Confidence Notes: software validation evidence only; not hardware qualification or certification evidence
Overall: Pass
Measurements:
- input_min_voltage_measurement: method=minimum_sample channel=input_v measured=0.000000 V sample_index=0 timestamp=0.000000
- input_max_voltage_measurement: method=maximum_sample channel=input_v measured=5.000000 V sample_index=4 timestamp=0.004000
Criteria:
- input_min_voltage: Pass measurement_id=input_min_voltage_measurement channel=input_v measured=0.000000 V required=0.000000 V tolerance=0.000000 sample_index=0 timestamp=0.000000 reason=minimum observed voltage was 0.000000 V
- input_max_voltage: Pass measurement_id=input_max_voltage_measurement channel=input_v measured=5.000000 V required=5.500000 V tolerance=0.000000 sample_index=4 timestamp=0.004000 reason=maximum observed voltage was 5.000000 V
```

Supported MVP CLI filter flags:

- `--moving-average <samples>` applies a trailing moving average to each channel.
- `--low-pass <hz>` applies a simple first-order low-pass filter to each channel.
- `--adc-quantize <bits:min_v:max_v>` simulates ideal ADC quantization before criteria.

Additional M11 desktop transforms are available through config files; see `examples/m11-transform-config.toml`.

JSON output:

```bash
cargo run --quiet --bin ferrisoxide-signal -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/basic-config.toml \
  --format json
```

The JSON report includes `waveform_metadata`, `evidence_context`, `overall_outcome`, reusable `measurements`, and per-criterion `results`. Each criterion result includes `measurement_id` so report consumers can link pass/fail decisions back to the measured evidence record.

Measurement-backed DSL configs are also supported:

```bash
cargo run --quiet --bin ferrisoxide-signal -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/basic-dsl-config.toml \
  --format text
```

Expected excerpt:

```text
Overall: Pass
Measurements:
- input_min_voltage_measurement: method=minimum_sample channel=input_v measured=0.000000 V sample_index=0 timestamp=0.000000
- input_max_voltage_measurement: method=maximum_sample channel=input_v measured=5.000000 V sample_index=4 timestamp=0.004000
Criteria:
- input_min_voltage: Pass measurement_id=input_min_voltage_measurement channel=input_v measured=0.000000 V required=0.000000 V tolerance=0.000000 sample_index=0 timestamp=0.000000 reason=minimum observed voltage was 0.000000 V
- input_max_voltage: Pass measurement_id=input_max_voltage_measurement channel=input_v measured=5.000000 V required=5.500000 V tolerance=0.000000 sample_index=4 timestamp=0.004000 reason=maximum observed voltage was 5.000000 V
```

See `docs/criteria-dsl-migration.md` for when to use DSL versus legacy explicit fields, explicit unit rules, compatibility expectations, and non-goals.

Explicit CLI criteria remain available for one-off checks:

```bash
cargo run --quiet --bin ferrisoxide-signal -- analyze \
  --input examples/basic-waveform.csv \
  --time-column time \
  --channels input_v \
  --moving-average 2 \
  --adc-quantize 8:0.0:5.0 \
  --min input_v:0.0 \
  --max input_v:5.5
```

ADC quantization is also available in TOML config:

```bash
cargo run --quiet --bin ferrisoxide-signal -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/adc-quantized-config.toml \
  --format text
```

Expected output:

```text
Waveform Analysis Report
Input: examples/basic-waveform.csv
Samples: 5 Channels: 1 Lineage: Derived
Sample Interval: nominal=0.001000000 s min=0.001000000 max=0.001000000 uniform=true
Nominal Sample Rate: 1000.000000 Hz
Transforms: adc_quantize(bits=8,min_v=0,max_v=5)
Validation Profile: engineering_validation
Evidence Source: local_file_analysis
Tolerance Policy: voltage=0.000000 V time=0.000000000 s
Confidence Notes: software validation evidence only; not hardware qualification or certification evidence
Overall: Pass
Measurements:
- input_max_after_adc_measurement: method=maximum_sample channel=input_v measured=5.000000 V sample_index=3 timestamp=0.003000
Criteria:
- input_max_after_adc: Pass measurement_id=input_max_after_adc_measurement channel=input_v measured=5.000000 V required=5.000000 V tolerance=0.000000 sample_index=3 timestamp=0.003000 reason=maximum observed voltage was 5.000000 V
```

Richer CSV dialect support and stable numerical filter guarantees remain planned. See `docs/filter-behavior.md`, `docs/measurements.md`, and `docs/time-axis-and-tolerances.md` for current transform, measurement, and tolerance semantics.

## SVG Plotting

Render a 2D SVG waveform plot:

```bash
cargo run --quiet --bin ferrisoxide-signal -- plot \
  --input examples/basic-waveform.csv \
  --time-column time \
  --channels input_v,output_v \
  --output basic-waveform.svg
```

Render a 3D SVG line plot with an auxiliary third-axis column:

```bash
cargo run --quiet --bin ferrisoxide-signal -- plot \
  --input tests/fixtures/plot_three_axis.csv \
  --time-column time_s \
  --channels signal_v \
  --z-column temperature_c \
  --output three-axis.svg
```

Plotting is desktop SVG output only. It does not add GUI, DAQ, embedded plotting, or certification scope.

Render a 2D SVG plot with criteria evidence overlays:

```bash
cargo run --quiet --bin ferrisoxide-signal -- plot \
  --input tests/fixtures/dropout_event.csv \
  --config tests/configs/transient-event-dropout-fail.toml \
  --output dropout-evidence.svg
```

Evidence overlays show pass/fail status, threshold lines, and failed-criterion markers using the same measurement evidence as JSON reports. Overlays are currently 2D only.
