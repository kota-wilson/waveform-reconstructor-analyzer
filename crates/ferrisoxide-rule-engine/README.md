# ferrisoxide-rule-engine

`ferrisoxide-rule-engine` owns the shared rule execution semantics for desktop and embedded-compatible callers.

The crate is `#![no_std]`. Its full evidence API uses `alloc` for desktop-style owned result records, while its borrowed summary API evaluates caller-provided rule data without owned criterion/result strings and without heap allocation on basic pass/fail paths. It deliberately avoids CSV parsing, TOML parsing, report rendering, plotting, file I/O, DAQ/controller I/O, hardware HALs, RTOS SDKs, and certification claims.

## Current Scope

- Minimum and maximum sample criteria.
- State transition counts.
- Pulse width.
- Stable-state duration.
- Transient event duration.
- Rise/fall time.
- Measurement-backed criteria with explicit operators.
- Result and measurement evidence records compatible with existing report paths.
- Borrowed summary results and borrowed/static error values for constrained embedded-compatible evaluation.

Future issues own desktop-vs-embedded exact parity fixtures and runtime package loaders.

## Hand-Off Note

Role: Core Software Engineer / Verification and Validation Engineer
Goal: Introduce shared rule execution semantics and the M8-007 no_std boundary.
Files changed: `crates/ferrisoxide-rule-engine/`
Checks run: `cargo test -p ferrisoxide-rule-engine`; `cargo check -p ferrisoxide-rule-engine --target aarch64-unknown-none`.
Status: no_std boundary implemented locally for M8-007; full workspace validation, protected PR, CI, merge, and issue closure pending.
Known gaps: exact desktop-vs-embedded parity fixtures remain M8-008; runtime package loaders remain future work.
Next recommended step: Complete M8-007 PR review, then add M8-008 parity fixtures.
