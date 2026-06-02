# Runtime Loader Design Gate

Date: 2026-06-01

Status: M24 design gate only. This document proposes the bounded package-consumption boundary for a future Raspberry Pi 5 bare-metal runtime. It does not implement a loader, binary package format, HAL, RTOS SDK adapter, target-board execution, cryptographic signing, hardware qualification, or certification evidence.

Related requirements: WRA-RQ-106, WRA-RQ-107, WRA-RQ-108, WRA-RQ-109.

## Gate Decision

| Gate | Decision | Reason | Next Owner |
|---|---|---|---|
| Loader design scope | Pass | The accepted subset, memory model, failure modes, checksum role, and target checks are defined without implementing runtime loading. | Embedded RTOS Engineer |
| Loader implementation | Blocked pending approval | Implementation would create new runtime behavior and needs a fresh issue, V&V plan, and target/profile gate. | Project Coordinator |
| Hardware/runtime claim | Not Applicable | No target execution, HAL, RTOS SDK, live DAQ, real-time guarantee, hardware qualification, or certification evidence is added. | Documentation Engineer |

## Accepted Package Subset

A future loader may only accept packages that validate against the current rule schema and contain this transform subset:

- `moving_average`
- `low_pass`
- `adc_quantize`
- `offset`
- `gain`
- `invert`

The first implementation should start with `offset`, `gain`, and `invert` because M22 already provides shared borrowed-slice semantics for that subset. `moving_average`, `low_pass`, and `adc_quantize` remain package-schema-supported but need separate bounded-buffer runtime implementation evidence before a loader executes them.

The loader must reject:

- unknown filter variants,
- known but unsupported runtime variants such as `clamp`, `deadband`, `dc_remove`, `baseline_subtract`, `high_pass_baseline`, and `moving_median`,
- packages whose `target.kind` does not match the compiled runtime profile,
- packages with missing channels, duplicate IDs, invalid units, invalid timing assumptions, invalid thresholds, or invalid criteria.

## Memory And Execution Constraints

Future loader implementation should use:

- caller-owned package input buffer,
- caller-owned sample input/output buffers,
- fixed transform and criterion capacity limits defined at compile time,
- no filesystem assumptions,
- no CSV/TOML/JSON parsing inside hard real-time evaluation loops,
- no SVG/report rendering,
- no desktop CLI code,
- no DAQ, HAL, RTOS SDK, unsafe FFI, or global setup requirement.

The parser/loader boundary should perform validation before evaluation. Evaluation should operate over already-decoded bounded structs or borrowed slices.

## Failure Modes

The loader contract should expose stable errors for:

- schema version mismatch,
- target profile mismatch,
- checksum mismatch,
- unsupported filter,
- unsupported criterion,
- duplicate filter or criterion ID,
- missing channel,
- invalid unit,
- invalid timing assumption,
- input/output buffer capacity mismatch,
- non-finite transform input, parameter, or output,
- package capacity exceeded.

Errors should be deterministic and should not require allocation in the runtime evaluation path where practical.

## Checksum Role

Existing checksums are dependency-free drift-detection evidence only. They are not signing, authentication, tamper proofing, secure boot, safety certification, or hardware qualification evidence.

A future loader may reject checksum mismatches, but any security claim requires a separate security design and approval gate.

## Target Checks

Before loader implementation is considered ready for review, validation should include:

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo check -p ferrisoxide-signal --target aarch64-unknown-none
cargo check -p ferrisoxide-embedded --target aarch64-unknown-none
```

If the future loader adds a crate, that crate must have explicit no_std target checks and dependency review. Target hardware execution remains a separate gate.

## Non-Goals

M24 does not add:

- runtime loader code,
- binary package serialization,
- target-board execution,
- live DAQ or controller I/O,
- HAL or RTOS SDK adapters,
- signing, authentication, or secure boot,
- hardware qualification,
- flight certification, regulatory compliance, safety certification, or airworthiness evidence.

## Hand-Off Note

Role: Embedded RTOS Engineer / Software Architect
Goal: Define the runtime-loader boundary before any implementation starts.
Files changed: `docs/runtime-loader-design-gate.md`
Checks run: See `docs/validation-log.md`.
Status: M24 design gate complete locally.
Known gaps: No loader implementation, binary package format, target execution, or hardware evidence exists.
Next recommended step: Create a fresh implementation proposal only after this design gate is reviewed and explicitly approved.
