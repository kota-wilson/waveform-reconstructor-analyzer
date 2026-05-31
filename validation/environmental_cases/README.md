# Environmental Case Validation Fixtures

Add environmental-style validation examples here after the expected behavior is defined independently of the analyzer.

Examples may include dropout, contact bounce, slow edge, noisy threshold crossing, and multi-channel correlation cases. These examples are engineering validation aids only; they are not hardware qualification or certification evidence.

Current cases:

| Case | Fixture | Config | Expected Report | Expected Values |
|---|---|---|---|---|
| Dropout event | `dropout_event.csv` | `dropout_event.toml` | `../reports/environmental_dropout_fail.json` | `expected-measurements.md` |
| Contact bounce event | `contact_bounce.csv` | `contact_bounce.toml` | `../reports/environmental_contact_bounce_fail.json` | `expected-measurements.md` |
