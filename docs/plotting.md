# SVG Plotting

Date: 2026-05-31

## Scope

`ferrisoxide-signal plot` renders local CSV waveform data to an SVG file. Plotting is a desktop CLI feature only. It does not add GUI windows, DAQ integration, embedded or RTOS plotting, interactive controls, hardware validation, or certification evidence.

Plotting code lives in `crates/ferrisoxide-plot`, so `ferrisoxide-core` and `ferrisoxide-signal` stay free of plotting dependencies.

## 2D Waveform Plot

Use 2D plotting when the CSV has time plus one or more signal channels:

```bash
cargo run --quiet --bin ferrisoxide-signal -- plot \
  --input examples/basic-waveform.csv \
  --time-column time \
  --channels input_v,output_v \
  --output basic-waveform.svg
```

The output uses:

- X axis: configured time column.
- Y axis: one or more configured signal channels.
- Output format: SVG.

## 2D Evidence Overlays

Use `--config` with `ferrisoxide-signal plot` to render measurement-backed evidence on a 2D SVG plot:

```bash
cargo run --quiet --bin ferrisoxide-signal -- plot \
  --input tests/fixtures/dropout_event.csv \
  --config tests/configs/transient-event-dropout-fail.toml \
  --output dropout-evidence.svg \
  --title "Dropout Evidence"
```

Evidence overlays currently render:

- Overall evidence status, `PASS` or `FAIL`.
- Voltage threshold lines when criteria or measurement context include thresholds.
- Failed-criterion markers at the evidence sample.
- Labels containing criterion ID, sample index, timestamp, channel, measured value, and required value.

The overlay path reuses `ferrisoxide-core` criteria evaluation and report measurement IDs. It does not recalculate separate plotting-only measurements.

## 3D Waveform Plot With Third Axis

Use `--z-column` when a CSV includes an auxiliary axis such as temperature, pressure, sweep position, or another synchronized measurement:

```bash
cargo run --quiet --bin ferrisoxide-signal -- plot \
  --input tests/fixtures/plot_three_axis.csv \
  --time-column time_s \
  --channels signal_v \
  --z-column temperature_c \
  --output three-axis.svg \
  --title "Three Axis Validation Plot"
```

The output uses:

- X axis: configured time column.
- Y axis: configured signal channel.
- Z axis: configured third-axis column.

Multiple signal channels may be plotted against the same third-axis column. The third-axis column must be separate from plotted signal channels.

## Dependency Boundary

The plotting slice uses Plotters with only the SVG backend and line-series features enabled:

```toml
plotters = { version = "0.3.7", default-features = false, features = ["svg_backend", "line_series"] }
```

This deliberately excludes bitmap encoders, GUI backends, GIF animation, and interactive plot surfaces.

## Limitations

- SVG output only.
- Local file input and output only.
- No automatic directory creation for output paths.
- No resampling or interpolation.
- Evidence overlays are currently 2D only.
- No camera controls, hover labels, or interactive UI.
- No claims about hardware behavior, environmental qualification, or certification use.
