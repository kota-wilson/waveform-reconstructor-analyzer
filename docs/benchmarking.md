# Benchmarking

Date: 2026-05-31

## Scope

Benchmarks are repeatable engineering measurements for large CSV behavior. They are not production performance guarantees, real-time DAQ throughput claims, embedded performance claims, or release qualification evidence.

## Repeatable Command

The benchmark script generates a square-wave CSV and matching config under `target/wra-benchmark/`, then runs `wra-bench`:

```bash
sh scripts/benchmark-large-csv.sh 100000 3
```

`wra-bench` reports average timings for:

- CSV file read.
- CSV parse and waveform reconstruction.
- Ordered transform chain.
- Criteria evaluation.
- JSON report rendering.
- Total local analysis path.

## Baseline Snapshot

Environment:

- Date: 2026-05-31.
- Working directory: `/Users/kota/Desktop/softwareai/projects/waveform-reconstructor-analyzer`.
- Tooling: project-local Cargo workspace; no new benchmark dependencies.
- Fixture strategy: generated `100000` sample, one-channel square wave with 1 kHz timestamps and a moving-average transform.

Command:

```bash
sh scripts/benchmark-large-csv.sh 100000 3
```

Observed output:

```text
wra_benchmark
input=target/wra-benchmark/large_square_wave_100000.csv
config=target/wra-benchmark/large_square_wave_100000.toml
iterations=3
samples=100000
channels=1
report_bytes=2395
read_ms_avg=0.316
parse_ms_avg=157.890
transform_ms_avg=5.725
criteria_ms_avg=5.084
report_ms_avg=0.070
total_ms_avg=169.084
```

## Interpretation

This baseline shows that parse/reconstruction dominates the current 100k-sample local CSV path. The data is sufficient to compare future parser, transform, criteria, and report changes on the same machine. It is not sufficient to claim worst-case performance, DAQ ingestion capability, memory ceilings, or cross-platform behavior.
