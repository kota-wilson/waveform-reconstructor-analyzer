# ferrisoxide-rule-engine

`ferrisoxide-rule-engine` owns the shared rule execution semantics for desktop and embedded-compatible callers.

The crate evaluates measurement-backed rule criteria over caller-provided time/sample slices. It deliberately avoids CSV parsing, TOML parsing, report rendering, plotting, file I/O, DAQ/controller I/O, hardware HALs, RTOS SDKs, and certification claims.

## Current Scope

- Minimum and maximum sample criteria.
- State transition counts.
- Pulse width.
- Stable-state duration.
- Transient event duration.
- Rise/fall time.
- Measurement-backed criteria with explicit operators.
- Result and measurement evidence records compatible with existing report paths.

Future issues own the no_std compatibility boundary and desktop-vs-embedded exact parity fixtures.

## Hand-Off Note

Role: Core Software Engineer / Verification and Validation Engineer
Goal: Introduce shared rule execution semantics for M8-006.
Files changed: `crates/ferrisoxide-rule-engine/`
Checks run: `cargo tree -p ferrisoxide-rule-engine`; `cargo test -p ferrisoxide-rule-engine`; `cargo test -p ferrisoxide-core`; `cargo test -p ferrisoxide-embedded`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Shared engine implemented locally for M8-006; protected PR, CI, merge, and issue closure pending.
Known gaps: no_std compatibility and exact desktop-vs-embedded parity fixtures remain future M8 issues.
Next recommended step: Complete M8-006 PR review, then add the no_std compatibility boundary.
