# MVP Implementation Plan

## MVP Goal

Deliver a CLI-driven Rust MVP that can parse CSV waveform fixtures into typed waveform data, apply derived transform interfaces, evaluate configurable waveform criteria, and produce text/JSON reports.

Current status: The dependency-reviewed MVP slice is implemented, public, and validated on protected `main`. Follow-up work has added v0.2.0 criteria, real fixtures, golden JSON reports, `wra-signal`, domain review, and ADC quantization. Remaining M1 backlog focuses on richer metadata and README expected-output polish.

## Milestone M1: Foundation

Owner: Core Software Engineer

Files:

- `crates/wra-core/src/model.rs`
- `crates/wra-core/src/csv.rs`
- `crates/wra-core/src/error.rs`
- `crates/wra-cli/src/main.rs`
- `tests/fixtures/basic_waveform.csv`

Acceptance criteria:

- `Waveform` validates sample lengths.
- CSV parser interface exists with a simple MVP parser.
- CLI accepts explicit local analysis arguments.
- Unit tests exist for model and parser basics.

## Milestone M2: Derived Transforms

Owner: Systems Engineer

Files:

- `crates/wra-core/src/filter.rs`
- `crates/wra-core/src/filter.rs` unit tests

Acceptance criteria:

- Filter trait and enum-backed filter steps exist.
- Moving average filter has basic implementation.
- Low-pass filter has a simple first-order implementation and must not claim validated frequency response yet.
- ADC quantization has an ideal endpoint-inclusive implementation and must not claim hardware ADC fidelity.
- Tests use synthetic fixtures with tolerances.

## Milestone M3: Criteria And Reports

Owner: Core Software Engineer / Documentation Engineer

Files:

- `crates/wra-core/src/criteria.rs`
- `crates/wra-core/src/analysis.rs`
- `crates/wra-core/src/report.rs`
- `examples/basic-config.toml`

Acceptance criteria:

- Min/max voltage, state transition, pulse width, transient event, stable-state duration, and rise/fall criteria can be represented.
- Analysis result reports pass/fail, measured value, required value, sample index, timestamp, channel, and reason.
- Report model can render text output.
- CLI accepts explicit min/max criteria flags and TOML config files.
- Report model can render JSON output.

## Milestone M4: Open-Source Readiness

Owner: GitHub Maintainer Specialist / Release Engineer

Acceptance criteria:

- CI passes.
- README has usage example.
- Contribution guide and security docs are current.
- License confirmed.
- Changelog has MVP entry.

## Validation Commands

```bash
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets
```

## Approval Stops

- Stop before adding dependencies.
- Stop before full GUI planning.
- Stop before tagged release publication.
