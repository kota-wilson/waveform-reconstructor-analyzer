# ADR-003: Filter Pipeline Architecture

Date: 2026-05-31

## Status

Accepted.

## Numbering Note

This decision was requested as the next filter-pipeline ADR. `ADR-002` already records the accepted MIT license decision, so this decision uses `ADR-003` to preserve the existing decision history.

## Context

Waveform Reconstructor and Analyzer needs to apply one or more filters to parsed waveform data before criteria evaluation. The MVP already has concrete moving-average and first-order low-pass filters, plus TOML configuration for filter definitions.

The next milestone needs a stable filter-chain abstraction without drifting into GUI, DAQ integration, plugin runtimes, or complex aerospace validation logic.

## Decision

Use a config-driven enum model for the MVP filter pipeline.

The pipeline should be represented as ordered config steps that deserialize from TOML and map to explicit Rust enum variants, such as:

```rust
pub enum FilterStep {
    MovingAverage { window_samples: usize },
    LowPass { cutoff_hz: f64 },
    AdcQuantize { bits: u8, min_v: f64, max_v: f64 },
}
```

Pipeline execution should:

- preserve the raw input waveform,
- apply steps in config order,
- return a derived waveform after each step,
- validate parameters before applying each filter,
- return structured errors for unsupported or invalid filter definitions,
- keep the public behavior deterministic and easy to document.

Trait-based extension remains allowed behind the implementation boundary, but the user-facing configuration format should stay enum-backed for M1.

## Options Considered

| Option | Tradeoff | Decision |
|---|---|---|
| Filter trait objects | Flexible runtime dispatch, but harder to serialize and document as a stable MVP config. | Defer as internal extension mechanism. |
| Generic filter structs | Strong compile-time types, but less useful for config-driven CLI workflows. | Use for concrete filter implementations only. |
| Enum-based filter definitions | Clear, serializable, testable, and easy to reject when unsupported. | Accept. |
| Config-driven pipeline steps | Matches TOML config and CLI analysis workflow. | Accept as the public MVP model. |

## Consequences

- Config parsing becomes the source of truth for user-facing filter selection.
- Unsupported filters fail early with clear errors.
- Tests can assert filter ordering through a small fixture.
- Future transform additions require adding an enum variant, docs, tests, and traceability updates.
- Trait-based extension can still be introduced later without exposing a plugin system.

## Implementation Notes

- Put the enum and chain execution in `crates/wra-core/src/filter.rs` or a small adjacent module if the file grows too large.
- Keep TOML deserialization in `crates/wra-core/src/config.rs`.
- Convert config filter definitions into pipeline steps before analysis.
- Add tests for ordering, invalid parameters, and raw waveform preservation.
- Treat ADC quantization as an ordered derived transform, not DAQ integration or hardware validation.
- Do not add GUI, DAQ integration, certification claims, or plugin runtime behavior.

## Related Work

- GitHub issue: `M1-003 Add filter-chain abstraction`
- Architecture: `docs/architecture.md`
- Risk: `risk-register.md` filter interpretation risk

## Hand-Off Note

Role: Software Architect
Goal: Decide the filter pipeline model for the validated MVP milestone.
Files changed: `decisions/ADR-003-filter-pipeline-architecture.md`
Checks run: Architecture review only; implementation follows in M1-003.
Status: Accepted.
Known gaps: Frequency-response validation and additional filters are out of scope for this decision.
Next recommended step: Implement M1-003 with config-driven enum pipeline steps and tests.
