# MVP Usage Sketch

This document describes the current MVP usage. The command shape can still change before a public release.

```bash
cargo run --bin wra -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/basic-config.toml \
  --format text
```

Current output:

```text
Waveform Analysis Report
Input: examples/basic-waveform.csv
Overall: Pass
Criteria:
- input_min_voltage: Pass channel=input_v measured=0.000000 V required=0.000000 V sample_index=0 timestamp=0.000000 reason=minimum observed voltage was 0.000000 V
- input_max_voltage: Pass channel=input_v measured=5.000000 V required=5.500000 V sample_index=4 timestamp=0.004000 reason=maximum observed voltage was 5.000000 V
```

Supported MVP filters:

- `--moving-average <samples>` applies a trailing moving average to each channel.
- `--low-pass <hz>` applies a simple first-order low-pass filter to each channel.
- `--adc-quantize <bits:min_v:max_v>` simulates ideal ADC quantization before criteria.

JSON output:

```bash
cargo run --bin wra -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/basic-config.toml \
  --format json
```

Explicit CLI criteria remain available for one-off checks:

```bash
cargo run --bin wra -- analyze \
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
cargo run --bin wra -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/adc-quantized-config.toml \
  --format text
```

Expected output:

```text
Waveform Analysis Report
Input: examples/basic-waveform.csv
Overall: Pass
Criteria:
- input_max_after_adc: Pass channel=input_v measured=5.000000 V required=5.000000 V sample_index=3 timestamp=0.003000 reason=maximum observed voltage was 5.000000 V
```

Richer CSV dialect support and stable numerical filter guarantees remain planned.
