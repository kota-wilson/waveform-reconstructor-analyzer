# Known-Answer Validation Fixtures

Add fixtures here only when the expected measurements are known before running the analyzer.

Each case should record:

- Source or generation method.
- Expected criterion measurements.
- Voltage and time tolerances.
- Command used to run the analyzer.
- Expected text or JSON report.
- Review notes and scope limits.

Current cases:

| Case | Fixture | Config | Expected Report | Expected Values |
|---|---|---|---|---|
| Tolerance-boundary square wave | `square_wave_tolerance.csv` | `square_wave_tolerance.toml` | `../reports/square_wave_tolerance.json` | `expected-measurements.md` |
