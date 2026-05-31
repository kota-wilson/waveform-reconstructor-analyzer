# M7-006 DSL Examples And Migration Docs Pipeline Report

Date: 2026-05-31

Branch: `feature/m7-006-dsl-examples-docs`

Milestone: #7, `v0.5.0: Measurement-Backed Criteria DSL`

Issue: #60, `M7-006 Add engineering DSL examples and migration docs`

Pull request: Pending.

Status: Implemented and locally validated; PR/CI pending.

## Scope

This slice gives engineering reviewers a working before/after path from legacy criteria configs to the measurement-backed DSL.

In scope:

- Add `examples/basic-dsl-config.toml`.
- Link the working DSL example from README and MVP usage docs.
- Add migration notes explaining when to use DSL versus legacy explicit fields.
- Include expected text output excerpts from a real CLI run.
- State explicit units, no unit shorthand parser, compatibility expectations, and non-goals.

Out of scope:

- Full schema reference and report evidence field notes for issue #61.
- New measurement primitives.
- GUI, DAQ, plugin runtime, batch analysis, RTOS expansion, hardware qualification, or certification claims.

## Research

- Owner role: Documentation Engineer / Test Automation Engineer
- Artifact: Issue #60, M7-003 runtime behavior, M7-004 parity fixtures, and M7-005 invalid-config tests.
- Evidence: DSL behavior is now implemented, parity-tested, and invalid-config tested enough to document a working migration path.
- Gate: Intake Gate.
- Decision: Pass.
- Residual risk: Schema reference details remain #61.
- Next owner: Documentation Engineer.

## Requirements

- Owner role: Documentation Engineer / V&V Engineer
- Artifact: WRA-RQ-042 in `requirements.md` and `traceability-matrix.md`.
- Requirement: README, examples, and docs shall show before/after configs, expected output, explicit unit rules, compatibility expectations, and non-goals.
- Gate: Requirements Traceability Gate.
- Decision: Pass.
- Residual risk: Report/schema notes remain in #61 before WRA-RQ-042 is fully complete.
- Next owner: Documentation Engineer.

## Architecture

- Owner role: Software Architect
- Artifact: `examples/basic-dsl-config.toml` and `docs/criteria-dsl-migration.md`.
- Design: Keep the DSL example equivalent to `examples/basic-config.toml` so users can compare old and new criteria forms without changing the input data or report evidence.
- Gate: Architecture Gate.
- Decision: Pass.
- Residual risk: Future rule-package docs should reuse the same measurement/requirement concepts.
- Next owner: Abstraction Review Engineer.

## Abstraction Review

- Owner role: Abstraction Review Engineer
- Artifact: This report and docs diff.
- Review: The slice is documentation/example-focused and does not absorb #61 schema reference work or v0.6.0 rule-package design.
- Gate: Granularity Gate.
- Decision: Pass.
- Residual risk: Keep schema tables out of this slice except where needed for migration.
- Next owner: Documentation Engineer.

## Implementation

- Owner role: Documentation Engineer / Core Software Engineer
- Artifact: README, `docs/usage-mvp.md`, `docs/criteria-dsl.md`, `docs/criteria-dsl-migration.md`, `examples/basic-dsl-config.toml`, and CLI test coverage.
- Behavior:
  - The checked-in DSL example analyzes `examples/basic-waveform.csv`.
  - CLI test coverage verifies the example produces a passing text report with measurement IDs.
  - Docs include before/after TOML snippets, command, output excerpt, unit rules, compatibility notes, and non-goals.
- Gate: Implementation Gate.
- Decision: Pass locally.
- Residual risk: Protected CI remains pending.
- Next owner: Test Automation Engineer.

## Testing

- Owner role: Test Automation Engineer
- Artifact: CLI smoke and unit test.
- Evidence:
  - `cargo run --quiet --bin ferrisoxide-signal -- analyze --input examples/basic-waveform.csv --config examples/basic-dsl-config.toml --format text`: Pass.
  - `runs_analysis_with_dsl_config_and_text_output` added to CLI tests.
  - `cargo test -p ferrisoxide-cli runs_analysis_with_dsl_config_and_text_output`: Pass.
  - `cargo fmt`: Pass.
  - `cargo fmt --check`: Pass.
  - `cargo test --workspace`: Pass; 106 tests passed.
  - `cargo clippy --workspace --all-targets -- -D warnings`: Pass.
  - `git diff --check`: Pass.
- Gate: Testing Gate.
- Decision: Pass locally.
- Residual risk: Protected CI remains pending until PR creation.
- Next owner: Verification and Validation Engineer.

## Verification And Validation

- Owner role: Verification and Validation Engineer
- Artifact: CLI output excerpt and docs.
- Verification: The documented output excerpt was taken from the actual CLI run against the checked-in DSL example.
- Validation: This is software documentation and example validation only; it is not hardware validation, DAQ validation, RTOS validation, production readiness, or certification evidence.
- Gate: V&V Gate.
- Decision: Pass locally.
- Residual risk: Full schema and report evidence docs remain #61.
- Next owner: QA Engineer.

## QA

- Owner role: QA Engineer
- Artifact: Documentation review.
- Evidence: Docs state compatibility expectations, no shorthand parser, explicit unit rules, and non-goals.
- Gate: QA Gate.
- Decision: Pass locally.
- Residual risk: Schema reference should keep wording consistent.
- Next owner: Security Engineer.

## Security

- Owner role: Security Engineer
- Artifact: Docs/dependency review.
- Evidence: No new dependencies, parser behavior, shell surface, network surface, DAQ SDK, HAL, RTOS SDK, FFI, or unsafe code were added.
- Gate: Security Gate.
- Decision: Pass locally.
- Residual risk: Future parser expansion requires separate review.
- Next owner: Performance Engineer.

## Performance

- Owner role: Performance Engineer
- Artifact: Example review.
- Evidence: The example uses a five-sample CSV and makes no performance claim.
- Gate: Performance Gate.
- Decision: Pass locally.
- Residual risk: None for docs/example scope.
- Next owner: Documentation Engineer.

## Documentation

- Owner role: Documentation Engineer
- Artifact: README, `docs/usage-mvp.md`, `docs/criteria-dsl.md`, and `docs/criteria-dsl-migration.md`.
- Evidence: Reviewer-facing migration content exists and links to the working checked-in example.
- Gate: Documentation Gate.
- Decision: Pass locally.
- Residual risk: #61 remains for schema/reference completeness.
- Next owner: Code Reviewer.

## Code Review

- Owner role: Code Review Engineer
- Artifact: Local review.
- Findings: No blocking findings. Example config mirrors the existing basic config and does not change runtime behavior.
- Gate: Code Review Gate.
- Decision: Pass locally.
- Residual risk: Protected CI and PR review remain pending.
- Next owner: Evaluation Engineer.

## Evaluation

- Owner role: Evaluation Engineer
- Artifact: This report.
- Result: #60 makes the DSL understandable to engineering reviewers without expanding feature scope.
- Gate: Evaluation Gate.
- Decision: Pass locally.
- Residual risk: Milestone #7 remains open for #61.
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
- Evidence: PR must include `Fixes #60` after full validation.
- Gate: Community Gate.
- Decision: Blocked until PR creation.
- Residual risk: Issue #60 remains open until PR merge.
- Next owner: Project Coordinator.

## Retrospective

- Owner role: Project Coordinator
- Artifact: This report.
- Lesson: Runtime examples should be generated from checked-in config files so docs do not drift from behavior.
- Gate: Retrospective Gate.
- Decision: Pass locally.
- Residual risk: #61 should keep schema docs synchronized with migration docs.
- Next owner: Project Orchestrator.

## Hand-Off Note

Role: Documentation Engineer
Goal: Complete M7-006 / issue #60.
Files changed: README, `examples/basic-dsl-config.toml`, `docs/criteria-dsl-migration.md`, `docs/criteria-dsl.md`, `docs/usage-mvp.md`, CLI tests, requirements, traceability, risk register, project state, validation log, and this report.
Checks run: CLI smoke for `examples/basic-dsl-config.toml`; `cargo test -p ferrisoxide-cli runs_analysis_with_dsl_config_and_text_output`; `cargo fmt`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Implemented and locally validated; PR pending.
Known gaps: #61 schema reference and report evidence notes remain open.
Next recommended step: Run full validation, open PR for #60, wait for required CI, merge, then continue issue #61.
