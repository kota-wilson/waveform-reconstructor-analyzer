# M7-002 DSL Operator And Unit Validation Pipeline Report

Date: 2026-05-31

Branch: `feature/m7-002-dsl-operator-units`

Milestone: #7, `v0.5.0: Measurement-Backed Criteria DSL`

Issue: #56, `M7-002 Implement DSL operator and explicit-unit validation`

Pull request: #65, `Add M7 DSL operator and unit validation`

Status: Implemented and merged.

## Scope

This slice validates the first DSL operator vocabulary and explicit-unit policy at the config boundary.

In scope:

- Accept `less_than`, `less_than_or_equal`, `greater_than`, `greater_than_or_equal`, and `equal_to`.
- Reject unknown operators with structured `InvalidParameter` errors.
- Require explicit unit fields for DSL requirement values.
- Require explicit unit fields for DSL threshold values when threshold values are present.
- Support only `V`, `s`, and `count` as initial DSL units.
- Reject supported-but-mismatched units based on measurement output.
- Reject unit shorthand values such as `value = "5ms"` by keeping numeric value fields.
- Preserve M7-001 legacy/DSL shape compatibility behavior.

Out of scope:

- Runtime DSL evaluation.
- DSL-to-legacy parity golden tests.
- Full invalid-config matrix for all field combinations.
- User-facing schema reference refresh.
- Unit conversion or shorthand parsing.
- New measurement primitives.
- GUI, DAQ, plugin runtime, batch analysis, RTOS expansion, hardware qualification, or certification claims.

## Research

- Owner role: Software Architect / Core Software Engineer
- Artifact: Issue #56, `docs/criteria-dsl.md`, and `crates/wra-core/src/config.rs`.
- Evidence: M6 documented the approved operator vocabulary and explicit-unit preference; M7-001 added config-layer DSL structs.
- Gate: Intake Gate.
- Decision: Pass.
- Residual risk: Unit validation can still be mistaken for unit conversion support; this slice rejects conversion instead.
- Next owner: Software Architect.

## Requirements

- Owner role: Software Architect / V&V Engineer
- Artifact: WRA-RQ-037 and WRA-RQ-038 in `requirements.md`; traceability rows in `traceability-matrix.md`.
- Requirements: validate the approved operator vocabulary and require explicit units for DSL requirements and thresholds.
- Gate: Requirements Traceability Gate.
- Decision: Pass.
- Residual risk: WRA-RQ-041 remains planned for issue #59's broader invalid-config matrix.
- Next owner: Software Architect.

## Architecture

- Owner role: Software Architect
- Artifact: `crates/wra-core/src/config.rs`.
- Design: Keep validation at the config boundary with `CriterionOperator`, `CriterionMeasurementKind`, supported-unit checks, expected-unit checks, and no evaluator changes.
- Gate: Architecture Gate.
- Decision: Pass.
- Residual risk: M7-003 must reuse the validated config values without reinterpreting units.
- Next owner: Abstraction Review Engineer.

## Abstraction Review

- Owner role: Abstraction Review Engineer
- Artifact: This report and config code.
- Review: The slice is specific to #56 and does not absorb #57 runtime evaluation, #58 parity tests, #59 full invalid matrix, or #60/#61 docs.
- Gate: Granularity Gate.
- Decision: Pass.
- Residual risk: Future docs should align exact accepted units and measurement output units with implementation.
- Next owner: Core Software Engineer.

## Implementation

- Owner role: Core Software Engineer
- Artifact: `crates/wra-core/src/config.rs`, `crates/wra-cli/src/main.rs`, and invalid DSL config fixtures.
- Behavior:
  - `CriterionOperator` validates the five approved operators.
  - `CriterionMeasurementKind` validates existing candidate measurement names and expected output units.
  - Requirement units are required, supported, and matched to measurement output.
  - Threshold units are required when threshold values exist and must be volts.
  - Shorthand values remain rejected by numeric TOML deserialization.
- Gate: Implementation Gate.
- Decision: Pass locally.
- Residual risk: Runtime comparator semantics remain deferred.
- Next owner: Test Automation Engineer.

## Testing

- Owner role: Test Automation Engineer
- Artifact: Config unit tests and CLI invalid-config fixtures.
- Evidence:
  - Supported-operator test covers all five operators.
  - Unit tests cover `V`, `s`, and `count` requirement units.
  - Rejection tests cover unknown operators, missing units, unsupported units, mismatched requirement units, mismatched threshold units, and shorthand string values.
  - CLI invalid-config tests cover unknown operator and missing requirement unit.
- Gate: Testing Gate.
- Decision: Pass locally.
- Validation:
  - `cargo fmt --check`: Pass.
  - `cargo test --workspace`: Pass.
  - `cargo clippy --workspace --all-targets -- -D warnings`: Pass.
  - `git diff --check`: Pass.
- Residual risk: Protected CI remains pending until PR creation.
- Next owner: Verification and Validation Engineer.

## Verification And Validation

- Owner role: Verification and Validation Engineer
- Artifact: WRA-RQ-037 and WRA-RQ-038 traceability.
- Verification: #56 acceptance criteria map to config code and focused tests.
- Validation: This is software config validation only; it is not runtime DSL evaluation, hardware validation, DAQ validation, or certification evidence.
- Gate: V&V Gate.
- Decision: Pass locally.
- Residual risk: Full invalid-config matrix belongs to #59.
- Next owner: QA Engineer.

## QA

- Owner role: QA Engineer
- Artifact: Focused test results and scope review.
- Evidence: Operator and unit validation fails clearly before runtime evaluation, preserving M7-001's not-implemented guard.
- Gate: QA Gate.
- Decision: Pass locally.
- Residual risk: User-facing error wording may need refinement during docs work.
- Next owner: Security Engineer.

## Security

- Owner role: Security Engineer
- Artifact: Dependency and parser-surface review.
- Evidence: No new dependencies, unsafe Rust, expression language, unit parser, plugin runtime, network, DAQ, SDK, HAL, FFI, or shell surface.
- Gate: Security Gate.
- Decision: Pass locally.
- Residual risk: Future shorthand parsing would need separate parser review.
- Next owner: Performance Engineer.

## Performance

- Owner role: Performance Engineer
- Artifact: Code inspection.
- Evidence: Validation is per-criterion string matching and does not add waveform scanning, report generation, allocation-heavy processing, or runtime analysis work.
- Gate: Performance Gate.
- Decision: Pass locally.
- Residual risk: Runtime DSL evaluation performance belongs to #57/#58.
- Next owner: Documentation Engineer.

## Documentation

- Owner role: Documentation Engineer
- Artifact: This report, requirements, traceability, and project-state updates.
- Evidence: The report records accepted operators, supported units, explicit non-goals, and deferred runtime/docs issues.
- Gate: Documentation Gate.
- Decision: Pass locally.
- Residual risk: Full user-facing schema docs remain in #61.
- Next owner: Code Reviewer.

## Code Review

- Owner role: Code Review Engineer
- Artifact: Local code review.
- Findings: No blocking findings. Validation remains in config code and does not change legacy criteria behavior.
- Gate: Code Review Gate.
- Decision: Pass locally.
- Residual risk: Exact unit policy should be rechecked when DSL evaluation maps to `Criterion`.
- Next owner: Evaluation Engineer.

## Evaluation

- Owner role: Evaluation Engineer
- Artifact: This report.
- Result: #56 is scoped, testable, and traceable without absorbing runtime evaluation or docs-refresh issues.
- Gate: Evaluation Gate.
- Decision: Pass locally.
- Residual risk: Milestone #7 remains open until #57-#61 close.
- Next owner: Release Engineer.

## Release

- Owner role: Release Engineer
- Artifact: Local validation evidence.
- Evidence: Full local validation passed; PR #65 required `rust` CI passed and merged on 2026-05-31 with merge commit `37cff043ff9ed16d7bb27ae2ddf315732ed20203`.
- Gate: Release Gate.
- Decision: Pass.
- Residual risk: Milestone #7 remains open for issues #57 through #61.
- Next owner: GitHub Maintainer Specialist.

## Community

- Owner role: Community Engineering Lead
- Artifact: PR #65 body.
- Evidence: PR #65 included `Fixes #56`; issue #56 closed after merge; issues #57 through #61 remain open.
- Gate: Community Gate.
- Decision: Pass.
- Residual risk: Issues #57 through #61 remain open.
- Next owner: Project Coordinator.

## Retrospective

- Owner role: Project Coordinator
- Artifact: This report.
- Lesson: Unit validation belongs before runtime evaluation so later DSL behavior cannot silently reinterpret unsupported units.
- Gate: Retrospective Gate.
- Decision: Pass.
- Residual risk: Later M7 issues still need parity and documentation checks.
- Next owner: Project Orchestrator.

## Hand-Off Note

Role: Core Software Engineer
Goal: Complete M7-002 / issue #56.
Files changed: `crates/wra-core/src/config.rs`, `crates/wra-cli/src/main.rs`, invalid DSL config fixtures, requirements, traceability, project state, and this report.
Checks run: `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Implemented and merged in PR #65.
Known gaps: Runtime DSL evaluation, parity golden tests, full invalid matrix, and user-facing docs remain in issues #57 through #61.
Next recommended step: Continue with M7-003 / issue #57.
