# M7-005 Invalid DSL Config Tests Pipeline Report

Date: 2026-05-31

Branch: `feature/m7-005-invalid-dsl-config-tests`

Milestone: #7, `v0.5.0: Measurement-Backed Criteria DSL`

Issue: #59, `M7-005 Add invalid DSL config validation tests`

Pull request: Pending.

Status: Implemented and locally validated; PR/CI pending.

## Scope

This slice adds focused invalid-config coverage so bad DSL TOML fails with contextual errors rather than panics or ambiguous behavior.

In scope:

- Unknown operator coverage.
- Missing, unsupported, and mismatched unit coverage.
- Missing measurement or requirement section coverage.
- Missing requirement value and measurement threshold coverage.
- Incompatible measurement parameter coverage.
- Ambiguous legacy/DSL field-mixing coverage.
- CLI error-message coverage with actionable `criteria.<id>...` paths.

Out of scope:

- New parser dependencies.
- Shorthand unit parsing.
- New measurements or DSL operators.
- GUI, DAQ, plugin runtime, batch analysis, RTOS expansion, hardware qualification, or certification claims.

## Research

- Owner role: Software Architect / Test Automation Engineer
- Artifact: Issue #59, M7-001/M7-002 validation work, and M7-003 runtime conversion.
- Evidence: Runtime DSL evaluation made measurement-parameter validation meaningful at the config boundary.
- Gate: Intake Gate.
- Decision: Pass.
- Residual risk: Validation still intentionally rejects shorthand rather than converting units.
- Next owner: Software Architect.

## Requirements

- Owner role: Software Architect / V&V Engineer
- Artifact: WRA-RQ-038 and WRA-RQ-041 in `requirements.md`; traceability row in `traceability-matrix.md`.
- Requirement: Invalid DSL TOML shall fail with explicit errors for unknown operators, unit issues, missing fields, incompatible parameters, and ambiguous mixed shapes.
- Gate: Requirements Traceability Gate.
- Decision: Pass.
- Residual risk: User-facing docs still need to describe these constraints.
- Next owner: Software Architect.

## Architecture

- Owner role: Software Architect
- Artifact: `crates/ferrisoxide-core/src/config.rs`.
- Design: Extend `AnalysisConfig::validate()` so DSL measurement parameters are validated at the config boundary using the same measurement-spec conversion as runtime criteria.
- Gate: Architecture Gate.
- Decision: Pass.
- Residual risk: Future shorthand-unit parsing would require a separate parser design and tests.
- Next owner: Abstraction Review Engineer.

## Abstraction Review

- Owner role: Abstraction Review Engineer
- Artifact: This report and invalid fixture diff.
- Review: The slice is restricted to validation and test coverage. It does not add runtime semantics, docs migration content, rule packages, or new measurement types.
- Gate: Granularity Gate.
- Decision: Pass.
- Residual risk: Docs in #60/#61 should reflect the exact validation behavior.
- Next owner: Core Software Engineer.

## Implementation

- Owner role: Core Software Engineer / Test Automation Engineer
- Artifact: `crates/ferrisoxide-core/src/config.rs`, `crates/ferrisoxide-cli/src/main.rs`, and `tests/configs/invalid-dsl-*`.
- Behavior:
  - `validate()` now catches missing measurement thresholds, invalid states, missing pulse-width `selection` for `equal_to`, and inverted edge thresholds.
  - CLI invalid-config tests now cover the new invalid DSL fixtures.
  - Existing unknown-operator, unit, shorthand, and mixed-shape coverage remains intact.
- Gate: Implementation Gate.
- Decision: Pass locally.
- Residual risk: Protected CI remains pending.
- Next owner: Test Automation Engineer.

## Testing

- Owner role: Test Automation Engineer
- Artifact: Config unit tests and CLI semantic-error test.
- Evidence:
  - `cargo test -p ferrisoxide-core config::tests -- --nocapture`: Pass; 19 config tests passed.
  - `cargo test -p ferrisoxide-cli invalid_config_semantics_return_clear_errors`: Pass.
  - `cargo fmt`: Pass.
  - `cargo fmt --check`: Pass.
  - `cargo test --workspace`: Pass; 105 tests passed.
  - `cargo clippy --workspace --all-targets -- -D warnings`: Pass.
  - `git diff --check`: Pass.
- Gate: Testing Gate.
- Decision: Pass locally.
- Residual risk: Protected CI remains pending until PR creation.
- Next owner: Verification and Validation Engineer.

## Verification And Validation

- Owner role: Verification and Validation Engineer
- Artifact: WRA-RQ-041 traceability.
- Verification: Tests cover all issue #59 invalid categories and assert error paths include enough TOML context for users to repair the config.
- Validation: This is software config validation only; it is not hardware validation, DAQ validation, RTOS validation, production readiness, or certification evidence.
- Gate: V&V Gate.
- Decision: Pass locally.
- Residual risk: Human-readable public docs remain #60/#61.
- Next owner: QA Engineer.

## QA

- Owner role: QA Engineer
- Artifact: Error-message review.
- Evidence: CLI errors retain the `invalid config: invalid parameter` prefix plus `criteria.<id>...` field paths for DSL-specific failures.
- Gate: QA Gate.
- Decision: Pass locally.
- Residual risk: Error wording may need examples in docs.
- Next owner: Security Engineer.

## Security

- Owner role: Security Engineer
- Artifact: Parser/dependency review.
- Evidence: No new dependencies, shorthand parser, expression language, unsafe code, network surface, DAQ SDK, HAL, RTOS SDK, FFI, or shell surface were added.
- Gate: Security Gate.
- Decision: Pass locally.
- Residual risk: Future parser expansion requires separate review.
- Next owner: Performance Engineer.

## Performance

- Owner role: Performance Engineer
- Artifact: Code inspection.
- Evidence: Validation adds per-criterion enum/string checks at config load time and no waveform scanning or runtime analysis overhead beyond existing conversion.
- Gate: Performance Gate.
- Decision: Pass locally.
- Residual risk: None for MVP-scale config validation.
- Next owner: Documentation Engineer.

## Documentation

- Owner role: Documentation Engineer
- Artifact: This report, requirements, traceability, risk register, and project state.
- Evidence: Pipeline artifacts document invalid categories and remaining docs issues.
- Gate: Documentation Gate.
- Decision: Pass locally.
- Residual risk: User-facing docs remain #60/#61.
- Next owner: Code Reviewer.

## Code Review

- Owner role: Code Review Engineer
- Artifact: Local code review.
- Findings: No blocking findings. Validation reuses existing measurement-spec conversion helpers and keeps invalid fixtures focused.
- Gate: Code Review Gate.
- Decision: Pass locally.
- Residual risk: Protected CI and PR review remain pending.
- Next owner: Evaluation Engineer.

## Evaluation

- Owner role: Evaluation Engineer
- Artifact: This report.
- Result: #59 closes the main validation gap before public docs expansion.
- Gate: Evaluation Gate.
- Decision: Pass locally.
- Residual risk: Milestone #7 remains open for #60 and #61.
- Next owner: Release Engineer.

## Release

- Owner role: Release Engineer
- Artifact: Focused validation evidence.
- Evidence: Full local validation passed; PR creation and required `rust` CI remain pending.
- Gate: Release Gate.
- Decision: Pass locally; blocked on protected CI before merge.
- Residual risk: PR/CI may uncover host-level issues.
- Next owner: GitHub Maintainer Specialist.

## Community

- Owner role: Community Engineering Lead
- Artifact: Pending PR body.
- Evidence: PR must include `Fixes #59` after full validation.
- Gate: Community Gate.
- Decision: Blocked until PR creation.
- Residual risk: Issue #59 remains open until PR merge.
- Next owner: Project Coordinator.

## Retrospective

- Owner role: Project Coordinator
- Artifact: This report.
- Lesson: Runtime DSL support should immediately be followed by invalid-config tests so docs can teach bounded behavior instead of assumptions.
- Gate: Retrospective Gate.
- Decision: Pass locally.
- Residual risk: #60/#61 docs should use the tested invalid cases as examples where useful.
- Next owner: Project Orchestrator.

## Hand-Off Note

Role: Test Automation Engineer / Core Software Engineer
Goal: Complete M7-005 / issue #59.
Files changed: `crates/ferrisoxide-core/src/config.rs`, `crates/ferrisoxide-cli/src/main.rs`, invalid DSL fixtures, requirements, traceability, risk register, project state, validation log, and this report.
Checks run: `cargo test -p ferrisoxide-core config::tests -- --nocapture`; `cargo test -p ferrisoxide-cli invalid_config_semantics_return_clear_errors`; `cargo fmt`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Implemented and locally validated; PR pending.
Known gaps: #60 engineering examples/migration docs and #61 schema/report evidence notes remain open.
Next recommended step: Run full validation, open PR for #59, wait for required CI, merge, then continue issue #60.
