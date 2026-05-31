# Test Plan

## Scope

Initial tests cover the Rust core model, waveform metadata, parser interface, TOML config model, filters, criteria evaluation, text/JSON reporting, and CLI smoke behavior.

## Test Types

| Type | Purpose | Evidence |
|---|---|---|
| Unit tests | Validate model invariants, parser functions, config conversion, filters, criteria, reports, and CLI argument helpers. | Cargo test output. |
| Fixture tests | Validate example CSV parsing. | `tests/fixtures/basic_waveform.csv`. |
| Synthetic signal tests | Validate filters with known shapes. | Inline generated fixtures in `filter.rs`. |
| CLI smoke tests | Validate basic executable behavior. | CLI unit tests and `cargo run --bin ferrisoxide-signal -- analyze ...` for explicit flags, config text, and config JSON. |

## Edge Cases

- Empty CSV.
- Missing time column.
- Missing channel column.
- Non-numeric values.
- Mismatched time/channel lengths.
- NaN or infinite values.
- Single-sample waveform.
- Multiple channels with different units.

## Validation Commands

```bash
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo run --bin ferrisoxide-signal -- analyze --input examples/basic-waveform.csv --time-column time --channels input_v --moving-average 2 --min input_v:0.0 --max input_v:5.5
cargo run --bin ferrisoxide-signal -- analyze --input examples/basic-waveform.csv --config examples/basic-config.toml --format text
cargo run --bin ferrisoxide-signal -- analyze --input examples/basic-waveform.csv --config examples/basic-config.toml --format json
```
