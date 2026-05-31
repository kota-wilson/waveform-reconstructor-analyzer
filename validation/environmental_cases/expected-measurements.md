# Environmental Case Expected Measurements

Date: 2026-05-31

## Scope

These examples are environmental-style software validation aids for reviewers. They show how dropout and contact-bounce cases are represented and interpreted. They are not chamber-integration evidence, DAQ evidence, hardware qualification, tool qualification, or certification evidence.

## Dropout Case

Fixture:

- `validation/environmental_cases/dropout_event.csv`
- `validation/environmental_cases/dropout_event.toml`

Expected behavior:

| Criterion | Expected Measurement | Required Value | Expected Outcome | Evidence Point |
|---|---:|---:|---|---|
| `supply_dropout_max_1ms` | `0.002 s` | `0.001 s maximum` | Fail | Low event starts at sample index 3, timestamp `0.003 s`. |

What it proves:

- The analyzer can identify an unintended low dropout on a signal expected to remain high.
- The report carries failed criterion, measured duration, required duration, sample index, timestamp, and channel.

What it does not prove:

- It does not prove actual supply behavior in environmental test hardware.
- It does not validate DAQ sampling accuracy or chamber conditions.

Expected report artifact:

- `validation/reports/environmental_dropout_fail.json`

## Contact-Bounce Case

Fixture:

- `validation/environmental_cases/contact_bounce.csv`
- `validation/environmental_cases/contact_bounce.toml`

Expected behavior:

| Criterion | Expected Measurement | Required Value | Expected Outcome | Evidence Point |
|---|---:|---:|---|---|
| `switch_contact_bounce_max_0p5ms` | `0.001 s` | `0.0005 s maximum` | Fail | The longest tied low bounce event is reported at sample index 4, timestamp `0.004 s`. |

What it proves:

- The analyzer can frame contact bounce as a subtype of transient event detection.
- The expected report uses engineering language such as contact bounce and transient event.

What it does not prove:

- It does not prove a physical switch passed or failed an environmental qualification run.
- It does not replace a test procedure, equipment calibration, or engineering signoff.

Expected report artifact:

- `validation/reports/environmental_contact_bounce_fail.json`
