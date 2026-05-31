# Benchmark Fixtures

This folder records benchmark strategy and baseline results for large CSV validation.

Large generated CSV files are not committed. The repeatable fixture strategy is:

```bash
scripts/benchmark-large-csv.sh 100000 3
```

The script writes generated CSV/config files under `target/wra-benchmark/`, then runs the dependency-free `wra-bench` helper to report average read, parse, transform, criteria, report, and total timings.

Benchmark output is engineering evidence only. It does not claim DAQ throughput, real-time behavior, production performance guarantees, or embedded performance.
