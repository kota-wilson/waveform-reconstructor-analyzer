# M7-001 Criteria DSL Config Schema Pipeline Report

Date: 2026-05-31

Branch: `feature/m7-001-dsl-config-schema`

Milestone: #7, `v0.5.0: Measurement-Backed Criteria DSL`

Issue: #55, `M7-001 Add criteria DSL config schema and compatibility adapter`

Status: Merged.

Merge commit: `9a8b0e667f9d829a1083168b7875db967ca4e960`

## Scope

This slice adds the configuration-layer schema and compatibility boundary for measurement-backed criteria DSL entries.

In scope:

- Deserialize legacy criteria with `type = "..."`
- Deserialize DSL criteria with `[criteria.measurement]` and `[criteria.requirement]`
- Allow legacy and DSL criteria entries side by side in one config
- Reject ambiguous criteria that mix legacy and DSL shapes
- Preserve legacy criteria conversion and runtime behavior
- Return a clear not-implemented error if a DSL criterion is converted for runtime evaluation before M7-003

Out of scope:

- DSL operator semantics
- Unit validation beyond deserializing explicit unit/value fields
- Runtime DSL evaluation
- DSL/legacy golden parity suite
- Full DSL documentation refresh
- Unit shorthand parser
- New measurements
- GUI, DAQ, plugin runtime, batch analysis, RTOS expansion, hardware qualification, or certification claims

## Research

- Owner role: Software Architect / Core Software Engineer
- Artifact: Issue #55, `docs/v0.5.0-criteria-dsl-milestone-proposal.md`, `docs/v0.5.0-issue-planning-report.md`, and `crates/wra-core/src/config.rs`.
- Evidence: Existing config conversion already maps legacy TOML entries into `Criterion`; M7 needs a config-boundary adapter before evaluation behavior changes.
- Gate: Intake Gate.
- Decision: Pass.
- Residual risk: Later issues must avoid broadening this schema into a general expression language.
- Next owner: Software Architect.

## Requirements

- Owner role: Software Architect / V&V Engineer
- Artifact: WRA-RQ-036 in `requirements.md`; WRA-RQ-036 traceability row.
- Requirement: The config model shall support measurement-backed DSL criteria without breaking existing criteria configs.
- Gate: Requirements Traceability Gate.
- Decision: Pass.
- Residual risk: WRA-RQ-037 through WRA-RQ-042 remain planned for later M7 issues.
- Next owner: Software Architect.

## Architecture

- Owner role: Software Architect
- Artifact: `crates/wra-core/src/config.rs`.
- Design: Keep the evaluator unchanged; add config-layer DSL structs and shape validation around the existing legacy `CriterionConfig` conversion path.
- Gate: Architecture Gate.
- Decision: Pass.
- Residual risk: M7-003 must decide how DSL measurement/requirement values map into existing `Criterion` variants.
- Next owner: Abstraction Review Engineer.

## Abstraction Review

- Owner role: Abstraction Review Engineer
- Artifact: This report and `crates/wra-core/src/config.rs`.
- Review: The slice is narrow enough for #55: schema, side-by-side deserialization, ambiguous-shape rejection, and legacy conversion preservation.
- Gate: Granularity Gate.
- Decision: Pass.
- Residual risk: Later issues should not retrofit operator behavior into this PR.
- Next owner: Core Software Engineer.

## Implementation

- Owner role: Core Software Engineer
- Artifact: `crates/wra-core/src/config.rs`, `crates/wra-cli/src/main.rs`, and `tests/configs/invalid-mixed-legacy-dsl-criterion.toml`.
- Behavior:
  - `CriterionConfig.kind` is optional so DSL-only criteria can deserialize.
  - `CriterionMeasurementConfig`, `CriterionRequirementConfig`, and `UnitValueConfig` represent the first DSL config shape.
  - `CriterionConfig::shape()` reports legacy or DSL shape.
  - `AnalysisConfig::validate()` rejects mixed legacy/DSL entries and incomplete DSL shape.
  - `CriterionConfig::to_criterion()` preserves legacy conversion and returns `NotImplemented` for DSL evaluation until M7-003.
- Gate: Implementation Gate.
- Decision: Pass.
- Residual risk: Unit/operator semantics are intentionally deferred to #56.
- Next owner: Test Automation Engineer.

## Testing

- Owner role: Test Automation Engineer
- Artifact: Config unit tests and CLI invalid-config fixture.
- Evidence:
  - Added side-by-side legacy/DSL deserialization test.
  - Added ambiguous mixed-shape rejection test.
  - Added test proving DSL config does not convert to runtime criteria yet.
  - Added CLI fixture coverage for mixed legacy/DSL config shape.
  - `cargo test --workspace` passed with 87 tests.
- Gate: Testing Gate.
- Decision: Pass.
- Residual risk: Visual and user-facing DSL ergonomics remain future work.
- Next owner: Verification and Validation Engineer.

## Verification And Validation

- Owner role: Verification and Validation Engineer
- Artifact: WRA-RQ-036 traceability and tests.
- Verification: #55 acceptance criteria map to config code and tests.
- Validation: This is a software configuration-boundary validation only; it is not runtime DSL validation, hardware validation, or certification evidence.
- Gate: V&V Gate.
- Decision: Pass.
- Residual risk: DSL parity validation belongs to #58.
- Next owner: QA Engineer.

## QA

- Owner role: QA Engineer
- Artifact: Local test results and scope review.
- Evidence: Legacy criteria tests continue to pass; DSL schema is accepted only at config shape level; ambiguous mixed entries fail clearly.
- Gate: QA Gate.
- Decision: Pass.
- Residual risk: External user feedback on DSL ergonomics remains future work.
- Next owner: Security Engineer.

## Security

- Owner role: Security Engineer
- Artifact: Dependency and surface review.
- Evidence: No new dependencies, unsafe Rust, file I/O surface, network surface, plugin runtime, DAQ integration, SDK, HAL, FFI, or executable expression language.
- Gate: Security Gate.
- Decision: Pass.
- Residual risk: Future DSL expansion must avoid expression evaluation or plugin execution without a fresh security review.
- Next owner: Performance Engineer.

## Performance

- Owner role: Performance Engineer
- Artifact: Code inspection and test scope.
- Evidence: Schema validation is linear over criteria entries and does not add waveform scanning or report generation work.
- Gate: Performance Gate.
- Decision: Pass.
- Residual risk: Runtime DSL evaluation performance belongs to #57/#58.
- Next owner: Documentation Engineer.

## Documentation

- Owner role: Documentation Engineer
- Artifact: This report plus requirements, traceability, and project-state updates.
- Evidence: Planning docs identify issue #55 and explicitly defer runtime evaluation, unit semantics, and docs refresh to later M7 issues.
- Gate: Documentation Gate.
- Decision: Pass.
- Residual risk: Full user-facing DSL docs remain in #60/#61.
- Next owner: Code Reviewer.

## Code Review

- Owner role: Code Review Engineer
- Artifact: Local code review.
- Findings: No blocking findings. The implementation keeps behavior at the config boundary and does not fork the evaluator.
- Gate: Code Review Gate.
- Decision: Pass.
- Residual risk: M7-003 should review any conversion from DSL structs to `Criterion` variants carefully.
- Next owner: Evaluation Engineer.

## Evaluation

- Owner role: Evaluation Engineer
- Artifact: This report.
- Result: #55 is implementation-actionable, verified, scoped, and traceable without absorbing #56/#57/#58/#60/#61.
- Gate: Evaluation Gate.
- Decision: Pass.
- Residual risk: The milestone remains open until all M7 issues close.
- Next owner: Release Engineer.

## Release

- Owner role: Release Engineer
- Artifact: Local validation evidence.
- Evidence: `cargo fmt --check`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, and `git diff --check` passed locally; PR #63 required `rust` CI passed in 28 seconds and merged at `9a8b0e667f9d829a1083168b7875db967ca4e960`.
- Gate: Release Gate.
- Decision: Pass.
- Residual risk: This is mainline repository evidence, not a tagged product release.
- Next owner: GitHub Maintainer Specialist.

## Community

- Owner role: Community Engineering Lead
- Artifact: Future PR body.
- Evidence: PR #63 closed issue #55. Milestone #7 remains open with issues #56 through #61.
- Gate: Community Gate.
- Decision: Pass.
- Residual risk: Issues #56 through #61 remain open.
- Next owner: Project Coordinator.

## Retrospective

- Owner role: Project Coordinator
- Artifact: This report.
- Lesson: Splitting schema recognition from operator semantics and evaluation keeps DSL risk reviewable.
- Gate: Retrospective Gate.
- Decision: Pass.
- Residual risk: Later M7 work still needs parity and invalid-config matrices.
- Next owner: Project Orchestrator.

## Hand-Off Note

Role: Core Software Engineer
Goal: Complete M7-001 / issue #55.
Files changed: `crates/wra-core/src/config.rs`, `crates/wra-cli/src/main.rs`, `tests/configs/invalid-mixed-legacy-dsl-criterion.toml`, requirements, traceability, project state, and this report.
Checks run: `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Complete; PR #63 merged and issue #55 closed.
Known gaps: Runtime DSL evaluation, operator semantics, explicit unit validation, DSL parity golden tests, and full user docs remain in issues #56 through #61.
Next recommended step: Start M7-002 / issue #56 through the implementation pipeline.
